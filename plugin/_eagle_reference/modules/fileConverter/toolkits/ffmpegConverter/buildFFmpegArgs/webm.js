const BaseBuilder = require('./base');

class WebmBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const baseArgs = super.buildArgs(src, dest, options);

        this.args = [
            '-pix_fmt', 'yuva420p',
            ...(options.codec === 'vp8' ? ['-auto-alt-ref', '0'] : []),
            ...super.getCodecArgs(options, this.getGPUArgs.bind(this)),
            ...this.getFrameRateArgs(options),
            ...baseArgs,
        ];

        return this.args.filter(arg => arg);
    }


    getGPUArgs(options, osType, gpuType) {
        // VideoToolbox 不支援 VP8/VP9/AV1，所以 WebM 在 macOS 上只能用 CPU 編碼器
        const gpuArgs = {
            vp8: {
                // 沒有任何 GPU 硬體編碼器支援 VP8（AMD / NVIDIA / Intel 都沒有）
                Darwin: 'libvpx',
                Windows_NT: {
                    NVIDIA: 'libvpx',
                    AMD: 'libvpx',
                    Intel: 'libvpx'
                }
            },
            vp9: {
                Darwin: 'libvpx-vp9',
                Windows_NT: {
                    NVIDIA: 'vp9_nvenc',
                    AMD: 'libvpx-vp9',
                    Intel: 'libvpx-vp9'
                }
            }
        };

        // Windows系統沒有GPU類型時使用軟體編解碼器
        if (osType === 'Windows_NT' && !gpuType) {
            return [
                '-c:v', gpuArgs[options.codec].Darwin,
                '-cpu-used', '5',
            ]; // 使用CPU編解碼器
        }

        if (osType === 'Windows_NT' && gpuType) {
            return ['-c:v', gpuArgs[options.codec][osType][gpuType]];
        }

        return [
            '-c:v', gpuArgs[options.codec][osType],
            '-cpu-used', '5',
        ];
    }

    getQualityArgs(options) {
        // VP8/VP9 CRF: 4–63, 越低越清晰，常用 15–35
        const crf = Math.round(63 - ((options.quality / 100) * 59));
        return ['-crf', crf.toString()];
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
            maxWidth: () => super._buildScaleFilter(`ceil(min'(iw,${options.sizeValue})'/2)*2`, '-2'),
            maxHeight: () => super._buildScaleFilter('-2', `ceil(min'(ih,${options.sizeValue})'/2)*2`),
            minWidth: () => super._buildScaleFilter(`ceil(max'(iw,${options.sizeValue})'/2)*2`, '-2'),
            minHeight: () => super._buildScaleFilter('-2', `ceil(max'(ih,${options.sizeValue})'/2)*2`),
            maxSide: () => super._buildMaxSideScaleFilter(options.sizeValue),
            minSide: () => super._buildMinSideScaleFilter(options.sizeValue),
            exact: () => super._buildExactScaleFilter(options)
        };

        const handler = sizeHandlers[options.sizeType];
        return handler ? handler() : null;
    }

}

module.exports = new WebmBuilder();