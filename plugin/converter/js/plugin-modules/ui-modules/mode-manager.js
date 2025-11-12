/**
 * Mode Manager - 模式管理模块
 * 负责管理智能模式和手动模式的切换
 * 从ui-handlers.js拆分
 */

export class ModeManager {
    constructor() {
        this.log = window.pixlyLog || console;
        this.currentMode = 'smart';
        this.serviceGuideShown = false;
    }

    /**
     * 初始化模式管理器
     */
    initialize() {
        // 监听模式变化
        if (window.eventManager) {
            window.eventManager.on('modeChanged', (mode) => {
                this.handleModeChange(mode);
            });
        }

        // 检测核心服务状态
        this.detectCoreServices();
        
        this.log.info('[ModeManager] ✅ Initialized');
    }

    /**
     * 处理模式切换
     */
    handleModeChange(mode) {
        this.currentMode = mode;
        
        const formatSection = document.getElementById('formatSection');
        const projectInfoSection = document.getElementById('projectInfoSection');
        const smartModeSection = document.getElementById('smartModeSection');
        const smartModeOptions = document.getElementById('smartModeOptions');
        const manualModeSection = document.getElementById('manualModeSection');
        const manualModeOptions = document.getElementById('manualModeOptions');
        const jpegNotice = document.getElementById('jpegLosslessNotice');
        const scopeFilterSection = document.getElementById('scopeFilterSection');

        if (mode === 'smart') {
            // 智能模式
            if (formatSection) formatSection.style.display = 'none';
            if (projectInfoSection) projectInfoSection.style.display = 'block';
            if (smartModeSection) smartModeSection.style.display = 'block';
            if (smartModeOptions) smartModeOptions.style.display = 'block';
            if (manualModeSection) manualModeSection.style.display = 'none';
            if (manualModeOptions) manualModeOptions.style.display = 'none';
            if (jpegNotice) jpegNotice.style.display = 'none';
            if (scopeFilterSection) scopeFilterSection.style.display = 'block';
            
            this.log.info('[ModeManager] Switched to SMART mode');
            
            // 智能模式下检测GO核心
            this.detectGoCore();
            
        } else if (mode === 'manual') {
            // 手动模式
            if (formatSection) formatSection.style.display = 'block';
            if (projectInfoSection) projectInfoSection.style.display = 'none';
            if (smartModeSection) smartModeSection.style.display = 'none';
            if (smartModeOptions) smartModeOptions.style.display = 'none';
            if (manualModeSection) manualModeSection.style.display = 'block';
            if (manualModeOptions) manualModeOptions.style.display = 'block';
            if (jpegNotice) jpegNotice.style.display = 'block';
            if (scopeFilterSection) scopeFilterSection.style.display = 'none';
            
            this.log.info('[ModeManager] Switched to MANUAL mode');
            
            // 手动模式检测
            this.checkManualModeRequirements();
            
            // 更新JPEG提示
            if (window.updateJPEGNotice) {
                window.updateJPEGNotice();
            }
        } else {
            this.log.warn(`[ModeManager] Unknown mode: ${mode}`);
        }
        
        // 更新标签样式
        this.updateTabStyles(mode);
    }

    /**
     * 更新标签样式
     */
    updateTabStyles(mode) {
        const modeTabs = document.querySelectorAll('#imageConversionPanel .mode-tab');
        modeTabs.forEach(tab => {
            const tabMode = tab.getAttribute('data-mode');
            if (tabMode === mode) {
                tab.classList.add('active');
            } else {
                tab.classList.remove('active');
            }
        });
    }

    /**
     * 检测核心服务状态
     */
    async detectCoreServices() {
        const rustStatus = await this.detectRustCore();
        const goStatus = this.currentMode === 'smart' ? await this.detectGoCore() : null;
        
        this.updateServiceStatus(rustStatus, goStatus);
    }

    /**
     * 检测Rust核心
     */
    async detectRustCore() {
        try {
            const { exec } = require('child_process');
            const { promisify } = require('util');
            const execAsync = promisify(exec);
            
            await execAsync('pixly-rust --version');
            this.log.info('[ModeManager] ✅ Rust core detected');
            return true;
        } catch (error) {
            this.log.warn('[ModeManager] ❌ Rust core not detected');
            return false;
        }
    }

    /**
     * 检测Go AI服务
     */
    async detectGoCore() {
        if (this._goCoreCache !== undefined) {
            return this._goCoreCache;
        }
        
        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 1000);
            
            const response = await fetch('http://localhost:50052/api/v1/version', {
                signal: controller.signal,
                method: 'GET',
                headers: { 'Accept': 'application/json' }
            });
            
            clearTimeout(timeoutId);
            
            if (response.ok) {
                const data = await response.json();
                this._goCoreCache = true;
                this.log.info('[ModeManager] ✅ Go AI service detected:', data);
                return true;
            }
        } catch (error) {
            this._goCoreCache = false;
            this.log.warn('[ModeManager] ❌ Go AI service not available');
        }
        
        return false;
    }

    /**
     * 检查手动模式要求
     */
    async checkManualModeRequirements() {
        const rustAvailable = await this.detectRustCore();
        
        if (!rustAvailable) {
            this.showServiceStartGuide();
            this.disableManualModeControls();
        } else {
            this.enableManualModeControls();
        }
    }

    /**
     * 显示服务启动指南
     */
    showServiceStartGuide() {
        if (this.serviceGuideShown) {
            this.log.warn('[ModeManager] Core services still not available');
            return;
        }
        
        this.serviceGuideShown = true;
        
        const guideMessage = `
⚠️ 核心服务未启动

请启动以下服务：

1. Rust核心服务:
   cd core/rust
   cargo build --release
   cargo install --path .

2. Go AI服务 (智能模式需要):
   cd core/go
   go run cmd/pixly-ai/main.go --port 50052
`;
        
        this.log.error('[ModeManager] ' + guideMessage);
        
        // 显示在UI中
        const statusElement = document.getElementById('serviceStatus');
        if (statusElement) {
            statusElement.innerHTML = `<pre style="color: #ff6b6b;">${guideMessage}</pre>`;
            statusElement.style.display = 'block';
        }
    }

    /**
     * 禁用手动模式控件
     */
    disableManualModeControls() {
        const controlIds = [
            'formatJPEG', 'formatPNG', 'formatWebP', 'formatAVIF', 
            'formatJXL', 'formatHEIC',
            'quality', 'manualLossless', 'manualNearLossless',
            'jxlEffort', 'jxlDistance', 'jxlPatches', 'jxlBitDepth',
            'webpMethod', 'webpFilterStrength', 'webpSharpness',
            'webpSegments', 'webpSnsStrength', 'webpPass',
            'avifSpeed', 'avifMinQuantizer', 'avifMaxQuantizer',
            'avifTilesRows', 'avifTilesCols',
            'heicQuality',
            'convertBtn'
        ];
        
        controlIds.forEach(id => {
            const element = document.getElementById(id);
            if (element) {
                element.disabled = true;
                element.classList.add('disabled');
            }
        });
        
        this.log.info('[ModeManager] Manual mode controls disabled');
    }

    /**
     * 启用手动模式控件
     */
    enableManualModeControls() {
        const controlIds = [
            'formatJPEG', 'formatPNG', 'formatWebP', 'formatAVIF', 
            'formatJXL', 'formatHEIC',
            'quality', 'manualLossless', 'manualNearLossless',
            'jxlEffort', 'jxlDistance', 'jxlPatches', 'jxlBitDepth',
            'webpMethod', 'webpFilterStrength', 'webpSharpness',
            'webpSegments', 'webpSnsStrength', 'webpPass',
            'avifSpeed', 'avifMinQuantizer', 'avifMaxQuantizer',
            'avifTilesRows', 'avifTilesCols',
            'heicQuality',
            'convertBtn'
        ];
        
        controlIds.forEach(id => {
            const element = document.getElementById(id);
            if (element) {
                element.disabled = false;
                element.classList.remove('disabled');
            }
        });
        
        this.log.info('[ModeManager] Manual mode controls enabled');
    }

    /**
     * 更新服务状态显示
     */
    updateServiceStatus(rustStatus, goStatus) {
        // 更新Rust核心状态图标
        const rustCoreInline = document.getElementById('rustCoreInline');
        if (rustCoreInline) {
            const statusIcon = rustStatus ? '✓' : '✗';
            const statusClass = rustStatus ? 'core-available' : 'core-unavailable';
            rustCoreInline.innerHTML = `<span class="${statusClass}">⚙️ ${statusIcon}</span>`;
        }
        
        // 更新Go AI服务状态图标（智能模式时）
        if (this.currentMode === 'smart') {
            const goCoreInline = document.getElementById('goCoreInline');
            if (goCoreInline) {
                const statusIcon = goStatus ? '✓' : '✗';
                const statusClass = goStatus ? 'core-available' : 'core-unavailable';
                goCoreInline.innerHTML = `<span class="${statusClass}">🤖 ${statusIcon}</span>`;
                goCoreInline.style.display = 'inline-block';
            }
        }
    }

    /**
     * 获取当前模式
     */
    getCurrentMode() {
        return this.currentMode;
    }

    /**
     * 强制切换到指定模式
     */
    switchToMode(mode) {
        const radioId = mode === 'smart' ? 'modeRadioSmart' : 'modeRadioManual';
        const radio = document.getElementById(radioId);
        
        if (radio) {
            radio.checked = true;
            radio.dispatchEvent(new Event('change', { bubbles: true }));
        }
    }
}

// 创建单例并导出
const modeManager = new ModeManager();
export default modeManager;
