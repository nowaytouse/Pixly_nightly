const url = require('node:url');
const { currentPath } = require('./data');
async function ensureModel() {
    const modelURL = url.pathToFileURL(`${currentPath}/migan_pipeline_v2.onnx`).href;
    const response = await fetch(modelURL);
    const buffer = await response.arrayBuffer();
    return buffer;
}

module.exports = {
    ensureModel
};
