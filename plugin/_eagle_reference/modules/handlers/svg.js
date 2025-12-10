const BaseHandler = require('./core/BaseHandler');
const File = require('../utils/file');
const fs = require('fs').promises;
const path = require('path');

class SVG extends BaseHandler {
    constructor() {
        super('svg');
        this.tempDir = `${__dirname}/temp`;
    }

    /**
     * SVG 有複雜的轉換邏輯，需要完全覆寫轉換方法
     */
    async convert(src, dest, options) {
        // 檢查 Canvas 是否支援目標格式
        if (this.canvasConverter.isSupportedFormat(options.format)) {
            // Canvas 支援的格式：直接使用 Canvas
            await this.canvasConverter.convert(src, dest, options);
        } else {
            // Canvas 不支援的格式：SVG -> PNG -> 目標格式
            await this.convertViaPng(src, dest, options);
        }
    }

    /**
     * 透過 PNG 中間格式轉換
     */
    async convertViaPng(src, dest, options) {
        const tempFilePath = File.generateTempFilePath(this.tempDir, '.png');
        const tempFileName = path.basename(tempFilePath, '.png');
        const tempFileOptions = {
            format: 'png',
            fileName: tempFileName,
            quality: 100,
            sizeType: options.sizeType,
            sizeValue: options.sizeValue,
            width: options.width,
            height: options.height
        };

        try {
            // 確保臨時目錄存在
            await fs.mkdir(this.tempDir, { recursive: true });
            
            // SVG -> PNG (使用 Canvas)
            await this.canvasConverter.convert(src, this.tempDir, tempFileOptions);            
            
            // PNG -> 目標格式 (使用 FFmpeg)
            await this.ffmpegConverter.convert(tempFilePath, dest, options);
        } finally {
            // 清理臨時文件
            try {
                await fs.unlink(tempFilePath);
            } catch (unlinkError) {
                console.warn(`Failed to delete temp file ${tempFilePath}:`, unlinkError.message);
            }
        }
    }
}

module.exports = new SVG();