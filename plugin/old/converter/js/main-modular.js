/**
 * PIXLY V3 - module化主入口
 * 使用 ES6 Modules 架构
 */

// �� 统一日志实例
const log = window.pixlyLog;

// 导入all核心module
import * as constants from './modules/constants.js';
import * as utils from './modules/utils.js';
import * as logger from './modules/logger.js';
import * as state from './modules/state.js';
import * as ui from './modules/ui.js';
import * as eagleAPI from './modules/eagle-api.js';
import * as fileHandler from './modules/file-handler.js';
import * as pixly from './modules/pixly.js';
import * as i18nUI from './modules/i18n-ui.js';
import * as video from './modules/video.js';

// 导入AI相关module
import { initAIIntegration, setupAIModeListeners, getAIPrediction } from './modules/ai-integration.js';

// 导入独立工具
import { EightLayerValidator, MediaDeduplicator } from './modules/standalone/index.js';

// ==================== 全局变量 ====================
let pluginInstance = null;
let selectedFiles = [];
let conversionProcess = null;
let isConverting = false;

// 暴露towindow（as兼容EaglepluginAPI）
window.pixlyModules = {
    constants,
    utils,
    logger,
    state,
    ui,
    eagleAPI,
    fileHandler,
    pixly,
    i18nUI,
    video
};

// ==================== Eagle plugin生命周期 ====================
eagle.onPluginCreate((plugin) => {
    log.info('PIXLY plugincreated（modular version）');
    pluginInstance = plugin;
    
    // initialize各个module
    initializeModules();
    
    // initialize主题
    initThemeToggle();
    
    // initializeAI
    initAIFeatures();
});

eagle.onPluginShow(() => {
    log.info('Plugin shown');
    refreshFileList();
});

eagle.onPluginRun(() => {
    log.info('Plugin running');
});

eagle.onPluginHide(() => {
    log.info('Plugin hidden');
});

eagle.onPluginBeforeExit(() => {
    log.info('Plugin about to exit');
    cleanupBeforeExit();
});

// ==================== moduleinitialize ====================
async function initializeModules() {
    log.info('[Main] Initializing all modules...');
    
    // Initialize logger
    if (logger.initLogger) {
        logger.initLogger();
    }
    
    // initializeUI
    if (ui.initUI) {
        ui.initUI();
    }
    
    // initialize状态管理
    if (state.initState) {
        state.initState();
    }
    
    // initializefile处理器
    if (fileHandler.initFileHandler) {
        fileHandler.initFileHandler();
    }
    
    // initialize视频控制
    if (video.initVideoControls) {
        video.initVideoControls();
    }
    
    // initialize国际化UI
    if (i18nUI.initI18nUI) {
        i18nUI.initI18nUI();
    }
    
    // initializePixlydetect
    if (pixly.initPixlyDetection) {
        pixly.initPixlyDetection();
    }
    
    // initialize事件监听器
    initEventListeners();
    
    log.info('[Main] ✅ allAll modules initialized');
}

// ==================== AI功能initialize ====================
async function initAIFeatures() {
    log.info('[Main] Initializing AI features...');
    
    try {
        await initAIIntegration();
        setupAIModeListeners();
        log.info('[Main] ✅ AI featuresInitializingcomplete');
    } catch (error) {
        log.error('[Main] ❌ AI featuresInitializingfailed:', error);
    }
}

// ==================== 事件监听器 ====================
function initEventListeners() {
    // 转换按钮
    const convertBtn = document.getElementById('convertBtn');
    if (convertBtn) {
        convertBtn.addEventListener('click', handleConvertClick);
    }
    
    // refresh按钮
    const refreshBtn = document.getElementById('refreshBtn');
    if (refreshBtn) {
        refreshBtn.addEventListener('click', refreshFileList);
    }
    
    // 模式切换
    const modeTabs = document.querySelectorAll('.mode-tab');
    modeTabs.forEach(tab => {
        tab.addEventListener('click', handleModeTabClick);
    });
    
    // 面板提示监听
    initPanelHintListeners();
}

// ==================== 转换处理 ====================
async function handleConvertClick() {
    if (isConverting) {
        logger.addLog('⚠️ 转换currentlyin progress...', 'warning');
        return;
    }
    
    const files = await getSelectedFiles();
    if (!files || files.length === 0) {
        logger.addLog('⚠️ 请先选择file', 'warning');
        return;
    }
    
    isConverting = true;
    
    try {
        logger.addLog(`start转换 ${files.length} 个file...`);
        
        for (const file of files) {
            // 检查is否启用AI预测
            let aiParams = null;
            if (window.aiClient && window.aiClient.enabled) {
                aiParams = await getAIPrediction(file, state.getTargetFormat());
            }
            
            // 执行转换
            await convertFile(file, aiParams);
        }
        
        logger.addLog('✅ allfile转换complete', 'success');
        
    } catch (error) {
        logger.addLog(`❌ 转换failed: ${error.message}`, 'error');
    } finally {
        isConverting = false;
    }
}

async function convertFile(file, aiParams) {
    logger.addLog(`转换: ${file.name}`);
    
    // 获取转换parameter
    const params = aiParams || state.getConversionParams();
    
    // 调用Pixly进行转换
    if (pixly.convertWithPixly) {
        await pixly.convertWithPixly(file, params);
    }
}

// ==================== file列表管理 ====================
async function getSelectedFiles() {
    if (eagleAPI.getSelectedItems) {
        return await eagleAPI.getSelectedItems();
    }
    return [];
}

async function refreshFileList() {
    logger.addLog('refreshfile列表...');
    selectedFiles = await getSelectedFiles();
    
    if (ui.updateFileList) {
        ui.updateFileList(selectedFiles);
    }
}

// ==================== 模式切换 ====================
function handleModeTabClick(event) {
    const mode = event.target.getAttribute('data-mode');
    
    // update标签激活状态
    document.querySelectorAll('.mode-tab').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
    
    // update状态
    if (state.setMode) {
        state.setMode(mode);
    }
    
    // updateUI显示
    if (ui.updateModeUI) {
        ui.updateModeUI(mode);
    }
}

// ==================== 主题切换 ====================
function initThemeToggle() {
    const themeToggle = document.getElementById('themeToggle');
    if (!themeToggle) return;
    
    themeToggle.addEventListener('click', () => {
        const currentTheme = document.body.getAttribute('data-theme') || 'light';
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        document.body.setAttribute('data-theme', newTheme);
        localStorage.setItem('pixly-theme', newTheme);
    });
    
    // loadsave主题
    const savedTheme = localStorage.getItem('pixly-theme') || 'light';
    document.body.setAttribute('data-theme', savedTheme);
}

// ==================== 面板提示 ====================
function initPanelHintListeners() {
    const instructionsDetails = document.querySelector('.advanced-control-panel details');
    const logDetails = document.querySelector('.log-panel details');
    
    if (instructionsDetails) {
        instructionsDetails.addEventListener('toggle', updatePanelHints);
    }
    
    if (logDetails) {
        logDetails.addEventListener('toggle', updatePanelHints);
    }
}

function updatePanelHints() {
    // update面板提示状态
    const lang = window.i18n?.getLanguage?.() || 'zh';
    
    // updatelog计数
    const logBadge = document.getElementById('logCountBadge');
    const logOutput = document.getElementById('logOutput');
    
    if (logBadge && logOutput) {
        const count = logOutput.querySelectorAll('.log-entry').length;
        logBadge.textContent = count > 0 ? `${count}` : (lang === 'zh' ? '空' : 'Empty');
    }
}

// ==================== 清理 ====================
function cleanupBeforeExit() {
    if (conversionProcess) {
        conversionProcess.kill();
    }
}

// ==================== DOMloadcomplete后initialize ====================
document.addEventListener('DOMContentLoaded', async function() {
    log.info('[Main] DOMLoadingcomplete，StartingInitializing...');
    
    initThemeToggle();
    
    // waitingEaglepluginAPI准备就绪
    if (typeof eagle !== 'undefined') {
        log.info('[Main] ✅ Eagle API ready');
    } else {
        log.warn('[Main] ⚠️ Eagle API not ready');
    }
});

// 导出主要函数供debug使用
window.pixlyMain = {
    refreshFileList,
    handleConvertClick,
    getSelectedFiles,
    initializeModules,
    initAIFeatures
};
