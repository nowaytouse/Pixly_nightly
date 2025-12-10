const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class GIF extends BaseHandler {
    constructor() {
        super('gif');
    }

    /**
     * 覆寫 getPlan 方法處理 WebP 特殊邏輯
     */
    getPlan(ctx) {
        if (ctx.in.animated) {
            return [ConverterType.FFMPEG];
        } else {
            return [ConverterType.CANVAS, ConverterType.FFMPEG];
        }
    }
}

module.exports = new GIF();