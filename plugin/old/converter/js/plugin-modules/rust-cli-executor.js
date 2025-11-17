/**
 * @file 28-rust-cli-executor.js
 * @description Unified Rust CLI executor using path resolver
 */

(function(PIXLY) {
    'use strict';

    class RustCLIExecutor {
        constructor() {
            const log = window.pixlyLog;
            log.info('PIXLY Rust CLI Executor', LOG.RUST_CLI_INITIALIZING);
            
            this.path = null;
            this.version = null;
            this.available = false;
            this.error = null;
            
            this.init();
        }

        init() {
            const log = window.pixlyLog;
            log.info('PIXLY Rust CLI', LOG.RUST_CLI_CHECKING);
            
            // �� Check if path resolver is available
            if (!window.PIXLY_PATH_RESOLVER) {
                this.available = false;
                this.error = 'PIXLY_PATH_RESOLVER not available. Make sure 00-path-resolver.js is loaded first.';
                log.error('PIXLY Rust CLI', LOG.RUST_CLI_NOT_AVAILABLE, { error: this.error });
                return;
            }
            
            try {
                // 🔥 Get Rust CLI path from path resolver
                this.path = window.PIXLY_PATH_RESOLVER.getRustCLIPath();
                
                if (!this.path) {
                    throw new Error('未找到pixly-rust可执行文件 (检查路径: converter/bin/pixly-rust)');
                }
                
                log.info('PIXLY Rust CLI', `📍 使用路径: ${this.path}`);
                
                // Test the CLI
                const { execSync } = require('child_process');
                const versionOutput = execSync(`"${this.path}" --version`, {
                    encoding: 'utf8',
                    timeout: 5000
                }).trim();
                
                this.version = versionOutput.replace(/^pixly-rust\s+/, '');
                this.available = true;
                
                log.info('PIXLY Rust CLI', `✅ 统一Rust Kernel可用 (v${this.version})`);
            } catch (error) {
                this.available = false;
                this.error = error.message;
                log.error('PIXLY Rust CLI', `❌ 初始化失败: ${error.message}`);
            }
        }

        async exec(command, args = [], options = {}) {
            if (!this.available) {
                throw new Error('Rust CLI not available: ' + this.error);
            }
            
            // 🔥 Phase 40.7.10: 如果有onProgress回调，使用异步spawn
            if (options.onProgress && typeof options.onProgress === 'function') {
                return await this._execAsync(command, args, options.onProgress);
            }
            
            // 🔥 Phase 38: 使用spawnSync避免shell转义问题
            const { spawnSync } = require('child_process');
            
            // 合并command和args成完整参数列表
            const log = window.pixlyLog;
            const fullArgs = [command, ...args];
            
            if (log) {
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_EXECUTING, { command: `${this.path} ${fullArgs.join(' ')}` });
            }
            
            // 🔥 Phase 40: 修复PATH截断问题
            // Eagle插件的Node.js环境PATH不完整，需要显式添加Homebrew路径
            const fullPath = [
                '/opt/homebrew/bin',
                '/opt/homebrew/sbin',
                '/usr/local/bin',
                process.env.PATH || '/usr/bin:/bin:/usr/sbin:/sbin'
            ].join(':');
            
            if (log) {
                log.debug('PIXLY Rust CLI', LOG.RUST_CLI_USING_ENV_PATH, { path: fullPath.split(':').slice(0, 3).join(':') + '...' });
            }
            
            try {
                // 使用spawnSync with array args - 避免shell转义问题
                const result = spawnSync(this.path, fullArgs, {
                    encoding: 'utf8',
                    timeout: 300000,
                    env: {
                        ...process.env,
                        PATH: fullPath
                    }
                });
                
                const stdout = result.stdout || '';
                const stderr = result.stderr || '';
                
                // 打印stdout
                if (stdout) {
                    log.debug('PIXLY Rust CLI', LOG.RUST_CLI_STDOUT_HEADER);
                    stdout.split('\n').forEach(line => {
                        if (line.trim()) log.debug('PIXLY Rust CLI', LOG.RUST_CLI_STDOUT_LINE, { line });
                    });
                }
                
                // 打印stderr（如果有）
                if (stderr) {
                    log.error('PIXLY Rust CLI', LOG.RUST_CLI_STDERR_HEADER);
                    stderr.split('\n').forEach(line => {
                        if (line.trim()) log.error('PIXLY Rust CLI', LOG.RUST_CLI_STDERR_LINE, { line });
                    });
                }
                
                if (result.error) {
                    log.error('PIXLY Rust CLI', LOG.RUST_CLI_SPAWN_ERROR, { error: result.error.message });
                    throw result.error;
                }
                
                if (result.status !== 0) {
                    log.error('PIXLY Rust CLI', LOG.RUST_CLI_CMD_FAILED, { code: result.status });
                    
                    // 🔥 Phase 47.18 (E-002): JXL编码器异常处理
                    if (stderr.includes('cjxl') && stderr.includes('not found')) {
                        const jxlError = new Error('❌ JXL编码器未安装！\n\n请先安装JXL工具：\n• macOS: brew install jpeg-xl\n• Linux: apt install libjxl-tools\n• Windows: 下载jpeg-xl工具包');
                        jxlError.code = 'JXL_NOT_INSTALLED';
                        jxlError.stdout = stdout;
                        jxlError.stderr = stderr;
                        throw jxlError;
                    }
                    
                    // JXL编码器版本问题
                    if (stderr.includes('unknown option') && fullArgs.some(arg => arg.includes('jxl'))) {
                        const jxlError = new Error('❌ JXL编码器版本不兼容！\n\n您的JXL版本可能过旧，请更新：\n• macOS: brew upgrade jpeg-xl\n• 需要v0.10+版本');
                        jxlError.code = 'JXL_VERSION_MISMATCH';
                        jxlError.stdout = stdout;
                        jxlError.stderr = stderr;
                        throw jxlError;
                    }
                    
                    const error = new Error(`Command failed with exit code ${result.status}`);
                    error.stdout = stdout;
                    error.stderr = stderr;
                    error.status = result.status;
                    throw error;
                }
                
                return stdout;
                
            } catch (execError) {
                log.error('PIXLY Rust CLI', LOG.RUST_CLI_EXEC_ERROR, { error: execError.message });
                throw execError;
            }
        }
        
        /**
         * 🔥 Phase 40.7.10: 异步执行with实时进度回调
         * 解析FFmpeg的stderr输出来提供实时进度
         */
        async _execAsync(command, args, onProgress) {
            const { spawn } = require('child_process');
            const log = window.pixlyLog;
            
            const fullArgs = [command, ...args];
            const fullPath = [
                '/opt/homebrew/bin',
                '/opt/homebrew/sbin',
                '/usr/local/bin',
                process.env.PATH || '/usr/bin:/bin:/usr/sbin:/sbin'
            ].join(':');
            
            if (log) {
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_ASYNC_START, { command: `${this.path} ${fullArgs.join(' ')}` });
            }
            
            return new Promise((resolve, reject) => {
                const child = spawn(this.path, fullArgs, {
                    env: {
                        ...process.env,
                        PATH: fullPath
                    }
                });
                
                let stdout = '';
                let stderr = '';
                let totalFrames = 0;
                let currentFrame = 0;
                
                child.stdout.on('data', (data) => {
                    const text = data.toString();
                    stdout += text;
                    
                    // 🔥 Phase 40.7.12: 解析Rust输出的进度JSON
                    const lines = text.split('\n');
                    for (const line of lines) {
                        if (line.trim().startsWith('{"type":"progress"')) {
                            try {
                                const progress = JSON.parse(line.trim());
                                if (progress.percent !== undefined) {
                                    log.info('PIXLY Progress', LOG.RUST_CLI_PROGRESS_PERCENT, { percent: progress.percent, message: progress.message });
                                    onProgress(progress.percent);
                                }
                            } catch (e) {
                                // 忽略JSON解析错误
                            }
                        }
                    }
                });
                
                child.stderr.on('data', (data) => {
                    const text = data.toString();
                    stderr += text;
                    
                    // 解析FFmpeg进度
                    const lines = text.split('\n');
                    for (const line of lines) {
                        // 解析总时长: Duration: 00:00:07.00
                        const durationMatch = line.match(/Duration:\s*(\d{2}):(\d{2}):(\d{2}\.\d{2})/);
                        if (durationMatch) {
                            const hours = parseInt(durationMatch[1]);
                            const minutes = parseInt(durationMatch[2]);
                            const seconds = parseFloat(durationMatch[3]);
                            const totalSeconds = hours * 3600 + minutes * 60 + seconds;
                            // GIF一般是低帧率，估计总帧数
                            totalFrames = Math.ceil(totalSeconds * 10);
                            log.debug('PIXLY Progress', LOG.RUST_CLI_DURATION, { duration: totalSeconds, frames: totalFrames });
                        }
                        
                        // 解析当前帧: frame=    7 fps=1.2
                        const frameMatch = line.match(/frame=\s*(\d+)/);
                        if (frameMatch) {
                            currentFrame = parseInt(frameMatch[1]);
                            if (totalFrames > 0) {
                                const progress = Math.min(Math.round((currentFrame / totalFrames) * 100), 99);
                                log.debug('PIXLY Progress', LOG.RUST_CLI_PROGRESS_FRAMES, { current: currentFrame, total: totalFrames, percent: progress });
                                onProgress(progress);
                            } else {
                                log.debug('PIXLY Progress', LOG.RUST_CLI_FRAME_NO_TOTAL, { frame: currentFrame });
                                onProgress(Math.min((currentFrame / 10) * 100, 90)); // 没有总数时，缓慢增长到90%
                            }
                        }
                    }
                });
                
                child.on('error', (error) => {
                    log.error('PIXLY Rust CLI', LOG.RUST_CLI_ASYNC_SPAWN_ERROR, { error: error.message });
                    reject(error);
                });
                
                child.on('close', (code) => {
                    // 🔥 Phase 40.25: 显示所有stdout（移除slice限制）以查看完整的Eagle/XMP日志
                    if (stdout) {
                        log.debug('PIXLY Rust CLI', LOG.RUST_CLI_STDOUT_HEADER);
                        stdout.split('\n').forEach(line => {
                            if (line.trim()) log.debug('PIXLY Rust CLI', LOG.RUST_CLI_STDOUT_LINE, { line });
                        });
                    }
                    
                    if (code === 0) {
                        log.info('PIXLY Rust CLI', LOG.RUST_CLI_ASYNC_COMPLETED);
                        // 最后设置为100%
                        onProgress(100);
                        resolve(stdout);
                    } else {
                        log.error('PIXLY Rust CLI', LOG.RUST_CLI_ASYNC_FAILED, { code });
                        log.error('PIXLY Rust CLI', LOG.RUST_CLI_STDERR_EXCERPT);
                        stderr.split('\n').slice(-10).forEach(line => {
                            if (line.trim()) log.error('PIXLY Rust CLI', LOG.RUST_CLI_STDERR_LINE, { line });
                        });
                        const error = new Error(`Command failed with exit code ${code}`);
                        error.code = code;
                        error.stdout = stdout;
                        error.stderr = stderr;
                        reject(error);
                    }
                });
            });
        }

        /**
         * 🔥 Phase 40.7: Execute with progress callback (async spawn)
         * @param {string} command - Command to execute
         * @param {Array<string>} args - Command arguments
         * @param {Object} callbacks - Progress callbacks
         * @returns {Promise<string>} stdout
         */
        async execWithProgress(command, args = [], callbacks = {}) {
            if (!this.available) {
                throw new Error('Rust CLI not available: ' + this.error);
            }
            
            const log = window.pixlyLog;
            const { onProgress, onStdout, onStderr } = callbacks;
            const fullArgs = [command, ...args];
            
            if (log) {
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_EXEC_WITH_PROGRESS, { command: `${this.path} ${fullArgs.join(' ')}` });
            }
            
            const fullPath = [
                '/opt/homebrew/bin',
                '/opt/homebrew/sbin',
                '/usr/local/bin',
                process.env.PATH || '/usr/bin:/bin:/usr/sbin:/sbin'
            ].join(':');
            
            return new Promise((resolve, reject) => {
                const { spawn } = require('child_process');
                
                const child = spawn(this.path, fullArgs, {
                    env: {
                        ...process.env,
                        PATH: fullPath
                    }
                });
                
                let stdout = '';
                let stderr = '';
                
                child.stdout.on('data', (data) => {
                    const text = data.toString();
                    stdout += text;
                    
                    // 打印到console
                    text.split('\n').forEach(line => {
                        if (line.trim()) log.debug('PIXLY Rust CLI', LOG.RUST_CLI_OUTPUT_LINE, { line });
                    });
                    
                    // 回调
                    if (onStdout) onStdout(text);
                    
                    // 🔥 解析进度（如果包含✅/❌符号，说明完成）
                    if (text.includes('✅')) {
                        if (onProgress) onProgress(100, 'success');
                    } else if (text.includes('❌')) {
                        if (onProgress) onProgress(100, 'error');
                    }
                });
                
                child.stderr.on('data', (data) => {
                    const text = data.toString();
                    stderr += text;
                    
                    text.split('\n').forEach(line => {
                        if (line.trim()) log.error('PIXLY Rust CLI', LOG.RUST_CLI_STDERR_LINE_PREFIX, { line });
                    });
                    
                    if (onStderr) onStderr(text);
                });
                
                child.on('close', (code) => {
                    if (code === 0) {
                        resolve(stdout);
                    } else {
                        log.error('PIXLY Rust CLI', LOG.RUST_CLI_CMD_FAILED, { code });
                        const error = new Error(`Command failed with exit code ${code}`);
                        error.stdout = stdout;
                        error.stderr = stderr;
                        error.status = code;
                        reject(error);
                    }
                });
                
                child.on('error', (error) => {
                    log.error('PIXLY Rust CLI', LOG.RUST_CLI_PROCESS_ERROR, { error: error.message });
                    reject(error);
                });
            });
        }

        /**
         * Convert an image using Rust CLI
         * @param {string} input - Input file path
         * @param {string} output - Output file path
         * @param {Object} options - Conversion options
         * @returns {Promise<Object>} Conversion result
         */
        async convert(input, output, options = {}) {
            const {
                format: targetFormat,
                quality,
                distance,
                effort,
                lossless,
                mergeXmpSidecar,      // 🔥 Phase 40.29: XMP合并选项
                normalizeFilenames,   // 🔥 Phase 40.29: 文件名规范化选项
                optimizeMode,         // 🔥 Phase 46.5.13: 优化模式（size/balanced/quality/general）
                onProgress,  // 🔥 Phase 40.7.10: 接受进度回调
                // 🔥 Phase 40.31: AI高级选项
                enableBayesian,
                enablePPO,
                enableSmartQuality,
                enableAutoOptimize,
                enableVideoForAnim
            } = options;

            if (!targetFormat) {
                throw new Error('Target format is required');
            }

            const log = window.pixlyLog;
            log.info('PIXLY Rust CLI', LOG.RUST_CLI_CONVERTING, { input, output });

            // 构建命令参数（移除引号，避免双重引号）
            const args = ['convert', input, output, '--format', targetFormat];

            // JXL格式：不能同时使用quality和distance
            if (targetFormat === 'jxl' || targetFormat === 'jpegxl') {
                if (distance !== undefined && distance !== null) {
                    // 优先使用distance
                    args.push('--distance', distance.toString());
                } else if (quality !== undefined && quality !== null) {
                    // 使用quality
                    args.push('--quality', quality.toString());
                }

                if (effort !== undefined && effort !== null) {
                    args.push('--effort', effort.toString());
                }
            } else {
                // 其他格式：只使用quality
                if (quality !== undefined && quality !== null) {
                    args.push('--quality', quality.toString());
                }
            }

            // 🔥 Phase 38: 添加lossless参数支持
            if (lossless === true) {
                args.push('--lossless');
            }
            
            // 🔥 Phase 40.7: 始终保留动画
            args.push('--animated');
            
            // 🔥 修复：XMP合并控制 (Rust CLI默认关闭，检测到XMP时显式启用)
            if (mergeXmpSidecar === true) {
                args.push('--merge-xmp');
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_XMP_MERGE_ENABLED);
            }
            
            // 🔥 Phase 40.29: 文件名规范化 (Rust CLI默认关闭,只在明确开启时添加参数)
            if (normalizeFilenames === true) {
                args.push('--normalize-filenames');
            }
            
            // 🔥 Phase 40.31: AI高级选项控制 (Rust CLI默认开启,只在明确关闭时添加参数)
            if (enableBayesian === false) {
                args.push('--no-bayesian');
            }
            if (enablePPO === false) {
                args.push('--no-ppo');
            }
            if (enableSmartQuality === false) {
                args.push('--no-smart-quality');
            }
            if (enableAutoOptimize === false) {
                args.push('--no-auto-optimize');
            }
            if (enableVideoForAnim === false) {
                args.push('--no-video-for-anim');
            }
            
            // 🔥 Phase 46.5.13: 优化模式控制
            if (optimizeMode && optimizeMode !== 'balanced') {
                args.push('--optimize-mode', optimizeMode);
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_OPTIMIZE_MODE_SET, { mode: optimizeMode });
            }
            
            if (log) {
                log.debug('PIXLY Rust CLI', LOG.RUST_CLI_COMMAND_ARGS, { args: args.join(' ') });
            }
            
            // 🔥 Phase 40.7.10: 传递onProgress到exec
            return await this.exec(args[0], args.slice(1), { onProgress });
        }

        /**
         * 🔥 Phase 38: convertImage() 适配器方法
         * 
         * 架构原则：保持向后兼容性
         * - 04-conversion-core.js 使用这个方法
         * - 内部适配到 convert() 方法
         * - 真实实现，不是摆设
         * 
         * @param {Object} options - Conversion options (对象参数)
         * @param {string} options.input - Input file path
         * @param {string} options.output - Output file path
         * @param {string} options.format - Target format
         * @param {number} options.quality - Quality (0-100)
         * @param {boolean} options.lossless - Lossless mode
         * @param {number} options.effort - Encoding effort (0-9)
         * @returns {Promise<Object>} Conversion result with success status
         */
        async convertImage(options) {
            if (!this.available) {
                throw new Error('Rust CLI is not available');
            }

            const {
                input,
                output,
                format,
                quality,
                lossless,
                effort,
                onProgress  // 🔥 Phase 40.7.10: 接受进度回调
            } = options;

            // 🔥 Phase 40.29: 从UI checkbox读取快捷工具选项
            const mergeXmpCheckbox = document.getElementById('autoMergeXmp');
            const normalizeCheckbox = document.getElementById('autoNormalize');
            
            // 🔥 修复：XMP合并根据文件检测自动决定
            const hasXMP = window.selectedFiles && window.selectedFiles.some(f => {
                const ext = (f.ext || '').toLowerCase();
                return ext === '.xmp' || ext === 'xmp';
            });
            const mergeXmpSidecar = mergeXmpCheckbox ? mergeXmpCheckbox.checked : hasXMP;  // 检测到XMP时自动启用
            const normalizeFilenames = normalizeCheckbox ? normalizeCheckbox.checked : false;  // 默认关闭
            
            const log = window.pixlyLog;
            if (hasXMP) {
                log.info('PIXLY Rust CLI', LOG.RUST_CLI_XMP_DETECTED);
            }
            
            // 🔥 Phase 40.31: 从UI checkbox读取AI高级选项
            const enableBayesianCheckbox = document.getElementById('enableBayesian');
            const enablePPOCheckbox = document.getElementById('enablePPO');
            const enableSmartQualityCheckbox = document.getElementById('enableSmartQuality');
            const enableAutoOptimizeCheckbox = document.getElementById('enableAutoOptimize');
            const enableVideoForAnimCheckbox = document.getElementById('enableVideoForAnimation');
            
            const enableBayesian = enableBayesianCheckbox ? enableBayesianCheckbox.checked : true;
            const enablePPO = enablePPOCheckbox ? enablePPOCheckbox.checked : true;
            const enableSmartQuality = enableSmartQualityCheckbox ? enableSmartQualityCheckbox.checked : true;
            const enableAutoOptimize = enableAutoOptimizeCheckbox ? enableAutoOptimizeCheckbox.checked : true;
            const enableVideoForAnim = enableVideoForAnimCheckbox ? enableVideoForAnimCheckbox.checked : true;

            // 🔥 Phase 40.31: 显示AI选项状态
            const aiOptions = { enableBayesian, enablePPO, enableSmartQuality, enableAutoOptimize, enableVideoForAnim };
            if (log) {
                log.debug('PIXLY Rust CLI', LOG.RUST_CLI_AI_OPTIONS, { options: JSON.stringify(aiOptions) });
            }

            log.info('PIXLY Rust CLI', LOG.RUST_CLI_CONVERTING_IMAGE);
            const convertOptions = { input, output, format, quality, lossless, effort, mergeXmpSidecar, normalizeFilenames, hasProgressCallback: !!onProgress };
            if (log) {
                log.debug('PIXLY Rust CLI', LOG.RUST_CLI_CONVERT_OPTIONS, { options: JSON.stringify(convertOptions) });
            }

            try {
                // 🔥 Phase 40.7.10: 传递onProgress到convert()方法
                // 🔥 Phase 40.29: 传递XMP和文件名规范化选项
                // 🔥 Phase 40.31: 传递AI高级选项
                const result = await this.convert(input, output, {
                    format,
                    quality,
                    mergeXmpSidecar,
                    normalizeFilenames,
                    lossless,
                    effort,
                    onProgress,
                    // Phase 40.31: AI高级选项
                    enableBayesian,
                    enablePPO,
                    enableSmartQuality,
                    enableAutoOptimize,
                    enableVideoForAnim
                });

                // 返回格式兼容04-conversion-core.js的期望
                return {
                    success: true,
                    output,
                    ...result
                };
            } catch (error) {
                log.error('PIXLY Rust CLI', LOG.RUST_CLI_CONVERT_FAILED, { error: error.message });
                return {
                    success: false,
                    error: error.message
                };
            }
        }

        isAvailable() {
            return this.available;
        }

        async test() {
            const log = window.pixlyLog;
            log.info('PIXLY Rust CLI', LOG.RUST_CLI_TESTING);
            return this.available ? { success: true } : { success: false, error: this.error };
                }

        /**
         * 🔥 Phase 40.7.15: 分析文件元数据（从Rust获取）
         * @param {string} filePath - 文件路径
         * @returns {Promise<Object>} 文件分析结果
         */
        async analyzeFile(filePath) {
            try {
                const result = await this.exec('file-info', [filePath]);
                
                // 🔥 Phase 40.7.15: 修复JSON解析（result可能是对象或字符串）
                let analysis;
                if (typeof result === 'string') {
                    analysis = JSON.parse(result);
                } else if (result && result.stdout) {
                    analysis = JSON.parse(result.stdout);
                } else {
                    throw new Error('Invalid result format');
                }
                
                const log = window.pixlyLog;
                if (log) {
                    log.debug('PIXLY Rust CLI', LOG.RUST_CLI_FILE_ANALYSIS, { analysis: JSON.stringify(analysis) });
                }
                return analysis;
            } catch (error) {
                // 🔥 Phase 40.8: 根除Fallback - 真实错误必须向上传播
                const log = window.pixlyLog;
                log.error('PIXLY Rust CLI', LOG.RUST_CLI_ANALYZE_FAILED, { error: error.message });
                log.warn('PIXLY Rust CLI', LOG.RUST_CLI_ANALYZE_REQUIRED);
                throw new Error(`File analysis failed: ${error.message}`);
            }
        }
    }

    // 创建全局实例
    const log = window.pixlyLog;
    log.info('PIXLY Rust CLI', LOG.RUST_CLI_CREATING_INSTANCE);
    window.rustCLI = new RustCLIExecutor();
    
    log.info('PIXLY Rust CLI Executor', LOG.RUST_CLI_MODULE_LOADED);
    log.info('PIXLY Rust CLI', LOG.RUST_CLI_STATUS, { status: window.rustCLI.available ? 'Available' : 'Not Available' });
    if (window.rustCLI.available) {
        log.info('PIXLY Rust CLI', LOG.RUST_CLI_VERSION_INFO, { version: window.rustCLI.version });
    }
    if (window.rustCLI.path) {
        log.info('PIXLY Rust CLI', LOG.RUST_CLI_PATH_INFO, { path: window.rustCLI.path });
    }
    
    // 注册到PIXLY命名空间
    if (PIXLY) {
        PIXLY.rustCLI = window.rustCLI;
    }
    
})(window.PIXLY);
