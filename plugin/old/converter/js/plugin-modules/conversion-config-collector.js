/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Conversion Config Collector
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 收集转换配置（模式、格式、质量等）
 * - 区分智能模式和手动模式
 * - 读取优化模式设置
 * 
 * 🚫 不做:
 * - AI预测（由kernel完成）
 * - 参数验证（由kernel完成）
 * - 文件操作
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    const LOG = window.LOG_CONSTANTS || {};
    
    /**
     * 读取优化模式
     */
    function getOptimizeMode() {
        const optimizeModeRadio = document.querySelector('input[name="optimizeMode"]:checked');
        return optimizeModeRadio ? optimizeModeRadio.value : 'balanced';
    }
    
    /**
     * 检查是否为智能模式
     */
    function isSmartMode() {
        const smartModeRadio = document.getElementById('modeRadioSmart');
        return smartModeRadio && smartModeRadio.checked;
    }
    
    /**
     * 获取用户选择的格式（智能模式）
     */
    function getUserSelectedFormat() {
        // 优先级：1. AI预期格式下拉框 > 2. 格式Radio按钮 > 3. 默认JXL
        const expectedFormatSelect = document.getElementById('expectedFormatSelect');
        if (expectedFormatSelect && expectedFormatSelect.value && 
            expectedFormatSelect.value !== '' && expectedFormatSelect.value !== 'disabled') {
            return expectedFormatSelect.value;
        }
        
        const formatRadio = document.querySelector('input[name="format"]:checked');
        return formatRadio ? formatRadio.value : 'jxl';
    }
    
    /**
     * 收集智能模式配置
     */
    function collectSmartModeConfig() {
        const optimizeMode = getOptimizeMode();
        const userFormat = getUserSelectedFormat();
        
        if (log) {
            log.info('Conversion', LOG.IMAGE_CONV_SMART_MODE_DELEGATING || 'Smart mode delegating to AI');
            log.info('Conversion', LOG.IMAGE_CONV_USER_FORMAT_AI || 'User format from AI', { format: userFormat });
        }
        
        // 🔥 质量宣言执行：智能模式不传递默认值
        // - 如果AI预测成功 → 使用AI返回的参数 ✅
        // - 如果AI预测失败 → Rust响亮报错，不使用hardcode ✅
        // - 【原则】响亮报错 > 静默降级
        return {
            format: userFormat,      // ✅ 尊重用户选择
            quality: undefined,      // ✅ 不传递默认值，强制使用AI预测
            speed: undefined,        // ✅ 不传递默认值，强制使用AI预测
            lossless: undefined,     // ✅ Go AI ML决定
            optimizeMode: optimizeMode,
            mode: 'smart'
        };
    }
    
    /**
     * 收集手动模式配置
     */
    function collectManualModeConfig() {
        const formatRadio = document.querySelector('input[name="format"]:checked');
        const format = formatRadio ? formatRadio.value : 'jxl';
        
        const qualitySlider = document.getElementById('quality');
        const speedSlider = document.getElementById('speed');
        const losslessCheckbox = document.getElementById('lossless');
        const manualLosslessCheckbox = document.getElementById('manualLossless');
        const enableJpegLosslessCheckbox = document.getElementById('enableJpegLossless');
        
        const quality = qualitySlider ? parseInt(qualitySlider.value) : 90;
        const speed = speedSlider ? parseInt(speedSlider.value) : 7;
        const lossless = losslessCheckbox ? losslessCheckbox.checked : 
                         (manualLosslessCheckbox ? manualLosslessCheckbox.checked : false);
        
        // 🔥 Phase 45.5: 读取JPEG无损转码选项（JXL格式专用）
        const jpegLossless = enableJpegLosslessCheckbox ? enableJpegLosslessCheckbox.checked : false;
        
        const optimizeMode = getOptimizeMode();
        
        const config = { format, quality, speed, lossless, jpegLossless, optimizeMode };
        
        if (log) {
            log.info('Conversion', LOG.IMAGE_CONV_MANUAL_MODE_CONFIG || 'Manual mode config', 
                     { config: JSON.stringify(config) });
        }
        
        return {
            format: format,
            quality: quality,
            speed: speed,
            lossless: lossless,
            jpeg_lossless: jpegLossless,
            optimizeMode: optimizeMode,
            mode: 'manual'
        };
    }
    
    /**
     * 获取转换配置（主入口）
     */
    function getConversionConfig() {
        if (isSmartMode()) {
            return collectSmartModeConfig();
        } else {
            return collectManualModeConfig();
        }
    }
    
    // 导出到全局
    window.ConversionConfigCollector = {
        getOptimizeMode,
        isSmartMode,
        getUserSelectedFormat,
        collectSmartModeConfig,
        collectManualModeConfig,
        getConversionConfig
    };
    
})(window);
