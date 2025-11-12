/**
 * PIXLY Logger Module - 使用Eagle官方日志API
 * 文档: https://developer.eagle.cool/plugin-api/zh-cn/api/log
 * 依赖: Eagle API (eagle.log)
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    // Log cache for deduplication - prevent spam
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
         * 添加log
         * @param {string} message - log消息
         * @param {string} type - log类型 (success/error/warning/info)
         */
                addLog: function(message, type = 'info') {
            // Deduplicate logs to prevent spam
            if (!shouldLog(message, type)) {
                return;
            }
            
            // 🔥 log过滤：仅保留warninganderror
            if (type !== 'warning' && type !== 'error') {
// console.log(`[PIXLY] [${type}] ${message}`); // 可选：保留in控制台
                return; // 不inUI中显示
            }
            
            const logOutput = document.getElementById('logOutput');
            const logContainer = document.getElementById('logContainer');
            
            // 如果log容器不exists，只in控制台输出
            if (!logOutput || !logContainer) {
 console.log(`[PIXLY] [${type}] ${message}`);
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
            
            // 🔥 自动显示log容器
            logContainer.style.display = 'block';
            
            // 自动滚动to底部
            logOutput.scrollTop = logOutput.scrollHeight;
            
            // 限制log数量（最多保留1000条）
            const maxLogs = 1000;
            while (logOutput.children.length > maxLogs) {
                logOutput.removeChild(logOutput.firstChild);
            }
        },
        
        /**
         * 清空log
         */
        clearLog: function() {
            const logOutput = document.getElementById('logOutput');
            if (logOutput) {
                logOutput.innerHTML = '';
 console.log('[PIXLY Logger] Log cleared');
            }
        },
        
        /**
         * 导出log
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
         * savelogtofile
         */
        saveLogToFile: function(filename) {
            const fs = require('fs');
            const path = require('path');
            
            const logContent = this.exportLog();
            const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
            const defaultFilename = `pixly-log-${timestamp}.txt`;
            const savePath = path.join(require('os').homedir(), 'Desktop', filename || defaultFilename);
            
            try {
                fs.writeFileSync(savePath, logContent, 'utf8');
                this.addLog(`✅ logsavedto: ${savePath}`, 'success');
                return savePath;
            } catch (error) {
                this.addLog(`❌ savelogfailed: ${error.message}`, 'error');
                return null;
            }
        },
        
        /**
         * 过滤log
         */
        filterLogs: function(type) {
            const logOutput = document.getElementById('logOutput');
            if (!logOutput) return;
            
            const entries = logOutput.querySelectorAll('.log-entry');
            
            entries.forEach(entry => {
                if (type === 'all' || entry.classList.contains(`log-${type}`)) {
                    entry.style.display = '';
                } else {
                    entry.style.display = 'none';
                }
            });
        },
        
        /**
         * 获取log统计
         */
        getStats: function() {
            const logOutput = document.getElementById('logOutput');
            if (!logOutput) return {
                total: 0,
                success: 0,
                error: 0,
                warning: 0,
                info: 0
            };
            
            return {
                total: logOutput.children.length,
                success: logOutput.querySelectorAll('.log-success').length,
                error: logOutput.querySelectorAll('.log-error').length,
                warning: logOutput.querySelectorAll('.log-warning').length,
                info: logOutput.querySelectorAll('.log-info').length
            };
        },
        
        /**
         * HTML转义
         */
        escapeHtml: function(text) {
            if (PIXLY.utils && PIXLY.utils.escapeHtml) {
                return PIXLY.utils.escapeHtml(text);
            }
            
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        },
        
        /**
         * log级别控制
         */
        logLevel: {
            DEBUG: 0,
            INFO: 1,
            WARNING: 2,
            ERROR: 3
        },
        
        currentLevel: 1, // 默认INFO级别
        
        /**
         * settingslog级别
         */
        setLevel: function(level) {
            this.currentLevel = level;
        },
        
        /**
         * Log with level check
         */
        log: function(message, level, type = 'info') {
            if (level < this.currentLogLevel) {
                return;
            }
            
            // Deduplicate logs to prevent spam
            if (!shouldLog(message, type)) {
                return;
            }
            
            this.addLog(message, type);
        },
        
        /**
         * Debuglog
         */
        debug: function(message) {
            this.log(message, this.logLevel.DEBUG, 'info');
        },
        
        /**
         * Infolog
         */
        info: function(message) {
            this.log(message, this.logLevel.INFO, 'info');
        },
        
        /**
         * Warninglog
         */
        warning: function(message) {
            this.log(message, this.logLevel.WARNING, 'warning');
        },
        
        /**
         * Warning log
         */
        warn: function(message) {
            this.log(message, this.logLevel.WARNING, 'warning');
        },
        
        /**
         * Errorlog
         */
        error: function(message) {
            this.log(message, this.logLevel.ERROR, 'error');
        }
    };
    
    // 暴露to全局
    PIXLY.Logger = Logger;
    
    // 向后兼容
    window.addLog = function(message, type) {
        Logger.addLog(message, type);
    };
    
    window.clearLog = function() {
        Logger.clearLog();
    };
    
    window.exportLog = function() {
        return Logger.exportLog();
    };
    
 console.log('[PIXLY Logger] Logger System module loaded');
    
})(window);
