const BaseHandler = require('./core/BaseHandler');
const { ConverterType } = require('./core/types');
const File = require('../utils/file');
const fs = require('fs').promises;
const path = require('path');

class HEIC extends BaseHandler {
    constructor() {
        super('heic');
        this.tempDir = path.join(__dirname, 'temp');
    }

    /**
     * HEIC 的預設轉換策略：只使用 HEIC 轉換器
     */
    getDefaultPlan(ctx) {
        return [ConverterType.HEIC];
    }

    /**
     * 覆寫轉換方法處理 AVIF 的特殊兩步驟轉換
     */
    async convert(src, dest, options) {
        // AVIF 特殊處理：HEIC -> PNG -> AVIF
        if (options.format === 'avif' || options.format === 'ico' || options.format === 'exr' || options.format === 'dds' || options.format === 'hdr') {
            return await this.convertViaPng(src, dest, options);
        }

        // 其他格式使用基礎轉換邏輯
        return await super.convert(src, dest, options);
    }

    /**
     * HEIC 轉 AVIF 的特殊兩步驟轉換
     */
    async convertViaPng(src, dest, options) {
        await File.createFolder(this.tempDir);
        
        // 先轉換成 PNG
        const tempFilePath = File.generateTempFilePath(this.tempDir, '.png');
        const tempFileName = path.parse(tempFilePath).name;
        const tempDir = path.dirname(tempFilePath);
        const tempFileOptions = {
            format: 'png',
            fileName: tempFileName
        };

        try {
            await this.heicHeifConverter.convert(src, tempDir, tempFileOptions);
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

module.exports = new HEIC();