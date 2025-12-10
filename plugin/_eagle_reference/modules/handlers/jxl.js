const BaseHandler = require('./core/BaseHandler');
const File = require('../utils/file');
const fs = require('fs').promises;
const path = require('path');

class JXL extends BaseHandler {
    constructor() {
        super('jxl');
        this.tempDir = path.join(__dirname, 'temp');
    }

    /**
     * JXL 的轉換策略：所有 JXL 都統一使用兩階段轉換
     * 第一階段：JXL -> PNG (使用 jxlDecoder)
     * 第二階段：PNG -> 目標格式 (使用 FFmpeg)
     */
    getPlan(ctx) {
        // JXL 輸入統一採用兩階段轉換，不使用預設策略
        return [];
    }

    /**
     * 覆寫轉換方法 - 統一使用兩階段轉換流程
     */
    async convert(src, dest, options) {
        // 如果目標格式本身就是 PNG，直接使用 jxlDecoder
        if (options.format.toLowerCase() === 'png') {
            return await this.jxlDecoder.convert(src, dest, options);
        }

        // 其他格式使用兩階段轉換：JXL -> PNG -> 目標格式
        return await this.convertViaPng(src, dest, options);
    }

    /**
     * JXL 兩階段轉換：JXL -> PNG -> 目標格式
     */
    async convertViaPng(src, dest, options) {
        await File.createFolder(this.tempDir);

        // 建立臨時 PNG 檔案
        const tempFilePath = File.generateTempFilePath(this.tempDir, '.png');
        const tempFileName = path.parse(tempFilePath).name;
        const tempDir = path.dirname(tempFilePath);
        const tempFileOptions = {
            format: 'png',
            fileName: tempFileName,
            // 傳遞尺寸選項給第一階段 JXL 解碼
            sizeType: options.sizeType,
            sizeValue: options.sizeValue,
            width: options.width,
            height: options.height
        };

        try {
            // 第一步：JXL -> PNG (使用 jxlDecoder)
            await this.jxlDecoder.convert(src, tempDir, tempFileOptions);

            // 第二步：PNG -> 目標格式 (使用 FFmpeg)
            await this.ffmpegConverter.convert(tempFilePath, dest, options);
        } finally {
            // 清理臨時檔案
            try {
                await fs.unlink(tempFilePath);
            } catch (unlinkError) {
                console.warn(`Failed to delete temp file ${tempFilePath}:`, unlinkError.message);
            }
        }
    }
}

module.exports = new JXL();