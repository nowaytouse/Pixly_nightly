const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class AVIF extends BaseHandler {
    constructor() {
        super('avif');
    }

    /**
     * AVIF 輸出的轉換策略：優先 FFmpeg，失敗時 Canvas
     * @param {Object} ctx 轉換上下文
     * @returns {Array<string>} 轉換器優先順序陣列
     */
    getPlan(ctx) {
        // AVIF 輸出時優先使用 FFmpeg
        if (ctx.out.ext === 'avif') {
            return [ConverterType.FFMPEG];
        }
        
        // 其他格式使用預設邏輯
        return this.getDefaultPlan(ctx);
    }

    /**
     * AVIF 的預設轉換策略：優先 FFmpeg，失敗時 Canvas
     */
    getDefaultPlan(ctx) {
        return [ConverterType.FFMPEG, ConverterType.CANVAS];
    }
}

module.exports = new AVIF();