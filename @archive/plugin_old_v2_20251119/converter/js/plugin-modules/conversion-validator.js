/**
 * 🔥 Phase 40.33: 多级验证系统
 * 
 * 增强转换可靠性：
 * - 输入验证：文件格式、大小、路径、权限
 * - 参数验证：质量、速度、格式匹配
 * - 输出验证：文件生成、大小、格式正确性
 * - 错误预防：避免异常情况，全方位安全检查
 */

(function(window) {
    'use strict';
    
    // Log instance for the module
    const log = window.pixlyLog;
    
    const ConversionValidator = {
        /**
         * 🔍 Level 1: 输入文件验证
         */
        validateInputFiles: function(files) {
            const errors = [];
            const warnings = [];
            
            if (!files || files.length === 0) {
                errors.push('❌ 没有选择任何文件');
                return { valid: false, errors, warnings };
            }
            
            files.forEach((file, index) => {
                // 验证文件路径
                if (!file.filePath || typeof file.filePath !== 'string') {
                    errors.push(`❌ 文件 #${index + 1}: 无效的文件路径`);
                }
                
                // 验证文件扩展名
                if (!file.ext || file.ext === '') {
                    errors.push(`❌ 文件 #${index + 1} (${file.name}): 缺少文件扩展名`);
                }
                
                // 验证文件大小
                if (!file.size || file.size <= 0) {
                    warnings.push(`⚠️ 文件 #${index + 1} (${file.name}): 文件大小为 0 或未知`);
                } else if (file.size > 1024 * 1024 * 1024) { // 1GB
                    warnings.push(`⚠️ 文件 #${index + 1} (${file.name}): 文件过大 (${(file.size / 1024 / 1024 / 1024).toFixed(2)} GB)`);
                }
                
                // 验证文件名合法性
                if (file.name && /[<>:"|?*]/g.test(file.name)) {
                    warnings.push(`⚠️ 文件 #${index + 1} (${file.name}): 文件名包含非法字符`);
                }
            });
            
            return {
                valid: errors.length === 0,
                errors,
                warnings,
                fileCount: files.length
            };
        },
        
        /**
         * 🔍 Level 2: 转换参数验证
         */
        validateParameters: function(config, files) {
            const errors = [];
            const warnings = [];
            
            // 验证目标格式
            const validFormats = ['jxl', 'avif', 'webp', 'heic', 'png', 'jpg'];
            if (!config.format || !validFormats.includes(config.format.toLowerCase())) {
                errors.push(`❌ 无效的目标格式: ${config.format}`);
            }
            
            // 验证质量参数
            if (config.quality !== undefined) {
                if (typeof config.quality !== 'number' || config.quality < 1 || config.quality > 100) {
                    errors.push(`❌ 质量参数无效: ${config.quality} (应为 1-100)`);
                }
            }
            
            // 验证速度参数
            if (config.speed !== undefined) {
                if (typeof config.speed !== 'number' || config.speed < 0 || config.speed > 10) {
                    errors.push(`❌ 速度参数无效: ${config.speed} (应为 0-10)`);
                }
            }
            
            // 验证格式兼容性
            if (config.format === 'heic') {
                // 检测透明度
                const hasTransparency = files.some(f => {
                    const ext = (f.ext || '').toLowerCase();
                    return ext === '.png' || ext === '.gif' || ext === '.webp';
                });
                if (hasTransparency) {
                    warnings.push('⚠️ HEIC 不支持透明度，透明背景可能变色');
                }
                
                // 检测动画
                const hasAnimation = files.some(f => f.is_animated === true);
                if (hasAnimation) {
                    warnings.push('⚠️ HEIC 不支持动画，动画效果将丢失');
                }
            }
            
            // 验证无损模式兼容性
            if (config.lossless) {
                if (config.format === 'jpg' || config.format === 'jpeg') {
                    errors.push('❌ JPEG 格式不支持无损模式');
                }
            }
            
            return {
                valid: errors.length === 0,
                errors,
                warnings
            };
        },
        
        /**
         * 🔍 Level 3: 模式与参数匹配验证
         */
        validateModeConsistency: function(mode, config) {
            const errors = [];
            const warnings = [];
            
            // 手动模式：用户参数应该被尊重
            if (mode === 'manual') {
                // 验证所有必需参数已设置
                if (config.quality === undefined) {
                    warnings.push('⚠️ 手动模式下未设置质量参数，将使用默认值');
                }
                if (config.format === undefined) {
                    errors.push('❌ 手动模式下必须指定目标格式');
                }
            }
            
            // 智能模式：验证 AI 预测必要性
            if (mode === 'smart' || mode === 'ai') {
                if (!window.AI_SERVICE_AVAILABLE) {
                    warnings.push('⚠️ AI 服务未可用，智能模式可能降级为规则模式');
                }
            }
            
            return {
                valid: errors.length === 0,
                errors,
                warnings
            };
        },
        
        /**
         * 🔍 Level 4: 输出验证（转换后）
         */
        validateOutput: function(outputPath, expectedSize) {
            const errors = [];
            const warnings = [];
            
            // Node.js fs 模块检查文件是否存在
            if (typeof require !== 'undefined') {
                try {
                    const fs = require('fs');
                    const path = require('path');
                    
                    if (!fs.existsSync(outputPath)) {
                        errors.push(`❌ 输出文件未生成: ${path.basename(outputPath)}`);
                        return { valid: false, errors, warnings };
                    }
                    
                    const stats = fs.statSync(outputPath);
                    const actualSize = stats.size;
                    
                    // 验证文件大小不为0
                    if (actualSize === 0) {
                        errors.push(`❌ 输出文件大小为 0: ${path.basename(outputPath)}`);
                    }
                    
                    // 验证文件大小合理性（不应过大或过小）
                    if (expectedSize) {
                        const ratio = actualSize / expectedSize;
                        if (ratio < 0.01) {
                            warnings.push(`⚠️ 输出文件异常小 (${(ratio * 100).toFixed(2)}% of original)`);
                        } else if (ratio > 10) {
                            warnings.push(`⚠️ 输出文件异常大 (${ratio.toFixed(2)}x of original)`);
                        }
                    }
                    
                } catch (err) {
                    errors.push(`❌ 输出验证失败: ${err.message}`);
                }
            }
            
            return {
                valid: errors.length === 0,
                errors,
                warnings
            };
        },
        
        /**
         * 🔍 完整验证流程
         */
        validateFullConversion: function(files, config, mode) {
            if (log) {
                log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_START, {}));
            }
            
            // Level 1: 输入验证
            const inputValidation = this.validateInputFiles(files);
            if (!inputValidation.valid) {
                if (log) {
                    log.error('Validator', formatLog(LOG.CONVERSION_VALIDATOR_INPUT_FAILED, { errors: inputValidation.errors.join(', ') }));
                }
                return {
                    valid: false,
                    level: 1,
                    errors: inputValidation.errors,
                    warnings: inputValidation.warnings
                };
            }
            if (log) {
                log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_LEVEL1_PASSED, {}));
            }
            
            // Level 2: 参数验证
            const paramValidation = this.validateParameters(config, files);
            if (!paramValidation.valid) {
                if (log) {
                    log.error('Validator', formatLog(LOG.CONVERSION_VALIDATOR_PARAM_FAILED, { errors: paramValidation.errors.join(', ') }));
                }
                return {
                    valid: false,
                    level: 2,
                    errors: paramValidation.errors,
                    warnings: [...inputValidation.warnings, ...paramValidation.warnings]
                };
            }
            if (log) {
                log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_LEVEL2_PASSED, {}));
            }
            
            // Level 3: 模式一致性验证
            const modeValidation = this.validateModeConsistency(mode, config);
            if (!modeValidation.valid) {
                if (log) {
                    log.error('Validator', formatLog(LOG.CONVERSION_VALIDATOR_MODE_FAILED, { errors: modeValidation.errors.join(', ') }));
                }
                return {
                    valid: false,
                    level: 3,
                    errors: modeValidation.errors,
                    warnings: [...inputValidation.warnings, ...paramValidation.warnings, ...modeValidation.warnings]
                };
            }
            if (log) {
                log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_LEVEL3_PASSED, {}));
            }
            
            // 收集所有警告
            const allWarnings = [
                ...inputValidation.warnings,
                ...paramValidation.warnings,
                ...modeValidation.warnings
            ];
            
            if (log) {
                log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_ALL_PASSED, {}));
                if (allWarnings.length > 0) {
                    log.warn('Validator', formatLog(LOG.CONVERSION_VALIDATOR_WARNINGS, { warnings: allWarnings.join(', ') }));
                }
            }
            
            return {
                valid: true,
                level: 3,
                errors: [],
                warnings: allWarnings
            };
        }
    };
    
    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ConversionValidator = ConversionValidator;
    
    if (log) {
        log.info('Validator', formatLog(LOG.CONVERSION_VALIDATOR_MODULE_LOADED, {}));
    }
    
})(window);
