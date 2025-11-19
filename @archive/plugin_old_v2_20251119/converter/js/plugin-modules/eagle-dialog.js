/**
 * PIXLY Eagle Dialog System
 * 统一的对话框系统 - 完全替代原生alert/confirm
 */
(function(window) {
    'use strict';
    
    // 确保PIXLY命名空间exists
    window.PIXLY = window.PIXLY || {};
    
    const EagleDialog = {
        /**
         * 显示Eagle对话框（优先）或回退到Toast通知
         * @param {Object} options - 对话框选项
         * @returns {Promise<number>} - 用户选择按钮索引
         */
        show: async function(options) {
            if (typeof eagle === 'undefined' || !eagle.app || typeof eagle.app.showDialog !== 'function') {
                const log = window.pixlyLog;
                if (log) {
                    log.warn('PIXLY Eagle', formatLog(LOG.EAGLE_DIALOG_API_UNAVAILABLE, {}));
                }
                // 使用Toast作为fallback
                if (window.PIXLY?.Toast) {
                    const type = options.type || 'info';
                    window.PIXLY.Toast.show(options.title, options.message, type, 5000);
                }
                return 0;
            }
            
            try {
                const result = await eagle.app.showDialog({
                    title: options.title,
                    message: options.message,
                    buttons: options.buttons || ['OK'],
                    defaultId: options.defaultId || 0
                });
                
                return result;
            } catch (error) {
                const log = window.pixlyLog;
                if (log) {
                    log.error('PIXLY Eagle', formatLog(LOG.EAGLE_DIALOG_FAILED, { error: error.message || error }));
                }
                // 使用Toast作为fallback
                if (window.PIXLY?.Toast) {
                    window.PIXLY.Toast.show(options.title, options.message, 'error', 5000);
                }
                return 0;
            }
        },
        
        /**
         * 统一的alert替代方法
         * @param {string} title - 标题
         * @param {string} message - 消息内容
         * @param {string} type - 类型 (info/warning/error/success)
         */
        alert: async function(title, message, type = 'info') {
            // 优先使用Eagle对话框
            if (typeof eagle !== 'undefined' && eagle.app && typeof eagle.app.showDialog === 'function') {
                try {
                    await eagle.app.showDialog({
                title: title,
                message: message,
                        buttons: ['OK'],
                        defaultId: 0
            });
                    return;
                } catch (error) {
                    const log = window.pixlyLog;
                    if (log) {
                        log.error('PIXLY Eagle', formatLog(LOG.EAGLE_DIALOG_ALERT_FAILED, { error: error.message || error }));
                    }
                }
            }
            
            // Fallback到Toast
            if (window.PIXLY?.Toast) {
                window.PIXLY.Toast.show(title, message, type, 5000);
            } else {
                const log = window.pixlyLog;
                if (log) {
                    log.warn('PIXLY Dialog', formatLog(LOG.EAGLE_DIALOG_NO_NOTIFICATION, {}));
                }
            }
        },
        
        /**
         * 统一的confirm替代方法
         * @param {string} title - 标题
         * @param {string} message - 消息内容
         * @param {Array<string>} buttons - 按钮文本数组，默认['Cancel', 'OK']
         * @returns {Promise<boolean>} - true表示确认，false表示取消
         */
        confirm: async function(title, message, buttons = ['Cancel', 'OK']) {
            // 优先使用Eagle对话框
            if (typeof eagle !== 'undefined' && eagle.app && typeof eagle.app.showDialog === 'function') {
                try {
                    const result = await eagle.app.showDialog({
                title: title,
                message: message,
                        buttons: buttons,
                        defaultId: 1
            });
                    return result === 1; // 返回true表示点击了第二个按钮（OK）
                } catch (error) {
                    const log = window.pixlyLog;
                    if (log) {
                        log.error('PIXLY Eagle', formatLog(LOG.EAGLE_DIALOG_CONFIRM_FAILED, { error: error.message || error }));
                    }
                }
            }
            
            // Fallback到Toast + console提示
            if (window.PIXLY?.Toast) {
                window.PIXLY.Toast.warning(title, message + ' (Auto-confirmed)', 3000);
            }
            const log = window.pixlyLog;
            if (log) {
                log.warn('PIXLY Dialog', formatLog(LOG.EAGLE_DIALOG_CONFIRM_FALLBACK, {}));
            }
            return true; // 默认确认
        },
        
        /**
         * 快捷方法：显示成功消息
         */
        success: async function(title, message) {
            await this.alert(title, message, 'success');
        },
        
        /**
         * 快捷方法：显示警告消息
         */
        warning: async function(title, message) {
            await this.alert(title, message, 'warning');
        },
        
        /**
         * 快捷方法：显示错误消息
         */
        error: async function(title, message) {
            await this.alert(title, message, 'error');
        },
        
        /**
         * 快捷方法：显示信息消息
         */
        info: async function(title, message) {
            await this.alert(title, message, 'info');
        }
    };
    
    // 导出到PIXLY命名空间
    window.PIXLY.EagleDialog = EagleDialog;
    
    // 向后兼容 - 保持原有的showEagleDialog函数
    window.showEagleDialog = EagleDialog.show;
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Eagle Dialog', formatLog(LOG.EAGLE_DIALOG_MODULE_LOADED, {}));
    }
    
})(window);
