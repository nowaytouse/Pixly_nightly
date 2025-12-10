const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class EXR extends BaseHandler {
    constructor() {
        super('exr');
    }

    /**
     * EXR 的預設轉換策略：優先 FFmpeg（更好支援高動態範圍），失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new EXR();