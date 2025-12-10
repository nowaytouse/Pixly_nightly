/**
 * 轉換上下文類型定義和工具函數
 */

/**
 * 建立轉換上下文
 * @param {string} inputExt 輸入檔案副檔名
 * @param {string} outputFormat 輸出格式
 * @param {boolean} isAnimated 是否為動畫檔案
 * @param {boolean} hasAlpha 是否有透明度通道
 * @returns {Object} 轉換上下文物件
 */
exports.createContext = (inputExt, outputFormat, isAnimated = false, hasAlpha = false) => ({
    in: {
        ext: inputExt.toLowerCase(),
        animated: isAnimated,
        hasAlpha: hasAlpha
    },
    out: {
        ext: outputFormat.toLowerCase(),
        wantAnimated: ['png', 'webp', 'gif', 'mp4', 'webm'].includes(outputFormat)
    }
});

/**
 * 轉換器類型枚舉
 */
exports.ConverterType = {
    CANVAS: 'canvas',
    FFMPEG: 'ffmpeg',
    HEIC: 'heic',
    JXL: 'jxl'
};