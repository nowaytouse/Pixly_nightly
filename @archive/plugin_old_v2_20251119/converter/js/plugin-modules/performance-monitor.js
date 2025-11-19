/**
 * 性能监控工具
 * 
 * 功能：
 * - 监控Module Loads时间
 * - 检测重复初始化
 * - 追踪Network Requests
 * - Memory Usage统计
 * - 性能报告生成
 */

const PerformanceMonitor = (function() {
    // 监控数据
    const metrics = {
        moduleLoads: [],
        functionCalls: {},
        networkRequests: [],
        memorySnapshots: [],
        warnings: []
    };
    
    // 启用标志
    let enabled = false;
    
    /**
     * 启用性能监控
     */
    const enable = () => {
        enabled = true;
        const log = window.pixlyLog;
        // Hybrid: pixlyLog for production + styled console for dev
        if (log) {
            log.info('Performance Monitor', formatLog(LOG.PERF_MONITOR_ENABLED, {}));
        }
        log.info('%c🔬 [Performance Monitor]', 'color: #8b5cf6; font-weight: bold;', 'Performance monitoring enabled');
        
        // 开始内存监控
        startMemoryMonitoring();
    };
    
    /**
     * 禁用性能监控
     */
    const disable = () => {
        enabled = false;
        const log = window.pixlyLog;
        // Hybrid: pixlyLog for production + styled console for dev
        if (log) {
            log.info('Performance Monitor', formatLog(LOG.PERF_MONITOR_DISABLED, {}));
        }
        log.info('%c📴 [Performance Monitor]', 'color: #8b5cf6; font-weight: bold;', 'Performance monitoring disabled');
    };
    
    /**
     * 记录Module Loads
     */
    const recordModuleLoad = (moduleName, duration) => {
        if (!enabled) return;
        
        metrics.moduleLoads.push({
            name: moduleName,
            duration,
            timestamp: Date.now()
        });
    };
    
    /**
     * 记录函数调用
     */
    const recordFunctionCall = (functionName) => {
        if (!enabled) return;
        
        if (!metrics.functionCalls[functionName]) {
            metrics.functionCalls[functionName] = {
                count: 0,
                timestamps: []
            };
        }
        
        metrics.functionCalls[functionName].count++;
        metrics.functionCalls[functionName].timestamps.push(Date.now());
        
        // 检测重复调用
        if (metrics.functionCalls[functionName].count > 3) {
            const warning = `${functionName} called ${metrics.functionCalls[functionName].count} times`;
            metrics.warnings.push(warning);
            const log = window.pixlyLog;
            if (log) {
                log.warn('Performance Monitor', formatLog(LOG.PERF_MONITOR_WARNING, { warning: warning }));
            }
            log.warn(`%c[Performance Monitor]`, 'color: #f59e0b;', warning);
        }
    };
    
    /**
     * 记录Network Requests
     */
    const recordNetworkRequest = (url, method = 'GET', duration = 0) => {
        if (!enabled) return;
        
        metrics.networkRequests.push({
            url,
            method,
            duration,
            timestamp: Date.now()
        });
    };
    
    /**
     * 内存监控
     */
    const startMemoryMonitoring = () => {
        if (!performance.memory) {
            const log = window.pixlyLog;
            if (log) {
                log.warn('Performance Monitor', formatLog(LOG.PERF_MONITOR_MEMORY_NOT_AVAILABLE, {}));
            }
            log.warn('[Performance Monitor] ⚠️ Memory API not available');
            return;
        }
        
        setInterval(() => {
            if (!enabled) return;
            
            metrics.memorySnapshots.push({
                usedJSHeapSize: performance.memory.usedJSHeapSize,
                totalJSHeapSize: performance.memory.totalJSHeapSize,
                jsHeapSizeLimit: performance.memory.jsHeapSizeLimit,
                timestamp: Date.now()
            });
            
            // 只保留最近50个快照
            if (metrics.memorySnapshots.length > 50) {
                metrics.memorySnapshots.shift();
            }
        }, 2000);
    };
    
    /**
     * 生成性能报告
     */
    const generateReport = () => {
        console.group('%c📊 [Performance Report]', 'color: #10b981; font-weight: bold; font-size: 14px;');
        
        // 1. Module Loads统计
        if (metrics.moduleLoads.length > 0) {
            console.group('📦 Module Loads');
            const totalDuration = metrics.moduleLoads.reduce((sum, m) => sum + m.duration, 0);
            log.info(`Total duration: ${totalDuration.toFixed(2)}ms`);
            log.info(`Module count: ${metrics.moduleLoads.length}`);
            
            // 最慢的5个模块
            const slowest = [...metrics.moduleLoads]
                .sort((a, b) => b.duration - a.duration)
                .slice(0, 5);
            
            console.table(slowest.map(m => ({
                '模块': m.name,
                '耗时(ms)': m.duration.toFixed(2)
            })));
            console.groupEnd();
        }
        
        // 2. 函数调用统计
        if (Object.keys(metrics.functionCalls).length > 0) {
            console.group('🔄 函数调用');
            const repeated = Object.entries(metrics.functionCalls)
                .filter(([_, data]) => data.count > 3)
                .sort((a, b) => b[1].count - a[1].count);
            
            if (repeated.length > 0) {
                log.info('⚠️ Repeated function calls:');
                console.table(repeated.map(([name, data]) => ({
                    '函数': name,
                    '调用次数': data.count,
                    '首次调用': new Date(data.timestamps[0]).toLocaleTimeString(),
                    '最后调用': new Date(data.timestamps[data.timestamps.length - 1]).toLocaleTimeString()
                })));
            } else {
                log.info('✅ No repeated calls detected');
            }
            console.groupEnd();
        }
        
        // 3. Network Requests统计
        if (metrics.networkRequests.length > 0) {
            console.group('🌐 Network Requests');
            log.info(`Total requests: ${metrics.networkRequests.length}`);
            
            // 按URL分组统计
            const urlCounts = {};
            metrics.networkRequests.forEach(req => {
                urlCounts[req.url] = (urlCounts[req.url] || 0) + 1;
            });
            
            const repeated = Object.entries(urlCounts)
                .filter(([_, count]) => count > 1)
                .sort((a, b) => b[1] - a[1]);
            
            if (repeated.length > 0) {
                log.info('⚠️ Repeated requests:');
                console.table(repeated.map(([url, count]) => ({
                    'URL': url.length > 50 ? url.substring(0, 50) + '...' : url,
                    '请求次数': count
                })));
            }
            console.groupEnd();
        }
        
        // 4. Memory Usage
        if (metrics.memorySnapshots.length > 0) {
            console.group('💾 Memory Usage');
            const latest = metrics.memorySnapshots[metrics.memorySnapshots.length - 1];
            const first = metrics.memorySnapshots[0];
            
            const formatBytes = (bytes) => {
                return (bytes / 1024 / 1024).toFixed(2) + ' MB';
            };
            
            log.info(`Current usage: ${formatBytes(latest.usedJSHeapSize)}`);
            log.info(`Total allocated: ${formatBytes(latest.totalJSHeapSize)}`);
            log.info(`Limit: ${formatBytes(latest.jsHeapSizeLimit)}`);
            log.info(`Growth: ${formatBytes(latest.usedJSHeapSize - first.usedJSHeapSize)}`);
            console.groupEnd();
        }
        
        // 5. 警告汇总
        if (metrics.warnings.length > 0) {
            console.group('⚠️ 警告');
            metrics.warnings.forEach(w => log.warn(w));
            console.groupEnd();
        }
        
        // 6. 性能建议
        console.group('💡 性能建议');
        const suggestions = [];
        
        // 检查重复调用
        const repeatedFunctions = Object.entries(metrics.functionCalls)
            .filter(([_, data]) => data.count > 3).length;
        if (repeatedFunctions > 0) {
            suggestions.push('🔄 检测到多个函数被重复调用，考虑添加缓存或防抖');
        }
        
        // 检查重复Network Requests
        const repeatedRequests = Object.values(
            metrics.networkRequests.reduce((acc, req) => {
                acc[req.url] = (acc[req.url] || 0) + 1;
                return acc;
            }, {})
        ).filter(count => count > 2).length;
        
        if (repeatedRequests > 0) {
            suggestions.push('🌐 检测到重复的Network Requests，建议添加请求缓存');
        }
        
        // 检查内存增长
        if (metrics.memorySnapshots.length > 10) {
            const first = metrics.memorySnapshots[0];
            const latest = metrics.memorySnapshots[metrics.memorySnapshots.length - 1];
            const growth = latest.usedJSHeapSize - first.usedJSHeapSize;
            const growthMB = growth / 1024 / 1024;
            
            if (growthMB > 50) {
                suggestions.push(`💾 内存增长 ${growthMB.toFixed(2)}MB，可能存在内存泄漏`);
            }
        }
        
        if (suggestions.length > 0) {
            suggestions.forEach(s => log.info(s));
        } else {
            log.info('✅ No obvious performance issues');
        }
        console.groupEnd();
        
        console.groupEnd();
    };
    
    /**
     * 重置统计数据
     */
    const reset = () => {
        metrics.moduleLoads = [];
        metrics.functionCalls = {};
        metrics.networkRequests = [];
        metrics.memorySnapshots = [];
        metrics.warnings = [];
        
        const log = window.pixlyLog;
        if (log) {
            log.info('Performance Monitor', 'Statistics reset');
        }
        log.info('%c🔄 [Performance Monitor]', 'color: #8b5cf6;', 'Statistics reset');
    };
    
    // 公开API
    return {
        enable,
        disable,
        recordModuleLoad,
        recordFunctionCall,
        recordNetworkRequest,
        generateReport,
        reset,
        isEnabled: () => enabled,
        getMetrics: () => metrics
    };
})();

// 暴露到全局
if (typeof window !== 'undefined') {
    window.pixlyPerf = PerformanceMonitor;
}

const log = window.pixlyLog;
if (log) {
    log.info('Performance Monitor', formatLog(LOG.PERF_MONITOR_MODULE_LOADED, {}));
}
log.info('%c🔬 [Performance Monitor]', 'color: #8b5cf6; font-weight: bold;', 'Performance Monitor loaded');
log.info('%c💡 Tip:', 'color: #3b82f6;', 'Use pixlyPerf.enable() to enable monitoring, pixlyPerf.generateReport() to view report');
