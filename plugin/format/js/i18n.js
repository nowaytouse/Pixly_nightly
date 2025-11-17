/**
 * PIXLY AI - 简化的国际化系统
 */

(function() {
    'use strict';
    
    // 内置翻译（避免加载失败）
    const translations = {
        'zh_CN': {
            'app.title': 'PIXLY AI - 智能多媒体处理',
            'app.subtitle': '智能多媒体处理 · 一键优化',
            'language.switchLanguage': '切换语言',
            'theme.toggle': '切换主题',
            'empty.title': '未选择文件',
            'empty.desc': '在 Eagle 中选择图片或视频，然后返回此插件',
            'empty.step1': '在 Eagle 选择文件',
            'empty.step2': '返回插件',
            'empty.step3': '一键转换',
            'files.selected': '个文件已选择',
            'files.clear': '清空',
            'files.refresh': '刷新文件列表',
            'ai.title': 'AI 智能选项',
            'ai.optimizeTarget': '优化目标',
            'ai.balanced': '平衡',
            'ai.balancedDesc': '质量与体积兼顾',
            'ai.quality': '质量优先',
            'ai.qualityDesc': '最佳画质',
            'ai.size': '体积优先',
            'ai.sizeDesc': '最小文件',
            'ai.features': 'AI 功能',
            'ai.smartQuality': '智能质量预测',
            'ai.smartQualityTooltip': 'AI会分析图像的纹理、边缘、色彩复杂度等特征，自动预测最适合的质量参数',
            'ai.formatRecommend': '格式智能推荐',
            'ai.formatRecommendTooltip': 'AI会根据图像特征（透明度、动画、色彩复杂度等）智能推荐最优格式',
            'ai.ssimValidation': 'SSIM 质量验证',
            'ai.ssimValidationTooltip': '转换完成后，使用结构相似性指数（SSIM）算法对比原始图像和转换后的图像，确保画质损失在可接受范围内',
            'ai.videoForAnimation': '动图转视频',
            'ai.videoForAnimationTooltip': '当检测到大型动图时，AI会智能推荐转换为视频格式，可大幅减小文件体积',
            'conversion.start': '开始智能转换',
            'progress.converting': '转换中...',
            'result.complete': '转换完成',
            'result.openFolder': '打开输出文件夹'
        },
        'en': {
            'app.title': 'PIXLY AI - Smart Media Converter',
            'app.subtitle': 'Smart Media Processing · One-Click Optimization',
            'language.switchLanguage': 'Switch Language',
            'theme.toggle': 'Toggle Theme',
            'empty.title': 'No Files Selected',
            'empty.desc': 'Select images or videos in Eagle, then return to this plugin',
            'empty.step1': 'Select files in Eagle',
            'empty.step2': 'Return to plugin',
            'empty.step3': 'One-click convert',
            'files.selected': 'files selected',
            'files.clear': 'Clear',
            'files.refresh': 'Refresh file list',
            'ai.title': 'AI Smart Options',
            'ai.optimizeTarget': 'Optimization Target',
            'ai.balanced': 'Balanced',
            'ai.balancedDesc': 'Balance quality and size',
            'ai.quality': 'Quality Priority',
            'ai.qualityDesc': 'Best quality',
            'ai.size': 'Size Priority',
            'ai.sizeDesc': 'Smallest file',
            'ai.features': 'AI Features',
            'ai.smartQuality': 'Smart Quality Prediction',
            'ai.smartQualityTooltip': 'AI analyzes image features to predict optimal quality parameters',
            'ai.formatRecommend': 'Smart Format Recommendation',
            'ai.formatRecommendTooltip': 'AI recommends optimal format based on image characteristics',
            'ai.ssimValidation': 'SSIM Quality Validation',
            'ai.ssimValidationTooltip': 'Validates image quality using SSIM algorithm after conversion',
            'ai.videoForAnimation': 'Animation to Video',
            'ai.videoForAnimationTooltip': 'AI recommends converting large animations to video format',
            'conversion.start': 'Start Smart Conversion',
            'progress.converting': 'Converting...',
            'result.complete': 'Conversion Complete',
            'result.openFolder': 'Open Output Folder'
        }
    };
    
    // 全局i18n对象
    window.i18n = {
        currentLocale: 'zh_CN',
        
        /**
         * 初始化i18n系统
         */
        init: async function() {
            // 从localStorage读取用户偏好
            const saved = localStorage.getItem('pixly_ai_language');
            if (saved && translations[saved]) {
                this.currentLocale = saved;
            }
            
            logger.info('i18n', LOG.I18N_INITIALIZED, { locale: this.currentLocale });
            
            // 更新UI
            this.updateAll();
        },
        
        /**
         * 翻译函数
         */
        t: function(key) {
            const trans = translations[this.currentLocale];
            return trans && trans[key] ? trans[key] : key;
        },
        
        /**
         * 切换语言
         */
        switchLanguage: async function(locale) {
            if (!translations[locale]) {
                logger.warn('i18n', LOG.I18N_UNSUPPORTED_LOCALE, { locale });
                return;
            }
            
            this.currentLocale = locale;
            localStorage.setItem('pixly_ai_language', locale);
            
            logger.info('i18n', LOG.I18N_LANGUAGE_SWITCHED, { locale });
            
            // 更新UI
            this.updateAll();
        },
        
        /**
         * 更新所有带data-i18n属性的元素
         */
        updateAll: function() {
            // 同步语言选择器
            const langSelect = document.getElementById('languageSelect');
            if (langSelect && langSelect.value !== this.currentLocale) {
                langSelect.value = this.currentLocale;
            }
            
            const elements = document.querySelectorAll('[data-i18n]');
            
            elements.forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (!key) return;
                
                // 支持分号分隔的多个绑定: "textKey;[attr]attrKey"
                const bindings = key.split(';').map(b => b.trim());
                
                bindings.forEach(binding => {
                    // 检查是否是属性绑定: [attribute]key
                    const attrMatch = binding.match(/^\[([^\]]+)\](.+)$/);
                    
                    if (attrMatch) {
                        // 属性翻译
                        const attrs = attrMatch[1].split(',').map(a => a.trim());
                        const translationKey = attrMatch[2];
                        const translation = this.t(translationKey);
                        
                        if (translation && translation !== translationKey) {
                            attrs.forEach(attr => {
                                el.setAttribute(attr, translation);
                            });
                        }
                    } else {
                        // 文本内容翻译
                        const translation = this.t(binding);
                        if (translation && translation !== binding) {
                            if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
                                if (el.type === 'button' || el.type === 'submit') {
                                    el.value = translation;
                                } else {
                                    el.placeholder = translation;
                                }
                            } else {
                                el.textContent = translation;
                            }
                        }
                    }
                });
            });
            
            logger.info('i18n', LOG.I18N_ELEMENTS_UPDATED, { count: elements.length });
        },
        
        /**
         * 获取当前语言
         */
        getCurrentLocale: function() {
            return this.currentLocale;
        }
    };
    
    logger.info('i18n', LOG.I18N_MODULE_LOADED);
    
})();
