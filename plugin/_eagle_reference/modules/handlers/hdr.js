const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class HDR extends BaseHandler {
    constructor() {
        super('hdr');
    }

    /**
     * HDR 的預設轉換策略：優先 FFmpeg（專業 HDR 支援），失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new HDR();