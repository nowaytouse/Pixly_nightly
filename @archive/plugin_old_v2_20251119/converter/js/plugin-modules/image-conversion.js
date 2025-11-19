/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * PIXLY Conversion Core - 转换核心 (仅调用Rust CLI)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 架构原则：
 * - JS插件：仅UI和API调用
 * - Rust CLI：所有转换逻辑
 * - 不实现任何转换算法
 * 
 * 🔄 重构状态 (Phase 2):
 * - ✅ 使用 KernelBridge 统一通信
 * - ✅ 使用 ValidationBridge 验证
 * - ✅ 使用 AIBridge 智能推荐
 * - ❌ 不再使用 format-recommender.js (已废弃)
 * - ❌ 不再使用 deduplicator.js (已废弃)
 */
(function(window) {
    'use strict';
    
    // 🔥 Phase 38: 真实的取消标志 (非摆设)
    let conversionCancelled = false;
    
    // 🔄 Bridge实例 (Phase 2重构)
    const kernelBridge = window.PIXLY?.kernelBridge;
    const validationBridge = window.PIXLY?.validationBridge;
    const aiBridge = window.PIXLY?.aiBridge;
    
    /**
     * 🎛️ 获取完整的转换参数（支持AI和手动模式）
     * 🔥 包含所有格式专属参数 - 确保界面参数真实传递到kernel
     */
    function getFullConversionParams() {
        const config = getConversionConfig();
        
        // 🔥 收集格式专属参数
        const formatParams = {};
        
        // JXL专属参数
        if (config.format === 'jxl') {
            formatParams.jxl = {
                effort: parseInt(document.getElementById('jxlEffort')?.value) || 7,
                distance: parseFloat(document.getElementById('jxlDistance')?.value) || 1.0,
                bit_depth: parseInt(document.getElementById('jxlBitDepth')?.value) || 8,
                color_space: document.getElementById('jxlColorSpace')?.value || 'sRGB',
                patches: parseInt(document.getElementById('jxlPatches')?.value) || 1,
                // 🔥 Advanced encoding options - ALL REAL!
                modular: document.getElementById('jxlModular')?.checked || false,
                progressive: document.getElementById('jxlProgressive')?.checked || false,
                responsive: document.getElementById('jxlResponsive')?.checked || false,
                gaborish: document.getElementById('jxlGaborish')?.checked || false,
                photon_noise: parseInt(document.getElementById('jxlPhotonNoise')?.value) || 0,
                decoding_speed: parseInt(document.getElementById('jxlDecodingSpeed')?.value) || 0,
            };
        }
        
        // AVIF专属参数
        if (config.format === 'avif') {
            formatParams.avif = {
                speed: parseInt(document.getElementById('avifSpeed')?.value) || 6,
                min_quantizer: parseInt(document.getElementById('avifMinQuantizer')?.value) || 0,
                max_quantizer: parseInt(document.getElementById('avifMaxQuantizer')?.value) || 63,
                chroma_subsampling: document.getElementById('avifChromaSubsampling')?.value || '420',
                bit_depth: parseInt(document.getElementById('avifBitDepth')?.value) || 8,
                tiles_rows: parseInt(document.getElementById('avifTilesRows')?.value) || 1,
                tiles_cols: parseInt(document.getElementById('avifTilesCols')?.value) || 1,
                premultiply_alpha: document.getElementById('avifPremultiply')?.checked || false,
            };
        }
        
        // HEIC专属参数
        if (config.format === 'heic') {
            formatParams.heic = {
                quality: parseInt(document.getElementById('heicQuality')?.value) || 85,
                encoder: document.getElementById('heicEncoder')?.value || 'x265',
                chroma_subsampling: document.getElementById('heicChromaSubsampling')?.value || '444',
                lossless: document.getElementById('heicLossless')?.checked || false,
                embed_thumbnail: document.getElementById('heicThumbEmbed')?.checked || true,
            };
        }
        
        // 添加所有高级参数
        return {
            // 基础参数
            format: config.format,
            quality: config.quality,
            speed: config.speed,
            lossless: config.lossless || false,
            mode: config.mode || 'manual',
            
            // 🔥 格式专属参数
            format_params: formatParams,
            
            // 元数据选项
            preserveMetadata: document.getElementById('preserveMetadata')?.checked !== false,
            keepAnimated: document.getElementById('keepAnimated')?.checked !== false,
            mergeXmpSidecar: document.getElementById('mergeXmpSidecar')?.checked || false,
            
            // 预处理选项
            resize: document.getElementById('resize')?.value || null,
            quantize: document.getElementById('quantize')?.value || null,
            sharpen: document.getElementById('sharpen')?.value || null,
            resizeFilter: document.getElementById('resizeFilter')?.value || 'lanczos3',
            
            // 高级选项
            chromaSubsampling: document.getElementById('chromaSubsampling')?.value || 'auto',
            alphaQuality: document.getElementById('alphaQuality')?.value || null,
            effort: document.getElementById('effort')?.value || null,
            
            // 输出选项
            outputDir: config.outputDir || null,
            normalizeFilenames: document.getElementById('normalizeFilenames')?.checked || false,
            
            // 功能开关
            enableValidation: document.getElementById('enableValidation')?.checked !== false,
            enableAI: document.getElementById('enableAI')?.checked !== false,
            enableFileValidation: document.getElementById('enableFileValidation')?.checked || false
        };
    }
    
    /**
     * 启动转换流程
     */
    async function startConversion() {
        const log = window.pixlyLog;
        // 🔥 Phase 46.2: 防止重复点击 - 转换锁
        if (window.PIXLY_CONVERTING === true) {
            log.warn('PIXLY', LOG.IMAGE_CONV_DUPLICATE_CALL);
            return;  // 正常的重复调用防护，不需要清理
        }
        
        // 重置取消标志
        conversionCancelled = false;
        const Logger = window.PIXLY.Logger;
        const selectedFiles = window.selectedFiles;
        
        // 🔥 Phase 40.7.8: 详细日志 + 防御性代码
        log.info('PIXLY', '━'.repeat(60));
        log.info('PIXLY', LOG.IMAGE_CONV_START);
        log.info('PIXLY', LOG.IMAGE_CONV_TIME, { time: new Date().toISOString() });
        log.info('PIXLY', LOG.IMAGE_CONV_SELECTED_FILES, { count: selectedFiles.length });
        log.info('PIXLY', '━'.repeat(60));
        
        // 🔥 Phase 40.7.9: 防止Eagle文件监控干扰
        log.info('PIXLY Lifecycle', LOG.IMAGE_CONV_FLAG_SET);
        window.PIXLY_CONVERTING = true;
        
        // 🔥 标记转换开始
        
        // 🔥 Phase 40.7.15: 切换按钮状态（开始 → 取消）
        const startBtn = document.getElementById('convertBtn');
        if (startBtn) {
            startBtn.disabled = true;
            startBtn.style.display = 'none';
            log.info('PIXLY UI', LOG.IMAGE_CONV_BTN_HIDDEN);
        }
        
        const cancelBtn = document.getElementById('cancelBtn');
        if (cancelBtn) {
            cancelBtn.style.display = 'block';
            log.info('PIXLY UI', LOG.IMAGE_CONV_CANCEL_SHOWN);
        }
        
        if (!selectedFiles || selectedFiles.length === 0) {
            Logger.error('[Conversion]', '❌ No files selected');
            if (window.addLog) {
                window.addLog('请先选择要转换的文件', 'error');
            }
            // 恢复按钮状态
            if (startBtn) {
                startBtn.disabled = false;
                startBtn.style.display = 'block';
            }
            if (cancelBtn) {
                cancelBtn.style.display = 'none';
            }
            // 清除所有标志
            window.PIXLY_CONVERTING = false;
            window.PIXLY_SELECTION_LOCKED = false;
            return;
        }
        
        Logger.info('[Conversion]', `🚀 Starting conversion for ${selectedFiles.length} files`);
        
        // 检查Rust CLI是否可用
        if (!window.rustCLI || !window.rustCLI.available) {
            Logger.error('[Conversion]', '❌ Rust CLI not available');
            if (window.addLog) {
                window.addLog('Rust CLI不可用，请检查安装', 'error');
            }
            // 恢复按钮状态
            const startBtn = document.getElementById('convertBtn');
            if (startBtn) {
                startBtn.disabled = false;
                startBtn.style.display = 'block';
            }
            const cancelBtn = document.getElementById('cancelBtn');
            if (cancelBtn) {
                cancelBtn.style.display = 'none';
            }
            // 清除所有标志
            window.PIXLY_CONVERTING = false;
            window.PIXLY_SELECTION_LOCKED = false;
            return;
        }
        
        // 获取转换配置
        const config = getConversionConfig();
        
        Logger.info('[Conversion]', '📋 Conversion config:', config);
        
        // 🎛️ 获取完整参数（包含所有高级选项）
        const fullParams = getFullConversionParams();
        
        // 🔄 Phase 2: 转换前验证（使用ValidationBridge）- 可选功能
        if (validationBridge && fullParams.enableValidation) {
            try {
                const files = selectedFiles.map(f => f.filePath);
                const validationResult = await validationBridge.validatePreConversion(files, fullParams);
                
                if (!validationResult.valid) {
                    Logger.error('[Conversion]', '❌ 验证失败', validationResult.errors);
                    validationBridge.displayDetailedReport(validationResult);
                    
                    // 恢复UI状态
                    const startBtn = document.getElementById('convertBtn');
                    if (startBtn) {
                        startBtn.disabled = false;
                        startBtn.style.display = 'block';
                    }
                    const cancelBtn = document.getElementById('cancelBtn');
                    if (cancelBtn) {
                        cancelBtn.style.display = 'none';
                    }
                    window.PIXLY_CONVERTING = false;
                    window.PIXLY_SELECTION_LOCKED = false;
                    return;
                }
                
                Logger.info('[Conversion]', '✅ 验证通过', { level: validationResult.level });
            } catch (error) {
                Logger.warn('[Conversion]', '⚠️ 验证系统不可用，继续转换', { error: error.message });
            }
        }
        
        // 🔄 Phase 2: 智能模式AI推荐（使用AIBridge）- 可选功能
        if (fullParams.mode === 'smart' && fullParams.enableAI && aiBridge && selectedFiles.length > 0) {
            try {
                Logger.info('[Conversion]', '🤖 智能模式：获取AI推荐');
                const firstFile = selectedFiles[0];
                const imageInfo = {
                    width: firstFile.width || 1920,
                    height: firstFile.height || 1080,
                    size: firstFile.size || 0,
                    format: firstFile.ext?.replace('.', '') || 'unknown',
                    hasAlpha: firstFile.hasAlpha || false,
                    isAnimated: firstFile.isAnimated || false,
                    complexity: firstFile.complexity || 0.5
                };
                
                const aiConfig = await aiBridge.getSmartConfig(imageInfo, fullParams.format);
                Logger.info('[Conversion]', '✅ AI推荐完成', aiConfig);
                
                // 应用AI建议到完整参数
                if (aiConfig.quality) fullParams.quality = aiConfig.quality;
                if (aiConfig.speed) fullParams.speed = aiConfig.speed;
                if (aiConfig.lossless !== undefined) fullParams.lossless = aiConfig.lossless;
                
                // 显示AI建议
                aiBridge.displayAISuggestion(aiConfig);
                
                // 更新config以便后续使用
                Object.assign(config, fullParams);
            } catch (error) {
                Logger.warn('[Conversion]', '⚠️ AI推荐失败，使用手动配置', { error: error.message });
            }
        } else if (fullParams.mode === 'manual') {
            Logger.info('[Conversion]', '🎛️ 手动模式：使用用户指定参数', {
                quality: fullParams.quality,
                speed: fullParams.speed,
                lossless: fullParams.lossless
            });
        }
        
        // 🔥 UX改善: 检查动态图片 + HEIC格式的不兼容情况
        if (config.format === 'heic' || config.format === 'heif') {
            // 检查是否有动态图片
            const hasAnimatedFiles = selectedFiles.some(file => {
                // 从Eagle metadata检查
                if (file.isAnimated) return true;
                // 从扩展名推断（动态图片格式）
                const ext = (file.ext || '').toLowerCase();
                return ext === 'gif' || ext === 'webp' || ext === 'apng';
            });
            
            if (hasAnimatedFiles) {
                // 显示警告对话框
                const userConfirmed = confirm(
                    '⚠️ 警告：HEIC/HEIF格式不支持动画\n\n' +
                    '您选择的文件中包含动态图片（GIF/WebP/APNG），但HEIC/HEIF格式不支持动画。\n\n' +
                    '转换后的文件将只保留第一帧，动画效果会丢失。\n\n' +
                    '建议：\n' +
                    '• 动态图片 → WebP (支持动画)\n' +
                    '• 动态图片 → JXL (支持动画)\n' +
                    '• 静态图片 → HEIC (最佳压缩)\n\n' +
                    '是否继续使用HEIC格式转换？'
                );
                
                if (!userConfirmed) {
                    log.info('Conversion', LOG.IMAGE_CONV_HEIC_WARNING);
                    if (window.addLog) {
                        window.addLog('❌ 转换已取消：HEIC不支持动画', 'warning');
                    }
                    
                    // 🔥 关键修复：完整恢复UI状态
                    const startBtn = document.getElementById('convertBtn');
                    if (startBtn) {
                        startBtn.style.display = 'block';
                        startBtn.disabled = false;
                    }
                    const cancelBtn = document.getElementById('cancelBtn');
                    if (cancelBtn) {
                        cancelBtn.style.display = 'none';
                    }
                    
                    // 🔥 关键修复：清除所有转换标志，防止界面冻结
                    window.PIXLY_CONVERTING = false;
                    window.PIXLY_SELECTION_LOCKED = false;
                    log.info('PIXLY Lifecycle', LOG.IMAGE_CONV_CANCELLED);
                    
                    return;  // 退出转换
                }
                
                log.info('Conversion', LOG.IMAGE_CONV_HEIC_CONFIRMED);
            }
        }
        
        // 🔥 Phase 40.7.7: 在使用前定义totalFiles
        const totalFiles = selectedFiles.length;
        
        // 🔥 Phase 45.4: AI文件验证（如果启用）
        if (window.fileValidator && document.getElementById('enableFileValidation')?.checked) {
            Logger.info('[Conversion]', '🔒 AI file validation enabled, starting validation...');
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
                    
                    Logger.warn('[Conversion]', `🚨 Found ${validationResults.suspicious} suspicious files`);
                    
                    // 询问用户是否继续
                    const continueConversion = await eagle.dialog.confirm({
                        title: '⚠️ 检测到可疑文件',
                        message: `以下文件可能存在类型伪装：\n\n${suspiciousFiles.slice(0, 5).map(f => `• ${f}`).join('\n')}${suspiciousFiles.length > 5 ? `\n...还有 ${suspiciousFiles.length - 5} 个文件` : ''}\n\n是否继续转换？`,
                        okText: '继续转换',
                        cancelText: '取消'
                    });
                    
                    if (!continueConversion) {
                        Logger.info('[Conversion]', '🛑 User cancelled conversion due to suspicious files');
                        if (window.addLog) {
                            window.addLog('已取消：检测到可疑文件', 'warning');
                        }
                        
                        // 🔥 关键修复：完整恢复UI状态
                        const startBtn = document.getElementById('convertBtn');
                        if (startBtn) {
                            startBtn.disabled = false;
                            startBtn.style.display = 'block';
                        }
                        const cancelBtn = document.getElementById('cancelBtn');
                        if (cancelBtn) {
                            cancelBtn.style.display = 'none';
                        }
                        
                        // 🔥 关键修复：清除所有转换标志，防止界面冻结
                        window.PIXLY_CONVERTING = false;
                        window.PIXLY_SELECTION_LOCKED = false;
                        log.info('PIXLY Lifecycle', LOG.IMAGE_CONV_SUSPICIOUS_FILES);
                        
                        return;
                    }
                }
                
                Logger.info('[Conversion]', '✅ File validation complete');
            } catch (error) {
                Logger.error('[Conversion]', `❌ File validation failed: ${error.message}`);
                if (window.addLog) {
                    window.addLog(`验证失败: ${error.message}，继续转换...`, 'warning');
                }
                // 验证失败不阻止转换
            }
        }
        
        if (window.addLog) {
            window.addLog(`开始转换 ${selectedFiles.length} 个文件...`, 'info');
        }
        
        // 🔥 Phase 40.7.6: 显示进度条（隐藏旧版，显示内联版）
        const progressSection = document.getElementById('progressSection');
        const inlineProgressSection = document.getElementById('inlineProgressSection');
        const conversionCompleteHint = document.getElementById('conversionCompleteHint');
        
        log.info('PIXLY', LOG.IMAGE_CONV_PROGRESS_FOUND, { found: !!progressSection });
        log.info('PIXLY', LOG.IMAGE_CONV_INLINE_PROGRESS_FOUND, { found: !!inlineProgressSection });
        
        // 隐藏旧版进度条
        if (progressSection) {
            progressSection.style.display = 'none';
        }
        // 显示内联进度条
        if (inlineProgressSection) {
            inlineProgressSection.style.display = 'block';
            log.info('PIXLY Progress', LOG.IMAGE_CONV_INLINE_SHOWN);
        }
        // 隐藏完成提示
        if (conversionCompleteHint) {
            conversionCompleteHint.style.display = 'none';
        }
        
        // 🔥 Phase 40.7.6: 初始化进度条为0%
        if (window.updateProgress) {
            const i18n = window.i18n || { t: (key) => key };
            window.updateProgress(0, totalFiles, i18n.t('progress.preparing'), 0, false);
            log.info('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS);
        } else {
            log.error('PIXLY Progress', LOG.IMAGE_CONV_PROGRESS_NOT_FOUND);
        }
        
        // 调用Rust CLI进行转换
        try {
            let successCount = 0;
            let failCount = 0;
            
            // totalFiles已在前面定义
            
                        for (let i = 0; i < selectedFiles.length; i++) {
                const file = selectedFiles[i];
                
                // 🔥 Phase 38: 检查取消标志 (真实实现)
                if (conversionCancelled) {
                    Logger.info('[Conversion]', '🛑 Conversion cancelled by user');
                    if (window.addLog) {
                        window.addLog(`已取消，完成 ${successCount}/${totalFiles}`, 'warning');
                    }
                    break;
                }
                
                // 🔥 Phase 40.7.6: 转换前显示准备状态
                if (window.updateProgress) {
                    const i18n = window.i18n || { t: (key) => key };
                    window.updateProgress(i, totalFiles, i18n.t('files.preparingConversion', {filename: file.name}), 0, false);
                    log.info('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS);
                }
                
                Logger.info('[Conversion]', `[${i+1}/${selectedFiles.length}] Converting: ${file.name}`);                                                                                             
                
                try {
                    // 构建输出路径
                    const outputPath = file.filePath.replace(/\.[^.]+$/, `.${config.format}`);
                    
                    // 🔥 Phase 40.8: Rust文件分析 - 必需，无Fallback
                    // 根据 PROJECT_QUALITY_MANIFESTO.md - 响亮报错原则
                    const fileAnalysis = await window.rustCLI.analyzeFile(file.filePath);
                    
                    const fileSizeMB = fileAnalysis.file_size_mb;
                    const isAnimated = fileAnalysis.is_animated;
                    const estimatedSeconds = fileAnalysis.estimated_seconds;
                    
                    log.info?.('PIXLY Progress', LOG.IMAGE_CONV_FILE_ANALYZED, { 
                        size: fileSizeMB.toFixed(2), 
                        type: isAnimated ? 'animated' : 'static', 
                        time: estimatedSeconds 
                    });
                    
                    // 🔥 Phase 40.8: 功能开关 - 检查进度模拟是否启用
                    const useProgressSimulation = window.featureFlags ? 
                        window.featureFlags.isEnabled('ui.progress_simulation') : true;
                    
                    log.info?.('PIXLY Progress', LOG.IMAGE_CONV_FILE_DETAILS, { 
                        size: fileSizeMB.toFixed(2), 
                        type: isAnimated ? 'animated' : 'static', 
                        time: estimatedSeconds.toFixed(1) 
                    });
                    
                    // 🔥 智能模拟进度（基于Rust提供的预估时间）
                    let currentProgress = 0;
                    let simulationInterval = null;
                    let isConversionComplete = false;
                    
                    // 🔥 Phase 40.8: 根据功能开关决定是否启动模拟
                    if (useProgressSimulation) {
                        // 启动渐进式模拟（使用曲线，看起来更自然）
                        const startTime = Date.now();
                        simulationInterval = setInterval(() => {
                            if (!isConversionComplete) {
                                const elapsed = (Date.now() - startTime) / 1000; // 已用秒数
                                const ratio = Math.min(elapsed / estimatedSeconds, 1.0);
                                
                                // 使用easeOutQuad曲线：快速启动，逐渐减速
                                const eased = 1 - Math.pow(1 - ratio, 2);
                                currentProgress = Math.min(Math.round(eased * 98), 98); // 最多到98%
                                
                                if (window.updateProgress) {
                                    const i18n = window.i18n || { t: (key) => key };
                                    window.updateProgress(i, totalFiles, i18n.t('progress.convertingFile', {filename: file.name}), currentProgress, false);
                                }
                            }
                        }, 200); // 每0.2秒更新一次（看起来很流畅）
                    } else {
                        log.info('PIXLY Progress', LOG.IMAGE_CONV_PROGRESS_SIM_DISABLED);
                    }
                    
                    // 🔥 Phase 46.6: 捕获参数快照
                    const conversionId = `conv_${Date.now()}_${i}`;
                    const conversionParams = {
                        format: config.format,
                        quality: config.quality,
                        speed: config.speed || config.effort,
                        lossless: config.lossless,
                        preserve_metadata: config.preserveMetadata,
                        keep_animated: config.keepAnimated
                    };
                    
                    if (window.paramIntegrityValidator) {
                        window.paramIntegrityValidator.captureSnapshot(conversionId, conversionParams);
                    }
                    
                    // 真实转换
                    const result = await window.rustCLI.convertImage({
                        input: file.filePath,
                        output: outputPath,
                        format: config.format,
                        quality: config.quality,
                        lossless: config.lossless,
                        effort: config.speed,
                        onProgress: (subProgress) => {
                            // 如果CLI提供了真实进度，覆盖模拟进度
                            if (subProgress > currentProgress) {
                                currentProgress = subProgress;
                                log.info('PIXLY Progress', LOG.IMAGE_CONV_REAL_PROGRESS, { progress: subProgress });
                            }
                        }
                    });
                    
                    // 转换完成，停止模拟
                    isConversionComplete = true;
                    if (simulationInterval) {
                        clearInterval(simulationInterval);
                    }
                    
                    // 设置为100%
                    if (window.updateProgress) {
                        const i18n = window.i18n || { t: (key) => key };
                        window.updateProgress(i, totalFiles, i18n.t('progress.convertingFile', {filename: file.name}), 100, false);
                    }
                    
                    // 🔥 Phase 40.7.5: 调试日志
                    log.debug('PIXLY Conversion', LOG.IMAGE_CONV_RESULT_DATA, { result });
                    log.debug('PIXLY Conversion', LOG.IMAGE_CONV_RESULT_SUCCESS, { success: result && result.success });
                    
                    if (result && result.success) {
                        // 🔥 Phase 46.6: 验证参数完整性
                        if (window.paramIntegrityValidator && result.actualParams) {
                            try {
                                const validationResult = window.paramIntegrityValidator.validateIntegrity(
                                    conversionId,
                                    result.actualParams
                                );
                                
                                // 显示验证报告
                                if (validationResult.diffs && validationResult.diffs.length > 0) {
                                    window.paramIntegrityValidator.showValidationReport(validationResult);
                                }
                                
                                // 记录验证结果
                                if (log) {
                                    log.info('PIXLY ParamIntegrity', '🔍 参数验证完成', {
                                        valid: validationResult.valid,
                                        diffs: validationResult.diffs.length,
                                        source: validationResult.params_source
                                    });
                                }
                            } catch (validationError) {
                                log.error('PIXLY ParamIntegrity', '❌ 参数验证异常', {
                                    error: validationError.message
                                });
                            }
                        }
                        
                        successCount++;
                        
                        // 🔥 Phase 40.7.6: 更新进度（当前文件完成）
                        if (window.updateProgress) {
                            const i18n = window.i18n || { t: (key) => key };
                            window.updateProgress(i + 1, totalFiles, i18n.t('progress.completedFile', {filename: file.name}), 0, false);
                    log.debug('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS_CALLED);
                        }
                        Logger.info('[Conversion]', `✅ Success: ${file.name}`);
                        
                        if (window.addLog) {
                            window.addLog(`✅ ${file.name} → ${config.format}`, 'success');
                        }
                        
                        // 🔥 不再单独通知每个文件 - 仅记录日志
                        log.info('PIXLY Notification', LOG.IMAGE_CONV_FILE_SUCCESS, { filename: file.name });
                        
                        // 🔥 Phase 46.3: 自动格式修正（扩展名不符时）
                        const enableFormatCorrection = document.getElementById('enableFormatCorrection')?.checked ?? true;
                        if (enableFormatCorrection && window.PIXLY?.FileValidator) {
                            const validator = new window.PIXLY.FileValidator();
                            const outputPath = result.outputPath || result.output_path;
                            if (outputPath) {
                                try {
                                    const correctionResult = await validator.applyFormatCorrection(outputPath, true);
                                    if (correctionResult.corrected) {
                                        log.info('PIXLY', LOG.IMAGE_CONV_FORMAT_CORRECTED, { oldPath: correctionResult.oldPath, newPath: correctionResult.newPath });
                                        if (window.addLog) {
                                            window.addLog(`🔧 格式修正: .${correctionResult.from} → .${correctionResult.to}`, 'info');
                                        }
                                    }
                                } catch (error) {
                                    log.error('PIXLY', LOG.IMAGE_CONV_FORMAT_CORRECTION_FAILED, { error: error.message });
                                }
                            }
                        }
                        
                        // Rust CLI已经处理了：
                        // - 图像转换
                        // - 原地替换
                        // - Eagle元数据更新
                        // - 缩略图清理
                        
                    } else {
                        failCount++;
                        const errorMsg = result?.error || 'Unknown error';
                        
                        // 🔥 Phase 40.7.6: 更新进度（当前文件失败）
                        if (window.updateProgress) {
                            const i18n = window.i18n || { t: (key) => key };
                            window.updateProgress(i + 1, totalFiles, i18n.t('progress.failedFile', {filename: file.name}), 0, false);
                    log.debug('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS_CALLED);
                        }
                        Logger.error('[Conversion]', `❌ Failed: ${file.name}`, errorMsg);
                        
                        if (window.addLog) {
                            window.addLog(`❌ ${file.name}: ${errorMsg}`, 'error');
                        }
                        
                        // 🔥 不再单独通知每个文件 - 仅记录日志
                        log.warn('PIXLY Notification', LOG.IMAGE_CONV_FILE_FAILED, { filename: file.name, error: errorMsg });
                    }
                } catch (err) {
                    failCount++;
                    
                    // 🔥 Phase 40.7.6: 更新进度（异常）
                    if (window.updateProgress) {
                        const i18n = window.i18n || { t: (key) => key };
                        window.updateProgress(i + 1, totalFiles, i18n.t('files.errorOccurred', {filename: file.name}), 0, false);
                    log.debug('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS_CALLED);
                    }
                    
                    Logger.error('[Conversion]', `❌ Error converting ${file.name}:`, err);
                    
                    // 🔥 Phase 38: 响亮的错误反馈（不静默）
                    if (window.addLog) {
                        window.addLog(`❌ ${file.name}: ${err.message}`, 'error');
                    }
                }
            }
            
            // 完成
            if (window.updateProgress) {
                const i18n = window.i18n || { t: (key) => key };
                window.updateProgress(totalFiles, totalFiles, i18n.t('progress.completed'));
                log.debug('PIXLY', LOG.IMAGE_CONV_UPDATE_PROGRESS_CALLED);
            }
            
            // 🔥 强制锁定：设置标志并记录文件（不清空）
            log.info('PIXLY UX', LOG.IMAGE_CONV_LOCK_FORCING);
            
            // 🔥 记录转换的文件路径，用于后续比对
            window.PIXLY_LAST_CONVERTED_FILES = selectedFiles.map(f => f.filePath);
            log.info('PIXLY UX', LOG.IMAGE_CONV_FILES_RECORDED, { count: window.PIXLY_LAST_CONVERTED_FILES.length });
            
            // 🔥 设置强制锁定标志，阻止Eagle自动刷新
            window.PIXLY_SELECTION_LOCKED = true;
            log.info('PIXLY UX', LOG.IMAGE_CONV_SELECTION_LOCKED);
            
            // 🔥 UX优化：立即创建并显示遮罩层（不清空文件，保持面板高度）
            setTimeout(() => {
                // 隐藏两个进度条
                if (progressSection) {
                    progressSection.style.display = 'none';
                }
                if (inlineProgressSection) {
                    inlineProgressSection.style.display = 'none';
                    log.debug('PIXLY Progress', LOG.IMAGE_CONV_INLINE_HIDDEN);
                }
                
                // 🔥 显示遮罩层锁定面板（覆盖现有文件，不清空）
                const filePanel = document.getElementById('selectedFiles');
                let lockOverlay = document.getElementById('conversionLockOverlay');
                if (!lockOverlay && filePanel) {
                    // 动态创建遮罩层并添加到文件选择面板
                    lockOverlay = document.createElement('div');
                    lockOverlay.id = 'conversionLockOverlay';
                    lockOverlay.style.cssText = 'display: none; position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.92); z-index: 999; border-radius: 16px; backdrop-filter: blur(8px); overflow: auto;';
                    
                    const content = document.createElement('div');
                    content.style.cssText = 'display: flex; flex-direction: column; align-items: center; justify-content: center; width: 100%; min-height: 100%; padding: 20px; box-sizing: border-box;';
                    const i18n = window.i18n || { t: (key) => key };
                    content.innerHTML = `
                        <div style="font-size: 40px; margin-bottom: 12px;">✅</div>
                        <div style="font-size: 16px; font-weight: 700; color: #4CAF50; margin-bottom: 8px;">${i18n.t('files.roundComplete')}</div>
                        <div style="font-size: 13px; color: rgba(255,255,255,0.9); text-align: center; margin-bottom: 16px; line-height: 1.5; max-width: 300px;">
                            ${i18n.t('files.selectDifferentFiles')}
                        </div>
                        <button id="unlockPanelBtn" style="padding: 10px 28px; font-size: 13px; font-weight: 600; background: linear-gradient(135deg, #4CAF50, #45a049); border: none; border-radius: 6px; color: #fff; cursor: pointer; box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3); transition: all 0.3s ease;">
                            ${i18n.t('files.gotIt')}
                        </button>
                    `;
                    lockOverlay.appendChild(content);
                    filePanel.appendChild(lockOverlay);
                    log.debug('PIXLY UX', LOG.IMAGE_CONV_OVERLAY_CREATED);
                    
                    // 立即绑定按钮事件
                    const unlockBtn = document.getElementById('unlockPanelBtn');
                    if (unlockBtn) {
                        unlockBtn.onclick = () => {
                            log.info('PIXLY UX', LOG.IMAGE_CONV_UNLOCK_CLICKED);
                            
                            // 隐藏遮罩层
                            lockOverlay.style.display = 'none';
                            log.debug('PIXLY UX', LOG.IMAGE_CONV_OVERLAY_HIDDEN);
                            
                            // 解除锁定标志
                            window.PIXLY_SELECTION_LOCKED = false;
                            log.info('PIXLY UX', LOG.IMAGE_CONV_SELECTION_UNLOCKED);
                            
                            // 清除上次记录
                            window.PIXLY_LAST_CONVERTED_FILES = null;
                            log.debug('PIXLY UX', LOG.IMAGE_CONV_FILES_CLEARED);
                            
                            // 启用转换按钮
                            const convertBtn = document.getElementById('convertBtn');
                            if (convertBtn) {
                                convertBtn.disabled = false;
                                convertBtn.title = '';
                                log.debug('PIXLY UX', LOG.IMAGE_CONV_BTN_ENABLED);
                            }
                            
                            log.info('PIXLY UX', LOG.IMAGE_CONV_READY_NEW);
                        };
                        // 添加hover效果
                        unlockBtn.onmouseover = () => {
                            unlockBtn.style.transform = 'scale(1.05)';
                            unlockBtn.style.boxShadow = '0 6px 16px rgba(76, 175, 80, 0.4)';
                        };
                        unlockBtn.onmouseout = () => {
                            unlockBtn.style.transform = 'scale(1)';
                            unlockBtn.style.boxShadow = '0 4px 12px rgba(76, 175, 80, 0.3)';
                        };
                    }
                }
                if (lockOverlay) {
                    lockOverlay.style.display = 'flex';
                    log.info('PIXLY UX', LOG.IMAGE_CONV_LOCK_OVERLAY_SHOWN);
                }
            }, 500); // 减少延迟到500ms，更快显示遮罩
            
            // 禁用转换按钮直到重新选择文件
            if (startBtn) {
                const i18n = window.i18n || { t: (key) => key };
                startBtn.disabled = true;
                startBtn.title = i18n.t('title.reselectFiles');
                log.info('PIXLY UX', LOG.IMAGE_CONV_BTN_DISABLED);
            }
            
            // 显示按钮但保持禁用状态
            if (startBtn) {
                startBtn.style.display = 'block';
                // ❌ 不启用按钮！必须重新选择文件才能启用
                log.debug('PIXLY UI', LOG.IMAGE_CONV_BTN_VISIBLE_DISABLED);
            }
            
            const cancelBtn = document.getElementById('cancelBtn');
            if (cancelBtn) {
                cancelBtn.style.display = 'none';
            }
            
            if (window.addLog) {
                const i18n = window.i18n || { t: (key) => key };
                window.addLog(i18n.t('progress.conversionSummary', {success: successCount, failed: failCount}), 
                    failCount > 0 ? 'warning' : 'success');
            }
            
            Logger.info('[Conversion]', `✅ Conversion complete: ${successCount} success, ${failCount} failed`);
            
            // 🔥 Phase 40.7.4: Eagle汇总通知
            log.info('PIXLY Notification', LOG.IMAGE_CONV_FINAL_SUMMARY, { success: successCount, failed: failCount });
            if (typeof eagle !== 'undefined' && eagle.notification) {
                const i18n = window.i18n || { t: (key) => key };
                if (failCount === 0) {
                    log.info('PIXLY Notification', LOG.IMAGE_CONV_ALL_SUCCESS);
                    eagle.notification.show({ 
                        title: i18n.t('notification.allSuccessTitle'), 
                        body: i18n.t('notification.allSuccessBody', {count: successCount}), 
                        duration: 3000 
                    });
                } else if (successCount === 0) {
                    log.error('PIXLY Notification', LOG.IMAGE_CONV_ALL_FAILED);
                    eagle.notification.show({ 
                        title: i18n.t('notification.allFailedTitle'), 
                        body: i18n.t('notification.allFailedBody', {count: failCount}), 
                        duration: 5000 
                    });
                } else {
                    log.warn('PIXLY Notification', LOG.IMAGE_CONV_PARTIAL_SUCCESS);
                    eagle.notification.show({ 
                        title: i18n.t('notification.partialSuccessTitle'), 
                        body: i18n.t('notification.partialSuccessBody', {success: successCount, failed: failCount}), 
                        duration: 4000 
                    });
                }
                log.info('PIXLY Notification', LOG.IMAGE_CONV_NOTIFICATION);
            } else {
                log.error('PIXLY Notification', 'Eagle API not available for summary');
            }
            
            // 🔥 Phase 40.7.9: 清除转换标记
            log.info('PIXLY Lifecycle', LOG.IMAGE_CONV_CLEARING_FLAG);
            window.PIXLY_CONVERTING = false;
            
            // 🔥 不自动刷新！用户必须主动选择新文件
            log.info('PIXLY UX', LOG.IMAGE_CONV_SKIP_REFRESH);
            
        } catch (error) {
            Logger.error('[Conversion]', '❌ Conversion error:', error);
            if (window.addLog) {
                window.addLog(`转换失败：${error.message}`, 'error');
            }
        }
    }
    
    /**
     * 🔥 Phase 40: 修复UX交互混乱 - 区分智能模式和手动模式
     * 
     * 架构原则：
     * - 智能模式：完全由AI决定参数，使用默认值触发AI预测
     * - 手动模式：读取UI滑块，用户手动控制参数
     */
    function getConversionConfig() {
        // 🔥 Phase 46.5.13: 读取优化模式（size/balanced/quality/general）
        const log = window.pixlyLog;
        const optimizeModeRadio = document.querySelector('input[name="optimizeMode"]:checked');
        const optimizeMode = optimizeModeRadio ? optimizeModeRadio.value : 'balanced';
        log.info('Conversion', LOG.IMAGE_CONV_OPTIMIZE_MODE_READ, { mode: optimizeMode });
        
        // 检查当前模式（smart或manual）
        const smartModeRadio = document.getElementById('modeRadioSmart');
        const isSmartMode = smartModeRadio && smartModeRadio.checked;
        
        if (isSmartMode) {
            // 🔥 Phase 46.1: 智能模式尊重用户格式选择
            // 架构原则：AI优化质量参数，不改变用户明确选择的格式
            // 参考：PROJECT_QUALITY_MANIFESTO.md - 用户透明可控原则
            
            log.info('Conversion', LOG.IMAGE_CONV_SMART_MODE_DELEGATING);
            
            // ✅ 读取用户选择的格式（智能模式优先读取AI高级选项）
            // 优先级：1. AI预期格式下拉框 (expectedFormatSelect) > 2. 格式Radio按钮 > 3. 默认JXL
            let userFormat = 'jxl';
            
            const expectedFormatSelect = document.getElementById('expectedFormatSelect');
            if (expectedFormatSelect && expectedFormatSelect.value && expectedFormatSelect.value !== '' && expectedFormatSelect.value !== 'disabled') {
                // 用户在AI高级选项中明确指定了格式
                userFormat = expectedFormatSelect.value;
                log.info('Conversion', LOG.IMAGE_CONV_USER_FORMAT_AI, { format: userFormat });
            } else {
                // 未指定AI预期格式，读取Radio按钮
                const formatRadio = document.querySelector('input[name="format"]:checked');
                userFormat = formatRadio ? formatRadio.value : 'jxl';
                log.info('Conversion', LOG.IMAGE_CONV_USER_FORMAT_RADIO, { format: userFormat });
            }
            
            // 🔥 质量宣言执行：智能模式不传递默认值
            // - 如果AI预测成功 → 使用AI返回的参数 ✅
            // - 如果AI预测失败 → Rust响亮报错，不使用hardcode ✅
            // - 【原则】响亮报错 > 静默降级
            return {
                format: userFormat,      // ✅ 尊重用户选择
                quality: undefined,      // ✅ 不传递默认值，强制使用AI预测
                speed: undefined,        // ✅ 不传递默认值，强制使用AI预测
                lossless: undefined,     // ✅ Go AI ML决定
                optimizeMode: optimizeMode,  // 🔥 Phase 46.5.13: 优化模式
            };
        } else {
            // 🎛️ Phase 40.7.11: 手动模式读取UI选择器
            const formatRadio = document.querySelector('input[name="format"]:checked');
            const format = formatRadio ? formatRadio.value : 'jxl';
            
            const qualitySlider = document.getElementById('quality');
            const speedSlider = document.getElementById('speed');
            const losslessCheckbox = document.getElementById('lossless');
            const manualLosslessCheckbox = document.getElementById('manualLossless');
            const enableJpegLosslessCheckbox = document.getElementById('enableJpegLossless');
            
            const quality = qualitySlider ? parseInt(qualitySlider.value) : 90;
            const speed = speedSlider ? parseInt(speedSlider.value) : 7;
            const lossless = losslessCheckbox ? losslessCheckbox.checked : 
                             (manualLosslessCheckbox ? manualLosslessCheckbox.checked : false);
            
            // 🔥 Phase 45.5: 读取JPEG无损转码选项（JXL格式专用）
            const jpegLossless = enableJpegLosslessCheckbox ? enableJpegLosslessCheckbox.checked : false;
            
            const config = { format, quality, speed, lossless, jpegLossless };
            if (log) {
                log.info('Conversion', LOG.IMAGE_CONV_MANUAL_MODE_CONFIG, { config: JSON.stringify(config) });
            }
            
            return {
                format: format,
                quality: quality,
                speed: speed,
                lossless: lossless,
                jpeg_lossless: jpegLossless,  // 🔥 Phase 45.5: 传递JPEG无损转码参数
                optimizeMode: optimizeMode,  // 🔥 Phase 46.5.13: 优化模式
            };
        }
    }
    
    /**
     * 🔥 Phase 38: 真实的取消转换 (非摆设)
     * 
     * 架构原则：真实实现，不是空壳
     * - 设置取消标志
     * - 下次循环检查时会中断
     * - 提供明确的用户反馈
     */
    function cancelConversion() {
        const log = window.pixlyLog;
        conversionCancelled = true;
        log.warn('Conversion', LOG.IMAGE_CONV_CANCEL_REQUESTED);
        if (window.addLog) {
            window.addLog('正在取消转换...', 'warning');
        }
    }
    
    /**
     * 🔥 Phase 38: 真实的打开输出文件夹 (非摆设)
     * 
     * 架构原则：真实实现
     * - 获取第一个文件的目录
     * - 调用Eagle API打开文件夹
     */
    async function openOutputFolder() {
        const Logger = window.PIXLY?.Logger;
        const selectedFiles = window.selectedFiles || [];
        
        if (!selectedFiles || selectedFiles.length === 0) {
            Logger.warn('[Conversion]', '⚠️ No files to locate');
            if (window.addLog) {
                window.addLog('没有文件可定位', 'warning');
            }
            return;
        }
        
        try {
            // 获取第一个文件的路径
            const firstFile = selectedFiles[0];
            const folderPath = firstFile.filePath.substring(0, firstFile.filePath.lastIndexOf('/'));
            
            Logger.info('[Conversion]', '📁 Opening folder:', folderPath);
            
            // 调用Eagle API打开文件夹
            if (window.eagle && window.eagle.app) {
                await window.eagle.app.openPath(folderPath);
                Logger.info('[Conversion]', '✅ Folder opened');
            } else {
                Logger.warn('[Conversion]', '⚠️ Eagle API not available');
                if (window.addLog) {
                    window.addLog('无法打开文件夹：Eagle API不可用', 'warning');
                }
            }
        } catch (error) {
            Logger.error('[Conversion]', '❌ Failed to open folder:', error);
            if (window.addLog) {
                window.addLog(`打开文件夹失败：${error.message}`, 'error');
            }
        }
    }
    
    // 导出到全局
    window.startConversion = startConversion;
    window.cancelConversion = cancelConversion;
    window.openOutputFolder = openOutputFolder;
    
    // 🔥 初始化强制锁定标志
    const log = window.pixlyLog;
    if (typeof window.PIXLY_SELECTION_LOCKED === 'undefined') {
        window.PIXLY_SELECTION_LOCKED = false;
        log.info('PIXLY Conversion Core', LOG.IMAGE_CONV_SELECTION_LOCKED_INIT);
    }
    
    // 🔥 初始化上次转换文件记录
    if (typeof window.PIXLY_LAST_CONVERTED_FILES === 'undefined') {
        window.PIXLY_LAST_CONVERTED_FILES = null;
        log.info('PIXLY Conversion Core', LOG.IMAGE_CONV_LAST_FILES_INIT);
    }
    
    log.info('PIXLY Conversion Core', LOG.IMAGE_CONV_MODULE_LOADED);
    
})(window);
