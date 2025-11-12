#!/usr/bin/env node
/**
 * 孤儿XMP清理工具
 * 删除所有没有对应媒体文件的XMP文件
 */

const fs = require('fs');
const path = require('path');

const EAGLE_LIBRARY = '/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library/images';

console.log('🧹 孤儿XMP清理工具');
console.log('================================\n');

// 扫描所有.info目录
const infoDirs = fs.readdirSync(EAGLE_LIBRARY)
    .filter(name => name.endsWith('.info'));

console.log(`📁 找到 ${infoDirs.length} 个.info目录\n`);

let totalXmp = 0;
let orphanXmp = 0;
let deletedXmp = 0;
let failedXmp = 0;

infoDirs.forEach((infoDir, index) => {
    const fullPath = path.join(EAGLE_LIBRARY, infoDir);
    
    try {
        const files = fs.readdirSync(fullPath);
        
        // 分类文件
        const mediaFiles = files.filter(f => {
            const lower = f.toLowerCase();
            return lower.match(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/) 
                   && !lower.includes('thumbnail');
        });
        
        const xmpFiles = files.filter(f => f.toLowerCase().endsWith('.xmp'));
        
        if (xmpFiles.length === 0) return;
        
        totalXmp += xmpFiles.length;
        
        xmpFiles.forEach(xmpFile => {
            const xmpPath = path.join(fullPath, xmpFile);
            const xmpBaseName = xmpFile.replace(/\.xmp$/i, '');
            
            // 检查是否有对应的媒体文件
            const hasMedia = mediaFiles.some(mediaFile => {
                const mediaBaseName = mediaFile.replace(/\.(jpg|jpeg|png|gif|webp|jxl|avif|bmp|tiff|heic|heif)$/i, '');
                return mediaBaseName === xmpBaseName;
            });
            
            if (!hasMedia) {
                orphanXmp++;
                console.log(`[${index + 1}/${infoDirs.length}] 🗑️  孤儿XMP: ${infoDir}`);
                console.log(`   文件: ${xmpFile}`);
                console.log(`   媒体文件数: ${mediaFiles.length}`);
                
                // 尝试删除
                try {
                    // 尝试1: 直接删除
                    try {
                        fs.unlinkSync(xmpPath);
                        deletedXmp++;
                        console.log(`   ✅ 已删除\n`);
                    } catch (deleteError) {
                        // 尝试2: 修改权限后删除
                        console.log(`   ⚠️ 第一次删除失败，尝试修改权限...`);
                        fs.chmodSync(xmpPath, 0o666);
                        fs.unlinkSync(xmpPath);
                        deletedXmp++;
                        console.log(`   ✅ 修改权限后已删除\n`);
                    }
                } catch (error) {
                    failedXmp++;
                    console.error(`   ❌ 删除失败: ${error.message}`);
                    console.error(`   文件信息:`, fs.statSync(xmpPath));
                    console.log('');
                }
            }
        });
        
    } catch (error) {
        console.error(`❌ 处理目录失败: ${infoDir}`, error.message);
    }
});

console.log('\n================================');
console.log('📊 清理统计:');
console.log(`   总XMP文件: ${totalXmp}`);
console.log(`   孤儿XMP: ${orphanXmp}`);
console.log(`   已删除: ${deletedXmp}`);
console.log(`   失败: ${failedXmp}`);
console.log('================================\n');

if (orphanXmp > 0) {
    console.log('💡 提示：');
    console.log('   - 孤儿XMP是指没有对应媒体文件的XMP文件');
    console.log('   - 这些文件可能是之前转换后遗留的');
    console.log('   - 已尝试删除所有孤儿XMP文件');
    
    if (failedXmp > 0) {
        console.log('\n⚠️ 警告：');
        console.log(`   - 有 ${failedXmp} 个文件删除失败`);
        console.log('   - 请检查文件权限或手动删除');
    }
}

console.log('\n✅ 清理完成！');
