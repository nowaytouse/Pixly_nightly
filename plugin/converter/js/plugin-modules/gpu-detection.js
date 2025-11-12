/**
 * PIXLY GPU detectedand管理module
 * 依赖: PIXLY命名空间, child_process
 */
(function(window) {
    'use strict';
    
    window.PIXLY = window.PIXLY || {};
    
    // GPU cache
    let gpuCache = null;
    
    /**
     * GPU 管理器
     */
    const GPUDetector = {
        
        /**
         * GPU 编码器列表
         */
        gpuEncoders: [
            // AV1 GPU 编码器（最优先）
            { codec: 'av1_nvenc', type: 'NVIDIA AV1 (RTX 40系列+)', boost: 10.0, name: 'av1' },
            { codec: 'av1_qsv', type: 'Intel AV1 (Arc系列)', boost: 5.0, name: 'av1' },
            { codec: 'av1_amf', type: 'AMD AV1 (RX 7000系列+)', boost: 7.0, name: 'av1' },
            
            // H.265 GPU 编码器
            { codec: 'hevc_videotoolbox', type: 'Apple H.265 (VideoToolbox)', boost: 7.0, name: 'h265' },
            { codec: 'hevc_nvenc', type: 'NVIDIA H.265', boost: 8.0, name: 'h265' },
            { codec: 'hevc_qsv', type: 'Intel H.265 (QuickSync)', boost: 4.0, name: 'h265' },
            { codec: 'hevc_amf', type: 'AMD H.265 (VCE)', boost: 6.0, name: 'h265' },
            
            // H.264 GPU 编码器
            { codec: 'h264_videotoolbox', type: 'Apple H.264 (VideoToolbox)', boost: 7.0, name: 'h264' },
            { codec: 'h264_nvenc', type: 'NVIDIA H.264', boost: 8.0, name: 'h264' },
            { codec: 'h264_qsv', type: 'Intel H.264 (QuickSync)', boost: 4.0, name: 'h264' },
            { codec: 'h264_amf', type: 'AMD H.264 (VCE)', boost: 6.0, name: 'h264' }
        ],
        
        /**
         * detected GPU
         */
        detectGPU: async function() {
            const exec = require('child_process').exec;
            const util = require('util');
            const execPromise = util.promisify(exec);
            
            if (window.addLog) {
                const i18n = window.i18n || { t: (key) => key };
                addLog(i18n.t('video.detectingGpu'), 'info');
            }
            
            try {
                // settings完整 PATH
                const env = Object.assign({}, require('process').env);
                env.PATH = `/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:${env.PATH || ''}`;
                
                const { stdout } = await execPromise('ffmpeg -codecs', { env });
                
                // detectedallavailable GPU 编码器
                const availableGPU = {
                    available: false,
                    type: 'Software',
                    codecs: {}
                };
                
                for (const enc of this.gpuEncoders) {
                    if (stdout.includes(enc.codec)) {
                        availableGPU.available = true;
                        availableGPU.type = enc.type;
                        availableGPU.boost = enc.boost;
                        
                        // 根据编码器类型映射to对应 codec
                        if (enc.codec.includes('h264')) {
                            availableGPU.codecs.h264 = enc.codec;
                        } else if (enc.codec.includes('hevc') || enc.codec.includes('h265')) {
                            availableGPU.codecs.h265 = enc.codec;
                        } else if (enc.codec.includes('av1')) {
                            availableGPU.codecs.av1 = enc.codec;
                        }
                    }
                }
                
                gpuCache = availableGPU;
                
                                if (availableGPU.available) {
                    const codecList = Object.keys(availableGPU.codecs).map(c => c.toUpperCase()).join(', ');                                                                      
                    if (window.addLog) {
                        addLog(i18n.t('messages.log.gpuAvailable', {type: availableGPU.type, boost: availableGPU.boost}), 'success');                                            
                        addLog(i18n.t('messages.log.gpuCodecs', {codecs: codecList}), 'info');
                    }
                    this.updateGPUUI(availableGPU);
                } else {
                    if (window.addLog) {
                        addLog(i18n.t('messages.log.gpuNotDetected'), 'info');
                    }
                }
                
                return availableGPU;
                
            } catch (error) {
                if (window.addLog) {
                    addLog(i18n.t('messages.log.gpuDetectionFailed', {error: error.message}), 'warning');
                }
                return { available: false };
            }
        },
        
        /**
         * initialized GPU detected
         */
        initGPUDetection: async function() {
            const result = await this.detectGPU();
            
            if (!result.available) {
                // 隐藏 GPU options
                const gpuCard = document.querySelector('.codec-card.gpu-enabled');
                if (gpuCard) gpuCard.style.display = 'none';
                
                // 更New GPU 状态显示
                const gpuInfo = document.getElementById('gpuInfo');
                if (gpuInfo) {
                    gpuInfo.innerHTML = i18n.t('messages.log.gpuNotAvailable');
                    gpuInfo.style.color = '#888';
                }
            }
        },
        
        /**
         * 更New GPU UI
         */
                updateGPUUI: function(gpu) {
            const gpuInfo = document.getElementById('gpuInfo');
            if (gpuInfo) {
                const availableText = i18n.t('video.gpuAvailableText');
                const accelerationText = i18n.t('video.gpuAcceleration');
                gpuInfo.innerHTML = `
                    ${availableText}: ${gpu.type}
                    <br><span style="color: #667eea; font-weight: bold;">${accelerationText}: ${gpu.boost}x</span>                                                                           
                `;
            }
            
            // 显示 GPU 编码器options
            const gpuCard = document.querySelector('.codec-card.gpu-enabled');
            if (gpuCard) {
                gpuCard.style.display = 'flex';
                
                // 更New GPU 卡片info
                const gpuStats = document.getElementById('gpuCodecStats');
                if (gpuStats) {
                    const i18n = window.i18n || { t: (key) => key };
                    gpuStats.textContent = `${gpu.type} | ${gpu.boost}x ${i18n.t('video.gpuAcceleration')}`;
                }
            }
        },
        
        /**
         * 获取 GPU cache
         */
        getGPUCache: function() {
            return gpuCache;
        },
        
        /**
         * Checkis否can使用 GPU
         */
        canUseGPU: function(codec) {
            // H.266 不支持 GPU 加速
            if (codec === 'h266') return false;
            
            return gpuCache && gpuCache.available && gpuCache.codecs[codec];
        },
        
        /**
         * 获取 GPU 编码器
         */
        getGPUCodec: function(codec) {
            if (!gpuCache || !gpuCache.codecs) return null;
            return gpuCache.codecs[codec];
        },
        
        /**
         * 获取 GPU 类型
         */
        getGPUType: function() {
            if (!gpuCache) return 'Software';
            return gpuCache.available ? gpuCache.type : 'Software';
        },
        
        /**
         * 获取 GPU 加速倍数
         */
        getGPUBoost: function() {
            if (!gpuCache || !gpuCache.available) return 1.0;
            return gpuCache.boost || 1.0;
        },
        
        /**
         * is否有 GPU available
         */
        isGPUAvailable: function() {
            return gpuCache && gpuCache.available;
        }
    };
    
    // 暴露to全局
    PIXLY.GPUDetector = GPUDetector;
    
    // 向后兼容
    window.detectGPU = function() {
        return GPUDetector.detectGPU();
    };
    
    window.initGPUDetection = function() {
        return GPUDetector.initGPUDetection();
    };
    
    window.updateGPUUI = function(gpu) {
        GPUDetector.updateGPUUI(gpu);
    };
    
    // 全局 gpuCache 访问（用于兼容，防止重复定义）
    if (!Object.getOwnPropertyDescriptor(window, 'gpuCache')) {
        Object.defineProperty(window, 'gpuCache', {
            get: function() {
                return gpuCache;
            },
            set: function(value) {
                gpuCache = value;
            },
            configurable: true
        });
    }
    
 const log = window.pixlyLog;
if (log) {
    log.info('PIXLY GPU', formatLog(LOG.GPU_DETECTION_MODULE_LOADED, {}));
}
    
})(window);
