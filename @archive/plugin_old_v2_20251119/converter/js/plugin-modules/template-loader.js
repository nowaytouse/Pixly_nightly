// PIXLY Template Loader - Last Modified: 2025-11-10 14:15:24
// Version: 3.0.0-fixed

/**
 * 🎨 Phase 41: HTML Template Loader
 * 使用fetch API动态加载HTML模板
 * 参考：https://developer.eagle.cool/plugin-api/zh-cn/tutorial/network-request
 */

(function() {
'use strict';

// Log instance for the module
const log = window.pixlyLog;

class TemplateLoader {
    constructor() {
        this.templates = new Map();
        this.basePath = 'templates/';
        if (log) {
            log.info('PIXLY Template Loader', formatLog(LOG.TEMPLATE_LOADER_INIT, {}));
        }
    }
    
    /**
     * 加载单个模板
     */
    async loadTemplate(templateName) {
        if (this.templates.has(templateName)) {
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_ALREADY_LOADED, { template: templateName }));
            }
            return this.templates.get(templateName);
        }
        
        const url = `${this.basePath}${templateName}.html`;
        if (log) {
            log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_LOADING, { url }));
        }
        
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const html = await response.text();
            this.templates.set(templateName, html);
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_LOADED, { template: templateName, size: html.length }));
            }
            return html;
        } catch (error) {
            if (log) {
                log.error('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_LOAD_FAILED, { template: templateName, error: error.message || error }));
            }
            throw error;
        }
    }
    
    /**
     * 加载并注入模板到指定容器
     */
    async loadAndInject(templateName, containerId) {
        const container = document.getElementById(containerId);
        if (!container) {
            throw new Error(`Container not found: ${containerId}`);
        }
        
        const html = await this.loadTemplate(templateName);
        container.innerHTML = html;
        if (log) {
            log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_INJECTED, { template: templateName, container: containerId }));
        }
    }
    
    /**
     * 批量加载所有模板
     */
    async loadAll() {
        if (log) {
            log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_LOADING_ALL, {}));
        }
        
        const startTime = performance.now();
        
        try {
            // 🔥 Phase 46.5: 分批加载 - 先加载外层容器，再加载内层面板
            
            // 第一批：外层模板（并行加载）
            const outerTemplates = [
                { name: 'header', container: 'header-container' },
                { name: 'file-selection', container: 'file-selection-container' },
                { name: 'empty-state', container: 'empty-state-container' },
                { name: 'progress-section', container: 'progress-section-container' },
                { name: 'main-panel', container: 'main-panel-container' }
            ];
            
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_OUTER_START, {}));
            }
            await Promise.all(
                outerTemplates.map(t => this.loadAndInject(t.name, t.container))
            );
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_OUTER_COMPLETE, {}));
            }
            
            // 第二批：内层面板（并行加载，此时容器已存在）
            const innerTemplates = [
                { name: 'image-panel', container: 'image-panel-container' },
                { name: 'video-panel', container: 'video-panel-container' }
            ];
            
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_INNER_START, {}));
            }
            await Promise.all(
                innerTemplates.map(t => this.loadAndInject(t.name, t.container))
            );
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_INNER_COMPLETE, {}));
            }
            
            // 第三批：模态窗口（追加到body末尾）
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_MODALS_START, {}));
            }
            const modalsHtml = await this.loadTemplate('modals');
            document.body.insertAdjacentHTML('beforeend', modalsHtml);
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_MODALS_COMPLETE, {}));
            }
            
            const duration = (performance.now() - startTime).toFixed(2);
            if (log) {
                log.info('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_ALL_COMPLETE, { count: 8, duration }));
            }
            
            // 触发自定义事件，通知模板加载完成
            document.dispatchEvent(new CustomEvent('pixlyTemplatesLoaded'));
            
        } catch (error) {
            if (log) {
                log.error('PIXLY Template', formatLog(LOG.TEMPLATE_LOADER_ALL_FAILED, { error: error.message || error }));
            }
            throw error;
        }
    }
}

// 创建全局实例
window.pixlyTemplateLoader = new TemplateLoader();

if (log) {
    log.info('PIXLY Template Loader', formatLog(LOG.TEMPLATE_LOADER_MODULE_LOADED, {}));
}

})(); // End of IIFE
