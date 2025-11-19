/**
 * 🌍 PIXLY i18n - 使用Eagle官方i18next
 * 文档: https://developer.eagle.cool/plugin-api/zh-cn/tutorial/i18n
 * 
 * Eagle插件内建i18next模块，无需自己实现加载逻辑
 * 翻译文件位于 _locales/ 目录，Eagle会自动加载
 */

(function() {
    'use strict';
    
    // 🔧 日志实例（延迟获取，避免初始化顺序问题）
    const getLog = () => window.pixlyLog || console;

    // 全局i18n对象 - 混合方案（无fallback链）
    window.i18n = {
        currentLocale: 'zh_CN',
        manualTranslation: null, // 用户切换后手动加载的翻译（完全替换模式）
        useManual: false,        // 是否使用手动翻译（false=Eagle i18next, true=手动JSON）
        
        /**
         * 初始化i18n系统
         * Eagle会自动加载_locales目录下的翻译文件
         */
        init: async function(locale = null) {
            // 检测Eagle当前语言作为默认
            if (typeof i18next !== 'undefined' && i18next.language) {
                this.currentLocale = i18next.language;
                getLog().info(`[i18n] 🦅 Eagle language detected: ${this.currentLocale}`);
            }
            
            // 检查localStorage（用户之前的选择）
            const saved = localStorage.getItem('pixly_language');
            if (saved) {
                this.currentLocale = saved;
                getLog().info(`[i18n] 💾 User preference found: ${saved}`);
            }
            
            // 🔥 总是手动加载翻译文件，确保完整性和一致性
            getLog().info(`[i18n] 📥 Loading translation file: ${this.currentLocale}`);
            const success = await this.loadTranslation(this.currentLocale);
            
            if (success) {
                // 使用手动加载的翻译
                this.useManual = true;
                getLog().info(`[i18n] ✅ Using manual translation: ${this.currentLocale}`);
            } else {
                // 加载失败，fallback到Eagle i18next
                getLog().warn(`[i18n] ⚠️ Failed to load translation, using Eagle i18next as fallback`);
                this.useManual = false;
            }
            
            // 更新所有UI文本
            this.updateAll();
            
            getLog().info(`[i18n] ✅ Initialized with locale: ${this.currentLocale}`);
        },

        /**
         * 加载指定语言的翻译文件
         */
        loadTranslation: async function(locale) {
            try {
                const response = await fetch(`_locales/${locale}.json`);
                if (!response.ok) {
                    throw new Error(`HTTP ${response.status}`);
                }
                
                const data = await response.json();
                this.manualTranslation = data;
                getLog().info(`[i18n] ✅ Translation loaded: ${locale}`);
                return true;
            } catch (error) {
                getLog().error(`[i18n] ❌ Failed to load ${locale}:`, error.message);
                return false;
            }
        },

        /**
         * 翻译函数（支持参数替换）
         */
        t: function(key, params) {
            let defaultValue;
            let replacements = {};
            
            // 兼容旧API：t(key, defaultValue) 或新API：t(key, {defaultValue, ...params})
            if (typeof params === 'string') {
                defaultValue = params;
            } else if (typeof params === 'object' && params !== null) {
                defaultValue = params.defaultValue;
                replacements = params;
            }
            
            let value;
            if (this.useManual && this.manualTranslation) {
                // 使用手动加载的翻译
                value = this._getNestedValue(this.manualTranslation, key);
                value = value || defaultValue || key;
            } else {
                // 使用Eagle i18next
                if (typeof i18next !== 'undefined' && i18next.t) {
                    return i18next.t(key, { defaultValue: defaultValue || key, ...replacements });
                }
                value = defaultValue || key;
            }
            
            // 替换参数占位符 {param}
            if (typeof value === 'string') {
                Object.keys(replacements).forEach(param => {
                    if (param !== 'defaultValue') {
                        value = value.replace(new RegExp(`\\{${param}\\}`, 'g'), replacements[param]);
                    }
                });
            }
            
            return value;
        },
        
        /**
         * 从嵌套对象中获取值（支持 'files.title' 这样的路径）
         */
        _getNestedValue: function(obj, path) {
            const keys = path.split('.');
            let value = obj;
            
            for (const key of keys) {
                if (value && typeof value === 'object' && key in value) {
                    value = value[key];
                } else {
                    return null;
                }
            }
            
            return value;
        },

        /**
         * 切换语言
         */
        switchLanguage: async function(locale) {
            if (this.currentLocale === locale && this.useManual) {
                getLog().info(`[i18n] ⚠️ Already using ${locale}`);
                return;
            }
            
            getLog().info(`[i18n] 🔄 User switching to: ${locale}`);
            
            this.currentLocale = locale;
            localStorage.setItem('pixly_language', locale);
            
            // 手动加载翻译文件（完全替换模式）
            getLog().info(`[i18n] 📥 Loading translation: ${locale}`);
            const success = await this.loadTranslation(locale);
            
            if (success) {
                // 切换到手动模式
                this.useManual = true;
                getLog().info(`[i18n] ✅ Switched to manual translation: ${locale}`);
            } else {
                // 加载失败，继续使用Eagle i18next
                getLog().error(`[i18n] ❌ Failed to load ${locale}, keeping Eagle i18next`);
                this.useManual = false;
            }
            
            // 更新DOM（会自动同步语言选择器）
            this.updateAll();
            
            // 🔥 触发自定义事件，通知需要重新渲染动态内容
            window.dispatchEvent(new CustomEvent('pixly:languageChanged', { 
                detail: { locale: locale } 
            }));
            
            getLog().info(`[i18n] ✅ Language switched to: ${locale}`);
        },

        /**
         * 更新所有带data-i18n属性的元素
         */
        updateAll: function() {
            // 🔥 首先同步语言选择器（确保每次都同步）
            const langSwitch = document.getElementById('languageSelect');
            getLog().info(`[i18n] 🔍 Selector debug: found=${!!langSwitch}, value=${langSwitch?.value}, target=${this.currentLocale}`);
            
            if (langSwitch) {
                if (langSwitch.value !== this.currentLocale) {
                    langSwitch.value = this.currentLocale;
                    getLog().info(`[i18n] 🔧 Synced language selector: ${langSwitch.value}`);
                } else {
                    getLog().info(`[i18n] ℹ️  Language selector already synced: ${langSwitch.value}`);
                }
            } else {
                getLog().warn(`[i18n] ⚠️ Language selector not found in DOM`);
            }
            
            const elements = document.querySelectorAll('[data-i18n]');
            let count = 0;
            let skipped = 0;
            
            // 🔥 调试：先测试一个翻译
            const testTranslation = this.t('files.title');
            getLog().info(`[i18n] 🧪 Test translation for 'files.title': "${testTranslation}"`);
            getLog().info(`[i18n] 🧪 Mode: ${this.useManual ? 'Manual JSON' : 'Eagle i18next'} | Locale: ${this.currentLocale}`);
            
            elements.forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (!key) return;
                
                // 🔥 支持分号分隔的多个绑定: "textKey;[attr]attrKey"
                const bindings = key.split(';').map(b => b.trim());
                let hasUpdated = false;
                
                bindings.forEach(binding => {
                    // 检查是否是属性绑定: [attribute]key
                    const attrMatch = binding.match(/^\[([^\]]+)\](.+)$/);
                    
                    if (attrMatch) {
                        // 属性翻译: [title]common.start
                        const attrs = attrMatch[1].split(',').map(a => a.trim());
                        const translationKey = attrMatch[2];
                        const translation = this.t(translationKey);
                        
                        if (translation && translation !== translationKey) {
                            attrs.forEach(attr => {
                                el.setAttribute(attr, translation);
                            });
                            hasUpdated = true;
                        }
                    } else {
                        // 文本内容翻译
                        const translation = this.t(binding);
                        if (translation && translation !== binding) {
                            // 根据元素类型更新不同属性
                            if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA') {
                                if (el.type === 'button' || el.type === 'submit') {
                                    el.value = translation;
                                } else {
                                    el.placeholder = translation;
                                }
                            } else {
                                // 🔥 直接更新textContent，即使有子元素
                                el.textContent = translation;
                            }
                            hasUpdated = true;
                        }
                    }
                });
                
                if (hasUpdated) {
                    count++;
                } else {
                    skipped++;
                }
            });
            
            getLog().info(`[i18n] 🔄 Updated ${count} elements (skipped ${skipped})`);
        },

        /**
         * 获取当前语言
         */
        getCurrentLocale: function() {
            return this.currentLocale;
        },

        /**
         * 检测系统语言
         */
        _detectLocale: function() {
            // 优先使用Eagle的语言设置
            if (typeof eagle !== 'undefined' && eagle.app && eagle.app.locale) {
                return eagle.app.locale;
            }
            
            // 从localStorage读取
            const saved = localStorage.getItem('pixly_language');
            if (saved) {
                return saved;
            }
            
            // 使用浏览器语言
            const browserLang = navigator.language || navigator.userLanguage;
            const langMap = {
                'zh-CN': 'zh_CN',
                'zh-TW': 'zh_TW',
                'zh-HK': 'zh_TW',
                'ja': 'ja_JP',
                'ja-JP': 'ja_JP',
                'en': 'en',
                'en-US': 'en',
                'en-GB': 'en'
            };
            
            return langMap[browserLang] || this.fallbackLocale;
        }
    };

    getLog().info('[i18n] ✅ i18n module loaded (Eagle i18next)');
    
    // 🔥 自动初始化（延迟到DOM完全加载后）
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setTimeout(() => {
                window.i18n.init();
            }, 100); // 延迟100ms确保模板已加载
        });
    } else {
        // DOM已加载，但仍延迟一下确保模板已渲染
        setTimeout(() => {
            window.i18n.init();
        }, 100);
    }
    
})();
