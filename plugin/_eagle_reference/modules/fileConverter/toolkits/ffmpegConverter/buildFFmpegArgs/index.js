// 格式別名映射 - 讓多個副檔名使用同一個 handler
const formatAliases = {
    'jpeg': 'jpg',
    'jpe': 'jpg',
    'jfif': 'jpg',
    'tif': 'tiff',
    // 可以在這裡添加更多別名映射
};

function buildFFmpegArgs(src, dest, options) {
    // 取得實際的格式名稱（處理別名）
    const format = options.format.toLowerCase();
    const actualFormat = formatAliases[format] || format;
    
    try {
        const builder = require(`./${actualFormat}`);
        return builder.buildArgs(src, dest, options);
    } catch (error) {
        // 如果找不到對應的 builder，拋出更友善的錯誤訊息
        if (error.code === 'MODULE_NOT_FOUND') {
            throw new Error(`Unsupported format: ${options.format}`);
        }
        throw error;
    }
}

module.exports = buildFFmpegArgs;