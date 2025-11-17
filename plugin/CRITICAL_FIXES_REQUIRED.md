# 🚨 CRITICAL FIXES REQUIRED - AI 版本

## 问题总结

当前 AI 版本存在以下严重问题：

1. ❌ **所有 console.log 必须替换为 pixlyLog + LOG 键名**
2. ❌ **Eagle 文件选择功能未正确实现**
3. ❌ **缺少完整的生命周期管理**
4. ❌ **缺少日志常量定义**
5. ❌ **缺少 formatLog 函数**

---

## 必须实现的核心功能

### 1. 日志系统（最高优先级）

#### 创建 `plugin/ai/js/log-constants.js`

```javascript
/**
 * AI 版本日志常量
 * 所有日志必须使用键名，不允许硬编码字符串
 */
const LOG = {
    // === 初始化 ===
    AI_INIT_START: 'AI Core initializing...',
    AI_INIT_COMPLETE: 'AI Core initialized successfully',
    AI_I18N_LOADED: 'i18n system loaded: {locale}',
    AI_THEME_LOADED: 'Theme system loaded: {theme}',
    
    // === Eagle 集成 ===
    AI_EAGLE_CREATED: 'Eagle plugin created',
    AI_EAGLE_SHOWN: 'Eagle plugin shown',
    AI_EAGLE_HIDDEN: 'Eagle plugin hidden',
    AI_EAGLE_EXIT: 'Eagle plugin exiting',
    AI_EAGLE_DEBOUNCED: 'Debounced: skipping duplicate onShow call',
    AI_EAGLE_CONVERTING: 'Conversion in progress, skipping file refresh',
    AI_EAGLE_REFRESHED: 'File list auto-refreshed',
    
    // === 文件选择 ===
    AI_FILE_SELECTION_START: 'Starting file selection...',
    AI_FILE_SELECTION_COMPLETE: 'File selection completed: {count} files in {time}ms',
    AI_FILE_SELECTED: 'Selected {count} files from Eagle',
    AI_FILE_NO_SELECTION: 'No files selected in Eagle',
    AI_FILE_MEDIA_TYPE_DETECTED: 'Media type detected: {types}',
    AI_FILE_TRANSPARENCY_DETECTED: 'Transparency detected in {count} files',
    AI_FILE_ANIMATION_DETECTED: 'Animation detected in {count} files',
    AI_FILE_VIDEO_DETECTED: 'Video detected in {count} files',
    AI_FILE_NON_STANDARD_DETECTED: 'Non-standard format detected: {formats}',
    
    // === AI 推荐 ===
    AI_RECOMMEND_GENERATING: 'Generating AI recommendations...',
    AI_RECOMMEND_COMPLETE: 'Generated {count} recommendations',
    AI_RECOMMEND_FORMAT: 'Recommended format: {format} for {type}',
    AI_RECOMMEND_QUALITY: 'Recommended quality: {quality}',
    AI_RECOMMEND_PRESET: 'Using preset: {preset}',
    
    // === 转换 ===
    AI_CONV_START: 'Starting AI-driven conversion...',
    AI_CONV_FILE_PROCESSING: '[{current}/{total}] Processing: {file}',
    AI_CONV_FILE_SUCCESS: 'File converted successfully: {file}',
    AI_CONV_FILE_FAILED: 'File conversion failed: {file} - {error}',
    AI_CONV_COMPLETE: 'Conversion complete: {success}/{total} successful',
    AI_CONV_CANCELLED: 'Conversion cancelled by user',
    
    // === Rust 核心 ===
    AI_RUST_DETECTING: 'Detecting Rust core...',
    AI_RUST_DETECTED: 'Rust core detected: {version}',
    AI_RUST_NOT_FOUND: 'Rust core not found',
    AI_RUST_CALL: 'Calling Rust CLI: {command}',
    AI_RUST_SUCCESS: 'Rust CLI call successful',
    AI_RUST_FAILED: 'Rust CLI call failed: {error}',
    
    // === UI 更新 ===
    AI_UI_FILE_LIST_UPDATED: 'File list UI updated: {count} files',
    AI_UI_PRESET_CHANGED: 'Preset changed: {preset}',
    AI_UI_FEATURE_TOGGLED: 'Feature toggled: {feature} = {status}',
    AI_UI_RECOMMENDATION_SHOWN: 'Recommendation panel shown',
    AI_UI_AUTO_FEATURES_SHOWN: 'Auto-features hint shown',
    
    // === 错误 ===
    AI_ERROR_INIT: 'Initialization error: {error}',
    AI_ERROR_FILE_SELECTION: 'File selection error: {error}',
    AI_ERROR_CONVERSION: 'Conversion error: {error}',
    AI_ERROR_RUST_CLI: 'Rust CLI error: {error}'
};

/**
 * 格式化日志消息
 */
function formatLog(template, params = {}) {
    if (!params || typeof params !== 'object') {
        return template;
    }
    
    return template.replace(/\{(\w+)\}/g, (match, key) => {
        return params.hasOwnProperty(key) ? params[key] : match;
    });
}

// 导出
if (typeof window !== 'undefined') {
    window.LOG = LOG;
    window.formatLog = formatLog;
}
```

### 2. Eagle 生命周期管理

#### 创建 `plugin/ai/js/eagle-lifecycle.js`

```javascript
/**
 * Eagle 生命周期管理
 * 完全参考旧版本实现
 */
(function(window) {
    'use strict';
    
    const EagleLifecycle = {
        _lastOnShowTime: 0,
        
        onCreate: function(plugin) {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY AI', formatLog(LOG.AI_EAGLE_CREATED, {}));
            }
            
            // 保存 plugin 实例
            window.pluginInstance = plugin;
            
            // 初始化组件
            this.initializeComponents();
        },
        
        onShow: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY AI', formatLog(LOG.AI_EAGLE_SHOWN, {}));
            }
            
            // 防抖：500ms 内只执行一次
            const now = Date.now();
            if (now - this._lastOnShowTime < 500) {
                if (log) {
                    log.debug('PIXLY AI', formatLog(LOG.AI_EAGLE_DEBOUNCED, {}));
                }
                return;
            }
            this._lastOnShowTime = now;
            
            // 转换中：跳过刷新
            if (window.PIXLY_CONVERTING) {
                if (log) {
                    log.warn('PIXLY AI', formatLog(LOG.AI_EAGLE_CONVERTING, {}));
                }
                return;
            }
            
            // 刷新文件列表
            if (window.selectFiles) {
                setTimeout(() => {
                    window.selectFiles();
                    if (log) {
                        log.info('PIXLY AI', formatLog(LOG.AI_EAGLE_REFRESHED, {}));
                    }
                }, 200);
            }
        },
        
        onHide: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY AI', formatLog(LOG.AI_EAGLE_HIDDEN, {}));
            }
        },
        
        onBeforeExit: function() {
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY AI', formatLog(LOG.AI_EAGLE_EXIT, {}));
            }
            
            // 清理转换进程
            if (window.conversionProcess) {
                try {
                    window.conversionProcess.kill('SIGTERM');
                } catch (e) {
                    // 忽略错误
                }
            }
        },
        
        initializeComponents: function() {
            // 初始化主题
            if (window.initTheme) {
                window.initTheme();
            }
            
            // 初始化 i18n
            if (window.i18n && window.i18n.init) {
                window.i18n.init();
            }
        },
        
        register: function() {
            if (typeof eagle === 'undefined') {
                return;
            }
            
            eagle.onPluginCreate((plugin) => this.onCreate(plugin));
            eagle.onPluginShow(() => this.onShow());
            eagle.onPluginHide(() => this.onHide());
            eagle.onPluginBeforeExit(() => this.onBeforeExit());
        }
    };
    
    // 暴露到全局
    window.EagleLifecycle = EagleLifecycle;
    
    // 自动注册
    if (typeof eagle !== 'undefined') {
        EagleLifecycle.register();
    }
    
})(window);
```

### 3. 文件选择器

#### 创建 `plugin/ai/js/file-handler.js`

```javascript
/**
 * 文件处理器 - AI 版本
 * 完全参考旧版本实现
 */
(function(window) {
    'use strict';
    
    const FileHandler = {
        
        /**
         * 从 Eagle 选择文件
         */
        selectFiles: async function() {
            const log = window.pixlyLog;
            if (!log) {
                throw new Error('pixlyLog is required');
            }
            
            log.debug('PIXLY AI', formatLog(LOG.AI_FILE_SELECTION_START, {}));
            const startTime = performance.now();
            
            // 检查转换状态
            if (window.PIXLY_CONVERTING === true) {
                log.warn('PIXLY AI', formatLog(LOG.AI_EAGLE_CONVERTING, {}));
                return;
            }
            
            try {
                // 检查 Rust 核心
                if (!window.rustCLI || !window.rustCLI.available) {
                    log.error('PIXLY AI', formatLog(LOG.AI_RUST_NOT_FOUND, {}));
                    return;
                }
                
                // 调用 Eagle API
                const eagleSelection = await eagle.item.getSelected();
                
                if (!eagleSelection || eagleSelection.length === 0) {
                    log.debug('PIXLY AI', formatLog(LOG.AI_FILE_NO_SELECTION, {}));
                    this.setSelectedFiles([]);
                    return;
                }
                
                log.info('PIXLY AI', formatLog(LOG.AI_FILE_SELECTED, { count: eagleSelection.length }));
                
                // 过滤支持的格式
                const supportedExts = [
                    // 图像
                    'jpg', 'jpeg', 'png', 'bmp', 'tiff', 'heic', 'avif', 'jxl',
                    // 动图
                    'gif', 'apng', 'webp',
                    // 视频
                    'mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', '3gp',
                    // RAW
                    'psd', 'cr2', 'nef', 'arw', 'dng', 'orf'
                ];
                
                const filteredFiles = eagleSelection.filter(item => {
                    const ext = (item.ext || '').toLowerCase().replace(/^\./, '');
                    return supportedExts.includes(ext);
                }).map(item => ({
                    id: item.id,
                    name: item.name,
                    ext: item.ext,
                    filePath: item.filePath || null,
                    size: item.size || 0,
                    width: item.width || 0,
                    height: item.height || 0,
                    isAnimated: item.isAnimated || false,
                    tags: item.tags || [],
                    folders: item.folders || []
                }));
                
                // 更新选中状态
                this.setSelectedFiles(filteredFiles);
                
                // 更新 UI
                this.updateUI(filteredFiles);
                
                // 分析媒体类型
                await this.analyzeMediaType(filteredFiles);
                
                const elapsed = (performance.now() - startTime).toFixed(2);
                log.info('PIXLY AI', formatLog(LOG.AI_FILE_SELECTION_COMPLETE, { 
                    count: filteredFiles.length, 
                    time: elapsed 
                }));
                
                return filteredFiles;
                
            } catch (error) {
                log.error('PIXLY AI', formatLog(LOG.AI_ERROR_FILE_SELECTION, { error: error.message }));
                this.setSelectedFiles([]);
                this.updateUI([]);
                throw error;
            }
        },
        
        /**
         * 设置选中的文件
         */
        setSelectedFiles: function(files) {
            window.selectedFiles = files;
            
            // 启用/禁用转换按钮
            const convertBtn = document.getElementById('convertBtn');
            if (convertBtn) {
                convertBtn.disabled = files.length === 0;
            }
        },
        
        /**
         * 更新 UI
         */
        updateUI: function(files) {
            const log = window.pixlyLog;
            
            // 更新文件计数
            const fileCount = document.getElementById('fileCount');
            if (fileCount) {
                fileCount.textContent = files.length;
            }
            
            // 显示/隐藏面板
            const fileArea = document.getElementById('fileSelectionArea');
            const fileList = document.getElementById('fileList');
            const aiPanel = document.getElementById('aiPanel');
            const autoFeaturesHint = document.getElementById('autoFeaturesHint');
            
            if (files.length === 0) {
                if (fileArea) fileArea.style.display = 'block';
                if (fileList) fileList.style.display = 'none';
                if (aiPanel) aiPanel.style.display = 'none';
                if (autoFeaturesHint) autoFeaturesHint.style.display = 'none';
                return;
            }
            
            if (fileArea) fileArea.style.display = 'none';
            if (fileList) fileList.style.display = 'block';
            if (aiPanel) aiPanel.style.display = 'block';
            if (autoFeaturesHint) autoFeaturesHint.style.display = 'block';
            
            // 渲染文件列表
            const fileItems = document.getElementById('fileItems');
            if (fileItems) {
                fileItems.innerHTML = files.map(file => `
                    <div class="file-item">
                        <div class="file-icon">${this.getFileIcon(file.ext)}</div>
                        <div class="file-info">
                            <div class="file-name">${file.name}</div>
                            <div class="file-meta">${file.ext.toUpperCase()} · ${this.formatFileSize(file.size)}</div>
                        </div>
                    </div>
                `).join('');
            }
            
            if (log) {
                log.debug('PIXLY AI', formatLog(LOG.AI_UI_FILE_LIST_UPDATED, { count: files.length }));
            }
        },
        
        /**
         * 分析媒体类型
         */
        analyzeMediaType: async function(files) {
            const log = window.pixlyLog;
            const types = new Set();
            const details = {
                hasTransparency: false,
                hasAnimation: false,
                isVideo: false,
                nonStandardFormats: []
            };
            
            files.forEach(file => {
                const ext = (file.ext || '').toLowerCase().replace(/^\./, '');
                
                // 静态图像
                if (['jpg', 'jpeg', 'png', 'bmp', 'tiff', 'heic', 'avif', 'jxl'].includes(ext)) {
                    types.add('image');
                    if (['png', 'webp', 'heic', 'avif', 'jxl'].includes(ext)) {
                        types.add('transparent');
                        details.hasTransparency = true;
                    }
                }
                
                // 动图
                if (['gif', 'apng'].includes(ext) || (ext === 'webp' && file.isAnimated)) {
                    types.add('animation');
                    details.hasAnimation = true;
                }
                
                // 视频
                if (['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', '3gp'].includes(ext)) {
                    types.add('video');
                    details.isVideo = true;
                }
                
                // 非标准格式
                if (['psd', 'cr2', 'nef', 'arw', 'dng', 'orf'].includes(ext)) {
                    types.add('non-standard');
                    details.nonStandardFormats.push(ext.toUpperCase());
                }
            });
            
            window.mediaType = Array.from(types);
            window.mediaDetails = details;
            
            if (log) {
                log.info('PIXLY AI', formatLog(LOG.AI_FILE_MEDIA_TYPE_DETECTED, { 
                    types: Array.from(types).join(', ') 
                }));
                
                if (details.hasTransparency) {
                    const count = files.filter(f => ['png', 'webp', 'heic', 'avif', 'jxl'].includes(
                        (f.ext || '').toLowerCase().replace(/^\./, '')
                    )).length;
                    log.info('PIXLY AI', formatLog(LOG.AI_FILE_TRANSPARENCY_DETECTED, { count }));
                }
                
                if (details.hasAnimation) {
                    const count = files.filter(f => ['gif', 'apng'].includes(
                        (f.ext || '').toLowerCase().replace(/^\./, '')
                    )).length;
                    log.info('PIXLY AI', formatLog(LOG.AI_FILE_ANIMATION_DETECTED, { count }));
                }
                
                if (details.nonStandardFormats.length > 0) {
                    log.info('PIXLY AI', formatLog(LOG.AI_FILE_NON_STANDARD_DETECTED, { 
                        formats: details.nonStandardFormats.join(', ') 
                    }));
                }
            }
            
            // 更新媒体类型显示
            this.updateMediaTypeDisplay();
            
            // 生成 AI 推荐
            this.generateRecommendations();
        },
        
        /**
         * 更新媒体类型显示
         */
        updateMediaTypeDisplay: function() {
            const display = document.getElementById('mediaTypeDisplay');
            if (!display) return;
            
            const types = window.mediaType || [];
            if (types.length === 0) {
                display.innerHTML = '<span class="type-badge" data-i18n="media_type_detecting">自动识别中...</span>';
                return;
            }
            
            const badges = {
                'image': 'media_type_image',
                'transparent': 'media_type_transparent',
                'animation': 'media_type_animation',
                'video': 'media_type_video',
                'non-standard': 'media_type_mixed'
            };
            
            display.innerHTML = types.map(type => {
                const key = badges[type] || 'media_type_mixed';
                const text = window.i18n ? window.i18n.getMessage(key) : type;
                return `<span class="type-badge" data-i18n="${key}">${text}</span>`;
            }).join('');
        },
        
        /**
         * 生成 AI 推荐
         */
        generateRecommendations: function() {
            const log = window.pixlyLog;
            if (log) {
                log.debug('PIXLY AI', formatLog(LOG.AI_RECOMMEND_GENERATING, {}));
            }
            
            const types = window.mediaType || [];
            const details = window.mediaDetails || {};
            const recs = [];
            
            // 基于媒体类型生成推荐
            if (types.includes('image')) {
                recs.push({
                    icon: '📦',
                    text: window.i18n ? window.i18n.getMessage('recommendation_static_image') : 
                          '静态图像推荐使用 JXL 格式'
                });
            }
            
            if (types.includes('transparent')) {
                recs.push({
                    icon: '🔲',
                    text: window.i18n ? window.i18n.getMessage('recommendation_transparent_image') : 
                          '透明图像将自动保留 Alpha 通道'
                });
            }
            
            if (types.includes('animation')) {
                recs.push({
                    icon: '🎞️',
                    text: window.i18n ? window.i18n.getMessage('recommendation_animation') : 
                          '动态图像推荐使用 AVIF 格式'
                });
            }
            
            if (types.includes('video')) {
                recs.push({
                    icon: '⚡',
                    text: window.i18n ? window.i18n.getMessage('recommendation_video') : 
                          '视频将使用 GPU 加速编码'
                });
            }
            
            // 显示推荐
            const recommendSection = document.getElementById('aiRecommendation');
            const recommendContent = document.getElementById('recommendContent');
            
            if (recommendSection && recommendContent) {
                if (recs.length > 0) {
                    recommendSection.style.display = 'block';
                    recommendContent.innerHTML = recs.map(rec => `
                        <div class="recommend-item">
                            <div class="recommend-item-icon">${rec.icon}</div>
                            <div class="recommend-item-text">${rec.text}</div>
                        </div>
                    `).join('');
                    
                    if (log) {
                        log.info('PIXLY AI', formatLog(LOG.AI_RECOMMEND_COMPLETE, { count: recs.length }));
                    }
                } else {
                    recommendSection.style.display = 'none';
                }
            }
        },
        
        /**
         * 获取文件图标
         */
        getFileIcon: function(ext) {
            const iconMap = {
                'jpg': '🖼️', 'jpeg': '🖼️', 'png': '🖼️',
                'gif': '🎞️', 'webp': '🎨', 'avif': '🎨',
                'jxl': '✨', 'heic': '📸', 'heif': '📸',
                'mp4': '🎬', 'mov': '🎬', 'avi': '🎬',
                'mkv': '🎬', 'webm': '🎬'
            };
            const cleanExt = (ext || '').toLowerCase().replace(/^\./, '');
            return iconMap[cleanExt] || '📄';
        },
        
        /**
         * 格式化文件大小
         */
        formatFileSize: function(bytes) {
            if (bytes === 0) return '0 B';
            
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            
            return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
        }
    };
    
    // 暴露到全局
    window.FileHandler = FileHandler;
    window.selectFiles = FileHandler.selectFiles.bind(FileHandler);
    
})(window);
```

---

## 必须修改的文件

### `plugin/ai/js/ai-core.js`

**所有 console.log 必须替换为**:

```javascript
// ❌ 错误
console.log('[AI Core] Initializing...');

// ✅ 正确
const log = window.pixlyLog;
if (log) {
    log.info('PIXLY AI', formatLog(LOG.AI_INIT_START, {}));
}
```

### `plugin/ai/index.html`

**必须添加脚本引用**:

```html
<!-- 日志常量（必须最先加载） -->
<script src="js/log-constants.js"></script>

<!-- Eagle 生命周期 -->
<script src="js/eagle-lifecycle.js"></script>

<!-- 文件处理器 -->
<script src="js/file-handler.js"></script>

<!-- 国际化系统 -->
<script src="js/i18n.js" type="module"></script>

<!-- AI 核心脚本 -->
<script src="js/ai-core.js" type="module"></script>
```

---

## 验证清单

- [ ] 所有 console.log 已替换为 pixlyLog + LOG 键名
- [ ] 创建了 log-constants.js
- [ ] 创建了 eagle-lifecycle.js
- [ ] 创建了 file-handler.js
- [ ] Eagle 文件选择功能正常工作
- [ ] 媒体类型检测正常工作
- [ ] AI 推荐生成正常工作
- [ ] 所有日志都有对应的 LOG 键名
- [ ] formatLog 函数正常工作

---

## 参考文件

必须完全参考旧版本的实现：

1. `plugin/old/converter/js/plugin-modules/log-constants.js` - 日志常量
2. `plugin/old/converter/js/plugin-modules/eagle-lifecycle.js` - Eagle 生命周期
3. `plugin/old/converter/js/plugin-modules/file-handler.js` - 文件处理器

**不要复制粘贴，要理解并适配到 AI 版本！**

---

**状态**: 🚨 CRITICAL - 必须立即修复
**优先级**: P0 - 最高优先级
**影响**: 所有日志和 Eagle 集成功能
