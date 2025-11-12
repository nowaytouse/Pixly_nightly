/**
 * PIXLY AVIF 缩略图生成器
 * 支持静态和动画 AVIF 文件
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

/**
 * 生成 AVIF 缩略图
 * @param {string} filePath - AVIF 文件路径
 * @param {number} size - 缩略图大小
 * @returns {Promise<Buffer>} PNG 缩略图数据
 */
async function generateThumbnail(filePath, size = 400) {
    console.log(`[AVIF Thumbnail] Generating for: ${filePath}`);
    
    try {
        // 检查文件是否存在
        if (!fs.existsSync(filePath)) {
            throw new Error(`File not found: ${filePath}`);
        }
        
        // 使用 avifdec 转换为 PNG
        const tempPng = path.join(
            path.dirname(filePath),
            `_pixly_temp_${Date.now()}.png`
        );
        
        // 调用 avifdec 解码器
        await decodeToPng(filePath, tempPng);
        
        // 读取 PNG 数据
        const pngData = fs.readFileSync(tempPng);
        
        // 清理临时文件
        try {
            fs.unlinkSync(tempPng);
        } catch (e) {
            console.warn('[AVIF Thumbnail] Failed to clean temp file:', e);
        }
        
        console.log(`[AVIF Thumbnail] Generated successfully (${pngData.length} bytes)`);
        return pngData;
        
    } catch (error) {
        console.error('[AVIF Thumbnail] Error:', error);
        
        // 返回占位符图像
        return generatePlaceholder(size);
    }
}

/**
 * 使用 avifdec 解码 AVIF 到 PNG
 */
function decodeToPng(input, output) {
    return new Promise((resolve, reject) => {
        // 查找 avifdec 工具
        const avifdecPath = findAvifdec();
        
        if (!avifdecPath) {
            reject(new Error('avifdec not found'));
            return;
        }
        
        const args = [
            input,
            output,
            // avifdec 会自动选择合适的解码器
        ];
        
        const process = spawn(avifdecPath, args);
        
        let stderr = '';
        let stdout = '';
        
        process.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        process.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        process.on('close', (code) => {
            if (code === 0) {
                resolve();
            } else {
                reject(new Error(`avifdec failed (code ${code}): ${stderr}`));
            }
        });
        
        // 超时处理
        setTimeout(() => {
            process.kill();
            reject(new Error('avifdec timeout'));
        }, 10000);
    });
}

/**
 * 查找 avifdec 工具
 */
function findAvifdec() {
    const paths = [
        '/opt/homebrew/bin/avifdec',
        '/usr/local/bin/avifdec',
        '/opt/local/bin/avifdec',
        '/usr/bin/avifdec',
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
