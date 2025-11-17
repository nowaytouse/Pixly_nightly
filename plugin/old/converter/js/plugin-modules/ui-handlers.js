// VERSION: COMMIT-169 - LOG MIGRATION COMPLETE - 2025-11-10 15:07
console.log('[UI-HANDLERS] ✅ LOG MIGRATION COMPLETE - 258 console → unified log - 2025-11-10 15:07');
/**
 * PIXLY UI事件处理module - 完整实现
 * from plugin.js 完整提取
 * 依赖: allPIXLYmodule
 * 
 * 包含:
 * - initializePlugin (412-813lines, 402lines)
 * - updateJPEGNotice (1262-1278lines)
 * - updateFormatSpecificParams (1279-1300lines)
 * - handleModeChange (1301-1341lines)
 * 总计: ~650 lines完整实现
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * initializedplugin - settingsallUI事件监听器
     * 来源: plugin.js 412-813lines
     */
function initializePlugin() {
    // 🎯 统一日志实例（整个函数共用）
    const log = window.pixlyLog;
    
    // 🎬 视频处理类型切换（图像Convert vs 视频处理）
    const typeTabs = document.querySelectorAll('.type-tab');
    typeTabs.forEach(tab => {
        tab.addEventListener('click', function() {
            const type = this.getAttribute('data-type');
            
            // 更New标签激活状态
            typeTabs.forEach(t => t.classList.remove('active'));
            this.classList.add('active');
            
            // 显示/隐藏对应面板
            const imagePanel = document.getElementById('imageConversionPanel');
            const videoPanel = document.getElementById('videoPanel');  // 🔥 统一视频面板
            
            if (type === 'image') {
                if (imagePanel) imagePanel.style.display = 'block';
                if (videoPanel) videoPanel.style.display = 'none';
                log.info?.('PIXLY UI', LOG.UI_SWITCHED_TO_IMAGE)
            } else if (type === 'animated' || type === 'video') {
                // 🔥 动图and视频都使用统一视频面板
                if (imagePanel) imagePanel.style.display = 'none';
                if (videoPanel) {
                    videoPanel.style.display = 'block';
                    
                    // 自动settings输入类型
                    const animatedRadio = document.querySelector('input[name="videoInputType"][value="animated"]');
                    const videoRadio = document.querySelector('input[name="videoInputType"][value="video"]');
                    if (type === 'animated' && animatedRadio) {
                        animatedRadio.checked = true;
                        // 触发change事件以更NewUI
                        animatedRadio.dispatchEvent(new Event('change', { bubbles: true }));
                    } else if (type === 'video' && videoRadio) {
                        videoRadio.checked = true;
                        videoRadio.dispatchEvent(new Event('change', { bubbles: true }));
                    }
                }
            // log已在函数开头声明，无需重复 (原行62)
                const typeLabel = type === 'animated' ? 'Animated→Video' : 'Video';
                log.info?.('PIXLY UI', LOG.UI_SWITCHED_TO_VIDEO, { type: typeLabel })
            }
            
            // 🔥 标签切换后自动刷Newfile列表（根据NewConvert类型过滤）
            setTimeout(() => {
                if (window.selectFiles) {
                    window.selectFiles();
            // log已在函数开头声明，无需重复 (原行71)
                    log.debug?.('PIXLY UI', LOG.UI_FILE_REFRESH_TRIGGERED)
                }
            }, 100);
        });
    });
    
    // 📑 日志级别选择器
    const logLevelSelect = document.getElementById('logLevelSelect');
    if (logLevelSelect) {
        // 初始化为当前级别
        if (window.pixlyLog) {
            logLevelSelect.value = window.pixlyLog.getLevel().toString();
        }
        
        logLevelSelect.addEventListener('change', (e) => {
            const level = parseInt(e.target.value);
            if (window.pixlyLog) {
                window.pixlyLog.setLevel(level);
                const levelNames = ['ERROR', 'WARN', 'INFO', 'DEBUG', 'TRACE'];
                log.info?.('Log Level', LOG.UI_LOG_LEVEL_SWITCHED, { level: `${levelNames[level]} (${level})` })
            }
        });
            // log已在函数开头声明，无需重复 (原行95)
        log.debug?.('PIXLY UI', LOG.UI_LOG_LEVEL_BOUND)
    }
    
    // 🕸️ 语言选择器
    const languageSelect = document.getElementById('languageSelect');
    if (languageSelect) {
        languageSelect.addEventListener('change', async function() {
            const selectedLang = this.value;
            log.info?.('PIXLY UI', LOG.UI_LANGUAGE_SWITCH, { locale: selectedLang })
            if (window.i18n && window.i18n.switchLanguage) {
                await window.i18n.switchLanguage(selectedLang);
                // switchLanguage会自动更新DOM和保存到localStorage
            } else if (window.PIXLY && window.PIXLY.I18n) {
                window.PIXLY.I18n.setLanguage(selectedLang);
            } else {
            // log已在函数开头声明，无需重复 (原行112)
                log.warn?.('PIXLY UI', 'i18n system not found')
            }
        });
    } else {
            // log已在函数开头声明，无需重复 (原行117)
        log.warn?.('PIXLY UI', 'Language selector not found')
    }
    
    // 🎨 主题切换 - 由11-theme.js自动处理
    // 11-theme.jsinitThemeToggle()already添加事件监听器
    // 不needin这里重复添加
    
    // 内置日志窗口功能已删除，所有日志输出到浏览器控制台
    
    // 🎯 功能说明点击 - 显示使用说明
    const headerFeatures = document.getElementById('headerFeatures');
    if (headerFeatures) {
        headerFeatures.addEventListener('click', function() {
            log.debug?.('PIXLY UI', '🎯 Feature description clicked');
            
            // 直接打开帮助模态窗口
            const helpModal = document.getElementById('helpModal');
            if (helpModal) {
                helpModal.style.display = 'flex';
                log.info?.('PIXLY UI', '📖 Help modal opened');
            } else {
                log.warn?.('PIXLY UI', '⚠️ Help modal not found');
            }
        });
            // log已在函数开头声明，无需重复 (原行143)
        log.debug?.('PIXLY UI', LOG.UI_DESCRIPTION_BOUND)
    }
    
    // 🎯 关闭帮助窗口
    document.addEventListener('click', function(e) {
        const helpModal = document.getElementById('helpModal');
        if (helpModal && helpModal.style.display === 'flex') {
            // 点击背景关闭
            if (e.target === helpModal) {
                helpModal.style.display = 'none';
            }
        }
    });
    
    // 关闭按钮监听（延迟绑定，因为模板可能还未加载）
    setTimeout(() => {
        const closeHelpBtn = document.getElementById('closeHelpBtn');
        if (closeHelpBtn) {
            closeHelpBtn.addEventListener('click', function() {
                const helpModal = document.getElementById('helpModal');
                if (helpModal) {
                    helpModal.style.display = 'none';
                    log.debug?.('PIXLY UI', '📖 Help modal closed');
                }
            });
            // log已在函数开头声明，无需重复 (原行170)
            log.debug?.('PIXLY UI', LOG.UI_HELP_MODAL_CLOSE_BOUND)
        }
    }, 500);
    
        // 🤖 视频AI（智能模式下默认强制启用）
    const enableVideoAI = document.getElementById('enableVideoAI');
    if (enableVideoAI) {
        // 智能模式下始终保持勾选状态
        enableVideoAI.checked = true;
            // log已在函数开头声明，无需重复 (原行180)
        log.debug?.('PIXLY UI', LOG.UI_VIDEO_AI_ENABLED)
    }
    
    // 🎛️ 视频AI高级控制开关
    const advancedFeatures = document.getElementById('enableAdvancedFeatures');
    const forceTransformer = document.getElementById('forceTransformer');
    const enableVMAF = document.getElementById('enableVMAF');
    
    if (advancedFeatures) {
        advancedFeatures.addEventListener('change', function() {
            const status = this.checked ? 'ON' : 'OFF';
            log.debug?.('PIXLY UI', LOG.UI_ADVANCED_FEATURES, { status })
        });
    }
    
    if (forceTransformer) {
        forceTransformer.addEventListener('change', function() {
            const status = this.checked ? 'ON' : 'OFF';
            log.debug?.('PIXLY UI', LOG.UI_FORCE_TRANSFORMER, { status })
        });
    }
    
    if (enableVMAF) {
        enableVMAF.addEventListener('change', function() {
            const status = this.checked ? 'ON' : 'OFF';
            log.debug?.('PIXLY UI', LOG.UI_VMAF_VALIDATION, { status })
        });
    }
    
    // ⚡ 视频AI快捷预设（radio样式，和图像处理保持一致）
    const videoPresetLabels = document.querySelectorAll('[data-preset]');
    const videoPresetDesc = document.getElementById('videoAIPresetDesc');
    
    if (videoPresetLabels.length > 0) {
        // 初始化选中状态（确保默认选中的按钮有正确样式）
        const initVideoPresetStyles = () => {
            videoPresetLabels.forEach(label => {
                const radio = label.querySelector('input[type="radio"]');
                if (radio && radio.checked) {
                    label.classList.add('active');
                } else {
                    label.classList.remove('active');
                }
            });
        };
        
        videoPresetLabels.forEach(label => {
            label.addEventListener('click', function() {
                const preset = this.dataset.preset;
                
                // 移除所有active类（fallback for :has() selector）
                videoPresetLabels.forEach(l => l.classList.remove('active'));
                // 添加当前选中的active类
                this.classList.add('active');
                
                // 更新描述文本
                const i18n = window.i18n || { t: (key) => key };
                const descriptions = {
                    fast: i18n.t('videoAI.fastDesc'),
                    balanced: i18n.t('videoAI.balancedDesc'),
                    quality: i18n.t('videoAI.fullDesc')
                };
                if (videoPresetDesc) {
                    videoPresetDesc.textContent = descriptions[preset] || descriptions.balanced;
                }
                
                // 🔥 动态更新视频说明卡片
                updateVideoOptimizeModeInfoCard(preset);
                
                // 应用预设配置
                switch(preset) {
                    case 'fast':
                        if (advancedFeatures) advancedFeatures.checked = false;
                        if (forceTransformer) forceTransformer.checked = false;
                        if (enableVMAF) enableVMAF.checked = false;
                        log.info?.('PIXLY UI', LOG.UI_VIDEO_PRESET_CHANGED, { preset: 'Fast' })
                        break;
                    case 'balanced':
                        if (advancedFeatures) advancedFeatures.checked = true;
                        if (forceTransformer) forceTransformer.checked = false;
                        if (enableVMAF) enableVMAF.checked = false;
                        const log2 = window.pixlyLog || console;
                        log2.info?.('PIXLY UI', LOG.UI_VIDEO_PRESET_CHANGED, { preset: 'Balanced' })
                        break;
                    case 'quality':
                        if (advancedFeatures) advancedFeatures.checked = true;
                        if (forceTransformer) forceTransformer.checked = true;
                        if (enableVMAF) enableVMAF.checked = false;
                        const log3 = window.pixlyLog || console;
                        log3.info?.('PIXLY UI', LOG.UI_VIDEO_PRESET_CHANGED, { preset: 'Quality' })
                        break;
                }
            });
        });
        
        // 初始化样式
        initVideoPresetStyles();
        
        // 🔥 初始化时更新说明卡片
        const checkedVideoPreset = document.querySelector('input[name="videoAIPreset"]:checked');
        if (checkedVideoPreset) {
            updateVideoOptimizeModeInfoCard(checkedVideoPreset.value);
        }
        
            // log已在函数开头声明，无需重复 (原行289)
        log.debug?.('PIXLY UI', LOG.UI_VIDEO_AI_PRESETS_BOUND)
    }
    
    // log已在函数开头声明，无需重复 (原行293)
    log.debug?.('PIXLY UI', LOG.UI_VIDEO_AI_CONTROLS_BOUND)
    
    // 🎬 视频模式切换（智能/手动）
    const videoModeTabs = document.querySelectorAll('#videoPanel .mode-tab');
    const videoAIOptionsSection = document.getElementById('videoAIOptionsSection');
    // Phase 45.8.4: videoManualModeSection已删除，说明框移到videoManualOptions内部
    const videoManualOptions = document.getElementById('videoManualOptions');
    const videoModeRadioSmart = document.getElementById('videoModeRadioSmart');
    const videoModeRadioManual = document.getElementById('videoModeRadioManual');
    
    if (videoModeTabs.length > 0) {
        videoModeTabs.forEach(tab => {
            tab.addEventListener('click', function() {
                const mode = this.dataset.mode;
                
                // 更新标签样式
                videoModeTabs.forEach(t => t.classList.remove('active'));
                this.classList.add('active');
                
                // 更新radio状态
                if (mode === 'smart-video') {
                    if (videoModeRadioSmart) videoModeRadioSmart.checked = true;
                    if (videoModeRadioManual) videoModeRadioManual.checked = false;
                    
                    // 显示智能模式，隐藏手动模式
                    if (videoAIOptionsSection) videoAIOptionsSection.style.display = 'block';
                    if (videoManualOptions) videoManualOptions.style.display = 'none';
                    
                    // 显示AI核心状态（智能模式需要显示GO核心）
                    const videoGoCoreInline = document.getElementById('videoGoCoreInline');
                    if (videoGoCoreInline) videoGoCoreInline.style.display = 'block';
                    
                    log.info?.('PIXLY UI', LOG.UI_VIDEO_MODE, { mode: 'Smart (AI-driven, GO+Rust cores active)' })
                    
                    // 🔥 Phase 45.8: 切换到智能模式时重新检测GO核心状态
                    setTimeout(async () => {
                        await detectVideoCoreStatus();
                    }, 100);
                } else {
                    if (videoModeRadioManual) videoModeRadioManual.checked = true;
                    if (videoModeRadioSmart) videoModeRadioSmart.checked = false;
                    
                    // 隐藏智能模式，显示手动模式
                    if (videoAIOptionsSection) videoAIOptionsSection.style.display = 'none';
                    if (videoManualOptions) videoManualOptions.style.display = 'block';
                    
                    // 隐藏AI核心状态（手动模式不需要AI预测）
                    const videoGoCoreInline = document.getElementById('videoGoCoreInline');
                    if (videoGoCoreInline) videoGoCoreInline.style.display = 'none';
                    
                    // log已在函数开头声明，无需重复 (原行345)
                    log.info?.('PIXLY UI', LOG.UI_VIDEO_MODE, { mode: 'Manual (Rust core only, no AI prediction)' })
                }
            });
        });
            // log已在函数开头声明，无需重复 (原行350)
        log.debug?.('PIXLY UI', LOG.UI_VIDEO_MODE_TABS_BOUND)
    }
    
    // ⚡ 图像处理快捷预设（和视频处理保持一致）
    const imagePresetLabels = document.querySelectorAll('input[name="optimizeMode"]');
    const imagePresetDesc = document.getElementById('optimizeModeDesc');
    
    if (imagePresetLabels.length > 0) {
        // 初始化选中状态
        const initImagePresetStyles = () => {
            imagePresetLabels.forEach(radio => {
                const label = radio.closest('.optimize-mode-btn');
                if (label) {
                    if (radio.checked) {
                        label.classList.add('active');
                    } else {
                        label.classList.remove('active');
                    }
                }
            });
        };
        
        // 监听选择变化
        imagePresetLabels.forEach(radio => {
            radio.addEventListener('change', function() {
                // 移除所有active类
                imagePresetLabels.forEach(r => {
                    const l = r.closest('.optimize-mode-btn');
                    if (l) l.classList.remove('active');
                });
                
                // 添加当前选中的active类
                const currentLabel = this.closest('.optimize-mode-btn');
                if (currentLabel) {
                    currentLabel.classList.add('active');
                }
                
                // 更新描述
                const descriptions = {
                    size: '体积：最大压缩 + 速度优先 - 大批量处理推荐',
                    balanced: '平衡：ML + balanced (Q85) + SSIM 校验 - 日常推荐',
                    quality: '质量：深度优化 + 质量优先 - 重要图像推荐',
                    general: '通用：兼容性优先 + 稳定可靠 - 通用场景推荐'
                };
                if (imagePresetDesc) {
                    imagePresetDesc.textContent = descriptions[this.value] || descriptions.balanced;
                }
                
                log.info?.('PIXLY UI', LOG.UI_PRESET_CHANGED, { preset: this.value })
                
                // 🔥 Phase 46.5.11: 预设切换时重新检测AI状态（通用模式会显示不同状态）
                detectImageCoreStatus();
                
                // 🔥 Phase 46.5.14: 更新优化模式说明卡片
                updateOptimizeModeInfoCard(this.value);
            });
        });
        
        // 初始化样式
        initImagePresetStyles();
            // log已在函数开头声明，无需重复 (原行412)
        log.debug?.('PIXLY UI', LOG.UI_IMAGE_PRESET_BOUND)
        
        // 🔥 Phase 46.5.14: 初始化优化模式说明卡片
        const checkedPreset = document.querySelector('input[name="optimizeMode"]:checked');
        if (checkedPreset) {
            updateOptimizeModeInfoCard(checkedPreset.value);
        }
    }
    
    // 主题切换逻辑：默认跟随system，点击切换并记忆
    
    // 🔥 deleted刷New按钮 - 用户要求使用自动detected
    
    // 模式切换
    // 模式切换 - 标签按钮（仅图像转换面板）
    const modeTabs = document.querySelectorAll('#imageConversionPanel .mode-tab');
    modeTabs.forEach(tab => {
        tab.addEventListener('click', function() {
            const mode = this.getAttribute('data-mode');
            
            // 更新标签激活状态（只影响图像面板的标签）
            modeTabs.forEach(t => t.classList.remove('active'));
            this.classList.add('active');
            
            // 更新隐藏 radio 状态
            if (mode === 'smart' || mode === 'manual') {
                const radioId = mode === 'smart' ? 'modeRadioSmart' : 'modeRadioManual';
                document.getElementById(radioId).checked = true;
            }
            
            // 调用模式切换处理
            handleModeChange({ target: { value: mode } });
        });
    });
    
    // 兼容Old radio 切换
    const modeRadios = document.querySelectorAll('input[name="mode"]');
    modeRadios.forEach(radio => {
        radio.addEventListener('change', handleModeChange);
    });

    // 质量滑块
    const qualitySlider = document.getElementById('quality');
    const qualityValue = document.getElementById('qualityValue');
    const qualityHint = document.getElementById('qualityHint');
    
    if (qualitySlider) {
    qualitySlider.addEventListener('input', (e) => {
        const value = parseInt(e.target.value);
        if (qualityValue) qualityValue.textContent = value;
        
        // 更New质量提示 (支持 i18n)
        if (qualityHint) {
            if (value >= 95) {
                qualityHint.textContent = window.i18n.t('quality.high');
            } else if (value >= 80) {
                qualityHint.textContent = window.i18n.t('quality.balanced');
            } else if (value >= 60) {
                qualityHint.textContent = window.i18n.t('quality.compressed');
            } else {
                qualityHint.textContent = window.i18n.t('quality.size');
            }
        }
    });
    }
    
    // 格式专属parameter滑块更New
    // JXL parameter监听器
    const jxlEffortEl = document.getElementById('jxlEffort');
    const jxlEffortValueEl = document.getElementById('jxlEffortValue');
    if (jxlEffortEl && jxlEffortValueEl) {
        jxlEffortEl.addEventListener('input', (e) => {
            jxlEffortValueEl.textContent = e.target.value;
        });
    }
    
    const jxlDistanceEl = document.getElementById('jxlDistance');
    const jxlDistanceValueEl = document.getElementById('jxlDistanceValue');
    if (jxlDistanceEl && jxlDistanceValueEl) {
        jxlDistanceEl.addEventListener('input', (e) => {
            jxlDistanceValueEl.textContent = parseFloat(e.target.value).toFixed(1);
        });
    }
    
    const jxlPatchesEl = document.getElementById('jxlPatches');
    const jxlPatchesValueEl = document.getElementById('jxlPatchesValue');
    if (jxlPatchesEl && jxlPatchesValueEl) {
        jxlPatchesEl.addEventListener('input', (e) => {
            jxlPatchesValueEl.textContent = e.target.value;
        });
    }
    
    const jxlBitDepthEl = document.getElementById('jxlBitDepth');
    const jxlBitDepthValueEl = document.getElementById('jxlBitDepthValue');
    if (jxlBitDepthEl && jxlBitDepthValueEl) {
        jxlBitDepthEl.addEventListener('change', (e) => {
            jxlBitDepthValueEl.textContent = e.target.value;
        });
    }
    
    // WebP parameter监听器
    const webpMethodEl = document.getElementById('webpMethod');
    const webpMethodValueEl = document.getElementById('webpMethodValue');
    if (webpMethodEl && webpMethodValueEl) {
        webpMethodEl.addEventListener('input', (e) => {
            webpMethodValueEl.textContent = e.target.value;
        });
    }
    
    const webpFilterStrengthEl = document.getElementById('webpFilterStrength');
    const webpFilterStrengthValueEl = document.getElementById('webpFilterStrengthValue');
    if (webpFilterStrengthEl && webpFilterStrengthValueEl) {
        webpFilterStrengthEl.addEventListener('input', (e) => {
            webpFilterStrengthValueEl.textContent = e.target.value;
        });
    }
    
    const webpSharpnessEl = document.getElementById('webpSharpness');
    const webpSharpnessValueEl = document.getElementById('webpSharpnessValue');
    if (webpSharpnessEl && webpSharpnessValueEl) {
        webpSharpnessEl.addEventListener('input', (e) => {
            webpSharpnessValueEl.textContent = e.target.value;
        });
    }
    
    const webpSegmentsEl = document.getElementById('webpSegments');
    const webpSegmentsValueEl = document.getElementById('webpSegmentsValue');
    if (webpSegmentsEl && webpSegmentsValueEl) {
        webpSegmentsEl.addEventListener('input', (e) => {
            webpSegmentsValueEl.textContent = e.target.value;
        });
    }
    
    const webpSnsStrengthEl = document.getElementById('webpSnsStrength');
    const webpSnsStrengthValueEl = document.getElementById('webpSnsStrengthValue');
    if (webpSnsStrengthEl && webpSnsStrengthValueEl) {
        webpSnsStrengthEl.addEventListener('input', (e) => {
            webpSnsStrengthValueEl.textContent = e.target.value;
        });
    }
    
    const webpPassEl = document.getElementById('webpPass');
    const webpPassValueEl = document.getElementById('webpPassValue');
    if (webpPassEl && webpPassValueEl) {
        webpPassEl.addEventListener('input', (e) => {
            webpPassValueEl.textContent = e.target.value;
        });
    }
    
    // AVIF parameter监听器
    const avifSpeedEl = document.getElementById('avifSpeed');
    const avifSpeedValueEl = document.getElementById('avifSpeedValue');
    if (avifSpeedEl && avifSpeedValueEl) {
        avifSpeedEl.addEventListener('input', (e) => {
            avifSpeedValueEl.textContent = e.target.value;
        });
    }
    
    const avifMinQuantizerEl = document.getElementById('avifMinQuantizer');
    const avifMinQuantizerValueEl = document.getElementById('avifMinQuantizerValue');
    if (avifMinQuantizerEl && avifMinQuantizerValueEl) {
        avifMinQuantizerEl.addEventListener('input', (e) => {
            avifMinQuantizerValueEl.textContent = e.target.value;
        });
    }
    
    const avifMaxQuantizerEl = document.getElementById('avifMaxQuantizer');
    const avifMaxQuantizerValueEl = document.getElementById('avifMaxQuantizerValue');
    if (avifMaxQuantizerEl && avifMaxQuantizerValueEl) {
        avifMaxQuantizerEl.addEventListener('input', (e) => {
            avifMaxQuantizerValueEl.textContent = e.target.value;
        });
    }
    
    const avifTilesRowsEl = document.getElementById('avifTilesRows');
    const avifTilesColsEl = document.getElementById('avifTilesCols');
    const avifTilesValueEl = document.getElementById('avifTilesValue');
    if (avifTilesRowsEl && avifTilesValueEl) {
        avifTilesRowsEl.addEventListener('input', (e) => {
            const rows = e.target.value;
            const cols = avifTilesColsEl?.value || '1';
            avifTilesValueEl.textContent = `${rows}×${cols}`;
        });
    }
    if (avifTilesColsEl && avifTilesValueEl) {
        avifTilesColsEl.addEventListener('input', (e) => {
            const rows = avifTilesRowsEl?.value || '1';
            const cols = e.target.value;
            avifTilesValueEl.textContent = `${rows}×${cols}`;
        });
    }
    
    // HEIC parameter监听器
    const heicQualityEl = document.getElementById('heicQuality');
    const heicQualityValueEl = document.getElementById('heicQualityValue');
    if (heicQualityEl && heicQualityValueEl) {
        heicQualityEl.addEventListener('input', (e) => {
            heicQualityValueEl.textContent = e.target.value;
        });
    }
    
    // 🔥 根据CPU核心数自动settings工作线程最大值
    const workersSlider = document.getElementById('workers');
    if (workersSlider && workersSlider.hasAttribute('data-auto-max')) {
        try {
            const os = require('os');
            const cpuCount = os.cpus().length;
            const maxWorkers = Math.min(cpuCount, 16); // 最多16 线程
            workersSlider.max = maxWorkers;
            log.debug?.('PIXLY UI', LOG.UI_CPU_CORES_DETECTED, { count: cpuCount, max: maxWorkers })
        } catch (error) {
            // log已在函数开头声明，无需重复 (原行625)
            log.warn?.('PIXLY UI', LOG.UI_CPU_CORES_DEFAULT)
        }
    }
    
    // 工作线程数滑块更New（deleted，强制自动）
    // document.getElementById('workers').addEventListener('input', (e) => {
    //     const value = parseInt(e.target.value);
    //     const lang = window.i18n ? window.i18n.getLanguage() : 'zh';
    //     document.getElementById('workersValue').textContent = value === 0 ? (lang === 'zh' ? '自动' : 'Auto') : value.toString();
    // });
    
    // 格式选择监听器 - 动态显示格式专属parameter
    const formatOptions = document.querySelectorAll('input[name="format"]');
    formatOptions.forEach(option => {
        option.addEventListener('change', () => {
            updateFormatSpecificParams();
            updateManualParamsAvailability();
        });
    });
    // initialized时也调用一次
    updateFormatSpecificParams();
    
    // 无损options监听器 - 更Newparameteravailable性
    const manualLosslessEl = document.getElementById('manualLossless');
    if (manualLosslessEl) {
        manualLosslessEl.addEventListener('change', updateManualParamsAvailability);
    }
    
    const manualNearLosslessEl = document.getElementById('manualNearLossless');
    if (manualNearLosslessEl) {
        manualNearLosslessEl.addEventListener('change', updateManualParamsAvailability);
    }

    // skip Live 图片复选框
    document.getElementById('skipLivePhotos')?.addEventListener('change', (e) => {
        const skipEnabled = e.target.checked;
        addLog(skipEnabled ? '✅ enabled：skip Live 图片' : '❌ disabled：skip Live 图片', 'info');
        // 🔥 减少刷New频率：no need立即刷New，下次操作时自动刷New
    });

    // 内置日志窗口按钮已删除

    // 🔥 转换按钮下拉菜单逻辑
    const convertBtnContainer = document.getElementById('convertBtnContainer');
    const convertBtn = document.getElementById('convertBtn');
    const convertBtnIcon = document.getElementById('convertBtnIcon');
    const convertBtnText = document.getElementById('convertBtnText');
    const convertNormalBtn = document.getElementById('convertNormal');
    const convertJXLBtn = document.getElementById('convertJXL');
    
    let currentConvertMode = 'normal'; // 'normal' or 'jxl'
    let _lastJpegDetection = { hasJPEG: false, fileCount: 0, timestamp: 0 };
    let _jpegDetectionTimer = null;
    
    // 🔥 智能检测JPEG并切换默认模式（添加缓存和防抖）
    function updateConvertButtonMode() {
        if (!convertBtn || !convertBtnIcon || !convertBtnText) return;
        
        const files = window.PIXLY_SELECTED_FILES || [];
        const fileCount = files.length;
        
        // 缓存检测结果（1秒内相同文件数不重复检测）
        const now = Date.now();
        if (_lastJpegDetection.fileCount === fileCount && 
            (now - _lastJpegDetection.timestamp) < 1000) {
            log.trace?.('PIXLY', '📦 Using cached JPEG detection');
            return;
        }
        
        const jpegExtensions = ['.jpg', '.jpeg', '.jpe', '.jfif', '.jfi'];
        const hasJPEG = files.some(file => {
            let ext = file.ext || '';
            if (!ext.startsWith('.')) ext = '.' + ext;
            return jpegExtensions.includes(ext.toLowerCase());
        });
        
        // 更新缓存
        _lastJpegDetection = { hasJPEG, fileCount, timestamp: now };
        
        // log已在函数开头声明，无需重复 (原行706)
        log.debug?.('PIXLY', `📸 JPEG detection: ${hasJPEG}, files: ${files.length}`)
        
        if (hasJPEG && currentConvertMode !== 'jxl') {
            // 切换到JXL模式
            currentConvertMode = 'jxl';
            convertBtnIcon.textContent = '✨';
            convertBtnText.setAttribute('data-i18n', 'quickAction.jpeg2jxl');
            convertBtnText.textContent = window.i18n ? window.i18n.t('quickAction.jpeg2jxl') : 'JPEG→JXL无损';
            convertBtn.classList.add('jxl-mode');
            
            // 🔥 Phase 45.8: 下拉菜单只显示主按钮没有显示的选项
            if (convertNormalBtn) convertNormalBtn.style.display = 'flex'; // 显示"开始转换"
            if (convertJXLBtn) convertJXLBtn.style.display = 'none'; // 隐藏"JPEG→JXL"
            
            // log已在函数开头声明，无需重复 (原行721)
            log.debug?.('PIXLY', LOG.UI_JXL_MODE_SWITCHED)
        } else if (!hasJPEG && currentConvertMode !== 'normal') {
            // 切换回普通模式
            currentConvertMode = 'normal';
            convertBtnIcon.textContent = '🚀';
            convertBtnText.setAttribute('data-i18n', 'conversion.start');
            convertBtnText.textContent = window.i18n ? window.i18n.t('conversion.start') : '开始转换';
            convertBtn.classList.remove('jxl-mode');
            
            // 🔥 Phase 45.8: 下拉菜单只显示主按钮没有显示的选项
            if (convertNormalBtn) convertNormalBtn.style.display = 'none'; // 隐藏"开始转换"
            if (convertJXLBtn) convertJXLBtn.style.display = 'flex'; // 显示"JPEG→JXL"
            
            // log已在函数开头声明，无需重复 (原行735)
            log.debug?.('PIXLY', LOG.UI_NORMAL_MODE_SWITCHED)
        }
    }
    
    // 🔧 FIX: 点击展开/收起下拉菜单（避免hover快速消失）
    const convertDropdown = document.getElementById('convertDropdown');
    if (convertBtn && convertDropdown) {
        // 点击主按钮右侧的▼图标展开菜单
        convertBtn.addEventListener('click', (e) => {
            // 如果点击的是▼区域（右侧30px），则展开菜单
            const rect = convertBtn.getBoundingClientRect();
            const clickX = e.clientX - rect.left;
            const buttonWidth = rect.width;
            
            // 右侧30px触发下拉菜单
            if (clickX > buttonWidth - 30) {
                e.stopPropagation();
                convertDropdown.classList.toggle('show');
                log.debug?.('PIXLY UI', LOG.UI_DROPDOWN_TOGGLED)
            }
            // 否则正常触发转换
        });
        
        // 点击页面其他地方关闭菜单
        document.addEventListener('click', (e) => {
            if (!convertBtnContainer.contains(e.target)) {
                convertDropdown.classList.remove('show');
            }
        });
        
        log.debug?.('PIXLY UI', LOG.UI_DROPDOWN_BOUND)
    }
    
    // 🔥 下拉菜单选项 - 标准转换
    if (convertNormalBtn) {
        convertNormalBtn.addEventListener('click', async () => {
            log.info?.('PIXLY', LOG.UI_NORMAL_CONVERSION_SELECTED)
            currentConvertMode = 'normal';
            
            // 触发标准转换
            const imageConversionModule = window.pixlyImageConversion;
            if (imageConversionModule && typeof imageConversionModule.startConversion === 'function') {
                await imageConversionModule.startConversion();
            } else {
            // log已在函数开头声明，无需重复 (原行783)
                log.error?.('PIXLY', 'Image conversion module not found')
            }
        });
    }
    
    // 🔥 下拉菜单选项 - JPEG→JXL无损
    if (convertJXLBtn) {
        convertJXLBtn.addEventListener('click', async () => {
            log.info?.('PIXLY', LOG.UI_JXL_CONVERSION_SELECTED)
            
            // 过滤出所有JPEG文件
            const jpegFiles = (window.selectedFiles || []).filter(file => {
                let ext = file.ext || '';
                if (!ext.startsWith('.')) ext = '.' + ext;
                ext = ext.toLowerCase();
                return ext === '.jpg' || ext === '.jpeg' || ext === '.jpe' || ext === '.jfif' || ext === '.jfi';
            });
            
            if (jpegFiles.length === 0) {
                showNotification('⚠️ 没有JPEG文件可转换', 'warning');
                return;
            }
            
            // log已在函数开头声明，无需重复
            log.info?.('PIXLY', 'Converting {count} JPEG files to JXL (lossless_jpeg=1)', { count: jpegFiles.length })
            
            // 使用Rust CLI批量转换
            const rustCLI = window.rustCLI;
            if (!rustCLI || !rustCLI.available) {
                showNotification('❌ Rust内核未检测到，无法进行转换', 'error');
                return;
            }
            
            // 开始批量转换
            const totalFiles = jpegFiles.length;
            let successCount = 0;
            let failedCount = 0;
            
            showNotification(`🚀 开始转换 ${totalFiles} 个JPEG文件...`, 'info');
            
            for (let i = 0; i < jpegFiles.length; i++) {
                const file = jpegFiles[i];
                const outputPath = file.filePath.replace(/\.(jpg|jpeg|jpe|jfif|jfi)$/i, '_optimized.jxl');
                
                try {
                    // 使用lossless_jpeg=1参数（Rust内核会自动识别JPEG并应用此参数）
                    const result = await rustCLI.convertImage({
                        input: file.filePath,
                        output: outputPath,
                        format: 'jxl',
                        quality: 100,  // JPEG→JXL lossless需要quality=100
                        jpeg_lossless: true,  // 🔥 启用JPEG无损转码
                        lossless: true,
                        preserveMetadata: true
                    });
                    
                    if (result.success) {
                        successCount++;
                        log.info(`✅ [${i+1}/${totalFiles}] ${file.name} → JXL success`);
                    } else {
                        failedCount++;
                        log.error(`❌ [${i+1}/${totalFiles}] ${file.name} conversion failed:`, result.error);
                    }
                } catch (error) {
                    failedCount++;
                    log.error(`❌ [${i+1}/${totalFiles}] ${file.name} conversion error:`, error);
                }
            }
            
            // 显示完成通知
            if (failedCount === 0) {
                showNotification(`🎉 全部完成！成功转换 ${successCount} JPEG files to JXL`, 'success');
            } else {
                showNotification(`⚠️ 转换完成：${successCount} 成功，${failedCount} 失败`, 'warning');
            }
            
            // log已在函数开头声明（792行），无需重复
            log.info?.('PIXLY', LOG.UI_JXL_BATCH_COMPLETE, { success: successCount, total: totalFiles })
            
            // 刷新Eagle
            if (window.eagle && window.eagle.item) {
                await window.eagle.item.refreshThumbnails();
            }
        });
    }
    
    // 🔥 Phase 46.3: 下拉菜单 - AI 文件验证（独立执行）
    const runValidationOnlyBtn = document.getElementById('runValidationOnly');
    if (runValidationOnlyBtn) {
        runValidationOnlyBtn.addEventListener('click', async () => {
            log.info?.('PIXLY', 'AI file validation (standalone)')
            
            // 检查文件
            if (!window.selectedFiles || window.selectedFiles.length === 0) {
                alert('⚠️ 请先选择文件');
                return;
    }
    
            // 检查文件验证是否启用
            const fileValidationEnabled = document.getElementById('enableFileValidationQuick')?.checked ?? true;
            if (!fileValidationEnabled) {
                alert('⚠️ 请先在快捷工具区启用 AI 文件验证');
                return;
            }
            
            // 执行验证
            if (window.PIXLY?.FileValidator) {
                const validator = new window.PIXLY.FileValidator();
                const results = await validator.validateBatch(window.selectedFiles);
                
                log.info('[PIXLY] 🔒 Validation results:', results);
                
                // 显示结果统计
                const total = results.length;
                const suspicious = results.filter(r => r.suspicious).length;
                const safe = total - suspicious;
        
                if (suspicious > 0) {
                    alert(`⚠️ 验证完成\n\n总计: ${total} 个文件\n✅ 正常: ${safe} 个\n🚨 可疑: ${suspicious} 个\n\n详细结果请查看控制台日志`);
        } else {
                    alert(`✅ 验证完成\n\n全部 ${total} 个文件通过验证`);
                }
            } else {
                alert('❌ 文件验证模块未加载');
            }
        });
    }
    
    // 🔥 Phase 46.3: 下拉菜单 - 格式修正（独立执行）
    const runCorrectionOnlyBtn = document.getElementById('runCorrectionOnly');
    if (runCorrectionOnlyBtn) {
        runCorrectionOnlyBtn.addEventListener('click', async () => {
            log.info?.('PIXLY', 'Format correction (standalone)')
            
            // 检查文件
            if (!window.selectedFiles || window.selectedFiles.length === 0) {
                alert('⚠️ 请先选择文件');
                return;
            }
            
            // 检查依赖
            const fileValidationEnabled = document.getElementById('enableFileValidationQuick')?.checked ?? false;
            if (!fileValidationEnabled) {
                alert('⚠️ 格式修正需要先启用 AI 文件验证');
                return;
            }
            
            const formatCorrectionEnabled = document.getElementById('enableFormatCorrectionQuick')?.checked ?? false;
            if (!formatCorrectionEnabled) {
                alert('⚠️ 请先在快捷工具区启用自动修正格式');
                return;
            }
            
            // 先执行验证以发现需要修正的文件
            if (window.PIXLY?.FileValidator) {
                const validator = new window.PIXLY.FileValidator();
                await validator.validateBatch(window.selectedFiles);
                
                // 获取需要修正的文件
                const corrections = Array.from(validator.formatCorrections.entries());
                
                if (corrections.length === 0) {
                    alert('✅ 未发现需要修正的文件\n\n所有文件的扩展名与实际类型一致');
                    return;
                }
                
                // 确认修正
                const confirmMsg = `发现 ${corrections.length} 个文件需要修正：\n\n${corrections.map(([path, info]) => `${info.fileName}: .${info.from} → .${info.to}`).join('\n')}\n\n是否继续修正？`;
                if (!confirm(confirmMsg)) {
                    return;
                }
                
                // 执行修正
                let successCount = 0;
                let failedCount = 0;
                
                for (const [filePath] of corrections) {
                    try {
                        const result = await validator.applyFormatCorrection(filePath, true);
                        if (result.corrected) {
                            successCount++;
                            log.info(`[PIXLY] ✅ Correction success: ${result.oldPath} → ${result.newPath}`);
                        }
                    } catch (error) {
                        failedCount++;
                        log.error(`[PIXLY] ❌ Correction failed: ${filePath}`, error);
                    }
                }
                
                // 显示结果
                if (failedCount === 0) {
                    alert(`🎉 格式修正完成\n\n成功修正 ${successCount} 个文件`);
                } else {
                    alert(`⚠️ 格式修正完成\n\n✅ 成功: ${successCount} 个\n❌ 失败: ${failedCount} 个`);
                }
                
                // 刷新 Eagle
                if (window.eagle && window.eagle.item) {
                    await window.eagle.item.refreshThumbnails();
                }
            } else {
                alert('❌ 文件验证模块未加载');
            }
        });
            }
            
    // Phase 46.2: 移除旧的重复监听器
    // 主转换按钮已在第1153行绑定了统一监听器（根据tab类型调用不同函数）
    // 此处的旧监听器会导致重复触发，已移除
    
    // 监听文件选择事件（防抖）
    let _jpegCheckDebounceTimer = null;
    if (typeof window.addEventListener !== 'undefined') {
        window.addEventListener('pixly:filesSelected', () => {
            clearTimeout(_jpegCheckDebounceTimer);
            _jpegCheckDebounceTimer = setTimeout(updateConvertButtonMode, 100);
        });
    }
            
    // 初始更新
    setTimeout(updateConvertButtonMode, 500);
    
    // 旧按钮逻辑已移除，功能已整合到下拉菜单中
    
    // 修复: 初始化 selectedFiles 为空数组
    if (!window.selectedFiles) {
        window.selectedFiles = [];
            }
            
    // 保留原有的智能特性标志逻辑（向下兼容）
            window.PIXLY_QUICK_MODE = {
        enabled: false,
                format: 'jxl',
                smartJPEG: true,      // JPEG 自动无损
                smartHEIC: true,      // HEIC 自动预处理
                smartAnimated: true   // 动画自动detected
            };
            
            log.info?.('PIXLY', LOG.UI_SMART_MODE_ENABLED)
            
            // validate格式is否correctsettings
            setTimeout(() => {
                const formatRadio = document.querySelector('input[name="format"]:checked');
                // log已在函数开头声明，无需重复 (原行1031)
                const format = formatRadio ? formatRadio.value : 'Not selected';
                log.debug?.('PIXLY', 'Current selected format: {format}', { format })
            }, 200);

    // 🔥 All→AVIF按钮已删除（功能已整合到标准转换流程中）
    
    // ====================================================================
    // 🎥 快捷视频Convert按钮
    // ====================================================================
    
    // GIF→MP4 快捷按钮
    const quickGIF2MP4Btn = document.getElementById('quickGIF2MP4');
    if (quickGIF2MP4Btn) {
        quickGIF2MP4Btn.addEventListener('click', async () => {
 log.info('[PIXLY] 🎥 Quick conversion: Animation→MP4');
            
            // 获取Selectedfile
            const files = window.selectedFiles || [];
            const animatedFiles = files.filter(f => {
                const ext = path.extname(f.filePath || f.path).toLowerCase();
                return ['.gif', '.webp', '.apng'].includes(ext);
            });
            
            if (animatedFiles.length === 0) {
                await PIXLY.EagleDialog.warning('No Animated Files', 'Please select animated files (GIF/WebP/APNG) in Eagle first');
                return;
            }
            
            addLog(`🎥 startingConvert ${animatedFiles.length}  动图to MP4`, 'info');
            
            try {
                quickGIF2MP4Btn.disabled = true;
                quickGIF2MP4Btn.textContent = i18n.t('progress.converting');
                
                // 🔒 强制维持原帧率，不指定fpsparameter
                const results = await window.convertAnimatedToMP4(animatedFiles, {
                    quality: 23,
                    preset: 'medium'
                });
                
                // 显示Result
                if (results.success > 0) {
                    const msg = `✅ successfullyConvert ${results.success} files`;
                    addLog(msg, 'success');
                    if (window.showEagleDialog) {
                        window.showEagleDialog('success', 'Convertsuccessfully', msg);
                    }
                }
                
                if (results.failed > 0) {
                    const msg = `❌ ${results.failed} filesfailed`;
                    addLog(msg, 'error');
                }
                
            } catch (error) {
 log.error('[PIXLY] ❌ conversionfailed:', error);
                addLog(`❌ Convertfailed: ${error.message}`, 'error');
            } finally {
                quickGIF2MP4Btn.disabled = false;
                quickGIF2MP4Btn.innerHTML = '<span>🎥</span><span>GIF→MP4</span>';
            }
        });
    }
    
    // Optimize视频快捷按钮
    const quickOptimizeVideoBtn = document.getElementById('quickOptimizeVideo');
    if (quickOptimizeVideoBtn) {
        quickOptimizeVideoBtn.addEventListener('click', async () => {
 log.info('[PIXLY] ⚡ Quick conversion: Video optimization');
            
            // 获取Selectedfile
            const files = window.selectedFiles || [];
            const videoFiles = files.filter(f => {
                const ext = path.extname(f.filePath || f.path).toLowerCase();
                return ['.mp4', '.mov', '.avi', '.mkv', '.webm'].includes(ext);
            });
            
            if (videoFiles.length === 0) {
                await PIXLY.EagleDialog.warning('No Video Files', 'Please select video files in Eagle first');
                return;
            }
            
            // 弹窗选择编码器
            const codec = await PIXLY.EagleDialog.confirm('Select Codec', 'AV1 (highest compression) or H.265 (faster)?', ['H.265', 'AV1']) ? 'av1' : 'hevc';
            
            addLog(`⚡ startingOptimize ${videoFiles.length}  视频 (${codec.toUpperCase()})`, 'info');
            
            try {
                quickOptimizeVideoBtn.disabled = true;
                quickOptimizeVideoBtn.textContent = i18n.t('progress.optimizing');
                
                // 读取VMAF验证选项
                const enableVMAFCheckbox = document.getElementById('enableVMAF');
                const enableVMAF = enableVMAFCheckbox?.checked || false;
                
                const results = await window.optimizeVideo(videoFiles, {
                    codec: codec,
                    quality: codec === 'av1' ? 30 : 23,
                    preset: 'medium',
                    enableVMAF: enableVMAF,
                    minVMAFScore: 85
                });
                
                // 显示Result
                if (results.success > 0) {
                    const msg = `✅ successfullyOptimize ${results.success} files`;
                    addLog(msg, 'success');
                    if (window.showEagleDialog) {
                        window.showEagleDialog('success', 'Optimizesuccessfully', msg);
                    }
                }
                
                if (results.failed > 0) {
                    const msg = `❌ ${results.failed} filesfailed`;
                    addLog(msg, 'error');
                }
                
            } catch (error) {
 log.error('[PIXLY] ❌ Optimizefailed:', error);
                addLog(`❌ Optimizefailed: ${error.message}`, 'error');
            } finally {
                quickOptimizeVideoBtn.disabled = false;
                quickOptimizeVideoBtn.innerHTML = i18n.t('tools.optimizeVideoBtn');
            }
        });
    }
    
    // ====================================================================
    // 📝 快捷规范化file名按钮
    // ====================================================================
    
    const quickNormalizeNameBtn = document.getElementById('quickNormalizeName');
    if (quickNormalizeNameBtn) {
        quickNormalizeNameBtn.addEventListener('click', async () => {
 log.info('[PIXLY] 📝 Quick action: Normalize filenames');
            
            // 获取Selectedfile
            const files = window.selectedFiles || [];
            
            if (files.length === 0) {
                await PIXLY.EagleDialog.warning('No Files', 'Please select files in Eagle first');
                return;
            }
            
            // 弹窗选择规范化预设
            const presetChoice = await PIXLY.EagleDialog.confirm(
                'Normalization Mode',
                'Standard (remove brackets + replace spaces) or Strict (remove brackets + spaces + hyphens + lowercase)?',
                ['Strict', 'Standard']
            );
            
            const options = {
                removeParentheses: true,
                removeSpaces: true,
                removeDashes: !presetChoice,
                lowercase: !presetChoice
            };
            
            addLog(`📝 starting规范化 ${files.length} files名`, 'info');
            
            try {
                quickNormalizeNameBtn.disabled = true;
                quickNormalizeNameBtn.textContent = i18n.t('progress.normalizing');
                
                // 执lines规范化（使用增强built-in函数）
                const results = await window.normalizeFileNames(files, options);
                
                // 显示Result
                if (results.successCount > 0) {
                    const msg = `✅ successfully规范化 ${results.successCount} files名`;
                    addLog(msg, 'success');
                    
                    // 显示重命名详情
                    if (results.renames && results.renames.length > 0) {
                        results.renames.slice(0, 5).forEach(r => {
                            addLog(`   ${r.old} → ${r.new}`, 'info');
                        });
                        if (results.renames.length > 5) {
                            addLog(`   ... 还有 ${results.renames.length - 5} files`, 'info');
                        }
                    }
                    
                    if (window.showEagleDialog) {
                        window.showEagleDialog('success', '规范化successfully', msg);
                    }
                    
                    // 刷Newfile列表
                    setTimeout(() => {
                        if (window.selectFiles) {
                            window.selectFiles();
                        }
                    }, 500);
                }
                
                if (results.skippedCount > 0) {
                    addLog(`⏭️  skip ${results.skippedCount} files（规范）`, 'info');
                }
                
                if (results.failedCount > 0) {
                    const msg = `❌ ${results.failedCount} filesfailed`;
                    addLog(msg, 'error');
                }
                
            } catch (error) {
 log.error('[PIXLY] ❌ Normalization failed:', error);
                addLog(`❌ 规范化failed: ${error.message}`, 'error');
            } finally {
                quickNormalizeNameBtn.disabled = false;
                quickNormalizeNameBtn.innerHTML = i18n.t('tools.normalizeNameBtn');
            }
        });
    }

    // 🔥 现代化交互：点击 Logo 打开说明
    const headerLogo = document.querySelector('.header-logo');
    
    // Logo 点击 → 打开说明
    if (headerLogo) {
        const i18n = window.i18n || { t: (key) => key };
        headerLogo.style.cursor = 'pointer';
        headerLogo.title = i18n.t('title.clickForHelp');
        headerLogo.addEventListener('click', () => {
            log.debug?.('PIXLY UI', '📖 Logo clicked, opening help');
            const helpModal = document.getElementById('helpModal');
            if (helpModal) {
                helpModal.style.display = 'flex';
            }
        });
    }
    
    // 标题和日志相关的点击事件已删除

    // 🔥 关闭说明按钮
    const closeHelpBtn = document.getElementById('closeHelpBtn');
    if (closeHelpBtn) {
        closeHelpBtn.addEventListener('click', () => {
            const helpModal = document.getElementById('helpModal');
            if (helpModal) {
                helpModal.style.display = 'none';
            }
        });
    }

    // 🔥 点击遮罩关闭说明
    const helpModal = document.getElementById('helpModal');
    if (helpModal) {
        helpModal.addEventListener('click', (e) => {
            if (e.target === helpModal) {
                helpModal.style.display = 'none';
            }
        });
    }

    // 🔥 Phase 40.34: 统一转换按钮 - 根据当前tab调用不同的转换函数
    const convertBtnElement = document.getElementById('convertBtn');
    if (convertBtnElement) {
        convertBtnElement.addEventListener('click', function() {
            // 🔥 Phase 40.35: 使用data-type属性准确判断tab
            const activeTab = document.querySelector('.type-tab.active');
            const tabType = activeTab ? activeTab.getAttribute('data-type') : 'image';
            
            log.info('[PIXLY UI] 🔍 Active tab type:', tabType);
        
        if (tabType === 'video') {
            log.info('[PIXLY UI] 🎬 Video tab active, calling startVideoConversion()');
            if (window.startVideoConversion) {
                window.startVideoConversion();
            } else {
                log.error('[PIXLY UI] ❌ startVideoConversion() not available');
                alert('视频转换功能尚未加载，请刷新插件');
            }
        } else {
            log.info('[PIXLY UI] 🖼️ Image tab active, calling startConversion()');
            if (window.startConversion) {
                window.startConversion();
            } else {
                log.error('[PIXLY UI] ❌ startConversion() not available');
                alert('图片转换功能尚未加载，请刷新插件');
            }
        }
        });
    }

    // cancel按钮 - 同样需要根据tab调用不同的取消函数
    const cancelBtn = document.getElementById('cancelBtn');
    if (cancelBtn) {
        cancelBtn.addEventListener('click', function() {
        // 🔥 Phase 40.35: 使用data-type属性准确判断tab
        const activeTab = document.querySelector('.type-tab.active');
        const tabType = activeTab ? activeTab.getAttribute('data-type') : 'image';
        
        if (tabType === 'video') {
            log.info('[PIXLY UI] 🛑 Cancelling video conversion');
            if (window.cancelVideoConversion) {
                window.cancelVideoConversion();
            }
        } else {
            log.info('[PIXLY UI] 🛑 Cancelling image conversion');
            if (window.cancelConversion) {
                window.cancelConversion();
            }
        }
        });
    }

    // 打开输出file夹按钮
    const openOutputBtn = document.getElementById('openOutputBtn');
    if (openOutputBtn) {
        openOutputBtn.addEventListener('click', openOutputFolder);
    }

    // Optimize模式切换（带视觉反馈）
    const optimizeModeRadios = document.querySelectorAll('input[name="optimizeMode"]');
    const optimizeModeDesc = document.getElementById('optimizeModeDesc');
    
    // 辅助函数：更NewOptimize模式按钮视觉状态
    function updateOptimizeModeVisuals() {
        optimizeModeRadios.forEach(r => {
            const label = r.parentElement;
            const span = label.querySelector('span');
            
            if (r.checked) {
                // Selected状态：紫色边框、半透明紫色背景、阴影、加粗文字
                label.style.border = '2px solid #667eea';
                label.style.background = 'rgba(102, 126, 234, 0.15)';
                label.style.boxShadow = '0 2px 4px rgba(102, 126, 234, 0.2)';
                if (span) span.style.fontWeight = '600';
            } else {
                // 未Selected状态：边框颜色、背景、无阴影、普通文字
                label.style.border = '2px solid var(--border)';
                label.style.background = 'var(--bg-card)';
                label.style.boxShadow = 'none';
                if (span) span.style.fontWeight = '500';
            }
        });
    }
    
    // initialized视觉状态
    updateOptimizeModeVisuals();
    
    // 🔥 辅助函数：切换AI选项启用/禁用状态
    function toggleAIOptions(enable) {
        // 🔥 定义Logger（必须在函数开始处）
        const Logger = window.PIXLY?.Logger || console;
        
        const aiCheckboxes = [
            document.getElementById('enableSmartQuality'),
            document.getElementById('enableAutoOptimize'),
            document.getElementById('enableSSIMValidation'),
            document.getElementById('enableVideoForAnimation'),
            document.getElementById('enableBayesian'),
            document.getElementById('enablePPO')
        ];
        
        // 处理checkbox和其label
        aiCheckboxes.forEach(checkbox => {
            if (checkbox) {
                checkbox.disabled = !enable;
                const label = checkbox.closest('label');
                if (label) {
                    if (enable) {
                        label.style.opacity = '1';
                        label.style.cursor = 'pointer';
                        label.style.filter = 'none';
                    } else {
                        label.style.opacity = '0.4';
                        label.style.cursor = 'not-allowed';
                        label.style.filter = 'grayscale(50%)';
                    }
                }
            }
        });
        
        // 🆕 处理自定义预期格式容器
        const expectedFormatContainer = document.getElementById('expectedFormatContainer');
        const expectedFormatSelect = document.getElementById('expectedFormatSelect');
        if (expectedFormatContainer && expectedFormatSelect) {
            if (enable) {
                expectedFormatContainer.style.opacity = '1';
                expectedFormatSelect.disabled = false;
                expectedFormatSelect.style.cursor = 'pointer';
            } else {
                expectedFormatContainer.style.opacity = '0.4';
                expectedFormatSelect.disabled = true;
                expectedFormatSelect.style.cursor = 'not-allowed';
            }
        }
        
        // 🆕 处理"高级AI模型"标题
        const advancedModelsTitle = document.getElementById('advancedModelsTitle');
        if (advancedModelsTitle) {
            if (enable) {
                advancedModelsTitle.style.opacity = '1';
                advancedModelsTitle.style.filter = 'none';
            } else {
                advancedModelsTitle.style.opacity = '0.4';
                advancedModelsTitle.style.filter = 'grayscale(50%)';
            }
        }
        
        Logger.info(`[PIXLY UI] 🎛️ AI options ${enable ? 'enabled' : 'disabled'}`);
        
        // 🔥 Phase 45.8.3: goCoreStatus元素已移除，状态由detectImageCoreStatus统一管理
        // 旧的goCoreStatus相关代码已废弃
        
        // 记录日志
        const lang = window.i18n ? window.i18n.currentLocale : 'en';
        const status = enable ? 'enabled' : 'disabled';
        log.info(`[PIXLY UI] 🎛️ AI options ${status}`);
    }
    
    // 监听变化事件
    optimizeModeRadios.forEach(radio => {
        radio.addEventListener('change', (e) => {
            const mode = e.target.value;
            const desc = window.i18n.t(`smartMode.${mode}Desc`);
            optimizeModeDesc.textContent = desc;
            optimizeModeDesc.setAttribute('data-i18n', `smartMode.${mode}Desc`);
            log.info(`[PIXLY UI] 📊 Image preset: ${mode}`);
            
            // ✨ 视觉反馈：更Newall按钮样式
            updateOptimizeModeVisuals();
            
            // 🔥 通用模式：禁用AI选项（避免理解负担）
            if (mode === 'general') {
                toggleAIOptions(false);
            } else {
                toggleAIOptions(true);
            }
        });
    });
    
    // 🔥 初始化时根据当前模式设置AI选项状态
    const initialOptimizeMode = document.querySelector('input[name="optimizeMode"]:checked');
    if (initialOptimizeMode && initialOptimizeMode.value === 'general') {
        toggleAIOptions(false);
    }
    
    // 🎯 格式选择变化时，detected并显示 JPEG 无损提示
    const formatRadios = document.querySelectorAll('input[name="format"]');
    const jpegNotice = document.getElementById('jpegLosslessNotice');
    
    formatRadios.forEach(radio => {
        radio.addEventListener('change', () => {
            log.debug?.('PIXLY UI', LOG.UI_FORMAT_CHANGED)
            // 🔥 Phase 45.8: 立即执行，不延迟
            updateJPEGNotice();
        });
    });
    
    // 🆕 自定义预期格式变化时，联动控制"动图转视频"选项
    const expectedFormatSelect = document.getElementById('expectedFormatSelect');
    const enableVideoForAnimation = document.getElementById('enableVideoForAnimation');
    
    if (expectedFormatSelect && enableVideoForAnimation) {
        expectedFormatSelect.addEventListener('change', function() {
            const value = this.value;
            // 当选择了具体的图像格式（非空且非disabled）或禁用时，禁用"动图转视频"
            // 只有 "自动推断" (value='') 时才允许动图转视频
            const shouldDisableVideo = value !== '';
            const label = enableVideoForAnimation.closest('label');
            
            // 根据选择的格式决定是否禁用"动图转视频"
            enableVideoForAnimation.disabled = shouldDisableVideo;
            
            if (label) {
                if (shouldDisableVideo) {
                    label.style.opacity = '0.4';
                    label.style.cursor = 'not-allowed';
                    label.style.filter = 'grayscale(50%)';
                    // 同时取消勾选
                    enableVideoForAnimation.checked = false;
                } else {
                    label.style.opacity = '1';
                    label.style.cursor = 'pointer';
                    label.style.filter = 'none';
                }
            }
            
            log.info(`[PIXLY UI] 🎬 Video for animation ${shouldDisableVideo ? 'disabled' : 'enabled'} (format: ${value || 'auto'})`);
        });
    }
    
    // 🔥 工作线程数滑块更New（in上面initialized，这里只添加事件监听器）
    // workersSliderin上面声明，不need重复声明
    if (document.getElementById('workers')) {
        document.getElementById('workers').addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            const workersValueEl = document.getElementById('workersValue');
            if (workersValueEl) {
                const lang = window.i18n ? window.i18n.currentLocale : 'zh_CN';
                workersValueEl.textContent = value === 0 ? (lang.startsWith('zh') ? '自动' : 'Auto') : value.toString();
            }
        });
    }

    // 🗑️ cache自动清理由 19-cache-manager.js module管理，no need手动调用
    
    // ========== v2.0 New增：视频相关控件 ==========
    if (typeof initVideoControls === 'function') {
        initVideoControls();
    }
    // initVideoConversionControls(); // 🔥 deleted，功能mergeto initVideoControls()

    // ========== v3.0 New增：统一视频面板输入类型切换 ==========
    const videoInputTypeRadios = document.querySelectorAll('input[name="videoInputType"]');
    videoInputTypeRadios.forEach(radio => {
        radio.addEventListener('change', function() {
            const inputType = this.value;
            const videoInfoSection = document.getElementById('videoInfoSection');
            const audioOptions = document.getElementById('audioOptions');
            const proResOption = document.getElementById('proResOption');
            const crfSection = document.getElementById('crfSection');
            
 log.info(`[PIXLY UI] 🎬 Video input type switch: ${inputType}`);
            
            if (inputType === 'animated') {
                // 动图→视频：隐藏视频info，隐藏音频options，隐藏ProRes
                if (videoInfoSection) videoInfoSection.style.display = 'none';
                if (audioOptions) audioOptions.style.display = 'none';
                if (proResOption) proResOption.style.display = 'none';
                if (crfSection) crfSection.style.display = 'block';
 log.info('[PIXLY UI] ✅ Animation mode: Simplified UI');
            } else if (inputType === 'video') {
                // 视频转码：显示视频info，显示音频options，显示ProRes
                if (videoInfoSection) videoInfoSection.style.display = 'block';
                if (audioOptions) audioOptions.style.display = 'block';
                if (proResOption) proResOption.style.display = 'block';
                if (crfSection) crfSection.style.display = 'block';
 log.info('[PIXLY UI] ✅ Video mode: Full UI');
                
                // 触发视频infodetected
                if (typeof detectVideoInfo === 'function') {
                    const selectedFiles = window.selectedFiles || [];
                    if (selectedFiles.length > 0) {
                        detectVideoInfo(selectedFiles[0]);
                    }
                }
            }
        });
    });
    
    // initialized输入类型（默认动图）
    const initialInputType = document.querySelector('input[name="videoInputType"]:checked');
    if (initialInputType) {
        initialInputType.dispatchEvent(new Event('change', { bubbles: true }));
    }

    // ========== ProRes编码器选择切换 ==========
    const videoCodecRadios = document.querySelectorAll('input[name="videoCodec"]');
    const proResConfig = document.getElementById('proResConfig');
    const crfSection = document.getElementById('crfSection');
    
    videoCodecRadios.forEach(radio => {
        radio.addEventListener('change', function() {
            const codec = this.value;
            
            // ProRes不使用CRF，使用Profile
            if (codec === 'prores') {
                if (crfSection) crfSection.style.display = 'none';
                if (proResConfig) proResConfig.style.display = 'block';
                log.debug?.('PIXLY UI', LOG.UI_PRORES_MODE)
            } else {
                if (crfSection) crfSection.style.display = 'block';
                if (proResConfig) proResConfig.style.display = 'none';
                // log已在函数开头声明，无需重复 (原行1600)
                log.debug?.('PIXLY UI', LOG.UI_ENCODER_SWITCHED, { encoder: codec.toUpperCase() })
            }
            
            // 更新兼容性提示
            updateVideoCompatibility();
        });
    });
    
    // ========== 容器格式选择切换 ==========
    const videoContainerRadios = document.querySelectorAll('input[name="videoContainer"]');
    videoContainerRadios.forEach(radio => {
        radio.addEventListener('change', function() {
            log.debug?.('PIXLY UI', LOG.UI_CONTAINER_SWITCHED, { container: this.value.toUpperCase() })
            // 更新兼容性提示
            updateVideoCompatibility();
        });
    });
    
    // ========== 视频兼容性提示更新函数 ==========
    function updateVideoCompatibility() {
        const selectedCodec = document.querySelector('input[name="videoCodec"]:checked');
        const selectedContainer = document.querySelector('input[name="videoContainer"]:checked');
        const compatCheck = document.getElementById('videoCompatCheck');
        
        if (!selectedCodec || !selectedContainer || !compatCheck) return;
        
        const codec = selectedCodec.value;
        const container = selectedContainer.value;
        
        // 使用 i18n 获取兼容性文本
        const key = `${codec}_${container}`;
        const i18n = window.i18n || { t: (key) => key };
        const compatText = i18n.t(`video.compat.${key}`) || `✅ ${codec.toUpperCase()} + ${container.toUpperCase()}`;
        
        compatCheck.textContent = compatText;
            // log已在函数开头声明，无需重复 (原行1637)
        log.debug?.('PIXLY UI', LOG.UI_COMPATIBILITY_UPDATED, { compat: compatText })
    }
    
    // 导出函数供其他模块使用
    window.updateVideoCompatibility = updateVideoCompatibility;
    
    // 初始化时更新一次
    setTimeout(() => {
        updateVideoCompatibility();
    }, 100);

    // ========== 统一视频Convert按钮 ==========
    const startVideoConversionBtn = document.getElementById('startVideoConversion');
    if (startVideoConversionBtn) {
        startVideoConversionBtn.addEventListener('click', async () => {
            const inputType = document.querySelector('input[name="videoInputType"]:checked')?.value;
            const selectedFiles = window.selectedFiles || [];
            
            if (selectedFiles.length === 0) {
                addLog(window.i18n.t('messages.log.selectFilesInEagle'), 'warning');
                return;
            }
            
 log.info(`[PIXLY] 🚀 Started unified video conversion: ${inputType} mode, ${selectedFiles.length} files`);
            
            try {
                startVideoConversionBtn.disabled = true;
                startVideoConversionBtn.textContent = i18n.t('progress.convertingWithIcon');
                
                // 🔥 fromUIreadparameter
                const codec = document.querySelector('input[name="videoCodec"]:checked')?.value || 'h264';
                const crf = parseInt(document.getElementById('videoCrfSlider')?.value || '23');
                const preset = document.getElementById('videoSpeed')?.value || 'medium';
                const container = document.getElementById('videoContainer')?.value || 'mp4';
                // 🔒 强制维持原帧率，不readfps输入框
                const proResProfile = document.getElementById('proResProfile')?.value || 'standard';
                const gpuAccel = document.getElementById('enableGPUAccel')?.checked || false;
                
                const options = {
                    codec,
                    quality: crf,
                    preset,
                    container,
                    // 🔒 不传fpsparameter，强制维持原帧率
                    proResProfile,
                    gpuAccel
                };
                
 log.info('[PIXLY] 🎯 conversionparameter:', options);
                
                if (inputType === 'animated') {
                    // 动图→视频
                    if (typeof convertAnimatedToVideo === 'function') {
                        await convertAnimatedToVideo(selectedFiles, options);
                    } else {
                        addLog(window.i18n.t('messages.log.animatedToVideoNotLoaded'), 'error');
                    }
                } else if (inputType === 'video') {
                    // 视频转码
                    if (typeof optimizeVideo === 'function') {
                        await optimizeVideo(selectedFiles, options);
                    } else {
                        addLog(window.i18n.t('messages.log.videoOptimizeNotLoaded'), 'error');
                    }
                }
            } catch (error) {
 log.error('[PIXLY] ❌ videoconversionfailed:', error);
                addLog(`❌ Convertfailed: ${error.message}`, 'error');
            } finally {
                startVideoConversionBtn.disabled = false;
                startVideoConversionBtn.textContent = '🚀 startingConvert';
            }
        });
    }

    // 🔥 GO核心detected按钮事件
    // 🔥 Phase 45.8.3: autoDetectGoCore已废弃，改用detectImageCoreStatus
    // 保留函数定义以防旧代码引用，但内部逻辑已禁用
    async function autoDetectGoCore() {
        // 旧的goCoreStatus元素已删除，此函数已废弃
        log.info('[PIXLY UI] ⚠️ autoDetectGoCore is deprecated, use detectImageCoreStatus instead');
        return;
        
        /* 以下代码已废弃
        const statusSpan = document.getElementById('goCoreStatus');
        
        try {
            // 自动检测GO核心
            const result = await detectGoCore();
            
            if (result.available) {
                if (statusSpan) {
                    statusSpan.innerHTML = `<span style="color: #4CAF50;">✅ ${result.version} (${result.port})</span>`;
                }
                log.info(`[PIXLY UI] ✅ GO core auto-detected: ${result.version}`);
            } else {
                // 🆕 未检测到GO核心 → 尝试自动启动 (fallback机制)
                if (statusSpan) {
                    statusSpan.innerHTML = `<span style="color: #FFA500;">🔄 自动启动中...</span>`;
                }
                log.info('[PIXLY UI] ⚠️ GO core not running, attempting auto-start...');
                
                // 尝试自动启动GO核心
                const startResult = await autoStartGoCore();
                
                if (startResult.success) {
                    // 启动成功，重新检测
                    log.info('[PIXLY UI] ✅ GO core auto-started successfully, re-detecting...');
                    await new Promise(resolve => setTimeout(resolve, 2000)); // 等待2秒
                    
                    const reCheckResult = await detectGoCore();
                    if (reCheckResult.available) {
                        if (statusSpan) {
                            statusSpan.innerHTML = `<span style="color: #4CAF50;">✅ ${reCheckResult.version} (${reCheckResult.port})</span>`;
                        }
                        log.info(`[PIXLY UI] ✅ GO core now running: ${reCheckResult.version}`);
                        
                        // 显示成功通知
                        if (window.showEagleNotification) {
                            await window.showEagleNotification(
                                'GO核心已自动启动',
                                `版本 ${reCheckResult.version} 运行在端口 ${reCheckResult.port}`,
                                'success',
                                3000
                            );
                        }
                    } else {
                        // 启动后仍然检测不到
                        if (statusSpan) {
                            statusSpan.innerHTML = `<span style="color: #f44336;">❌ 启动失败</span>`;
                        }
                        log.warn('[PIXLY UI] ⚠️ GO core auto-start completed but service not detected');
                    }
                } else {
                    // 启动失败 - 显示友好提示
                    if (statusSpan) {
                        statusSpan.innerHTML = `<span style="color: #f44336;">❌ 未安装</span>`;
                    }
                    log.warn('[PIXLY UI] ⚠️ GO core auto-start failed:', startResult.error);
                    
                    // 显示安装建议
                    if (startResult.suggestion) {
                        log.info('[PIXLY UI] 💡 Suggestion:', startResult.suggestion);
                        
                        // 可选：显示用户通知
                        if (window.showEagleNotification) {
                            await window.showEagleNotification(
                                'GO核心未安装',
                                startResult.suggestion,
                                'warning',
                                5000
                            );
                        }
                    }
                }
            }
        } catch (error) {
            if (statusSpan) {
                statusSpan.innerHTML = `<span style="color: #f44336;">❌ 检测失败</span>`;
            }
            log.warn('[PIXLY UI] ⚠️ GO core detection error:', error.message);
        }
        */ // 结束废弃代码注释
    }
    
    // 🔥 初始化时自动检测（仅智能模式）
    // Phase 45.8.3: 此处调用已废弃，改用detectImageCoreStatus
    // const initMode = document.querySelector('input[name="optimizeMode"]:checked');
    // if (initMode && initMode.value !== 'general') {
    //     autoDetectGoCore();
    // }

    // 🔥 Phase 45.5: 恢复enableJpegLossless checkbox功能
    const enableJpegLosslessEl = document.getElementById('enableJpegLossless');
    if (enableJpegLosslessEl) {
        enableJpegLosslessEl.addEventListener('change', function() {
            const status = this.checked;
            log.debug?.('PIXLY UI', LOG.UI_JPEG_LOSSLESS_CHANGED, { status })
            updateManualParamsAvailability();
            
            // 更新视觉反馈
            const label = this.closest('label');
            if (label) {
                if (this.checked) {
                    label.style.background = 'rgba(76, 175, 80, 0.2)';
                    label.style.borderColor = 'rgba(76, 175, 80, 0.5)';
                } else {
                    label.style.background = 'rgba(76, 175, 80, 0.1)';
                    label.style.borderColor = 'rgba(76, 175, 80, 0.3)';
                }
            }
        });
            // log已在函数开头声明，无需重复 (原行1831)
        log.debug?.('PIXLY UI', LOG.UI_CHECKBOX_BOUND)
    }
    
    // 🔥 初始化时触发默认模式（确保UI正确显示）
    setTimeout(() => {
        // 图像转换面板 - 触发智能模式
        const imageSmartRadio = document.getElementById('modeRadioSmart');
        if (imageSmartRadio && imageSmartRadio.checked) {
            handleModeChange({ target: { value: 'smart' } });
            // log已在函数开头声明，无需重复 (原行1841)
            log.debug?.('PIXLY UI', LOG.UI_IMAGE_SMART_ACTIVATED)
        }
        
        // 视频处理面板 - 触发智能模式
        const videoSmartRadio = document.getElementById('videoModeRadioSmart');
        if (videoSmartRadio && videoSmartRadio.checked) {
            // 触发视频面板的智能模式显示
            const videoAIOptionsSection = document.getElementById('videoAIOptionsSection');
            const videoManualOptions = document.getElementById('videoManualOptions');
            if (videoAIOptionsSection) videoAIOptionsSection.style.display = 'block';
            if (videoManualOptions) videoManualOptions.style.display = 'none';
            // log已在函数开头声明，无需重复 (原行1853)
            log.debug?.('PIXLY UI', LOG.UI_VIDEO_SMART_ACTIVATED)
            
            // 🔥 Phase 45.8: 立即检测GO核心状态并应用禁用样式（如果AI离线）
            setTimeout(async () => {
                await detectVideoCoreStatus();
            }, 100);
        }
    }, 50);
    
    // 🔥 Phase 46.3: 快捷工具区 - AI 验证和格式修正功能
    const fileValidationQuick = document.getElementById('enableFileValidationQuick');
    const formatCorrectionQuick = document.getElementById('enableFormatCorrectionQuick');
    const fileValidationAdvanced = document.getElementById('enableFileValidation');
    const formatCorrectionAdvanced = document.getElementById('enableFormatCorrection');
    
    // 同步快捷工具和高级选项的状态
    if (fileValidationQuick && fileValidationAdvanced) {
        fileValidationQuick.addEventListener('change', (e) => {
            fileValidationAdvanced.checked = e.target.checked;
            // 如果关闭验证，也关闭格式修正
            if (!e.target.checked && formatCorrectionQuick) {
                formatCorrectionQuick.checked = false;
                if (formatCorrectionAdvanced) formatCorrectionAdvanced.checked = false;
            }
            const status = e.target.checked ? 'enabled' : 'disabled';
            log.debug?.('PIXLY Quick Tools', LOG.UI_AI_VALIDATION_CHANGED, { status })
        });
        
        fileValidationAdvanced.addEventListener('change', (e) => {
            fileValidationQuick.checked = e.target.checked;
            if (!e.target.checked && formatCorrectionQuick) {
                formatCorrectionQuick.checked = false;
                if (formatCorrectionAdvanced) formatCorrectionAdvanced.checked = false;
            }
        });
    }
    
    if (formatCorrectionQuick && formatCorrectionAdvanced) {
        formatCorrectionQuick.addEventListener('change', (e) => {
            // 如果启用格式修正，必须先启用验证
            if (e.target.checked && fileValidationQuick && !fileValidationQuick.checked) {
                alert('⚠️ 自动修正格式需要先启用 AI 文件验证');
                formatCorrectionQuick.checked = false;
                return;
            }
            formatCorrectionAdvanced.checked = e.target.checked;
            const status = e.target.checked ? 'enabled' : 'disabled';
            log.debug?.('PIXLY Quick Tools', LOG.UI_FORMAT_CORRECTION_CHANGED, { status })
        });
        
        formatCorrectionAdvanced.addEventListener('change', (e) => {
            if (e.target.checked && fileValidationAdvanced && !fileValidationAdvanced.checked) {
                alert('⚠️ 自动修正格式需要先启用 AI 文件验证');
                formatCorrectionAdvanced.checked = false;
                return;
            }
            formatCorrectionQuick.checked = e.target.checked;
        });
    }
    
    log.debug?.('PIXLY UI', LOG.UI_QUICK_TOOLS_BOUND)
    
    // 🎯 CRF滑块事件绑定
    const videoCrfSlider = document.getElementById('videoCrfSlider');
    const videoCrfValue = document.getElementById('videoCrfValue');
    if (videoCrfSlider && videoCrfValue) {
        videoCrfSlider.addEventListener('input', (e) => {
            videoCrfValue.textContent = e.target.value;
        });
        // 初始化显示值
        videoCrfValue.textContent = videoCrfSlider.value;
        // log已在函数开头声明，无需重复 (原行1928)
        log.debug?.('PIXLY UI', LOG.UI_CRF_SLIDER_BOUND)
    }
    
    // 🌐 监听语言切换事件，重新渲染动态内容
    window.addEventListener('languageChanged', () => {
        // log已在函数开头声明，无需重复 (原行1934)
        log.debug?.('PIXLY UI', LOG.UI_LANGUAGE_CHANGED)
        
        const i18n = window.i18n || { t: (key) => key };
        
        // 重新渲染图像AI预设信息 - 调用完整的更新函数
        const imagePreset = document.querySelector('input[name="aiPreset"]:checked')?.value || 'balanced';
        if (typeof updateOptimizeModeInfoCard === 'function') {
            updateOptimizeModeInfoCard(imagePreset);
        }
        
        // 重新渲染视频AI预设信息
        const videoPreset = document.querySelector('input[name="videoAIPreset"]:checked')?.value || 'balanced';
        if (typeof updateVideoOptimizeModeInfoCard === 'function') {
            updateVideoOptimizeModeInfoCard(videoPreset);
        }
        
        // 🔥 直接更新AI状态文本（不重新检测，因为会用缓存）
        // 图像AI状态
        const imageGoCoreStatus = document.getElementById('imageGoCoreStatus');
        if (imageGoCoreStatus && _goCoreCache && _goCoreCache.result.available) {
            imageGoCoreStatus.innerHTML = `<span style="color: #4CAF50;">🤖 ${i18n.t('mode.smartModeReady')} ✅</span>`;
        }
        
        const imageManualCoreStatus = document.getElementById('imageManualCoreStatus');
        if (imageManualCoreStatus && _goCoreCache && _goCoreCache.result.available) {
            imageManualCoreStatus.innerHTML = `<span style="color: #4CAF50;">🔧 ${i18n.t('mode.manualModeWithGo')} ✅</span>`;
        }
        
        // 视频AI状态
        const videoGoCoreInline = document.getElementById('videoGoCoreInline');
        if (videoGoCoreInline && _goCoreCache && _goCoreCache.result.available) {
            videoGoCoreInline.innerHTML = `<span style="color: #4CAF50;">🤖 ${i18n.t('ai.online')} ✅</span>`;
        }
        
        // 🔥 更新文件计数器（动态文本，无data-i18n属性）
        if (window.selectedFiles && window.updateFileCounter) {
            window.updateFileCounter(window.selectedFiles.length);
        }
        
        // 🔥 更新GPU信息显示（动态文本，无data-i18n属性）
        if (window.PIXLY && window.PIXLY.GPUDetector) {
            const gpuCache = window.PIXLY.GPUDetector.getGPUCache();
            if (gpuCache) {
                window.PIXLY.GPUDetector.updateGPUUI(gpuCache);
            }
        }
        
        // 🔥 更新视频占位符文本（动态文本，无data-i18n属性）
        if (window.updateVideoInfoPlaceholder) {
            window.updateVideoInfoPlaceholder(i18n.t('video.selectVideoOrAnimation'));
        }
        
        // 🔥 更新视频兼容性提示（动态文本，无data-i18n属性）
        if (window.updateVideoCompatibility) {
            window.updateVideoCompatibility();
        }
        
        // 🔥 更新Rust内核状态显示（动态文本，无data-i18n属性）
        const imageManualStatus = document.getElementById('imageManualCoreStatus');
        const videoManualStatus = document.getElementById('videoManualRustCoreStatus');
        if (window.rustCLI && window.rustCLI.available) {
            const version = window.rustCLI.version || 'unknown';
            if (imageManualStatus) {
                imageManualStatus.innerHTML = `<span style="color: #10B981;">✅ Rust ${i18n.t('common.online')}</span>`;
                imageManualStatus.title = version;
            }
            if (videoManualStatus) {
                videoManualStatus.innerHTML = `<span style="color: #10B981;">✅ Rust ${i18n.t('common.online')}</span>`;
                videoManualStatus.title = version;
            }
        } else {
            if (imageManualStatus) {
                imageManualStatus.innerHTML = `<span style="color: #EF4444;">❌ Rust ${i18n.t('ai.offline')}</span>`;
                imageManualStatus.title = 'Rust CLI not detected';
            }
            if (videoManualStatus) {
                videoManualStatus.innerHTML = `<span style="color: #EF4444;">❌ Rust ${i18n.t('ai.offline')}</span>`;
                videoManualStatus.title = 'Rust CLI not detected';
            }
        }
        
            // log已在函数开头声明，无需重复 (原行2016)
        log.debug?.('PIXLY UI', LOG.UI_DYNAMIC_REFRESHED)
    });
    
            // log已在函数开头声明，无需重复 (原行2020)
    log.info?.('PIXLY UI', LOG.UI_INIT_COMPLETE)
    
    // initialized GPU detected（异步）
    initGPUDetection();
    
    // initialized PIXLY detected（立即执lines）
    initPixlyDetection();
    
    // 🔥 初始化AI客户端并检查GO核心状态
    if (window.PIXLY?.AIClient && typeof window.PIXLY.AIClient.init === 'function') {
        setTimeout(async () => {
            try {
                await window.PIXLY.AIClient.init();
            // log已在函数开头声明，无需重复 (原行2034)
                log.info?.('PIXLY UI', 'AI client initialization complete')
                
                // 如果当前是智能模式，触发一次状态更新
                const smartModeRadio = document.querySelector('input[name="mode"][value="smart"]');
                if (smartModeRadio && smartModeRadio.checked) {
                    // 手动触发一次GO核心状态检查
                    const event = new Event('change');
                    smartModeRadio.dispatchEvent(event);
                }
            } catch (e) {
 log.warn('[PIXLY UI] ⚠️ AI client initialization failed:', e);
            }
        }, 500);
    }
    
    // 🔥 Phase 45.8: 图像和视频面板核心状态检测（增加延迟和重试）
    setTimeout(async () => {
            // log已在函数开头声明，无需重复 (原行2052)
        log.debug?.('PIXLY UI', 'Starting initial core status detection...')
        await detectImageCoreStatus(); // 检测图像面板AI状态
        await detectVideoCoreStatus(); // 检测视频面板AI状态
        
        // 🔥 如果初次检测失败，1秒后重试一次
        setTimeout(async () => {
            const smartStatusEl = document.getElementById('imageGoCoreStatus');
            if (smartStatusEl && smartStatusEl.textContent.includes('AI离线')) {
                log.info('[PIXLY UI] 🔄 AI offline detected, retrying detection...');
                await detectImageCoreStatus();
        await detectVideoCoreStatus();
            }
        }, 1000);
    }, 500); // 从300ms增加到500ms
    
    // 🎯 范围收束功能初始化
    initScopeFilterHandlers();
    
    // ✨ 智能模式高级选项展开/折叠
    initAdvancedAIToggle();
    
    // 🎯 初始化AI功能依赖关系控制
    initAIDependencies();
    
    // 🔧 图像智能模式GO核心检测（缩短间隔：300ms）
    setTimeout(async () => {
        await detectImageGoCoreStatus();
    }, 300);
    
    // 🤖 自动detectedandinstalled依赖（异步，不阻塞 UI）
    setTimeout(() => {
        autoInstallToolsSilently().catch(err => {
            const Logger = window.Logger || console;
            Logger.warn('[PIXLY UI] 自动安装依赖失败:', err.message || err);
        });
    }, 1000);
    
    // initializedparameteravailable性Check
    setTimeout(() => updateManualParamsAvailability(), 200);
    
    // 自动刷Newfile列表（延迟执lines，确保 Eagle API 就绪）
    setTimeout(() => {
        selectFiles().catch(err => {
            const Logger = window.Logger || console;
            Logger.warn('[PIXLY UI] 自动刷新文件列表失败:', err.message || err);
        });
    }, 500);
    
    // 🔥 窗口获得焦点时刷Newfile列表（节流防抖，降低频率）
    let focusRefreshTimer = null;
    window.addEventListener('focus', () => {
        // 🔥 检查转换状态和锁定标志
        if (!window.isConverting && !window.PIXLY_SELECTION_LOCKED) {
            // 清除之前计时器，防止频繁刷新
            if (focusRefreshTimer) clearTimeout(focusRefreshTimer);
            
            focusRefreshTimer = setTimeout(() => {
                if (window.selectFiles) {
                    window.selectFiles();
                    log.info('[PIXLY UI] ✅ Focus event triggered file refresh (debounced)');
                }
                focusRefreshTimer = null;
            }, 500); // 增加延迟，减少频率
        } else if (window.PIXLY_SELECTION_LOCKED) {
            log.info('[PIXLY UI] 🔒 Focus event blocked by selection lock');
        }
    });
    
    // 🔥 定期自动刷Newfile列表（用户强烈要求resume，确保100%可靠）
    // 🔄 【层4】定期detecteddeleted - 避免过于频繁刷New影响用户体验
    // setInterval(() => {
    //     if (!window.isConverting) {
    //         if (window.selectFiles) {
    //             window.selectFiles();
    //         }
    //     }
    // }, 5000); // Too frequent, deleted
    
    // Periodically auto-check and clean cache (managed by 19-cache-manager.js automatically)
    // setInterval(() => {
    //     autoCleanCache().catch(err => {
    //         log.warn('Auto cache cleanup failed:', err);
    //     });
    // }, 3600000); // Every hour
    
    // Language switcher (handled by i18n.fixed.js automatically)
        // Removed all hardcoded translations - strictly depends on i18n system
    
    // 🎨 主题切换器由 11-theme.js 自动initialized
    // 不needin这里手动调用

    // 刷New面板空内容提示with收起状态提示 (移除)
    // updatePanelHints();

    // 监听面板开合，实时更New提示 (移除)
    // initPanelHintListeners();
    
    // 🔥 修复：初始化时检测AI服务状态（强制执行）
    setTimeout(() => {
        log.info('[PIXLY UI] 🔍 Forcing AI service detection...');
        if (window.PIXLY_AI_INTEGRATION && window.PIXLY_AI_INTEGRATION.testService) {
            window.PIXLY_AI_INTEGRATION.testService().catch(err => {
                log.warn('[PIXLY UI] ❌ AI service test failed:', err);
            });
        } else {
            log.warn('[PIXLY UI] ⚠️ PIXLY_AI_INTEGRATION not found, AI detection skipped');
        }
    }, 1000); // 延迟1秒，确保所有模块加载完成
}
// ==================== 工具detectedandinstalled ====================
// 工具detectedcache（会话级别）
const toolsCache = {
    checked: false,
    pixlyInstalled: false,
    pixlyPath: null,
    results: null,
    lastCheck: null
};

// initialized时detected PIXLY
function initPixlyDetection() {
    const log = window.pixlyLog;
    const path = require('path');
    
    log.debug?.('PIXLY UI', LOG.UI_DETECTING_CORE)
    
    const pixlyPath = window.PIXLY.FileHandler?.getPixlyBinary?.();
    if (pixlyPath) {
        toolsCache.pixlyPath = pixlyPath;
        toolsCache.pixlyInstalled = true;
        toolsCache.checked = true;
        toolsCache.lastCheck = Date.now();
        
 log.info(`[PIXLY UI] ✅ PIXLY ready: ${path.basename(pixlyPath)}`);
 log.info(`[PIXLY UI] Path: ${pixlyPath}`);
        
        // validateversion
        try {
            const { execSync } = require('child_process');
            const version = execSync(`"${pixlyPath}" --version 2>&1`, { 
                encoding: 'utf8',
                timeout: 3000
            }).trim();
 log.info(`[PIXLY UI] version: ${version}`);
        } catch (e) {
 log.warn('[PIXLY UI] ⚠️ Cannot getversioninfo');
        }
    } else {
        toolsCache.pixlyInstalled = false;
        toolsCache.checked = true;
        toolsCache.lastCheck = Date.now();
 log.warn('[PIXLY UI] ⚠️ PIXLY core not detected');
    }
}

/**
 * 更NewJPEG无损提示
 * 来源: plugin.js 1262-1278lines
 */
function updateJPEGNotice() {
    const jpegNotice = document.getElementById('jpegLosslessNotice');
    const manualLosslessLabel = document.getElementById('manualLosslessLabel');
    if (!jpegNotice) return;
    
    const selectedFormat = document.querySelector('input[name="format"]:checked');
    const isJXL = selectedFormat && selectedFormat.value === 'jxl';
    
    // 🔥 Phase 45.7: 支持带/不带点号的扩展名
    const jpegExtensions = ['jpg', 'jpeg', 'jpe', 'jfif', 'jfi', '.jpg', '.jpeg', '.jpe', '.jfif', '.jfi'];
    const hasJPEG = (window.selectedFiles || []).some(f => {
        if (!f || !f.ext) return false;
        const ext = f.ext.toLowerCase();
        return jpegExtensions.includes(ext);
    });
    
    log.info('[PIXLY UI] 🔍 JPEG Notice Update - Format:', selectedFormat?.value, 'isJXL:', isJXL, 'hasJPEG:', hasJPEG, 'files:', window.selectedFiles?.length);
    
    // 🔥 Phase 45.8: JXL格式下修改标签文本为"无损转码"
    if (manualLosslessLabel) {
        const labelSpan = manualLosslessLabel.querySelector('span[data-i18n="manual.mathematicalLossless"]');
        if (isJXL) {
            // JXL格式：改为"无损转码"
            if (labelSpan) labelSpan.textContent = i18n.t('manual.losslessTranscode');
            log.info('[PIXLY UI] 🔄 Changed label to losslessTranscode for JXL format');
        } else {
            // 其他格式：保持"数学无损"
            if (labelSpan) labelSpan.textContent = i18n.t('manual.mathematicalLossless');
        }
    }
    
    if (isJXL && hasJPEG) {
        // JXL格式 + 有JPEG文件：显示JPEG专用无损选项，隐藏通用强制无损
        jpegNotice.style.display = 'flex'; // 🔥 Phase 45.8: 使用flex保持在同一行
        if (manualLosslessLabel) manualLosslessLabel.style.display = 'none';
        log.info('[PIXLY UI] ✅ Showing JPEG lossless option');
        // 触发parameteravailable性更New
        setTimeout(() => updateManualParamsAvailability(), 100);
    } else {
        // 其他情况：隐藏JPEG专用选项，显示通用强制无损
        jpegNotice.style.display = 'none';
        if (manualLosslessLabel) manualLosslessLabel.style.display = 'flex';
        log.info('[PIXLY UI] ⚠️ Hiding JPEG lossless option');
        // resumeparameteravailable性
        setTimeout(() => updateManualParamsAvailability(), 100);
    }
}

/**
 * 更新手动模式参数可用性
 * 当启用无损模式时禁用质量参数
 * 🔥 Phase 45.5: 恢复enableJpegLossless检查
 */
function updateManualParamsAvailability() {
    const manualLosslessEl = document.getElementById('manualLossless');
    const enableJpegLosslessEl = document.getElementById('enableJpegLossless');
    const qualitySlider = document.getElementById('quality');
    
    if (!qualitySlider) return;
    
    // 检查是否启用数学无损模式或JPEG无损转码
    const isManualLossless = manualLosslessEl && manualLosslessEl.checked;
    const isJpegLossless = enableJpegLosslessEl && enableJpegLosslessEl.checked;
    const isAnyLossless = isManualLossless || isJpegLossless;
    
    // 禁用/启用all质量相关parameter
    const paramsToToggle = [
        qualitySlider,
        document.getElementById('jxlEffort'),
        document.getElementById('jxlDecodeSpeed'),
        document.getElementById('jxlPhotonNoise'),
        document.getElementById('webpMethod'),
        document.getElementById('webpAutoFilter'),
        document.getElementById('webpSharpness'),
        document.getElementById('avifSpeed'),
        document.getElementById('avifTiles'),
        document.getElementById('heicQuality')
    ];
    
    paramsToToggle.forEach(el => {
        if (el) {
            el.disabled = isAnyLossless;
            el.style.opacity = isAnyLossless ? '0.5' : '1';
            el.style.cursor = isAnyLossless ? 'not-allowed' : 'pointer';
        }
    });
    
    // 🔥 当JPEG无损转码启用时，也需要禁用manualLossless checkbox（避免冲突）
    if (manualLosslessEl) {
        if (isJpegLossless) {
            manualLosslessEl.disabled = true;
            const label = manualLosslessEl.closest('label');
            if (label) {
                label.style.opacity = '0.5';
                label.style.cursor = 'not-allowed';
            }
        } else {
            manualLosslessEl.disabled = false;
            const label = manualLosslessEl.closest('label');
            if (label) {
                label.style.opacity = '1';
                label.style.cursor = 'pointer';
            }
        }
    }
    
    log.info('[PIXLY UI] 📊 Params availability updated:', {
        manualLossless: isManualLossless,
        jpegLossless: isJpegLossless,
        anyLossless: isAnyLossless
    });
}

/**
 * 更New格式专属parameter显示
 * 来源: plugin.js 1279-1300lines
 */
function updateFormatSpecificParams() {
    const selectedFormat = document.querySelector('input[name="format"]:checked');
    if (!selectedFormat) return;
    
    const format = selectedFormat.value;
    
    // 隐藏all格式专属parameter
    document.getElementById('jxlParams').style.display = 'none';
    document.getElementById('webpParams').style.display = 'none';
    document.getElementById('avifParams').style.display = 'none';
    document.getElementById('heicParams').style.display = 'none';
    
    // 显示对应格式专属parameter
    if (format === 'jxl') {
        document.getElementById('jxlParams').style.display = 'block';
    } else if (format === 'webp') {
        document.getElementById('webpParams').style.display = 'block';
    } else if (format === 'avif') {
        document.getElementById('avifParams').style.display = 'block';
    } else if (format === 'heic') {
        document.getElementById('heicParams').style.display = 'block';
    }
    // PNG 没有专属parameter，只使用通用parameter
}

/**
 * ✨ 智能模式高级AI选项展开/折叠（图像+视频）
 */
function initAdvancedAIToggle() {
    // 图像处理的展开按钮
    const toggleButton = document.getElementById('toggleAdvancedAI');
    const advancedAIOptions = document.getElementById('advancedAIOptions');
    
    if (toggleButton && advancedAIOptions) {
        let isExpanded = false;
        
        toggleButton.addEventListener('click', function() {
            isExpanded = !isExpanded;
            
            const iconSpan = toggleButton.querySelector('span:first-child');
            const textSpan = toggleButton.querySelector('span[data-i18n]');
            
            if (isExpanded) {
                advancedAIOptions.style.display = 'grid';
                if (iconSpan) iconSpan.textContent = '▲';
                if (textSpan) {
                    textSpan.setAttribute('data-i18n', 'ai.hideAdvancedOptions');
                    textSpan.textContent = i18n.t('ai.hideAdvancedOptions');
                }
                log.debug?.('PIXLY AI', 'Image advanced AI options expanded')
            } else {
                advancedAIOptions.style.display = 'none';
                if (iconSpan) iconSpan.textContent = '▼';
                if (textSpan) {
                    textSpan.setAttribute('data-i18n', 'ai.showAdvancedOptions');
                    textSpan.textContent = i18n.t('ai.showAdvancedOptions');
                }
            // log已在函数开头声明，无需重复 (原行2387)
                log.debug?.('PIXLY AI', 'Image advanced AI options collapsed')
            }
        });
        
            // log已在函数开头声明，无需重复 (原行2392)
        log.debug?.('PIXLY AI', 'Image advanced AI toggle initialized')
    }
    
    // 视频处理的展开按钮
    const toggleVideoButton = document.getElementById('toggleAdvancedVideoAI');
    const advancedVideoAIOptions = document.getElementById('advancedVideoAIOptions');
    
    if (toggleVideoButton && advancedVideoAIOptions) {
        let isVideoExpanded = false;
        
        toggleVideoButton.addEventListener('click', function() {
            isVideoExpanded = !isVideoExpanded;
            
            const iconSpan = toggleVideoButton.querySelector('span:first-child');
            const textSpan = toggleVideoButton.querySelector('span[data-i18n]');
            
            if (isVideoExpanded) {
                advancedVideoAIOptions.style.display = 'grid';
                if (iconSpan) iconSpan.textContent = '▲';
                if (textSpan) {
                    textSpan.setAttribute('data-i18n', 'videoAI.hideAdvancedOptions');
                    textSpan.textContent = i18n.t('ai.hideAdvancedOptions');
                }
                log.info('[PIXLY Video AI] Advanced AI options expanded');
            } else {
                advancedVideoAIOptions.style.display = 'none';
                if (iconSpan) iconSpan.textContent = '▼';
                if (textSpan) {
                    textSpan.setAttribute('data-i18n', 'videoAI.showAdvancedOptions');
                    textSpan.textContent = i18n.t('ai.showAdvancedOptions');
                }
                log.info('[PIXLY Video AI] Advanced AI options collapsed');
            }
        });
        
        log.info('[PIXLY Video AI] Advanced AI toggle initialized');
    }
    
    if (!toggleButton && !toggleVideoButton) {
        log.warn('[PIXLY AI] No advanced AI toggle buttons found');
    }
}

/**
 * 🎯 初始化AI功能依赖关系控制
 */
function initAIDependencies() {
    const log = window.pixlyLog;
    // 1. 贝叶斯优化依赖自动参数优化
    const autoOptimizeCheckbox = document.getElementById('enableAutoOptimize');
    const bayesianCheckbox = document.getElementById('enableBayesian');
    const bayesianLabel = document.getElementById('bayesianLabel');
    
    if (autoOptimizeCheckbox && bayesianCheckbox && bayesianLabel) {
        const updateBayesianState = () => {
            if (!autoOptimizeCheckbox.checked) {
                bayesianCheckbox.checked = false;
                bayesianCheckbox.disabled = true;
                bayesianLabel.style.opacity = '0.5';
                bayesianLabel.style.cursor = 'not-allowed';
                log.info('[PIXLY AI] Bayesian optimization disabled (auto-optimize off)');
            } else {
                bayesianCheckbox.disabled = false;
                bayesianLabel.style.opacity = '1';
                bayesianLabel.style.cursor = 'pointer';
            }
        };
        
        autoOptimizeCheckbox.addEventListener('change', updateBayesianState);
        updateBayesianState(); // 初始化状态
        
        log.debug?.('PIXLY AI', 'Bayesian dependency initialized')
    }
    
    // 2. 图像动图转视频依赖格式推荐
    const expectedFormatSelect = document.getElementById('expectedFormatSelect');
    const videoForAnimationCheckbox = document.getElementById('enableVideoForAnimation');
    
    if (expectedFormatSelect && videoForAnimationCheckbox) {
        const updateVideoForAnimationState = () => {
            if (expectedFormatSelect.value === 'disabled') {
                videoForAnimationCheckbox.checked = false;
                videoForAnimationCheckbox.disabled = true;
                videoForAnimationCheckbox.parentElement.style.opacity = '0.5';
                videoForAnimationCheckbox.parentElement.style.cursor = 'not-allowed';
                log.info('[PIXLY AI] Video for animation disabled (format recommendation off)');
            } else {
                videoForAnimationCheckbox.disabled = false;
                videoForAnimationCheckbox.parentElement.style.opacity = '1';
                videoForAnimationCheckbox.parentElement.style.cursor = 'pointer';
            }
        };
        
        expectedFormatSelect.addEventListener('change', updateVideoForAnimationState);
        updateVideoForAnimationState(); // 初始化状态
        
        log.debug?.('PIXLY AI', LOG.UI_DEPENDENCY_INIT)
    }
    
    // 3. 视频动图转视频依赖（暂时没有对应的格式推荐下拉框，预留接口）
    const videoForAnimationVideoCheckbox = document.getElementById('enableVideoForAnimationVideo');
    const videoForAnimationLabel = document.getElementById('videoForAnimationLabel');
    
    if (videoForAnimationVideoCheckbox && videoForAnimationLabel) {
        // 视频面板暂时没有格式推荐禁用选项，但预留了ID便于未来扩展
            // log已在函数开头声明，无需重复 (原行2499)
        log.debug?.('PIXLY Video AI', LOG.UI_VIDEO_ANIM_READY)
    }
}

/**
 * 🔧 图像智能模式GO核心检测（统一简洁样式）
 */
async function detectImageGoCoreStatus() {
    const smartStatusEl = document.getElementById('imageGoCoreStatus');
    const manualStatusEl = document.getElementById('imageManualCoreStatus');
    const smartPanel = document.getElementById('aiOptionsSection');
    const manualPanel = document.getElementById('manualOptions');
    
    try {
        const goResult = await detectGoCore();
        
        // 🤖 Update smart mode status
        const i18n = window.i18n || { t: (key) => key };
        if (smartStatusEl) {
            if (goResult.available) {
                smartStatusEl.innerHTML = `<span style="color: #4CAF50;">🤖 ${i18n.t('mode.smartModeReady')} ✅</span>`;
                log.info(`[PIXLY Image Smart] GO core ready: ${goResult.version}`);
            } else {
                // 🔥 Phase 45.8: Provide fix option when AI offline
                smartStatusEl.innerHTML = `
                    <span style="color: #F44336;">❌ ${i18n.t('ai.offline')}</span>
                    <button onclick="window.PIXLY.UIHandlers.fixAICore()" 
                            style="margin-left: 8px; padding: 2px 8px; font-size: 11px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;"                                                                                                                                                        
                            onmouseover="this.style.background='#45a049'" 
                            onmouseout="this.style.background='#4CAF50'">
                        🔧 ${i18n.t('common.fix')}
                    </button>
                `;
                log.info('[PIXLY Image Smart] GO core not connected');
            }
        }
        
        // 🔧 更新手动模式状态（手动模式可使用本地工具，不强制要求GO核心）
        if (manualStatusEl) {
            if (goResult.available) {
                manualStatusEl.innerHTML = `<span style="color: #4CAF50;">🔧 ${i18n.t('mode.manualModeWithGo')} ✅</span>`;
                log.info(`[PIXLY Image Manual] Tools + GO ready`);
            } else {
                manualStatusEl.innerHTML = `<span style="color: #FFA500;">🔧 Local</span>`;
                log.info('[PIXLY Image Manual] Local tools only');
            }
        }
        
        // 🎯 智能模式：需要GO核心
        // 🔥 Phase 46.1: AI离线时不禁用pointerEvents，以便修复按钮可以点击
        if (smartPanel) {
            if (goResult.available) {
                smartPanel.style.opacity = '1';
                smartPanel.style.pointerEvents = 'auto';
                smartPanel.style.filter = 'none';
            } else {
                // ✅ 仅降低透明度和饱和度，但保持pointerEvents可用（让修复按钮可点击）
                smartPanel.style.opacity = '0.7';
                smartPanel.style.pointerEvents = 'auto';  // 不禁用，让修复按钮可点击
                smartPanel.style.filter = 'grayscale(30%)';
            }
        }
        
        // 🎯 手动模式：总是可用（有本地工具兜底）
        if (manualPanel) {
            manualPanel.style.opacity = '1';
            manualPanel.style.pointerEvents = 'auto';
            manualPanel.style.filter = 'none';
        }
        
    } catch (error) {
        log.error('[PIXLY Image] Core detection failed:', error);
        
        if (smartStatusEl) {
            smartStatusEl.innerHTML = `<span style="color: #F44336;">🤖 ✗</span>`;
        }
        if (manualStatusEl) {
            manualStatusEl.innerHTML = `<span style="color: #FFA500;">🔧 Local</span>`;
        }
        if (smartPanel) {
            smartPanel.style.opacity = '0.5';
            smartPanel.style.pointerEvents = 'none';
            smartPanel.style.filter = 'grayscale(50%)';
        }
    }
}

/**
 * 🎯 范围收束功能处理器初始化
 */
function initScopeFilterHandlers() {
    const log = window.pixlyLog;
    // 主开关
    const enableScopeFilter = document.getElementById('enableScopeFilter');
    const scopeFilterOptions = document.getElementById('scopeFilterOptions');
    
    // 文件大小过滤
    const enableSizeFilter = document.getElementById('enableSizeFilter');
    const sizeFilterInputs = document.getElementById('sizeFilterInputs');
    
    // 分辨率过滤
    const enableResolutionFilter = document.getElementById('enableResolutionFilter');
    const resolutionFilterInputs = document.getElementById('resolutionFilterInputs');
    
    // 主开关处理
    if (enableScopeFilter && scopeFilterOptions) {
        enableScopeFilter.addEventListener('change', function() {
            scopeFilterOptions.style.display = this.checked ? 'block' : 'none';
            log.info('[PIXLY Scope Filter] Scope narrowing:', this.checked ? 'enabled' : 'disabled');
        });
    }
    
    // 文件大小过滤开关
    if (enableSizeFilter && sizeFilterInputs) {
        enableSizeFilter.addEventListener('change', function() {
            sizeFilterInputs.style.display = this.checked ? 'block' : 'none';
            log.info('[PIXLY Scope Filter] File size filter:', this.checked ? 'enabled' : 'disabled');
        });
    }
    
    // 分辨率过滤开关
    if (enableResolutionFilter && resolutionFilterInputs) {
        enableResolutionFilter.addEventListener('change', function() {
            resolutionFilterInputs.style.display = this.checked ? 'block' : 'none';
            log.info('[PIXLY Scope Filter] Resolution filter:', this.checked ? 'enabled' : 'disabled');
        });
    }
    
    log.debug?.('PIXLY Scope Filter', LOG.UI_SCOPE_FILTER_INIT)
}

/**
 * 🎯 检查文件是否通过范围收束过滤
 * @param {Object} file - 文件对象
 * @param {Object} imageInfo - 图像信息（包含宽度、高度等）
 * @returns {Object} { pass: boolean, reason: string }
 */
function checkScopeFilter(file, imageInfo) {
    const enableScopeFilter = document.getElementById('enableScopeFilter');
    
    // 如果未启用范围收束，全部通过
    if (!enableScopeFilter || !enableScopeFilter.checked) {
        return { pass: true, reason: '' };
    }
    
    // 文件大小过滤
    const enableSizeFilter = document.getElementById('enableSizeFilter');
    if (enableSizeFilter && enableSizeFilter.checked) {
        const minFileSize = parseFloat(document.getElementById('minFileSize')?.value || 0);
        const maxFileSize = parseFloat(document.getElementById('maxFileSize')?.value || 0);
        const fileSizeMB = file.size / (1024 * 1024);
        
        if (minFileSize > 0 && fileSizeMB < minFileSize) {
            return { pass: false, reason: `文件大小 ${fileSizeMB.toFixed(2)}MB 小于最小值 ${minFileSize}MB` };
        }
        
        if (maxFileSize > 0 && fileSizeMB > maxFileSize) {
            return { pass: false, reason: `文件大小 ${fileSizeMB.toFixed(2)}MB 大于最大值 ${maxFileSize}MB` };
        }
    }
    
    // 分辨率过滤
    const enableResolutionFilter = document.getElementById('enableResolutionFilter');
    if (enableResolutionFilter && enableResolutionFilter.checked && imageInfo) {
        const minWidth = parseInt(document.getElementById('minWidth')?.value || 0);
        const minHeight = parseInt(document.getElementById('minHeight')?.value || 0);
        const maxWidth = parseInt(document.getElementById('maxWidth')?.value || 0);
        const maxHeight = parseInt(document.getElementById('maxHeight')?.value || 0);
        
        const width = imageInfo.width || 0;
        const height = imageInfo.height || 0;
        
        if (minWidth > 0 && width < minWidth) {
            return { pass: false, reason: `图片宽度 ${width}px 小于最小值 ${minWidth}px` };
        }
        
        if (minHeight > 0 && height < minHeight) {
            return { pass: false, reason: `图片高度 ${height}px 小于最小值 ${minHeight}px` };
        }
        
        if (maxWidth > 0 && width > maxWidth) {
            return { pass: false, reason: `图片宽度 ${width}px 大于最大值 ${maxWidth}px` };
        }
        
        if (maxHeight > 0 && height > maxHeight) {
            return { pass: false, reason: `图片高度 ${height}px 大于最大值 ${maxHeight}px` };
        }
    }
    
    return { pass: true, reason: '' };
}

/**
 * 🔥 Phase 46.5.14: 更新优化模式说明卡片/**
 * 根据选择的优化模式动态更新图像面板UI说明
 */
function updateOptimizeModeInfoCard(mode) {
    const card = document.getElementById('optimizeModeInfoCard');
    const icon = document.getElementById('optimizeModeIcon');
    const title = document.getElementById('optimizeModeTitle');
    const desc = document.getElementById('optimizeModeDesc');
    const hint = document.getElementById('optimizeModeHint');
    const advancedOptions = document.getElementById('advancedAIOptions');
    const generalModeRules = document.getElementById('generalModeRules');
    
    if (!card || !icon || !title || !desc || !hint) {
        log.warn('[PIXLY UI] ⚠️ Optimize mode info card elements not found');
        return;
    }
    
    // 根据不同模式设置不同的内容
    const i18n = window.i18n || { t: (key) => key };
    
    switch(mode) {
        case 'general':
        case 'universal':
            // 通用模式 - 隐藏AI高级选项，显示规则说明，隐藏自定义格式
            card.style.background = 'linear-gradient(135deg, rgba(156, 163, 175, 0.15) 0%, rgba(107, 114, 128, 0.15) 100%)';
            card.style.borderColor = 'rgba(156, 163, 175, 0.3)';
            icon.textContent = '🔧';
            title.textContent = i18n.t('ai.universalModeTitle');
            desc.textContent = i18n.t('ai.universalModeDesc');
            hint.innerHTML = i18n.t('ai.universalModeHint');
            if (advancedOptions) advancedOptions.style.display = 'none';
            if (generalModeRules) generalModeRules.style.display = 'block';
            // 隐藏自定义格式卡片
            const expectedFormatContainer = document.getElementById('expectedFormatContainer');
            if (expectedFormatContainer) expectedFormatContainer.style.display = 'none';
            log.info('[PIXLY UI] 📋 Updated to General mode info');
            break;
            
        case 'size':
            // 体积优先 - 显示高级AI选项和自定义格式，隐藏规则说明
            card.style.background = 'linear-gradient(135deg, rgba(59, 130, 246, 0.15) 0%, rgba(37, 99, 235, 0.15) 100%)';
            card.style.borderColor = 'rgba(59, 130, 246, 0.3)';
            icon.textContent = '📦';
            title.textContent = i18n.t('ai.sizeModeTitle');
            desc.textContent = i18n.t('ai.sizeModeDesc');
            hint.textContent = i18n.t('ai.sizeModeHint');
            if (advancedOptions) advancedOptions.style.display = 'grid';
            if (generalModeRules) generalModeRules.style.display = 'none';
            const expectedFormatSize = document.getElementById('expectedFormatContainer');
            if (expectedFormatSize) expectedFormatSize.style.display = 'block';
            log.info('[PIXLY UI] 📦 Updated to Size mode info');
            break;
            
        case 'quality':
            // 质量优先 - 显示高级AI选项和自定义格式，隐藏规则说明
            card.style.background = 'linear-gradient(135deg, rgba(168, 85, 247, 0.15) 0%, rgba(147, 51, 234, 0.15) 100%)';
            card.style.borderColor = 'rgba(168, 85, 247, 0.3)';
            icon.textContent = '💎';
            title.textContent = i18n.t('ai.qualityModeTitle');
            desc.textContent = i18n.t('ai.qualityModeDesc');
            hint.textContent = i18n.t('ai.qualityModeHint');
            if (advancedOptions) advancedOptions.style.display = 'grid';
            if (generalModeRules) generalModeRules.style.display = 'none';
            const expectedFormatQuality = document.getElementById('expectedFormatContainer');
            if (expectedFormatQuality) expectedFormatQuality.style.display = 'block';
            log.info('[PIXLY UI] 💎 Updated to Quality mode info');
            break;
            
        case 'balanced':
        default:
            // 智能平衡（默认）- 显示高级AI选项和自定义格式，隐藏规则说明
            card.style.background = 'linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(76, 175, 80, 0.15) 100%)';
            card.style.borderColor = 'rgba(76, 175, 80, 0.3)';
            icon.textContent = '✨';
            title.textContent = i18n.t('ai.balancedModeTitle');
            desc.textContent = i18n.t('ai.balancedModeDesc');
            hint.innerHTML = i18n.t('ai.balancedModeHint');
            if (advancedOptions) advancedOptions.style.display = 'grid';
            if (generalModeRules) generalModeRules.style.display = 'none';
            const expectedFormatBalanced = document.getElementById('expectedFormatContainer');
            if (expectedFormatBalanced) expectedFormatBalanced.style.display = 'block';
            log.info('[PIXLY UI] ✨ Updated to Balanced mode info');
            break;
    }
}

/**
 * 根据选择的视频AI预设动态更新视频面板UI说明
 */
function updateVideoOptimizeModeInfoCard(preset) {
    const card = document.getElementById('videoOptimizeModeInfoCard');
    const icon = document.getElementById('videoOptimizeModeIcon');
    const title = document.getElementById('videoOptimizeModeTitle');
    const desc = document.getElementById('videoOptimizeModeDesc');
    const hint = document.getElementById('videoOptimizeModeHint');
    
    if (!card || !icon || !title || !desc || !hint) {
        log.warn('[PIXLY Video AI] ⚠️ Video optimize mode info card elements not found');
        return;
    }
    
    // 根据不同预设设置不同的内容
    const i18n = window.i18n || { t: (key) => key };
    
    switch(preset) {
        case 'fast':
            // 快速模式
            card.style.background = 'linear-gradient(135deg, rgba(59, 130, 246, 0.15) 0%, rgba(37, 99, 235, 0.15) 100%)';
            card.style.borderColor = 'rgba(59, 130, 246, 0.3)';
            icon.textContent = '⚡';
            title.textContent = i18n.t('videoAI.fastModeTitle');
            desc.textContent = i18n.t('videoAI.fastModeDesc');
            hint.innerHTML = i18n.t('videoAI.fastModeHint');
            log.info('[PIXLY Video AI] ⚡ Updated to Fast mode info');
            break;
            
        case 'quality':
            // 质量/完整模式
            card.style.background = 'linear-gradient(135deg, rgba(168, 85, 247, 0.15) 0%, rgba(147, 51, 234, 0.15) 100%)';
            card.style.borderColor = 'rgba(168, 85, 247, 0.3)';
            icon.textContent = '⭐';
            title.textContent = i18n.t('videoAI.qualityModeTitle');
            desc.textContent = i18n.t('videoAI.qualityModeDesc');
            hint.innerHTML = i18n.t('videoAI.qualityModeHint');
            log.info('[PIXLY Video AI] ⭐ Updated to Quality mode info');
            break;
            
        case 'balanced':
        default:
            // 平衡模式（默认）
            card.style.background = 'linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(76, 175, 80, 0.15) 100%)';
            card.style.borderColor = 'rgba(76, 175, 80, 0.3)';
            icon.textContent = '✨';
            title.textContent = i18n.t('videoAI.balancedModeTitle');
            desc.textContent = i18n.t('videoAI.balancedModeDesc');
            hint.innerHTML = i18n.t('videoAI.balancedModeHint');
            log.info('[PIXLY Video AI] ✨ Updated to Balanced mode info');
            break;
    }
}

/**
 * 🔥 Phase 45.8: 图像处理面板核心状态检测
 * 检测GO核心状态（用于智能模式）
 */
async function detectImageCoreStatus() {
    const smartStatusEl = document.getElementById('imageGoCoreStatus');
    const manualStatusEl = document.getElementById('imageManualCoreStatus');
    const smartPanel = document.getElementById('aiOptionsSection');
    const manualPanel = document.getElementById('manualOptions');
    
    // 🔥 Phase 46.5.11: 检查是否为通用模式（规则引擎，无需AI）
    const generalPreset = document.querySelector('input[name="optimizeMode"][value="general"]');
    const isGeneralMode = generalPreset && generalPreset.checked;
    
    // 如果是通用模式，显示规则模式状态并跳过AI检测
    if (isGeneralMode && smartStatusEl) {
        const i18n = window.i18n || { t: (key) => key };
        smartStatusEl.innerHTML = `<span style="color: #10B981;">${i18n.t('ai.ruleMode')}</span>`;
        if (smartPanel) {
            smartPanel.style.opacity = '1';
            smartPanel.style.pointerEvents = 'auto';
            smartPanel.style.filter = 'none';
        }
        return; // 跳过AI检测
    }
    
    try {
        const goResult = await detectGoCore();
        
        // 🤖 Update smart mode status
        const i18n = window.i18n || { t: (key) => key };
        if (smartStatusEl) {
            if (goResult.available) {
                smartStatusEl.innerHTML = `<span style="color: #4CAF50;">🤖 ${i18n.t('mode.smartModeReady')} ✅</span>`;
                log.info(`[PIXLY Image Smart] GO core ready: ${goResult.version}`);
            } else {
                // 🔥 Phase 45.8: Provide fix option when AI offline
                smartStatusEl.innerHTML = `
                    <span style="color: #F44336;">❌ ${i18n.t('ai.offline')}</span>
                    <button onclick="window.PIXLY.UIHandlers.fixAICore()" 
                            style="margin-left: 8px; padding: 2px 8px; font-size: 11px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                            onmouseover="this.style.background='#45a049'" 
                            onmouseout="this.style.background='#4CAF50'">
                        🔧 ${i18n.t('common.fix')}
                    </button>
                `;
                log.info('[PIXLY Image Smart] GO core not connected');
            }
        }
        
        // 🔧 更新手动模式状态（手动模式可使用本地工具，不强制要求GO核心）
        if (manualStatusEl) {
            if (goResult.available) {
                manualStatusEl.innerHTML = `<span style="color: #4CAF50;">🔧 ${i18n.t('mode.manualModeWithGo')} ✅</span>`;
                log.info(`[PIXLY Image Manual] Tools + GO ready`);
            } else {
                manualStatusEl.innerHTML = `<span style="color: #FFA500;">🔧 Local</span>`;
                log.info('[PIXLY Image Manual] Local tools only');
            }
        }
        
        // 🎯 智能模式：需要GO核心
        // 🔥 Phase 46.1: AI离线时不禁用pointerEvents，以便修复按钮可以点击
        if (smartPanel) {
            if (goResult.available) {
                smartPanel.style.opacity = '1';
                smartPanel.style.pointerEvents = 'auto';
                smartPanel.style.filter = 'none';
            } else {
                // ✅ 仅降低透明度和饱和度，但保持pointerEvents可用（让修复按钮可点击）
                smartPanel.style.opacity = '0.7';
                smartPanel.style.pointerEvents = 'auto';  // 不禁用，让修复按钮可点击
                smartPanel.style.filter = 'grayscale(30%)';
            }
        }
        
        // 🎯 手动模式：总是可用（有本地工具兜底）
        if (manualPanel) {
            manualPanel.style.opacity = '1';
            manualPanel.style.pointerEvents = 'auto';
            manualPanel.style.filter = 'none';
        }
        
    } catch (error) {
        log.error('[PIXLY Image] Core detection failed:', error);
        
        if (smartStatusEl) {
            smartStatusEl.innerHTML = `
                <span style="color: #F44336;">❌ AI离线</span>
                <button onclick="window.PIXLY.UIHandlers.fixAICore()" 
                        style="margin-left: 8px; padding: 2px 8px; font-size: 11px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                        onmouseover="this.style.background='#45a049'" 
                        onmouseout="this.style.background='#4CAF50'">
                    🔧 修复
                </button>
            `;
        }
        if (manualStatusEl) {
            manualStatusEl.innerHTML = `<span style="color: #FFA500;">🔧 Local</span>`;
        }
        if (smartPanel) {
            smartPanel.style.opacity = '0.5';
            smartPanel.style.pointerEvents = 'none';
            smartPanel.style.filter = 'grayscale(50%)';
        }
    }
}

/**
 * 🔥 视频处理面板核心状态检测
 * 检测GO核心和Rust核心状态
 */
async function detectVideoCoreStatus() {
    const log = window.pixlyLog;
    const goSmartStatusEl = document.getElementById('videoGoCoreInline');
    const rustManualStatusEl = document.getElementById('videoManualRustCoreStatus');
    const smartPanel = document.getElementById('videoAIOptionsSection');
    const manualPanel = document.getElementById('videoManualOptions');
    
    let goAvailable = false;
    let rustAvailable = false;
    
    // 🤖 检测GO核心（智能模式用）
    if (goSmartStatusEl) {
        try {
            const goResult = await detectGoCore();
            goAvailable = goResult.available;
            if (goResult.available) {
                const i18n = window.i18n || { t: (key) => key };
                goSmartStatusEl.innerHTML = `<span style="color: #4CAF50;">🤖 ${i18n.t('ai.online')} ✅</span>`;
                log.info?.('PIXLY GO', LOG.GO_CORE_READY, { version: goResult.version })
            } else {
                // 🔥 Phase 45.8: Provide fix option when AI offline
                const i18n = window.i18n || { t: (key) => key };
                goSmartStatusEl.innerHTML = `
                    <span style="color: #F44336;">❌ ${i18n.t('ai.offline')}</span>
                    <button onclick="window.PIXLY.UIHandlers.fixAICore()" 
                            style="margin-left: 8px; padding: 2px 8px; font-size: 11px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                            onmouseover="this.style.background='#45a049'" 
                            onmouseout="this.style.background='#4CAF50'">
                        🔧 ${i18n.t('common.fix')}
                    </button>
                `;
                log.info('[PIXLY Video Smart] GO core not connected');
            }
        } catch (error) {
            const i18n = window.i18n || { t: (key) => key };
            goSmartStatusEl.innerHTML = `
                <span style="color: #F44336;">❌ ${i18n.t('ai.offline')}</span>
                <button onclick="window.PIXLY.UIHandlers.fixAICore()" 
                        style="margin-left: 8px; padding: 2px 8px; font-size: 11px; background: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;"
                        onmouseover="this.style.background='#45a049'" 
                        onmouseout="this.style.background='#4CAF50'">
                    🔧 ${i18n.t('common.fix')}
                </button>
            `;
            log.error('[PIXLY Video Smart] GO core detection failed:', error);
        }
    }
    
    // ⚙️ 检测Rust核心（手动模式用）
    if (rustManualStatusEl) {
        try {
            const rustResult = await detectRustCore();
            rustAvailable = rustResult.available;
            if (rustResult.available) {
                rustManualStatusEl.innerHTML = `<span style="color: #4CAF50;">⚙️ Rust核心 ✅</span>`;
                log.info(`[PIXLY Video Manual] Rust core ready: ${rustResult.version}`);
            } else {
                rustManualStatusEl.innerHTML = `<span style="color: #F44336;">⚙️ ✗</span>`;
                log.info('[PIXLY Video Manual] Rust core not connected');
            }
        } catch (error) {
            rustManualStatusEl.innerHTML = `<span style="color: #F44336;">⚙️ ✗</span>`;
            log.error('[PIXLY Video Manual] Rust core detection failed:', error);
        }
    }
    
    // 🎯 智能模式：需要GO核心（主要用于AI预测）
    // 🔥 Phase 46.1: AI离线时不禁用pointerEvents，以便修复按钮可以点击
    if (smartPanel) {
        if (goAvailable) {
            smartPanel.style.opacity = '1';
            smartPanel.style.pointerEvents = 'auto';
            smartPanel.style.filter = 'none';
        } else {
            // ✅ 仅降低透明度和饱和度，但保持pointerEvents可用（让修复按钮可点击）
            smartPanel.style.opacity = '0.7';
            smartPanel.style.pointerEvents = 'auto';  // 不禁用，让修复按钮可点击
            smartPanel.style.filter = 'grayscale(30%)';
        }
    }
    
    // 🎯 手动模式：需要Rust核心（用于编解码）
    if (manualPanel) {
        if (rustAvailable) {
            manualPanel.style.opacity = '1';
            manualPanel.style.pointerEvents = 'auto';
            manualPanel.style.filter = 'none';
        } else {
            manualPanel.style.opacity = '0.5';
            manualPanel.style.pointerEvents = 'none';
            manualPanel.style.filter = 'grayscale(50%)';
        }
    }
}

/**
 * 🔥 detectedGO核心状态
 * tryconnectedto本地gRPCservice
 * 
 * 添加缓存机制，避免重复检测
 */
let _goCoreCache = null;
let _goCoreDetecting = null;  // 防止并发检测

async function detectGoCore() {
    const log = window.pixlyLog;
    // 返回缓存结果（5分钟内有效）
    if (_goCoreCache && (Date.now() - _goCoreCache.timestamp < 5 * 60 * 1000)) {
        log.debug?.('PIXLY GO', LOG.GO_CACHED_STATUS)
        return _goCoreCache.result;
    }
    
    // 如果正在检测，等待结果
    if (_goCoreDetecting) {
        // log已在函数开头声明，无需重复 (原行3059)
        log.debug?.('PIXLY GO', 'Waiting for concurrent detection...')
        return await _goCoreDetecting;
    }
    
    // 开始新的检测
    // log已在函数开头声明，无需重复 (原行3065)
    log.debug?.('PIXLY GO', LOG.GO_DETECTING)
    
    const { spawn } = require('child_process');
    const net = require('net');
    
    // tryport列表
    const ports = [50051, 50052, 50053];
    
    // 创建检测Promise
    _goCoreDetecting = (async () => {
        try {
            return await _detectGoCoreInternal(net, ports);
        } finally {
            _goCoreDetecting = null;  // 清除检测标志
        }
    })();
    
    const result = await _goCoreDetecting;
    
    // 缓存结果
    _goCoreCache = {
        result,
        timestamp: Date.now()
    };
    
    return result;
}

// 内部检测函数
async function _detectGoCoreInternal(net, ports) {
    const log = window.pixlyLog;
    
    for (const port of ports) {
        try {
            // detectedportis否被占用
            const isListening = await new Promise((resolve) => {
                const socket = new net.Socket();
                socket.setTimeout(1000);
                
                socket.on('connect', () => {
                    socket.destroy();
                    resolve(true);
                });
                
                socket.on('timeout', () => {
                    socket.destroy();
                    resolve(false);
                });
                
                socket.on('error', () => {
                    resolve(false);
                });
                
                const host = window.CONFIG?.services?.ai?.host || 'localhost';
                socket.connect(port, host);
            });
            
            if (isListening) {
                // try获取versioninfo
                log.info?.('PIXLY GO', LOG.GO_SERVICE_DETECTED, { port })
                
                // passedHTTP API获取实际version
                let version = '1.0.0';
                try {
                    const http = require('http');
                    const versionData = await new Promise((resolve, reject) => {
                        const timeout = setTimeout(() => reject(new Error('timeout')), 3000);
                        const host = window.CONFIG?.services?.ai?.host || 'localhost';
                        const req = http.get(`http://${host}:${port}/api/v1/version`, (res) => {
                            clearTimeout(timeout);
                            let data = '';
                            res.on('data', chunk => data += chunk);
                            res.on('end', () => {
                                try {
                                    const parsed = JSON.parse(data);
                                    log.info(`[PIXLY GO] 📥 Version response:`, parsed);
                                    resolve(parsed);
                                } catch (e) {
                                    log.warn(`[PIXLY GO] ⚠️ Failed to parse version JSON:`, e);
                                    resolve({ version: '1.0.0' });
                                }
                            });
                        });
                        req.on('error', (err) => {
                            log.warn(`[PIXLY GO] ⚠️ Version request error:`, err.message);
                            resolve({ version: '1.0.0' });
                    });
                    });
                    version = versionData.version || '1.0.0';
                    log.info(`[PIXLY GO] ✅ Service version detected: ${version} on port ${port}`);
                } catch (e) {
                    log.warn(`[PIXLY GO] ⚠️ Cannot get version info, using default:`, e.message);
                }
                
                return {
                    available: true,
                    port: port,
                    version: version,
                    features: ['quality-prediction', 'auto-optimize', 'ssim-validation']
                };
            }
        } catch (error) {
 log.warn(`[PIXLY GO] ⚠️ Port ${port} detection failed:`, error.message);
        }
    }
    
    // allport都not available
    return {
        available: false,
        port: null,
        version: null,
        features: []
    };
}

/**
 * 🆕 自动启动GO核心服务 (Fallback机制)
 * 尝试多种方式启动GO核心：
 * 1. 使用已编译的 ai-service 可执行文件
 * 2. 使用 start_ai_service.sh 脚本
 * 3. 使用 go run 直接运行
 */
async function autoStartGoCore() {
    const { spawn } = require('child_process');
    const fs = require('fs');
    const path = require('path');
    
    log.info('[PIXLY GO] 🚀 Attempting to auto-start GO core...');
    
    try {
        // 🔥 方案1: 从window.location推导项目根目录（通用方案）
        let projectRoot = null;
        
        try {
            const currentUrl = window.location.href;
            // 检查是否在plugin或plugin_v3目录中
            let pluginIndex = -1;
            if (currentUrl.includes('/plugin/')) {
                pluginIndex = currentUrl.indexOf('/plugin/');
            } else if (currentUrl.includes('/plugin_v3/')) {
                pluginIndex = currentUrl.indexOf('/plugin_v3/');
            }
            
            if (pluginIndex !== -1) {
                // 从 file://.../plugin/... 推导到项目根目录
                projectRoot = currentUrl.substring(7, pluginIndex); // 7 = 'file://'.length
                log.info('[PIXLY GO] 📂 Derived project root from URL:', projectRoot);
            } else {
                log.warn('[PIXLY GO] ⚠️ Cannot find plugin directory in URL:', currentUrl);
            }
        } catch (e) {
            log.warn('[PIXLY GO] ⚠️ Cannot derive project root from URL:', e.message);
        }
        
        // 🔥 方案2: 如果URL推导失败，尝试在PATH中查找ai-service
        if (!projectRoot) {
            log.debug?.('PIXLY GO', 'Trying to find ai-service in PATH...')
            try {
                const { execSync } = require('child_process');
                const whichResult = execSync('which ai-service 2>/dev/null || which pixly-ai 2>/dev/null', { encoding: 'utf8' }).trim();
                if (whichResult) {
                    log.info?.('PIXLY GO', 'Found ai-service in PATH: {path}', { path: whichResult })
                    // 直接使用PATH中的命令，不需要projectRoot
                    projectRoot = 'USE_PATH_COMMAND';
                }
            } catch (e) {
                log.warn?.('PIXLY GO', 'ai-service not found in PATH')
            }
        }
        
        // 🔥 如果没有找到项目路径，返回友好错误
        if (!projectRoot) {
            log.error('[PIXLY GO] ❌ Cannot locate project root or ai-service command');
            return {
                success: false,
                error: 'Please install ai-service to PATH or ensure plugin is in correct location',
                suggestion: 'Run: brew install pixly-ai  OR  manually start: ./start_ai_service.sh'
            };
        }
        
        // 🔥 方法1: 使用PATH中的ai-service命令（通用方案）
        if (projectRoot === 'USE_PATH_COMMAND') {
            log.info('[PIXLY GO] 🚀 Starting ai-service from PATH...');
            
            try {
                const child = spawn('ai-service', [], {
                    detached: true,
                    stdio: 'ignore',
                    shell: true
                });
                
                child.unref();
                
                log.info('[PIXLY GO] ✅ ai-service started from PATH (PID: ' + child.pid + ')');
                return { success: true, method: 'path-command', pid: child.pid };
            } catch (e) {
                log.error('[PIXLY GO] ❌ Failed to start ai-service from PATH:', e.message);
                return { success: false, error: e.message };
            }
        }
        
        // 🔥 方法2-4: 尝试本地项目文件（开发模式）
        log.info('[PIXLY GO] 🔍 Trying local project files in:', projectRoot);
        
        // 方法2: 编译的 ai-service 二进制 (多个可能位置)
        const possiblePaths = [
            path.join(projectRoot, 'bin', 'ai-service'),
            path.join(projectRoot, 'cmd', 'ai-service', 'ai-service'),
            path.join(projectRoot, 'ai-service'),
            path.join(projectRoot, 'build', 'ai-service')
        ];
        
        for (const aiServicePath of possiblePaths) {
            if (fs.existsSync(aiServicePath)) {
                log.info('[PIXLY GO] 📦 Found compiled ai-service at:', aiServicePath);
                
                const child = spawn(aiServicePath, [], {
                    detached: true,
                    stdio: 'ignore',
                    cwd: projectRoot
                });
                
                child.unref();
                
                log.info('[PIXLY GO] ✅ ai-service started (PID: ' + child.pid + ')');
                return { success: true, method: 'local-binary', pid: child.pid, path: aiServicePath };
            }
        }
        
        log.info('[PIXLY GO] ⚠️ No compiled ai-service found in standard locations');
        
        // 方法3: 启动脚本
        const startScriptPath = path.join(projectRoot, 'start_ai_service.sh');
        if (fs.existsSync(startScriptPath)) {
            log.info('[PIXLY GO] 📜 Found start_ai_service.sh, executing...');
            
            const child = spawn('bash', [startScriptPath], {
                detached: true,
                stdio: 'ignore',
                cwd: projectRoot
            });
            
            child.unref();
            
            log.info('[PIXLY GO] ✅ start_ai_service.sh executed (PID: ' + child.pid + ')');
            return { success: true, method: 'start-script', pid: child.pid };
        }
        
        // 方法4: go run (最慢，仅开发)
        const mainGoPath = path.join(projectRoot, 'cmd', 'ai-service', 'main.go');
        if (fs.existsSync(mainGoPath)) {
            log.info('[PIXLY GO] 🔧 Found main.go, using go run...');
            
            const child = spawn('go', ['run', mainGoPath], {
                detached: true,
                stdio: 'ignore',
                cwd: projectRoot
            });
            
            child.unref();
            
            log.info('[PIXLY GO] ✅ go run started (PID: ' + child.pid + ')');
            return { success: true, method: 'go-run', pid: child.pid };
        }
        
        // 所有方法都失败 - 返回友好错误
        log.error('[PIXLY GO] ❌ No suitable method found to start GO core');
        log.error('[PIXLY GO] 💡 Suggestion: Install ai-service or compile the project');
        return {
            success: false,
            error: 'GO core not found in PATH or local project',
            suggestion: 'Please run: cd "' + projectRoot + '" && go build -o ai-service cmd/ai-service/main.go'
        };
        
    } catch (error) {
        log.error('[PIXLY GO] ❌ Auto-start failed:', error.message);
        return {
            success: false,
            error: error.message
        };
    }
}

/**
 * 🔥 检测核心服务并启用手动模式
 * 用于手动模式下的GO/Rust核心检测与自动修复
 */
async function checkAndEnableManualMode() {
    log.info('[PIXLY Manual] 🔍 Checking core services for manual mode...');
    
    try {
        // 🔥 0. 首先检查 Rust CLI（最重要的核心）
        const rustCLI = window.rustCLI;
        if (rustCLI && rustCLI.available) {
            log.info('[PIXLY Manual] ✅ Rust CLI available, enabling controls immediately...');
            enableManualModeControls(true);
            showManualModeStatus('✓ Rust CLI', 'success');
            return; // 早期返回，Rust CLI可用即可
        }
        
        // 1. 检测 GO AI Service
        const goResult = await detectGoCore();
        
        // 2. 检测 Rust Service (HTTP)
        const rustResult = await detectRustService();
        
        const goCoreAvailable = goResult.available;
        const rustCoreAvailable = rustResult.available;
        
        // 🔥 修复：Rust HTTP服务可用时也应该立即启用控件
        if (rustCoreAvailable) {
            log.info('[PIXLY Manual] ✅ Rust HTTP service available, enabling controls...');
            enableManualModeControls(true);
            
            if (goCoreAvailable) {
                showManualModeStatus('✓ GO + Rust', 'success');
            } else {
                showManualModeStatus('✓ Rust HTTP', 'success');
            }
            return; // 早期返回，不执行后续逻辑
        }
        
        // 3. Rust不可用，检查GO
        if (goCoreAvailable) {
            log.info('[PIXLY Manual] ✅ GO service available, enabling controls...');
            enableManualModeControls(true);
            showManualModeStatus('✓ GO only', 'success');
            return;
        }
        
        // 4. 两个服务都不可用，尝试自动启动
        log.warn('[PIXLY Manual] ⚠️ No core services available, attempting auto-start...');
            showManualModeStatus('🔄 Starting...', 'warning');
            
            // 尝试启动 GO Service
        await autoStartGoCore();
            
            // 等待2秒再检测
            await new Promise(resolve => setTimeout(resolve, 2000));
            
            // 重新检测
            const goRecheck = await detectGoCore();
            const rustRecheck = await detectRustService();
            
            if (goRecheck.available || rustRecheck.available) {
            log.info('[PIXLY Manual] ✅ Service started successfully');
                enableManualModeControls(true);
                showManualModeStatus('✓ Ready', 'success');
            } else {
                // 启动失败，禁用所有控件
            log.error('[PIXLY Manual] ❌ Failed to start services');
                disableManualModeControls();
                showManualModeStatus('✗ Not available', 'error');
                showServiceStartGuide();
        }
        
    } catch (error) {
        log.error('[PIXLY Manual] ❌ Error checking services:', error.message);
        // 🔥 修复：检测失败时不应立即禁用，可能Rust CLI仍可用
        const rustCLI = window.rustCLI;
        if (rustCLI && rustCLI.available) {
            log.info('[PIXLY Manual] 💡 Fallback to Rust CLI');
            enableManualModeControls(true);
            showManualModeStatus('✓ Rust CLI', 'success');
        } else {
        disableManualModeControls();
        showManualModeStatus('✗ Check failed', 'error');
        }
    }
}

/**
 * 🔥 检测 Rust 服务状态
 */
async function detectRustService() {
    try {
        const http = require('http');
        
        const result = await new Promise((resolve) => {
            const timeout = setTimeout(() => resolve({ available: false }), 1000);
            
            // 使用配置系统获取主机地址和端口
            const host = window.CONFIG?.services?.rust?.host || 'localhost';
            const port = window.CONFIG?.services?.rust?.port || 8080;
            const req = http.get(`http://${host}:${port}/health`, (res) => {
                clearTimeout(timeout);
                
                if (res.statusCode === 200) {
                    let data = '';
                    res.on('data', chunk => data += chunk);
                    res.on('end', () => {
                        try {
                            const json = JSON.parse(data);
                            resolve({
                                available: true,
                                version: json.version || 'unknown',
                                port: 8080
                            });
                        } catch {
                            resolve({ available: true, version: 'unknown', port: 8080 });
                        }
                    });
                } else {
                    resolve({ available: false });
                }
            });
            
            req.on('error', () => {
                clearTimeout(timeout);
                resolve({ available: false });
            });
        });
        
        if (result.available) {
            log.info(`[PIXLY Rust] ✅ Service running on port ${result.port}`);
        }
        
        return result;
    } catch (error) {
        return { available: false };
    }
}

/**
 * 🔥 检测Rust核心状态
 * 尝试连接到本地Rust服务
 */
async function detectRustCore() {
    // 🔥 Phase 45.7: 直接检查Rust CLI可用性（不是HTTP服务）
    const rustCLI = window.rustCLI;
    if (rustCLI && rustCLI.available) {
        return {
            available: true,
            version: rustCLI.version || 'Unknown',
            source: 'CLI'
        };
    }
    
    // 🔥 向下兼容：检测Rust HTTP服务（如果存在）
    const net = require('net');
    const ports = [8080, 50061, 50062, 50063];
    
    for (const port of ports) {
        try {
            // 检测端口是否被占用
            const isListening = await new Promise((resolve) => {
                const socket = new net.Socket();
                socket.setTimeout(1000);
                
                socket.on('connect', () => {
                    socket.destroy();
                    resolve(true);
                });
                
                socket.on('timeout', () => {
                    socket.destroy();
                    resolve(false);
                });
                
                socket.on('error', () => {
                    resolve(false);
                });
                
                const host = window.CONFIG?.services?.ai?.host || 'localhost';
                socket.connect(port, host);
            });
            
            if (isListening) {
                log.info(`[PIXLY Rust] ✅ Detected Rust service on port ${port}`);
                
                // 尝试获取版本信息
                let version = 'v1.0.0';
                try {
                    const http = require('http');
                    const versionData = await new Promise((resolve, reject) => {
                        const timeout = setTimeout(() => reject(new Error('timeout')), 2000);
                        const host = window.CONFIG?.services?.ai?.host || 'localhost';
                        const req = http.get(`http://${host}:${port}/api/v1/version`, (res) => {
                            clearTimeout(timeout);
                            let data = '';
                            res.on('data', chunk => data += chunk);
                            res.on('end', () => {
                                try {
                                    resolve(JSON.parse(data));
                                } catch (e) {
                                    resolve({ version: 'unknown' });
                                }
                            });
                        });
                        req.on('error', reject);
                    });
                    
                    if (versionData && versionData.version) {
                        version = versionData.version;
                    }
                } catch (versionError) {
                    log.warn('[PIXLY Rust] ⚠️ Cannot get Rust core version:', versionError.message);
                }
                
                return {
                    available: true,
                    port: port,
                    version: version
                };
            }
        } catch (error) {
            log.warn(`[PIXLY Rust] Port ${port} check failed:`, error.message);
        }
    }
    
    return { available: false };
}

/**
 * 🔥 禁用手动模式所有控件
 */
function disableManualModeControls() {
    log.info('[PIXLY Manual] 🚫 Disabling all manual mode controls...');
    
    const controlIds = [
        // 图像转换控件
        'quality',
        'jxlEffort',
        'jxlDecodeSpeed',
        'jxlDistance',
        'jxlPhotonNoise',
        'webpMethod',
        'webpAutoFilter',
        'webpSharpness',
        'avifSpeed',
        'avifTiles',
        'heicQuality',
        'manualLossless',
        'enableJpegLossless', // 🔥 Phase 45.5: 恢复enableJpegLossless
        // 🔥 Phase 45.6: 添加视频转换控件
        'videoSpeed',
        'videoCRF',
        'videoContainer',
        'videoConversionAdvancedBtn',
        'enableHardwareAccel',
        'enableTwoPass'
    ];
    
    controlIds.forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.disabled = true;
            el.style.opacity = '0.3';
            el.style.cursor = 'not-allowed';
        }
    });
    
    // 禁用格式选择（图像）
    const formatRadios = document.querySelectorAll('input[name="format"]');
    formatRadios.forEach(radio => {
        radio.disabled = true;
        const label = radio.closest('label');
        if (label) {
            label.style.opacity = '0.3';
            label.style.cursor = 'not-allowed';
        }
    });
    
    // 🔥 禁用视频编码器选择（视频）
    const videoCodecRadios = document.querySelectorAll('input[name="videoCodec"]');
    videoCodecRadios.forEach(radio => {
        radio.disabled = true;
        const label = radio.closest('label');
        if (label) {
            label.style.opacity = '0.3';
            label.style.cursor = 'not-allowed';
        }
    });
    
    // 禁用转换按钮
    const convertBtn = document.getElementById('convertBtn');
    if (convertBtn) {
        convertBtn.disabled = true;
        convertBtn.style.opacity = '0.5';
    }
    
    // 🔥 禁用视频转换按钮
    const startVideoConversionBtn = document.getElementById('startVideoConversion');
    if (startVideoConversionBtn) {
        startVideoConversionBtn.disabled = true;
        startVideoConversionBtn.style.opacity = '0.5';
    }
}

/**
 * 🔥 启用手动模式所有控件
 */
function enableManualModeControls(enable = true) {
    log.info('[PIXLY Manual] ✅ Enabling all manual mode controls...');
    
    const controlIds = [
        // 图像转换控件
        'quality',
        'jxlEffort',
        'jxlDecodeSpeed',
        'jxlDistance',
        'jxlPhotonNoise',
        'webpMethod',
        'webpAutoFilter',
        // 视频转换控件
        'videoSpeed',
        'videoCRF',
        'videoContainer',
        'videoConversionAdvancedBtn',
        'enableHardwareAccel',
        'enableTwoPass',
        'webpSharpness',
        'avifSpeed',
        'avifTiles',
        'heicQuality',
        'manualLossless',
        'enableJpegLossless' // 🔥 Phase 45.5: 恢复enableJpegLossless
    ];
    
    controlIds.forEach(id => {
        const el = document.getElementById(id);
        if (el) {
            el.disabled = !enable;
            el.style.opacity = enable ? '1' : '0.3';
            el.style.cursor = enable ? 'pointer' : 'not-allowed';
        }
    });
    
    // 启用格式选择（图像）
    const formatRadios = document.querySelectorAll('input[name="format"]');
    formatRadios.forEach(radio => {
        radio.disabled = !enable;
        const label = radio.closest('label');
        if (label) {
            label.style.opacity = enable ? '1' : '0.3';
            label.style.cursor = enable ? 'pointer' : 'not-allowed';
        }
    });
    
    // 🔥 启用视频编码器选择（视频）
    const videoCodecRadios = document.querySelectorAll('input[name="videoCodec"]');
    videoCodecRadios.forEach(radio => {
        radio.disabled = !enable;
        const label = radio.closest('label');
        if (label) {
            label.style.opacity = enable ? '1' : '0.3';
            label.style.cursor = enable ? 'pointer' : 'not-allowed';
        }
    });
    
    // 启用转换按钮
    const convertBtn = document.getElementById('convertBtn');
    if (convertBtn) {
        convertBtn.disabled = !enable;
        convertBtn.style.opacity = enable ? '1' : '0.5';
    }
    
    // 🔥 启用视频转换按钮
    const startVideoConversionBtn = document.getElementById('startVideoConversion');
    if (startVideoConversionBtn) {
        startVideoConversionBtn.disabled = !enable;
        startVideoConversionBtn.style.opacity = enable ? '1' : '0.5';
    }
}

/**
 * 🔥 显示手动模式状态
 */
/**
 * ❌ 已禁用：与图标检测重复
 * 手动模式状态已通过图标显示（🤖 ✓/✗ 和 ⚙️ ✓/✗）
 */
function showManualModeStatus(message, type = 'info') {
    // 禁用：避免与图标检测重复
    return;
}

/**
 * 🔥 显示服务启动指南（仅首次，避免频繁报错）
 */
let serviceGuideShown = false;
function showServiceStartGuide() {
    // 避免频繁报错 - 每个会话只显示一次
    if (serviceGuideShown) {
        log.warn('[PIXLY Manual] Core services still not available (guide already shown)');
        return;
    }
    
    serviceGuideShown = true;
    
    const guide = `
╔════════════════════════════════════════════════════════╗
║  ⚠️  Core Services Not Available                      ║
╚════════════════════════════════════════════════════════╝

Manual mode requires at least one core service:

1️⃣  GO AI Service (recommended)
   Start: ./quick_start_go_core.sh
   Port: 50052

2️⃣  Rust Service (high performance)
   Start: ./start-rust-service.sh
   Port: 8080

Please start at least one service and try again.
    `.trim();
    
    log.error(guide);
    log.info('[PIXLY Manual] This guide will only show once per session to avoid spam');
}

/**
 * 处理模式切换事件
 * 来源: plugin.js 1301-1341lines
 */
function handleModeChange(e) {
    const mode = e.target.value;
    const formatSection = document.getElementById('formatSection');
    const projectInfoSection = document.getElementById('projectInfoSection');
    const smartModeToolsPanel = document.getElementById('smartModeToolsPanel');
    const manualOptions = document.getElementById('manualOptions');
    const smartOptions = document.getElementById('smartOptions');
    const manualAdvancedOptions = document.getElementById('manualAdvancedOptions');
    const smartDefaultBehavior = document.getElementById('smartDefaultBehavior');
    const aiOptionsSection = document.getElementById('aiOptionsSection'); // 🔥 AIoptions区域
    const manualAdvancedSection = document.getElementById('manualAdvancedSection'); // 🔧 手动模式高级optionssection
    
    // 隐藏all内容
    if (formatSection) formatSection.style.display = 'none';
    if (projectInfoSection) projectInfoSection.style.display = 'none';
    if (smartModeToolsPanel) smartModeToolsPanel.style.display = 'none';
    if (manualOptions) manualOptions.style.display = 'none';
    if (manualAdvancedOptions) manualAdvancedOptions.style.display = 'none';
    if (manualAdvancedSection) manualAdvancedSection.style.display = 'none'; // 🔧 隐藏手动模式高级options
    if (smartOptions) smartOptions.style.display = 'none';
    if (smartDefaultBehavior) smartDefaultBehavior.style.display = 'none';
    if (aiOptionsSection) aiOptionsSection.style.display = 'none'; // 🔥 默认隐藏AIoptions
    
    // 🎬 动图转视频选项（只在智能模式显示）
    const enableVideoForAnimationLabel = document.querySelector('label[for="enableVideoForAnimation"]') || 
                                         document.getElementById('enableVideoForAnimation')?.closest('label');
    
    if (mode === 'smart') {
        // 智能模式
        if (projectInfoSection) projectInfoSection.style.display = 'block';
        if (smartOptions) smartOptions.style.display = 'block';
        if (smartDefaultBehavior) smartDefaultBehavior.style.display = 'block';
        if (smartModeToolsPanel) smartModeToolsPanel.style.display = 'block'; // 显示快捷工具面板
        if (aiOptionsSection) aiOptionsSection.style.display = 'block'; // 🔥 智能模式显示AIoptions
        
        // 🎬 智能模式显示动图转视频选项
        if (enableVideoForAnimationLabel) {
            enableVideoForAnimationLabel.style.display = 'flex';
        }
        
        // 🔄 Phase 2: 智能模式自动获取AI推荐
        const aiBridge = window.PIXLY?.aiBridge;
        if (aiBridge && window.selectedFiles && window.selectedFiles.length > 0) {
            const firstFile = window.selectedFiles[0];
            const imageInfo = {
                width: firstFile.width || 1920,
                height: firstFile.height || 1080,
                size: firstFile.size || 0,
                format: firstFile.ext?.replace('.', '') || 'unknown',
                hasAlpha: firstFile.hasAlpha || false,
                isAnimated: firstFile.isAnimated || false
            };
            
            // 异步获取AI推荐并应用到UI
            aiBridge.getSmartConfig(imageInfo).then(config => {
                aiBridge.applyAISuggestionToUI(config);
                aiBridge.displayAISuggestion(config);
            }).catch(err => {
                console.warn('AI推荐失败:', err);
            });
        }
        
        addLog(window.i18n.t('messages.log.switchedToSmartMode'));
    } else if (mode === 'manual') {
        // 手动模式 - 也需要检测核心状态
        if (formatSection) formatSection.style.display = 'block';
        if (manualOptions) manualOptions.style.display = 'block';
        if (manualAdvancedOptions) manualAdvancedOptions.style.display = 'block';
        if (manualAdvancedSection) manualAdvancedSection.style.display = 'block'; // 🔧 显示手动模式高级options
        
        // 🎬 手动模式隐藏动图转视频选项
        if (enableVideoForAnimationLabel) {
            enableVideoForAnimationLabel.style.display = 'none';
        }
        
        addLog(window.i18n.t('messages.log.switchedToManualMode'));
        
        // 🔥 修复：立即启用控件（避免闪烁灰色状态）
        log.info('[PIXLY Manual] 💡 Pre-enabling controls before detection...');
        enableManualModeControls(true);
        
        // 🔥 手动模式也需要检测和启动核心（异步，不阻塞UI）
        checkAndEnableManualMode();
        
        // 🔥 Phase 45.8: 切换到手动模式时立即更新JPEG Notice（包括JXL标签）
        setTimeout(() => {
            updateManualParamsAvailability();
            updateJPEGNotice(); // 立即更新JXL标签文本
            log.info('[PIXLY Manual] 🔄 Triggered JPEG notice update on mode switch');
        }, 100);
    } else if (mode === 'tools') {
        // 实用工具模式
        if (smartModeToolsPanel) smartModeToolsPanel.style.display = 'block';
        
        // 🎬 工具模式隐藏动图转视频选项
        if (enableVideoForAnimationLabel) {
            enableVideoForAnimationLabel.style.display = 'none';
        }
        
        addLog(window.i18n.t('messages.log.switchedToToolsMode'));
    }
}


    // ============================================================
    // module导出
    // ============================================================
    
    const UIHandlers = {
        init: initializePlugin,
        handleModeChange,
        updateFormatSpecificParams,
        updateJPEGNotice,
        /**
         * 🔥 Phase 45.8.4: 修复AI核心（增强版）
         * 提供积极、智能的修复手段
         */
        fixAICore: async function() {
            log.info('[PIXLY] 🔧 Starting enhanced AI core fix...');
            
            // 🔥 修复：立即清除缓存，强制重新检测
            _goCoreCache = null;
            _goCoreDetecting = null;
            log.info('[PIXLY] 🗑️ Cleared GO core detection cache');
            
            if (window.addLog) {
                addLog('🔧 开始智能修复AI核心...', 'info');
            }
            
            if (window.PIXLY && window.PIXLY.Toast) {
                window.PIXLY.Toast.info('AI核心修复', '正在智能诊断并尝试修复...', 3000);
            }
            
            // 🔥 Phase 46.X: 立即检测服务是否已经在运行
            log.info('[PIXLY] 🔍 Pre-check: Detecting if service is already running...');
            try {
                const quickCheck = await detectGoCore();
                if (quickCheck.available) {
                    log.info(`[PIXLY] ✅ Service already running: ${quickCheck.version}`);
                    if (window.PIXLY && window.PIXLY.Toast) {
                        window.PIXLY.Toast.success('AI核心已在线', `✅ ${quickCheck.version} 运行正常`, 3000);
                    }
                    if (window.addLog) {
                        addLog(`✅ AI服务已在线: ${quickCheck.version}`, 'success');
                    }
                    // 刷新UI状态
                    await detectImageCoreStatus();
                    await detectVideoCoreStatus();
                    return; // 服务已运行，直接返回
                }
                log.info('[PIXLY] ℹ️  Service not detected, proceeding with fix...');
            } catch (e) {
                log.info('[PIXLY] ⚠️  Pre-check failed, proceeding with fix...', e.message);
            }
            
            try {
                // 🔥 Phase 46.1: 增强错误处理和日志
                let spawn, execSync, path, fs, net;
                
                try {
                    const childProcess = require('child_process');
                    spawn = childProcess.spawn;
                    execSync = childProcess.execSync;
                    path = require('path');
                    fs = require('fs');
                    net = require('net');
                } catch (e) {
                    throw new Error(`❌ 无法加载Node.js模块: ${e.message}\n浏览器环境限制，请手动启动AI服务`);
                }
                
                // 🔥 Phase 46.1: 使用path-resolver而不是eagle.package.path（更可靠）
                const pluginPath = eagle?.package?.path;
                let goServicePath;
                
                if (pluginPath) {
                    // 方法1: 使用eagle.package.path（如果可用）
                    log.info('[PIXLY] 📁 Using eagle.package.path:', pluginPath);
                    goServicePath = path.join(pluginPath, 'core', 'go');
                } else {
                    // 方法2: 使用path-resolver（fallback）
                    log.info('[PIXLY] ⚠️ eagle.package.path is empty, using path-resolver fallback');
                    if (window.PIXLY_PATH_RESOLVER?.coreRoots?.go) {
                        goServicePath = window.PIXLY_PATH_RESOLVER.coreRoots.go;
                        log.info('[PIXLY] 📁 Using path-resolver GO core:', goServicePath);
                    } else if (window.PIXLY_PATH_RESOLVER?.projectRoot) {
                        // 方法3: 从project root推断（最后手段）
                        goServicePath = path.join(window.PIXLY_PATH_RESOLVER.projectRoot, 'go');
                        log.info('[PIXLY] 📁 Using project root fallback:', goServicePath);
                    } else {
                        throw new Error('❌ 无法获取GO服务路径：\n1. eagle.package.path为空\n2. path-resolver不可用\n\n请手动启动AI服务');
                    }
                }
                
                const mainGoPath = path.join(goServicePath, 'cmd', 'pixly-ai', 'main.go');
                const startScriptPath = path.join(goServicePath, 'start_ai_service.sh');
                
                log.info('[PIXLY] 📁 GO service path:', goServicePath);
                log.info('[PIXLY] 📄 main.go path:', mainGoPath);
                
                // 🔍 步骤1: 检查Go是否安装
                log.info('[PIXLY] 🔍 Step 1: Checking Go installation...');
                let goInstalled = false;
                let goCommand = 'go';
                
                // 🔥 Phase 46.5.15: 尝试多个Go路径（解决Homebrew PATH问题）
                const goPaths = [
                    'go',                                    // 系统PATH
                    '/opt/homebrew/bin/go',                  // Apple Silicon Homebrew
                    '/usr/local/bin/go',                     // Intel Homebrew
                    '/usr/local/go/bin/go',                  // 官方安装
                ];
                
                for (const goPath of goPaths) {
                    try {
                        const env = { ...process.env };
                        // 添加Homebrew bin目录到PATH
                        env.PATH = `/opt/homebrew/bin:/usr/local/bin:${process.env.PATH}`;
                        
                        execSync(`${goPath} version`, { timeout: 3000, stdio: 'pipe', env });
                        goInstalled = true;
                        goCommand = goPath;
                        log.info(`[PIXLY] ✅ Go is installed at: ${goPath}`);
                        if (window.addLog) {
                            addLog(`✅ Go环境已安装: ${goPath}`, 'info');
                        }
                        break;
                    } catch (e) {
                        // 继续尝试下一个路径
                        continue;
                    }
                }
                
                if (!goInstalled) {
                    log.error('[PIXLY] ❌ Go not found in any common location');
                    
                    // 🔥 Phase 46.5.15: 友好的Go未安装提示
                    const goNotInstalledMsg = `
Go语言环境检测失败

💡 可能的原因：
1. Go未安装
2. Go已安装但PATH配置有问题

📋 解决方案：
1️⃣ 重新打开终端验证Go安装：
   • 运行: go version
   • 应该显示: go version go1.25.x

2️⃣ 如果显示版本，说明Go已安装
   • 问题：Eagle插件无法访问Go
   • 建议：使用"通用模式"（无需AI）

3️⃣ 使用通用模式（推荐）：
   • 切换到"通用模式"
   • 速度更快，规则引擎
   • JPEG/PNG无损转换
   • 无需AI服务
                    `.trim();
                    
                    throw new Error(goNotInstalledMsg);
                }
                
                // 🔍 步骤2: 检查AI服务文件是否存在
                log.info('[PIXLY] 🔍 Step 2: Checking AI service files...');
                if (!fs.existsSync(mainGoPath)) {
                    throw new Error(`找不到AI服务主文件: ${mainGoPath}`);
                }
                log.info('[PIXLY] ✅ AI service files found');
                if (window.addLog) {
                    addLog('✅ AI服务文件完整', 'info');
                }
                
                // 🔍 步骤3: 检查端口50052是否被占用
                log.info('[PIXLY] 🔍 Step 3: Checking port 50052...');
                const portInUse = await new Promise((resolve) => {
                    const socket = new net.Socket();
                    socket.setTimeout(1000);
                    socket.on('connect', () => {
                        socket.destroy();
                        resolve(true);
                    });
                    socket.on('timeout', () => {
                        socket.destroy();
                        resolve(false);
                    });
                    socket.on('error', () => {
                        resolve(false);
                    });
                    socket.connect(50052, 'localhost');
                });
                
                if (portInUse) {
                    log.info('[PIXLY] ℹ️  Port 50052 is already in use, service might be running');
                    if (window.addLog) {
                        addLog('ℹ️  端口50052已被占用，服务可能正在运行', 'info');
                        addLog('正在重新检测...', 'info');
                    }
                    
                    // 端口被占用，直接重新检测
                    const goResult = await detectGoCore();
                    if (goResult.available) {
                        if (window.PIXLY && window.PIXLY.Toast) {
                            window.PIXLY.Toast.success('AI核心已在线', `✅ ${goResult.version} 正常运行`, 3000);
                        }
                        if (window.addLog) {
                            addLog(`✅ AI核心检测成功: ${goResult.version}`, 'success');
                        }
                        
                        // 刷新UI
                        await detectImageCoreStatus();
                        await detectVideoCoreStatus();
                        return;
                    }
                }
                
                // 🚀 步骤4: 尝试启动AI服务
                log.info('[PIXLY] 🚀 Step 4: Starting AI service...');
                if (window.addLog) {
                    addLog('🚀 正在启动AI服务...', 'info');
                }
                
                let goProcess;
                
                // 🔥 Phase 46.5.16: 启动脚本有交互式提示，跳过直接使用go run
                // 启动脚本问题：
                // 1. 包含 read -p 交互式提示（后台执行时会卡住）
                // 2. go run前台运行（不会后台化）
                // 解决：直接使用go run + nohup后台化
                
                log.info(`[PIXLY] 🔧 Starting AI service with: ${goCommand}`);
                const env = { ...process.env };
                // 添加Homebrew bin目录到PATH
                env.PATH = `/opt/homebrew/bin:/usr/local/bin:${process.env.PATH}`;
                
                // 日志文件路径
                const logPath = path.join(goServicePath, 'ai-service.log');
                log.info(`[PIXLY] 📝 Logs will be written to: ${logPath}`);
                
                // 创建日志文件stream
                const logFile = fs.openSync(logPath, 'a');
                
                // 后台启动Go服务
                goProcess = spawn(goCommand, ['run', 'cmd/pixly-ai/main.go'], {
                    cwd: goServicePath,
                    detached: true,
                    stdio: ['ignore', logFile, logFile],  // 输出到日志文件
                    env: env
                });
                
                if (goProcess && goProcess.pid) {
                    log.info(`[PIXLY] ✅ AI service started with PID: ${goProcess.pid}`);
                    if (window.addLog) {
                        addLog(`✅ AI服务已启动 (PID: ${goProcess.pid})`, 'success');
                        addLog(`📝 日志文件: ${logPath}`, 'info');
                    }
                }
                
                goProcess.unref(); // 让子进程独立运行
                
                // ⏳ 步骤5: 等待服务启动并检测
                log.info('[PIXLY] ⏳ Step 5: Waiting for service to start...');
                if (window.addLog) {
                    addLog('⏳ 等待AI服务启动（首次启动需编译，约5-8秒）...', 'info');
                }
                
                // 🔥 Phase 46.5.16: 智能等待 - 每2秒检测一次，最多5次（10秒）
                log.info('[PIXLY] ✅ Step 6: Verifying service (intelligent retry)...');
                
                let goResult = { available: false };
                const maxRetries = 5;
                
                for (let i = 0; i < maxRetries; i++) {
                    await new Promise(resolve => setTimeout(resolve, 2000));
                    log.info(`[PIXLY] 🔍 Detection attempt ${i + 1}/${maxRetries}...`);
                    
                    goResult = await detectGoCore();
                    if (goResult.available) {
                        log.info(`[PIXLY] ✅ Service detected on attempt ${i + 1}`);
                        break;
                    }
                }
                
                if (goResult.available) {
                    // 🎉 成功！
                    log.info('[PIXLY] 🎉 AI service started successfully!');
                    if (window.PIXLY && window.PIXLY.Toast) {
                        window.PIXLY.Toast.success(
                            'AI核心修复成功！',
                            `✅ 版本 ${goResult.version} 已在端口 ${goResult.port} 启动`,
                            5000
                        );
                    }
                    if (window.addLog) {
                        addLog(`🎉 AI核心修复成功！`, 'success');
                        addLog(`✅ 版本: ${goResult.version}`, 'success');
                        addLog(`✅ 端口: ${goResult.port}`, 'success');
                    }
                    
                    // 刷新所有面板的状态显示
                    setTimeout(async () => {
                        await detectImageCoreStatus();
                        await detectVideoCoreStatus();
                        log.info('[PIXLY] ✅ UI status refreshed');
                    }, 500);
                } else {
                    // ❌ 启动失败
                    const errorMsg = `AI服务启动失败：10秒内无法检测到服务

可能的原因：
1. Go服务编译或启动出错
2. 端口50052被其他程序占用
3. 依赖包缺失

📝 请查看日志文件排查问题：
${logPath}

💡 手动启动命令：
cd ${goServicePath}
go run cmd/pixly-ai/main.go`;
                    
                    log.error('[PIXLY] ❌ Service start failed. Log file:', logPath);
                    throw new Error(errorMsg);
                }
            } catch (error) {
                log.error('[PIXLY] ❌ AI core fix failed:', error);
                
                // 🔥 Phase 46.1: 使用goServicePath变量（如果可用）或path-resolver
                let goPath = '/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/go';  // 固定路径作为fallback
                
                try {
                    if (typeof goServicePath !== 'undefined' && goServicePath) {
                        goPath = goServicePath;
                    } else if (window.PIXLY_PATH_RESOLVER?.coreRoots?.go) {
                        goPath = window.PIXLY_PATH_RESOLVER.coreRoots.go;
                    }
                } catch (e) {
                    log.warn('[PIXLY] ⚠️ Cannot get GO path for error message:', e);
                }
                
                if (window.PIXLY && window.PIXLY.Toast) {
                    window.PIXLY.Toast.error(
                        'AI核心修复失败',
                        `❌ ${error.message}\n\n💡 手动修复步骤：\n1. 打开终端\n2. cd ${goPath}\n3. go run cmd/pixly-ai/main.go`,
                        10000
                    );
                }
                
                if (window.addLog) {
                    addLog(`❌ AI核心修复失败: ${error.message}`, 'error');
                    addLog('', 'error');
                    addLog('💡 手动修复步骤：', 'info');
                    addLog('   1. 打开终端（Terminal.app）', 'info');
                    addLog(`   2. 执行: cd ${goPath}`, 'info');
                    addLog('   3. 执行: go run cmd/pixly-ai/main.go', 'info');
                    addLog('', 'info');
                    if (error.message.includes('Go未安装')) {
                        addLog('💡 安装Go: https://golang.org/dl/', 'info');
                    }
                }
            }
        },
        showProgress: function() {
            const progressSection = document.getElementById('progressSection');
            if (progressSection) progressSection.style.display = 'block';
        },
        hideProgress: function() {
            const progressSection = document.getElementById('progressSection');
            if (progressSection) progressSection.style.display = 'none';
        },
        // 🔥 updateProgress 已移至 01-globals.js，支持 sub-progress 和平滑动画
        setConvertButtonEnabled: function(enabled) {
            const convertBtn = document.getElementById('convertBtn');
            if (convertBtn) convertBtn.disabled = !enabled;
        },
        setCancelButtonVisible: function(visible) {
            const cancelBtn = document.getElementById('cancelBtn');
            if (cancelBtn) cancelBtn.style.display = visible ? 'inline-block' : 'none';
        }
    };
    
    PIXLY.UIHandlers = UIHandlers;
    
    // 向后兼容
    window.initializePlugin = initializePlugin;
    window.handleModeChange = handleModeChange;
    window.updateFormatSpecificParams = updateFormatSpecificParams;
    window.updateJPEGNotice = updateJPEGNotice;
    window.showProgress = UIHandlers.showProgress;
    window.hideProgress = UIHandlers.hideProgress;
    // 🔥 updateProgress 已在 01-globals.js 中定义，不再覆盖
    
    // 🔥 Phase 46.3: 格式修正功能已改为自动化（AI高级选项多选框）
    // 旧的手动格式修正按钮功能已移除，现在格式修正会在转换完成后自动执行
    // 参见：file-validator.js 的 applyFormatCorrection() 方法
    // 参见：image-conversion.js 中converted successfully后的自动调用
    
    // 🔧 一键修复功能
    window.runQuickFix = async function() {
        const btn = document.getElementById('quickFixBtn');
        if (!btn) return;
        
        const originalText = btn.innerHTML;
        btn.disabled = true;
        btn.innerHTML = '<div class="quick-tool-content"><div class="quick-tool-icon">⏳</div><div class="quick-tool-info"><div class="quick-tool-name">修复中...</div><div class="quick-tool-desc">请稍候</div></div></div>';
        
        // 使用Toastnotification（持续3秒）
        if (window.PIXLY && window.PIXLY.Toast) {
            window.PIXLY.Toast.info(i18n.t('tools.quickFix'), i18n.t('messages.toast.quickFixStart'), 3000);
        }
        
        if (window.addLog) {
            addLog(window.i18n.t('messages.log.quickFixStart'), 'info');
        }
        
        try {
            const { spawn } = require('child_process');
            const path = require('path');
            const fs = require('fs');
            
            let scriptOutput = '';  // 存储脚本输出，供后续使用
            
            // passed查找loadedscript标签获取Plugin root directory
            let pluginRoot = null;
            const scripts = document.querySelectorAll('script[src*="plugin-modules"]');
            if (scripts.length > 0) {
                // from第一 module脚本URL推断path
                // 例如: file:///.../plxy-easy2jxlavif/plugin_v3/js/plugin-modules/01-globals.js
                const scriptUrl = new URL(scripts[0].src);
                const scriptPath = scriptUrl.pathname;
                // 向上3级: /plugin-modules -> /js -> /plugin_v3 -> /plxy-easy2jxlavif (项目根)
                pluginRoot = path.resolve(path.dirname(scriptPath), '../../..');
            } else {
                // 备用：from当前页面URL推断
                const pageUrl = new URL(window.location.href);
                // file:///.../plxy-easy2jxlavif/plugin_v3/index.html
                // 向上1级: /plugin_v3 -> /plxy-easy2jxlavif (项目根)
                pluginRoot = path.resolve(path.dirname(pageUrl.pathname), '..');
            }
            
                        const fixScript = path.join(pluginRoot, 'tools', 'CLEANUP_DUPLICATE_FILES.js');
            
            // debuglog
log.info('[PIXLY UI] Plugin root directory:', pluginRoot);
log.info('[PIXLY UI] Fix script path:', fixScript);
log.info('[PIXLY UI] Script exists:', fs.existsSync(fixScript));
            
            if (window.addLog) {
                addLog(window.i18n.t('messages.log.cleaningDuplicates'), 'info');
            }
            
            // Check脚本is否exists
            if (!fs.existsSync(fixScript)) {
                throw new Error(i18n.t('messages.error.fixScriptNotFound', {path: fixScript}));
            }
            
            // 执lines清理脚本（使用完整nodepath）
            const nodePath = require('fs').existsSync('/opt/homebrew/bin/node') 
                ? '/opt/homebrew/bin/node'  // Apple Silicon
                : require('fs').existsSync('/usr/local/bin/node')
                ? '/usr/local/bin/node'      // Intel Mac
                : 'node';                     // 回退toPATH
            
            await new Promise((resolve, reject) => {
 log.info('[PIXLY UI] Starting to executefix script...');
 log.info('[PIXLY UI] nodepath:', nodePath);
 log.info('[PIXLY UI] working directory:', pluginRoot);
                
                const proc = spawn(nodePath, [fixScript], {
                    cwd: pluginRoot
                });
                
                let stdout = '';
                let stderr = '';
                
                proc.stdout.on('data', (data) => {
                    const text = data.toString();
                    stdout += text;
 log.info('[PIXLY UI] stdout:', text);
                });
                
                proc.stderr.on('data', (data) => {
                    const text = data.toString();
                    stderr += text;
 log.info('[PIXLY UI] stderr:', text);
                });
                
                proc.on('close', (code) => {
 log.info('[PIXLY UI] Process ended，exit code:', code);
 log.info('[PIXLY UI] stdouttotal length:', stdout.length);
 log.info('[PIXLY UI] stderrtotal length:', stderr.length);
                    
                    // save输出to外层变量
                    scriptOutput = stdout;
                    
                    if (code === 0) {
                        if (window.addLog) {
                            // 显示脚本输出
                            if (stdout.trim()) {
                                const lines = stdout.split('\n').filter(line => line.trim());
 log.info('[PIXLY UI] Preparing to display', lines.length, 'lines of output');
                                lines.forEach(line => {
                                    addLog(`  ${line}`, 'info');
                                });
                            } else {
 log.info('[PIXLY UI] stdoutis empty，no content to display');
                            }
                            addLog(window.i18n.t('messages.log.cleanupComplete'), 'success');
                        } else {
 log.info('[PIXLY UI] ⚠️ window.addLog not available');
                        }
                        
                        resolve();
                    } else {
                        const errorMsg = stderr || stdout || `fix scriptexit code: ${code}`;
 log.info('[PIXLY UI] Script executionfailed:', errorMsg);
                        reject(new Error(errorMsg));
                    }
                });
                
                proc.on('error', (err) => {
 log.info('[PIXLY UI] Process error:', err);
                    reject(new Error(`无法执linesfix script: ${err.message}`));
                });
            });
            
            if (window.addLog) {
                addLog(window.i18n.t('messages.log.quickFixComplete'), 'success');
                addLog(window.i18n.t('messages.log.refreshEagleHint'), 'info');
            }
            
            // 使用Toastnotification显示successfully（持续5秒）
            if (window.PIXLY && window.PIXLY.Toast) {
                const summary = scriptOutput.match(/Found (\d+)  /);
                const message = summary ? i18n.t('messages.toast.quickFixScanned', {count: summary[1]}) : i18n.t('messages.toast.quickFixCleaned');
                window.PIXLY.Toast.success(i18n.t('messages.toast.quickFixCompleteTitle'), `✅ ${message}`, 5000);
            }
            
        } catch (error) {
 log.error('[PIXLY UI] Quick fix failed:', error);
            if (window.addLog) {
                addLog(window.i18n.t('messages.log.quickFixFailed', {error: error.message}), 'error');
            }
            
            // 使用Toastnotification显示error（持续5秒）
            if (window.PIXLY && window.PIXLY.Toast) {
                window.PIXLY.Toast.error(i18n.t('messages.toast.quickFixFailedTitle'), `❌ ${error.message}`, 5000);
            }
        } finally {
            btn.disabled = false;
            btn.innerHTML = originalText;
        }
    };
    
    // ========================================================================
    // 🆕 v5.1.0: 精度模式自动切换 + 自定义预期格式
    // ========================================================================
    
    /**
     * 更新自定义预期格式显示/隐藏
     * 规则: 格式智能选择关闭时显示
     */
    function updateExpectedFormatVisibility() {
        const smartFormatCheckbox = document.getElementById('enableSmartFormat');
        const expectedFormatContainer = document.getElementById('expectedFormatContainer');
        
        if (smartFormatCheckbox && expectedFormatContainer) {
            // 当格式智能选择被取消勾选时，显示自定义预期格式
            if (!smartFormatCheckbox.checked) {
                expectedFormatContainer.style.display = 'block';
            } else {
                expectedFormatContainer.style.display = 'none';
            }
        }
    }
    
    // 绑定所有AI功能checkbox的change事件
    const aiCheckboxIds = [
        'enableSmartQuality',
        'enableAutoOptimize',
        'enableSSIMValidation',
        'enableSmartFormat'
    ];
    
    aiCheckboxIds.forEach(id => {
        const checkbox = document.getElementById(id);
        if (checkbox) {
            checkbox.addEventListener('change', () => {
                updateExpectedFormatVisibility();
            });
        }
    });
    
    // 初始化时更新一次
    setTimeout(() => {
        updateExpectedFormatVisibility();
    }, 100);
    
     log.info('[PIXLY UI] ✅ Precision mode auto-switching initialized');
    
    // 导出范围收束过滤函数供其他模块使用
    window.checkScopeFilter = checkScopeFilter;
    
 log.info('[PIXLY UI] UI Event Handler module loaded (Full Implementation, ~650 lines)');                                                                    
    
    
})(window);
// Cache buster: 1762740676
