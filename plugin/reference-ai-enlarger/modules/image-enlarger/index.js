const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const https = require('https');
const { spawn } = require('child_process');
const ASSETS_URL_PREFIX = 'https://r2-plugin.eagle.cool/image-enlarger';

const utils = require('../utils');
class ImageEnlarger {
    // serverProcess = null;
    #dependenciesPath;

    constructor(params) {
        if (params.devMode) {
            this.#dependenciesPath = `${eagle.os.tmpdir()}/image-enlarger/bin`;
            // this.#dependenciesPath = `${eagle.plugin.path}/modules/image-enlarger/bin`;
        } else {
            // NOTE: 因為之前 isDev 變數有問題，導致很多用戶都下載到了 tmp 目錄，所以這裡為了節省流量，當發現 tmp 目錄有檔案時，將 tmp 目錄移動到正確的目錄
            const tmpPath = `${eagle.os.tmpdir()}/image-enlarger/bin`;
            const destPath = `${eagle.plugin.path}/modules/image-enlarger/bin`;

            if (!fs.existsSync(destPath) && fs.existsSync(tmpPath)) {
                fs.renameSync(tmpPath, destPath);
            }

            this.#dependenciesPath = `${eagle.plugin.path}/modules/image-enlarger/bin`;
        }
        // this.checkDependencies();
        console.log(this.#dependenciesPath);
        this.ensureDirectoryExistence(`${this.#dependenciesPath}/temp/`);
    }

    async imageEnlarge(model = 'realesrgan-x4plus-anime', sourcePath, outputPath, onProgress) {
        try {
            const result = await new Promise((resolve, reject) => {
                const CLI_PATH =
                    `${this.#dependenciesPath}/realesrgan-ncnn-vulkan` +
                    (eagle.app.platform === 'win32' ? '.exe' : '');
                const src = sourcePath;
                const dist = outputPath;

                let process = spawn(
                    CLI_PATH,
                    ['-i', src, '-o', dist, '-m', `${this.#dependenciesPath}/models`, '-n', model],
                    { timeout: 100 * 1000 }
                );

                process.stdout.on('data', (data) => {
                    console.log(`stdout: ${data}`);
                });

                process.stderr.on('data', (data) => {
                    const regex = /\d+\.\d/g;
                    const found = data.toString().match(regex);
                    const error_regex = /invalid/g;
                    const error_found = data.toString().match(error_regex);
                    if (error_found) {
                        reject(data.toString());
                    } else {
                        if (found) {
                            onProgress(Number(found.pop()));
                        }
                    }
                });

                process.on('close', async (code) => {
                    onProgress(100);
                    if (await utils.file.exist(outputPath)) resolve(outputPath);
                    reject('convertError');
                });
            });

            return result;
        } catch (err) {
            throw err;
        }
    }

    async ensureDirectoryExistence(filePath) {
        var dirname = path.dirname(filePath);
        await fs.promises.mkdir(dirname, { recursive: true });
    }

    async checkDependencies() {
        return (await this.getMissingDependencies().length) === 0;
    }

    async getMissingDependencies() {
        const dependencies = this.getDependencies();
        const missingDependencies = [];

        for (const dependency of dependencies) {
            try {
                await fs.promises.access(dependency.path);
                const filePath = dependency.path;
                const chunkSize = 4096;
                const headBuffer = utils.file.readChunkSync(filePath, 0, chunkSize);
                const bodyBuffer = utils.file.readChunkSync(
                    filePath,
                    Math.floor((dependency.size - chunkSize) / 2),
                    chunkSize
                );
                const tailBuffer = utils.file.readChunkSync(
                    filePath,
                    dependency.size - chunkSize,
                    chunkSize
                );
                const mergeBuffer = Buffer.concat([headBuffer, bodyBuffer, tailBuffer]);

                const md5 = crypto.createHash('md5').update(mergeBuffer).digest('hex');

                if (md5 !== dependency.md5) throw 'file MD5 not match';
            } catch (error) {
                missingDependencies.push(dependency);
            }
        }

        return missingDependencies;
    }

    getDependencies() {
        let dependencies = [
            {
                name: 'realesrgan-x4plus-anime.param',
                path: `${this.#dependenciesPath}/models/realesrgan-x4plus-anime.param`,
                type: 'model',
                url: ASSETS_URL_PREFIX + '/models/realesrgan-x4plus-anime.param',
                size: 30290,
                md5: 'cf5b2df335675a5b3fa5e5ce47eca847'
            },
            {
                name: 'realesrgan-x4plus.param',
                path: `${this.#dependenciesPath}/models/realesrgan-x4plus.param`,
                type: 'model',
                url: ASSETS_URL_PREFIX + '/models/realesrgan-x4plus.param',
                size: 116029,
                md5: '49dce87479b2b5ffb4405aa449a9ce26'
            },
            {
                name: 'realesrgan-x4plus-anime.bin',
                path: `${this.#dependenciesPath}/models/realesrgan-x4plus-anime.bin`,
                type: 'model',
                url: ASSETS_URL_PREFIX + '/models/realesrgan-x4plus-anime.bin',
                size: 8943500,
                md5: 'd1036ffcccfe945901f8ba824735cbcb'
            },
            {
                name: 'realesrgan-x4plus.bin',
                path: `${this.#dependenciesPath}/models/realesrgan-x4plus.bin`,
                type: 'model',
                url: ASSETS_URL_PREFIX + '/models/realesrgan-x4plus.bin',
                size: 33424520,
                md5: 'fcb6b30c7169869ddc1eaf2a965ef880'
            }
        ];

        if (eagle.app.platform === 'win32') {
            dependencies = dependencies.concat(this.getWindowsDependencies());
        }

        if (eagle.app.platform === 'darwin') {
            dependencies = dependencies.concat(this.getMacDependencies());
        }

        return dependencies;
    }

    getWindowsDependencies() {
        return [
            {
                name: 'realesrgan-ncnn-vulkan.exe',
                path: `${this.#dependenciesPath}/realesrgan-ncnn-vulkan.exe`,
                type: 'binary',
                url: ASSETS_URL_PREFIX + '/realesrgan-ncnn-vulkan.exe',
                size: 6161408,
                md5: 'a098cc6eba1814cd8229d9f2bc989be4'
            },
            {
                name: 'vcomp140.dll',
                path: `${this.#dependenciesPath}/vcomp140.dll`,
                type: 'library',
                url: ASSETS_URL_PREFIX + '/vcomp140.dll',
                size: 182704,
                md5: '12f8fc3cd4c9dd9d344be239b05857f2'
            },
            {
                name: 'vcomp140d.dll',
                path: `${this.#dependenciesPath}/vcomp140d.dll`,
                type: 'library',
                url: ASSETS_URL_PREFIX + '/vcomp140d.dll',
                size: 207736,
                md5: '829756a44056089e7c12b2d3f4e39475'
            }
        ];
    }

    getMacDependencies() {
        return [
            {
                name: 'realesrgan-ncnn-vulkan',
                path: `${this.#dependenciesPath}/realesrgan-ncnn-vulkan`,
                type: 'binary',
                url: ASSETS_URL_PREFIX + '/realesrgan-ncnn-vulkan',
                size: 26787080,
                md5: '41d8e06f37e236d6f8e118612a81c826'
            }
        ];
    }

    async getMissingDependenciesSize() {
        const missingDependencies = await this.getMissingDependencies();
        let totalMissingSize = 0;

        missingDependencies.forEach((dependency) => {
            totalMissingSize += dependency.size;
        });

        return totalMissingSize;
    }

    async downloadDependencies(missingDependencies, onProgress, onComplete, onError) {
        const proxyUrl = async (url) => {
            const urls = [url, 'https://proxy.eagle.cool/?url=' + url];

            for (let i of urls) {
                if (
                    await new Promise((resolve) => {
                        https
                            .request(i, { method: 'HEAD' }, (res) => {
                                resolve(res?.statusCode === 200);
                            })
                            .on('error', (err) => {
                                resolve(false);
                            })
                            .end();
                    })
                ) {
                    return i;
                }
            }
            throw 'Unable to request the URL.';
        };
        let downloaded = 0;
        let completedDownloads = 0;
        const totalSize = missingDependencies.reduce(
            (total, dependency) => total + dependency.size,
            0
        );

        const errors = [];
        for (let dependency of missingDependencies) {
            try {
                const url = await proxyUrl(dependency.url);
                eagle.log.info(`start downloading model : ${dependency.name}, url : ${url}`);

                await new Promise((resolve, reject) => {
                    https
                        .get(url, (response) => {
                            const content = [];

                            response.on('data', (chunk) => {
                                content.push(chunk);
                                downloaded += chunk.length;
                                onProgress(downloaded, totalSize);
                            });

                            response.setTimeout(10000, () => {
                                reject('socket is destroyed due to timeout');
                            });

                            response.on('end', async () => {
                                const contentBuffer = Buffer.concat(content);
                                try {
                                    await this.ensureDirectoryExistence(dependency.path);
                                    await fs.promises.writeFile(dependency.path, contentBuffer);

                                    if (dependency.type == 'binary') {
                                        await fs.promises.chmod(dependency.path, '755');
                                    }
                                } catch (error) {
                                    console.error('Failed to write file', dependency.name, error);
                                    errors.push(error);
                                }
                                completedDownloads++;
                                console.log(`Overall progress: ${(downloaded / totalSize) * 100}%`);
                                if (completedDownloads === missingDependencies.length) {
                                    onComplete();
                                }
                                resolve();
                            });
                        })
                        .on('error', (error) => {
                            errors.push(error.message);
                            reject(error.message);
                        });
                });
            } catch (error) {
                eagle.log.error(`model '${dependency.name}' download error : ${error}`);

                throw error;
            } finally {
                eagle.log.info('end downloading model');
            }
        }
        if (errors.length > 0) throw 'download error';
    }

    async download(onProgress, onComplete) {
        try {
            const missingDependencies = await this.getMissingDependencies();
            await this.downloadDependencies(missingDependencies, onProgress, onComplete);
        } catch (error) {
            throw error;
        }
    }
}

module.exports = new ImageEnlarger({
    devMode: !!eagle.isDev
});
