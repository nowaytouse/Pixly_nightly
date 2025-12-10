const path = require('path');
const fs = require('fs').promises;
const Queue = require('./queue');

// 初始化全局 ProcessManager 單例
if (!global.processManager) {
    global.processManager = require('./processManager');
}

class FileConverter {

    constructor() {
        this.queue = null;
        this.isConverting = false;

        // 格式別名映射 - 讓多個副檔名使用同一個 handler
        this.formatAliases = {
            'jpeg': 'jpg',
            'jpe': 'jpg',
            'jfif': 'jpg',
            'tif': 'tiff',
            'heif': 'heic',
            'hif': 'heic',
            'mp4': 'video',
            'webm': 'video',
            'mov': 'video',
            'm4v': 'video',
            'mkv': 'video',
            // 可以在這裡添加更多別名映射
        };

        // 獲取支援的格式列表 - 與 main.js 保持一致的格式陣列
        // CRITICAL: 這個陣列必須與 src/assets/scripts/main.js 中的 suppertedFileTypes 保持同步
        this.supportedFormats = [
            'bmp',
            'exr',
            'gif',
            'hdr',
            'heic',
            'heif',
            'hif',
            'ico',
            'jpeg',
            'jpg',
            'jfif',
            'png',
            'svg',
            'tga',
            'tif',
            'tiff',
            'webp',
            'avif',
            'insp',
            'jxl',
            'jpe'
        ];
    }

    /**
     * 檢查格式是否支援
     * @param {string} ext 副檔名
     * @returns {boolean} 是否支援該格式
     */
    isFormatSupported(ext) {
        const normalizedExt = ext.toLowerCase();
        return this.supportedFormats.includes(normalizedExt);
    }

    /**
     * 取得實際的 handler 檔名
     * @param {string} ext 副檔名
     * @returns {string} handler 檔名
     */
    getHandlerName(ext) {
        const normalizedExt = ext.toLowerCase();
        return this.formatAliases[normalizedExt] || normalizedExt;
    }

    /**
     * 判斷是否應該直接複製檔案（無需轉換）
     * @param {string} inputExt 輸入副檔名
     * @param {Object} options 轉換選項
     * @returns {boolean} 是否應該直接複製
     */
    shouldCopyDirectly(inputExt, options) {
        const targetFormat = options.format.toLowerCase();
        const sourceExt = inputExt.toLowerCase();

        // 原始格式直接複製
        if (targetFormat === 'original') {
            return options.sizeType === 'original';
        }

        // 格式完全相同且無尺寸調整
        if (sourceExt === targetFormat) {
            return options.sizeType === 'original';
        }

        // 檢查別名映射下的格式等價性
        const sourceHandler = this._resolveActualHandler(sourceExt);
        const targetHandler = this._resolveActualHandler(targetFormat);

        return sourceHandler === targetHandler && options.sizeType === 'original';
    }

    /**
     * 解析實際處理器名稱（處理 video 特殊情況）
     * @private
     * @param {string} format 格式名稱
     * @returns {string} 實際處理器名稱
     */
    _resolveActualHandler(format) {
        const baseHandler = this.getHandlerName(format);
        // video 類型保持原始副檔名，其他格式使用別名映射結果
        return baseHandler === 'video' ? format : baseHandler;
    }

    /**
     * 直接複製檔案到輸出路徑
     * @param {string} src 來源檔案路徑
     * @param {string} dest 輸出目錄
     * @param {Object} options 轉換選項
     * @returns {Promise<Object>} 返回實際使用的檔名和輸出路徑
     */
    async copyFile(src, dest, options) {
        // 確保輸出目錄存在
        await fs.mkdir(dest, { recursive: true });

        // 確定輸出檔案的副檔名
        let outputExt = options.format;
        if (outputExt === 'original') {
            outputExt = path.parse(src).ext.slice(1);
        }

        // 直接使用提供的檔名
        const finalFileName = options.fileName;
        const finalOutputPath = path.join(dest, `${finalFileName}.${outputExt}`);

        // 直接複製檔案
        await fs.copyFile(src, finalOutputPath);

        // 返回實際使用的檔名（供上層更新）
        return { actualFileName: finalFileName, outputPath: finalOutputPath };
    }

    /**
     * Convert files with callback-based progress tracking
     * @param {Array|string} items - Array of conversion tasks or single file path
     * @param {Object|string} options - Configuration options or destination path
     * @param {Function} options.onTaskComplete - Called when each task completes
     * @param {Function} options.onProgress - Called with overall progress
     * @param {Function} options.onError - Called when task fails
     * @param {Function} options.onComplete - Called when all tasks complete
     * @returns {Promise<Object>} 轉換結果
     */
    async convert(items, options = {}) {
        // 支持舊的調用方式 (單個文件轉換) - 向後兼容
        if (typeof items === 'string') {
            // 構造一個任務對象
            const task = {
                src: items,
                options: {
                    ...options,
                    output: arguments[1] // dest parameter
                }
            };

            // 調用新的批量轉換方法，但只有一個任務
            const result = await this.convertBatch([task], {
                onTaskComplete: arguments[2]?.onProgress
            });

            return result.successfulTasks[0];
        }

        // 新的調用方式 (批量轉換)
        return this.convertBatch(items, options);
    }

    /**
     * 批次轉換檔案
     * @param {Array} tasks - Array of conversion tasks
     * @param {Object} options - Configuration options
     * @param {Function} options.onTaskComplete - Called when each task completes
     * @param {Function} options.onProgress - Called with overall progress
     * @param {Function} options.onError - Called when task fails
     * @param {Function} options.onComplete - Called when all tasks complete
     * @returns {Promise<Object>} 轉換結果
     */
    async convertBatch(tasks, options = {}) {
        const {
            onTaskComplete = () => { },
            onProgress = () => { },
            onError = () => { },
            onComplete = () => { },
            onCancelled = () => { }
        } = options;
        const results = [];
        const errors = [];
        let completedTasks = 0;
        const totalTasks = tasks.length;
        this.isConverting = true;

        console.log('[FileConverter] Starting batch conversion:', {
            taskCount: tasks.length,
            tasks: tasks.map(t => ({ src: t.src, format: t.options?.format }))
        });

        return new Promise((resolve, reject) => {
            // Reset queue if it was previously cancelled

            if (this.formatAliases[tasks[0].options?.format] === 'video') {
                this.queue = new Queue(1);
            } else {
                this.queue = new Queue(8);
            }

            if (this.queue.cancelled) {
                this.queue.reset();
            }

            // 為每個任務創建一個處理函數
            tasks.forEach((task) => {
                this.queue.push(async () => {
                    try {
                        // 合併選項
                        const taskOptions = {
                            ...task.options,
                            _enableCancellation: true, // 啟用取消模式
                            onProgress: (progress) => {
                                if (typeof onProgress === 'function') {
                                    // 計算總體進度
                                    const totalProgress = ((completedTasks + (progress / 100)) / totalTasks) * 100;
                                    onProgress(totalProgress);
                                }
                            }
                        };

                        const ext = path.parse(task.src).ext.slice(1).toLowerCase();

                        // 檢查是否應該直接複製檔案（原格式輸出且無尺寸調整）
                        const shouldCopy = this.shouldCopyDirectly(ext, taskOptions);
                        let actualFileName = taskOptions.fileName;
                        let actualOutputPath;

                        if (shouldCopy) {
                            const copyResult = await this.copyFile(task.src, taskOptions.output, taskOptions);
                            // 如果有返回實際檔名，更新任務選項
                            if (copyResult.actualFileName) {
                                actualFileName = copyResult.actualFileName;
                                taskOptions.fileName = actualFileName;
                            }
                            actualOutputPath = copyResult.outputPath;
                        } else {
                            // 非直接複製的檔案也需要執行時檢查
                            const outputExt = taskOptions.format === 'original'
                                ? path.parse(task.src).ext.slice(1)
                                : taskOptions.format;

                            // 直接使用提供的檔名（已在 main.js 處理過衝突）

                            const handlerName = this.getHandlerName(ext);
                            const converter = require(`../handlers/${handlerName}`);

                            // 呼叫 converter，它可能返回普通 Promise 或包含 killProcess 的物件
                            const conversionResult = converter.convert(task.src, taskOptions.output, taskOptions);


                            // 檢查是否是可取消的轉換
                            if (conversionResult && conversionResult.promise && conversionResult.killProcess) {

                                // 可取消模式：等待 promise 完成，但保留 killProcess 引用
                                await conversionResult.promise;

                                // 構建並返回包含 killProcess 的任務結果
                                actualOutputPath = path.join(taskOptions.output, `${actualFileName}.${taskOptions.format}`);

                                const taskResult = {
                                    src: task.src,
                                    success: true,
                                    outputPath: actualOutputPath,
                                    itemId: task.itemId,
                                    actualFileName: actualFileName
                                };

                                completedTasks++;

                                // 返回可取消的任務結果給 Queue
                                return {
                                    promise: Promise.resolve(taskResult),
                                    killProcess: conversionResult.killProcess
                                };
                            } else {
                                // 標準模式：直接等待轉換完成
                                await conversionResult;

                                // 構建實際的輸出路徑
                                actualOutputPath = path.join(taskOptions.output, `${actualFileName}.${taskOptions.format}`);
                            }
                        }

                        let result = {
                            src: task.src,
                            success: true,
                            outputPath: actualOutputPath,
                            itemId: task.itemId,
                            actualFileName: actualFileName // 新增：返回實際使用的檔名
                        };

                        // 如果是替換模式且不是直接複製，使用 Eagle API 的 replaceFile 方法
                        if (taskOptions.isReplaceMode && !shouldCopy && typeof eagle !== 'undefined') {
                            try {
                                console.log('[FileConverter] Replace mode: attempting to replace file in Eagle');
                                // 根據文檔，需要通過 filePath 來查找對應的 item
                                // 先獲取所有項目，然後通過 filePath 匹配
                                const allItems = await eagle.item.getAll();
                                const matchingItem = allItems.find(item => item.filePath === task.src);

                                if (matchingItem) {
                                    console.log('[FileConverter] Found matching item, replacing file:', {
                                        itemId: matchingItem.id,
                                        originalPath: task.src,
                                        newPath: result.outputPath
                                    });
                                    // 使用 Eagle API 的 replaceFile 方法替換檔案
                                    const replaceSuccess = await matchingItem.replaceFile(result.outputPath);
                                    if (replaceSuccess) {
                                        result.replacedInEagle = true;
                                        console.log('[FileConverter] Successfully replaced file in Eagle:', task.src);
                                        // 刪除臨時輸出檔案（因為已經替換到 Eagle 中）
                                        await fs.unlink(result.outputPath).catch((unlinkError) => {
                                            console.warn('[FileConverter] Failed to clean up temp file:', unlinkError.message);
                                        });
                                    } else {
                                        console.error('[FileConverter] Failed to replace file in Eagle:', task.src);
                                    }
                                } else {
                                    console.error('[FileConverter] Could not find matching Eagle item for:', task.src);
                                }
                            } catch (replaceError) {
                                console.error(`[FileConverter] Task processing failed:
                                    Error: ${error.message}
                                    Code: ${error.code || 'N/A'}
                                    Source: ${task.src}
                                    Format: ${task.options?.format || 'N/A'}
                                    Output: ${task.options?.output || 'N/A'}
                            `);
                                // 替換失敗不影響轉換成功，檔案仍然存在於輸出目錄
                            }
                        }

                        results.push(result);

                        if (typeof onTaskComplete === 'function') {
                            onTaskComplete(result);
                        }
                    } catch (error) {
                        // console.error(`[FileConverter] Task processing failed:
                        //         Error: ${error.message}
                        //         Code: ${error.code || 'N/A'}
                        //         Source: ${task.src}
                        //         Format: ${task.options?.format || 'N/A'}
                        //         Output: ${task.options?.output || 'N/A'}
                        // `);

                        console.error(error);

                        const errorResult = {
                            src: task.src,
                            success: false,
                            error: error.message,
                            itemId: task.itemId
                        };

                        errors.push(errorResult);

                        if (typeof onError === 'function') {
                            onError(errorResult);
                        }

                        if (typeof onTaskComplete === 'function') {
                            onTaskComplete(errorResult);
                        }

                        const outputPath = path.join(task.options.output, `${task.options.fileName}.${task.options.format}`);

                        await fs.unlink(outputPath);

                    }

                    completedTasks++;
                });
            });

            // 當所有任務完成時
            this.queue.onAllComplete(() => {
                this.isConverting = false;
                const finalResult = {
                    successfulTasks: results,
                    failedTasks: errors,
                    statistics: {
                        total: totalTasks,
                        successful: results.length,
                        failed: errors.length
                    },
                    // 向後相容
                    get results() { return this.successfulTasks; },
                    get errors() { return this.failedTasks; }
                };

                console.log('[FileConverter] Batch conversion completed:', {
                    total: totalTasks,
                    successful: results.length,
                    failed: errors.length,
                    successRate: totalTasks > 0 ? Math.round((results.length / totalTasks) * 100) + '%' : '0%'
                });

                if (this.queue.cancelled) {
                    console.log('[FileConverter] Conversion was cancelled');
                    finalResult.wasCancelled = true;
                    if (typeof onCancelled === 'function') {
                        onCancelled(finalResult);
                    }
                } else if (typeof onComplete === 'function') {
                    onComplete(finalResult);
                }

                resolve(finalResult);
            });
        });
    }

    /**
     * Cancel all pending conversion tasks
     */
    cancel() {
        console.log('[FileConverter] Cancelling all conversion tasks');
        global.processManager.killAll();
        this.queue.cancel();
        this.isConverting = false;
    }

    /**
     * Check if conversion is currently running
     * @returns {boolean}
     */
    isRunning() {
        return this.isConverting;
    }
}

// 導出單例實例
module.exports = new FileConverter();