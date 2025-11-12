/**
 * PIXLY Eagle pluginlifecycle管理
 * 依赖: PIXLY命名空间, eagle API
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * Eaglepluginlifecycle处理器
     */
    const EagleLifecycle = {
        
        /**
         * plugincreate时
         */
        onCreate: function(plugin) {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_CREATED, {}));
            }
            
            if (PIXLY.setState) {
                PIXLY.setState('pluginInstance', plugin);
            }
            
            // initialized各 component
            this.initializeComponents();
            
            // ✅ 已删除硬编码翻译调用
        },
        
        /**
         * Plugin shown时 - 刷新file列表（带防抖）
         */
        _lastOnShowTime: 0,
        onShow: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_SHOWN, {}));
            }
            
            // 🔥 防抖：500ms内只执行一次
            const now = Date.now();
            if (now - this._lastOnShowTime < 500) {
                if (log) {
                    log.debug('PIXLY Eagle', formatLog(LOG.EAGLE_DEBOUNCED, {}));
                }
                return;
            }
            this._lastOnShowTime = now;
            
            // 🔥 转换中：完全跳过刷新
            if (window.PIXLY_CONVERTING) {
                if (log) {
                    log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_CONVERTING, {}));
                }
                return;
            }
            
            // 🔥 锁定状态：完全跳过刷新，保持遮罩层显示
            if (window.PIXLY_SELECTION_LOCKED) {
                if (log) {
                    log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_LOCKED, {}));
                    log.info('PIXLY Eagle', formatLog(LOG.EAGLE_LOCKED_TIP, {}));
                }
                return;
            }
            
            // 🔥 单次刷新，减少频率
            if (window.selectFiles) {
                setTimeout(() => {
                    window.selectFiles();
                    if (log) {
                        log.info('PIXLY Eagle', formatLog(LOG.EAGLE_REFRESHED, {}));
                    }
                }, 200);
            } else {
                if (log) {
                    log.error('PIXLY Eagle', formatLog(LOG.EAGLE_SELECT_NOT_FOUND, {}));
                    log.error('PIXLY Eagle', formatLog(LOG.EAGLE_SELECT_TIP, {}));
                }
            }
        },
        
        /**
         * plugin运lines时
         */
        onRun: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_RUNNING, {}));
            }
            // 🔥 减少刷New频率：onShow刷New，这里不再重复
        },
        
        /**
         * Plugin hidden时
         */
        onHide: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_HIDDEN, {}));
            }
        },
        
        /**
         * plugin退出前
         */
        onBeforeExit: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_EXIT, {}));
            }
            
            // 清理Convert进程（优雅关闭）
            const conversionProcess = PIXLY.getState ? PIXLY.getState('conversionProcess') : null;
            if (conversionProcess) {
                try {
                    if (log) {
                        log.info('PIXLY Eagle', formatLog(LOG.EAGLE_GRACEFUL_SHUTDOWN, {}));
                    }
                    conversionProcess.kill('SIGTERM');
                    
                    // 2秒后强制kill（退出时时间紧迫）
                    setTimeout(() => {
                        if (conversionProcess && !conversionProcess.killed) {
                            if (log) {
                                log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_FORCE_KILL, {}));
                            }
                            try {
                                conversionProcess.kill('SIGKILL');
                            } catch (e) {
                                // 进程可能已经结束
                            }
                        }
                    }, 2000);
                } catch (e) {
                    if (log) {
                        log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_CLEANUP_FAILED, { error: e.message }));
                    }
                }
            }
            
            // 清理所有定时器（防止内存泄漏）
            if (window.TimerManager) {
                try {
                    const stats = window.TimerManager.getStats();
                    if (stats.total > 0) {
                        if (log) {
                            log.info('PIXLY Eagle', formatLog(LOG.EAGLE_CLEAR_TIMERS, { count: stats.total }));
                        }
                        window.TimerManager.clearAll();
                    }
                } catch (e) {
                    if (log) {
                        log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_TIMER_CLEANUP_FAILED, { error: e.message }));
                    }
                }
            }
        },
        
        /**
         * initializedcomponent
         */
        initializeComponents: function() {
            // initialized主题
            if (window.initThemeToggle) {
                initThemeToggle();
            }
            
            // initializedAI
            if (typeof initAIIntegration === 'function') {
                initAIIntegration().then(() => {
                    if (typeof setupAIModeListeners === 'function') {
                        setupAIModeListeners();
                    }
                });
            }
            
            // initialized其他component
            if (window.initializePlugin) {
                initializePlugin();
            }
        },
        
                // ✅ applyFallbackTranslations 已删除 - 不再使用硬编码翻译
        
        /**
         * registeredalllifecycle事件
         */
        register: function() {
            if (typeof eagle === 'undefined') {
                const log = window.pixlyLog;
                if (log) {
                    log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_API_NOT_AVAILABLE, {}));
                }
                return;
            }
            
            eagle.onPluginCreate((plugin) => this.onCreate(plugin));
            eagle.onPluginShow(() => this.onShow());
            eagle.onPluginRun(() => this.onRun());
            eagle.onPluginHide(() => this.onHide());
            eagle.onPluginBeforeExit(() => this.onBeforeExit());
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY Eagle', formatLog(LOG.EAGLE_REGISTERED, {}));
            }
        }
    };
    
    // 暴露to全局
    PIXLY.EagleLifecycle = EagleLifecycle;
    
    // 自动registered（如果eagleavailable）
    if (typeof eagle !== 'undefined') {
        EagleLifecycle.register();
    } else {
        // waitingeagleload
        document.addEventListener('DOMContentLoaded', () => {
            if (typeof eagle !== 'undefined') {
                EagleLifecycle.register();
            }
        });
    }
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Eagle', formatLog(LOG.EAGLE_MODULE_LOADED, {}));
    }
    
})(window);
