/**
 * PIXLY Format - 专业格式转换核心
 * 继承自旧版本的所有手动控制功能
 */

// ==================== 状态管理 ====================
const state = {
    selectedFiles: [],
    conversionType: 'image', // 'image' or 'video'
    
    // 图像参数
    imageFormat: 'jxl',
    quality: 90,
    
    // JXL参数
    jxl: {
        jpegLossless: false,
        effort: 7,
        distance: 1.0,
        modular: false,
        progressive: false,
        responsive: false,
        gaborish: false,
        bitDepth: 'auto',      // 默认自动根据源文件
        colorSpace: 'auto'     // 默认保持源色彩空间
    },
    
    // AVIF参数
    avif: {
        speed: 6,
        minQuantizer: 0,
        maxQuantizer: 63,
        chroma: 'auto',  // 默认让FFmpeg自动选择
        tilesRows: 1,
        tilesCols: 1
    },
    
    // WebP参数
    webp: {
        method: 4,
        filterStrength: 60,
        sharpness: 0
    },
    
    // HEIC参数
    heic: {
        encoder: 'x265',
        chroma: 'auto',  // 默认让FFmpeg自动选择
        lossless: false,
        thumbnail: false
    },
    
    // 视频参数
    video: {
        container: 'mp4',
        encoder: 'h265',
        crf: 23,
        rateControl: 'crf',
        gop: 250,
        bframes: 3,
        refs: 3,
        meMethod: 'hex',
        pixFmt: 'auto'  // 默认让FFmpeg自动选择
    },
    
    isConverting: false,
    rustCorePath: null
};

// ==================== 初始化 ====================
document.addEventListener('DOMContentLoaded', async () => {
    // 初始化日志系统
    if (typeof config !== 'undefined') {
        logger.init({ development: config.development });
        if (config.logLevel) {
            const levelMap = {
                'DEBUG': 0,
                'INFO': 1,
                'WARN': 2,
                'ERROR': 3
            };
            logger.setLevel(levelMap[config.logLevel] || 1);
        }
    }
    
    // 初始化国际化
    if (window.i18n && window.i18n.init) {
        await window.i18n.init();
    }
    
    // 初始化主题
    initTheme();
    
    // 绑定事件
    bindEvents();
    
    // 检测Rust核心
    await detectRustCore();
    
    // 注册Eagle生命周期
    registerEagleLifecycle();
    
    logger.info('PIXLY Format', LOG.INIT_COMPLETE);
});

// 暴露到全局，供刷新按钮调用
window.loadFilesFromEagle = loadFilesFromEagle;

// ==================== 主题系统 ====================
function initTheme() {
    const themeToggle = document.getElementById('themeToggle');
    const savedTheme = localStorage.getItem('pixly_format_theme') || 'dark';
    
    document.documentElement.setAttribute('data-theme', savedTheme);
    updateThemeIcon(savedTheme);
    
    if (themeToggle) {
        themeToggle.addEventListener('click', () => {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            document.documentElement.setAttribute('data-theme', newTheme);
            localStorage.setItem('pixly_format_theme', newTheme);
            updateThemeIcon(newTheme);
        });
    }
}

function updateThemeIcon(theme) {
    const icon = document.querySelector('.theme-icon');
    if (icon) {
        icon.textContent = theme === 'dark' ? '🌙' : '☀️';
    }
}

// ==================== 事件绑定 ====================
function bindEvents() {
    // 转换类型切换
    const typeTabs = document.querySelectorAll('.type-tab');
    typeTabs.forEach(tab => {
        tab.addEventListener('click', function() {
            const type = this.getAttribute('data-type');
            switchConversionType(type);
        });
    });
    
    // 清空文件按钮
    const clearBtn = document.getElementById('clearFilesBtn');
    if (clearBtn) {
        clearBtn.addEventListener('click', clearFiles);
    }
    
    // 转换按钮
    const convertBtn = document.getElementById('convertBtn');
    if (convertBtn) {
        convertBtn.addEventListener('click', startConversion);
    }
    
    // 打开文件夹按钮
    const openFolderBtn = document.getElementById('openFolderBtn');
    if (openFolderBtn) {
        openFolderBtn.addEventListener('click', openOutputFolder);
    }
    
    // 语言切换
    const langSelect = document.getElementById('languageSelect');
    if (langSelect && window.i18n) {
        langSelect.addEventListener('change', async (e) => {
            await window.i18n.switchLanguage(e.target.value);
        });
    }
    
    // 图像格式切换
    bindFormatButtons();
    
    // 图像参数绑定
    bindImageParams();
    
    // 视频参数绑定
    bindVideoParams();
}

function bindFormatButtons() {
    const formatBtns = document.querySelectorAll('.format-btn');
    formatBtns.forEach(btn => {
        btn.addEventListener('click', function() {
            formatBtns.forEach(b => b.classList.remove('active'));
            this.classList.add('active');
            const radio = this.querySelector('input[type="radio"]');
            if (radio) {
                state.imageFormat = radio.value;
                switchFormatParams(radio.value);
                logger.info('PIXLY Format', LOG.FORMAT_SWITCHED, { format: state.imageFormat });
            }
        });
    });
}

function switchFormatParams(format) {
    // 隐藏所有格式专属面板
    const allParams = document.querySelectorAll('.format-specific-panel');
    allParams.forEach(panel => panel.style.display = 'none');
    
    // 显示当前格式的面板
    const currentPanel = document.getElementById(`${format}Params`);
    if (currentPanel) {
        currentPanel.style.display = 'block';
    }
}

function bindImageParams() {
    // 质量滑块
    const qualitySlider = document.getElementById('quality');
    const qualityValue = document.getElementById('qualityValue');
    const qualityHint = document.getElementById('qualityHint');
    
    if (qualitySlider) {
        qualitySlider.addEventListener('input', (e) => {
            const value = parseInt(e.target.value);
            state.quality = value;
            if (qualityValue) qualityValue.textContent = value;
            
            // 更新提示
            if (qualityHint) {
                const i18n = window.i18n || { t: (key) => key };
                if (value >= 95) {
                    qualityHint.textContent = i18n.t('quality.high');
                } else if (value >= 80) {
                    qualityHint.textContent = i18n.t('quality.balanced');
                } else if (value >= 60) {
                    qualityHint.textContent = i18n.t('quality.compressed');
                } else {
                    qualityHint.textContent = i18n.t('quality.size');
                }
            }
        });
    }
    
    // JXL参数
    bindSlider('jxlEffort', 'jxlEffortValue', (v) => state.jxl.effort = v);
    bindSlider('jxlDistance', 'jxlDistanceValue', (v) => state.jxl.distance = parseFloat(v), true);
    bindCheckbox('jpegLossless', (v) => state.jxl.jpegLossless = v);
    bindCheckbox('jxlModular', (v) => state.jxl.modular = v);
    bindCheckbox('jxlProgressive', (v) => state.jxl.progressive = v);
    bindCheckbox('jxlResponsive', (v) => state.jxl.responsive = v);
    bindCheckbox('jxlGaborish', (v) => state.jxl.gaborish = v);
    bindSelect('jxlBitDepth', (v) => state.jxl.bitDepth = v);
    bindSelect('jxlColorSpace', (v) => state.jxl.colorSpace = v);
    
    // AVIF参数
    bindSlider('avifSpeed', 'avifSpeedValue', (v) => state.avif.speed = v);
    bindSlider('avifMinQuantizer', 'avifMinQuantizerValue', (v) => state.avif.minQuantizer = v);
    bindSlider('avifMaxQuantizer', 'avifMaxQuantizerValue', (v) => state.avif.maxQuantizer = v);
    bindSelect('avifChroma', (v) => state.avif.chroma = v);
    bindNumber('avifTilesRows', (v) => state.avif.tilesRows = v);
    bindNumber('avifTilesCols', (v) => state.avif.tilesCols = v);
    
    // WebP参数
    bindSlider('webpMethod', 'webpMethodValue', (v) => state.webp.method = v);
    bindSlider('webpFilterStrength', 'webpFilterStrengthValue', (v) => state.webp.filterStrength = v);
    bindSlider('webpSharpness', 'webpSharpnessValue', (v) => state.webp.sharpness = v);
    
    // HEIC参数
    bindSelect('heicEncoder', (v) => state.heic.encoder = v);
    bindSelect('heicChroma', (v) => state.heic.chroma = v);
    bindCheckbox('heicLossless', (v) => state.heic.lossless = v);
    bindCheckbox('heicThumbnail', (v) => state.heic.thumbnail = v);
}

function bindVideoParams() {
    // 容器格式
    const containerBtns = document.querySelectorAll('input[name="videoContainer"]');
    containerBtns.forEach(radio => {
        radio.addEventListener('change', (e) => {
            state.video.container = e.target.value;
            // 更新按钮样式
            document.querySelectorAll('.format-btn').forEach(btn => {
                btn.classList.remove('active');
                if (btn.querySelector('input[name="videoContainer"]')?.value === e.target.value) {
                    btn.classList.add('active');
                }
            });
        });
    });
    
    // 编码器
    const encoderBtns = document.querySelectorAll('input[name="videoEncoder"]');
    encoderBtns.forEach(radio => {
        radio.addEventListener('change', (e) => {
            state.video.encoder = e.target.value;
            // 更新按钮样式
            document.querySelectorAll('.encoder-btn').forEach(btn => {
                btn.classList.remove('active');
                if (btn.querySelector('input[name="videoEncoder"]')?.value === e.target.value) {
                    btn.classList.add('active');
                }
            });
        });
    });
    
    // 视频参数
    bindSlider('videoCrf', 'videoCrfValue', (v) => state.video.crf = v);
    bindSelect('videoRateControl', (v) => state.video.rateControl = v);
    bindSlider('videoGop', 'videoGopValue', (v) => state.video.gop = v);
    bindSlider('videoBframes', 'videoBframesValue', (v) => state.video.bframes = v);
    bindSlider('videoRefs', 'videoRefsValue', (v) => state.video.refs = v);
    bindSelect('videoMeMethod', (v) => state.video.meMethod = v);
    bindSelect('videoPixFmt', (v) => state.video.pixFmt = v);
}

// 辅助函数：绑定滑块
function bindSlider(sliderId, valueId, callback, isFloat = false) {
    const slider = document.getElementById(sliderId);
    const valueEl = document.getElementById(valueId);
    
    if (slider) {
        slider.addEventListener('input', (e) => {
            const value = isFloat ? parseFloat(e.target.value) : parseInt(e.target.value);
            if (valueEl) {
                valueEl.textContent = isFloat ? value.toFixed(1) : value;
            }
            callback(value);
        });
    }
}

// 辅助函数：绑定复选框
function bindCheckbox(checkboxId, callback) {
    const checkbox = document.getElementById(checkboxId);
    if (checkbox) {
        checkbox.addEventListener('change', (e) => {
            callback(e.target.checked);
        });
    }
}

// 辅助函数：绑定选择框
function bindSelect(selectId, callback) {
    const select = document.getElementById(selectId);
    if (select) {
        select.addEventListener('change', (e) => {
            callback(e.target.value);
        });
    }
}

// 辅助函数：绑定数字输入
function bindNumber(inputId, callback) {
    const input = document.getElementById(inputId);
    if (input) {
        input.addEventListener('input', (e) => {
            callback(parseInt(e.target.value) || 1);
        });
    }
}

// ==================== 转换类型切换 ====================
function switchConversionType(type) {
    state.conversionType = type;
    
    // 更新标签样式
    const typeTabs = document.querySelectorAll('.type-tab');
    typeTabs.forEach(tab => {
        if (tab.getAttribute('data-type') === type) {
            tab.classList.add('active');
        } else {
            tab.classList.remove('active');
        }
    });
    
    // 切换面板
    const imagePanel = document.getElementById('imagePanel');
    const videoPanel = document.getElementById('videoPanel');
    
    if (type === 'image') {
        if (imagePanel) imagePanel.style.display = 'flex';
        if (videoPanel) videoPanel.style.display = 'none';
    } else {
        if (imagePanel) imagePanel.style.display = 'none';
        if (videoPanel) videoPanel.style.display = 'flex';
    }
    
    logger.info('PIXLY Format', LOG.TYPE_SWITCHED, { type });
}

// ==================== Eagle生命周期 ====================
function registerEagleLifecycle() {
    if (typeof eagle === 'undefined') {
        logger.warn('PIXLY Format', LOG.EAGLE_API_UNAVAILABLE);
        return;
    }
    
    eagle.onPluginCreate((plugin) => {
        logger.info('PIXLY Format', LOG.EAGLE_PLUGIN_CREATED);
    });
    
    eagle.onPluginShow(() => {
        logger.info('PIXLY Format', LOG.EAGLE_PLUGIN_SHOWN);
        if (!state.isConverting) {
            setTimeout(() => {
                loadFilesFromEagle();
            }, 300);
        }
    });
    
    eagle.onPluginRun(() => {
        logger.info('PIXLY Format', LOG.EAGLE_PLUGIN_RUNNING);
        // 运行时也刷新文件列表
        if (!state.isConverting) {
            setTimeout(() => {
                loadFilesFromEagle();
            }, 100);
        }
    });
    
    eagle.onPluginHide(() => {
        logger.info('PIXLY Format', LOG.EAGLE_PLUGIN_HIDDEN);
    });
    
    eagle.onPluginBeforeExit(() => {
        logger.info('PIXLY Format', LOG.EAGLE_PLUGIN_EXITING);
    });
}

// ==================== 文件处理 ====================
async function loadFilesFromEagle() {
    logger.info('PIXLY Format', LOG.FILE_LOADING_START);
    
    if (typeof eagle === 'undefined') {
        logger.error('PIXLY Format', LOG.EAGLE_API_UNAVAILABLE);
        alert('Eagle API 不可用，请确保在Eagle中运行此插件');
        return;
    }
    
    try {
        logger.info('PIXLY Format', LOG.FILE_LOADING_EAGLE_CALL);
        const items = await eagle.item.getSelected();
        
        logger.info('PIXLY Format', LOG.FILE_LOADING_EAGLE_RETURNED, { count: items ? items.length : 0 });
        
        if (!items || items.length === 0) {
            logger.warn('PIXLY Format', LOG.FILE_LOADING_NO_FILES);
            state.selectedFiles = [];
            updateFilesUI();
            return;
        }
        
        // 过滤支持的文件类型（根据当前转换类型）
        const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff'];
        const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm'];
        const supportedExts = state.conversionType === 'image' ? imageExts : videoExts;
        
        logger.info('PIXLY Format', LOG.FILE_LOADING_TYPE_INFO, { type: state.conversionType, exts: supportedExts.length });
        
        state.selectedFiles = items.filter(item => {
            const ext = (item.ext || '').toLowerCase().replace(/^\./, '');
            const isSupported = supportedExts.includes(ext);
            logger.info('PIXLY Format', LOG.FILE_LOADING_FILE_CHECK, { name: item.name, ext, supported: isSupported });
            return isSupported;
        }).map(item => ({
            id: item.id,
            name: item.name,
            ext: item.ext,
            filePath: item.filePath,
            size: item.size || 0,
            width: item.width || 0,
            height: item.height || 0,
            isAnimated: item.isAnimated || false
        }));
        
        logger.info('PIXLY Format', LOG.FILE_LOADING_FILTERED, { count: state.selectedFiles.length });
        
        updateFilesUI();
        
        logger.info('PIXLY Format', LOG.FILE_LOADING_COMPLETE, { count: state.selectedFiles.length });
        
    } catch (error) {
        logger.error('PIXLY Format', LOG.FILE_LOADING_ERROR, {}, error);
        alert(`加载文件失败: ${error.message}`);
    }
}

function updateFilesUI() {
    const emptyState = document.getElementById('emptyState');
    const mainGrid = document.getElementById('mainGrid');
    const filesList = document.getElementById('filesList');
    const filesCount = document.getElementById('filesCount');
    const convertBtn = document.getElementById('convertBtn');
    
    if (state.selectedFiles.length === 0) {
        if (emptyState) emptyState.style.display = 'flex';
        if (mainGrid) mainGrid.style.display = 'none';
        if (convertBtn) convertBtn.disabled = true;
        return;
    }
    
    if (emptyState) emptyState.style.display = 'none';
    if (mainGrid) mainGrid.style.display = 'grid';
    if (convertBtn) convertBtn.disabled = false;
    
    // 更新文件数量
    if (filesCount) {
        filesCount.textContent = state.selectedFiles.length;
    }
    
    // 更新文件列表
    if (filesList) {
        filesList.innerHTML = state.selectedFiles.map((file, index) => {
            const icon = getFileIcon(file.ext);
            const sizeKB = (file.size / 1024).toFixed(1);
            const dimensions = file.width && file.height ? `${file.width}×${file.height}` : '';
            
            return `
                <div class="file-item">
                    <span class="file-icon">${icon}</span>
                    <div class="file-info">
                        <div class="file-name">${file.name}</div>
                        <div class="file-meta">${file.ext.toUpperCase()} · ${sizeKB} KB${dimensions ? ' · ' + dimensions : ''}</div>
                    </div>
                </div>
            `;
        }).join('');
    }
}

function getFileIcon(ext) {
    const iconMap = {
        'jpg': '🖼️', 'jpeg': '🖼️', 'png': '🖼️',
        'gif': '🎞️', 'webp': '🎨', 'avif': '🎨',
        'jxl': '✨', 'heic': '📸', 'heif': '📸',
        'mp4': '🎬', 'mov': '🎬', 'avi': '🎬',
        'mkv': '🎬', 'webm': '🎬'
    };
    const cleanExt = ext.toLowerCase().replace(/^\./, '');
    return iconMap[cleanExt] || '📄';
}

function clearFiles() {
    state.selectedFiles = [];
    updateFilesUI();
    logger.info('PIXLY Format', LOG.FILES_CLEARED);
}

// ==================== Rust核心检测 ====================
async function detectRustCore() {
    try {
        const { spawn } = require('child_process');
        const path = require('path');
        const fs = require('fs');
        
        logger.debug('PIXLY Format', 'Rust core detection started', {
            dirname: __dirname,
            cwd: process.cwd()
        });
        
        const possiblePaths = [
            path.join(__dirname, 'bin/pixly-converter'),  // 插件内部bin目录（优先）
            path.join(__dirname, '../bin/pixly-converter'),
            path.join(__dirname, '../../bin/pixly-converter'),
            path.join(__dirname, '../../../target/release/pixly-converter'),
            path.join(__dirname, '../../../target/debug/pixly-converter'),
            'pixly-converter'  // 系统PATH
        ];
        
        logger.debug('PIXLY Format', 'Checking paths', { count: possiblePaths.length });
        
        for (const rustPath of possiblePaths) {
            try {
                const resolved = path.resolve(rustPath);
                if (!fs.existsSync(resolved) && rustPath !== 'pixly-converter') {
                    continue;
                }
                
                const proc = spawn(rustPath, ['--version'], { timeout: 3000 });
                let output = '';
                
                proc.stdout.on('data', (data) => {
                    output += data.toString();
                });
                
                await new Promise((resolve, reject) => {
                    proc.on('close', (code) => {
                        if (code === 0) {
                            state.rustCorePath = rustPath;
                            logger.info('PIXLY Format', LOG.RUST_CORE_FOUND, { path: rustPath });
                            logger.info('PIXLY Format', LOG.RUST_CORE_VERSION, { version: output.trim() });
                            resolve();
                        } else {
                            reject();
                        }
                    });
                    proc.on('error', reject);
                });
                
                return;
            } catch (e) {
                continue;
            }
        }
        
        logger.warn('PIXLY Format', LOG.RUST_CORE_NOT_FOUND);
    } catch (error) {
        logger.error('PIXLY Format', LOG.RUST_CORE_DETECTION_FAILED, {}, error);
    }
}

// ==================== 转换处理 ====================
async function startConversion() {
    if (state.selectedFiles.length === 0) {
        alert('请先选择文件');
        return;
    }
    
    if (!state.rustCorePath) {
        alert('Rust核心未找到，无法转换');
        return;
    }
    
    state.isConverting = true;
    
    // 显示进度面板
    const progressPanel = document.getElementById('progressPanel');
    const resultPanel = document.getElementById('resultPanel');
    if (progressPanel) progressPanel.style.display = 'block';
    if (resultPanel) resultPanel.style.display = 'none';
    
    // 禁用转换按钮
    const convertBtn = document.getElementById('convertBtn');
    if (convertBtn) {
        convertBtn.disabled = true;
        convertBtn.querySelector('.btn-text').textContent = '转换中...';
    }
    
    logger.info('PIXLY Format', LOG.CONVERSION_START);
    logger.info('PIXLY Format', LOG.CONVERSION_TYPE, { type: state.conversionType });
    
    try {
        const results = await convertFiles();
        showResults(results);
    } catch (error) {
        logger.error('PIXLY Format', LOG.CONVERSION_ERROR, {}, error);
        alert(`转换失败: ${error.message}`);
    } finally {
        state.isConverting = false;
        if (convertBtn) {
            convertBtn.disabled = false;
            convertBtn.querySelector('.btn-text').textContent = '开始转换';
        }
    }
}

async function convertFiles() {
    const { spawn } = require('child_process');
    const results = {
        success: 0,
        failed: 0,
        total: state.selectedFiles.length
    };
    
    for (let i = 0; i < state.selectedFiles.length; i++) {
        const file = state.selectedFiles[i];
        
        // 更新进度
        updateProgress(i + 1, state.selectedFiles.length, file.name);
        
        try {
            // 构建Rust CLI参数
            const args = buildConversionArgs(file);
            
            // 执行转换
            await executeRustCLI(args);
            
            results.success++;
        } catch (error) {
            logger.error('PIXLY Format', LOG.CONVERSION_FILE_FAILED, { name: file.name }, error);
            results.failed++;
        }
    }
    
    return results;
}

function buildConversionArgs(file) {
    logger.debug('PIXLY Format', 'Building conversion arguments', { 
        fileName: file.name,
        type: state.conversionType
    });
    
    const args = ['convert', file.filePath];
    
    if (state.conversionType === 'image') {
        // 图像转换参数
        args.push('--format', state.imageFormat);
        args.push('--quality', state.quality.toString());
        
        // 格式专属参数
        if (state.imageFormat === 'jxl') {
            if (state.jxl.jpegLossless) args.push('--jpeg-lossless');
            args.push('--effort', state.jxl.effort.toString());
            args.push('--distance', state.jxl.distance.toString());
            if (state.jxl.modular) args.push('--modular');
            if (state.jxl.progressive) args.push('--progressive');
            if (state.jxl.responsive) args.push('--responsive');
            if (state.jxl.gaborish) args.push('--gaborish');
            // 🔥 只有非auto时才传递，让工具自动处理
            if (state.jxl.bitDepth && state.jxl.bitDepth !== 'auto') {
                args.push('--bit-depth', state.jxl.bitDepth);
            }
            if (state.jxl.colorSpace && state.jxl.colorSpace !== 'auto') {
                args.push('--color-space', state.jxl.colorSpace);
            }
        } else if (state.imageFormat === 'avif') {
            args.push('--speed', state.avif.speed.toString());
            args.push('--min-quantizer', state.avif.minQuantizer.toString());
            args.push('--max-quantizer', state.avif.maxQuantizer.toString());
            // 🔥 只有非auto时才传递chroma参数，让FFmpeg自动选择
            if (state.avif.chroma && state.avif.chroma !== 'auto') {
                args.push('--chroma', state.avif.chroma);
            }
            args.push('--tiles', `${state.avif.tilesRows}x${state.avif.tilesCols}`);
        } else if (state.imageFormat === 'webp') {
            args.push('--method', state.webp.method.toString());
            args.push('--filter-strength', state.webp.filterStrength.toString());
            args.push('--sharpness', state.webp.sharpness.toString());
        } else if (state.imageFormat === 'heic') {
            args.push('--encoder', state.heic.encoder);
            // 🔥 只有非auto时才传递chroma参数，让FFmpeg自动选择
            if (state.heic.chroma && state.heic.chroma !== 'auto') {
                args.push('--chroma', state.heic.chroma);
            }
            if (state.heic.lossless) args.push('--lossless');
            if (state.heic.thumbnail) args.push('--thumbnail');
        }
    } else {
        // 视频转换参数
        args.push('--container', state.video.container);
        args.push('--encoder', state.video.encoder);
        args.push('--crf', state.video.crf.toString());
        args.push('--rate-control', state.video.rateControl);
        args.push('--gop', state.video.gop.toString());
        args.push('--bframes', state.video.bframes.toString());
        args.push('--refs', state.video.refs.toString());
        args.push('--me-method', state.video.meMethod);
        // 🔥 只有非auto时才传递pix-fmt参数，让FFmpeg自动选择
        if (state.video.pixFmt && state.video.pixFmt !== 'auto') {
            args.push('--pix-fmt', state.video.pixFmt);
        }
    }
    
    // 输出目录
    const path = require('path');
    const outputDir = path.join(path.dirname(file.filePath), 'pixly_output');
    args.push('--output', outputDir);
    
    logger.debug('PIXLY Format', 'Conversion arguments built', { 
        argsCount: args.length,
        outputDir
    });
    
    return args;
}

function executeRustCLI(args) {
    return new Promise((resolve, reject) => {
        const { spawn } = require('child_process');
        
        logger.debug('PIXLY Format', 'CLI execution started', { 
            path: state.rustCorePath,
            args: args.join(' ')
        });
        
        const proc = spawn(state.rustCorePath, args);
        
        let stdout = '';
        let stderr = '';
        
        proc.stdout.on('data', (data) => {
            stdout += data.toString();
        });
        
        proc.stderr.on('data', (data) => {
            stderr += data.toString();
        });
        
        proc.on('close', (code) => {
            logger.debug('PIXLY Format', 'CLI execution finished', { 
                code,
                stdoutLength: stdout.length,
                stderrLength: stderr.length
            });
            
            if (code === 0) {
                resolve(stdout);
            } else {
                reject(new Error(stderr || `Exit code: ${code}`));
            }
        });
        
        proc.on('error', (err) => {
            logger.error('PIXLY Format', 'CLI spawn failed', {}, err);
            reject(err);
        });
    });
}

function updateProgress(current, total, fileName) {
    const percent = Math.round((current / total) * 100);
    
    const progressPercent = document.getElementById('progressPercent');
    const progressFill = document.getElementById('progressFill');
    const progressInfo = document.getElementById('progressInfo');
    
    if (progressPercent) progressPercent.textContent = `${percent}%`;
    if (progressFill) progressFill.style.width = `${percent}%`;
    if (progressInfo) progressInfo.textContent = `正在处理: ${fileName} (${current}/${total})`;
}

function showResults(results) {
    const progressPanel = document.getElementById('progressPanel');
    const resultPanel = document.getElementById('resultPanel');
    const resultStats = document.getElementById('resultStats');
    
    if (progressPanel) progressPanel.style.display = 'none';
    if (resultPanel) resultPanel.style.display = 'block';
    
    if (resultStats) {
        resultStats.innerHTML = `
            <div class="stat-item">
                <span class="stat-label">总计:</span>
                <span class="stat-value">${results.total}</span>
            </div>
            <div class="stat-item success">
                <span class="stat-label">成功:</span>
                <span class="stat-value">${results.success}</span>
            </div>
            ${results.failed > 0 ? `
            <div class="stat-item failed">
                <span class="stat-label">失败:</span>
                <span class="stat-value">${results.failed}</span>
            </div>
            ` : ''}
        `;
    }
    
    logger.info('PIXLY Format', LOG.CONVERSION_COMPLETE);
}

function openOutputFolder() {
    if (state.selectedFiles.length === 0) return;
    
    const path = require('path');
    const { shell } = require('electron');
    
    const firstFile = state.selectedFiles[0];
    const outputDir = path.join(path.dirname(firstFile.filePath), 'pixly_output');
    
    shell.openPath(outputDir);
}
