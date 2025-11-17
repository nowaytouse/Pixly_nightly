/**
 * PIXLY 工具函数集
 * 独立module - 通用工具函数
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * 格式化file大小
     */
    function formatFileSize(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
    
    /**
     * 格式化持续时间
     */
    function formatDuration(ms) {
        if (ms < 1000) return `${Math.round(ms)}ms`;
        const seconds = ms / 1000;
        if (seconds < 60) return `${seconds.toFixed(1)}s`;
        const minutes = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${minutes}m ${secs}s`;
    }
    
    /**
     * 格式化日期时间
     */
    function formatDateTime(date) {
        if (!(date instanceof Date)) {
            date = new Date(date);
        }
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        const seconds = String(date.getSeconds()).padStart(2, '0');
        return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
    }
    
    /**
     * 清理file名（移除非法字符）
     */
    function sanitizeFilename(filename) {
        return filename.replace(/[<>:"/\\|?*]/g, '_');
    }
    
    /**
     * 转义HTML特殊字符
     */
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    /**
     * 解析查询parameter
     */
    function parseQueryString(url) {
        const params = {};
        const queryString = url.split('?')[1];
        if (!queryString) return params;
        
        queryString.split('&').forEach(param => {
            const [key, value] = param.split('=');
            params[decodeURIComponent(key)] = decodeURIComponent(value || '');
        });
        return params;
    }
    
    /**
     * 深拷贝object
     */
    function deepClone(obj) {
        if (obj === null || typeof obj !== 'object') return obj;
        if (obj instanceof Date) return new Date(obj.getTime());
        if (obj instanceof Array) return obj.map(item => deepClone(item));
        
        const clonedObj = {};
        for (const key in obj) {
            if (obj.hasOwnProperty(key)) {
                clonedObj[key] = deepClone(obj[key]);
            }
        }
        return clonedObj;
    }
    
    /**
     * 🔥 Phase 40.33: Tooltip 系统
     */
    let tooltipElement = null;
    
    function showTooltip(target, text) {
        // 移除旧 tooltip
        hideTooltip();
        
        // 创建 tooltip 元素
        tooltipElement = document.createElement('div');
        tooltipElement.className = 'pixly-tooltip';
        tooltipElement.textContent = text;
        tooltipElement.style.cssText = `
            position: fixed;
            background: rgba(0, 0, 0, 0.9);
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 12px;
            z-index: 10000;
            pointer-events: none;
            max-width: 300px;
            line-height: 1.4;
            box-shadow: 0 4px 12px rgba(0,0,0,0.3);
        `;
        
        document.body.appendChild(tooltipElement);
        
        // 定位 tooltip
        const rect = target.getBoundingClientRect();
        const tooltipRect = tooltipElement.getBoundingClientRect();
        
        let top = rect.top - tooltipRect.height - 8;
        let left = rect.left + (rect.width - tooltipRect.width) / 2;
        
        // 防止超出屏幕
        if (top < 8) {
            top = rect.bottom + 8;
        }
        if (left < 8) {
            left = 8;
        }
        if (left + tooltipRect.width > window.innerWidth - 8) {
            left = window.innerWidth - tooltipRect.width - 8;
        }
        
        tooltipElement.style.top = top + 'px';
        tooltipElement.style.left = left + 'px';
    }
    
    function hideTooltip() {
        if (tooltipElement) {
            tooltipElement.remove();
            tooltipElement = null;
        }
    }
    
    function initTooltips() {
        // 为所有 data-tooltip 和 data-i18n-tooltip 元素添加事件监听
        document.addEventListener('mouseover', (e) => {
            const target = e.target.closest('[data-tooltip], [data-i18n-tooltip]');
            if (target) {
                // 优先使用i18n翻译的tooltip
                const i18nKey = target.getAttribute('data-i18n-tooltip');
                const i18n = window.i18n || { t: (key) => key };
                
                let text;
                if (i18nKey) {
                    text = i18n.t(i18nKey);
                } else {
                    text = target.getAttribute('data-tooltip');
                }
                
                if (text) {
                    showTooltip(target, text);
                }
            }
        });
        
        document.addEventListener('mouseout', (e) => {
            const target = e.target.closest('[data-tooltip], [data-i18n-tooltip]');
            if (target) {
                hideTooltip();
            }
        });
        
        const log = window.pixlyLog;
        if (log) {
            log.info('PIXLY Utils', formatLog(LOG.UTILS_TOOLTIP_INITIALIZED, {}));
        }
    }
    
    /**
     * 防抖函数
     */
    function debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }
    
    /**
     * 节流函数
     */
    function throttle(func, limit) {
        let inThrottle;
        return function(...args) {
            if (!inThrottle) {
                func.apply(this, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }
    
    /**
     * 生成唯一ID
     */
    function generateId(prefix = 'pixly') {
        return `${prefix}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }
    
    /**
     * validatefile扩展名
     */
    function validateFileExtension(filename, allowedExtensions) {
        const ext = filename.toLowerCase().split('.').pop();
        return allowedExtensions.includes(ext);
    }
    
    /**
     * 获取file扩展名
     */
    function getFileExtension(filename) {
        return filename.toLowerCase().split('.').pop();
    }
    
    /**
     * 获取file名（不含扩展名）
     */
    function getFileNameWithoutExt(filename) {
        return filename.substring(0, filename.lastIndexOf('.')) || filename;
    }
    
    /**
     * 判断is否as图片file
     */
    function isImageFile(filename) {
        const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'jxl', 'avif', 'heic', 'heif'];
        return validateFileExtension(filename, imageExts);
    }
    
    /**
     * 判断is否as视频file
     */
    function isVideoFile(filename) {
        const videoExts = ['mp4', 'avi', 'mov', 'mkv', 'flv', 'wmv', 'webm', 'm4v', 'mpg', 'mpeg'];
        return validateFileExtension(filename, videoExts);
    }
    
    /**
     * 判断is否as动图
     */
    function isAnimatedImage(filename) {
        const ext = getFileExtension(filename);
        return ext === 'gif' || ext === 'apng' || ext === 'webp'; // WebPmayis动图
    }
    
    /**
     * 延迟执lines
     */
    function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
    
    /**
     * retry函数
     */
    async function retry(fn, maxAttempts = 3, delay = 1000) {
        for (let attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                return await fn();
            } catch (error) {
                if (attempt === maxAttempts) throw error;
                const log = window.pixlyLog;
                if (log) {
                    log.warn('Retry', formatLog(LOG.UTILS_RETRY_ATTEMPT, { attempt, maxAttempts, delay }));
                }
                await sleep(delay);
            }
        }
    }
    
    /**
     * calculation百分比
     */
    function calculatePercentage(value, total) {
        if (total === 0) return 0;
        return Math.round((value / total) * 100);
    }
    
    /**
     * 限制数值范围
     */
    function clamp(value, min, max) {
        return Math.min(Math.max(value, min), max);
    }
    
    /**
     * 判断is否is emptyobject
     */
    function isEmptyObject(obj) {
        return Object.keys(obj).length === 0;
    }
    
    /**
     * mergeobject（深度merge）
     */
    function mergeDeep(target, ...sources) {
        if (!sources.length) return target;
        const source = sources.shift();
        
        if (typeof target === 'object' && typeof source === 'object') {
            for (const key in source) {
                if (typeof source[key] === 'object' && !Array.isArray(source[key])) {
                    if (!target[key]) Object.assign(target, { [key]: {} });
                    mergeDeep(target[key], source[key]);
                } else {
                    Object.assign(target, { [key]: source[key] });
                }
            }
        }
        
        return mergeDeep(target, ...sources);
    }
    
    // 导出到全局
    window.PIXLY.Utils = {
        formatFileSize,
        formatDuration,
        formatDateTime,
        sanitizeFilename,
        escapeHtml,
        parseQueryString,
        deepClone,
        debounce,
        throttle,
        generateId,
        sleep,
        retry,
        // isValidEmail,        ❌ 孤儿代码：未定义也未使用
        // isValidUrl,          ❌ 孤儿代码：未定义也未使用
        // truncate,            ❌ 孤儿代码：未定义也未使用
        // capitalizeFirst,     ❌ 孤儿代码：未定义也未使用
        // toCamelCase,         ❌ 孤儿代码：未定义也未使用
        // toSnakeCase,         ❌ 孤儿代码：未定义也未使用
        // objectToQueryString, ❌ 孤儿代码：未定义也未使用
        // 🔥 Phase 40.33: Tooltip API
        showTooltip,
        hideTooltip,
        initTooltips
    };
    
    // 🔥 Phase 40.33: 自动初始化 tooltip
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initTooltips);
    } else {
        initTooltips();
    }

    // 兼容Old代码（直接暴露to全局）
    window.formatFileSize = formatFileSize;
    window.formatDuration = formatDuration;
    window.sanitizeFilename = sanitizeFilename;
    window.escapeHtml = escapeHtml;
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY Utils', formatLog(LOG.UTILS_MODULE_LOADED, {}));
    }
    
})(window);
