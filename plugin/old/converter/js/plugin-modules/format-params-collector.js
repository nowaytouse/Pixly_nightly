/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * Format Parameters Collector
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 收集所有格式专属参数
 * - 确保UI参数完整传递到kernel
 * - 支持JXL/AVIF/HEIC的所有高级参数
 * 
 * 🚫 不做:
 * - 文件操作
 * - 参数验证（由kernel完成）
 * - 格式转换
 */
(function(window) {
    'use strict';
    
    /**
     * 收集JXL格式专属参数
     */
    function collectJXLParams() {
        return {
            effort: parseInt(document.getElementById('jxlEffort')?.value) || 7,
            distance: parseFloat(document.getElementById('jxlDistance')?.value) || 1.0,
            bit_depth: parseInt(document.getElementById('jxlBitDepth')?.value) || 8,
            color_space: document.getElementById('jxlColorSpace')?.value || 'sRGB',
            patches: parseInt(document.getElementById('jxlPatches')?.value) || 1,
            // 🔥 Advanced encoding options - ALL REAL!
            modular: document.getElementById('jxlModular')?.checked || false,
            progressive: document.getElementById('jxlProgressive')?.checked || false,
            responsive: document.getElementById('jxlResponsive')?.checked || false,
            gaborish: document.getElementById('jxlGaborish')?.checked || false,
            photon_noise: parseInt(document.getElementById('jxlPhotonNoise')?.value) || 0,
            decoding_speed: parseInt(document.getElementById('jxlDecodingSpeed')?.value) || 0,
        };
    }
    
    /**
     * 收集AVIF格式专属参数
     */
    function collectAVIFParams() {
        return {
            speed: parseInt(document.getElementById('avifSpeed')?.value) || 6,
            min_quantizer: parseInt(document.getElementById('avifMinQuantizer')?.value) || 0,
            max_quantizer: parseInt(document.getElementById('avifMaxQuantizer')?.value) || 63,
            chroma_subsampling: document.getElementById('avifChromaSubsampling')?.value || '420',
            bit_depth: parseInt(document.getElementById('avifBitDepth')?.value) || 8,
            tiles_rows: parseInt(document.getElementById('avifTilesRows')?.value) || 1,
            tiles_cols: parseInt(document.getElementById('avifTilesCols')?.value) || 1,
            premultiply_alpha: document.getElementById('avifPremultiply')?.checked || false,
        };
    }
    
    /**
     * 收集HEIC格式专属参数
     */
    function collectHEICParams() {
        return {
            quality: parseInt(document.getElementById('heicQuality')?.value) || 85,
            encoder: document.getElementById('heicEncoder')?.value || 'x265',
            chroma_subsampling: document.getElementById('heicChromaSubsampling')?.value || '444',
            lossless: document.getElementById('heicLossless')?.checked || false,
            embed_thumbnail: document.getElementById('heicThumbEmbed')?.checked || true,
        };
    }
    
    /**
     * 根据格式收集对应的专属参数
     */
    function collectFormatSpecificParams(format) {
        const formatParams = {};
        
        switch(format) {
            case 'jxl':
                formatParams.jxl = collectJXLParams();
                break;
            case 'avif':
                formatParams.avif = collectAVIFParams();
                break;
            case 'heic':
                formatParams.heic = collectHEICParams();
                break;
        }
        
        return formatParams;
    }
    
    /**
     * 收集所有转换参数（基础+格式专属）
     */
    function collectAllConversionParams(config) {
        return {
            // 基础参数
            format: config.format,
            quality: config.quality,
            speed: config.speed,
            lossless: config.lossless || false,
            mode: config.mode || 'manual',
            
            // 🔥 格式专属参数
            format_params: collectFormatSpecificParams(config.format),
            
            // 元数据选项
            preserveMetadata: document.getElementById('preserveMetadata')?.checked !== false,
            keepAnimated: document.getElementById('keepAnimated')?.checked !== false,
            mergeXmpSidecar: document.getElementById('mergeXmpSidecar')?.checked || false,
            
            // 预处理选项
            resize: document.getElementById('resize')?.value || null,
            quantize: document.getElementById('quantize')?.value || null,
            sharpen: document.getElementById('sharpen')?.value || null,
            resizeFilter: document.getElementById('resizeFilter')?.value || 'lanczos3',
            
            // 高级选项
            chromaSubsampling: document.getElementById('chromaSubsampling')?.value || 'auto',
            alphaQuality: document.getElementById('alphaQuality')?.value || null,
            effort: document.getElementById('effort')?.value || null,
            
            // 输出选项
            outputDir: config.outputDir || null,
            normalizeFilenames: document.getElementById('normalizeFilenames')?.checked || false,
            
            // 功能开关
            enableValidation: document.getElementById('enableValidation')?.checked !== false,
            enableAI: document.getElementById('enableAI')?.checked !== false,
            enableFileValidation: document.getElementById('enableFileValidation')?.checked || false
        };
    }
    
    // 导出到全局
    window.FormatParamsCollector = {
        collectJXLParams,
        collectAVIFParams,
        collectHEICParams,
        collectFormatSpecificParams,
        collectAllConversionParams
    };
    
})(window);
