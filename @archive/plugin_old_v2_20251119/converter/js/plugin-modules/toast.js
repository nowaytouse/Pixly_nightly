/**
 * PIXLY Toast notificationsystem
 * 独立module - 无外部依赖
 */
(function(window) {
    'use strict';
    
    // 确保PIXLY命名空间exists
    window.PIXLY = window.PIXLY || {};
    
    // Toast类型枚举
    const ToastType = {
        SUCCESS: 'success',
        ERROR: 'error',
        WARNING: 'warning',
        INFO: 'info'
    };
    
    /**
     * 显示Toastnotification
     */
    function showToast(title, message, type = ToastType.INFO, duration = 3000) {
        const container = document.getElementById('toastContainer');
        if (!container) return;
        
        // create toast elements
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        // 图标映射
        const icons = {
            success: '✅',
            error: '❌',
            warning: '⚠️',
            info: 'ℹ️'
        };
        
        toast.innerHTML = `
            <div class="toast-icon">${icons[type] || icons.info}</div>
            <div class="toast-content">
                <div class="toast-title">${title}</div>
                ${message ? `<div class="toast-message">${message}</div>` : ''}
            </div>
            <button class="toast-close">×</button>
        `;
        
        container.appendChild(toast);
        
        // 关闭按钮事件
        const closeBtn = toast.querySelector('.toast-close');
        closeBtn.addEventListener('click', () => removeToast(toast));
        
        // 动画显示
        setTimeout(() => toast.classList.add('show'), 10);
        
        // 自动关闭
        if (duration > 0) {
            setTimeout(() => removeToast(toast), duration);
        }
    }
    
    /**
     * 移除Toast
     */
    function removeToast(toast) {
        toast.classList.add('hide');
        setTimeout(() => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, 300);
    }
    
    // 暴露to全局
    PIXLY.Toast = {
        show: showToast,
        success: (title, message, duration) => showToast(title, message, ToastType.SUCCESS, duration),
        error: (title, message, duration) => showToast(title, message, ToastType.ERROR, duration),
        warning: (title, message, duration) => showToast(title, message, ToastType.WARNING, duration),
        info: (title, message, duration) => showToast(title, message, ToastType.INFO, duration),
        Type: ToastType
    };
    
    // 兼容Old代码
    window.showToast = showToast;
    window.ToastType = ToastType;
    
 const log = window.pixlyLog;
if (log) {
    log.info('PIXLY Toast', formatLog(LOG.TOAST_MODULE_LOADED, {}));
}
    
})(window);
