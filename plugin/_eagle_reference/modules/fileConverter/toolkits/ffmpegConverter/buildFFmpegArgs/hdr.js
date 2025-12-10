const BaseBuilder = require("./base");

class HdrBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {

        const baseArgs = super.buildArgs(src, dest, options);
        this.args = [
            '-c:v', 'hdr',
            '-pix_fmt', 'rgb24',
            ...baseArgs,
        ];

        return this.args.filter(arg => arg);
    }
}

module.exports = new HdrBuilder();