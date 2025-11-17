/**
 * 日志管理器 - 分级日志系统
 * 
 * 日志级别：
 * - ERROR (0): 严重错误，总是显示
 * - WARN (1): 警告信息，生产环境显示
 * - INFO (2): 重要信息，生产环境显示
 * - DEBUG (3): 调试信息，仅开发环境
 * - TRACE (4): 详细追踪，仅开发环境+启用trace
 */

const LogManager = (function() {
    // 日志级别枚举
    const LogLevel = {
        ERROR: 0,
        WARN: 1,
        INFO: 2,
        DEBUG: 3,
        TRACE: 4
    };
    
    // 当前日志级别（默认：生产环境仅显示INFO及以上）
    let currentLevel = LogLevel.INFO;
    
    // 🔧 JSON输出模式（用于日志收集）
    let jsonOutputEnabled = false;
    
    // 日志去重缓存（防止短时间内重复日志）
    const logCache = new Map();
    const DEDUPE_WINDOW = 1000; // 1秒内相同日志只输出一次
    
    // 日志节流缓存（高频日志限流）
    const throttleCache = new Map();
    const THROTTLE_INTERVAL = 500; // 高频日志最多500ms输出一次
    
    // 日志计数器（用于折叠重复日志）
    const repeatCounts = new Map();
    
    // 检测是否为开发模式
    const isDevelopment = () => {
        return localStorage.getItem('pixly_debug_mode') === 'true' || 
               window.location.hostname === 'localhost' ||
               window.location.hostname === '127.0.0.1';
    };
    
    // 初始化日志级别
    const initLogLevel = () => {
        const savedLevel = localStorage.getItem('pixly_log_level');
        if (savedLevel) {
            currentLevel = parseInt(savedLevel);
        } else if (isDevelopment()) {
            currentLevel = LogLevel.DEBUG;
        } else {
            currentLevel = LogLevel.INFO;
        }
        
        // 检查JSON输出模式
        const jsonMode = localStorage.getItem('pixly_log_json');
        if (jsonMode === 'true') {
            jsonOutputEnabled = true;
        }
    };
    
    // emoji映射
    const emoji = {
        ERROR: '❌',
        WARN: '⚠️',
        INFO: '✅',
        DEBUG: '🔍',
        TRACE: '🔬'
    };
    
    // 颜色映射
    const colors = {
        ERROR: 'color: #ef4444; font-weight: bold;',
        WARN: 'color: #f59e0b; font-weight: bold;',
        INFO: 'color: #10b981;',
        DEBUG: 'color: #3b82f6;',
        TRACE: 'color: #8b5cf6; font-size: 10px;'
    };
    
    /**
     * 生成日志唯一键（用于去重）
     * 🚀 优化：使用快速hash算法替代字符串拼接
     */
    const getLogKey = (level, module, message) => {
        // 使用FNV-1a hash算法，比字符串拼接快约 3-5倍
        if (window.PIXLY && window.PIXLY.PerformanceUtils) {
            return window.PIXLY.PerformanceUtils.hashStrings(
                String(level),
                module,
                message
            );
        }
        // Fallback
        return `${level}:${module}:${message}`;
    };
    
    /**
     * 检查是否应该去重
     */
    const shouldDedupe = (key) => {
        const now = Date.now();
        const lastTime = logCache.get(key);
        
        if (lastTime && now - lastTime < DEDUPE_WINDOW) {
            // 更新计数
            const count = repeatCounts.get(key) || 0;
            repeatCounts.set(key, count + 1);
            return true; // 去重
        }
        
        // 如果有重复计数，先输出提示
        const count = repeatCounts.get(key);
        if (count && count > 0) {
            log.info(`%c🔁`, 'color: #6b7280;', `(above message repeated ${count} times)`);
            repeatCounts.delete(key);
        }
        
        logCache.set(key, now);
        return false; // 不去重
    };
    
    /**
     * 检查是否应该节流
     */
    const shouldThrottle = (key) => {
        const now = Date.now();
        const lastTime = throttleCache.get(key);
        
        if (lastTime && now - lastTime < THROTTLE_INTERVAL) {
            return true; // 节流
        }
        
        throttleCache.set(key, now);
        return false; // 不节流
    };
    
    /**
     * 通用日志函数（增强版：支持去重、节流、LOG常量和参数替换）
     */
    const log = (level, module, message, ...args) => {
        if (level > currentLevel) return;
        
        const levelName = Object.keys(LogLevel).find(key => LogLevel[key] === level);
        const prefix = `[${module}]`;
        const icon = emoji[levelName] || '';
        const style = colors[levelName] || '';
        
        // 🔥 智能处理消息：支持LOG常量和参数对象
        let finalMessage = message;
        let finalArgs = args;
        
        // 如果第一个参数是对象且消息中有占位符，则进行参数替换
        if (args.length > 0 && typeof args[0] === 'object' && !Array.isArray(args[0]) && message.includes('{')) {
            const params = args[0];
            // 使用全局formatLog函数（如果可用）
            if (typeof window !== 'undefined' && window.formatLog) {
                finalMessage = window.formatLog(message, params);
                finalArgs = args.slice(1); // 移除参数对象
            }
        }
        
        // 生成日志键（使用原始message，不是格式化后的）
        const logKey = getLogKey(level, module, message);
        
        // ERROR和WARN级别永不去重
        if (level > LogLevel.WARN) {
            // 检查是否需要去重
            if (shouldDedupe(logKey)) {
                return; // 跳过重复日志
            }
        }
        
        // 🔧 JSON输出模式
        if (jsonOutputEnabled) {
            const logEntry = {
                timestamp: new Date().toISOString(),
                level: levelName.toLowerCase(),
                module: `js.${module}`,
                message: finalMessage,
                args: finalArgs.length > 0 ? finalArgs : undefined
            };
            console.log(JSON.stringify(logEntry));
            return;
        }
        
        // 标准输出
        switch (level) {
            case LogLevel.ERROR:
                console.error(`%c${icon} ${prefix}`, style, finalMessage, ...finalArgs);
                break;
            case LogLevel.WARN:
                console.warn(`%c${icon} ${prefix}`, style, finalMessage, ...finalArgs);
                break;
            default:
                console.info(`%c${icon} ${prefix}`, style, finalMessage, ...finalArgs);
                break;
        }
    };
    
    /**
     * 性能追踪（仅DEBUG及以上级别）
     */
    const perfStart = (label) => {
        if (currentLevel >= LogLevel.DEBUG) {
            performance.mark(`${label}-start`);
        }
    };
    
    const perfEnd = (label, module = 'PERF') => {
        if (currentLevel >= LogLevel.DEBUG) {
            performance.mark(`${label}-end`);
            try {
                performance.measure(label, `${label}-start`, `${label}-end`);
                const measure = performance.getEntriesByName(label)[0];
                log(LogLevel.DEBUG, module, `⏱️ ${label}: ${measure.duration.toFixed(2)}ms`);
                performance.clearMarks(`${label}-start`);
                performance.clearMarks(`${label}-end`);
                performance.clearMeasures(label);
            } catch (e) {
                // 忽略性能测量错误
            }
        }
    };
    
    /**
     * 分组日志（仅DEBUG及以上）
     */
    const group = (label, level = LogLevel.DEBUG) => {
        if (level <= currentLevel) {
            console.group(label);
        }
    };
    
    const groupEnd = (level = LogLevel.DEBUG) => {
        if (level <= currentLevel) {
            console.groupEnd();
        }
    };
    
    // 初始化
    initLogLevel();
    
    // 公开API
    return {
        // 日志级别
        Level: LogLevel,
        
        // 设置日志级别
        setLevel: (level) => {
            currentLevel = level;
            localStorage.setItem('pixly_log_level', level);
            log.info(`%c🔧 [LogManager]`, 'color: #8b5cf6; font-weight: bold;', `Log level set to: ${Object.keys(LogLevel).find(k => LogLevel[k] === level)}`);
        },
        
        // 获取当前级别
        getLevel: () => currentLevel,
        
        // 🔧 JSON输出模式控制
        enableJSON: () => {
            jsonOutputEnabled = true;
            localStorage.setItem('pixly_log_json', 'true');
            console.info('📋 JSON log output enabled');
        },
        
        disableJSON: () => {
            jsonOutputEnabled = false;
            localStorage.removeItem('pixly_log_json');
            console.info('📋 JSON log output disabled');
        },
        
        isJSONEnabled: () => jsonOutputEnabled,
        
        // 快捷方法
        error: (module, message, ...args) => log(LogLevel.ERROR, module, message, ...args),
        warn: (module, message, ...args) => log(LogLevel.WARN, module, message, ...args),
        info: (module, message, ...args) => log(LogLevel.INFO, module, message, ...args),
        debug: (module, message, ...args) => log(LogLevel.DEBUG, module, message, ...args),
        trace: (module, message, ...args) => log(LogLevel.TRACE, module, message, ...args),
        
        // 性能追踪
        perf: {
            start: perfStart,
            end: perfEnd
        },
        
        // 分组
        group,
        groupEnd,
        
        // 工具方法
        isDev: isDevelopment,
        
        // 快速启用调试模式
        enableDebug: () => {
            localStorage.setItem('pixly_debug_mode', 'true');
            currentLevel = LogLevel.DEBUG;
            log.info('%c🔬 [LogManager]', 'color: #10b981; font-weight: bold;', 'Debug mode enabled');
        },
        
        // 快速disable调试模式
        disableDebug: () => {
            localStorage.setItem('pixly_debug_mode', 'false');
            currentLevel = LogLevel.INFO;
            log.info('%c📴 [LogManager]', 'color: #8b5cf6; font-weight: bold;', 'Debug mode disabled');
        },
        
        // 清理日志缓存（用于测试或强制显示）
        clearCache: () => {
            logCache.clear();
            throttleCache.clear();
            repeatCounts.clear();
            log.info('%c🧹 [LogManager]', 'color: #10b981;', 'Log cache cleared');
        },
        
        // 日志节流方法（用于高频日志）
        throttled: (module, message, ...args) => {
            const key = `throttle:${module}:${message}`;
            if (!shouldThrottle(key)) {
                log(LogLevel.DEBUG, module, message, ...args);
            }
        }
    };
})();

// 将LogManager暴露为全局变量
if (typeof window !== 'undefined') {
    window.pixlyLog = LogManager;
}

// 使用console输出加载信息（避免循环依赖）
console.log('%c🎯 [LogManager]', 'color: #10b981; font-weight: bold;', `Log Manager loaded | Current level: ${Object.keys(LogManager.Level).find(k => LogManager.Level[k] === LogManager.getLevel())}`);
console.log('%c💡 Tip:', 'color: #3b82f6;', 'Use pixlyLog.enableDebug() to enable detailed logging, pixlyLog.disableDebug() to disable');
