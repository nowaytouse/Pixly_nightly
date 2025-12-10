const { createContext, ConverterType } = require('./types');
const path = require('path');

/**
 * 統一的轉換處理器基礎類別
 */
class BaseHandler {
    constructor(name) {
        this.name = name;
        this.canvasConverter = require('../../fileConverter/toolkits/canvasConverter');
        this.ffmpegConverter = require('../../fileConverter/toolkits/ffmpegConverter');
        this.ffprobe = require('../../fileConverter/toolkits/ffprobe');
        this.heicHeifConverter = require('../../fileConverter/toolkits/heicHeifConverter');
        this.jxlDecoder = require('../../fileConverter/toolkits/jxlDecoder');
        this.webpDecoder = require('../../fileConverter/toolkits/webpDecoder');
    }

    /**
     * 子類別覆寫此方法定義特定輸出格式的轉換器優先順序
     * @param {Object} ctx 轉換上下文
     * @returns {Array<string>} 轉換器優先順序陣列
     */
    getPlan(ctx) {
        // 不支援輸出到 HEIC/HEIF 格式
        if (ctx.out.ext === 'heic' || ctx.out.ext === 'heif') {
            throw new Error(`輸出到 ${ctx.out.ext.toUpperCase()} 格式已被停用`);
        }

        return this.getDefaultPlan(ctx);
    }

    /**
     * 預設轉換器選擇邏輯
     * @param {Object} ctx 轉換上下文
     * @returns {Array<string>} 轉換器優先順序陣列
     */
    getDefaultPlan(ctx) {
        // GIF 或動畫檔案優先使用 FFmpeg
        if (ctx.out.ext === 'gif' || ctx.in.animated) {
            return [ConverterType.FFMPEG, ConverterType.CANVAS];
        }

        // 其他格式預設 Canvas 優先（速度快）
        return [ConverterType.CANVAS, ConverterType.FFMPEG];
    }

    /**
     * 取得轉換器實例
     * @param {string} converterId 轉換器ID
     * @returns {Object} 轉換器實例
     */
    getConverter(converterId) {
        switch (converterId) {
            case ConverterType.CANVAS:
                return this.canvasConverter;
            case ConverterType.FFMPEG:
                return this.ffmpegConverter;
            case ConverterType.HEIC:
                return this.heicHeifConverter;
            case ConverterType.JXL:
                return this.jxlDecoder;
            case ConverterType.WEBP:
                return this.webpDecoder;
            default:
                throw new Error(`Unknown converter: ${converterId}`);
        }
    }

    /**
     * 檢查轉換器是否能處理指定的轉換
     * @param {Object} converter 轉換器實例
     * @param {Object} ctx 轉換上下文
     * @returns {boolean}
     */
    canConverterHandle(converter, ctx) {
        // HEIC 轉換器只能讀取 HEIC/HEIF 輸入並轉換到其他格式
        // 不支援寫入 HEIC/HEIF 輸出
        if (converter === this.heicHeifConverter) {
            const canReadHeic = ctx.in.ext === 'heic' || ctx.in.ext === 'heif';
            return canReadHeic;
        }

        // JXL 解碼器只能讀取 JXL 輸入並只能輸出 PNG
        if (converter === this.jxlDecoder) {
            const canReadJxl = ctx.in.ext === 'jxl';
            const canWritePng = ctx.out.ext === 'png';
            return canReadJxl && canWritePng;
        }

        return true;
    }

    /**
     * 統一轉換邏輯
     * @param {string} src 來源檔案路徑
     * @param {string} dest 目標檔案路徑
     * @param {Object} options 轉換選項
     * @returns {Promise<Object|undefined>} 可能返回包含 killProcess 的物件
     */
    async convert(src, dest, options) {
        // 建立轉換上下文
        const isAnimated = await this.ffprobe.isAnimatedImage(src);        

        const inputExt = path.extname(src).slice(1);
        const ctx = createContext(inputExt, options.format, isAnimated);
        options.isAnimated = isAnimated;

        // 取得轉換器執行順序
        const plan = this.getPlan(ctx);        

        // 依序嘗試轉換器
        for (let i = 0; i < plan.length; i++) {
            const converterId = plan[i];
            const converter = this.getConverter(converterId);

            if (!this.canConverterHandle(converter, ctx)) {
                continue;
            }

            try {
                const result = await converter.convert(src, dest, options);

                // CRITICAL: 檢查特殊的 killProcess 回傳
                if (result && result.killProcess) {
                    return result;
                }

                return; // 成功完成

            } catch (error) {
                // 如果是最後一個轉換器，則拋出錯誤
                if (i === plan.length - 1) {
                    throw error;
                }

                // 否則警告並嘗試下一個轉換器
                console.warn(`${converterId} converter failed, trying next:`, error.message);
            }
        }

        throw new Error(`所有轉換器都失敗了 ${ctx.in.ext} -> ${ctx.out.ext}`);
    }
}

module.exports = BaseHandler;