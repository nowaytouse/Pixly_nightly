#!/usr/bin/env node
/**
 * AVIF缩略图修复工具
 * 为所有AVIF文件重新生成缩略图
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

const EAGLE_LIBRARY = '/Users/nyamiiko/Documents/git/plxy-easy2jxlavif/1.library/images';

console.log('🖼️  AVIF缩略图修复工具');
console.log('================================\n');

// 检查必要的工具
function checkTools() {
    const tools = ['ffmpeg', 'convert']; // ImageMagick's convert
    const available = [];
    
    tools.forEach(tool => {
        try {
            const result = spawn('which', [tool]);
            result.on('close', (code) => {
                if (code === 0) {
                    available.push(tool);
                }
            });
        } catch (e) {
            // Tool not available
        }
    });
    
    return available;
}

// 使用ffmpeg生成缩略图
function generateThumbnailFFmpeg(inputPath, outputPath) {
    return new Promise((resolve, reject) => {
        const ffmpeg = spawn('ffmpeg', [
            '-i', inputPath,
            '-vframes', '1',
            '-vf', 'scale=800:-1',
            '-y',
            outputPath
        ]);
        
        let stderr = '';
        ffmpeg.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        ffmpeg.on('close', (code) => {
            if (code === 0) {
                resolve();
            } else {
                reject(new Error(`FFmpeg failed: ${stderr}`));
            }
        });
    });
}

// 扫描所有.info目录
const infoDirs = fs.readdirSync(EAGLE_LIBRARY)
    .filter(name => name.endsWith('.info'));

console.log(`📁 找到 ${infoDirs.length} 个.info目录\n`);

let totalAvif = 0;
let processedAvif = 0;
let failedAvif = 0;

async function processDirectory(infoDir, index) {
    const fullPath = path.join(EAGLE_LIBRARY, infoDir);
    
    try {
        const files = fs.readdirSync(fullPath);
        
        // 找到AVIF文件
        const avifFiles = files.filter(f => {
            const lower = f.toLowerCase();
            return lower.endsWith('.avif') && !lower.includes('thumbnail');
        });
        
        if (avifFiles.length === 0) return;
        
        totalAvif += avifFiles.length;
        
        for (const avifFile of avifFiles) {
            const avifPath = path.join(fullPath, avifFile);
            const baseName = avifFile.replace(/\.avif$/i, '');
            const thumbnailPath = path.join(fullPath, `${baseName}_thumbnail.png`);
            
            console.log(`[${index + 1}/${infoDirs.length}] 🖼️  处理: ${infoDir}`);
            console.log(`   文件: ${avifFile}`);
            
            // 检查缩略图是否已存在
            const thumbnailExists = fs.existsSync(thumbnailPath);
            console.log(`   缩略图状态: ${thumbnailExists ? '存在' : '不存在'}`);
            
            // 如果缩略图不存在，或者文件很旧，就重新生成
            if (!thumbnailExists) {
                try {
                    console.log(`   🔄 正在生成缩略图...`);
                    await generateThumbnailFFmpeg(avifPath, thumbnailPath);
                    processedAvif++;
                    console.log(`   ✅ 缩略图已生成\n`);
                } catch (error) {
                    failedAvif++;
                    console.error(`   ❌ 生成失败: ${error.message}\n`);
                }
            } else {
                console.log(`   ℹ️  缩略图已存在，跳过\n`);
            }
        }
        
    } catch (error) {
        console.error(`❌ 处理目录失败: ${infoDir}`, error.message);
    }
}

// 主函数
async function main() {
    console.log('🔍 检查必要的工具...');
    console.log('   需要: ffmpeg (用于生成缩略图)\n');
    
    // 检查ffmpeg
    try {
        const { exec } = require('child_process');
        const util = require('util');
        const execPromise = util.promisify(exec);
        await execPromise('which ffmpeg');
        console.log('✅ ffmpeg 可用\n');
    } catch (error) {
        console.error('❌ 未找到 ffmpeg，请先安装：');
        console.error('   brew install ffmpeg\n');
        process.exit(1);
    }
    
    console.log('🚀 开始处理...\n');
    
    // 处理所有目录
    for (let i = 0; i < infoDirs.length; i++) {
        await processDirectory(infoDirs[i], i);
    }
    
    console.log('\n================================');
    console.log('📊 处理统计:');
    console.log(`   找到AVIF文件: ${totalAvif}`);
    console.log(`   已生成缩略图: ${processedAvif}`);
    console.log(`   失败: ${failedAvif}`);
    console.log('================================\n');
    
    if (processedAvif > 0) {
        console.log('💡 提示：');
        console.log('   - 缩略图已生成，请在Eagle中刷新查看');
        console.log('   - 可能需要重启Eagle才能看到变化');
    }
    
    console.log('\n✅ 处理完成！');
}

main().catch(error => {
    console.error('❌ 处理失败:', error);
    process.exit(1);
});
