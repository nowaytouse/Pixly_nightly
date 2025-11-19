/**
 * PIXLY 全局变量andconfig
 * 基础module - 最先load
 */
(function(window) {
    'use strict';
    
    // initializedPIXLY命名空间
    window.PIXLY = window.PIXLY || {};
    
    // versioninfo
    PIXLY.VERSION = '3.0.0';
    PIXLY.BUILD_DATE = '2024-11-01';
    
    // 全局状态
    PIXLY.state = {
        pluginInstance: null,
        selectedFiles: [],
        conversionProcess: null,
        isConverting: false,
        currentMode: 'smart', // smart | manual
        currentFormat: 'jxl', // jxl | avif | webp | png
        gpuEnabled: false,
        gpuType: null
    };
    
    // config
    PIXLY.config = {
        // 默认质量parameter
        defaultQuality: 90,
        defaultEffort: 7,
        
        // AIconfig
        aiEnabled: false,
        aiMode: 'hybrid', // standard | advanced | hybrid
        aiServerUrl: 'http://localhost:5000',
        
        // GPUconfig
        autoDetectGPU: true,
        preferredGPU: 'auto', // auto | nvenc | qsv | amf | videotoolbox
        
        // Convertconfig
        preserveMetadata: true,
        preserveAlpha: true,
        autoReplaceOriginal: false,
        workers: 0, // 0 = auto
        
        // logconfig
        maxLogEntries: 1000,
        logLevel: 'info', // debug | info | warning | error
        
        // cacheconfig
        cacheDir: null,
        autoClearCache: true,
        cacheMaxSize: 1024 // MB
    };
    
    // 格式config
    PIXLY.formats = {
        jxl: {
            name: 'JPEG XL',
            ext: '.jxl',
            tool: 'cjxl',
            supports: {
                lossless: true,
                alpha: true,
                animation: true,
                metadata: true
            }
        },
        avif: {
            name: 'AVIF',
            ext: '.avif',
            tool: 'avifenc',
            supports: {
                lossless: true,
                alpha: true,
                animation: true,
                metadata: true
            }
        },
        webp: {
            name: 'WebP',
            ext: '.webp',
            tool: 'cwebp',
            supports: {
                lossless: true,
                alpha: true,
                animation: true,
                metadata: false
            }
        },
        png: {
            name: 'PNG',
            ext: '.png',
            tool: 'ffmpeg',
            supports: {
                lossless: true,
                alpha: true,
                animation: false,
                metadata: true
            }
        }
    };
    
    // Optimize模式config
    PIXLY.optimizeModes = {
        size: {
            name: '体积优先',
            quality: 75,
            effort: 7,
            description: 'ML + 体积优先 (Q75) + SSIM validate'
        },
        balanced: {
            name: '平衡',
            quality: 85,
            effort: 7,
            description: 'ML + 平衡质量 (Q85) + SSIM validate - 推荐'
        },
        quality: {
            name: '质量优先',
            quality: 95,
            effort: 9,
            description: 'ML + 最高质量 (Q95, 无损) + SSIM validate'
        },
        universal: {
            name: '通用',
            quality: 100,
            effort: 7,
            description: 'JPEG→JXL 无损 | PNG→JXL 无损 | GIF→AVIF'
        }
    };
    
    // GPU类型config
    PIXLY.gpuTypes = {
        nvenc: { name: 'NVIDIA NVENC', flag: '--gpu nvenc' },
        qsv: { name: 'Intel QuickSync', flag: '--gpu qsv' },
        amf: { name: 'AMD AMF', flag: '--gpu amf' },
        videotoolbox: { name: 'Apple VideoToolbox', flag: '--gpu videotoolbox' }
    };
    
    // 工具函数 - 获取当前config
    PIXLY.getConfig = function(key) {
        if (key) {
            return PIXLY.config[key];
        }
        return PIXLY.config;
    };
    
    // 工具函数 - settingsconfig
    PIXLY.setConfig = function(key, value) {
        if (typeof key === 'object') {
            Object.assign(PIXLY.config, key);
        } else {
            PIXLY.config[key] = value;
        }
    };
    
    // 工具函数 - 获取当前状态
    PIXLY.getState = function(key) {
        if (key) {
            return PIXLY.state[key];
        }
        return PIXLY.state;
    };
    
        // 工具函数 - settings状态
    PIXLY.setState = function(key, value) {
        if (typeof key === 'object') {
            Object.assign(PIXLY.state, key);
        } else {
            PIXLY.state[key] = value;
        }
    };
    
    // 🔥 Phase 40.32: 日志优化 - 仅记录关键进度点
    let lastLoggedProgress = -1;
    
    // 🚀 性能优化：RAF节流器，减少DOM更新频率
    let progressThrottle = null;
    
    // 🔥 增强的进度更新函数（支持平滑动画和失败回退）
    // 🚀 添加RAF节流：高频调用时自动合并为每帧一次更新
    window.updateProgress = function(current, total, message, subProgress = 0, isFailed = false) {
        // 初始化节流器
        if (!progressThrottle && window.PIXLY && window.PIXLY.PerformanceUtils) {
            progressThrottle = new window.PIXLY.PerformanceUtils.RAFThrottle(
                _updateProgressInternal
            );
        }
        
        // 失败状态立即执行，不节流
        if (isFailed || current === total) {
            _updateProgressInternal(current, total, message, subProgress, isFailed);
            return;
        }
        
        // 正常更新使用节流
        if (progressThrottle) {
            progressThrottle.call(current, total, message, subProgress, isFailed);
        } else {
            // Fallback：直接执行
            _updateProgressInternal(current, total, message, subProgress, isFailed);
        }
    };
    
    // 内部实现，由节流器调用
    function _updateProgressInternal(current, total, message, subProgress = 0, isFailed = false) {
        // 🔥 Phase 40.32: 只在进度变化>=10%或完成时记录详细日志
        const fileProgress = ((current / total) * 100);
        // 🔥 修复：保留1位小数，避免进度条长时间停在整数值
        const totalProgress = Math.min(
            Math.round((fileProgress + (subProgress / total)) * 10) / 10,  // 1位小数
            100
        );
        const shouldLog = (Math.abs(totalProgress - lastLoggedProgress) >= 10) || 
                         totalProgress === 0 || 
                         totalProgress === 100 || 
                         isFailed;
        
        if (shouldLog) {
            const log = window.pixlyLog;
            if (log) {
                log.debug('PIXLY Progress', formatLog(LOG.GLOBALS_PROGRESS_UPDATE, { 
                    progress: totalProgress, 
                    current, 
                    total, 
                    message 
                }));
            }
            lastLoggedProgress = totalProgress;
        }
        
        const progressFill = document.getElementById('progressFill');
        const progressText = document.getElementById('progressText');
        const progressPercent = document.getElementById('progressPercent');
        const currentFile = document.getElementById('currentFile');
        
        // 🔥 新增：内联进度条元素
        const inlineProgressFill = document.getElementById('inlineProgressFill');
        const inlineProgressText = document.getElementById('inlineProgressText');
        const inlineProgressPercent = document.getElementById('inlineProgressPercent');
        
        if (shouldLog && false) {  // 🔥 Phase 40.32: 关闭DOM元素检查日志（仅调试时启用）
            const log = window.pixlyLog;
            if (log) {
                log.trace('PIXLY Progress', formatLog(LOG.GLOBALS_PROGRESS_DOM_CHECK, { 
                    status: JSON.stringify({
                        progressFill: !!progressFill,
                        progressText: !!progressText,
                        progressPercent: !!progressPercent,
                        inlineProgressBar: !!inlineProgressBar,
                        inlineProgressPercent: !!inlineProgressPercent
                    })
                }));
            }
        }
        
        if (!progressFill || !progressText || !progressPercent) {
            const log = window.pixlyLog;
            if (log) {
                log.error('PIXLY Progress', formatLog(LOG.GLOBALS_PROGRESS_ELEMENTS_NOT_FOUND, {
                    missing: JSON.stringify({
                        progressFill: !progressFill,
                        progressText: !progressText,
                        progressPercent: !progressPercent
                    })
                }));
            }
            return;
        }
        
        // 🚨 失败状态：触发回退动画（持续1.5秒，快速响应）
        if (isFailed) {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Progress', formatLog(LOG.GLOBALS_PROGRESS_ROLLBACK, {}));
            }
            
            // 1. 立即变红、抖动、回退到0%（同时进行）
            progressFill.style.background = 'linear-gradient(90deg, #ff1744, #f44336)';
            progressFill.style.animation = 'shake 0.5s';
            progressFill.style.boxShadow = '0 0 20px rgba(255, 23, 68, 0.8)';
            progressFill.style.transition = 'width 0.8s cubic-bezier(0.25, 0.1, 0.25, 1)';  // 平滑回退
            progressFill.style.width = '0%';  // 立即开始回退
            
            // 2. 立即更新文字为红色
            progressPercent.textContent = '0%';
            progressPercent.style.color = '#ff1744';
            progressText.textContent = message || '❌ 转换失败';
            progressText.style.color = '#ff1744';
            if (currentFile) {
                currentFile.textContent = i18n.t('common.failed');
                currentFile.style.color = '#ff1744';
            }
            
            // 3. 1.5秒后恢复正常颜色（但保持0%）
            setTimeout(() => {
                progressFill.style.background = '';
                progressFill.style.animation = '';
                progressFill.style.boxShadow = '';
                progressPercent.style.color = '';
                progressText.style.color = '';
                if (currentFile) currentFile.style.color = '';
            }, 1500);
            
            return;
        }
        
        // 正常进度更新
        // 计算总体进度（包括子进度）
        const fileWeight = 100 / total; // 每个文件占的权重
        // totalProgress已在函数开头计算
        
        // 🔥 Phase 40.32: 调试日志已移至函数开头（根据shouldLog控制）
        
        // 更新进度条（带过渡动画）
        progressFill.style.width = `${totalProgress}%`;
        progressPercent.textContent = `${totalProgress}%`;
        
        // 🔥 同步更新内联进度条
        if (inlineProgressFill) {
            inlineProgressFill.style.width = `${totalProgress}%`;
        }
        if (inlineProgressPercent) {
            inlineProgressPercent.textContent = `${totalProgress}%`;
        }
        if (inlineProgressText && message) {
            inlineProgressText.textContent = message;
        }
        
        // 更新状态文本
        if (message) {
            progressText.textContent = message;
        }
        
        // 更新当前文件名
        if (currentFile) {
            if (current < total) {
                currentFile.textContent = `[${current + 1}/${total}]`;
            } else {
                currentFile.textContent = '';
            }
        }
        
        // 进度完成时的特殊处理
        if (totalProgress >= 100) {
            progressFill.style.background = 'linear-gradient(90deg, #4CAF50 0%, #45a049 100%)';
            setTimeout(() => {
                progressFill.style.background = '';
            }, 1000);
        }
    };
    
    // 🔥 更新子任务进度（用于单个文件的转换阶段）
    window.updateSubProgress = function(stage, stageProgress) {
        const i18n = window.i18n || { t: (key) => key };
        const stages = {
            'prepare': { weight: 0.1, name: i18n.t('progress.prepare') },
            'convert': { weight: 0.7, name: i18n.t('progress.convert') },
            'validate': { weight: 0.1, name: i18n.t('progress.validate') },
            'finalize': { weight: 0.1, name: i18n.t('progress.finalize') }
        };
        
        const stageInfo = stages[stage] || { weight: 0, name: stage };
        
        // 计算当前阶段的累计进度
        const stageKeys = Object.keys(stages);
        const currentStageIndex = stageKeys.indexOf(stage);
        
        let accumulatedProgress = 0;
        for (let i = 0; i < currentStageIndex; i++) {
            accumulatedProgress += stages[stageKeys[i]].weight;
        }
        
        // 加上当前阶段的进度
        const currentStageProgress = accumulatedProgress + (stageInfo.weight * stageProgress / 100);
        
        return currentStageProgress * 100; // 返回0-100的值，供 updateProgress 使用
    };
    
    // 🔥 验证函数是否正确注册
    const log = window.pixlyLog;
    if (log) {
        log.debug('PIXLY Core', formatLog(LOG.GLOBALS_UPDATE_PROGRESS_REGISTERED, { type: typeof window.updateProgress }));
        log.debug('PIXLY Core', formatLog(LOG.GLOBALS_UPDATE_SUBPROGRESS_REGISTERED, { type: typeof window.updateSubProgress }));
        log.info('PIXLY Core', formatLog(LOG.GLOBALS_MODULE_LOADED, { version: PIXLY.VERSION }));
    }
    
})(window);
