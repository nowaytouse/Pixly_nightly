// 🔥 Phase 40.8: 功能开关系统
// 作用：集中管理所有可选功能的开关状态
// 架构原则：
// - 职责：功能开关存储、UI同步、持久化
// - 依赖：localStorage、Eagle API
// - 真实性：开关真实影响功能，无假开关

(function(PIXLY) {
    'use strict';
    
    // Log instance for the module
    const log = window.pixlyLog;
    
    if (log) {
        log.info('PIXLY Feature Flags', formatLog(LOG.FEATURE_FLAGS_LOADING, {}));
    }
    
    /**
     * 功能开关配置
     * 
     * 分类：
     * - core: 核心功能（不可禁用）
     * - ai: AI相关功能
     * - metadata: 元数据处理
     * - optimization: 优化功能
     * - experimental: 实验性功能
     */
    const DEFAULT_FLAGS = {
        // AI功能
        'ai.parameter_prediction': true,      // AI参数预测
        'ai.format_recommendation': true,     // AI格式推荐
        'ai.quality_validation': true,        // AI质量验证
        'ai.auto_feedback': true,             // 自动学习反馈
        
        // 元数据处理
        'metadata.preserve_exif': true,       // 保留EXIF
        'metadata.preserve_xmp': true,        // 保留XMP
        'metadata.preserve_icc': true,        // 保留ICC配置
        'metadata.preserve_timestamp': true,  // 保留时间戳
        
        // 文件处理
        'file.normalize_name': false,         // 规范化文件名（默认关闭）
        'file.eagle_integration': true,       // Eagle集成
        'file.generate_thumbnail': true,      // 生成缩略图
        
        // 转换优化
        'optimization.multi_thread': true,    // 多线程转换
        'optimization.cache_enabled': true,   // 缓存系统
        'optimization.animation_detect': true,// 动画检测
        
        // UI功能
        'ui.progress_simulation': true,       // 进度条模拟
        'ui.notifications': true,             // 系统通知
        'ui.theme_auto': true,                // 自动主题
        
        // 实验性功能
        'experimental.http_server': false,    // HTTP服务器
        'experimental.batch_parallel': false, // 并行批处理
    };
    
    /**
     * 功能开关管理器
     */
    class FeatureFlags {
        constructor() {
            this.flags = {};
            this.listeners = {}; // 监听器：flag -> [callbacks]
            this.storageKey = 'pixly_feature_flags';
            
            // 加载配置
            this.load();
        }
        
        /**
         * 加载配置（从localStorage）
         */
        load() {
            try {
                const stored = localStorage.getItem(this.storageKey);
                if (stored) {
                    const parsed = JSON.parse(stored);
                    this.flags = { ...DEFAULT_FLAGS, ...parsed };
                    if (log) {
                        log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_LOADED_STORAGE, { count: Object.keys(parsed).length }));
                    }
                } else {
                    this.flags = { ...DEFAULT_FLAGS };
                    if (log) {
                        log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_USING_DEFAULTS, {}));
                    }
                }
            } catch (e) {
                if (log) {
                    log.error('Feature Flags', formatLog(LOG.FEATURE_FLAGS_LOAD_FAILED, { error: e.message || e }));
                }
                this.flags = { ...DEFAULT_FLAGS };
            }
        }
        
        /**
         * 保存配置（到localStorage）
         */
        save() {
            try {
                // 只保存与默认值不同的配置
                const overrides = {};
                for (const [key, value] of Object.entries(this.flags)) {
                    if (DEFAULT_FLAGS[key] !== value) {
                        overrides[key] = value;
                    }
                }
                
                localStorage.setItem(this.storageKey, JSON.stringify(overrides));
                if (log) {
                    log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_SAVED, { count: Object.keys(overrides).length }));
                }
            } catch (e) {
                if (log) {
                    log.error('Feature Flags', formatLog(LOG.FEATURE_FLAGS_SAVE_FAILED, { error: e.message || e }));
                }
            }
        }
        
        /**
         * 获取功能开关状态
         * @param {string} flag - 功能标识
         * @returns {boolean}
         */
        isEnabled(flag) {
            return this.flags[flag] === true;
        }
        
        /**
         * 设置功能开关状态
         * @param {string} flag - 功能标识
         * @param {boolean} enabled - 是否启用
         */
        set(flag, enabled) {
            const oldValue = this.flags[flag];
            this.flags[flag] = enabled;
            
            if (log) {
                log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_TOGGLED, { status: enabled ? '✅' : '❌', flag }));
            }
            
            // 保存
            this.save();
            
            // 触发监听器
            if (oldValue !== enabled) {
                this.notifyListeners(flag, enabled);
            }
        }
        
        /**
         * 切换功能开关
         * @param {string} flag - 功能标识
         */
        toggle(flag) {
            this.set(flag, !this.isEnabled(flag));
        }
        
        /**
         * 重置为默认值
         */
        reset() {
            this.flags = { ...DEFAULT_FLAGS };
            this.save();
            if (log) {
                log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_RESET, {}));
            }
            
            // 触发所有监听器
            for (const flag of Object.keys(this.flags)) {
                this.notifyListeners(flag, this.flags[flag]);
            }
        }
        
        /**
         * 获取所有功能列表
         * @returns {Object} 分类后的功能列表
         */
        getAll() {
            const categorized = {
                ai: {},
                metadata: {},
                file: {},
                optimization: {},
                ui: {},
                experimental: {}
            };
            
            for (const [key, value] of Object.entries(this.flags)) {
                const category = key.split('.')[0];
                if (categorized[category]) {
                    categorized[category][key] = value;
                }
            }
            
            return categorized;
        }
        
        /**
         * 监听功能开关变化
         * @param {string} flag - 功能标识
         * @param {Function} callback - 回调函数
         */
        onChange(flag, callback) {
            if (!this.listeners[flag]) {
                this.listeners[flag] = [];
            }
            this.listeners[flag].push(callback);
        }
        
        /**
         * 通知监听器
         * @private
         */
        notifyListeners(flag, enabled) {
            if (this.listeners[flag]) {
                for (const callback of this.listeners[flag]) {
                    try {
                        callback(enabled);
                    } catch (e) {
                        if (log) {
                            log.error('Feature Flags', formatLog(LOG.FEATURE_FLAGS_LISTENER_ERROR, { flag, error: e.message || e }));
                        }
                    }
                }
            }
        }
        
        /**
         * 导出配置（用于分享或备份）
         * @returns {string} JSON字符串
         */
        export() {
            return JSON.stringify(this.flags, null, 2);
        }
        
        /**
         * 导入配置
         * @param {string} json - JSON字符串
         */
        import(json) {
            try {
                const imported = JSON.parse(json);
                this.flags = { ...DEFAULT_FLAGS, ...imported };
                this.save();
                if (log) {
                    log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_IMPORTED, {}));
                }
                
                // 触发所有监听器
                for (const flag of Object.keys(this.flags)) {
                    this.notifyListeners(flag, this.flags[flag]);
                }
            } catch (e) {
                if (log) {
                    log.error('Feature Flags', formatLog(LOG.FEATURE_FLAGS_IMPORT_FAILED, { error: e.message || e }));
                }
                throw new Error('Invalid configuration JSON');
            }
        }
    }
    
    /**
     * 创建全局实例
     */
    window.featureFlags = new FeatureFlags();
    PIXLY.featureFlags = window.featureFlags;
    
    if (log) {
        log.info('PIXLY Feature Flags', formatLog(LOG.FEATURE_FLAGS_MODULE_LOADED, {}));
        log.info('Feature Flags', formatLog(LOG.FEATURE_FLAGS_ACTIVE_COUNT, { count: Object.keys(window.featureFlags.flags).filter(k => window.featureFlags.isEnabled(k)).length }));
    }
    
})(window.PIXLY);
