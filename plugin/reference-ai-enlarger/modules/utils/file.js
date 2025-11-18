'use strict';
const fs = require('node:fs');
const path = require('node:path');

module.exports = class {
    // 建立文件資料夾
    static createFolder = async (folderPath) => {
        folderPath = path.normalize(folderPath);
        try {
            if (!fs.existsSync(folderPath)) {
                eagle.log.info(`start create folder, path: ${folderPath}`);
                await fs.promises.mkdir(folderPath);
                eagle.log.info(`start create folder`);
            }
        } catch (error) {
            throw error;
        }
    };
    // 刪除文件資料夾
    static deleteFolder = async (folderPath) => {
        folderPath = path.normalize(folderPath);
        try {
            if (!(await this.exist(folderPath))) return;
            eagle.log.info(`start delete folder, path: ${folderPath}`);
            await fs.promises.rmdir(folderPath, { recursive: true });
            eagle.log.info(`end delete folder`);
        } catch (error) {
            throw error;
        }
    };
    static replace = async (sourcePath, targetPath) => {
        sourcePath = path.normalize(sourcePath);
        targetPath = path.normalize(targetPath);
        try {
            eagle.log.info(
                `start replace file, sourcePath:${sourcePath}, targetPath:${targetPath}`
            );

            await fs.promises.copyFile(sourcePath, targetPath);

            eagle.log.info(`end replace file`);
        } catch (error) {
            throw error;
        }
    };
    static saveAs = async (sourcePath, targetPath) => {
        // 檢查檔案是否存在
        const hasFile = async (filePath) => {
            try {
                await fs.promises.access(filePath);
                return true;
            } catch (error) {
                return false;
            }
        };
        sourcePath = path.normalize(sourcePath);
        targetPath = path.normalize(targetPath);
        try {
            eagle.log.info(`start saveAs file, sourcePath:${sourcePath}, targetPath:${targetPath}`);
            if (await hasFile(targetPath)) {
                const { dir, name, ext } = path.parse(targetPath);
                let i = 1;
                while (await hasFile(`${dir}/${name} (${i})${ext}`)) {
                    i++;
                }
                targetPath = `${dir}/${name} (${i})${ext}`;
            }
            await fs.promises.copyFile(sourcePath, targetPath);
            eagle.log.info(`end saveAs file`);
        } catch (error) {
            throw error;
        }
    };
    // 儲存文件檔案
    static save = async (filePath, buffer) => {
        filePath = path.normalize(filePath);
        try {
            eagle.log.info(`start save file, path:${filePath}`);
            await fs.promises.writeFile(filePath, buffer);
            eagle.log.info(`end save file`);
        } catch (error) {
            throw error;
        }
    };

    // 銷毀文件
    static destroy = async (filePath) => {
        filePath = path.normalize(filePath);
        try {
            eagle.log.info(`start destroy file, path:${filePath}`);
            await fs.promises.unlink(filePath);
            eagle.log.info(`end destroy file`);
        } catch (error) {
            throw error;
        }
    };
    static exist = async (filePath) => {
        filePath = path.normalize(filePath);
        try {
            await fs.promises.access(filePath, fs.constants.F_OK);
            return true;
        } catch (error) {
            return false;
        }
    };
    // 檢查檔名是否不同
    static isEqualExt(ext, formatExt) {
        let a = ext.toLowerCase();
        if (a === 'jpg') a = 'jpeg';
        let b = formatExt.toLowerCase();
        if (a === b) {
            throw 'equalExtension';
        }
    }

    static readChunkSync = (filePath, startPosition, length) => {
        let buffer = Buffer.alloc(length);
        const fileDescriptor = fs.openSync(filePath, 'r');

        try {
            const bytesRead = fs.readSync(fileDescriptor, buffer, {
                length,
                position: startPosition
            });

            if (bytesRead < length) buffer = buffer.subarray(0, bytesRead);

            return buffer;
        } finally {
            fs.closeSync(fileDescriptor);
        }
    };

    static convert = class {
        // file or blob
        static fileToDataURL = async (file) => {
            const reader = new FileReader();
            reader.readAsDataURL(file);
            try {
                return new Promise((resolve, reject) => {
                    reader.onload = () => {
                        resolve(reader.result);
                    };
                    reader.onerror = (error) => {
                        reject(error);
                    };
                });
            } catch (err) {
                throw err;
            }
        };
        static bufferToDataURL = async (buffer, type) => {
            const blob = new Blob([buffer], { type });
            return await this.fileToDataURL(blob);
        };
    };
};
