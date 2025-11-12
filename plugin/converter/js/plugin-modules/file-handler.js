/**
 * file-handler.js - UI Layer Only
 * 
 * 🔥 文件处理逻辑已迁移到Rust！
 * 
 * 迁移到Rust的功能：
 * - EaglePathResolver::resolve_eagle_path() - Eagle路径解析
 * - FileIconMapper::get_icon() - 文件图标映射
 * 
 * 本文件保留：
 * - Eagle API调用（UI层）
 * - 文件选择状态管理（UI层）
 * - UI事件处理
 * 
 * @see core/rust/src/converter/file_manager.rs
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    /**
     * 文件处理器（UI层）
     * 注意：文件处理逻辑已迁移到Rust，这里只保留UI相关功能
     */
    const fileHandler = {
        // 防抖定时器
        _debounceTimer: null,
        _lastSelectionTime: 0,
        
        // 🚀 性能优化：文件信息缓存 (LRU, 1000项, TTL 60秒)
        _fileInfoCache: null,
        
        /**
         * 初始化文件信息缓存
         */
        _initCache: function() {
            if (!this._fileInfoCache && window.PIXLY && window.PIXLY.PerformanceUtils) {
                this._fileInfoCache = new window.PIXLY.PerformanceUtils.LRUCache(1000, 60000);
            }
        },
        
        /**
         * 获取缓存的文件信息
         */
        _getCachedFileInfo: function(filePath) {
            this._initCache();
            if (this._fileInfoCache) {
                return this._fileInfoCache.get(filePath);
            }
            return null;
        },
        
        /**
         * 缓存文件信息
         */
        _setCachedFileInfo: function(filePath, info) {
            this._initCache();
            if (this._fileInfoCache) {
                this._fileInfoCache.set(filePath, info);
            }
        },
        
        /**
         * 获取当前选中的文件列表
         * @returns {Array} 选中的文件数组
         */
        getSelectedFiles: function() {
            return window.selectedFiles || [];
        },
        
        /**
         * 设置选中的文件列表
         * @param {Array} files - 文件数组
         */
        setSelectedFiles: function(files) {
            window.selectedFiles = files;
            const log = window.pixlyLog;
            if (!log) {
                throw new Error('pixlyLog is required for file-handler module');
            }
            log.info('PIXLY File', formatLog(LOG.FILE_SELECTED_COUNT, { count: files.length }));
        },
        
        /**
         * 从Eagle API选择文件（UI层功能）
         * 
         * 注意：
         * - Eagle路径解析由Rust处理
         * - 文件验证由Rust处理
         * - 这里只负责调用Eagle API和更新UI状态
         */
        selectFiles: async function() {
            const log = window.pixlyLog;
            if (!log) {
                throw new Error('pixlyLog is required for file-handler module');
            }
            log.debug('PIXLY File', formatLog(LOG.FILE_SELECTION_START, {}));
            const startTime = performance.now();
            
            // 🔥 修复：转换进行中禁止选择文件（与video-conversion保持一致）
            if (window.PIXLY_CONVERTING === true || window.isConverting === true) {
                log.warn('PIXLY File', formatLog(LOG.FILE_SELECTION_BLOCKED, {}));
                return;
            }
            
            try {
                // 🔥 检查Rust内核状态
                if (!window.rustCLI || !window.rustCLI.available) {
                    log.error('PIXLY File', formatLog(LOG.FILE_KERNEL_NOT_AVAILABLE, {}));
                    alert('❌ Rust内核未启动！\n\n无法选择文件。请先启动Rust内核。');
                    return;
                }
                
                // 1. 调用Eagle API获取选中的文件
                log.trace('PIXLY File', 'Calling Eagle API: eagle.item.getSelected()');
                const eagleSelection = await eagle.item.getSelected();
                
                if (!eagleSelection || eagleSelection.length === 0) {
                    log.debug('PIXLY File', 'No files selected in Eagle');
                    this.setSelectedFiles([]);
                    return;
                }
                
                log.info('PIXLY File', formatLog(LOG.FILE_SELECTED, { count: eagleSelection.length }));
                
                // 2. 确定当前转换类型（图像/视频）
                // 🔥 修复BUG：使用正确的面板ID（与ui-handlers.js一致）
                const imagePanel = document.getElementById('imageConversionPanel');
                const videoPanel = document.getElementById('videoPanel');
                const isImagePanelVisible = imagePanel && imagePanel.style.display !== 'none';
                const isVideoPanelVisible = videoPanel && videoPanel.style.display !== 'none';
                const conversionType = isVideoPanelVisible ? 'video' : 'image';
                
                log.debug('PIXLY File', formatLog(LOG.FILE_CONVERSION_TYPE, { type: conversionType }));
                log.trace('PIXLY File', `Panel visibility: image=${isImagePanelVisible}, video=${isVideoPanelVisible}`);
                
                // 3. 根据转换类型过滤文件
                // 动图（gif/webp）在两个模式都支持
                const animatedFormats = ['gif', 'webp'];
                const videoOnlyFormats = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'm4v', 'flv', 'wmv'];
                const imageOnlyFormats = ['jpg', 'jpeg', 'png', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff'];
                const metadataFormats = ['xmp'];  // 🔥 修复：支持XMP元数据文件
                
                const supportedExts = conversionType === 'video' 
                    ? [...videoOnlyFormats, ...animatedFormats]
                    : [...imageOnlyFormats, ...animatedFormats, ...metadataFormats];
                
                const filteredFiles = eagleSelection.filter(item => {
                    const ext = (item.ext || '').toLowerCase();
                    return supportedExts.includes(ext);
                }).map(item => {
                    // 4. 构建文件对象（路径解析交给Rust CLI）
                    // JS只传递基本信息，Rust负责路径解析和验证
                    return {
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
                    };
                });
                
                // 5. 更新选中状态
                this.setSelectedFiles(filteredFiles);
                
                // 6. 更新UI显示
                this.updateUI(filteredFiles);
                
                // 7. 启用/禁用转换按钮
                const convertBtn = document.getElementById('convertBtn');
                if (convertBtn) {
                    convertBtn.disabled = filteredFiles.length === 0;
                }
                
                // 🔥 解除锁定：用户选择了新文件（必须是不同的文件）
                if (filteredFiles.length > 0 && window.PIXLY_SELECTION_LOCKED) {
                    // 检查是否是真的新文件（路径不同）
                    const currentPaths = filteredFiles.map(f => f.filePath).sort().join('|');
                    const lastPaths = (window.PIXLY_LAST_CONVERTED_FILES || []).sort().join('|');
                    
                    if (currentPaths !== lastPaths) {
                        window.PIXLY_SELECTION_LOCKED = false;
                        log.info('PIXLY UX', formatLog(LOG.FILE_LOCK_STATUS_CHANGE, { status: 'false (NEW files selected)' }));
                        log.debug('PIXLY UX', `Previous: ${lastPaths}`);
                        log.debug('PIXLY UX', `Current: ${currentPaths}`);
                        
                        // 隐藏遮罩层
                        const lockOverlay = document.getElementById('conversionLockOverlay');
                        if (lockOverlay) {
                            lockOverlay.style.display = 'none';
                            log.info('PIXLY UX', formatLog(LOG.FILE_LOCK_OVERLAY_HIDDEN, {}));
                        }
                        
                        // 清除上次记录
                        window.PIXLY_LAST_CONVERTED_FILES = null;
                    } else {
                        log.debug('PIXLY UX', formatLog(LOG.FILE_SAME_FILES_KEEPING_LOCK, {}));
                    }
                }
                
                const elapsed = (performance.now() - startTime).toFixed(2);
                log.info('PIXLY File', formatLog(LOG.FILE_SELECTION_COMPLETE, { count: filteredFiles.length, time: elapsed }));
                
                return filteredFiles;
                
            } catch (error) {
                if (!log) {
                    throw error;
                }
                log.error('PIXLY File', formatLog(LOG.FILE_SELECTION_FAILED, { error: error.message }));
                this.setSelectedFiles([]);
                this.updateUI([]);
                throw error;
            }
        },
        
        /**
         * 带防抖的文件选择（减少频繁刷新）
         * @param {number} delay - 防抖延迟（毫秒）
         */
        selectFilesDebounced: function(delay = 300) {
            clearTimeout(this._debounceTimer);
            this._debounceTimer = setTimeout(() => {
                this.selectFiles();
            }, delay);
        },
        
        /**
         * 更新UI显示（UI层功能）
         * 
         * @param {Array} files - 文件数组
         */
        updateUI: function(files) {
            // 1. 更新文件计数
            const filesStats = document.getElementById('filesStats');
            if (filesStats) {
                const i18n = window.i18n || { t: (key) => key };
                filesStats.textContent = `${files.length} ${i18n.t('files.count')}`;
            }
            
            // 2. 显示/隐藏文件面板和空状态页面
            // 明确指定容器，避免 ID 冲突
            const fileSelectionContainer = document.getElementById('file-selection-container');
            const selectedFilesDiv = fileSelectionContainer?.querySelector('#selectedFiles') || document.getElementById('selectedFiles');
            const filesPanel = selectedFilesDiv?.closest('.files-selection-panel');
            const emptyState = document.querySelector('.empty-state');
            
            if (selectedFilesDiv) {
                selectedFilesDiv.style.display = files.length > 0 ? 'block' : 'none';
                
                // 🔥 通过class控制最小高度和显示隐藏（应用到.files-selection-panel）
                if (filesPanel) {
                    if (files.length > 0) {
                        filesPanel.classList.add('has-files');
                        filesPanel.classList.remove('empty');
                    } else {
                        filesPanel.classList.add('empty');
                        filesPanel.classList.remove('has-files');
                    }
                }
                
                // 🐛 Debug: Force visibility with strong styles
                if (files.length > 0) {
                    selectedFilesDiv.style.minHeight = '200px';
                    selectedFilesDiv.style.backgroundColor = 'rgba(102, 126, 234, 0.1)';
                    selectedFilesDiv.style.border = '2px solid #667eea';
                    selectedFilesDiv.style.position = 'relative';
                    selectedFilesDiv.style.zIndex = '100';
                    selectedFilesDiv.style.opacity = '1';
                    selectedFilesDiv.style.visibility = 'visible';
                    
                    const log = window.pixlyLog;
                    if (log) {
                        log.debug('PIXLY File', formatLog(LOG.FILE_FORCED_PANEL_STYLES, { 
                            display: selectedFilesDiv.style.display, 
                            minHeight: selectedFilesDiv.style.minHeight 
                        }));
                        log.trace('PIXLY File', `Parent: ${selectedFilesDiv.parentElement?.tagName} #${selectedFilesDiv.parentElement?.id}`);
                        log.trace('PIXLY File', `Children count: ${selectedFilesDiv.children.length}`);
                    }
                }
            }
            
            if (emptyState) {
                emptyState.style.display = files.length > 0 ? 'none' : 'block';
            }
            
            // 3. 更新文件列表（增强显示信息）
            const filesList = document.getElementById('filesList');
            if (filesList) {
                if (files.length === 0) {
                    filesList.innerHTML = '';
                    return;
                }
                
                filesList.innerHTML = files.map((file, index) => {
                    const fileSizeKB = file.size ? (file.size / 1024).toFixed(1) : '?';
                    const dimensions = (file.width && file.height) ? `${file.width}×${file.height}` : '';
                    
                    return `
                        <div class="file-card" data-index="${index}" title="${file.name}">
                            <div class="file-card-icon">${this.getFileIcon(file.ext)}</div>
                            <div class="file-card-info">
                                <div class="file-card-name">${file.name}</div>
                                <div class="file-card-meta">
                                    ${file.ext.toUpperCase()} · ${fileSizeKB} KB${dimensions ? ' · ' + dimensions : ''}
                                </div>
                            </div>
                            <div class="file-card-remove" onclick="window.PIXLY.FileHandler.removeFile(${index})" title="移除">×</div>
                        </div>
                    `;
                }).join('');
            }
            
            const log = window.pixlyLog;
            if (log) {
                log.debug('PIXLY File', formatLog(LOG.FILE_UI_UPDATED, { count: files.length }));
            }
            
            // 🐛 Debug: Check filesList content
            if (filesList && log) {
                log.trace('PIXLY File', formatLog(LOG.FILE_LIST_DEBUG_INFO, {
                    childCount: filesList.children.length,
                    htmlLength: filesList.innerHTML.length
                }));
                log.trace('PIXLY File', `Heights - offset: ${filesList.offsetHeight}, scroll: ${filesList.scrollHeight}`);
                log.trace('PIXLY File', `First 200 chars: ${filesList.innerHTML.substring(0, 200)}`);
            }
            
            // 🔥 Phase 45.8: 检测XMP文件并显示提示
            this.updateXMPHint(files);
            
            // 🎬 更新视频文件信息面板（仅视频转换模式）
            this.updateVideoFileInfo(files);
        },
        
        /**
         * 更新XMP自动合并提示
         * 
         * @param {Array} files - 文件数组
         */
        updateXMPHint: function(files) {
            const xmpHint = document.getElementById('xmpAutoMergeHint');
            const separator = document.getElementById('hintSeparator');
            const validationHint = document.getElementById('validationHint');
            const validationSeparator = document.getElementById('validationSeparator');
            
            if (!xmpHint) return;
            
            // 检测是否有XMP文件
            const hasXMP = files.some(f => {
                const ext = (f.ext || '').toLowerCase();
                return ext === '.xmp' || ext === 'xmp';
            });
            
            // 检测是否有对应的图像文件
            const hasImages = files.some(f => {
                const ext = (f.ext || '').toLowerCase();
                const imageExts = ['.jpg', '.jpeg', '.png', '.tif', '.tiff', '.dng', '.cr2', '.nef', '.arw'];
                return imageExts.includes(ext) || imageExts.includes('.' + ext);
            });
            
            // 🆕 动态调整位置：只有同时选中XMP文件和图像文件时才显示XMP提示
            if (hasXMP && hasImages) {
                // XMP提示显示，8层验证在左侧
                xmpHint.style.display = 'inline';
                if (separator) separator.style.display = 'inline';
                
                // 8层验证保持在左侧（order: 1）
                if (validationHint) {
                    validationHint.style.order = '1';
                }
                if (validationSeparator) {
                    validationSeparator.style.display = 'inline';
                    validationSeparator.style.order = '2';
                }
                
                if (log) {
                    log.info('PIXLY File', formatLog(LOG.FILE_XMP_DETECTED, {}));
                }
            } else {
                // XMP提示隐藏，8层验证移到右侧
                xmpHint.style.display = 'none';
                if (separator) separator.style.display = 'none';
                
                // 8层验证移到右侧（order: 10，在Filename之后）
                if (validationHint) {
                    validationHint.style.order = '10';
                }
                if (validationSeparator) {
                    validationSeparator.style.display = 'inline';
                    validationSeparator.style.order = '9';
                }
            }
        },
        
        /**
         * 获取文件图标（UI辅助函数）
         * 
         * @param {string} ext - 文件扩展名
         * @returns {string} - 图标emoji
         */
        getFileIcon: function(ext) {
            const iconMap = {
                '.jpg': '🖼️', '.jpeg': '🖼️', '.png': '🖼️',
                '.gif': '🎞️', '.webp': '🎨', '.avif': '🎨',
                '.jxl': '✨', '.heic': '📸', '.heif': '📸',
                '.mp4': '🎬', '.mov': '🎬', '.avi': '🎬',
                '.mkv': '🎬', '.webm': '🎬'
            };
            return iconMap[ext.toLowerCase()] || '📄';
        },
        
        /**
         * 更新视频文件信息面板
         * 遵循PROJECT_QUALITY_MANIFESTO.md：必须调用Rust内核获取真实数据
         * 
         * @param {Array} files - 文件数组
         */
        updateVideoFileInfo: async function(files) {
            const panel = document.getElementById('videoFileInfoPanel');
            if (!panel) return;
            
            // 检查是否在视频处理面板
            const videoPanel = document.getElementById('video-panel-container');
            const isVideoPanel = videoPanel && videoPanel.style.display !== 'none';
            
            // 只有在视频面板才显示
            if (!isVideoPanel) {
                return; // 图像面板不显示视频信息
            }
            
            // 面板永远显示，避免右下角空旷
            panel.style.display = 'block';
            
            // 如果没有文件，显示占位信息
            if (!files || files.length === 0) {
                const i18n = window.i18n || { t: (key) => key };
                this.updateVideoInfoPlaceholder(i18n.t('video.selectVideoOrAnimation'));
                const fileNameEl = document.getElementById('videoFileName');
                if (fileNameEl) {
                    const i18n = window.i18n || { t: (key) => key };
                    fileNameEl.textContent = i18n.t('video.noFileSelected');
                }
                return;
            }
            
            // 过滤出视频/动图文件（ext不带点号）
            const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'gif', 'webp'];
            const videoFiles = files.filter(f => {
                const ext = (f.ext || '').toLowerCase().replace(/^\./, ''); // 移除开头的点号
                return videoExts.includes(ext);
            });
            
            // 如果没有视频文件，显示提示
            if (videoFiles.length === 0) {
                const i18n = window.i18n || { t: (key) => key };
                this.updateVideoInfoPlaceholder(i18n.t('video.selectVideoOrAnimation'));
                const fileNameEl = document.getElementById('videoFileName');
                if (fileNameEl) {
                    fileNameEl.textContent = i18n.t('video.selectVideoOrAnimationHint');
                }
                return;
            }
            
            // 展示最后一个视频文件
            const lastFile = videoFiles[videoFiles.length - 1];
            
            const log = window.pixlyLog;
            if (log) {
                log.info('PIXLY File', formatLog(LOG.FILE_VIDEO_INFO_UPDATE, { name: lastFile.name }));
            }
            
            // 更新文件名
            const fileNameEl = document.getElementById('videoFileName');
            if (fileNameEl) {
                fileNameEl.textContent = lastFile.name;
            }
            
            // ✅ 调用Rust内核获取真实视频信息
            try {
                if (!window.rustCLI || !window.rustCLI.available) {
                    if (log) {
                        log.warn('PIXLY File', formatLog(LOG.FILE_RUST_CLI_NOT_AVAILABLE, {}));
                    }
                    const i18n = window.i18n || { t: (key) => key };
                    this.updateVideoInfoPlaceholder(i18n.t('error.toolNotFound'));
                    return;
                }
                
                const { spawn } = require('child_process');
                const rustPath = window.rustCLI.path;
                
                // 调用Rust CLI获取文件信息（info命令支持图像和视频，--json输出JSON格式）
                // Eagle返回的字段名是filePath，不是path
                const filePath = lastFile.filePath || lastFile.path;
                const args = ['info', filePath, '--json'];
                if (log) {
                    log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_CALL, { path: rustPath }));
                    log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_ARGS, { args: args.join(' ') }));
                    log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_FILE, { filePath: filePath }));
                }
                
                // spawn选项：跨平台环境变量设置
                const platform = process.platform;
                let additionalPaths = [];
                
                // 根据操作系统添加常见工具路径
                if (platform === 'darwin') {
                    // macOS: Homebrew (Intel + Apple Silicon)
                    additionalPaths = ['/opt/homebrew/bin', '/opt/homebrew/sbin', '/usr/local/bin'];
                } else if (platform === 'win32') {
                    // Windows: 常见安装路径
                    additionalPaths = [
                        'C:\\Program Files\\FFmpeg\\bin',
                        'C:\\ffmpeg\\bin',
                        process.env.USERPROFILE + '\\ffmpeg\\bin'
                    ];
                } else {
                    // Linux: 常见系统路径
                    additionalPaths = ['/usr/local/bin', '/usr/bin', '/opt/ffmpeg/bin'];
                }
                
                const pathSeparator = platform === 'win32' ? ';' : ':';
                const env = {
                    ...process.env,
                    PATH: `${additionalPaths.join(pathSeparator)}${pathSeparator}${process.env.PATH || ''}`
                };
                
                const proc = spawn(rustPath, args, {
                    env: env,
                    cwd: process.cwd()
                });
                let stdout = '';
                let stderr = '';
                
                proc.stdout.on('data', (data) => {
                    stdout += data.toString();
                });
                
                proc.stderr.on('data', (data) => {
                    stderr += data.toString();
                });
                
                proc.on('close', (code) => {
                    if (code !== 0) {
                        if (log) {
                            log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_FAILED, { code: code }));
                            log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_COMMAND, { command: `${rustPath} ${args.join(' ')}` }));
                            log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_STDERR, { stderr: stderr || '(empty)' }));
                            log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_RUST_CLI_STDOUT, { stdout: stdout || '(empty)' }));
                        }
                        const i18n = window.i18n || { t: (key) => key };
                        this.updateVideoInfoPlaceholder(i18n.t('error.conversionFailed') + ': ' + (stderr || 'Unknown error'));
                        return;
                    }
                    
                    try {
                        const info = JSON.parse(stdout);
                        if (log) {
                            log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_VIDEO_INFO_SUCCESS, { info: JSON.stringify(info) }));
                            log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_VIDEO_INFO_DETAILS, { 
                                duration: info.duration, 
                                frames: info.frame_count, 
                                fps: info.fps 
                            }));
                        }
                        this.updateVideoInfoDisplay(info);
                    } catch (e) {
                        if (log) {
                            log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_VIDEO_INFO_PARSE_FAILED, { error: e.message }));
                        }
                        const i18n = window.i18n || { t: (key) => key };
                        this.updateVideoInfoPlaceholder(i18n.t('error.invalidFormat'));
                    }
                });
                
            } catch (error) {
                if (log) {
                    log.error('PIXLY File', formatLog(LOG.FILE_HANDLER_VIDEO_INFO_ERROR, { error: error.message }));
                }
                const i18n = window.i18n || { t: (key) => key };
                this.updateVideoInfoPlaceholder(i18n.t('error.conversionFailed'));
            }
        },
        
        /**
         * 更新视频信息显示（真实数据）
         * 适配MediaInfo格式（Rust CLI --json输出）
         */
        updateVideoInfoDisplay: function(info) {
            // 文件大小
            const sizeEl = document.getElementById('videoFileSize');
            if (sizeEl && info.size) {
                const sizeMB = (info.size / (1024 * 1024)).toFixed(2);
                sizeEl.textContent = `${sizeMB} MB`;
            }
            
            // 分辨率（MediaInfo.resolution是数组[width, height]）
            const resEl = document.getElementById('videoResolution');
            if (resEl && info.resolution) {
                const [width, height] = info.resolution;
                resEl.textContent = `${width}×${height}`;
            }
            
            // 时长/帧数（动画优先显示帧数+时长）
            const durationEl = document.getElementById('videoDuration');
            if (durationEl) {
                if (info.media_type === 'animation' && info.frame_count) {
                    // 动画：显示帧数和时长
                    const durationMs = (info.duration * 1000).toFixed(0);
                    if (info.duration && info.duration < 1) {
                        durationEl.textContent = `${info.frame_count}帧 (${durationMs}ms)`;
                    } else if (info.duration) {
                        const mins = Math.floor(info.duration / 60);
                        const secs = (info.duration % 60).toFixed(1);
                        durationEl.textContent = `${info.frame_count}帧 (${mins}:${secs.toString().padStart(4, '0')})`;
                    } else {
                        durationEl.textContent = `${info.frame_count}帧`;
                    }
                } else if (info.duration) {
                    // 视频：显示时长
                    const mins = Math.floor(info.duration / 60);
                    const secs = Math.floor(info.duration % 60);
                    durationEl.textContent = `${mins}:${secs.toString().padStart(2, '0')}`;
                } else if (info.frame_count) {
                    // fallback：只有帧数
                    durationEl.textContent = `${info.frame_count}帧`;
                }
            }
            
            // 格式
            const formatEl = document.getElementById('videoFormat');
            if (formatEl && info.format) {
                formatEl.textContent = info.format.toUpperCase();
            }
            
            // 编码器（对于视频用format，对于静态图像可能没有codec字段）
            const codecEl = document.getElementById('videoCodec');
            if (codecEl) {
                codecEl.textContent = info.format || 'N/A';
            }
            
            // 比特率
            const bitrateEl = document.getElementById('videoBitrate');
            if (bitrateEl && info.bitrate) {
                const kbps = (info.bitrate / 1000).toFixed(0);
                bitrateEl.textContent = `${kbps} kbps`;
            }
        },
        
        /**
         * 更新占位符（当无法获取信息时）
         */
        updateVideoInfoPlaceholder: function(message) {
            const fields = ['videoFileSize', 'videoResolution', 'videoDuration', 'videoFormat', 'videoCodec', 'videoBitrate'];
            fields.forEach(id => {
                const el = document.getElementById(id);
                if (el) el.textContent = '--';
            });
            const log = window.pixlyLog;
            if (log) {
                log.warn('PIXLY File', formatLog(LOG.FILE_HANDLER_VIDEO_INFO_PLACEHOLDER, { message: message }));
            }
        },
        
        /**
         * 移除单个文件（UI层功能）
         * 
         * @param {number} index - 文件索引
         */
        removeFile: function(index) {
            const files = window.selectedFiles || [];
            if (index >= 0 && index < files.length) {
                files.splice(index, 1);
                this.setSelectedFiles(files);
                this.updateUI(files);
                const log = window.pixlyLog;
                if (log) {
                    log.info('PIXLY File', formatLog(LOG.FILE_HANDLER_FILE_REMOVED, { index: index }));
                }
            }
        },
        
        /**
         * 清空文件选择（UI层功能）
         */
        clearSelection: function() {
            this.setSelectedFiles([]);
            this.updateUI([]);
        }
    };
    
    // 暴露到全局
    window.PIXLY.FileHandler = fileHandler;
    window.selectFiles = fileHandler.selectFiles.bind(fileHandler);
    window.selectFilesDebounced = fileHandler.selectFilesDebounced.bind(fileHandler);
    window.clearFileSelection = fileHandler.clearSelection.bind(fileHandler);
    window.updateFileCounter = (count) => {
        const filesStats = document.getElementById('filesStats');
        if (filesStats) {
            const i18n = window.i18n || { t: (key) => key };
            filesStats.textContent = `${count} ${i18n.t('files.count')}`;
        }
    };
    
    const log = window.pixlyLog;
    if (log) {
        log.info('PIXLY File', '✅ File handler (UI layer) loaded');
        log.debug('PIXLY File', '💡 File processing logic → Rust (file_manager.rs)');
        log.debug('PIXLY File', '💡 Global functions: selectFiles(), clearFileSelection(), updateFileCounter()');
    }
    
})(window);
