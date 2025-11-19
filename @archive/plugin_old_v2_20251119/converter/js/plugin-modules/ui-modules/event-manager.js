/**
 * Event Manager - 事件管理模块
 * 负责管理所有UI事件的绑定和触发
 * 从ui-handlers.js拆分
 */

export class EventManager {
    constructor() {
        this.log = window.pixlyLog || console;
        this.listeners = new Map();
        this.initialized = false;
    }

    /**
     * 初始化事件管理器
     */
    initialize() {
        if (this.initialized) {
            this.log.warn('[EventManager] Already initialized');
            return;
        }

        this.bindFormatEvents();
        this.bindModeEvents();
        this.bindConversionEvents();
        this.bindLivePhotoEvents();
        this.bindParameterEvents();
        
        this.initialized = true;
        this.log.info('[EventManager] ✅ All events bound');
    }

    /**
     * 绑定格式相关事件
     */
    bindFormatEvents() {
        const formatOptions = document.querySelectorAll('input[name="format"]');
        
        formatOptions.forEach(option => {
            option.addEventListener('change', () => {
                this.emit('formatChanged', option.value);
                
                // 更新格式专属参数
                if (window.updateFormatSpecificParams) {
                    window.updateFormatSpecificParams();
                }
                
                // 更新参数可用性
                if (window.updateManualParamsAvailability) {
                    window.updateManualParamsAvailability();
                }
                
                this.log.debug(`[EventManager] Format changed to ${option.value}`);
            });
        });
        
        // 初始化时调用一次
        if (window.updateFormatSpecificParams) {
            window.updateFormatSpecificParams();
        }
    }

    /**
     * 绑定模式切换事件
     */
    bindModeEvents() {
        // Radio模式切换
        const modeRadios = document.querySelectorAll('input[name="mode"]');
        modeRadios.forEach(radio => {
            radio.addEventListener('change', (e) => {
                this.emit('modeChanged', e.target.value);
                if (window.handleModeChange) {
                    window.handleModeChange(e);
                }
            });
        });

        // 视频输入类型切换
        const videoInputTypes = document.querySelectorAll('input[name="videoInputType"]');
        videoInputTypes.forEach(radio => {
            radio.addEventListener('change', (e) => {
                this.emit('videoInputTypeChanged', e.target.value);
                this.log.debug(`[EventManager] Video input type changed to ${e.target.value}`);
            });
        });
    }

    /**
     * 绑定转换相关事件
     */
    bindConversionEvents() {
        // 标准转换按钮
        const convertNormalBtn = document.getElementById('convertNormal');
        if (convertNormalBtn) {
            convertNormalBtn.addEventListener('click', () => {
                this.handleConversion('normal');
            });
        }
        
        // JPEG→JXL转换按钮
        const convertJXLBtn = document.getElementById('convertJXL');
        if (convertJXLBtn) {
            convertJXLBtn.addEventListener('click', () => {
                this.handleConversion('jxl');
            });
        }
        
        // 主转换按钮
        const convertBtn = document.getElementById('convertBtn');
        if (convertBtn) {
            convertBtn.addEventListener('click', (e) => {
                // 如果不是点击下拉箭头，执行转换
                const rect = convertBtn.getBoundingClientRect();
                const clickX = e.clientX - rect.left;
                const buttonWidth = rect.width;
                
                if (clickX <= buttonWidth - 30) {
                    const mode = convertBtn.classList.contains('jxl-mode') ? 'jxl' : 'normal';
                    this.handleConversion(mode);
                }
            });
        }
    }

    /**
     * 处理转换操作
     */
    handleConversion(mode) {
        this.log.info(`[EventManager] Starting ${mode} conversion`);
        
        if (mode === 'jxl') {
            // 设置JXL专用参数
            this.setupJXLConversion();
        }
        
        // 触发转换
        if (window.convertImages) {
            window.convertImages();
        }
        
        // 隐藏下拉菜单
        const convertDropdown = document.getElementById('convertDropdown');
        if (convertDropdown) {
            convertDropdown.classList.remove('show');
        }
    }

    /**
     * 设置JXL转换参数
     */
    setupJXLConversion() {
        // 强制选择JXL格式
        const jxlRadio = document.getElementById('formatJXL');
        if (jxlRadio) {
            jxlRadio.checked = true;
            jxlRadio.dispatchEvent(new Event('change', { bubbles: true }));
        }
        
        // 启用JPEG无损选项
        const enableJpegLossless = document.getElementById('enableJpegLossless');
        if (enableJpegLossless) {
            enableJpegLossless.checked = true;
        }
        
        // 手动模式下也启用无损
        const manualLossless = document.getElementById('manualLossless');
        if (manualLossless) {
            manualLossless.checked = true;
            manualLossless.dispatchEvent(new Event('change', { bubbles: true }));
        }
        
        this.log.debug('[EventManager] JXL conversion parameters set');
    }

    /**
     * 绑定Live Photo相关事件
     */
    bindLivePhotoEvents() {
        const skipLivePhotos = document.getElementById('skipLivePhotos');
        
        if (skipLivePhotos) {
            skipLivePhotos.addEventListener('change', (e) => {
                const skipEnabled = e.target.checked;
                this.emit('skipLivePhotosChanged', skipEnabled);
                this.log.info(`[EventManager] Skip Live Photos: ${skipEnabled ? 'enabled' : 'disabled'}`);
            });
        }
    }

    /**
     * 绑定参数调整事件
     */
    bindParameterEvents() {
        // 无损选项监听器
        const manualLossless = document.getElementById('manualLossless');
        if (manualLossless) {
            manualLossless.addEventListener('change', () => {
                if (window.updateManualParamsAvailability) {
                    window.updateManualParamsAvailability();
                }
                this.emit('losslessChanged', manualLossless.checked);
            });
        }
        
        // 近无损选项监听器
        const manualNearLossless = document.getElementById('manualNearLossless');
        if (manualNearLossless) {
            manualNearLossless.addEventListener('change', () => {
                if (window.updateManualParamsAvailability) {
                    window.updateManualParamsAvailability();
                }
                this.emit('nearLosslessChanged', manualNearLossless.checked);
            });
        }

        // 启用JPEG无损选项
        const enableJpegLossless = document.getElementById('enableJpegLossless');
        if (enableJpegLossless) {
            enableJpegLossless.addEventListener('change', (e) => {
                this.emit('jpegLosslessChanged', e.target.checked);
                if (window.updateJPEGNotice) {
                    window.updateJPEGNotice();
                }
            });
        }
    }

    /**
     * 注册事件监听器
     */
    on(event, callback) {
        if (!this.listeners.has(event)) {
            this.listeners.set(event, []);
        }
        this.listeners.get(event).push(callback);
    }

    /**
     * 取消事件监听器
     */
    off(event, callback) {
        if (!this.listeners.has(event)) return;
        
        const callbacks = this.listeners.get(event);
        const index = callbacks.indexOf(callback);
        
        if (index > -1) {
            callbacks.splice(index, 1);
        }
    }

    /**
     * 触发事件
     */
    emit(event, ...args) {
        if (!this.listeners.has(event)) return;
        
        const callbacks = this.listeners.get(event);
        callbacks.forEach(callback => {
            try {
                callback(...args);
            } catch (error) {
                this.log.error(`[EventManager] Error in event handler for ${event}:`, error);
            }
        });
    }

    /**
     * 清理所有事件监听器
     */
    cleanup() {
        this.listeners.clear();
        this.initialized = false;
        this.log.info('[EventManager] All event listeners cleaned up');
    }
}

// 创建单例并导出
const eventManager = new EventManager();
export default eventManager;
