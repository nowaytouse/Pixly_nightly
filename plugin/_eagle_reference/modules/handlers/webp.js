const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');

class WEBP extends BaseHandler {
    constructor() {
        super('webp');
    }

    /**
     * WebP 需要動畫檢測（雖然轉換策略相同）
     */
    needsAnimationCheck() {
        return true;
    }

    /**
     * 根據是否為動畫決定轉換策略
     */
    async convert(src, dest, options) {
        // 檢查是否為動態 WebP
        const isAnimated = await this.ffprobe.isAnimatedImage(src);

        if (isAnimated) {
            // 動態 WebP 使用 buffer 轉換
            await this.convertViaBuffer(src, dest, options);
        } else {
            // 靜態 WebP 使用父類的標準轉換流程
            return super.convert(src, dest, options);
        }
    }

    /**
     * 覆寫 getPlan 為靜態 WebP 提供轉換器優先順序
     */
    getPlan(ctx) {
        // 靜態 WebP：優先 Canvas（快），失敗時 FFmpeg
        if (!ctx.in.animated) {
            return [ConverterType.CANVAS, ConverterType.FFMPEG];
        }
        // 動態 WebP 不會走到這裡（在 convert 中直接處理）
        return super.getPlan(ctx);
    }

    /**
     * WebP 兩階段轉換：WebP -> Buffer -> 目標格式
     */
    async convertViaBuffer(src, dest, options) {

        try {
            // 第一步：decode Webp (使用 webpDecoder)
            const imageData = await this.webpDecoder.decode(src, options);
            options.imageData = imageData;
            options.isAnimated = true;

            // 第二步：RGBA -> 目標格式 (使用 FFmpeg)
            await this.ffmpegConverter.convert(src, dest, options);
        } catch (error) {
            throw error;
        }
    }
}

module.exports = new WEBP();