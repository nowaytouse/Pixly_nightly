const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class Video extends BaseHandler {
    constructor() {
        super('video');
    }

    /**
     * JPG 的預設轉換策略：優先 Canvas（快），失敗時 FFmpeg
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG];
    }
}

module.exports = new Video();