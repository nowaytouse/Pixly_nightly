#!/usr/bin/env node
/**
 * Eagle 元数据修复工具
 * 修复 metadata.json 与实际文件不匹配的问题
 * 
 * 问题1: name字段包含旧扩展名（gif表情_814.gif）
 * 问题2: ext字段未更新（实际是jxl但JSON里是jpg）
 */

const fs = require('fs');
const path = require('path');

const EAGLE_LIBRARY = '/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library/images';

console.log('🔧 Eagle 元数据修复工具');
console.log('================================\n');

const infoDirs = fs.readdirSync(EAGLE_LIBRARY).filter(name => name.endsWith('.info'));
console.log(`📁 找到 ${infoDirs.length} 个.info目录\n`);

let totalChecked = 0;
let nameFixed = 0;
let extFixed = 0;
let errors = 0;

infoDirs.forEach((infoDir, index) => {
    const fullPath = path.join(EAGLE_LIBRARY, infoDir);
    const metadataPath = path.join(fullPath, 'metadata.json');
    
    if (!fs.existsSync(metadataPath)) return;
    
    try {
        const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));
        totalChecked++;
        
        // 查找实际的媒体文件
        const files = fs.readdirSync(fullPath);
        const mediaFiles = files.filter(f => {
            const lower = f.toLowerCase();
            return lower.match(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/) 
                   && !lower.includes('thumbnail');
        });
        
        if (mediaFiles.length === 0) return;
        if (mediaFiles.length > 1) {
            console.log(`[${index+1}/${infoDirs.length}] ⚠️  多个媒体文件: ${infoDir}`);
            mediaFiles.forEach(f => console.log(`   - ${f}`));
        }
        
        const actualFile = mediaFiles[0];
        const actualBaseName = actualFile.replace(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/i, '');
        const actualExt = path.extname(actualFile).substring(1).toLowerCase();
        
        let needUpdate = false;
        let changes = [];
        
        // 检查1: name字段是否包含扩展名
        const nameHasExt = metadata.name !== actualBaseName;
        if (nameHasExt) {
            console.log(`[${index+1}/${infoDirs.length}] 🔧 name字段修复: ${infoDir}`);
            console.log(`   旧name: "${metadata.name}"`);
            console.log(`   新name: "${actualBaseName}"`);
            console.log(`   文件:   ${actualFile}`);
            
            metadata.name = actualBaseName;
            nameFixed++;
            needUpdate = true;
            changes.push('name');
        }
        
        // 检查2: ext字段是否匹配实际文件
        if (metadata.ext !== actualExt) {
            console.log(`[${index+1}/${infoDirs.length}] 🔧 ext字段修复: ${infoDir}`);
            console.log(`   旧ext: "${metadata.ext}"`);
            console.log(`   新ext: "${actualExt}"`);
            console.log(`   文件:  ${actualFile}`);
            
            metadata.ext = actualExt;
            extFixed++;
            needUpdate = true;
            changes.push('ext');
        }
        
        // 如果需要更新，写回文件
        if (needUpdate) {
            // 备份原文件
            const backupPath = metadataPath + '.backup';
            fs.copyFileSync(metadataPath, backupPath);
            
            // 更新时间戳
            metadata.modificationTime = Date.now();
            metadata.lastModified = Date.now();
            
            // 写入新内容
            fs.writeFileSync(metadataPath, JSON.stringify(metadata, null, 4), 'utf8');
            
            console.log(`   ✅ 已更新: ${changes.join(', ')}`);
            console.log(`   ✅ 备份: metadata.json.backup\n`);
        }
        
    } catch (error) {
        errors++;
        console.error(`[${index+1}/${infoDirs.length}] ❌ 处理失败: ${infoDir}`, error.message);
    }
});

console.log('\n================================');
console.log('📊 修复统计:');
console.log(`   检查目录: ${totalChecked}`);
console.log(`   name修复: ${nameFixed}`);
console.log(`   ext修复:  ${extFixed}`);
console.log(`   错误:     ${errors}`);
console.log('================================\n');

if (nameFixed > 0 || extFixed > 0) {
    console.log('💡 提示：');
    console.log('   - 元数据已修复');
    console.log('   - 请重启Eagle查看效果');
    console.log('   - 备份文件已保存为 metadata.json.backup');
}

console.log('\n✅ 修复完成！');
