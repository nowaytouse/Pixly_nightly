const BaseBuilder = require("./base");

class ExrBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const baseArgs = super.buildArgs(src, dest, options);
        this.args = [            
            '-pix_fmt', 'gbrapf32le',            
            ...baseArgs,
        ];        

        return this.args.filter(arg => arg);
    }
}

module.exports = new ExrBuilder();