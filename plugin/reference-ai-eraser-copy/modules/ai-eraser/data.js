const fs = require('node:fs');
const path = require('node:path');

const cloudUrl = 'https://r2-plugin.eagle.cool/ai-eraser';
const currentPath = (() => {
    let currentPath = '';
    const modelName = 'ai-eraser';
    if (eagle.isDev) {
        currentPath = `${eagle.os.tmpdir()}/${modelName}/bin`;
        // currentPath = `${eagle.plugin.path}/modules/${modelName}/bin`;
    } else {
        // NOTE: 因為之前 isDev 變數有問題，導致很多用戶都下載到了 tmp 目錄，所以這裡為了節省流量，當發現 tmp 目錄有檔案時，將 tmp 目錄移動到正確的目錄
        const tmpPath = `${eagle.os.tmpdir()}/${modelName}/bin`;
        const destPath = `${eagle.plugin.path}/modules/${modelName}/bin`;

        if (!fs.existsSync(destPath) && fs.existsSync(tmpPath)) {
            fs.renameSync(tmpPath, destPath);
        }

        currentPath = path.normalize(`${eagle.plugin.path}/modules/${modelName}/bin`);
    }

    return currentPath;
})();

class Dependency {
    constructor(name, path, type, url, md5, size) {
        this.name = name;
        this.path = path;
        this.type = type;
        this.url = url;
        this.md5 = md5;
        this.size = size;
    }
}

const getModelDependencies = () => {
    return [
        new Dependency(
            'ort-wasm-threaded.wasm',
            `${currentPath}/ort-wasm-threaded.wasm`,
            'model',
            `${cloudUrl}/ort-wasm-threaded.wasm`,
            '2aed5545848ccd5c1bae0ba3a12bfa84',
            9931804
        ),
        new Dependency(
            'ort-wasm.wasm',
            `${currentPath}/ort-wasm.wasm`,
            'model',
            `${cloudUrl}/ort-wasm.wasm`,
            '58ce8299b5f16b171fbbc44dda8c53c2',
            9868357
        ),
        new Dependency(
            'ort-wasm-simd-threaded.wasm',
            `${currentPath}/ort-wasm-simd-threaded.wasm`,
            'model',
            `${cloudUrl}/ort-wasm-simd-threaded.wasm`,
            'cddb189d6dc6663c7f88f78af8cc03d7',
            10867989
        ),
        new Dependency(
            'migan_pipeline_v2.onnx',
            `${currentPath}/migan_pipeline_v2.onnx`,
            'model',
            `${cloudUrl}/migan_pipeline_v2.onnx`,
            '5cb584b37036920451d2a6b9150df8e3',
            28079181
        ),
        new Dependency(
            'opencv.js',
            `${currentPath}/opencv.js`,
            'model',
            `${cloudUrl}/opencv.js`,
            '0dc97a794553d2053f547bb58dce6016',
            7943648
        )
    ];
};
const getMacDependencies = () => {
    return [];
};
const getWinDependencies = () => {
    return [];
};

const getDependenciesInfo = () => {
    const dependencies = getModelDependencies();
    if (process.platform === 'darwin') dependencies.push(...getMacDependencies());
    if (process.platform === 'win32') dependencies.push(...getWinDependencies());
    return dependencies;
};

module.exports = {
    cloudUrl,
    currentPath,
    getDependenciesInfo
};
