const BaseBuilder = require("./base");
const path = require('path');

// 策略類：靜態 PNG
class StaticPngStrategy {
    getCodecArgs() {
        return ['-c:v', 'png'];
    }

    getSizeExtension(options) {
        return [];
    }
}

// 策略類：動畫 APNG
class AnimatedPngStrategy {

    getCodecArgs() {
        return ['-c:v', 'apng', '-plays', '0'];
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

class PngBuilder extends BaseBuilder {
    constructor() {
        super();
        this._strategies = {
            static: new StaticPngStrategy(),
            animated: new AnimatedPngStrategy()
        };
    }

    buildArgs(src, dest, options) {
        this._strategy = this._selectStrategy(options);
        const baseArgs = super.buildArgs(src, dest, options);
        const codecArgs = this._strategy.getCodecArgs();
        return [...codecArgs, ...baseArgs];
    }

    getOutputPath(dest, options) {
        if (options.isAnimated) {
            return path.join(dest, `${options.fileName}.apng`);
        }
        return path.join(dest, `${options.fileName}.${options.format}`);
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

module.exports = new PngBuilder();