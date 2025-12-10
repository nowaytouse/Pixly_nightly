const BaseBuilder = require('./base');

class Mp4Builder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const baseArgs = super.buildArgs(src, dest, options);

        this.args = [
            // 當前 eagle ffmpeg 版本 h265不支援透明
            // '-pix_fmt', 'yuva420p',
            ...super.getCodecArgs(options, this.getGPUArgs.bind(this)),
            ...this.getFrameRateArgs(options),
            ...baseArgs
        ];

        return this.args.filter((arg) => arg);
    }


    getGPUArgs(options, osType, gpuType) {
        // Windows系統沒有GPU類型時使用軟體編解碼器
        if (osType === 'Windows_NT' && !gpuType) {
            // Map codec names to correct libx encoder names
            const codecMap = {
                'h264': 'libx264',
                'h265': 'libx265'
            };
            return [
                '-c:v', codecMap[options.codec],
                '-preset', 'ultrafast',
            ];
        }

        const gpuArgs = {
            h264: {
                Darwin: 'h264_videotoolbox',
                Windows_NT: {
                    NVIDIA: 'h264_nvenc',
                    AMD: 'h264_amf',
                    Intel: 'h264_qsv',
                }
            },
            h265: {
                Darwin: 'hevc_videotoolbox',
                Windows_NT: {
                    NVIDIA: 'hevc_nvenc',
                    AMD: 'hevc_amf',
                    Intel: 'hevc_qsv',
                }
            }
        };

        if (gpuType) {
            return ['-c:v', gpuArgs[options.codec][osType][gpuType]];
        }

        return ['-c:v', gpuArgs[options.codec][osType]];
    }

    getQualityArgs(options) {
        // 品質控制，數字越低越清晰但檔案越大（常用範圍 18–28）。
        const ffmpegQuality = Math.round(28 - (options.quality / 100) * 28);
        return ['-crf', ffmpegQuality.toString()];
    }

    getFrameRateArgs(options) {
        if (options.animatedFps && options.animatedFps !== 'sameAsSource') {
            return ['-r', options.animatedFps];
        }
        return [];
    }

    getSizeFilter(options) {
        if (!options.sizeType) {
            return null;
        }

        options.sizeValue = Math.ceil(options.sizeValue / 2) * 2;

        const sizeHandlers = {
            maxWidth: () =>
                super._buildScaleFilter(`ceil(min'(iw,${options.sizeValue})'/2)*2`, '-2'),
            maxHeight: () =>
                super._buildScaleFilter('-2', `ceil(min'(ih,${options.sizeValue})'/2)*2`),
            minWidth: () =>
                super._buildScaleFilter(`ceil(max'(iw,${options.sizeValue})'/2)*2`, '-2'),
            minHeight: () =>
                super._buildScaleFilter('-2', `ceil(max'(ih,${options.sizeValue})'/2)*2`),
            maxSide: () => super._buildMaxSideScaleFilter(options.sizeValue),
            minSide: () => super._buildMinSideScaleFilter(options.sizeValue),
            exact: () => super._buildExactScaleFilter(options)
        };

        const handler = sizeHandlers[options.sizeType];
        return handler ? handler() : null;
    }
}

module.exports = new Mp4Builder();
