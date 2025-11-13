#!/usr/bin/env node
/**
 * 🔧 PIXLY Eagle JSON修复工具
 * 
 * 用途：修复Eagle库中metadata.json与实际文件扩展名不匹配的问题
 * 
 * 使用方法：
 * node FIX_EAGLE_JSON.js [Eagle库路径]
 * 
 * 例如：
 * node FIX_EAGLE_JSON.js "/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library"
 */

const fs = require('fs');
const path = require('path');

// 解析命令行参数
const libraryPath = process.argv[2] || '/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library';
const imagesPath = path.join(libraryPath, 'images');

console.log('🔧 PIXLY Eagle JSON修复工具');
console.log('================================');
console.log(`📁 Eagle库路径: ${libraryPath}`);
console.log(`📂 图像目录: ${imagesPath}`);
console.log('');

if (!fs.existsSync(imagesPath)) {
    console.error('❌ 错误: 图像目录不存在！');
    console.error(`   路径: ${imagesPath}`);
    process.exit(1);
}

// 统计
let totalChecked = 0;
let totalFixed = 0;
let totalErrors = 0;
const fixedFiles = [];

// 扫描所有.info目录
console.log('🔍 扫描Eagle库...\n');

const infoDirs = fs.readdirSync(imagesPath).filter(name => name.endsWith('.info'));

console.log(`📊 找到 ${infoDirs.length} 个项目\n`);

for (const infoDir of infoDirs) {
    const infoDirPath = path.join(imagesPath, infoDir);
    const metadataJsonPath = path.join(infoDirPath, 'metadata.json');
    
    // 跳过没有metadata.json的目录
    if (!fs.existsSync(metadataJsonPath)) {
        continue;
    }
    
    try {
        // 读取metadata.json
        const metadata = JSON.parse(fs.readFileSync(metadataJsonPath, 'utf8'));
        const jsonExt = metadata.ext;
        const name = metadata.name;
        
        // 查找实际文件
        const files = fs.readdirSync(infoDirPath).filter(f => 
            !f.endsWith('.json') && 
            !f.endsWith('_thumbnail.png') &&
            !f.endsWith('.backup') &&
            f.startsWith(name)
        );
        
        if (files.length === 0) {
            continue; // 没有媒体文件
        }
        
        totalChecked++;
        
        // 获取实际文件的扩展名
        const actualFile = files[0];
        const actualExt = path.extname(actualFile).slice(1).toLowerCase();
        
        // 检查是否需要修复
        if (jsonExt !== actualExt) {
            console.log(`🔧 [${totalChecked}] 发现不匹配:`);
            console.log(`   名称: ${name}`);
            console.log(`   JSON扩展名: ${jsonExt}`);
            console.log(`   实际扩展名: ${actualExt}`);
            console.log(`   文件: ${actualFile}`);
            
            // 备份JSON
            const backupPath = `${metadataJsonPath}.backup`;
            fs.copyFileSync(metadataJsonPath, backupPath);
            
            try {
                // 更新metadata
                metadata.ext = actualExt;
                metadata.lastModified = Date.now();
                metadata.modificationTime = Date.now();
                metadata.mtime = Date.now();
                
                // 写回文件
                fs.writeFileSync(metadataJsonPath, JSON.stringify(metadata, null, 4), 'utf8');
                
                // 验证
                const verify = JSON.parse(fs.readFileSync(metadataJsonPath, 'utf8'));
                if (verify.ext !== actualExt) {
                    throw new Error('验证失败：扩展名未更新');
                }
                
                // 删除备份
                fs.unlinkSync(backupPath);
                
                console.log(`   ✅ 已修复: ${jsonExt} → ${actualExt}\n`);
                totalFixed++;
                fixedFiles.push({
                    name,
                    from: jsonExt,
                    to: actualExt,
                    file: actualFile
                });
                
            } catch (fixError) {
                // 恢复备份
                if (fs.existsSync(backupPath)) {
                    fs.copyFileSync(backupPath, metadataJsonPath);
                    fs.unlinkSync(backupPath);
                }
                console.error(`   ❌ 修复失败: ${fixError.message}\n`);
                totalErrors++;
            }
        }
        
    } catch (error) {
        console.error(`❌ 处理失败: ${infoDir} - ${error.message}`);
        totalErrors++;
    }
}

// 输出报告
console.log('\n================================');
console.log('📊 修复完成！');
console.log('================================');
console.log(`检查项目数: ${totalChecked}`);
console.log(`修复成功: ${totalFixed}`);
console.log(`错误: ${totalErrors}`);
console.log('');

if (fixedFiles.length > 0) {
    console.log('✅ 修复的文件:');
    fixedFiles.forEach((item, index) => {
        console.log(`${index + 1}. ${item.name} (${item.from} → ${item.to})`);
    });
    console.log('');
}

console.log('💡 提示: 请重新启动 Eagle 应用以刷新缓存');
console.log('');


