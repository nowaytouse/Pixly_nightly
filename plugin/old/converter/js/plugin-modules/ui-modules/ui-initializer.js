/**
 * UI Initializer - UI初始化模块
 * 负责初始化插件的所有UI组件和基础事件绑定
 * 从ui-handlers.js拆分 (~800行)
 */

export class UIInitializer {
    constructor() {
        this.log = window.pixlyLog || console;
        this.initialized = false;
    }

    /**
     * 主初始化方法
     */
    async initialize() {
        if (this.initialized) {
            this.log.warn('UIInitializer already initialized');
            return;
        }

        try {
            // 初始化各个组件
            this.initTypeTabs();
            this.initLogLevel();
            this.initLanguageSelector();
            this.initHelpModal();
            this.initVideoAI();
            this.initVideoPresets();
            this.initImagePresets();
            this.initModeTabs();
            this.initQualitySliders();
            this.initFormatParameters();
            this.initWorkerThreads();
            this.initConvertButton();
            
            // 标记已初始化
            this.initialized = true;
            this.log.info('[UIInitializer] ✅ All UI components initialized');
        } catch (error) {
            this.log.error('[UIInitializer] ❌ Initialization failed:', error);
            throw error;
        }
    }

    /**
     * 初始化类型标签（图像/视频）
     */
    initTypeTabs() {
        const typeTabs = document.querySelectorAll('.type-tab');
        
        typeTabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const type = tab.getAttribute('data-type');
                
                // 更新标签状态
                typeTabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                // 切换面板
                const imagePanel = document.getElementById('imageConversionPanel');
                const videoPanel = document.getElementById('videoPanel');
                
                if (type === 'image') {
                    if (imagePanel) imagePanel.style.display = 'block';
                    if (videoPanel) videoPanel.style.display = 'none';
                    this.log.info('[UIInitializer] Switched to image mode');
                } else if (type === 'animated' || type === 'video') {
                    if (imagePanel) imagePanel.style.display = 'none';
                    if (videoPanel) {
                        videoPanel.style.display = 'block';
                        this.updateVideoInputType(type);
                    }
                    this.log.info(`[UIInitializer] Switched to ${type} mode`);
                }
                
                // 触发文件列表刷新
                this.refreshFileList();
            });
        });

        this.log.debug('[UIInitializer] Type tabs initialized');
    }

    /**
     * 更新视频输入类型
     */
    updateVideoInputType(type) {
        const animatedRadio = document.querySelector('input[name="videoInputType"][value="animated"]');
        const videoRadio = document.querySelector('input[name="videoInputType"][value="video"]');
        
        if (type === 'animated' && animatedRadio) {
            animatedRadio.checked = true;
            animatedRadio.dispatchEvent(new Event('change', { bubbles: true }));
        } else if (type === 'video' && videoRadio) {
            videoRadio.checked = true;
            videoRadio.dispatchEvent(new Event('change', { bubbles: true }));
        }
    }

    /**
     * 刷新文件列表
     */
    refreshFileList() {
        setTimeout(() => {
            if (window.selectFiles) {
                window.selectFiles();
                this.log.debug('[UIInitializer] File list refreshed');
            }
        }, 100);
    }

    /**
     * 初始化日志级别选择器
     */
    initLogLevel() {
        const logLevelSelect = document.getElementById('logLevelSelect');
        if (!logLevelSelect) return;

        // 设置初始值
        if (window.pixlyLog) {
            logLevelSelect.value = window.pixlyLog.getLevel().toString();
        }
        
        // 绑定变更事件
        logLevelSelect.addEventListener('change', (e) => {
            const level = parseInt(e.target.value);
            if (window.pixlyLog) {
                window.pixlyLog.setLevel(level);
                const levelNames = ['ERROR', 'WARN', 'INFO', 'DEBUG', 'TRACE'];
                this.log.info(`[UIInitializer] Log level changed to ${levelNames[level]}`);
            }
        });

        this.log.debug('[UIInitializer] Log level selector initialized');
    }

    /**
     * 初始化语言选择器
     */
    initLanguageSelector() {
        const languageSelect = document.getElementById('languageSelect');
        if (!languageSelect) {
            this.log.warn('[UIInitializer] Language selector not found');
            return;
        }

        languageSelect.addEventListener('change', async function() {
            const selectedLang = this.value;
            
            if (window.i18n && window.i18n.switchLanguage) {
                await window.i18n.switchLanguage(selectedLang);
            } else if (window.PIXLY && window.PIXLY.I18n) {
                window.PIXLY.I18n.setLanguage(selectedLang);
            } else {
                console.warn('[UIInitializer] i18n system not found');
            }
            
            console.info(`[UIInitializer] Language switched to ${selectedLang}`);
        });

        this.log.debug('[UIInitializer] Language selector initialized');
    }

    /**
     * 初始化帮助模态窗口
     */
    initHelpModal() {
        const headerFeatures = document.getElementById('headerFeatures');
        const helpModal = document.getElementById('helpModal');
        
        if (headerFeatures) {
            headerFeatures.addEventListener('click', () => {
                if (helpModal) {
                    helpModal.style.display = 'flex';
                    this.log.info('[UIInitializer] Help modal opened');
                }
            });
        }
        
        // 关闭模态窗口
        document.addEventListener('click', (e) => {
            if (helpModal && helpModal.style.display === 'flex') {
                if (e.target === helpModal) {
                    helpModal.style.display = 'none';
                    this.log.debug('[UIInitializer] Help modal closed');
                }
            }
        });
        
        // 延迟绑定关闭按钮
        setTimeout(() => {
            const closeHelpBtn = document.getElementById('closeHelpBtn');
            if (closeHelpBtn) {
                closeHelpBtn.addEventListener('click', () => {
                    if (helpModal) {
                        helpModal.style.display = 'none';
                    }
                });
            }
        }, 500);

        this.log.debug('[UIInitializer] Help modal initialized');
    }

    /**
     * 初始化视频AI控件
     */
    initVideoAI() {
        // 视频AI开关（智能模式下默认启用）
        const enableVideoAI = document.getElementById('enableVideoAI');
        if (enableVideoAI) {
            enableVideoAI.checked = true;
            this.log.debug('[UIInitializer] Video AI enabled by default');
        }
        
        // 高级功能开关
        const advancedFeatures = document.getElementById('enableAdvancedFeatures');
        const forceTransformer = document.getElementById('forceTransformer');
        const enableVMAF = document.getElementById('enableVMAF');
        
        if (advancedFeatures) {
            advancedFeatures.addEventListener('change', function() {
                const status = this.checked ? 'ON' : 'OFF';
                console.debug(`[UIInitializer] Advanced features ${status}`);
            });
        }
        
        if (forceTransformer) {
            forceTransformer.addEventListener('change', function() {
                const status = this.checked ? 'ON' : 'OFF';
                console.debug(`[UIInitializer] Force transformer ${status}`);
            });
        }
        
        if (enableVMAF) {
            enableVMAF.addEventListener('change', function() {
                const status = this.checked ? 'ON' : 'OFF';
                console.debug(`[UIInitializer] VMAF validation ${status}`);
            });
        }

        this.log.debug('[UIInitializer] Video AI controls initialized');
    }

    /**
     * 初始化视频预设
     */
    initVideoPresets() {
        const videoPresetLabels = document.querySelectorAll('[data-preset]');
        const videoPresetDesc = document.getElementById('videoAIPresetDesc');
        
        if (videoPresetLabels.length === 0) return;
        
        // 初始化样式
        this.updateVideoPresetStyles(videoPresetLabels);
        
        // 绑定点击事件
        videoPresetLabels.forEach(label => {
            label.addEventListener('click', () => {
                const preset = label.dataset.preset;
                
                // 更新样式
                videoPresetLabels.forEach(l => l.classList.remove('active'));
                label.classList.add('active');
                
                // 更新描述
                this.updateVideoPresetDescription(preset, videoPresetDesc);
                
                // 更新说明卡片
                if (window.updateVideoOptimizeModeInfoCard) {
                    window.updateVideoOptimizeModeInfoCard(preset);
                }
                
                // 应用预设配置
                this.applyVideoPreset(preset);
                
                this.log.info(`[UIInitializer] Video preset changed to ${preset}`);
            });
        });
        
        // 初始化说明卡片
        const checkedVideoPreset = document.querySelector('input[name="videoAIPreset"]:checked');
        if (checkedVideoPreset && window.updateVideoOptimizeModeInfoCard) {
            window.updateVideoOptimizeModeInfoCard(checkedVideoPreset.value);
        }

        this.log.debug('[UIInitializer] Video presets initialized');
    }

    /**
     * 更新视频预设样式
     */
    updateVideoPresetStyles(labels) {
        labels.forEach(label => {
            const radio = label.querySelector('input[type="radio"]');
            if (radio && radio.checked) {
                label.classList.add('active');
            } else {
                label.classList.remove('active');
            }
        });
    }

    /**
     * 更新视频预设描述
     */
    updateVideoPresetDescription(preset, descElement) {
        if (!descElement) return;
        
        const i18n = window.i18n || { t: (key) => key };
        const descriptions = {
            fast: i18n.t('videoAI.fastDesc'),
            balanced: i18n.t('videoAI.balancedDesc'),
            quality: i18n.t('videoAI.fullDesc')
        };
        
        descElement.textContent = descriptions[preset] || descriptions.balanced;
    }

    /**
     * 应用视频预设配置
     */
    applyVideoPreset(preset) {
        const advancedFeatures = document.getElementById('enableAdvancedFeatures');
        const forceTransformer = document.getElementById('forceTransformer');
        const enableVMAF = document.getElementById('enableVMAF');
        
        switch(preset) {
            case 'fast':
                if (advancedFeatures) advancedFeatures.checked = false;
                if (forceTransformer) forceTransformer.checked = false;
                if (enableVMAF) enableVMAF.checked = false;
                break;
            case 'balanced':
                if (advancedFeatures) advancedFeatures.checked = true;
                if (forceTransformer) forceTransformer.checked = false;
                if (enableVMAF) enableVMAF.checked = false;
                break;
            case 'quality':
                if (advancedFeatures) advancedFeatures.checked = true;
                if (forceTransformer) forceTransformer.checked = true;
                if (enableVMAF) enableVMAF.checked = false;
                break;
        }
    }

    /**
     * 初始化图像预设
     */
    initImagePresets() {
        const imagePresetLabels = document.querySelectorAll('input[name="optimizeMode"]');
        const imagePresetDesc = document.getElementById('optimizeModeDesc');
        
        if (imagePresetLabels.length === 0) return;
        
        // 初始化样式
        this.updateImagePresetStyles(imagePresetLabels);
        
        // 监听变化
        imagePresetLabels.forEach(radio => {
            radio.addEventListener('change', function() {
                // 更新样式
                imagePresetLabels.forEach(r => {
                    const l = r.closest('.optimize-mode-btn');
                    if (l) l.classList.remove('active');
                });
                
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
                
                console.info(`[UIInitializer] Image preset changed to ${this.value}`);
                
                // 检测核心状态
                if (window.detectImageCoreStatus) {
                    window.detectImageCoreStatus();
                }
                
                // 更新说明卡片
                if (window.updateOptimizeModeInfoCard) {
                    window.updateOptimizeModeInfoCard(this.value);
                }
            });
        });
        
        // 初始化说明卡片
        const checkedPreset = document.querySelector('input[name="optimizeMode"]:checked');
        if (checkedPreset && window.updateOptimizeModeInfoCard) {
            window.updateOptimizeModeInfoCard(checkedPreset.value);
        }

        this.log.debug('[UIInitializer] Image presets initialized');
    }

    /**
     * 更新图像预设样式
     */
    updateImagePresetStyles(radios) {
        radios.forEach(radio => {
            const label = radio.closest('.optimize-mode-btn');
            if (label) {
                if (radio.checked) {
                    label.classList.add('active');
                } else {
                    label.classList.remove('active');
                }
            }
        });
    }

    /**
     * 初始化模式标签（智能/手动）
     */
    initModeTabs() {
        // 图像模式标签
        const modeTabs = document.querySelectorAll('#imageConversionPanel .mode-tab');
        modeTabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const mode = tab.getAttribute('data-mode');
                
                // 更新标签状态
                modeTabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                // 更新radio状态
                if (mode === 'smart' || mode === 'manual') {
                    const radioId = mode === 'smart' ? 'modeRadioSmart' : 'modeRadioManual';
                    const radio = document.getElementById(radioId);
                    if (radio) radio.checked = true;
                }
                
                // 触发模式切换
                if (window.handleModeChange) {
                    window.handleModeChange({ target: { value: mode } });
                }
            });
        });
        
        // 视频模式标签
        const videoModeTabs = document.querySelectorAll('#videoPanel .mode-tab');
        this.initVideoModeTabs(videoModeTabs);

        this.log.debug('[UIInitializer] Mode tabs initialized');
    }

    /**
     * 初始化视频模式标签
     */
    initVideoModeTabs(tabs) {
        if (tabs.length === 0) return;
        
        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const mode = tab.dataset.mode;
                
                // 更新标签样式
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                // 更新UI显示
                this.updateVideoModeUI(mode);
                
                this.log.info(`[UIInitializer] Video mode changed to ${mode}`);
            });
        });
    }

    /**
     * 更新视频模式UI
     */
    updateVideoModeUI(mode) {
        const videoAIOptionsSection = document.getElementById('videoAIOptionsSection');
        const videoManualOptions = document.getElementById('videoManualOptions');
        const videoModeRadioSmart = document.getElementById('videoModeRadioSmart');
        const videoModeRadioManual = document.getElementById('videoModeRadioManual');
        const videoGoCoreInline = document.getElementById('videoGoCoreInline');
        
        if (mode === 'smart-video') {
            if (videoModeRadioSmart) videoModeRadioSmart.checked = true;
            if (videoModeRadioManual) videoModeRadioManual.checked = false;
            if (videoAIOptionsSection) videoAIOptionsSection.style.display = 'block';
            if (videoManualOptions) videoManualOptions.style.display = 'none';
            if (videoGoCoreInline) videoGoCoreInline.style.display = 'block';
            
            // 检测核心状态
            if (window.detectVideoCoreStatus) {
                setTimeout(() => window.detectVideoCoreStatus(), 100);
            }
        } else {
            if (videoModeRadioManual) videoModeRadioManual.checked = true;
            if (videoModeRadioSmart) videoModeRadioSmart.checked = false;
            if (videoAIOptionsSection) videoAIOptionsSection.style.display = 'none';
            if (videoManualOptions) videoManualOptions.style.display = 'block';
            if (videoGoCoreInline) videoGoCoreInline.style.display = 'none';
        }
    }

    /**
     * 初始化质量滑块
     */
    initQualitySliders() {
        const qualitySlider = document.getElementById('quality');
        const qualityValue = document.getElementById('qualityValue');
        const qualityHint = document.getElementById('qualityHint');
        
        if (qualitySlider) {
            qualitySlider.addEventListener('input', (e) => {
                const value = parseInt(e.target.value);
                if (qualityValue) qualityValue.textContent = value;
                
                // 更新质量提示
                if (qualityHint && window.i18n) {
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

        this.log.debug('[UIInitializer] Quality sliders initialized');
    }

    /**
     * 初始化格式参数控件
     */
    initFormatParameters() {
        // JXL参数
        this.initSlider('jxlEffort', 'jxlEffortValue');
        this.initSlider('jxlDistance', 'jxlDistanceValue', (v) => parseFloat(v).toFixed(1));
        this.initSlider('jxlPatches', 'jxlPatchesValue');
        
        // WebP参数
        this.initSlider('webpMethod', 'webpMethodValue');
        this.initSlider('webpFilterStrength', 'webpFilterStrengthValue');
        this.initSlider('webpSharpness', 'webpSharpnessValue');
        this.initSlider('webpSegments', 'webpSegmentsValue');
        this.initSlider('webpSnsStrength', 'webpSnsStrengthValue');
        this.initSlider('webpPass', 'webpPassValue');
        
        // AVIF参数
        this.initSlider('avifSpeed', 'avifSpeedValue');
        this.initSlider('avifMinQuantizer', 'avifMinQuantizerValue');
        this.initSlider('avifMaxQuantizer', 'avifMaxQuantizerValue');
        this.initAvifTiles();
        
        // HEIC参数
        this.initSlider('heicQuality', 'heicQualityValue');

        this.log.debug('[UIInitializer] Format parameters initialized');
    }

    /**
     * 初始化滑块控件
     */
    initSlider(sliderId, valueId, formatter) {
        const slider = document.getElementById(sliderId);
        const value = document.getElementById(valueId);
        
        if (slider && value) {
            slider.addEventListener('input', (e) => {
                value.textContent = formatter ? formatter(e.target.value) : e.target.value;
            });
        }
    }

    /**
     * 初始化AVIF瓦片设置
     */
    initAvifTiles() {
        const rowsEl = document.getElementById('avifTilesRows');
        const colsEl = document.getElementById('avifTilesCols');
        const valueEl = document.getElementById('avifTilesValue');
        
        if (rowsEl && valueEl) {
            rowsEl.addEventListener('input', (e) => {
                const rows = e.target.value;
                const cols = colsEl?.value || '1';
                valueEl.textContent = `${rows}×${cols}`;
            });
        }
        
        if (colsEl && valueEl) {
            colsEl.addEventListener('input', (e) => {
                const rows = rowsEl?.value || '1';
                const cols = e.target.value;
                valueEl.textContent = `${rows}×${cols}`;
            });
        }
    }

    /**
     * 初始化工作线程设置
     */
    initWorkerThreads() {
        const workersSlider = document.getElementById('workers');
        
        if (workersSlider && workersSlider.hasAttribute('data-auto-max')) {
            try {
                const os = require('os');
                const cpuCount = os.cpus().length;
                const maxWorkers = Math.min(cpuCount, 16);
                workersSlider.max = maxWorkers;
                this.log.debug(`[UIInitializer] CPU cores: ${cpuCount}, max workers: ${maxWorkers}`);
            } catch (error) {
                this.log.warn('[UIInitializer] Failed to detect CPU cores, using default');
            }
        }
    }

    /**
     * 初始化转换按钮
     */
    initConvertButton() {
        const convertBtn = document.getElementById('convertBtn');
        const convertDropdown = document.getElementById('convertDropdown');
        const convertBtnContainer = document.getElementById('convertBtnContainer');
        
        if (convertBtn && convertDropdown) {
            // 点击右侧展开菜单
            convertBtn.addEventListener('click', (e) => {
                const rect = convertBtn.getBoundingClientRect();
                const clickX = e.clientX - rect.left;
                const buttonWidth = rect.width;
                
                // 右侧30px触发下拉菜单
                if (clickX > buttonWidth - 30) {
                    e.stopPropagation();
                    convertDropdown.classList.toggle('show');
                    this.log.debug('[UIInitializer] Dropdown toggled');
                }
            });
            
            // 点击其他地方关闭菜单
            document.addEventListener('click', (e) => {
                if (convertBtnContainer && !convertBtnContainer.contains(e.target)) {
                    convertDropdown.classList.remove('show');
                }
            });
        }

        // 智能检测JPEG文件
        if (window.updateConvertButtonMode) {
            window.updateConvertButtonMode();
        }

        this.log.debug('[UIInitializer] Convert button initialized');
    }
}

// 创建单例并导出
const initializer = new UIInitializer();
export default initializer;
