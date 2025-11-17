/**
 * Scope Filter Manager - 范围过滤管理模块
 * 负责管理文件过滤功能（尺寸、格式、文件大小等）
 * 从ui-handlers.js拆分
 */

export class ScopeFilterManager {
    constructor() {
        this.log = window.pixlyLog || console;
        this.enabled = false;
        this.filters = {
            minWidth: 0,
            minHeight: 0,
            maxFileSize: 0,
            allowedFormats: new Set()
        };
    }

    /**
     * 初始化范围过滤管理器
     */
    initialize() {
        this.bindEvents();
        this.loadSavedSettings();
        this.log.info('[ScopeFilterManager] ✅ Initialized');
    }

    /**
     * 绑定事件
     */
    bindEvents() {
        // 主开关
        const enableScopeFilter = document.getElementById('enableScopeFilter');
        if (enableScopeFilter) {
            enableScopeFilter.addEventListener('change', (e) => {
                this.enabled = e.target.checked;
                this.toggleFilterOptions(this.enabled);
                this.saveSettings();
                
                this.log.info(`[ScopeFilterManager] ${this.enabled ? 'Enabled' : 'Disabled'}`);
                
                // 触发文件列表刷新
                if (window.selectFiles) {
                    setTimeout(() => window.selectFiles(), 100);
                }
            });
        }

        // 最小尺寸输入
        const minWidthInput = document.getElementById('filterMinWidth');
        const minHeightInput = document.getElementById('filterMinHeight');
        
        if (minWidthInput) {
            minWidthInput.addEventListener('change', (e) => {
                this.filters.minWidth = parseInt(e.target.value) || 0;
                this.saveSettings();
                this.applyFilters();
            });
        }
        
        if (minHeightInput) {
            minHeightInput.addEventListener('change', (e) => {
                this.filters.minHeight = parseInt(e.target.value) || 0;
                this.saveSettings();
                this.applyFilters();
            });
        }

        // 最大文件大小
        const maxSizeInput = document.getElementById('filterMaxSize');
        const maxSizeUnit = document.getElementById('filterMaxSizeUnit');
        
        if (maxSizeInput && maxSizeUnit) {
            const updateMaxSize = () => {
                const value = parseFloat(maxSizeInput.value) || 0;
                const unit = maxSizeUnit.value;
                const multiplier = unit === 'MB' ? 1024 * 1024 : 1024; // MB or KB
                this.filters.maxFileSize = value * multiplier;
                this.saveSettings();
                this.applyFilters();
            };
            
            maxSizeInput.addEventListener('change', updateMaxSize);
            maxSizeUnit.addEventListener('change', updateMaxSize);
        }

        // 格式过滤
        const formatCheckboxes = document.querySelectorAll('.filter-format-checkbox');
        formatCheckboxes.forEach(checkbox => {
            checkbox.addEventListener('change', (e) => {
                const format = e.target.value;
                if (e.target.checked) {
                    this.filters.allowedFormats.add(format);
                } else {
                    this.filters.allowedFormats.delete(format);
                }
                this.saveSettings();
                this.applyFilters();
            });
        });
    }

    /**
     * 切换过滤选项显示
     */
    toggleFilterOptions(show) {
        const filterOptions = document.getElementById('scopeFilterOptions');
        if (filterOptions) {
            filterOptions.style.display = show ? 'block' : 'none';
        }
    }

    /**
     * 检查文件是否通过过滤
     */
    checkFile(file, imageInfo = null) {
        // 如果未启用，全部通过
        if (!this.enabled) {
            return { pass: true, reason: '' };
        }

        // 检查文件格式
        if (this.filters.allowedFormats.size > 0) {
            const ext = (file.ext || '').toLowerCase().replace('.', '');
            if (!this.filters.allowedFormats.has(ext)) {
                return { 
                    pass: false, 
                    reason: `格式 ${ext} 不在允许范围内` 
                };
            }
        }

        // 检查文件大小
        if (this.filters.maxFileSize > 0) {
            const fileSize = file.size || 0;
            if (fileSize > this.filters.maxFileSize) {
                const sizeMB = (fileSize / (1024 * 1024)).toFixed(2);
                const maxMB = (this.filters.maxFileSize / (1024 * 1024)).toFixed(2);
                return { 
                    pass: false, 
                    reason: `文件大小 ${sizeMB}MB 超过限制 ${maxMB}MB` 
                };
            }
        }

        // 检查图像尺寸
        if (imageInfo) {
            if (this.filters.minWidth > 0 && imageInfo.width < this.filters.minWidth) {
                return { 
                    pass: false, 
                    reason: `宽度 ${imageInfo.width}px 小于最小值 ${this.filters.minWidth}px` 
                };
            }
            
            if (this.filters.minHeight > 0 && imageInfo.height < this.filters.minHeight) {
                return { 
                    pass: false, 
                    reason: `高度 ${imageInfo.height}px 小于最小值 ${this.filters.minHeight}px` 
                };
            }
        }

        return { pass: true, reason: '' };
    }

    /**
     * 应用过滤器
     */
    applyFilters() {
        if (!this.enabled) return;

        // 触发文件列表更新
        if (window.selectFiles) {
            setTimeout(() => {
                window.selectFiles();
                this.updateFilterStatus();
            }, 100);
        }
    }

    /**
     * 更新过滤状态显示
     */
    updateFilterStatus() {
        const statusElement = document.getElementById('filterStatus');
        if (!statusElement) return;

        const files = window.PIXLY_SELECTED_FILES || [];
        const totalFiles = files.length;
        let filteredCount = 0;

        files.forEach(file => {
            const result = this.checkFile(file);
            if (!result.pass) {
                filteredCount++;
            }
        });

        if (filteredCount > 0) {
            statusElement.textContent = `已过滤 ${filteredCount} 个文件`;
            statusElement.style.display = 'block';
        } else {
            statusElement.style.display = 'none';
        }
    }

    /**
     * 保存设置到localStorage
     */
    saveSettings() {
        const settings = {
            enabled: this.enabled,
            filters: {
                minWidth: this.filters.minWidth,
                minHeight: this.filters.minHeight,
                maxFileSize: this.filters.maxFileSize,
                allowedFormats: Array.from(this.filters.allowedFormats)
            }
        };
        
        try {
            localStorage.setItem('pixly_scope_filter_settings', JSON.stringify(settings));
            this.log.debug('[ScopeFilterManager] Settings saved');
        } catch (error) {
            this.log.error('[ScopeFilterManager] Failed to save settings:', error);
        }
    }

    /**
     * 加载保存的设置
     */
    loadSavedSettings() {
        try {
            const saved = localStorage.getItem('pixly_scope_filter_settings');
            if (!saved) return;

            const settings = JSON.parse(saved);
            
            // 应用设置
            this.enabled = settings.enabled || false;
            this.filters.minWidth = settings.filters?.minWidth || 0;
            this.filters.minHeight = settings.filters?.minHeight || 0;
            this.filters.maxFileSize = settings.filters?.maxFileSize || 0;
            this.filters.allowedFormats = new Set(settings.filters?.allowedFormats || []);

            // 更新UI
            this.updateUI();
            
            this.log.debug('[ScopeFilterManager] Settings loaded');
        } catch (error) {
            this.log.error('[ScopeFilterManager] Failed to load settings:', error);
        }
    }

    /**
     * 更新UI显示
     */
    updateUI() {
        // 主开关
        const enableScopeFilter = document.getElementById('enableScopeFilter');
        if (enableScopeFilter) {
            enableScopeFilter.checked = this.enabled;
        }

        // 选项显示
        this.toggleFilterOptions(this.enabled);

        // 尺寸输入
        const minWidthInput = document.getElementById('filterMinWidth');
        if (minWidthInput && this.filters.minWidth > 0) {
            minWidthInput.value = this.filters.minWidth;
        }

        const minHeightInput = document.getElementById('filterMinHeight');
        if (minHeightInput && this.filters.minHeight > 0) {
            minHeightInput.value = this.filters.minHeight;
        }

        // 文件大小
        if (this.filters.maxFileSize > 0) {
            const maxSizeInput = document.getElementById('filterMaxSize');
            const maxSizeUnit = document.getElementById('filterMaxSizeUnit');
            
            if (maxSizeInput && maxSizeUnit) {
                if (this.filters.maxFileSize >= 1024 * 1024) {
                    maxSizeInput.value = (this.filters.maxFileSize / (1024 * 1024)).toFixed(2);
                    maxSizeUnit.value = 'MB';
                } else {
                    maxSizeInput.value = (this.filters.maxFileSize / 1024).toFixed(2);
                    maxSizeUnit.value = 'KB';
                }
            }
        }

        // 格式复选框
        this.filters.allowedFormats.forEach(format => {
            const checkbox = document.querySelector(`.filter-format-checkbox[value="${format}"]`);
            if (checkbox) {
                checkbox.checked = true;
            }
        });
    }

    /**
     * 重置所有过滤器
     */
    reset() {
        this.enabled = false;
        this.filters = {
            minWidth: 0,
            minHeight: 0,
            maxFileSize: 0,
            allowedFormats: new Set()
        };
        
        this.saveSettings();
        this.updateUI();
        this.applyFilters();
        
        this.log.info('[ScopeFilterManager] Filters reset');
    }

    /**
     * 获取当前过滤器状态
     */
    getStatus() {
        return {
            enabled: this.enabled,
            filters: { ...this.filters }
        };
    }
}

// 创建单例并导出
const scopeFilterManager = new ScopeFilterManager();

// 导出全局函数供其他模块使用
window.checkScopeFilter = (file, imageInfo) => {
    return scopeFilterManager.checkFile(file, imageInfo);
};

export default scopeFilterManager;
