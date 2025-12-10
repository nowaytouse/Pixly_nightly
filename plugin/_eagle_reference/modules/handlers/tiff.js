const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class TIFF extends BaseHandler {
    constructor() {
        super('tiff');
    }

    /**
     * TIFF 的預設轉換策略：優先 FFmpeg，失敗時用 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new TIFF();