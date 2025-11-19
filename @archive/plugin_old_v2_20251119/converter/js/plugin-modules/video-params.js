/**
 * 模块 07: Video Encoding Parameters处理
 * 处理视频编码的底层参数、动态显示逻辑
 */

// 初始化视频参数控制
function initVideoParamsHandlers() {
    const rateControlSelect = document.getElementById('videoRateControl');
    const bitrateControl = document.getElementById('bitrateControl');
    const deblockCheckbox = document.getElementById('videoDeblock');
    const deblockParams = document.getElementById('deblockParams');
    
    if (!rateControlSelect) return;
    
    // 码率控制模式切换
    rateControlSelect.addEventListener('change', () => {
        updateBitrateVisibility();
    });
    
    // 去块滤波开关
    if (deblockCheckbox && deblockParams) {
        deblockCheckbox.addEventListener('change', () => {
            deblockParams.style.display = deblockCheckbox.checked ? 'grid' : 'none';
        });
    }
    
    // 初始化状态
    updateBitrateVisibility();
}

// 更新目标码率显示
function updateBitrateVisibility() {
    const rateControlSelect = document.getElementById('videoRateControl');
    const bitrateControl = document.getElementById('bitrateControl');
    
    if (!rateControlSelect || !bitrateControl) return;
    
    const mode = rateControlSelect.value;
    const showBitrate = ['cbr', 'vbr', 'abr'].includes(mode);
    bitrateControl.style.display = showBitrate ? 'block' : 'none';
}

// 获取Video Encoding Parameters
function getVideoEncodingParams() {
    const params = {};
    
    // 码率控制模式
    const rateControl = document.getElementById('videoRateControl');
    if (rateControl) {
        params.rateControl = rateControl.value;
    }
    
    // 目标码率（CBR/VBR/ABR时有效）
    const bitrate = document.getElementById('videoBitrate');
    if (bitrate && params.rateControl !== 'crf') {
        params.bitrate = parseInt(bitrate.value) || 10;
    }
    
    // GOP大小
    const gop = document.getElementById('videoGop');
    if (gop) {
        params.gop = parseInt(gop.value) || 250;
    }
    
    // B帧数量
    const bframes = document.getElementById('videoBframes');
    if (bframes) {
        params.bframes = parseInt(bframes.value) || 3;
    }
    
    // 参考帧数量
    const refs = document.getElementById('videoRefs');
    if (refs) {
        params.refs = parseInt(refs.value) || 4;
    }
    
    // 运动搜索范围
    const meRange = document.getElementById('videoMeRange');
    if (meRange) {
        params.meRange = parseInt(meRange.value) || 16;
    }
    
    // 子像素运动估计
    const subme = document.getElementById('videoSubme');
    if (subme) {
        params.subme = parseInt(subme.value) || 5;
    }
    
    // 运动估计算法
    const meMethod = document.getElementById('videoMeMethod');
    if (meMethod) {
        params.meMethod = meMethod.value || 'hex';
    }
    
    // 去块滤波
    const deblock = document.getElementById('videoDeblock');
    if (deblock) {
        params.deblock = deblock.checked;
        
        if (params.deblock) {
            const alpha = document.getElementById('videoDeblockAlpha');
            const beta = document.getElementById('videoDeblockBeta');
            params.deblockAlpha = parseInt(alpha?.value) || 0;
            params.deblockBeta = parseInt(beta?.value) || 0;
        }
    }
    
    return params;
}

// 验证Video Encoding Parameters
function validateVideoParams(params) {
    const errors = [];
    
    // 验证GOP范围
    if (params.gop && (params.gop < 1 || params.gop > 600)) {
        errors.push('GOP大小必须在1-600之间');
    }
    
    // 验证B帧范围
    if (params.bframes && (params.bframes < 0 || params.bframes > 16)) {
        errors.push('B帧数量必须在0-16之间');
    }
    
    // 验证参考帧范围
    if (params.refs && (params.refs < 1 || params.refs > 16)) {
        errors.push('参考帧数量必须在1-16之间');
    }
    
    // 验证运动搜索范围
    if (params.meRange && (params.meRange < 4 || params.meRange > 512)) {
        errors.push('运动搜索范围必须在4-512之间');
    }
    
    // 验证子像素运动估计
    if (params.subme && (params.subme < 1 || params.subme > 10)) {
        errors.push('子像素运动估计必须在1-10之间');
    }
    
    // 验证去块滤波参数
    if (params.deblock && params.deblockAlpha !== undefined) {
        if (params.deblockAlpha < -6 || params.deblockAlpha > 6) {
            errors.push('Deblock Alpha必须在-6到6之间');
        }
    }
    if (params.deblock && params.deblockBeta !== undefined) {
        if (params.deblockBeta < -6 || params.deblockBeta > 6) {
            errors.push('Deblock Beta必须在-6到6之间');
        }
    }
    
    // 验证码率（如果适用）
    if (params.rateControl !== 'crf' && params.bitrate) {
        if (params.bitrate < 1 || params.bitrate > 100) {
            errors.push('目标码率必须在1-100 Mbps之间');
        }
    }
    
    return {
        valid: errors.length === 0,
        errors: errors
    };
}

// 导出底层x264/x265参数字符串（用于后端）
function exportVideoParamsString(params) {
    const parts = [];
    
    // GOP大小
    if (params.gop) {
        parts.push(`keyint=${params.gop}`);
    }
    
    // B帧数量
    if (params.bframes !== undefined) {
        parts.push(`bframes=${params.bframes}`);
    }
    
    // 参考帧数量
    if (params.refs) {
        parts.push(`ref=${params.refs}`);
    }
    
    // 运动搜索范围
    if (params.meRange) {
        parts.push(`merange=${params.meRange}`);
    }
    
    // 子像素运动估计
    if (params.subme) {
        parts.push(`subme=${params.subme}`);
    }
    
    // 运动估计算法
    if (params.meMethod) {
        parts.push(`me=${params.meMethod}`);
    }
    
    // 去块滤波
    if (params.deblock === false) {
        parts.push('no-deblock');
    } else if (params.deblock && (params.deblockAlpha !== 0 || params.deblockBeta !== 0)) {
        parts.push(`deblock=${params.deblockAlpha}:${params.deblockBeta}`);
    }
    
    return parts.join(':');
}

// 日志输出视频参数（调试用）
function logVideoParams(params) {
    // 🔥 Hybrid strategy: console.group for dev tools + pixlyLog for production
    const log = window.pixlyLog;
    if (log) {
        log.info('Video Params', formatLog(LOG.VIDEO_PARAMS_DISPLAYED, {}));
    }
    
    // Preserve console.group for dev tools experience
    console.group('🎬 Video Encoding Parameters');
    log.info('Rate control:', params.rateControl || 'crf');
    if (params.bitrate) log.info('Target bitrate:', params.bitrate, 'Mbps');
    log.info('GOP size:', params.gop || 250);
    log.info('B-frames:', params.bframes ?? 3);
    log.info('Reference frames:', params.refs || 4);
    log.info('Motion search range:', params.meRange || 16);
    log.info('Subpixel ME:', params.subme || 5);
    log.info('ME algorithm:', params.meMethod || 'hex');
    log.info('Deblocking:', params.deblock ? `Enabled (${params.deblockAlpha}:${params.deblockBeta})` : 'Disabled');
    log.info('Params string:', exportVideoParamsString(params));
    console.groupEnd();
}

// 获取动图转视频配置
function getAnimatedToVideoConfig() {
    const Logger = window.PIXLY?.Logger || console;
    const smartVideoTab = document.getElementById('smart-video');
    const isSmartMode = smartVideoTab && smartVideoTab.classList.contains('active');
    
    const config = {
        encoder: 'h265',
        crf: 23,
        preset: 'medium',
        container: 'mp4',
        fps: null,  // 🔒 默认null，保持原帧率
        smartMode: isSmartMode
    };
    
    if (!isSmartMode) {
        // 手动模式：读取UI配置
        const encoderRadios = document.querySelectorAll('input[name="videoEncoder"]');
        for (const radio of encoderRadios) {
            if (radio.checked) {
                config.encoder = radio.value;
                break;
            }
        }
        
        const crfSlider = document.getElementById('videoCRF');
        if (crfSlider) {
            config.crf = parseInt(crfSlider.value) || 23;
        }
        
        const presetSelect = document.getElementById('videoPreset');
        if (presetSelect) {
            config.preset = presetSelect.value || 'medium';
        }
        
        const containerSelect = document.getElementById('videoContainer');
        if (containerSelect) {
            config.container = containerSelect.value || 'mp4';
        }
        
        // 🎬 FPS处理：只有用户明确输入了FPS才使用，否则保持原帧率
        const fpsInput = document.getElementById('videoFps');
        if (fpsInput && fpsInput.value && fpsInput.value.trim() !== '') {
            const fpsValue = parseInt(fpsInput.value);
            if (fpsValue > 0) {
                config.fps = fpsValue;
                Logger.info(`[Video Config] 📊 User specified FPS: ${fpsValue}`);
            } else {
                config.fps = null;  // 保持原帧率
                Logger.info(`[Video Config] 📊 FPS: Keep original`);
            }
        } else {
            config.fps = null;  // 未设置FPS，保持原帧率
            Logger.info(`[Video Config] 📊 FPS: Keep original (not set)`);
        }
    }
    
    Logger.info(`[Video Config] Animated→Video | Mode=${isSmartMode ? 'Smart' : 'Manual'} | Encoder=${config.encoder} | CRF=${config.crf}`);
    return config;
}

// 获取视频转视频配置
function getVideoToVideoConfig() {
    const Logger = window.PIXLY?.Logger || console;
    const smartVideoTab = document.getElementById('smart-video');
    const isSmartMode = smartVideoTab && smartVideoTab.classList.contains('active');
    
    const config = {
        encoder: 'h265',
        crf: 23,
        preset: 'medium',
        container: 'mp4',
        fps: null,  // 🔒 默认null，保持原帧率
        smartMode: isSmartMode,
        enableAdvancedFeatures: false,
        enableTransformer: false,
        enableVMAF: false
    };
    
    if (!isSmartMode) {
        // 手动模式：读取UI配置
        const encoderRadios = document.querySelectorAll('input[name="videoEncoder"]');
        for (const radio of encoderRadios) {
            if (radio.checked) {
                config.encoder = radio.value;
                break;
            }
        }
        
        const crfSlider = document.getElementById('videoCRF');
        if (crfSlider) {
            config.crf = parseInt(crfSlider.value) || 23;
        }
        
        const presetSelect = document.getElementById('videoPreset');
        if (presetSelect) {
            config.preset = presetSelect.value || 'medium';
        }
        
        const containerSelect = document.getElementById('videoContainer');
        if (containerSelect) {
            config.container = containerSelect.value || 'mp4';
        }
        
        // 🎬 FPS处理：只有用户明确输入了FPS才使用，否则保持原帧率
        const fpsInput = document.getElementById('videoFps');
        if (fpsInput && fpsInput.value && fpsInput.value.trim() !== '') {
            const fpsValue = parseInt(fpsInput.value);
            if (fpsValue > 0) {
                config.fps = fpsValue;
                Logger.info(`[Video Config] 📊 User specified FPS: ${fpsValue}`);
            }
        }
        // 如果未设置或无效，保持fps=null（保持原帧率）
    } else {
        // 智能模式：读取AI选项
        const advancedFeatures = document.getElementById('enableAdvancedFeatures');
        if (advancedFeatures) {
            config.enableAdvancedFeatures = advancedFeatures.checked;
        }
        
        const forceTransformer = document.getElementById('forceTransformer');
        if (forceTransformer) {
            config.enableTransformer = forceTransformer.checked;
        }
        
        const enableVMAF = document.getElementById('enableVMAF');
        if (enableVMAF) {
            config.enableVMAF = enableVMAF.checked;
        }
    }
    
    Logger.info(`[Video Config] Video→Video | Mode=${isSmartMode ? 'Smart' : 'Manual'} | Encoder=${config.encoder} | CRF=${config.crf}`);
    return config;
}

// 导出到window对象
if (typeof window !== 'undefined') {
    window.getAnimatedToVideoConfig = getAnimatedToVideoConfig;
    window.getVideoToVideoConfig = getVideoToVideoConfig;
    window.getVideoEncodingParams = getVideoEncodingParams;
    window.exportVideoParamsString = exportVideoParamsString;
}

// 页面加载时初始化
document.addEventListener('DOMContentLoaded', () => {
    initVideoParamsHandlers();
    const log = window.pixlyLog;
    if (log) {
        log.info('Video Params', formatLog(LOG.VIDEO_PARAMS_MODULE_LOADED, {}));
    }
});
