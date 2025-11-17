/**
 * UI Handlers - 模块化版本
 * 将原4440行的ui-handlers.js重构为多个职责单一的模块
 * 
 * 重构前: ui-handlers.js (4440行单文件)
 * 重构后: 9个专门模块，每个负责特定功能
 * 
 * @version 2.0.0
 * @date 2025-11-10
 */

(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    const log = window.pixlyLog || console;
    
    /**
     * 动态导入模块
     */
    async function loadModules() {
        try {
            log.info('[UIHandlers] Loading modular components...');
            
            // 基础路径
            const basePath = './ui-modules';
            
            // 导入所有模块
            const modules = await Promise.all([
                import(`${basePath}/ui-initializer.js`),
                import(`${basePath}/event-manager.js`),
                import(`${basePath}/mode-manager.js`),
                import(`${basePath}/scope-filter-manager.js`)
            ]);
            
            // 解构模块
            const [
                { default: initializer },
                { default: eventManager },
                { default: modeManager },
                { default: scopeFilterManager }
            ] = modules;
            
            // 挂载到全局对象
            window.PIXLY.UIInitializer = initializer;
            window.PIXLY.EventManager = eventManager;
            window.PIXLY.ModeManager = modeManager;
            window.PIXLY.ScopeFilterManager = scopeFilterManager;
            
            // 全局引用（兼容旧代码）
            window.eventManager = eventManager;
            window.modeManager = modeManager;
            window.scopeFilterManager = scopeFilterManager;
            
            log.info('[UIHandlers] ✅ All modules loaded');
            
            return { initializer, eventManager, modeManager, scopeFilterManager };
            
        } catch (error) {
            log.error('[UIHandlers] ❌ Failed to load modules:', error);
            throw error;
        }
    }
    
    /**
     * 初始化所有UI组件和事件
     */
    async function initializeUI() {
        try {
            // 加载模块
            const { initializer, eventManager, modeManager, scopeFilterManager } = await loadModules();
            
            // 初始化各模块
            await initializer.initialize();
            eventManager.initialize();
            modeManager.initialize();
            scopeFilterManager.initialize();
            
            // 设置全局函数引用（保持向后兼容）
            setupGlobalFunctions();
            
            // 绑定额外的事件处理
            bindLegacyEvents();
            
            log.info('[UIHandlers] ✅ UI initialization complete');
            
        } catch (error) {
            log.error('[UIHandlers] ❌ UI initialization failed:', error);
        }
    }
    
    /**
     * 设置全局函数（向后兼容）
     */
    function setupGlobalFunctions() {
        // 模式切换
        window.handleModeChange = function(e) {
            const mode = e.target ? e.target.value : e;
            if (window.modeManager) {
                window.modeManager.handleModeChange(mode);
            }
        };
        
        // 更新JPEG提示
        window.updateJPEGNotice = function() {
            const jpegNotice = document.getElementById('jpegLosslessNotice');
            const manualLosslessLabel = document.getElementById('manualLosslessLabel');
            if (!jpegNotice) return;
            
            const selectedFormat = document.querySelector('input[name="format"]:checked');
            const isJXL = selectedFormat && selectedFormat.value === 'jxl';
            const enableJpegLossless = document.getElementById('enableJpegLossless');
            const isSmartMode = document.getElementById('modeRadioSmart')?.checked;
            
            if (isJXL && isSmartMode && enableJpegLossless?.checked) {
                jpegNotice.style.display = 'block';
                if (manualLosslessLabel) manualLosslessLabel.style.display = 'none';
            } else {
                jpegNotice.style.display = 'none';
                if (manualLosslessLabel) manualLosslessLabel.style.display = 'inline-block';
            }
        };
        
        // 更新格式参数
        window.updateFormatSpecificParams = function() {
            const selectedFormat = document.querySelector('input[name="format"]:checked');
            if (!selectedFormat) return;
            
            const format = selectedFormat.value;
            const allParams = document.querySelectorAll('.format-specific-params');
            
            allParams.forEach(param => {
                param.style.display = 'none';
            });
            
            const targetParam = document.getElementById(`${format}Params`);
            if (targetParam) {
                targetParam.style.display = 'block';
            }
        };
        
        // 更新手动参数可用性
        window.updateManualParamsAvailability = function() {
            const manualLosslessEl = document.getElementById('manualLossless');
            const enableJpegLosslessEl = document.getElementById('enableJpegLossless');
            const qualitySlider = document.getElementById('quality');
            const qualityGroup = document.getElementById('qualityGroup');
            
            const isLossless = manualLosslessEl?.checked || 
                             enableJpegLosslessEl?.checked;
            
            if (qualitySlider) {
                qualitySlider.disabled = isLossless;
            }
            
            if (qualityGroup) {
                if (isLossless) {
                    qualityGroup.classList.add('disabled');
                } else {
                    qualityGroup.classList.remove('disabled');
                }
            }
        };
        
        // 更新转换按钮模式
        window.updateConvertButtonMode = function() {
            const convertBtn = document.getElementById('convertBtn');
            const convertBtnIcon = document.getElementById('convertBtnIcon');
            const convertBtnText = document.getElementById('convertBtnText');
            const convertNormalBtn = document.getElementById('convertNormal');
            const convertJXLBtn = document.getElementById('convertJXL');
            
            if (!convertBtn || !convertBtnIcon || !convertBtnText) return;
            
            const files = window.PIXLY_SELECTED_FILES || [];
            const jpegExtensions = ['.jpg', '.jpeg', '.jpe', '.jfif', '.jfi'];
            
            const hasJPEG = files.some(file => {
                let ext = file.ext || '';
                if (!ext.startsWith('.')) ext = '.' + ext;
                return jpegExtensions.includes(ext.toLowerCase());
            });
            
            if (hasJPEG) {
                // JXL模式
                convertBtnIcon.textContent = '✨';
                convertBtnText.setAttribute('data-i18n', 'quickAction.jpeg2jxl');
                convertBtnText.textContent = window.i18n ? 
                    window.i18n.t('quickAction.jpeg2jxl') : 'JPEG→JXL无损';
                convertBtn.classList.add('jxl-mode');
                
                if (convertNormalBtn) convertNormalBtn.style.display = 'flex';
                if (convertJXLBtn) convertJXLBtn.style.display = 'none';
            } else {
                // 普通模式
                convertBtnIcon.textContent = '🚀';
                convertBtnText.setAttribute('data-i18n', 'conversion.start');
                convertBtnText.textContent = window.i18n ? 
                    window.i18n.t('conversion.start') : '开始转换';
                convertBtn.classList.remove('jxl-mode');
                
                if (convertNormalBtn) convertNormalBtn.style.display = 'none';
                if (convertJXLBtn) convertJXLBtn.style.display = 'flex';
            }
        };
        
        // 更新优化模式信息卡片
        window.updateOptimizeModeInfoCard = function(mode) {
            const card = document.getElementById('optimizeModeInfoCard');
            const icon = document.getElementById('optimizeModeIcon');
            const title = document.getElementById('optimizeModeTitle');
            const desc = document.getElementById('optimizeModeDesc');
            
            if (!card || !icon || !title || !desc) return;
            
            const modeInfo = {
                size: {
                    icon: '📦',
                    title: '体积优先',
                    desc: '最大压缩率，适合批量处理和网络传输'
                },
                balanced: {
                    icon: '⚖️',
                    title: '智能平衡',
                    desc: 'AI自动平衡质量与体积，适合日常使用'
                },
                quality: {
                    icon: '✨',
                    title: '质量优先',
                    desc: '保持最佳视觉质量，适合重要图像'
                },
                general: {
                    icon: '🔧',
                    title: '通用模式',
                    desc: '兼容性优先，适合各种场景'
                }
            };
            
            const info = modeInfo[mode] || modeInfo.balanced;
            icon.textContent = info.icon;
            title.textContent = info.title;
            desc.textContent = info.desc;
            
            card.className = `optimize-mode-info-card ${mode}-mode`;
        };
        
        // 更新视频优化模式信息卡片
        window.updateVideoOptimizeModeInfoCard = function(preset) {
            const card = document.getElementById('videoOptimizeModeInfoCard');
            const icon = document.getElementById('videoOptimizeModeIcon');
            const title = document.getElementById('videoOptimizeModeTitle');
            const desc = document.getElementById('videoOptimizeModeDesc');
            
            if (!card || !icon || !title || !desc) return;
            
            const presetInfo = {
                fast: {
                    icon: '⚡',
                    title: '快速模式',
                    desc: '速度优先，适合快速处理'
                },
                balanced: {
                    icon: '⚖️',
                    title: '平衡模式',
                    desc: 'AI智能优化，平衡质量与速度'
                },
                quality: {
                    icon: '🎬',
                    title: '质量模式',
                    desc: '质量优先，深度优化'
                }
            };
            
            const info = presetInfo[preset] || presetInfo.balanced;
            icon.textContent = info.icon;
            title.textContent = info.title;
            desc.textContent = info.desc;
            
            card.className = `video-optimize-info-card ${preset}-mode`;
        };
        
        // 检测核心状态
        window.detectImageCoreStatus = async function() {
            if (window.modeManager) {
                await window.modeManager.detectCoreServices();
            }
        };
        
        window.detectVideoCoreStatus = async function() {
            // 视频核心状态检测逻辑
            log.debug('[UIHandlers] Detecting video core status...');
        };
        
        // 工具缓存
        window.toolsCache = {
            checked: false,
            pixlyInstalled: false,
            pixlyPath: null,
            ffmpegInstalled: false,
            ffmpegPath: null
        };
        
        log.info('[UIHandlers] Global functions registered');
    }
    
    /**
     * 绑定遗留事件（未模块化的部分）
     */
    function bindLegacyEvents() {
        // AI功能依赖关系
        initAIDependencies();
        
        // 高级AI选项展开/折叠
        initAdvancedAIToggle();
        
        log.info('[UIHandlers] Legacy events bound');
    }
    
    /**
     * 初始化AI功能依赖关系
     */
    function initAIDependencies() {
        const autoOptimizeCheckbox = document.getElementById('enableAutoOptimize');
        const bayesianCheckbox = document.getElementById('enableBayesian');
        
        if (autoOptimizeCheckbox && bayesianCheckbox) {
            autoOptimizeCheckbox.addEventListener('change', function() {
                if (!this.checked) {
                    bayesianCheckbox.checked = false;
                    bayesianCheckbox.disabled = true;
                } else {
                    bayesianCheckbox.disabled = false;
                }
            });
        }
    }
    
    /**
     * 初始化高级AI选项展开/折叠
     */
    function initAdvancedAIToggle() {
        const toggleButton = document.getElementById('toggleAdvancedAI');
        const advancedAIOptions = document.getElementById('advancedAIOptions');
        const toggleVideoAdvanced = document.getElementById('toggleVideoAdvanced');
        const videoAdvancedOptions = document.getElementById('videoAdvancedOptions');
        
        if (toggleButton && advancedAIOptions) {
            toggleButton.addEventListener('click', function() {
                const isExpanded = advancedAIOptions.style.display === 'block';
                advancedAIOptions.style.display = isExpanded ? 'none' : 'block';
                this.classList.toggle('expanded');
            });
        }
        
        if (toggleVideoAdvanced && videoAdvancedOptions) {
            toggleVideoAdvanced.addEventListener('click', function() {
                const isExpanded = videoAdvancedOptions.style.display === 'block';
                videoAdvancedOptions.style.display = isExpanded ? 'none' : 'block';
                this.classList.toggle('expanded');
            });
        }
    }
    
    /**
     * 文档准备就绪后初始化
     */
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initializeUI);
    } else {
        // DOM已加载
        setTimeout(initializeUI, 100);
    }
    
    // 导出初始化函数
    window.PIXLY.initializeUI = initializeUI;
    
    log.info('[UIHandlers] Modular UI handlers loaded (v2.0.0)');
    
})(window);
