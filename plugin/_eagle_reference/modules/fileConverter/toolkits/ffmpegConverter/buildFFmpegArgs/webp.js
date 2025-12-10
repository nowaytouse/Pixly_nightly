const BaseBuilder = require("./base");

// 策略類：靜態 WebP
class StaticWebpStrategy {
    getCodecArgs() {
        return ['-c:v', 'libwebp'];
    }

    getQualityArgs(quality) {
        return quality ? ['-q:v', Math.round(quality)] : [];
    }

    getSizeExtension(options) {
        return [];
    }
}

// 策略類：動畫 WebP
class AnimatedWebpStrategy {
    getCodecArgs() {
        return ['-c:v', 'libwebp_anim', '-lossless', '1', '-pix_fmt', 'rgb24', '-loop', '0'];
    }

    getQualityArgs(quality) {
        return quality ? ['-qscale', Math.round(quality)] : [];
    }

    getSizeExtension(options) {
        if (options.animatedFps && options.animatedFps !== 'sameAsSource') {
            return ['-r', options.animatedFps];
        }
        return [];
    }
}

class WebpBuilder extends BaseBuilder {
    constructor() {
        super();
        // 策略實例（可重用，無狀態）
        this._strategies = {
            static: new StaticWebpStrategy(),
            animated: new AnimatedWebpStrategy()
        };
    }

    buildArgs(src, dest, options) {
        // 選擇策略
        this._strategy = this._selectStrategy(options);

        // 使用父類的標準流程，但加入 codec 參數
        const baseArgs = super.buildArgs(src, dest, options);
        const codecArgs = this._strategy.getCodecArgs();

        // Codec 參數必須在最前面
        return [...codecArgs, ...baseArgs];
    }

    // 覆寫父類方法，使用策略
    getQualityArgs(options) {
        return this._strategy.getQualityArgs(options.quality);
    }

    getSizeArgs(options) {
        const baseArgs = super.getSizeFilter(options) || [];
        const extension = this._strategy.getSizeExtension(options);
        return [...baseArgs, ...extension];
    }

    // 選擇策略
    _selectStrategy(options) {
        return this._strategies[options.isAnimated ? 'animated' : 'static'];
    }
}

module.exports = new WebpBuilder();