/**
 * PIXLY AI - 智能多媒体处理核心
 * 继承自旧版本的所有AI功能，最简化界面
 */

// ==================== 状态管理 ====================
const state = {
    selectedFiles: [],
    mediaTypes: {
        images: [],      // 静态图片
        animations: [],  // 动图（GIF/APNG/WebP动画）
        videos: []       // 视频
    },
    aiPreset: 'balanced',
    aiFeatures: {
        smartQuality: true,
        formatRecommend: true,
        ssimValidation: true,
        videoForAnimation: true  // 默认启用
    },
    isConverting: false,
    rustCorePath: null
};

// ==================== 初始化 ====================
document.addEventListener('DOMContentLoaded', async () => {
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
    
    console.log('[PIXLY AI] ✅ 初始化完成');
});

// 暴露到全局，供刷新按钮调用
window.loadFilesFromEagle = loadFilesFromEagle;

// ==================== 主题系统 ====================
function initTheme() {
    const themeToggle = document.getElementById('themeToggle');
    const savedTheme = localStorage.getItem('pixly_theme') || 'dark';
    
    document.documentElement.setAttribute('data-theme', savedTheme);
    updateThemeIcon(savedTheme);
    
    if (themeToggle) {
        themeToggle.addEventListener('click', () => {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            document.documentElement.setAttribute('data-theme', newTheme);
            localStorage.setItem('pixly_theme', newTheme);
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
    
    // AI预设切换
    const presetBtns = document.querySelectorAll('.preset-btn');
    presetBtns.forEach(btn => {
        btn.addEventListener('click', function() {
            presetBtns.forEach(b => b.classList.remove('active'));
            this.classList.add('active');
            const radio = this.querySelector('input[type="radio"]');
            if (radio) {
                state.aiPreset = radio.value;
                console.log(`[PIXLY AI] 切换预设: ${state.aiPreset}`);
            }
        });
    });
    
    // AI功能开关
    document.getElementById('smartQuality')?.addEventListener('change', (e) => {
        state.aiFeatures.smartQuality = e.target.checked;
    });
    document.getElementById('formatRecommend')?.addEventListener('change', (e) => {
        state.aiFeatures.formatRecommend = e.target.checked;
    });
    document.getElementById('ssimValidation')?.addEventListener('change', (e) => {
        state.aiFeatures.ssimValidation = e.target.checked;
    });
    document.getElementById('videoForAnimation')?.addEventListener('change', (e) => {
        state.aiFeatures.videoForAnimation = e.target.checked;
    });
    
    // 语言切换
    const langSelect = document.getElementById('languageSelect');
    if (langSelect && window.i18n) {
        langSelect.addEventListener('change', async (e) => {
            await window.i18n.switchLanguage(e.target.value);
        });
    }
}

// ==================== Eagle生命周期 ====================
function registerEagleLifecycle() {
    if (typeof eagle === 'undefined') {
        console.warn('[PIXLY AI] Eagle API 不可用');
        return;
    }
    
    eagle.onPluginCreate((plugin) => {
        console.log('[PIXLY AI] 插件创建', plugin);
    });
    
    eagle.onPluginShow(() => {
        console.log('[PIXLY AI] 插件显示');
        // 自动刷新文件列表（带防抖）
        if (!state.isConverting) {
            setTimeout(() => {
                loadFilesFromEagle();
            }, 300);
        }
    });
    
    eagle.onPluginRun(() => {
        console.log('[PIXLY AI] 插件运行');
        // 运行时也刷新文件列表
        if (!state.isConverting) {
            setTimeout(() => {
                loadFilesFromEagle();
            }, 100);
        }
    });
    
    eagle.onPluginHide(() => {
        console.log('[PIXLY AI] 插件隐藏');
    });
    
    eagle.onPluginBeforeExit(() => {
        console.log('[PIXLY AI] 插件退出');
    });
}

// ==================== 文件处理 ====================
async function loadFilesFromEagle() {
    console.log('[PIXLY AI] 🔍 开始加载文件...');
    
    if (typeof eagle === 'undefined') {
        console.error('[PIXLY AI] ❌ Eagle API 不可用');
        alert('Eagle API 不可用，请确保在Eagle中运行此插件');
        return;
    }
    
    try {
        console.log('[PIXLY AI] 📞 调用 eagle.item.getSelected()...');
        const items = await eagle.item.getSelected();
        
        console.log('[PIXLY AI] 📦 Eagle返回:', items);
        console.log('[PIXLY AI] 📊 文件数量:', items ? items.length : 0);
        
        if (!items || items.length === 0) {
            console.log('[PIXLY AI] ⚠️ 未选择文件');
            state.selectedFiles = [];
            updateFilesUI();
            return;
        }
        
        // 过滤支持的文件类型
        const supportedExts = [
            'jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff',
            'mp4', 'mov', 'avi', 'mkv', 'webm'
        ];
        
        console.log('[PIXLY AI] 🔍 开始过滤文件...');
        state.selectedFiles = items.filter(item => {
            const ext = (item.ext || '').toLowerCase().replace(/^\./, '');
            const isSupported = supportedExts.includes(ext);
            console.log(`[PIXLY AI] 文件: ${item.name}, 扩展名: ${ext}, 支持: ${isSupported}`);
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
        
        console.log('[PIXLY AI] ✅ 过滤后文件数:', state.selectedFiles.length);
        
        // 自动检测媒体类型
        detectMediaType();
        
        updateFilesUI();
        
        console.log(`[PIXLY AI] ✅ 已加载 ${state.selectedFiles.length} 个文件`);
        
    } catch (error) {
        console.error('[PIXLY AI] ❌ 加载文件失败:', error);
        console.error('[PIXLY AI] 错误堆栈:', error.stack);
        alert(`加载文件失败: ${error.message}`);
    }
}

function detectMediaType() {
    // 清空分类
    state.mediaTypes = {
        images: [],
        animations: [],
        videos: []
    };
    
    // 根据扩展名和isAnimated标志分类
    state.selectedFiles.forEach(file => {
        const ext = (file.ext || '').toLowerCase().replace(/^\./, '');
        
        // 视频格式
        const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'm4v', 'flv'];
        if (videoExts.includes(ext)) {
            state.mediaTypes.videos.push(file);
            return;
        }
        
        // 动图格式（GIF/APNG/WebP动画）
        const animExts = ['gif', 'apng'];
        if (animExts.includes(ext) || (ext === 'webp' && file.isAnimated)) {
            state.mediaTypes.animations.push(file);
            return;
        }
        
        // 静态图片
        state.mediaTypes.images.push(file);
    });
    
    console.log('[PIXLY AI] 媒体分类:', {
        images: state.mediaTypes.images.length,
        animations: state.mediaTypes.animations.length,
        videos: state.mediaTypes.videos.length
    });
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
    
    // 更新文件数量（显示分类统计）
    if (filesCount) {
        const total = state.selectedFiles.length;
        const images = state.mediaTypes.images.length;
        const animations = state.mediaTypes.animations.length;
        const videos = state.mediaTypes.videos.length;
        
        let countText = `${total}`;
        const details = [];
        if (images > 0) details.push(`${images}图`);
        if (animations > 0) details.push(`${animations}动图`);
        if (videos > 0) details.push(`${videos}视频`);
        
        if (details.length > 0) {
            countText += ` (${details.join(' + ')})`;
        }
        
        filesCount.textContent = countText;
    }
    
    // 更新文件列表（按媒体类型分组显示）
    if (filesList) {
        let html = '';
        
        // 静态图片
        if (state.mediaTypes.images.length > 0) {
            html += `<div class="media-group-title">📷 静态图片 (${state.mediaTypes.images.length})</div>`;
            html += state.mediaTypes.images.map(file => renderFileItem(file)).join('');
        }
        
        // 动图
        if (state.mediaTypes.animations.length > 0) {
            html += `<div class="media-group-title">🎞️ 动图 (${state.mediaTypes.animations.length})</div>`;
            html += state.mediaTypes.animations.map(file => renderFileItem(file)).join('');
        }
        
        // 视频
        if (state.mediaTypes.videos.length > 0) {
            html += `<div class="media-group-title">🎬 视频 (${state.mediaTypes.videos.length})</div>`;
            html += state.mediaTypes.videos.map(file => renderFileItem(file)).join('');
        }
        
        filesList.innerHTML = html;
    }
}

function renderFileItem(file) {
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
    console.log('[PIXLY AI] 已清空文件列表');
}

// ==================== Rust核心检测 ====================
async function detectRustCore() {
    console.log('[PIXLY AI] 🔍 开始检测Rust核心...');
    
    try {
        const { spawn } = require('child_process');
        const path = require('path');
        const os = require('os');
        
        // 🔥 根据操作系统确定可执行文件名
        const platform = os.platform();
        const exeName = platform === 'win32' ? 'pixly-rust.exe' : 'pixly-rust';
        
        // 尝试多个可能的路径
        const possiblePaths = [
            path.join(__dirname, '..', 'bin', exeName),
            path.join(__dirname, '..', '..', 'bin', exeName),
            path.join(__dirname, '..', '..', '..', 'bin', exeName),
            path.join(process.cwd(), 'bin', exeName),
            exeName  // 系统PATH中
        ];
        
        console.log('[PIXLY AI] 尝试路径:', possiblePaths);
        
        for (const rustPath of possiblePaths) {
            try {
                console.log(`[PIXLY AI] 尝试: ${rustPath}`);
                
                const proc = spawn(rustPath, ['--version'], { 
                    timeout: 3000,
                    stdio: ['ignore', 'pipe', 'pipe']
                });
                
                let output = '';
                let error = '';
                
                proc.stdout.on('data', (data) => {
                    output += data.toString();
                });
                
                proc.stderr.on('data', (data) => {
                    error += data.toString();
                });
                
                const code = await new Promise((resolve, reject) => {
                    proc.on('close', resolve);
                    proc.on('error', reject);
                    setTimeout(() => reject(new Error('timeout')), 3000);
                });
                
                if (code === 0 && output) {
                    state.rustCorePath = rustPath;
                    console.log(`[PIXLY AI] ✅ Rust核心已找到: ${rustPath}`);
                    console.log(`[PIXLY AI] 版本: ${output.trim()}`);
                    console.log(`[PIXLY AI] 🔥 AI驱动架构已激活`);
                    return;
                } else {
                    console.log(`[PIXLY AI] ❌ 路径无效: ${rustPath} (code: ${code})`);
                }
            } catch (e) {
                console.log(`[PIXLY AI] ❌ 路径失败: ${rustPath} (${e.message})`);
                continue;
            }
        }
        
        console.error('[PIXLY AI] ⚠️ 未找到Rust核心');
        console.error('[PIXLY AI] 💡 请确保已编译Rust核心: cargo build --release');
        console.error('[PIXLY AI] 💡 或将pixly-rust放在bin/目录下');
        
    } catch (error) {
        console.error('[PIXLY AI] Rust核心检测失败:', error);
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
    
    console.log('[PIXLY AI] 🚀 开始转换');
    console.log(`[PIXLY AI] 预设: ${state.aiPreset}`);
    console.log(`[PIXLY AI] 功能:`, state.aiFeatures);
    
    try {
        const results = await convertFiles();
        showResults(results);
    } catch (error) {
        console.error('[PIXLY AI] 转换失败:', error);
        alert(`转换失败: ${error.message}`);
    } finally {
        state.isConverting = false;
        if (convertBtn) {
            convertBtn.disabled = false;
            convertBtn.querySelector('.btn-text').textContent = '开始智能转换';
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
            console.error(`[PIXLY AI] 转换失败: ${file.name}`, error);
            results.failed++;
        }
    }
    
    return results;
}

function buildConversionArgs(file) {
    const args = ['convert', file.filePath];
    
    // 🔥 AI预设（必须传递给Rust核心）
    args.push('--ai-preset', state.aiPreset);
    
    // 🔥 AI功能开关（传递给Rust核心）
    if (state.aiFeatures.smartQuality) {
        args.push('--ai-quality');  // Rust核心的AI质量预测
    }
    
    if (state.aiFeatures.formatRecommend) {
        args.push('--ai-format');  // Rust核心的格式推荐
    }
    
    if (state.aiFeatures.ssimValidation) {
        args.push('--ssim-threshold', '0.95');  // SSIM验证阈值
    }
    
    // 🔥 动图转视频（大型动图自动转MP4）
    if (state.aiFeatures.videoForAnimation) {
        args.push('--animation-to-video');
        args.push('--animation-threshold', '10485760');  // 10MB以上的动图转视频
    }
    
    // 🔥 输出目录
    const path = require('path');
    const outputDir = path.join(path.dirname(file.filePath), 'pixly_output');
    args.push('--output', outputDir);
    
    // 🔥 强制使用AI模式（不允许fallback）
    args.push('--no-fallback');
    
    // 🔥 详细日志
    args.push('--verbose');
    
    console.log('[PIXLY AI] Rust CLI参数:', args.join(' '));
    
    return args;
}

function executeRustCLI(args) {
    return new Promise((resolve, reject) => {
        const { spawn } = require('child_process');
        
        console.log('[PIXLY AI] 🚀 调用Rust核心...');
        console.log('[PIXLY AI] 命令:', state.rustCorePath);
        console.log('[PIXLY AI] 参数:', args.join(' '));
        
        const proc = spawn(state.rustCorePath, args, {
            stdio: ['ignore', 'pipe', 'pipe']
        });
        
        let stdout = '';
        let stderr = '';
        
        proc.stdout.on('data', (data) => {
            const text = data.toString();
            stdout += text;
            // 实时输出Rust日志
            console.log('[Rust]', text.trim());
        });
        
        proc.stderr.on('data', (data) => {
            const text = data.toString();
            stderr += text;
            // 实时输出Rust错误
            console.error('[Rust Error]', text.trim());
        });
        
        proc.on('close', (code) => {
            console.log(`[PIXLY AI] Rust进程退出: code=${code}`);
            
            if (code === 0) {
                console.log('[PIXLY AI] ✅ 转换成功');
                resolve(stdout);
            } else {
                console.error('[PIXLY AI] ❌ 转换失败');
                console.error('[PIXLY AI] 错误输出:', stderr);
                reject(new Error(stderr || `Rust exit code: ${code}`));
            }
        });
        
        proc.on('error', (err) => {
            console.error('[PIXLY AI] ❌ 进程错误:', err);
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
    
    console.log('[PIXLY AI] ✅ 转换完成', results);
}

function openOutputFolder() {
    if (state.selectedFiles.length === 0) return;
    
    const path = require('path');
    const { shell } = require('electron');
    
    const firstFile = state.selectedFiles[0];
    const outputDir = path.join(path.dirname(firstFile.filePath), 'pixly_output');
    
    shell.openPath(outputDir);
}
