/**
 * Canvas 圖像縮放工具模組
 * 提供高品質的漸進式圖像縮放功能，避免縮放鋸齒
 */

/**
 * 使用漸進式縮放方法將圖像從來源尺寸縮放到目標尺寸
 * 當縮放比例大於 2:1 時，會使用多步驟縮放以獲得更好的品質
 * 
 * @param {HTMLCanvasElement|HTMLImageElement} source - 來源圖像（可以是 canvas 或 img 元素）
 * @param {number} targetWidth - 目標寬度
 * @param {number} targetHeight - 目標高度
 * @param {Object} options - 可選參數
 * @param {boolean} options.imageSmoothingEnabled - 是否啟用圖像平滑（預設：true）
 * @param {string} options.imageSmoothingQuality - 圖像平滑品質 'low' | 'medium' | 'high'（預設：'high'）
 * @returns {HTMLCanvasElement} - 包含縮放後圖像的 canvas 元素
 */
function progressiveScale(source, targetWidth, targetHeight, options = {}) {
    const {
        imageSmoothingEnabled = true,
        imageSmoothingQuality = 'high'
    } = options;

    // 取得來源尺寸
    const sourceWidth = source.width;
    const sourceHeight = source.height;

    // 如果不需要縮放，直接返回
    if (sourceWidth === targetWidth && sourceHeight === targetHeight) {
        const canvas = document.createElement('canvas');
        canvas.width = targetWidth;
        canvas.height = targetHeight;
        const ctx = canvas.getContext('2d');
        ctx.drawImage(source, 0, 0);
        return canvas;
    }

    // 計算縮放比例
    const scaleX = targetWidth / sourceWidth;
    const scaleY = targetHeight / sourceHeight;
    const minScale = Math.min(scaleX, scaleY);

    // 創建最終輸出 canvas
    const outputCanvas = document.createElement('canvas');
    outputCanvas.width = targetWidth;
    outputCanvas.height = targetHeight;
    const outputCtx = outputCanvas.getContext('2d');
    outputCtx.imageSmoothingEnabled = imageSmoothingEnabled;
    outputCtx.imageSmoothingQuality = imageSmoothingQuality;

    // 如果是放大或小幅縮小（縮放比例 >= 0.5），直接縮放
    if (minScale >= 0.5) {
        outputCtx.drawImage(source, 0, 0, targetWidth, targetHeight);
        return outputCanvas;
    }

    // 大幅縮小時使用漸進式縮放
    // 每次縮小不超過 50%，以避免鋸齒
    let currentCanvas = document.createElement('canvas');
    let currentCtx = currentCanvas.getContext('2d');
    currentCtx.imageSmoothingEnabled = imageSmoothingEnabled;
    currentCtx.imageSmoothingQuality = imageSmoothingQuality;

    // 第一步：繪製原始圖像
    currentCanvas.width = sourceWidth;
    currentCanvas.height = sourceHeight;
    currentCtx.drawImage(source, 0, 0);

    let currentWidth = sourceWidth;
    let currentHeight = sourceHeight;

    // 多步驟縮小，每次縮小 50%，直到接近目標尺寸
    while (currentWidth > targetWidth * 1.5 || currentHeight > targetHeight * 1.5) {
        // 計算下一步的尺寸（縮小 50% 或到達目標尺寸）
        currentWidth = Math.max(targetWidth, Math.floor(currentWidth * 0.5));
        currentHeight = Math.max(targetHeight, Math.floor(currentHeight * 0.5));

        // 創建臨時 canvas
        const tempCanvas = document.createElement('canvas');
        tempCanvas.width = currentWidth;
        tempCanvas.height = currentHeight;
        const tempCtx = tempCanvas.getContext('2d');
        tempCtx.imageSmoothingEnabled = imageSmoothingEnabled;
        tempCtx.imageSmoothingQuality = imageSmoothingQuality;

        // 從當前 canvas 繪製到臨時 canvas
        tempCtx.drawImage(currentCanvas, 0, 0, currentWidth, currentHeight);

        // 更新當前 canvas
        currentCanvas = tempCanvas;
        currentCtx = tempCtx;
    }

    // 最後一步：從當前尺寸縮放到目標尺寸
    outputCtx.drawImage(currentCanvas, 0, 0, targetWidth, targetHeight);

    return outputCanvas;
}

/**
 * 在現有的 canvas 上使用漸進式縮放繪製圖像
 * 
 * @param {CanvasRenderingContext2D} ctx - 目標 canvas 的 2D 上下文
 * @param {HTMLCanvasElement|HTMLImageElement} source - 來源圖像
 * @param {number} x - 繪製位置 x 座標
 * @param {number} y - 繪製位置 y 座標
 * @param {number} width - 繪製寬度
 * @param {number} height - 繪製高度
 * @param {Object} options - 可選參數（同 progressiveScale）
 */
function drawImageWithProgressiveScale(ctx, source, x, y, width, height, options = {}) {
    // 使用漸進式縮放獲得高品質的縮放圖像
    const scaledCanvas = progressiveScale(source, width, height, options);
    
    // 將縮放後的圖像繪製到目標 canvas
    ctx.drawImage(scaledCanvas, x, y);
}

/**
 * 計算保持長寬比的新尺寸
 * 
 * @param {number} originalWidth - 原始寬度
 * @param {number} originalHeight - 原始高度
 * @param {Object} options - 尺寸選項
 * @param {string} options.sizeType - 尺寸類型
 * @param {number} options.sizeValue - 尺寸值
 * @param {number} options.width - 指定寬度（exact 模式）
 * @param {number} options.height - 指定高度（exact 模式）
 * @returns {{width: number, height: number}} - 計算後的新尺寸
 */
function calculateNewDimensions(originalWidth, originalHeight, options) {
    let newWidth = originalWidth;
    let newHeight = originalHeight;

    switch (options.sizeType) {
        case 'maxWidth':
            if (originalWidth > options.sizeValue) {
                newWidth = options.sizeValue;
                newHeight = Math.round(originalHeight * (options.sizeValue / originalWidth));
            }
            break;

        case 'maxHeight':
            if (originalHeight > options.sizeValue) {
                newHeight = options.sizeValue;
                newWidth = Math.round(originalWidth * (options.sizeValue / originalHeight));
            }
            break;

        case 'minWidth':
            if (originalWidth < options.sizeValue) {
                newWidth = options.sizeValue;
                newHeight = Math.round(originalHeight * (options.sizeValue / originalWidth));
            }
            break;

        case 'minHeight':
            if (originalHeight < options.sizeValue) {
                newHeight = options.sizeValue;
                newWidth = Math.round(originalWidth * (options.sizeValue / originalHeight));
            }
            break;

        case 'maxSide':
            const maxSide = Math.max(originalWidth, originalHeight);
            if (maxSide > options.sizeValue) {
                const ratio = options.sizeValue / maxSide;
                newWidth = Math.round(originalWidth * ratio);
                newHeight = Math.round(originalHeight * ratio);
            }
            break;

        case 'minSide':
            const minSide = Math.min(originalWidth, originalHeight);
            if (minSide < options.sizeValue) {
                const ratio = options.sizeValue / minSide;
                newWidth = Math.round(originalWidth * ratio);
                newHeight = Math.round(originalHeight * ratio);
            }
            break;

        case 'exact':
            if (options.width && options.height) {
                newWidth = options.width;
                newHeight = options.height;
            } else if (options.width) {
                newWidth = options.width;
                newHeight = Math.round(originalHeight * (options.width / originalWidth));
            } else if (options.height) {
                newHeight = options.height;
                newWidth = Math.round(originalWidth * (options.height / originalHeight));
            }
            break;
    }

    return { width: newWidth, height: newHeight };
}

// 導出函數
module.exports = {
    progressiveScale,
    drawImageWithProgressiveScale,
    calculateNewDimensions
};