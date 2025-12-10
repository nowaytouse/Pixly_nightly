const BaseBuilder = require("./base");
const path = require('path');

// 策略類：GIF 輸入（保留動畫）
class staticGifStrategy {
    getCodecArgs() {
        return ['-vcodec', 'gif', '-loop', '0'];
    }

    getFrameRateArgs(options) {
        if (options.animatedFps && options.animatedFps !== 'sameAsSource') {
            return ['-r', options.animatedFps];
        }
        return [];
    }
}

// 策略類：非 GIF 輸入（生成靜態 GIF）
class animatedGifStrategy {
    getCodecArgs() {
        return ['-vcodec', 'gif'];
    }

    getFrameRateArgs(options) {
        // 非 GIF 輸入不需要幀率設定
        return [];
    }
}

class GifBuilder extends BaseBuilder {
    constructor() {
        super();
        // 策略實例（可重用，無狀態）
        this._strategies = {
            static: new staticGifStrategy(),
            animated: new animatedGifStrategy()
        };
    }

    buildArgs(src, dest, options) {
        // 選擇策略
        this._strategy = this._selectStrategy(options);

        this.args = [];

        // 添加 codec 參數
        this.args.push(...this._strategy.getCodecArgs());

        // 添加幀率參數
        this.args.push(...this._strategy.getFrameRateArgs(options));

        // 處理品質和濾鏡
        this.args.push(...this._getQualityAndFilterArgs(options));

        // 添加輸出路徑
        this.args.push(this.getOutputPath(dest, options));

        return this.args.filter(arg => arg);
    }

    // 選擇策略
    _selectStrategy(options) {
        const isAnimated = options.isAnimated;
        return this._strategies[isAnimated ? 'animated' : 'static'];
    }

    // 處理品質和濾鏡邏輯
    _getQualityAndFilterArgs(options) {
        const args = [];

        const colors = Math.round((options.quality / 100) * 256);
        const sizeFilter = this.getSizeFilter(options);

        if (sizeFilter) {
            // 如果有尺寸調整，需要合併濾鏡            
            args.push('-filter_complex', `[0:v]${sizeFilter}[processed];[processed]split[a][b];[a]palettegen=max_colors=${colors}[p];[b][p]paletteuse`);
        } else {
            args.push('-filter_complex', `split[a][b];[a]palettegen=max_colors=${colors}[p];[b][p]paletteuse`);
        }

        return args;
    }

}

module.exports = new GifBuilder();