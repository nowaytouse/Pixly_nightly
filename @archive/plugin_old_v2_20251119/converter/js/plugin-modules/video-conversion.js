/**
 * 🎬 视频转换核心模块 (Rust CLI Backend)
 * Phase 40.34: 完整实现视频转换功能
 * 
 * 架构：
 * - UI层: index.html (视频面板)
 * - 逻辑层: video-conversion.js (本模块)
 * - 执行层: rust-cli-executor.js
 * - 后端: Rust CLI `video` 命令
 * 
 * 🔄 重构状态 (Phase 2):
 * - ✅ 使用 KernelBridge 统一通信
 * - ✅ 使用 ValidationBridge 验证
 * - ✅ 视频特定验证
 */

(function() {
    'use strict';
    
    let conversionCancelled = false;
    
    // 🔄 Bridge实例
    const kernelBridge = window.PIXLY?.kernelBridge;
    const validationBridge = window.PIXLY?.validationBridge;
    
    /**
     * 🎬 启动视频转换流程
     */
    async function startVideoConversion() {
        const log = window.pixlyLog;
        if (!log) {
            throw new Error('pixlyLog is required for video-conversion');
        }
        
        // 🔥 Phase 46.2: 防止重复点击 - 转换锁
        if (window.isConverting === true) {
            log.warn('PIXLY Video', formatLog(LOG.VIDEO_CONV_ALREADY_IN_PROGRESS, {}));
            return;
        }
        
        // 重置取消标志
        conversionCancelled = false;
        const Logger = window.PIXLY?.Logger || console;
        const selectedFiles = window.selectedFiles || [];
        
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_DIVIDER, {}));
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_START_CALLED, {}));
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_TIME, { time: new Date().toISOString() }));
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_SELECTED_FILES, { count: selectedFiles.length }));
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_DIVIDER, {}));
        
        // 🔥 防止Eagle文件监控干扰
        log.info('PIXLY Video Lifecycle', formatLog(LOG.VIDEO_CONV_SETTING_FLAG, {}));
        window.isConverting = true;
        
        // 🔥 隐藏开始按钮
        const startBtn = document.getElementById('convertBtn');
        if (startBtn) {
            startBtn.style.display = 'none';
            log.info('PIXLY Video UI', formatLog(LOG.VIDEO_CONV_START_BTN_HIDDEN, {}));
        }
        
        // 🔥 显示取消按钮
        const cancelBtn = document.getElementById('cancelBtn');
        if (cancelBtn) {
            cancelBtn.style.display = 'block';
            log.info('PIXLY Video UI', formatLog(LOG.VIDEO_CONV_CANCEL_BTN_SHOWN, {}));
        }
        
        // 检查文件
        if (!selectedFiles || selectedFiles.length === 0) {
            if (window.addLog) {
                window.addLog('请先选择视频文件', 'error');
            }
            resetUI();
            return;
        }
        
        // 🔥 Phase 45.4: AI文件验证（如果启用）
        if (window.fileValidator && document.getElementById('enableFileValidation')?.checked) {
            Logger.info('[Video Conversion]', '🔒 AI file validation enabled, starting validation...');
            if (window.addLog) {
                window.addLog('🔍 正在进行 AI 文件类型验证...', 'info');
            }
            
            try {
                const validationResults = await window.fileValidator.validateBatch(
                    selectedFiles,
                    (progress) => {
                        if (window.addLog) {
                            window.addLog(`验证进度: ${progress.percent}% (${progress.current}/${progress.total})`, 'info');
                        }
                    }
                );
                
                // 显示验证结果
                window.fileValidator.showBatchResults(validationResults);
                
                // 检查是否有可疑文件
                if (validationResults.suspicious > 0) {
                    const suspiciousFiles = validationResults.details
                        .filter(r => r.suspicious)
                        .map(r => r.file.name);
                    
                    Logger.warn('[Video Conversion]', `🚨 Found ${validationResults.suspicious} suspicious files`);
                    
                    // 询问用户是否继续
                    const continueConversion = await eagle.dialog.confirm({
                        title: '⚠️ 检测到可疑文件',
                        message: `以下文件可能存在类型伪装：\n\n${suspiciousFiles.slice(0, 5).map(f => `• ${f}`).join('\n')}${suspiciousFiles.length > 5 ? `\n...还有 ${suspiciousFiles.length - 5} 个文件` : ''}\n\n是否继续转换？`,
                        okText: '继续转换',
                        cancelText: '取消'
                    });
                    
                    if (!continueConversion) {
                        Logger.info('[Video Conversion]', '🛑 User cancelled conversion due to suspicious files');
                        if (window.addLog) {
                            window.addLog('已取消：检测到可疑文件', 'warning');
                        }
                        resetUI();
                        return;
                    }
                }
                
                Logger.info('[Video Conversion]', '✅ File validation complete');
            } catch (error) {
                Logger.error('[Video Conversion]', `❌ File validation failed: ${error.message}`);
                if (window.addLog) {
                    window.addLog(`验证失败: ${error.message}，继续转换...`, 'warning');
                }
                // 验证失败不阻止转换
            }
        }
        
        // 获取转换配置
        const config = getVideoConversionConfig();
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_CONFIG, { config: JSON.stringify(config) }));
        
        // 🔄 Phase 2: 视频转换前验证（使用ValidationBridge）
        if (validationBridge) {
            try {
                const files = selectedFiles.map(f => f.filePath);
                const validationResult = await validationBridge.validatePreConversion(files, {
                    format: config.format,
                    quality: config.quality,
                    speed: config.speed,
                    lossless: false,
                    mode: 'manual'
                });
                
                if (!validationResult.valid) {
                    Logger.error('[Video Conversion]', '❌ 验证失败', validationResult.errors);
                    validationBridge.displayDetailedReport(validationResult);
                    resetUI();
                    return;
                }
                
                Logger.info('[Video Conversion]', '✅ 验证通过', { level: validationResult.level });
                
                // 显示警告（如果有）
                if (validationResult.warnings.length > 0) {
                    validationBridge.displayValidationWarnings(validationResult.warnings);
                }
            } catch (error) {
                Logger.warn('[Video Conversion]', '⚠️ 验证系统不可用，继续转换', { error: error.message });
            }
        }
        
        // 显示进度条
        const progressSection = document.getElementById('progressSection');
        if (progressSection) {
            progressSection.style.display = 'block';
            log.info('PIXLY Video Progress', formatLog(LOG.VIDEO_CONV_PROGRESS_SHOWN, {}));
        }
        
        // 更新进度
        if (window.updateProgress) {
            const i18n = window.i18n || { t: (key) => key };
            window.updateProgress(0, selectedFiles.length, i18n.t('progress.preparing'));
        }
        
        // 统计
        let successCount = 0;
        let failCount = 0;
        const failedFiles = [];
        
        // 逐个转换
        for (let i = 0; i < selectedFiles.length; i++) {
            if (conversionCancelled) {
                log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_CANCELLED, {}));
                break;
            }
            
            const file = selectedFiles[i];
            const fileName = file.name || 'unknown';
            
            if (window.updateProgress) {
                const i18n = window.i18n || { t: (key) => key };
                window.updateProgress(i, selectedFiles.length, i18n.t('files.videoPreparingConversion', {filename: fileName}));
            }
            
            try {
                // 🔥 调用Rust CLI进行视频转换
                const result = await convertVideo(file, config, (progress) => {
                    if (window.updateSubProgress) {
                        window.updateSubProgress(i, selectedFiles.length, progress);
                    }
                });
                
                if (result && result.success) {
                    successCount++;
                    if (window.updateProgress) {
                        const i18n = window.i18n || { t: (key) => key };
                        window.updateProgress(i + 1, selectedFiles.length, i18n.t('files.videoCompleted', {filename: fileName}));
                    }
                } else {
                    failCount++;
                    failedFiles.push(fileName);
                    if (window.updateProgress) {
                        const i18n = window.i18n || { t: (key) => key };
                        window.updateProgress(i + 1, selectedFiles.length, i18n.t('files.videoFailed', {filename: fileName}), 0, true);
                    }
                }
            } catch (error) {
                log.error('PIXLY Video', formatLog(LOG.VIDEO_CONV_ERROR, { error: error.message }));
                failCount++;
                failedFiles.push(fileName);
                if (window.updateProgress) {
                    const i18n = window.i18n || { t: (key) => key };
                    window.updateProgress(i + 1, selectedFiles.length, i18n.t('files.videoFailed', {filename: fileName}), 0, true);
                }
            }
        }
        
        // 完成
        if (window.updateProgress) {
            const i18n = window.i18n || { t: (key) => key };
            window.updateProgress(selectedFiles.length, selectedFiles.length, i18n.t('progress.completed'));
        }
        
        // 显示结果
        showVideoConversionResult(successCount, failCount, failedFiles);
        
        // 恢复UI
        resetUI();
        
        log.info('PIXLY Video Lifecycle', formatLog(LOG.VIDEO_CONV_COMPLETE, {}));
        window.isConverting = false;
    }
    
    /**
     * 🎛️ 获取视频转换配置
     * 区分智能模式和手动模式
     */
    function getVideoConversionConfig() {
        const log = window.pixlyLog;
        if (!log) return {};
        
        // 检查当前模式
        const smartModeRadio = document.getElementById('videoModeRadioSmart');
        const isSmartMode = smartModeRadio && smartModeRadio.checked;
        
        if (isSmartMode) {
            // 🤖 智能模式：使用AI预测
            log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_SMART_MODE, {}));
            
            // 读取AI预设
            const presetRadio = document.querySelector('input[name="videoAIPreset"]:checked');
            const preset = presetRadio ? presetRadio.value : 'balanced';
            
            return {
                useAI: true,
                preset: preset,
                codec: 'auto',  // AI决定
                crf: null,      // AI决定
                speed: 'medium', // AI决定
                container: 'auto' // AI决定
            };
        } else {
            // 🎛️ 手动模式：读取UI选择
            log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_MANUAL_MODE, {}));
            
            // 读取编解码器（radio按钮组）
            const codecRadio = document.querySelector('input[name="videoCodec"]:checked');
            const codec = codecRadio ? codecRadio.value : 'h265';
            
            // 读取容器格式
            const containerSelect = document.getElementById('videoContainer');
            const container = containerSelect ? containerSelect.value : 'mp4';
            
            // CRF和Preset目前在UI中可能不存在，使用默认值
            const crfSlider = document.getElementById('videoCRF');
            const crf = crfSlider ? parseInt(crfSlider.value) : 23;
            
            const presetSelect = document.getElementById('videoPreset');
            const speed = presetSelect ? presetSelect.value : 'medium';
            
            log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_MANUAL_PARAMS, { params: JSON.stringify({ codec, container, crf, speed }) }));
            
            return {
                useAI: false,
                codec: codec,
                crf: crf,
                speed: speed,
                container: container
            };
        }
    }
    
    /**
     * 🎬 转换单个视频/动图文件
     * Phase 40.36: 支持GIF/APNG/WebP → 视频转换
     */
    async function convertVideo(file, config, onProgress) {
        const log = window.pixlyLog;
        if (!log) return { success: false, error: 'pixlyLog not available' };
        
        const rustCLI = window.rustCLI;
        if (!rustCLI || !rustCLI.available) {
            throw new Error('Rust CLI not available');
        }
        
        const input = file.filePath;
        const inputDir = input.substring(0, input.lastIndexOf('/'));
        const inputName = input.substring(input.lastIndexOf('/') + 1);
        const inputExt = inputName.substring(inputName.lastIndexOf('.') + 1).toLowerCase();
        const inputBaseName = inputName.substring(0, inputName.lastIndexOf('.'));
        
        // 🔥 Phase 40.36: 检测动图输入
        const isAnimatedImage = ['gif', 'apng', 'webp'].includes(inputExt);
        if (isAnimatedImage) {
            log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_ANIMATION_TO_VIDEO, { ext: inputExt.toUpperCase() }));
        }
        
        // 确定输出路径
        let outputExt = config.container || 'mp4';
        if (config.codec === 'vp9' || config.codec === 'av1') {
            outputExt = 'webm';
        }
        const output = `${inputDir}/${inputBaseName}.${outputExt}`;
        
        log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_CONVERTING, { input, output }));
        log.debug('PIXLY Video', formatLog(LOG.VIDEO_CONV_CONFIG, { config: JSON.stringify(config) }));
        
        // 构建Rust CLI参数
        const args = ['video', input, output];
        
        if (config.useAI) {
            args.push('--ai');
        }
        
        if (config.codec && config.codec !== 'auto') {
            args.push('--codec', config.codec);
        }
        
        if (config.crf !== null) {
            args.push('--crf', config.crf.toString());
        }
        
        if (config.speed && config.speed !== 'medium') {
            args.push('--preset', config.speed);
        }
        
        if (config.container && config.container !== 'auto') {
            args.push('--container', config.container);
        }
        
        log.debug('PIXLY Video', formatLog(LOG.VIDEO_CONV_CLI_ARGS, { args: JSON.stringify(args) }));
        
                // 🔥 Phase 40.37: 使用正确的方法名 execWithProgress
        // args[0]='video', args[1]=input, args[2]=output, args[3...]是选项
        // execWithProgress(command, args数组, callbacks)
        try {
            const stdout = await rustCLI.execWithProgress('video', args.slice(1), {                                                                             
                onStdout: (data) => {
                    log.trace('PIXLY Video', formatLog(LOG.VIDEO_CONV_RUST_OUTPUT, { data }));
                },
                onProgress: onProgress
            });
            
            // 🔥 Phase 40.39: 解析输出，返回正确的结果对象
            const success = stdout.includes('✅ Video conversion complete');
            
            // 🔥 Phase 46.1: 动图转视频原地替换
            // ✅ 遵循架构原则：文件删除由Rust内核处理（commands.rs）
            // JS层只负责日志输出，不直接操作文件
            if (success && isAnimatedImage) {
                log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_ANIM_COMPLETE, {}));
                log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_RUST_HANDLES_REPLACEMENT, {}));
                log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_ARCHITECTURE, {}));
            }
            
            return {
                success: success,
                stdout: stdout,
                error: success ? null : 'Conversion failed'
            };
        } catch (error) {
            log.error('PIXLY Video', formatLog(LOG.VIDEO_CONV_ERROR, { error: error.message }));
            return {
                success: false,
                stdout: error.stdout || '',
                error: error.message || 'Unknown error'
            };
        }
    }
    
    /**
     * 🎉 显示视频转换结果
     */
    function showVideoConversionResult(successCount, failCount, failedFiles) {
        const i18n = window.i18n || { t: (key) => key };
        if (failCount === 0) {
            // 全部成功
            eagle.notification.show({
                title: i18n.t('notification.videoAllSuccessTitle'),
                description: i18n.t('notification.videoAllSuccessBody', {count: successCount}),
                duration: 3000
            });
        } else if (successCount === 0) {
            // 全部失败
            eagle.notification.show({
                title: i18n.t('notification.videoAllFailedTitle'),
                description: i18n.t('notification.videoAllFailedBody', {count: failCount}),
                duration: 5000
            });
        } else {
            // 部分成功
            eagle.notification.show({
                title: i18n.t('notification.videoPartialSuccessTitle'),
                description: i18n.t('notification.videoPartialSuccessBody', {success: successCount, failed: failCount}),
                duration: 4000
            });
        }
    }
    
    /**
     * 🔄 恢复UI
     */
    function resetUI() {
        const startBtn = document.getElementById('convertBtn');
        if (startBtn) {
            startBtn.style.display = 'block';
        }
        
        const cancelBtn = document.getElementById('cancelBtn');
        if (cancelBtn) {
            cancelBtn.style.display = 'none';
        }
    }
    
    /**
     * 🛑 取消视频转换
     */
    function cancelVideoConversion() {
        const log = window.pixlyLog;
        if (log) {
            log.info('PIXLY Video', formatLog(LOG.VIDEO_CONV_CANCEL_REQUESTED, {}));
        }
        conversionCancelled = true;
    }
    
    // 导出到全局
    window.startVideoConversion = startVideoConversion;
    window.cancelVideoConversion = cancelVideoConversion;
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Video Conversion', '✅ Module loaded (Rust CLI Backend)');
        log.debug('PIXLY Video Conversion', `Global functions: startVideoConversion=${typeof window.startVideoConversion}, cancelVideoConversion=${typeof window.cancelVideoConversion}`);
    }
    
})();
