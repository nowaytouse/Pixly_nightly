const BaseBuilder = require("./base");

class TiffBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        // 使用父類的標準流程，但加入 codec 參數
        const baseArgs = super.buildArgs(src, dest, options);

        this.args = [
            '-pix_fmt', 'rgba',
            ...baseArgs,
        ]

        return this.args;
    }
}

module.exports = new TiffBuilder();