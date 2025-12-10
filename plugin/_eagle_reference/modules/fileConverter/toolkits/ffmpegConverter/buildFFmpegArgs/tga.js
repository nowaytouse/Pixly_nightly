const BaseBuilder = require("./base");

class TgaBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        const baseArgs = super.buildArgs(src, dest, options);
        this.args = [
            '-c:v', 'targa',
            '-pix_fmt', 'bgra',            
            ...baseArgs,
        ];        

        return this.args.filter(arg => arg);
    }
}

module.exports = new TgaBuilder();