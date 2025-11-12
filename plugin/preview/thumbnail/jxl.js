/**
 * PIXLY JXL 缩略图生成器
 * 支持静态和动画 JXL 文件
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

/**
 * 生成 JXL 缩略图
 * @param {string} filePath - JXL 文件路径
 * @param {number} size - 缩略图大小
 * @returns {Promise<Buffer>} PNG 缩略图数据
 */
async function generateThumbnail(filePath, size = 400) {
    console.log(`[JXL Thumbnail] Generating for: ${filePath}`);
    
    try {
        // 检查文件是否存在
        if (!fs.existsSync(filePath)) {
            throw new Error(`File not found: ${filePath}`);
        }
        
        // 使用 djxl (JPEG XL decoder) 转换为 PNG
        const tempPng = path.join(
            path.dirname(filePath),
            `_pixly_temp_${Date.now()}.png`
        );
        
        // 调用 djxl 解码器
        await decodeToPng(filePath, tempPng, size);
        
        // 读取 PNG 数据
        const pngData = fs.readFileSync(tempPng);
        
        // 清理临时文件
        try {
            fs.unlinkSync(tempPng);
        } catch (e) {
            console.warn('[JXL Thumbnail] Failed to clean temp file:', e);
        }
        
        console.log(`[JXL Thumbnail] Generated successfully (${pngData.length} bytes)`);
        return pngData;
        
    } catch (error) {
        console.error('[JXL Thumbnail] Error:', error);
        
        // 返回占位符图像
        return generatePlaceholder(size);
    }
}

/**
 * 使用 djxl 解码 JXL 到 PNG
 */
function decodeToPng(input, output, size) {
    return new Promise((resolve, reject) => {
        // 查找 djxl 工具
        const djxlPath = findDjxl();
        
        if (!djxlPath) {
            reject(new Error('djxl not found'));
            return;
        }
        
        // 对于动画 JXL，使用 APNG 格式保留动画
        // 静态 JXL 解码为 PNG
        const args = [
            input,
            output,
            '--output_format=apng',  // 明确指定 APNG 格式
            '--output_frames'        // 输出所有帧
        ];
        
        const process = spawn(djxlPath, args);
        
        let stderr = '';
        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        process.on('close', (code) => {
            if (code === 0) {
                resolve();
            } else {
                reject(new Error(`djxl failed (code ${code}): ${stderr}`));
            }
        });
        
        // 超时处理
        setTimeout(() => {
            process.kill();
            reject(new Error('djxl timeout'));
        }, 10000);
    });
}

/**
 * 查找 djxl 工具
 */
function findDjxl() {
    const paths = [
        '/opt/homebrew/bin/djxl',
        '/usr/local/bin/djxl',
        '/opt/local/bin/djxl',
        '/usr/bin/djxl',
    ];
    
    for (const p of paths) {
        if (fs.existsSync(p)) {
            return p;
        }
    }
    
    return null;
}

/**
 * 生成占位符图像
 */
function generatePlaceholder(size) {
    // 简单的 1x1 灰色 PNG
    const placeholder = Buffer.from(
        'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mN89uzZfwAJxQO00B4Y7wAAAABJRU5ErkJggg==',
        'base64'
    );
    return placeholder;
}

// Eagle 插件入口
module.exports = async function(filePath, size) {
    return await generateThumbnail(filePath, size);
};
