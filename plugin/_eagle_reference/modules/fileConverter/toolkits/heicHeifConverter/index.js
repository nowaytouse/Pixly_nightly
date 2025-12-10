const path = require('path');
const { promisify } = require('util');
const { exec, spawn } = require('child_process');
const execAsync = promisify(exec);
const fs = require('fs').promises;
const { validateOutputFile, getOutputPath } = require('../../validate');
const { progressiveScale, calculateNewDimensions } = require('../../../utils/canvasScaler');

class heicHeifConverter {
    constructor() {
        this.platform = process.platform;
    }

    /**
     * 驗證輸出格式是否支援
     * @param {string} format 目標格式
     */
    validateOutputFormat(format) {
        if (!format) return; // 使用預設格式
        
        const formatLower = format.toLowerCase();
        
        // 非 macOS 平台不支援輸出 HEIC/HEIF
        if (this.platform !== 'darwin' && (formatLower === 'heic' || formatLower === 'heif')) {
            throw new Error(i18next.t('error.heicOutputNotSupportedOnWindows'));
        }
    }

    async convert(src, dest, options) {
        // 統一驗證放在最前面，失敗即快速返回
        this.validateOutputFormat(options.format);
        
        // macOS 平台：優先嘗試 sips，失敗則降級到 Canvas
        if (this.platform === 'darwin') {
            try {
                return await this.convertBySips(src, dest, options);
            } catch (sipsError) {                
                console.log('SIPS conversion failed, falling back to Canvas:', sipsError.message);
                // 明確的降級路徑 - 不是意外，是設計
            }
        }
        
        // 非 macOS 平台或 sips 失敗後的統一 Canvas 路徑
        try {
            return await this.convertByCanvas(src, dest, options);
        } catch (canvasError) {
            // 統一的錯誤格式，包含具體失敗原因            
            throw new Error(i18next.t('error.failedToConvertImage', { 
                error: canvasError.message 
            }));
        }
    }

    async getImageDimensions(imagePath) {
        try {
            const { stdout } = await execAsync(`sips -g pixelWidth -g pixelHeight "${imagePath}"`);
            const lines = stdout.split('\n');
            const width = parseInt(lines[1].split(':')[1].trim());
            const height = parseInt(lines[2].split(':')[1].trim());
            return { width, height };
        } catch (error) {
            throw new Error(i18next.t('error.failedToGetImageDimensions', { error: error.message }));
        }
    }

    buildResizeCommand(originalWidth, originalHeight, options) {
        let resizeArg = '';

        switch (options.sizeType) {
            case 'maxWidth':
                // scale=min(iw,1920):-1
                resizeArg = ` --resampleWidth ${Math.min(originalWidth, options.sizeValue)}`;
                break;

            case 'maxHeight':
                // scale=-1:min(ih,1080)
                resizeArg = ` --resampleHeight ${Math.min(originalHeight, options.sizeValue)}`;
                break;

            case 'minWidth':
                // scale=max(iw,1920):-1
                resizeArg = ` --resampleWidth ${Math.max(originalWidth, options.sizeValue)}`;
                break;

            case 'minHeight':
                // scale=-1:max(ih,1080)
                resizeArg = ` --resampleHeight ${Math.max(originalHeight, options.sizeValue)}`;
                break;

            case 'maxSide': {
                // scale='if(gt(iw,ih),1920,-1)':'if(gt(iw,ih),-1,1920)'
                if (originalWidth > originalHeight) {
                    resizeArg = ` --resampleWidth ${Math.min(originalWidth, options.sizeValue)}`;
                } else {
                    resizeArg = ` --resampleHeight ${Math.min(originalHeight, options.sizeValue)}`;
                }
                break;
            }

            case 'minSide': {
                // scale='if(gt(iw,ih),-1,1920)':'if(gt(iw,ih),1920,-1)'
                if (originalWidth > originalHeight) {
                    resizeArg = ` --resampleHeight ${Math.max(originalHeight, options.sizeValue)}`;
                } else {
                    resizeArg = ` --resampleWidth ${Math.max(originalWidth, options.sizeValue)}`;
                }
                break;
            }

            case 'exact':
                if (options.width && options.height) {
                    resizeArg = ` --resampleHeightWidth ${options.height} ${options.width}`;
                } else if (options.width) {
                    resizeArg = ` --resampleWidth ${options.width}`;
                } else if (options.height) {
                    resizeArg = ` --resampleHeight ${options.height}`;
                }
                break;
        }

        return resizeArg;
    }

    async convertBySips(src, dest, options) {
        try {
            const format = options.format || 'png'; // 預設使用 png 格式
            const outputPath = path.join(dest, `${options.fileName}.${format}`);
            
            // 建立基本的 sips 命令
            let command = `sips -s format ${format} "${src}"`;

            // 如果需要調整尺寸
            if (options.sizeType) {
                // 獲取原始圖片尺寸
                const originalDimensions = await this.getImageDimensions(src);
                
                // 根據 fileConverter 的邏輯構建調整尺寸的命令
                const resizeArg = this.buildResizeCommand(
                    originalDimensions.width,
                    originalDimensions.height,
                    options
                );

                if (resizeArg) {
                    command += resizeArg;
                }
            }

            // 添加輸出路徑
            command += ` --out "${outputPath}"`;            

            // 如果支援取消模式，使用 spawn 取得進程引用
            if (options._enableCancellation) {
                const { promise, killProcess } = await this._execWithCancellation(command);
                await promise;
                
                // 驗證輸出文件
                await validateOutputFile(outputPath, {
                    minSize: 1,
                    timeout: 5000
                });

                return {
                    success: true,
                    outputPath,
                    killProcess
                };
            } else {
                // 向後相容模式
                try {
                    await execAsync(command);
                } catch (error) {
                    // 如果是已知的警告訊息，我們可以檢查檔案是否實際創建
                    if (error.message.includes('Can\'t write format: public.heif') && 
                        error.message.includes('Error 13: an unknown error occurred')) {
                        // 檢查檔案是否存在且有效
                        const stats = await fs.stat(outputPath);
                        if (stats.size > 0) {
                            return {
                                success: true,
                                outputPath
                            };
                        }
                    }
                    throw error;
                }
            }

            // 驗證輸出文件
            await validateOutputFile(outputPath, {
                minSize: 1,
                timeout: 5000
            });

            return {
                success: true,
                outputPath
            };
        } catch (error) {
            throw new Error(i18next.t('error.failedToConvertImage', { error: error.message }));
        }
    }

    /**
     * 執行可取消的命令
     * @param {string} command Shell 命令
     * @returns {Promise<{promise: Promise, killProcess: Function}>}
     * @private
     */
    async _execWithCancellation(command) {
        let childProcess = null;

        const promise = new Promise((resolve, reject) => {
            // 將命令分解為 sips 和參數
            const args = command.match(/(?:[^\s"]+|"[^"]*")+/g) || [];
            const cmd = args.shift(); // 移除第一個元素 (sips)
            
            // 清理引號
            const cleanArgs = args.map(arg => arg.replace(/^"(.*)"$/, '$1'));
            
            childProcess = spawn(cmd, cleanArgs);
            let stderr = '';

            childProcess.stderr.on('data', (data) => {
                stderr += data.toString();
            });

            childProcess.on('close', (code) => {
                if (code === 0) {
                    resolve();
                } else if (code === null || code === 130 || code === 143) {
                    // Process was killed
                    reject(new Error('conversionCancelled'));
                } else {
                    // 檢查是否是已知的警告但實際成功
                    if (stderr.includes('Can\'t write format: public.heif') && 
                        stderr.includes('Error 13: an unknown error occurred')) {
                        // 將由外部檢查檔案是否創建成功
                        resolve();
                    } else {
                        reject(new Error(stderr || `Process exited with code ${code}`));
                    }
                }
            });

            childProcess.on('error', (err) => {
                reject(err);
            });
        });

        return {
            promise,
            killProcess: () => {
                if (childProcess && !childProcess.killed) {
                    childProcess.kill('SIGTERM');
                    // 如果 SIGTERM 沒用，1秒後用 SIGKILL
                    setTimeout(() => {
                        if (childProcess && !childProcess.killed) {
                            childProcess.kill('SIGKILL');
                        }
                    }, 1000);
                }
            }
        };
    }

    /**
     * 使用 Canvas 和 libheif.js 轉換 HEIC 文件 (適用於 Windows/Linux)
     * @param {string} src 來源文件路徑
     * @param {string} dest 目標資料夾路徑
     * @param {Object} options 轉換選項
     * @returns {Promise<Object>} 轉換結果
     */
    async convertByCanvas(src, dest, options) {
        return new Promise(async (resolve, reject) => {
            try {
                const fs = require('fs');
                
                // 格式已在 convert 方法中驗證過
                const format = options.format || 'png';
                
                // 動態載入 libheif.js
                const script = document.createElement('script');
                script.src = path.join(__dirname, 'libheif.js');
                
                script.onload = async () => {
                    try {
                        // 初始化 libheif
                        const libheifCore = await libheif();
                        
                        // 等待 WASM 載入完成
                        await new Promise(resolve => setTimeout(resolve, 50));
                        const decoder = new libheifCore.HeifDecoder();
                        
                        // 讀取 HEIC 文件
                        const fileBuffer = fs.readFileSync(src);
                        const arrayBuffer = new Uint8Array(fileBuffer).buffer;
                        
                        // 解碼 HEIC 圖片
                        const imageData = decoder.decode(arrayBuffer);
                        const image = imageData[0];

                        // 建立 Canvas 和圖像資料
                        const [w, h] = [image.get_width(), image.get_height()];
                        
                        // 先創建原始尺寸的 Canvas
                        const canvas = document.createElement('canvas');
                        canvas.width = w;
                        canvas.height = h;

                        const ctx = canvas.getContext('2d');
                        const image_data = ctx.createImageData(w, h);
                        const data = image_data.data;

                        // 設定透明度
                        for (let i = 0; i < w * h; i++) {
                            data[i * 4 + 3] = 255;
                        }

                        // 顯示圖像到 Canvas
                        await new Promise((resolveDisplay, rejectDisplay) => {
                            image.display(image_data, (display_image_data) => {
                                try {
                                    ctx.putImageData(display_image_data, 0, 0);
                                    resolveDisplay();
                                } catch (e) {
                                    rejectDisplay(e);
                                }
                            });
                        });

                        // 處理尺寸調整
                        let finalCanvas = canvas;
                        if (options.sizeType) {
                            const newDimensions = calculateNewDimensions(w, h, options);
                            if (newDimensions.width !== w || newDimensions.height !== h) {
                                // 使用漸進式縮放
                                finalCanvas = progressiveScale(canvas, newDimensions.width, newDimensions.height);
                            }
                        }

                        // 轉換為目標格式並儲存 (format 已在前面驗證過)
                        const outputPath = path.join(dest, `${options.fileName}.${format}`);
                        
                        // 設定品質
                        let quality = 0.92;
                        if (options.quality !== undefined) {
                            quality = options.quality / 100;
                        }
                        
                        // 轉換為 blob 並儲存
                        const mimeType = this.getMimeType(format);
                        const dataUrl = finalCanvas.toDataURL(mimeType, quality);
                        const base64Data = dataUrl.split(',')[1];
                        const buffer = Buffer.from(base64Data, 'base64');
                        
                        await fs.promises.writeFile(outputPath, buffer);
                        
                        // 驗證輸出文件
                        await validateOutputFile(outputPath, {
                            minSize: 1,
                            timeout: 5000
                        });

                        resolve({
                            success: true,
                            outputPath
                        });
                    } catch (e) {
                        console.log(e);
                        try {
                            // 如果 HEIC 解碼失敗，嘗試作為普通圖片載入
                            console.log('This image may not be in HEIC format, trying to load as JPG/PNG instead...');
                            const image = await new Promise((resolve, reject) => {
                                const img = new Image();
                                img.src = src;
                                img.onload = () => resolve(img);
                                img.onerror = (event) => {
                                    // 提供更明確的錯誤訊息
                                    const errorMsg = `Failed to load image: ${src}. The file may be corrupted or in an unsupported format.`;
                                    reject(new Error(errorMsg));
                                };
                            });

                            // 創建原始尺寸的 Canvas
                            const canvas = document.createElement('canvas');
                            canvas.width = image.width;
                            canvas.height = image.height;
                            const ctx = canvas.getContext('2d');
                            ctx.drawImage(image, 0, 0);
                            
                            // 處理尺寸調整
                            let finalCanvas = canvas;
                            if (options.sizeType) {
                                const newDimensions = calculateNewDimensions(image.width, image.height, options);
                                if (newDimensions.width !== image.width || newDimensions.height !== image.height) {
                                    // 使用漸進式縮放
                                    finalCanvas = progressiveScale(canvas, newDimensions.width, newDimensions.height);
                                }
                            }

                            if (finalCanvas.width && finalCanvas.height) {
                                console.log('Successfully loaded as JPG/PNG');
                                
                                // 轉換為目標格式並儲存 (format 已在前面驗證過)
                                const fallbackOutputPath = path.join(dest, `${options.fileName}.${format}`);
                                
                                let quality = 0.92;
                                if (options.quality !== undefined) {
                                    quality = options.quality / 100;
                                }
                                
                                const mimeType = this.getMimeType(format);
                                const dataUrl = finalCanvas.toDataURL(mimeType, quality);
                                const base64Data = dataUrl.split(',')[1];
                                const buffer = Buffer.from(base64Data, 'base64');
                                
                                await fs.promises.writeFile(fallbackOutputPath, buffer);
                                
                                await validateOutputFile(fallbackOutputPath, {
                                    minSize: 1,
                                    timeout: 5000
                                });
                                
                                resolve({
                                    success: true,
                                    outputPath: fallbackOutputPath
                                });
                            } else {
                                console.log('Failed to load as JPG/PNG');
                                reject(new Error('Invalid image dimensions'));
                            }
                        } catch (fallbackError) {
                            console.error('Canvas conversion failed:', fallbackError.message || fallbackError);
                            // 確保總是傳遞有意義的錯誤訊息
                            const errorToReject = fallbackError instanceof Error ? 
                                fallbackError : 
                                new Error(`Canvas conversion failed: ${fallbackError}`);
                            reject(errorToReject);
                        }
                    }
                };

                script.onerror = reject;
                document.body.appendChild(script);
            } catch (error) {
                reject(error);
            }
        });
    }


    /**
     * 根據格式獲取 MIME 類型
     */
    getMimeType(format) {
        const mimeTypes = {
            'png': 'image/png',
            'jpg': 'image/jpeg',
            'jpeg': 'image/jpeg',
            'webp': 'image/webp'
        };
        return mimeTypes[format.toLowerCase()] || 'image/png';
    }
}

module.exports = new heicHeifConverter();