const fs = require('fs').promises;
const path = require('path');
const { validateOutputFile, getOutputPath } = require('../../validate');
const { progressiveScale, calculateNewDimensions } = require('../../../utils/canvasScaler');

class CanvasConverter {
    constructor() {
        // 支援的輸出格式 (只有瀏覽器 Canvas 原生支援的格式)
        this.supportedFormats = [
            "jpeg",
            "jpg",
            "png",
            "webp",
        ];

        // 不支援透明背景的格式
        this.nonTransparentFormats = [
            "jpeg",
            "jpg",
            "bmp",
            "tiff",
            "tif"
        ];

    }

    /**
     * 檢查是否支援指定的輸出格式
     * @param {string} format 
     * @returns {boolean}
     */
    isSupportedFormat(format) {
        return this.supportedFormats.includes(format.toLowerCase());
    }

    /**
     * 檢查格式是否支援透明
     * @param {string} format 
     * @returns {boolean}
     */
    isTransparentFormat(format) {
        return !this.nonTransparentFormats.includes(format.toLowerCase());
    }

    /**
     * 轉換單個圖片
     * @param {string} src 來源圖片路徑
     * @param {Object} options 輸出設置
     * @param {string} options.output 輸出目錄路徑
     * @param {string} options.format 輸出格式
     * @param {number} options.quality 輸出品質 (1-100)
     * @param {string} options.fileName 新檔名
     * @param {string} options.sizeType 尺寸調整類型
     * @param {number} options.sizeValue 尺寸值
     * @param {number} options.width 指定寬度
     * @param {number} options.height 指定高度
     * @returns {Promise<void>}
     */
    async convert(src, dest, options) {
        // 驗證輸入參數
        if (!src) throw new Error('sourceMissing');
        if (!dest) throw new Error('outputMissing');
        if (!options.format) throw new Error('formatMissing');
        if (!options.fileName) throw new Error('fileNameMissing');
        if (!this.isSupportedFormat(options.format)) {
            throw new Error('Export format not supported');
        }

        // 使用原生 Canvas 進行轉換
        try {
            await convertWithBrowserCanvas(src, dest, options, this);
        } catch (error) {
            console.log(error);
            throw error;
        }
    }

}

// 導出單例實例
module.exports = new CanvasConverter();

// 使用原生 Canvas 處理圖片轉換
async function convertWithBrowserCanvas(src, dest, options, converter) {
    return new Promise(async (resolve, reject) => {
        try {
            // 檢查是否在 Electron 渲染進程中
            const isRenderer = process && process.type === 'renderer';

            if (!isRenderer) {
                // 如果不在渲染進程中，無法使用原生 Canvas
                reject(new Error(i18next.t('error.canvasRendererRequired')));
                return;
            }

            // 創建原生 Canvas 元素
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');

            // 設置高品質渲染參數
            ctx.imageSmoothingEnabled = true;
            ctx.imageSmoothingQuality = 'high';

            // 創建 Image 對象
            const img = new Image();

            // 處理跨域問題（如果需要）
            img.crossOrigin = 'anonymous';

            // 當圖片載入完成後
            img.onload = async () => {
                try {
                    // 設置 Canvas 尺寸
                    let imgWidth = img.width;
                    let imgHeight = img.height;

                    // 如果是 SVG 且沒有內在尺寸，使用默認尺寸或從 viewBox 解析
                    if (src.toLowerCase().endsWith('.svg')) {
                        // 嘗試從 DataURL 中解析 SVG 的 viewBox
                        const svgContent = atob(img.src.split(',')[1]);
                        const viewBoxMatch = svgContent.match(/viewBox="([^"]+)"/);
                        if (viewBoxMatch) {
                            const [, , width, height] = viewBoxMatch[1].split(/\s+/).map(Number);
                            imgWidth = width || 24;
                            imgHeight = height || 24;
                        } else {
                            // 嘗試從 width/height 屬性解析
                            const widthMatch = svgContent.match(/width="([^"]+)"/);
                            const heightMatch = svgContent.match(/height="([^"]+)"/);
                            imgWidth = widthMatch ? parseInt(widthMatch[1]) : 24;
                            imgHeight = heightMatch ? parseInt(heightMatch[1]) : 24;
                        }
                    }

                    canvas.width = imgWidth;
                    canvas.height = imgHeight;

                    // 根據尺寸設置調整 Canvas 大小
                    if (options.sizeType && options.sizeValue) {
                        const newDimensions = calculateNewDimensions(imgWidth, imgHeight, options);
                        canvas.width = newDimensions.width;
                        canvas.height = newDimensions.height;
                    } else if (options.width && options.height) {
                        canvas.width = options.width;
                        canvas.height = options.height;
                    }                    

                    // 為不支援透明的格式設置白色背景
                    if (options.backgroundColor) {
                        ctx.fillStyle = options.backgroundColor;
                        ctx.fillRect(0, 0, canvas.width, canvas.height);
                    } else if (!converter.isTransparentFormat(options.format)) {
                        // 只對不支援透明的格式設置白色背景
                        ctx.fillStyle = '#ffffff';
                        ctx.fillRect(0, 0, canvas.width, canvas.height);
                    }

                    // 使用漸進式縮放來獲得高品質的圖像
                    if (imgWidth !== canvas.width || imgHeight !== canvas.height) {
                        // 創建原始尺寸的 canvas
                        const originalCanvas = document.createElement('canvas');
                        originalCanvas.width = imgWidth;
                        originalCanvas.height = imgHeight;
                        const originalCtx = originalCanvas.getContext('2d');
                        originalCtx.drawImage(img, 0, 0);
                        
                        // 使用漸進式縮放
                        const scaledCanvas = progressiveScale(originalCanvas, canvas.width, canvas.height);
                        ctx.drawImage(scaledCanvas, 0, 0);
                    } else {
                        // 尺寸相同，直接繪製
                        ctx.drawImage(img, 0, 0);
                    }

                    // 轉換為所需格式
                    let mimeType = 'image/png';
                    let fileExt = 'png';

                    switch (options.format.toLowerCase()) {
                        case 'jpeg':
                        case 'jpg':
                            mimeType = 'image/jpeg';
                            fileExt = 'jpg';
                            break;
                        case 'png':
                            mimeType = 'image/png';
                            fileExt = 'png';
                            break;
                        case 'webp':
                            mimeType = 'image/webp';
                            fileExt = 'webp';
                            break;
                        case 'bmp':
                        case 'tif':
                        case 'tiff':
                        case 'gif':
                            // 這些格式瀏覽器 Canvas 不支援，拋出錯誤讓 FFmpeg 處理
                            throw new Error(i18next.t('error.canvasDoesNotSupport', { format: options.format }));
                        default:
                            // 未知格式，拋出錯誤
                            throw new Error(i18next.t('error.unsupportedFormat', { format: options.format }));
                    }

                    try {
                        // 轉換為 DataURL
                        const dataURL = canvas.toDataURL(mimeType, options.quality / 100);

                        // 檢查是否成功轉換為指定格式
                        if (dataURL.indexOf(`data:${mimeType}`) !== 0) {
                            console.warn(`Failed to convert to ${options.format}, format not supported by browser`);
                            // 如果轉換失敗，嘗試使用 PNG 格式
                            const pngDataURL = canvas.toDataURL('image/png');
                            const pngBase64Data = pngDataURL.replace(/^data:image\/\w+;base64,/, '');
                            const pngBuffer = Buffer.from(pngBase64Data, 'base64');

                            // 確保目錄存在
                            await fs.mkdir(dest, { recursive: true });

                            // 寫入 PNG 文件
                            const outputPath = path.join(dest, `${options.fileName}.png`);
                            await fs.writeFile(outputPath, pngBuffer);

                            resolve();
                            return;
                        }

                        // 轉換 DataURL 為 Buffer
                        const base64Data = dataURL.replace(/^data:image\/\w+;base64,/, '');
                        const buffer = Buffer.from(base64Data, 'base64');

                        // 確保目錄存在
                        await fs.mkdir(dest, { recursive: true });

                        // 寫入文件
                        const outputPath = path.join(dest, `${options.fileName}.${fileExt}`);
                        await fs.writeFile(outputPath, buffer);

                        // 驗證輸出文件
                        await validateOutputFile(outputPath, {
                            minSize: 1,
                            timeout: 5000
                        });

                        resolve();
                    } catch (convError) {
                        console.error('Format conversion error:', convError);

                        // 如果轉換失敗，嘗試使用 PNG 格式
                        const pngDataURL = canvas.toDataURL('image/png');
                        const pngBase64Data = pngDataURL.replace(/^data:image\/\w+;base64,/, '');
                        const pngBuffer = Buffer.from(pngBase64Data, 'base64');

                        // 確保目錄存在
                        await fs.mkdir(dest, { recursive: true });

                        // 寫入 PNG 文件
                        const outputPath = path.join(dest, `${options.fileName}.png`);
                        await fs.writeFile(outputPath, pngBuffer);

                        // 驗證輸出文件
                        await validateOutputFile(outputPath, {
                            minSize: 1,
                            timeout: 5000
                        });

                        resolve();
                    }
                } catch (error) {
                    reject(error);
                }
            };

            // 處理圖片載入錯誤
            img.onerror = (error) => {
                reject(new Error(i18next.t('error.failedToLoadImage', { error: error })));
            };

            // 載入圖片
            if (src.startsWith('data:')) {
                // 如果已經是 DataURL，直接使用
                img.src = src;
            } else {
                // 如果是文件路徑，需要先讀取文件
                const fs = require('fs').promises;
                const data = await fs.readFile(src);

                // 轉換為 DataURL
                let dataToUse = data;

                // 如果是 SVG，處理 currentColor 和顯示問題
                if (src.toLowerCase().endsWith('.svg')) {
                    let svgContent = data.toString('utf8');

                    // 將 currentColor 替換為黑色，確保在白色背景上可見
                    svgContent = svgContent.replace(/stroke="currentColor"/g, 'stroke="#000000"');
                    svgContent = svgContent.replace(/fill="currentColor"/g, 'fill="#000000"');

                    // 對於只有描邊的 SVG，增加描邊寬度以確保可見性
                    // 檢查是否有 fill="none" 而只有 stroke
                    if (svgContent.includes('fill="none"') && svgContent.includes('stroke=')) {
                        // 增加描邊寬度，最小為 2
                        svgContent = svgContent.replace(/stroke-width="([^"]+)"/g, (match, width) => {
                            const newWidth = Math.max(2, parseFloat(width) * 1.5);
                            return `stroke-width="${newWidth}"`;
                        });
                    }

                    dataToUse = Buffer.from(svgContent, 'utf8');
                }

                const base64 = dataToUse.toString('base64');
                const ext = src.split('.').pop().toLowerCase();
                const mimeType = ext === 'svg' ? 'image/svg+xml' : `image/${ext}`;
                img.src = `data:${mimeType};base64,${base64}`;
            }
        } catch (error) {
            reject(error);
        }
    });
}

