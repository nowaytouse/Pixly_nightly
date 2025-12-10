const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class PNG extends BaseHandler {
    constructor() {
        super('png');
    }

    /**
     * PNG 的預設轉換策略：優先 Canvas（快），失敗時 FFmpeg
     */
    getDefaultPlan(ctx) {

        if (ctx.in.animated) {
            return [ConverterType.FFMPEG];
        }

        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}

module.exports = new PNG();