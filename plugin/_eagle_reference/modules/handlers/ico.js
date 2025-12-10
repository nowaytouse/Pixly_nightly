const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class ICO extends BaseHandler {
    constructor() {
        super('ico');
    }

    /**
     * ICO 的預設轉換策略：優先 FFmpeg（更好支援 ICO），失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new ICO();