const BaseBuilder = require("./base");

class AvifBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const avifQuality = Math.round(63 - ((options.quality / 100) * 63));

        const baseArgs = super.buildArgs(src, dest, options);

        this.args = [
            '-c:v', 'libaom-av1',
            '-crf', avifQuality.toString(),
            '-strict', 'experimental',
            '-pix_fmt', 'yuva420p',
            '-cpu-used', '8',
            '-row-mt', '1',
            '-tiles', '2x2',
            ...baseArgs,
        ];

        return this.args.filter(arg => arg);
    }
}

module.exports = new AvifBuilder();