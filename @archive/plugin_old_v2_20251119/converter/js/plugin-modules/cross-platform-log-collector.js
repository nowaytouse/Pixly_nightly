/**
 * 🌐 PIXLY跨平台日志收集器
 * 
 * 统一收集和处理来自三端的日志：
 * - JavaScript插件层（本地）
 * - Rust内核层（stdout JSON）
 * - GO AI服务层（HTTP/WebSocket）
 * 
 * 遵循CROSS_PLATFORM_LOG_SPEC.md规范
 */

// Log instance for the module
const getLog = () => window.pixlyLog || console;

class CrossPlatformLogCollector {
    constructor() {
        this.logs = [];
        this.maxLogs = 10000;
        this.subscribers = new Set();
        this.initialized = false;
        
        // 日志级别映射
        this.levelMap = {
            'ERROR': 0,
            'WARN': 1,
            'INFO': 2,
            'DEBUG': 3,
            'TRACE': 4,
            'error': 0,
            'warn': 1,
            'info': 2,
            'debug': 3,
            'trace': 4
        };
    }

    /**
     * 初始化收集器
     */
    init() {
        if (this.initialized) return;
        
        if (getLog()) { const log = getLog();
            log.info('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_INIT, {}));
        }
        
        // 1. 集成JS插件日志
        this.integrateJSLogs();
        
        // 2. 监听Rust CLI输出
        this.integrateRustLogs();
        
        // 3. 连接GO服务日志
        this.integrateGoLogs();
        
        this.initialized = true;
        if (getLog()) { const log = getLog();
            log.info('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_INITIALIZED, {}));
        }
    }

    /**
     * 集成JS插件日志
     */
    integrateJSLogs() {
        if (window.pixlyLog) {
            const originalLog = window.pixlyLog;
            const self = this;
            
            // 拦截所有日志方法
            ['error', 'warn', 'info', 'debug', 'trace'].forEach(level => {
                const original = originalLog[level];
                if (original) {
                    originalLog[level] = function(module, message, params) {
                        // 调用原方法
                        const result = original.call(originalLog, module, message, params);
                        
                        // 收集日志
                        self.collectJS(level.toUpperCase(), module, message, params);
                        
                        return result;
                    };
                }
            });
        }
    }

    /**
     * 集成Rust CLI日志
     */
    integrateRustLogs() {
        if (!window.rustCLI) {
            if (getLog()) { const log = getLog();
                log.warn('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_RUST_NOT_FOUND, {}));
            }
            return;
        }

        const self = this;
        
        // 监听stdout
        const originalOn = window.rustCLI.on || window.rustCLI.stdout?.on;
        if (originalOn) {
            originalOn.call(window.rustCLI, 'data', (data) => {
                try {
                    // 尝试解析JSON日志
                    const logEntry = JSON.parse(data);
                    if (logEntry.level && logEntry.message) {
                        self.collectRust(logEntry);
                    }
                } catch (e) {
                    // 非JSON输出，忽略或按纯文本处理
                }
            });
        }
    }

    /**
     * 集成GO服务日志
     */
    integrateGoLogs() {
        // GO服务通过HTTP轮询或WebSocket连接
        // 这里实现基础HTTP轮询
        
        const goServiceUrl = 'http://localhost:50052';
        
        // 检查GO服务是否可用
        fetch(`${goServiceUrl}/api/v1/health`)
            .then(response => {
                if (response.ok) {
                    if (getLog()) { const log = getLog();
                        log.info('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_GO_DETECTED, {}));
                    }
                    this.startGoLogPolling(goServiceUrl);
                }
            })
            .catch(() => {
                if (getLog()) { const log = getLog();
                    log.info('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_GO_NOT_AVAILABLE, {}));
                }
            });
    }

    /**
     * 启动GO日志轮询
     */
    startGoLogPolling(baseUrl) {
        let consecutiveFailures = 0;
        const maxFailures = 3;
        
        // 每5秒轮询一次
        const interval = setInterval(async () => {
            try {
                const response = await fetch(`${baseUrl}/api/v1/logs?limit=50`);
                if (response.ok) {
                    consecutiveFailures = 0;
                    const logs = await response.json();
                    logs.forEach(log => this.collectGo(log));
                } else if (response.status === 404) {
                    // API endpoint不存在，停止轮询
                    consecutiveFailures++;
                    if (consecutiveFailures >= maxFailures) {
                        if (getLog()) { const log = getLog();
                            log.info('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_GO_API_UNAVAILABLE, {}));
                        }
                        clearInterval(interval);
                    }
                }
            } catch (e) {
                // 静默失败
                consecutiveFailures++;
                if (consecutiveFailures >= maxFailures) {
                    clearInterval(interval);
                }
            }
        }, 5000);
    }

    /**
     * 收集JS日志
     */
    collectJS(level, module, message, params) {
        this.addLog({
            timestamp: Date.now(),
            source: 'JS',
            level: level,
            module: module,
            message: message,
            params: params
        });
    }

    /**
     * 收集Rust日志
     */
    collectRust(jsonLog) {
        this.addLog({
            timestamp: jsonLog.timestamp || Date.now(),
            source: 'Rust',
            level: (jsonLog.level || 'INFO').toUpperCase(),
            module: jsonLog.target || 'pixly_rust',
            message: jsonLog.message,
            params: jsonLog.fields
        });
    }

    /**
     * 收集GO日志
     */
    collectGo(jsonLog) {
        this.addLog({
            timestamp: new Date(jsonLog.time).getTime() || Date.now(),
            source: 'GO',
            level: (jsonLog.level || 'info').toUpperCase(),
            module: jsonLog.module || 'pixly-go',
            message: jsonLog.message,
            params: jsonLog
        });
    }

    /**
     * 添加日志条目
     */
    addLog(entry) {
        // 添加到缓冲区
        this.logs.push(entry);
        
        // 限制最大数量
        if (this.logs.length > this.maxLogs) {
            this.logs.shift();
        }
        
        // 通知订阅者
        this.notify(entry);
    }

    /**
     * 通知订阅者
     */
    notify(entry) {
        this.subscribers.forEach(callback => {
            try {
                callback(entry);
            } catch (e) {
                if (getLog()) { const log = getLog();
                    log.error('Cross-Platform Log', formatLog(LOG.LOG_COLLECTOR_SUBSCRIBER_ERROR, { error: e.message || e }));
                }
            }
        });
    }

    /**
     * 订阅日志事件
     * 
     * @param {Function} callback - 回调函数 (logEntry) => {}
     */
    subscribe(callback) {
        this.subscribers.add(callback);
        return () => this.subscribers.delete(callback);
    }

    /**
     * 查询日志
     * 
     * @param {Object} filters - 过滤条件
     * @returns {Array} 匹配的日志
     */
    query(filters = {}) {
        return this.logs.filter(log => {
            if (filters.source && log.source !== filters.source) return false;
            if (filters.level !== undefined) {
                const filterLevel = this.levelMap[filters.level];
                const logLevel = this.levelMap[log.level];
                if (logLevel > filterLevel) return false;
            }
            if (filters.module && !log.module.includes(filters.module)) return false;
            if (filters.search && !log.message.includes(filters.search)) return false;
            if (filters.startTime && log.timestamp < filters.startTime) return false;
            if (filters.endTime && log.timestamp > filters.endTime) return false;
            return true;
        });
    }

    /**
     * 获取最近的日志
     */
    getRecent(count = 100) {
        return this.logs.slice(-count);
    }

    /**
     * 按来源统计
     */
    getStatsBySource() {
        const stats = { JS: 0, Rust: 0, GO: 0 };
        this.logs.forEach(log => {
            if (stats[log.source] !== undefined) {
                stats[log.source]++;
            }
        });
        return stats;
    }

    /**
     * 按级别统计
     */
    getStatsByLevel() {
        const stats = { ERROR: 0, WARN: 0, INFO: 0, DEBUG: 0, TRACE: 0 };
        this.logs.forEach(log => {
            if (stats[log.level] !== undefined) {
                stats[log.level]++;
            }
        });
        return stats;
    }

    /**
     * 导出日志
     * 
     * @param {String} format - 'json' | 'csv' | 'text'
     * @returns {String} 导出内容
     */
    export(format = 'json') {
        if (format === 'json') {
            return JSON.stringify(this.logs, null, 2);
        } else if (format === 'csv') {
            let csv = 'Timestamp,Source,Level,Module,Message,Params\n';
            this.logs.forEach(log => {
                const params = JSON.stringify(log.params || {}).replace(/"/g, '""');
                csv += `${new Date(log.timestamp).toISOString()},${log.source},${log.level},${log.module},"${log.message}","${params}"\n`;
            });
            return csv;
        } else if (format === 'text') {
            return this.logs.map(log => {
                const time = new Date(log.timestamp).toISOString();
                const params = log.params ? ` ${JSON.stringify(log.params)}` : '';
                return `[${time}] [${log.source}] [${log.level}] [${log.module}] ${log.message}${params}`;
            }).join('\n');
        }
        return '';
    }

    /**
     * 清空日志
     */
    clear() {
        this.logs = [];
    }

    /**
     * 获取日志数量
     */
    get count() {
        return this.logs.length;
    }
}

// 创建全局实例
window.crossPlatformLogCollector = new CrossPlatformLogCollector();

// 自动初始化（延迟以确保其他模块加载完成）
setTimeout(() => {
    window.crossPlatformLogCollector.init();
}, 1000);

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = CrossPlatformLogCollector;
}
