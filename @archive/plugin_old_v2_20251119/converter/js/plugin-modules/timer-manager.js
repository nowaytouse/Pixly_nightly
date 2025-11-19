/**
 * ==========================================
 * PIXLY Timer Manager
 * ==========================================
 *
 * 统一定时器管理，自动清理，防止内存泄漏
 *
 * 功能:
 * - 自动追踪所有定时器
 * - 组件卸载时自动清理
 * - 提供命名空间隔离
 *
 * @module TimerManager
 * @version 1.0.0
 * @date 2025-11-05
 */

(function() {
    'use strict';

    const Logger = window.Logger || console;

    /**
     * 定时器管理器
     */
    class TimerManager {
        constructor() {
            this.timers = new Map(); // 所有定时器
            this.namespaces = new Map(); // 命名空间
            this.idCounter = 0;
            
            // 监听页面卸载，自动清理所有定时器
            if (typeof window !== 'undefined') {
                window.addEventListener('beforeunload', () => this.clearAll());
            }
        }

        /**
         * 创建setTimeout，自动追踪
         * @param {Function} callback - 回调函数
         * @param {number} delay - 延迟时间（毫秒）
         * @param {string} namespace - 命名空间（可选）
         * @returns {number} 定时器ID
         */
        setTimeout(callback, delay, namespace = 'default') {
            const id = ++this.idCounter;
            const timerId = setTimeout(() => {
                try {
                    callback();
                } finally {
                    this.timers.delete(id);
                    this._removeFromNamespace(namespace, id);
                }
            }, delay);

            this.timers.set(id, {
                timerId,
                type: 'timeout',
                namespace,
                createdAt: Date.now()
            });

            this._addToNamespace(namespace, id);

            return id;
        }

        /**
         * 创建setInterval，自动追踪
         * @param {Function} callback - 回调函数
         * @param {number} interval - 间隔时间（毫秒）
         * @param {string} namespace - 命名空间（可选）
         * @returns {number} 定时器ID
         */
        setInterval(callback, interval, namespace = 'default') {
            const id = ++this.idCounter;
            const timerId = setInterval(() => {
                try {
                    callback();
                } catch (error) {
                    Logger.error('[TimerManager] Interval callback error:', error);
                }
            }, interval);

            this.timers.set(id, {
                timerId,
                type: 'interval',
                namespace,
                createdAt: Date.now()
            });

            this._addToNamespace(namespace, id);

            return id;
        }

        /**
         * 清除指定定时器
         * @param {number} id - 定时器ID
         */
        clear(id) {
            const timer = this.timers.get(id);
            if (!timer) return;

            if (timer.type === 'timeout') {
                clearTimeout(timer.timerId);
            } else {
                clearInterval(timer.timerId);
            }

            this.timers.delete(id);
            this._removeFromNamespace(timer.namespace, id);
        }

        /**
         * 清除指定命名空间的所有定时器
         * @param {string} namespace - 命名空间
         */
        clearNamespace(namespace) {
            const ids = this.namespaces.get(namespace);
            if (!ids) return;

            ids.forEach(id => this.clear(id));
            this.namespaces.delete(namespace);
        }

        /**
         * 清除所有定时器
         */
        clearAll() {
            this.timers.forEach((timer, id) => {
                if (timer.type === 'timeout') {
                    clearTimeout(timer.timerId);
                } else {
                    clearInterval(timer.timerId);
                }
            });

            this.timers.clear();
            this.namespaces.clear();
        }

        /**
         * 获取统计信息
         */
        getStats() {
            const stats = {
                total: this.timers.size,
                byType: { timeout: 0, interval: 0 },
                byNamespace: {},
                oldest: null
            };

            let oldestTime = Infinity;
            this.timers.forEach(timer => {
                stats.byType[timer.type]++;
                stats.byNamespace[timer.namespace] = (stats.byNamespace[timer.namespace] || 0) + 1;
                
                if (timer.createdAt < oldestTime) {
                    oldestTime = timer.createdAt;
                    stats.oldest = {
                        age: Date.now() - timer.createdAt,
                        type: timer.type,
                        namespace: timer.namespace
                    };
                }
            });

            return stats;
        }

        /**
         * 添加到命名空间
         * @private
         */
        _addToNamespace(namespace, id) {
            if (!this.namespaces.has(namespace)) {
                this.namespaces.set(namespace, new Set());
            }
            this.namespaces.get(namespace).add(id);
        }

        /**
         * 从命名空间移除
         * @private
         */
        _removeFromNamespace(namespace, id) {
            const ids = this.namespaces.get(namespace);
            if (ids) {
                ids.delete(id);
                if (ids.size === 0) {
                    this.namespaces.delete(namespace);
                }
            }
        }
    }

    // ==========================================
    // 初始化
    // ==========================================

    const timerManager = new TimerManager();

    // 暴露到全局
    window.TimerManager = timerManager;

    // 提供便捷方法
    window.safeSetTimeout = (callback, delay, namespace) => 
        timerManager.setTimeout(callback, delay, namespace);
    
    window.safeSetInterval = (callback, interval, namespace) => 
        timerManager.setInterval(callback, interval, namespace);
    
    window.clearTimer = (id) => 
        timerManager.clear(id);
    
    window.clearTimerNamespace = (namespace) => 
        timerManager.clearNamespace(namespace);

    Logger.info('[PIXLY Timer Manager] ✅ Initialized');
    Logger.info('[PIXLY Timer Manager] 💡 Use window.safeSetTimeout() / window.safeSetInterval()');

})();
