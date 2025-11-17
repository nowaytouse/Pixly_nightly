/**
 * PIXLY Logger Module - 使用Eagle官方日志API
 * 文档: https://developer.eagle.cool/plugin-api/zh-cn/api/log
 * 依赖: Eagle API (eagle.log)
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    // 日志缓存用于去重 - 防止日志刷屏
    const logCache = new Map();
    const LOG_DEDUPE_WINDOW = 5000; // 5秒去重窗口

    /**
     * 检查是否应该记录日志（去重）
     * @param {string} message - 日志消息
     * @param {string} level - 日志级别
     * @returns {boolean} - true表示应该记录，false表示应该抑制
     */
    function shouldLog(message, level) {
        const cacheKey = `${level}:${message}`;
        const now = Date.now();
        
        if (logCache.has(cacheKey)) {
            const lastTime = logCache.get(cacheKey);
            if (now - lastTime < LOG_DEDUPE_WINDOW) {
                // 在时间窗口内抑制重复日志
                return false;
            }
        }
        
        // 更新缓存时间
        logCache.set(cacheKey, now);
        
        // 清理旧条目（保持缓存大小可控）
        if (logCache.size > 100) {
            const oldestAllowed = now - LOG_DEDUPE_WINDOW;
            for (const [key, time] of logCache.entries()) {
                if (time < oldestAllowed) {
                    logCache.delete(key);
                }
            }
        }
        
        return true;
    }

    /**
     * PIXLY日志管理器 - 封装Eagle官方API
     */
    const Logger = {
        
        /**
         * 添加日志 - 同时记录到Eagle和UI
         * @param {string} module - 模块名称
         * @param {string} message - 日志消息
         * @param {string} type - 日志类型 (success/error/warning/info/debug)
         */
        log: function(module, message, type = 'info') {
            // 去重检查
            if (!shouldLog(message, type)) {
                return;
            }
            
            const fullMessage = `[${module}] ${message}`;
            
            // 🔥 修复：使用Eagle官方API需要提供context对象
            // Eagle API: eagle.log.info({ name: 'pluginName' }, message)
            const context = { name: 'PIXLY' };  // 提供context避免 "Cannot read properties of undefined (reading 'name')"
            
            if (typeof eagle !== 'undefined' && eagle.log) {
                try {
                    switch(type) {
                        case 'error':
                            eagle.log.error(context, fullMessage);
                            break;
                        case 'warning':
                            eagle.log.warn(context, fullMessage);
                            break;
                        case 'debug':
                            eagle.log.debug(context, fullMessage);
                            break;
                        case 'info':
                        case 'success':
                        default:
                            eagle.log.info(context, fullMessage);
                            break;
                    }
                } catch (e) {
                    // 如果Eagle API调用失败，静默处理，不阻止程序继续
                    const log = window.pixlyLog;
                    if (log) {
                        log.error('PIXLY Logger', formatLog(LOG.LOGGER_EAGLE_API_ERROR, { error: e.message || e }));
                    }
                }
            }
            
            // 🔥 仅warning和error显示在UI中
            if (type !== 'warning' && type !== 'error') {
                return;
            }
            
            const logOutput = document.getElementById('logOutput');
            const logContainer = document.getElementById('logContainer');
            
            // 如果UI容器不存在，已经记录到Eagle日志，直接返回
            if (!logOutput || !logContainer) {
                return;
            }
            
            const logEntry = document.createElement('div');
            logEntry.className = `log-entry log-${type}`;
            
            // 添加时间戳
            const timestamp = new Date().toLocaleTimeString('zh-CN', { 
                hour12: false,
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit'
            });
            
            // 添加图标
            const icons = {
                success: '✅',
                error: '❌',
                warning: '⚠️',
                info: 'ℹ️'
            };
            const icon = icons[type] || 'ℹ️';
            
            logEntry.innerHTML = `
                <span class="log-time">${timestamp}</span>
                <span class="log-icon">${icon}</span>
                <span class="log-message">${this.escapeHtml(message)}</span>
            `;
            
            logOutput.appendChild(logEntry);
            
            // 🔥 自动显示日志容器
            logContainer.style.display = 'block';
            
            // 自动滚动到底部
            logOutput.scrollTop = logOutput.scrollHeight;
            
            // 限制日志数量（最多保留1000条）
            const maxLogs = 1000;
            while (logOutput.children.length > maxLogs) {
                logOutput.removeChild(logOutput.firstChild);
            }
        },
        
        /**
         * 兼容旧API - addLog
         */
        addLog: function(message, type = 'info') {
            this.log('PIXLY', message, type);
        },
        
        /**
         * 快捷方法
         */
        debug: function(module, message) { this.log(module, message, 'debug'); },
        info: function(module, message) { this.log(module, message, 'info'); },
        warn: function(module, message) { this.log(module, message, 'warning'); },
        error: function(module, message) { this.log(module, message, 'error'); },
        success: function(module, message) { this.log(module, message, 'success'); },
        
        /**
         * 清空UI日志
         */
        clear: function() {
            const logOutput = document.getElementById('logOutput');
            if (logOutput) {
                logOutput.innerHTML = '';
                if (typeof eagle !== 'undefined' && eagle.log) {
                    try {
                        eagle.log.info({ name: 'PIXLY' }, '[PIXLY Logger] UI log cleared');
                    } catch (e) {
                        const log = window.pixlyLog;
                        if (log) {
                            log.error('PIXLY Logger', formatLog(LOG.LOGGER_EAGLE_API_ERROR, { error: `in clear(): ${e.message || e}` }));
                        }
                    }
                }
            }
        },
        
        /**
         * 导出日志
         */
        exportLog: function() {
            const logOutput = document.getElementById('logOutput');
            if (!logOutput) return '';
            
            const logs = [];
            const entries = logOutput.querySelectorAll('.log-entry');
            
            entries.forEach(entry => {
                const time = entry.querySelector('.log-time')?.textContent || '';
                const message = entry.querySelector('.log-message')?.textContent || '';
                logs.push(`[${time}] ${message}`);
            });
            
            return logs.join('\n');
        },
        
        /**
         * HTML转义
         */
        escapeHtml: function(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
    };
    
    // 导出到PIXLY命名空间
    window.PIXLY.Logger = Logger;
    
    // 兼容旧API
    window.addLog = Logger.addLog.bind(Logger);
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Logger', formatLog(LOG.LOGGER_MODULE_LOADED, {}));
    }
    
})(window);
