const utils = require('../utils');
const https = require('node:https');
const fs = require('node:fs');
const { currentPath, getDependenciesInfo } = require('./data');

module.exports = class {
    constructor() {}

    async downloadMissingDependencies(onProgress = () => {}) {
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
        const missingDependenciesInfo = await this.getMissingDependencies();
        await utils.file.createFolder(currentPath);
        const totalSize = missingDependenciesInfo.reduce(
            (acc, dependencyInfo) => acc + dependencyInfo.size,
            0
        );
        return new Promise(async (resolve, reject) => {
            let downloadedSize = 0;
            let completedDownloads = 0;
            for (const dependencyInfo of missingDependenciesInfo) {
                try {
                    const url = await proxyUrl(dependencyInfo.url);
                    eagle.log.info(`start downloading model : ${dependencyInfo?.name}, url : ${url}`);
                    https.get(url, (response) => {
                        const content = [];
                        response.on('data', (chunk) => {
                            content.push(chunk);
                            downloadedSize += chunk.length;
                            onProgress((downloadedSize / totalSize) * 100);
                        });

                        response.on('end', async () => {
                            const contentBuffer = Buffer.concat(content);
                            try {
                                await utils.file.save(dependencyInfo.path, contentBuffer);

                                if (dependencyInfo.type === 'binary') {
                                    await fs.promises.chmod(dependencyInfo.path, '755');
                                }
                            } catch (e) {
                                throw `Failed to write file ${dependencyInfo.name} : ${e}`;
                            }
                            completedDownloads++;
                            console.log(
                                `completed Downloads: ${completedDownloads} / ${missingDependenciesInfo.length}`
                            );
                            if (completedDownloads === missingDependenciesInfo.length) {
                                resolve();
                            }
                        });
                    });
                } catch (error) {
                    eagle.log.error(`model '${dependencyInfo?.name}' download error : ${error}`);
                    reject(error);
                } finally {
                    eagle.log.info('end downloading model');
                }
            }
        });
    }

    async getMissingDependencies() {
        const dependenciesInfo = getDependenciesInfo();
        const missingDependenciesInfo = [];
        for (const dependencyInfo of dependenciesInfo) {
            try {
                const md5 = await utils.file.getMd5(dependencyInfo.path);
                if (eagle.isDev) console.log(`MD5 of ${dependencyInfo?.name}: ${md5}`);
                if (md5 !== dependencyInfo.md5) throw 'file MD5 not match';
            } catch (error) {
                eagle.log.error(error);
                missingDependenciesInfo.push(dependencyInfo);
            }
        }

        return missingDependenciesInfo;
    }
};
