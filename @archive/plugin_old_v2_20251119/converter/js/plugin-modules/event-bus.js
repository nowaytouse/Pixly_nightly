/**
 * Event Bus - 事件总线模块
 * 
 * 🔥 Phase 40.23: 循环依赖解耦
 * 
 * 用于解耦模块间的循环依赖，提供中心化的事件分发机制。
 * 
 * @module event-bus
 */

class EventBus {
    constructor() {
        this.listeners = new Map();
        this.eventLog = [];
        this.maxLogSize = 100;
        
        logger.log('info', 'EventBus', '事件总线已初始化');
    }
    
    /**
     * 注册事件监听器
     * @param {string} eventName - 事件名称
     * @param {Function} callback - 回调函数
     * @param {Object} context - 上下文（可选）
     * @returns {Function} - 取消订阅函数
     */
    on(eventName, callback, context = null) {
        if (!this.listeners.has(eventName)) {
            this.listeners.set(eventName, []);
        }
        
        const listener = { callback, context };
        this.listeners.get(eventName).push(listener);
        
        logger.log('debug', 'EventBus', `注册监听器: ${eventName}`);
        
        // 返回取消订阅函数
        return () => this.off(eventName, callback);
    }
    
    /**
     * 取消事件监听器
     * @param {string} eventName - 事件名称
     * @param {Function} callback - 回调函数
     */
    off(eventName, callback) {
        if (!this.listeners.has(eventName)) {
            return;
        }
        
        const listeners = this.listeners.get(eventName);
        const index = listeners.findIndex(l => l.callback === callback);
        
        if (index !== -1) {
            listeners.splice(index, 1);
            logger.log('debug', 'EventBus', `取消监听器: ${eventName}`);
        }
        
        // 如果没有监听器了，删除事件
        if (listeners.length === 0) {
            this.listeners.delete(eventName);
        }
    }
    
    /**
     * 触发事件
     * @param {string} eventName - 事件名称
     * @param {*} data - 事件数据
     * @returns {Promise<Array>} - 所有监听器的返回值
     */
    async emit(eventName, data = null) {
        // 记录事件
        this.logEvent(eventName, data);
        
        if (!this.listeners.has(eventName)) {
            logger.log('debug', 'EventBus', `无监听器: ${eventName}`);
            return [];
        }
        
        const listeners = this.listeners.get(eventName);
        logger.log('debug', 'EventBus', `触发事件: ${eventName} (${listeners.length}个监听器)`);
        
        // 并行执行所有监听器
        const results = await Promise.allSettled(
            listeners.map(async ({ callback, context }) => {
                try {
                    if (context) {
                        return await callback.call(context, data);
                    } else {
                        return await callback(data);
                    }
                } catch (error) {
                    logger.log('error', 'EventBus', `事件监听器错误 [${eventName}]:`, error);
                    throw error;
                }
            })
        );
        
        return results;
    }
    
    /**
     * 同步触发事件
     * @param {string} eventName - 事件名称
     * @param {*} data - 事件数据
     * @returns {Array} - 所有监听器的返回值
     */
    emitSync(eventName, data = null) {
        // 记录事件
        this.logEvent(eventName, data);
        
        if (!this.listeners.has(eventName)) {
            logger.log('debug', 'EventBus', `无监听器: ${eventName}`);
            return [];
        }
        
        const listeners = this.listeners.get(eventName);
        logger.log('debug', 'EventBus', `触发同步事件: ${eventName} (${listeners.length}个监听器)`);
        
        const results = [];
        for (const { callback, context } of listeners) {
            try {
                const result = context ? callback.call(context, data) : callback(data);
                results.push(result);
            } catch (error) {
                logger.log('error', 'EventBus', `同步事件监听器错误 [${eventName}]:`, error);
                results.push(null);
            }
        }
        
        return results;
    }
    
    /**
     * 一次性事件监听器
     * @param {string} eventName - 事件名称
     * @param {Function} callback - 回调函数
     * @param {Object} context - 上下文（可选）
     */
    once(eventName, callback, context = null) {
        const wrappedCallback = async (data) => {
            this.off(eventName, wrappedCallback);
            if (context) {
                return await callback.call(context, data);
            } else {
                return await callback(data);
            }
        };
        
        this.on(eventName, wrappedCallback, context);
    }
    
    /**
     * 记录事件（用于调试）
     * @param {string} eventName - 事件名称
     * @param {*} data - 事件数据
     */
    logEvent(eventName, data) {
        const event = {
            name: eventName,
            timestamp: Date.now(),
            data: data,
        };
        
        this.eventLog.push(event);
        
        // 限制日志大小
        if (this.eventLog.length > this.maxLogSize) {
            this.eventLog.shift();
        }
    }
    
    /**
     * 获取事件日志
     * @param {number} count - 获取最近的事件数量
     * @returns {Array} - 事件日志
     */
    getEventLog(count = 10) {
        return this.eventLog.slice(-count);
    }
    
    /**
     * 清空所有监听器
     */
    clearAll() {
        this.listeners.clear();
        logger.log('info', 'EventBus', '已清空所有监听器');
    }
    
    /**
     * 获取监听器统计
     * @returns {Object} - 监听器统计信息
     */
    getStats() {
        const stats = {
            totalEvents: this.listeners.size,
            totalListeners: 0,
            events: {},
        };
        
        for (const [eventName, listeners] of this.listeners.entries()) {
            stats.events[eventName] = listeners.length;
            stats.totalListeners += listeners.length;
        }
        
        return stats;
    }
}

// 🔥 Phase 40.23: 创建全局单例
const eventBus = new EventBus();

// 🔥 Phase 40.23: 定义标准事件名称
const Events = {
    // Rust CLI 事件
    RUST_CLI_EXECUTE: 'rust:cli:execute',
    RUST_CLI_SUCCESS: 'rust:cli:success',
    RUST_CLI_ERROR: 'rust:cli:error',
    
    // Eagle 事件
    EAGLE_LIBRARY_SCAN: 'eagle:library:scan',
    EAGLE_LIBRARY_LOADED: 'eagle:library:loaded',
    EAGLE_IMAGE_CONVERT: 'eagle:image:convert',
    EAGLE_IMAGE_CONVERTED: 'eagle:image:converted',
    EAGLE_BATCH_START: 'eagle:batch:start',
    EAGLE_BATCH_PROGRESS: 'eagle:batch:progress',
    EAGLE_BATCH_COMPLETE: 'eagle:batch:complete',
    
    // 转换事件
    CONVERSION_START: 'conversion:start',
    CONVERSION_PROGRESS: 'conversion:progress',
    CONVERSION_SUCCESS: 'conversion:success',
    CONVERSION_ERROR: 'conversion:error',
    
    // AI 事件
    AI_PREDICTION_START: 'ai:prediction:start',
    AI_PREDICTION_SUCCESS: 'ai:prediction:success',
    AI_PREDICTION_ERROR: 'ai:prediction:error',
    AI_FEEDBACK_SENT: 'ai:feedback:sent',
    
    // UI 事件
    UI_MODAL_OPEN: 'ui:modal:open',
    UI_MODAL_CLOSE: 'ui:modal:close',
    UI_PROGRESS_UPDATE: 'ui:progress:update',
    
    // 系统事件
    SYSTEM_READY: 'system:ready',
    SYSTEM_ERROR: 'system:error',
};

// 导出
window.EventBus = eventBus;
window.Events = Events;

logger.log('info', 'event-bus', '✅ 事件总线模块已加载');
