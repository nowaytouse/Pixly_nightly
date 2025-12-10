const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class TGA extends BaseHandler {
    constructor() {
        super('tga');
    }

    /**
     * TGA 的預設轉換策略：優先 FFmpeg（更好支援 Targa），失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new TGA();