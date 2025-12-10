const path = require('path');
const { validateOutputFile, getOutputPath } = require('../../validate');
const { progressiveScale, calculateNewDimensions } = require('../../../utils/canvasScaler');

class jxlDecoder {
    constructor() {
        this.jxl2canvas = require('./jxl2canvas');
    }

    /**
     * JXL 解碼器轉換方法 - 將 JXL 解碼為 PNG
     * @param {string} src 來源 JXL 檔案路徑
     * @param {string} dest 目標資料夾路徑
     * @param {Object} options 轉換選項
     * @returns {Promise<Object>} 轉換結果
     */
    async convert(src, dest, options) {
        try {
            // JXL 解碼器只支援輸出為 PNG
            const outputFormat = options.format || 'png';
            if (outputFormat.toLowerCase() !== 'png') {
                throw new Error('JXL decoder only supports PNG output format');
            }

            const outputPath = path.join(dest, `${options.fileName}.png`);
            
            // 使用 jxl2canvas 解碼 JXL
            const plugin = {
                path: path.dirname(__filename)
            };
            
            const result = await this.jxl2canvas(src, plugin);
            if (!result || !result.canvas) {
                throw new Error('Failed to decode JXL image');
            }

            const canvas = result.canvas;
            let finalWidth = canvas.width;
            let finalHeight = canvas.height;
            
            // 處理尺寸調整
            if (options.sizeType) {
                const newDimensions = calculateNewDimensions(canvas.width, canvas.height, options);
                finalWidth = newDimensions.width;
                finalHeight = newDimensions.height;
                
                // 如果需要調整尺寸，使用漸進式縮放
                if (finalWidth !== canvas.width || finalHeight !== canvas.height) {
                    const resizedCanvas = progressiveScale(canvas, finalWidth, finalHeight);
                    canvas.width = finalWidth;
                    canvas.height = finalHeight;
                    canvas.getContext('2d').drawImage(resizedCanvas, 0, 0);
                }
            }

            // 設定品質（PNG 不支援品質設定，但保持介面一致性）
            let quality = 1.0; // PNG 是無損格式
            
            // 轉換為 PNG 並儲存
            const dataUrl = canvas.toDataURL('image/png', quality);
            const base64Data = dataUrl.split(',')[1];
            const buffer = Buffer.from(base64Data, 'base64');
            
            const fs = require('fs').promises;
            await fs.writeFile(outputPath, buffer);
            
            // 驗證輸出文件
            await validateOutputFile(outputPath, {
                minSize: 1,
                timeout: 5000
            });

            return {
                success: true,
                outputPath
            };

        } catch (error) {
            throw new Error(`JXL decode failed: ${error.message}`);
        }
    }

}

module.exports = new jxlDecoder();