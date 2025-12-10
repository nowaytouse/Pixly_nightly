const BaseBuilder = require("./base");

class IcoBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const baseArgs = super.buildArgs(src, dest, options);
        this.args = [
            '-pix_fmt', 'bgra',
            ...baseArgs
        ]

        return this.args;
    }

    /**
     * 獲取尺寸調整濾鏡（覆寫父類方法以處理 ICO 的標準尺寸限制）
     * 組合 scale 和 pad 濾鏡，確保輸出為正方形且保持透明背景
     */
    getSizeFilter(options) {
        if (options.sizeType === 'original') {
            // 對於 original 類型，使用動態選擇邏輯
            return this._getOriginalSizeFilterWithPadding();
        }

        const standardSize = this._getOptimalIcoSize(options);

        if (standardSize) {
            return this._buildScaleAndPadFilter(standardSize);
        }

        return null;
    }

    /**
     * 為 original 類型生成動態尺寸選擇濾鏡（含透明背景填充）
     */
    _getOriginalSizeFilterWithPadding() {
        // 使用 FFmpeg 表達式在運行時選擇最合適的標準尺寸
        // 根據原圖的最大邊選擇最接近的標準尺寸
        const expr = `
            if(gte(max(iw\\,ih)\\,192)\\,256\\,
            if(gte(max(iw\\,ih)\\,96)\\,128\\,
            if(gte(max(iw\\,ih)\\,56)\\,64\\,
            if(gte(max(iw\\,ih)\\,40)\\,48\\,
            if(gte(max(iw\\,ih)\\,28)\\,32\\,
            if(gte(max(iw\\,ih)\\,20)\\,24\\,16))))))
        `.replace(/\s+/g, '');

        // 組合 scale 和 pad 濾鏡：先縮放到合適大小，再填充為正方形
        const targetSize = expr;
        return `scale='min(iw,${targetSize})':'min(ih,${targetSize})':flags=lanczos,pad=${targetSize}:${targetSize}:(ow-iw)/2:(oh-ih)/2:color=0x00000000`;
    }

    /**
     * 建構 scale + pad 濾鏡組合，實現透明背景填充
     * @param {number} targetSize - 目標正方形尺寸
     * @returns {string} 組合濾鏡字串
     * @private
     */
    _buildScaleAndPadFilter(targetSize) {
        // 先縮放到目標尺寸（保持比例），再用透明背景填充為正方形
        // scale: 縮放圖片，保持比例，不超過目標尺寸
        // pad: 填充為正方形，使用透明背景，居中對齊
        return `scale='min(iw,${targetSize})':'min(ih,${targetSize})':flags=lanczos,pad=${targetSize}:${targetSize}:(ow-iw)/2:(oh-ih)/2:color=0x00000000`;
    }

    /**
     * 根據不同的 sizeType 智能選擇最合適的 ICO 標準尺寸
     * @param {Object} options 尺寸配置選項
     * @returns {number|null} 最適合的標準尺寸，如果無法確定則返回 null
     */
    _getOptimalIcoSize(options) {
        // ICO 支持的標準尺寸
        const standardSizes = [16, 24, 32, 48, 64, 128, 256];

        switch (options.sizeType) {
            case 'maxWidth':
            case 'maxHeight':
            case 'maxSide':
                return this._findLargestSizeWithinLimit(standardSizes, options.sizeValue || 256);

            case 'minWidth':
            case 'minHeight':
            case 'minSide':
                return this._findSmallestSizeAboveLimit(standardSizes, options.sizeValue || 16);

            case 'exact':
                if (options.width && options.height) {
                    // 如果指定了寬高，選擇最接近的正方形尺寸
                    const targetSize = Math.max(options.width, options.height);
                    return this._findClosestSize(standardSizes, targetSize);
                } else if (options.width || options.height) {
                    // 如果只指定了一個維度，使用該值
                    const targetSize = options.width || options.height;
                    return this._findClosestSize(standardSizes, targetSize);
                }
                return 256; // 默認使用最大尺寸

            case 'original':
                // original 類型在 getSizeFilter 中單獨處理
                return null;

            default:
                return 256; // 默認使用最大標準尺寸
        }
    }

    /**
     * 找到不超過指定限制的最大標準尺寸
     */
    _findLargestSizeWithinLimit(standardSizes, limit) {
        for (let i = standardSizes.length - 1; i >= 0; i--) {
            if (standardSizes[i] <= limit) {
                return standardSizes[i];
            }
        }
        return standardSizes[0]; // 如果都超過限制，返回最小尺寸
    }

    /**
     * 找到大於等於指定限制的最小標準尺寸
     */
    _findSmallestSizeAboveLimit(standardSizes, limit) {
        for (let i = 0; i < standardSizes.length; i++) {
            if (standardSizes[i] >= limit) {
                return standardSizes[i];
            }
        }
        return standardSizes[standardSizes.length - 1]; // 如果都小於限制，返回最大尺寸
    }

    /**
     * 找到最接近目標尺寸的標準尺寸
     */
    _findClosestSize(standardSizes, targetSize) {
        let closestSize = standardSizes[0];
        let minDifference = Math.abs(targetSize - closestSize);

        for (let i = 1; i < standardSizes.length; i++) {
            const difference = Math.abs(targetSize - standardSizes[i]);
            if (difference < minDifference) {
                minDifference = difference;
                closestSize = standardSizes[i];
            }
        }

        return closestSize;
    }

    // 移除舊的 getSizeArgs 方法，完全使用父類的新架構

}

module.exports = new IcoBuilder();