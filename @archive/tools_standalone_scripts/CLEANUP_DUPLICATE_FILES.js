#!/usr/bin/env node
/**
 * 清理重复文件工具
 * 当同一目录有多个媒体文件时（如jpg+jxl），保留较新的并更新metadata.json
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

console.log('🧹 清理重复文件工具');
console.log('================================\n');

/**
 * 智能检测Eagle库路径
 */
function detectEagleLibrary() {
    // 1. 从命令行参数获取
    if (process.argv[2]) {
        const argPath = process.argv[2];
        if (fs.existsSync(argPath)) {
            console.log('✅ 使用命令行指定的路径:', argPath);
            return argPath;
        }
    }
    
    // 2. 从环境变量获取
    if (process.env.EAGLE_LIBRARY_PATH && fs.existsSync(process.env.EAGLE_LIBRARY_PATH)) {
        console.log('✅ 使用环境变量 EAGLE_LIBRARY_PATH:', process.env.EAGLE_LIBRARY_PATH);
        return process.env.EAGLE_LIBRARY_PATH;
    }
    
    // 3. 尝试从Eagle配置文件读取（macOS）
    const eagleConfigPath = path.join(os.homedir(), 'Library', 'Application Support', 'Eagle', 'config.json');
    if (fs.existsSync(eagleConfigPath)) {
        try {
            const config = JSON.parse(fs.readFileSync(eagleConfigPath, 'utf8'));
            if (config.library && config.library.path) {
                const libraryPath = path.join(config.library.path, 'images');
                if (fs.existsSync(libraryPath)) {
                    console.log('✅ 从Eagle配置文件检测到:', libraryPath);
                    return libraryPath;
                }
            }
        } catch (e) {
            // 配置文件解析失败，继续尝试其他方法
        }
    }
    
    // 4. 搜索常见位置
    const commonPaths = [
        path.join(os.homedir(), 'Documents', 'Eagle', 'Library.library', 'images'),
        path.join(os.homedir(), 'Documents', 'Eagle Library', 'images'),
        path.join(os.homedir(), 'Eagle', 'Library.library', 'images'),
        // 项目内测试库
        path.join(__dirname, '..', '1.library', 'images'),
    ];
    
    for (const tryPath of commonPaths) {
        if (fs.existsSync(tryPath)) {
            console.log('✅ 在常见位置找到Eagle库:', tryPath);
            return tryPath;
        }
    }
    
    return null;
}

const EAGLE_LIBRARY = detectEagleLibrary();

if (!EAGLE_LIBRARY) {
    console.log('⚠️  未能自动检测到Eagle库目录');
    console.log('💡 请使用以下方式之一指定Eagle库路径：');
    console.log('   1. 命令行参数: node CLEANUP_DUPLICATE_FILES.js /path/to/eagle/library/images');
    console.log('   2. 环境变量: export EAGLE_LIBRARY_PATH=/path/to/eagle/library/images');
    console.log('✅ 跳过清理（无需操作）\n');
    process.exit(0);
}

const PRIORITY = ['jxl', 'avif', 'webp', 'png', 'jpg', 'jpeg', 'gif', 'heic', 'bmp', 'tiff'];

const infoDirs = fs.readdirSync(EAGLE_LIBRARY).filter(name => name.endsWith('.info'));
console.log(`📁 找到 ${infoDirs.length} 个.info目录\n`);

let duplicateFound = 0;
let filesDeleted = 0;
let metadataFixed = 0;

infoDirs.forEach((infoDir, index) => {
    const fullPath = path.join(EAGLE_LIBRARY, infoDir);
    const metadataPath = path.join(fullPath, 'metadata.json');
    
    if (!fs.existsSync(metadataPath)) return;
    
    try {
        // 查找所有媒体文件
        const files = fs.readdirSync(fullPath);
        const mediaFiles = files.filter(f => {
            const lower = f.toLowerCase();
            return lower.match(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/) 
                   && !lower.includes('thumbnail');
        });
        
        if (mediaFiles.length <= 1) return;  // 没有重复
        
        duplicateFound++;
        console.log(`[${index+1}/${infoDirs.length}] 🔧 发现重复文件: ${infoDir}`);
        
        // 按优先级和时间排序
        const sortedFiles = mediaFiles.map(f => {
            const fullFilePath = path.join(fullPath, f);
            const ext = path.extname(f).substring(1).toLowerCase();
            const stats = fs.statSync(fullFilePath);
            return {
                name: f,
                path: fullFilePath,
                ext: ext,
                mtime: stats.mtime,
                size: stats.size,
                priority: PRIORITY.indexOf(ext) >= 0 ? PRIORITY.indexOf(ext) : 999
            };
        }).sort((a, b) => {
            // 先按优先级（jxl > avif > webp...）
            if (a.priority !== b.priority) return a.priority - b.priority;
            // 再按时间（较新的优先）
            return b.mtime - a.mtime;
        });
        
        // 保留第一个（优先级最高且最新）
        const toKeep = sortedFiles[0];
        const toDelete = sortedFiles.slice(1);
        
        console.log(`   ✅ 保留: ${toKeep.name} (${toKeep.ext}, ${(toKeep.size/1024).toFixed(1)}KB)`);
        
        toDelete.forEach(file => {
            console.log(`   🗑️  删除: ${file.name} (${file.ext}, ${(file.size/1024).toFixed(1)}KB)`);
            
            try {
                fs.unlinkSync(file.path);
                filesDeleted++;
            } catch (deleteError) {
                console.error(`   ❌ 删除失败: ${deleteError.message}`);
            }
        });
        
        // 更新metadata.json
        const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));
        const keepBaseName = toKeep.name.replace(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/i, '');
        
        let needUpdate = false;
        
        if (metadata.name !== keepBaseName) {
            console.log(`   📝 更新name: "${metadata.name}" → "${keepBaseName}"`);
            metadata.name = keepBaseName;
            needUpdate = true;
        }
        
        if (metadata.ext !== toKeep.ext) {
            console.log(`   📝 更新ext: "${metadata.ext}" → "${toKeep.ext}"`);
            metadata.ext = toKeep.ext;
            needUpdate = true;
        }
        
        if (metadata.size !== toKeep.size) {
            console.log(`   📝 更新size: ${metadata.size} → ${toKeep.size}`);
            metadata.size = toKeep.size;
            needUpdate = true;
        }
        
        if (needUpdate) {
            // 备份
            fs.copyFileSync(metadataPath, metadataPath + '.backup');
            
            // 更新时间戳
            metadata.modificationTime = Date.now();
            metadata.lastModified = Date.now();
            
            // 写入
            fs.writeFileSync(metadataPath, JSON.stringify(metadata, null, 4), 'utf8');
            metadataFixed++;
            console.log(`   ✅ metadata.json 已更新`);
        }
        
        console.log('');
        
    } catch (error) {
        console.error(`[${index+1}/${infoDirs.length}] ❌ 处理失败: ${infoDir}`, error.message);
    }
});

console.log('\n================================');
console.log('📊 清理统计:');
console.log(`   发现重复: ${duplicateFound}`);
console.log(`   删除文件: ${filesDeleted}`);
console.log(`   更新元数据: ${metadataFixed}`);
console.log('================================\n');

if (duplicateFound > 0) {
    console.log('💡 提示：');
    console.log('   - 已删除旧文件，保留最新和最优格式');
    console.log('   - 元数据已同步更新');
    console.log('   - 请重启Eagle查看效果');
}

console.log('\n✅ 清理完成！');
