/**
 * 参数兼容性检查器
 * 确保AI预测和手动参数都能正确传递到Rust Kernel
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    
    class ParamCompatibility {
        /**
         * 验证参数完整性
         */
        static validateParams(params) {
            const errors = [];
            const warnings = [];
            
            // 必需参数
            if (!params.format) {
                errors.push('缺少目标格式');
            }
            
            // 质量参数范围
            if (params.quality !== undefined && params.quality !== null) {
                if (params.quality < 1 || params.quality > 100) {
                    errors.push(`质量参数超出范围: ${params.quality} (应为1-100)`);
                }
            }
            
            // 速度参数范围
            if (params.speed !== undefined && params.speed !== null) {
                if (params.speed < 0 || params.speed > 10) {
                    errors.push(`速度参数超出范围: ${params.speed} (应为0-10)`);
                }
            }
            
            // 格式特定检查
            if (params.format === 'jpeg' && params.lossless) {
                warnings.push('JPEG不支持无损模式，将忽略lossless参数');
            }
            
            return { valid: errors.length === 0, errors, warnings };
        }
        
        /**
         * 标准化参数（确保类型正确）
         */
        static normalizeParams(params) {
            const normalized = { ...params };
            
            // 确保数值类型
            if (normalized.quality) normalized.quality = Number(normalized.quality);
            if (normalized.speed) normalized.speed = Number(normalized.speed);
            if (normalized.quantize) normalized.quantize = Number(normalized.quantize);
            if (normalized.sharpen) normalized.sharpen = Number(normalized.sharpen);
            if (normalized.alphaQuality) normalized.alphaQuality = Number(normalized.alphaQuality);
            if (normalized.effort) normalized.effort = Number(normalized.effort);
            
            // 确保布尔类型
            normalized.lossless = Boolean(normalized.lossless);
            normalized.preserveMetadata = Boolean(normalized.preserveMetadata);
            normalized.keepAnimated = Boolean(normalized.keepAnimated);
            normalized.mergeXmpSidecar = Boolean(normalized.mergeXmpSidecar);
            normalized.normalizeFilenames = Boolean(normalized.normalizeFilenames);
            
            // 确保字符串类型
            if (normalized.format) normalized.format = String(normalized.format).toLowerCase();
            if (normalized.mode) normalized.mode = String(normalized.mode).toLowerCase();
            
            return normalized;
        }
        
        /**
         * 合并AI预测和手动参数
         */
        static mergeParams(manualParams, aiParams) {
            // AI参数优先级较低，手动参数优先
            const merged = {
                ...aiParams,
                ...manualParams
            };
            
            // 如果是手动模式，完全使用手动参数
            if (manualParams.mode === 'manual') {
                return this.normalizeParams(manualParams);
            }
            
            // 智能模式：AI建议 + 用户覆盖
            return this.normalizeParams(merged);
        }
        
        /**
         * 转换为Rust CLI参数格式
         */
        static toRustParams(params) {
            return {
                // 基础参数（snake_case）
                target_format: params.format,
                quality: params.quality,
                speed: params.speed,
                lossless: params.lossless,
                
                // 元数据
                preserve_metadata: params.preserveMetadata,
                keep_animated: params.keepAnimated,
                merge_xmp_sidecar: params.mergeXmpSidecar,
                
                // 预处理
                resize: params.resize,
                quantize: params.quantize,
                sharpen: params.sharpen,
                resize_filter: params.resizeFilter,
                
                // 高级选项
                chroma_subsampling: params.chromaSubsampling,
                alpha_quality: params.alphaQuality,
                effort: params.effort,
                
                // 输出
                output_dir: params.outputDir,
                normalize_filenames: params.normalizeFilenames,
                
                // 模式
                mode: params.mode
            };
        }
        
        /**
         * 显示参数摘要
         */
        static displaySummary(params) {
            if (!window.addLog) return;
            
            window.addLog('📋 转换参数:', 'info');
            window.addLog(`  格式: ${params.format}`, 'info');
            window.addLog(`  质量: ${params.quality}`, 'info');
            window.addLog(`  速度: ${params.speed}`, 'info');
            window.addLog(`  无损: ${params.lossless ? '是' : '否'}`, 'info');
            window.addLog(`  模式: ${params.mode}`, 'info');
            
            if (params.resize) {
                window.addLog(`  调整大小: ${params.resize}`, 'info');
            }
            if (params.quantize) {
                window.addLog(`  颜色量化: ${params.quantize}`, 'info');
            }
            if (params.sharpen) {
                window.addLog(`  锐化: ${params.sharpen}`, 'info');
            }
        }
    }
    
    // 导出
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ParamCompatibility = ParamCompatibility;
    
    if (log) {
        log.info('ParamCompatibility', '✅ 参数兼容性检查器已加载');
    }
    
})(window);
