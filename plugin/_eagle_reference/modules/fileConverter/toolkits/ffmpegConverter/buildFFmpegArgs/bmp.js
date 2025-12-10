const BaseBuilder = require("./base");

class BmpBuilder extends BaseBuilder {
    constructor() {
        super();
    }

    buildArgs(src, dest, options) {
        // 使用父類的標準流程，但加入 codec 參數
        const baseArgs = super.buildArgs(src, dest, options);

        // Codec 參數必須在最前面
        return ['-frames:v', '1', ...baseArgs];
    }
}

module.exports = new BmpBuilder();