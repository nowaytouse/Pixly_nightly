const Download = require('./download');
module.exports = new (class extends Download {
    constructor() {
        super();
    }
    async convert(imageBase64, maskBase64) {
        const inpaint = require('./inpainting');
        const result = await inpaint(imageBase64, maskBase64);
        return result;
    }
})();
