const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class BMP extends BaseHandler {
    constructor() {
        super('bmp');
    }

    /**
     * BMP 的預設轉換策略：優先 Canvas（快），失敗時 FFmpeg
     */
    getDefaultPlan(ctx) {
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }
}

module.exports = new BMP();