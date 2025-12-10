const path = require('path');
const fs = require('fs').promises;
const { spawn } = require('child_process');
const { validateOutputFile, getOutputPath } = require('../../validate');

class FFmpegConverter {

    constructor() {
        // 不支援透明背景的格式
        this.nonTransparentFormats = [
            'jpeg',
            'jpg',
            'jpe',
            'jfif',
            'bmp',
            'tiff',
            'tif'
        ];
    }

    /**
     * 檢查格式是否支援透明
     * @param {string} format 
     * @returns {boolean}
     */
    isTransparentFormat(format) {
        return !this.nonTransparentFormats.includes(format.toLowerCase());
    }

    /**
     * 轉換單個圖片
     * @param {string} src 來源圖片路徑
     * @param {Object} options 輸出設置
     * @param {string} options.output 輸出目錄路徑
     * @param {string} options.format 輸出格式 ('jpg'|'png'|'webp'|'bmp'|'tiff'|'gif'|'heic'|'heif'|'jxl')
     * @param {number} options.quality 輸出品質 (1-100)
     * @param {string} options.sizeType 尺寸調整類型 ('maxWidth'|'maxHeight'|'minWidth'|'minHeight'|'maxSide'|'minSide'|'exact')
     * @param {number} options.sizeValue 尺寸值
     * @param {string} options.fileName 新檔名
     * @param {number} options.width 指定寬度 (當 sizeType 為 'exact' 時使用)
     * @param {number} options.height 指定高度 (當 sizeType 為 'exact' 時使用)
     * @param {number} options.animatedFps 動畫 FPS (用於 GIF 和 WebP 動畫)
     * @param {Function} options.onProgress 進度回調
     * @returns {Promise<void>}
     */
    async convert(src, dest, options) {
        try {
            // 驗證輸入參數            
            if (!src) throw new Error('sourceMissing');
            if (!dest) throw new Error('outputMissing');
            if (!options.format) throw new Error('formatMissing');
            // if (!options.fileName) throw new Error('fileNameMissing');

            // 確保輸出目錄存在
            await fs.mkdir(dest, { recursive: true });

            const result = await this._convertWithFFmpeg(src, dest, options);

            // 如果是可取消模式，等待 promise 完成
            if (options._enableCancellation && result.promise) {
                await result.promise;
            }

            // 驗證輸出文件
            const outputPath = getOutputPath(dest, options);
            await validateOutputFile(outputPath, {
                minSize: 1,
                timeout: 5000
            });

            // 如果是可取消模式，返回包含 killProcess 的結果
            if (options._enableCancellation && result.killProcess) {
                return result;
            }

        } catch (error) {
            throw error;
        }

    }

    /** 
     * 時間字串轉換為秒數
     * 支援 HH:MM:SS(.ms/us) / 純秒 / 純微秒
     * @private
     */
    _timeToSeconds(s) {
        if (!s) return 0;
        const t = String(s).trim();
        if (/^\d+$/.test(t)) return Number(t) / 1_000_000; // 微秒
        if (/^\d+(\.\d+)?$/.test(t)) return Number(t);      // 純秒
        const m = t.match(/^(\d{1,2}):(\d{2}):(\d{2})(?:\.(\d+))?$/);
        if (!m) return 0;
        const [, hh, mm, ss, frac = '0'] = m;
        return (+hh) * 3600 + (+mm) * 60 + (+ss) + Number(`0.${frac}`);
    }

    /**
     * 從 ffmpeg stderr 提取當前時間（秒）
     * @private
     */
    _extractCurrentTimeSec(chunkStr) {
        // 1) out_time_ms=xxxxxx
        const ms = chunkStr.match(/out_time_ms=(\d+)/);
        if (ms) return Number(ms[1]) / 1_000_000;

        // 2) out_time=HH:MM:SS.xx
        const ot = chunkStr.match(/out_time=([0-9:.]+)/);
        if (ot) return this._timeToSeconds(ot[1]);

        // 3) time=HH:MM:SS.xx
        const tm = chunkStr.match(/time=([0-9:.]+)/);
        if (tm) return this._timeToSeconds(tm[1]);

        return null;
    }

    /**
     * 從 ffmpeg 開頭 stderr 提取 Duration
     * @private
     */
    _extractDurationFromHeader(allErrSoFar) {
        const m = allErrSoFar.match(/Duration:\s*([0-9:.]+)/);
        if (!m) return 0;
        return this._timeToSeconds(m[1]);
    }

    /**
     * 使用 ffprobe 取得總時長（秒）
     * @private
     */
    _probeDurationSeconds(ffprobePath, inputPath) {
        return new Promise((resolve) => {
            const args = ['-v', 'error', '-show_entries', 'format=duration', '-of', 'default=nk=1:nw=1', inputPath];
            const p = spawn(ffprobePath, args);
            let out = '';
            p.stdout.on('data', (d) => (out += d.toString()));
            p.on('close', (code) => {
                if (code === 0) {
                    const sec = parseFloat(out.trim());
                    if (isFinite(sec) && sec > 0) return resolve(sec);
                }
                resolve(0);
            });
            p.on('error', () => resolve(0));
        });
    }

    /**
     * 使用 FFmpeg 進行轉換
     * @private
     */
    async _convertWithFFmpeg(src, dest, options) {
        const ffmpegPaths = await eagle.extraModule.ffmpeg.getPaths();
        const ffmpegPath = ffmpegPaths.ffmpeg;
        const ffprobePath = ffmpegPaths.ffprobe || 'ffprobe';

        // 準備輸入參數
        const usingStdin = !!options.imageData;
        const inputArgs = usingStdin ? [
            '-f', 'rawvideo',
            '-pix_fmt', 'rgba',
            '-s', `${options.imageData[0].width}x${options.imageData[0].height}`,
            '-r', String(options.animatedFps || 25),
            '-i', 'pipe:0',
        ] : ['-i', src];

        let args = [
            ...inputArgs,
            '-y',
            '-threads', '0',
        ];

        const buildFFmpegArgs = require('./buildFFmpegArgs');
        args = [
            ...args,
            ...buildFFmpegArgs(src, dest, options),
        ]

        // console.log(args);

        let ffmpegProcess = null;

        // 先探測總秒數（優先 ffprobe；若 stdin 或 ffprobe 失敗再備援）
        let totalSec = 0;
        if (!usingStdin && src) {
            totalSec = await this._probeDurationSeconds(ffprobePath, src);
        }
        // stdin 輸入可依幀數推估
        if (usingStdin && options.animatedFps && Array.isArray(options.imageData)) {
            const f = options.imageData.length;
            if (f > 0 && options.animatedFps > 0) {
                totalSec = f / options.animatedFps;
            }
        }

        const promise = new Promise((resolve, reject) => {
            ffmpegProcess = global.processManager.spawn(ffmpegPath, args);

            let stderrAll = ''; // 保留完整錯誤訊息
            let lastPercent = -1; // 降噪，只在 >=1% 變化時回報
            let durationPatched = false; // 是否已從 header 解析到 Duration

            // 舊版相容：動畫幀數進度追蹤
            let progressRegex = /frame=\s*(\d+)/;
            let totalFrames = 0;
            let currentFrame = 0;

            if (usingStdin) {
                for (const frame of options.imageData) {
                    const imageBuffer = Buffer.from(frame.data);
                    ffmpegProcess.stdin.write(imageBuffer);
                }
                ffmpegProcess.stdin.end();
            }

            const emitProgress = (curr) => {
                if (typeof options.onProgress !== 'function') return;
                if (!isFinite(totalSec) || totalSec <= 0) return; // 沒有分母就不報
                const ratio = Math.min(curr / totalSec, 0.999); // 先最多到 99.9%
                const percent = Math.floor(ratio * 100);
                if (percent !== lastPercent && percent >= 0 && percent <= 99) {
                    lastPercent = percent;
                    options.onProgress(percent);
                }
            };

            ffmpegProcess.stderr.on('data', (data) => {
                const chunk = data.toString();
                stderrAll += chunk;
                // console.log(stderr);

                // 若尚未取得 totalSec，嘗試從 header 的 Duration 補齊
                if (!durationPatched && (!totalSec || totalSec <= 0)) {
                    const sec = this._extractDurationFromHeader(stderrAll);
                    if (sec > 0) {
                        totalSec = sec;
                        durationPatched = true;
                    }
                }

                // 優先使用時間進度解析（適用於影片）
                const curr = this._extractCurrentTimeSec(chunk);
                if (curr != null && totalSec > 0) {
                    emitProgress(curr);
                } else if (options.isAnimated) {
                    // 舊版動畫幀數進度（備援方案）
                    const match = progressRegex.exec(chunk);
                    if (match && match[1]) {
                        currentFrame = parseInt(match[1], 10);

                        // 如果我們不知道總幀數，假設為 100
                        if (totalFrames === 0) {
                            totalFrames = 100;
                        }

                        // 計算進度百分比
                        const progress = Math.min(Math.round((currentFrame / totalFrames) * 100), 99);

                        if (typeof options.onProgress === 'function') {
                            options.onProgress(progress);
                        }
                    }
                } else if (typeof options.onProgress === 'function') {
                    // 靜態圖片固定 50% 進度
                    options.onProgress(50);
                }
            });

            ffmpegProcess.on('close', async (code) => {
                // console.timeEnd("ffmpeg command")
                if (typeof options.onProgress === 'function') {
                    options.onProgress(100);
                }

                if (code === 0) {
                    // 動畫 png 轉檔後改名
                    try {
                        if (options.format === 'png' && options.isAnimated) {
                            const apngPath = path.join(dest, `${options.fileName}.apng`);
                            const pngPath = path.join(dest, `${options.fileName}.png`);
                            await fs.rename(apngPath, pngPath);
                        }
                    } catch (_) {
                        // 忽略改名失敗，不影響主流程
                    }
                    resolve();
                } else if (code === null || code === 130 || code === 143) {
                    // Process was killed (SIGINT or SIGTERM)
                    reject(new Error('conversionCancelled'));
                } else {
                    const stderr = stderrAll || '';
                    if (stderr.includes('Could not open file')) {
                        reject(new Error('outputPathCouldNotOpen'));
                    } else if (stderr.includes('Invalid data found when processing input')) {
                        reject(new Error("Invalid data found when processing input"));
                    } else {
                        reject(new Error(stderr));
                    }
                }
            });

            ffmpegProcess.on('error', (err) => {
                reject(err);
            });
        });

        // 如果支援可取消轉換模式，返回 promise 和 kill 函數
        if (options._enableCancellation) {
            return {
                promise,
                killProcess: () => {
                    if (ffmpegProcess && !ffmpegProcess.killed) {
                        ffmpegProcess.kill('SIGTERM');
                        // 如果 SIGTERM 沒用，1秒後用 SIGKILL
                        setTimeout(() => {
                            if (ffmpegProcess && !ffmpegProcess.killed) {
                                ffmpegProcess.kill('SIGKILL');
                            }
                        }, 1000);
                    }
                }
            };
        }

        // 向後相容：直接返回 promise
        return promise;
    }

}

module.exports = new FFmpegConverter();