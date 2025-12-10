const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class INSP extends BaseHandler {
    constructor() {
        super('insp');
    }

    /**
     * INSP 的預設轉換策略：優先 FFmpeg（更好支援），失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new INSP();