/**
 * 🔥 Phase 46.6: 参数完整性验证器
 * 
 * 用途：参数完整性验证
 * - UI发送的参数快照
 * - Rust返回的actual_params对比
 * - 验证参数传递过程的完整性
 * - 增强系统可靠性和可信度
 * 
 * 核心原则：
 * - 质量 > 速度
 * - 响亮报错 > 静默降级
 * - 真实验证 > 假设正确
 */

(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    
    class ParamIntegrityValidator {
        constructor() {
            this.paramSnapshots = new Map();  // 存储参数快照
            this.validationResults = [];      // 验证结果历史
            this.enabled = true;               // 是否启用验证
            this.strictMode = false;           // 严格模式（参数不匹配时拒绝执行）
        }
        
        /**
         * 捕获参数快照
         * 
         * @param {string} id - 转换任务ID
         * @param {Object} uiParams - UI层发送的参数
         * @returns {string} 快照ID
         */
        captureSnapshot(id, uiParams) {
            const snapshot = {
                id: id,
                timestamp: Date.now(),
                ui_params: JSON.parse(JSON.stringify(uiParams)),  // 深拷贝
                sent_at: new Date().toISOString()
            };
            
            this.paramSnapshots.set(id, snapshot);
            
            if (log) {
                log.info('ParamIntegrity', '📸 参数快照已捕获', {
                    id: id,
                    params: JSON.stringify(uiParams)
                });
            }
            
            return id;
        }
        
        /**
         * 验证参数完整性
         * 
         * @param {string} id - 转换任务ID
         * @param {Object} actualParams - Rust返回的实际参数
         * @returns {Object} 验证结果
         */
        validateIntegrity(id, actualParams) {
            if (!this.enabled) {
                return { valid: true, skipped: true };
            }
            
            const snapshot = this.paramSnapshots.get(id);
            if (!snapshot) {
                if (log) {
                    log.warn('ParamIntegrity', '⚠️ 未找到参数快照', { id });
                }
                return {
                    valid: false,
                    error: '未找到参数快照，无法验证'
                };
            }
            
            const uiParams = snapshot.ui_params;
            const diffs = [];
            const warnings = [];
            const errors = [];
            
            // 1. 验证质量参数
            if (uiParams.quality !== undefined && actualParams.quality !== undefined) {
                if (uiParams.quality !== actualParams.quality) {
                    const diff = {
                        param: 'quality',
                        ui_value: uiParams.quality,
                        actual_value: actualParams.quality,
                        change: actualParams.quality - uiParams.quality
                    };
                    diffs.push(diff);
                    
                    // 判断变化是否合理
                    if (Math.abs(diff.change) > 20) {
                        warnings.push(`质量参数变化过大: ${diff.change}`);
                    }
                }
            }
            
            // 2. 验证速度参数
            if (uiParams.speed !== undefined && actualParams.speed !== undefined) {
                if (uiParams.speed !== actualParams.speed) {
                    const diff = {
                        param: 'speed',
                        ui_value: uiParams.speed,
                        actual_value: actualParams.speed,
                        change: actualParams.speed - uiParams.speed
                    };
                    diffs.push(diff);
                    
                    // 判断变化是否合理
                    if (Math.abs(diff.change) > 3) {
                        warnings.push(`速度参数变化过大: ${diff.change}`);
                    }
                }
            }
            
            // 3. 验证无损模式
            if (uiParams.lossless !== undefined && actualParams.lossless !== undefined) {
                if (uiParams.lossless !== actualParams.lossless) {
                    const diff = {
                        param: 'lossless',
                        ui_value: uiParams.lossless,
                        actual_value: actualParams.lossless,
                        change: 'boolean_change'
                    };
                    diffs.push(diff);
                    
                    // 无损模式改变需要警告
                    warnings.push(`无损模式被改变: ${uiParams.lossless} → ${actualParams.lossless}`);
                }
            }
            
            // 4. 验证格式
            if (uiParams.format !== actualParams.format) {
                errors.push(`格式不匹配: UI=${uiParams.format}, Rust=${actualParams.format}`);
            }
            
            // 5. 参数来源验证
            const expectedSource = this.determineExpectedSource(uiParams);
            if (actualParams.params_source !== expectedSource) {
                warnings.push(`参数来源不符预期: 期望=${expectedSource}, 实际=${actualParams.params_source}`);
            }
            
            // 构建验证结果
            const result = {
                valid: errors.length === 0,
                snapshot_id: id,
                timestamp: Date.now(),
                ui_params: uiParams,
                actual_params: actualParams,
                diffs: diffs,
                warnings: warnings,
                errors: errors,
                params_source: actualParams.params_source,
                strategy_used: actualParams.strategy_type
            };
            
            // 保存验证结果
            this.validationResults.push(result);
            
            // 记录日志
            if (log) {
                if (result.valid) {
                    if (diffs.length > 0) {
                        log.info('ParamIntegrity', '✅ 参数验证通过（有调整）', {
                            diffs: JSON.stringify(diffs),
                            source: actualParams.params_source
                        });
                    } else {
                        log.info('ParamIntegrity', '✅ 参数验证完全匹配');
                    }
                } else {
                    log.error('ParamIntegrity', '❌ 参数验证失败', {
                        errors: errors.join(', ')
                    });
                }
                
                if (warnings.length > 0) {
                    log.warn('ParamIntegrity', '⚠️ 参数验证警告', {
                        warnings: warnings.join(', ')
                    });
                }
            }
            
            // 严格模式下，有错误则抛出异常
            if (this.strictMode && !result.valid) {
                throw new Error(`参数完整性验证失败: ${errors.join('; ')}`);
            }
            
            // 清理快照（避免内存泄漏）
            this.paramSnapshots.delete(id);
            
            return result;
        }
        
        /**
         * 判断期望的参数来源
         */
        determineExpectedSource(uiParams) {
            // 智能模式：参数由AI决定
            const modeRadio = document.querySelector('input[name="conversionMode"]:checked');
            const mode = modeRadio ? modeRadio.value : 'manual';
            
            if (mode === 'smart') {
                // 智能模式下，如果UI没有提供具体参数，应该是AI
                if (uiParams.quality === undefined && uiParams.speed === undefined) {
                    return 'ai';
                }
                // 如果UI提供了部分参数，可能是hybrid
                return 'hybrid';
            } else {
                // 手动模式，应该是user
                return 'user';
            }
        }
        
        /**
         * 显示验证报告
         * 
         * @param {Object} result - 验证结果
         */
        showValidationReport(result) {
            if (!result.diffs || result.diffs.length === 0) {
                return;  // 没有差异，不显示报告
            }
            
            let message = '📊 参数验证报告:\n\n';
            
            // 显示参数差异
            if (result.diffs.length > 0) {
                message += '参数调整:\n';
                result.diffs.forEach(diff => {
                    const arrow = diff.change > 0 ? '↑' : diff.change < 0 ? '↓' : '→';
                    message += `• ${diff.param}: ${diff.ui_value} → ${diff.actual_value} ${arrow}\n`;
                });
                message += '\n';
            }
            
            // 显示警告
            if (result.warnings.length > 0) {
                message += '⚠️ 警告:\n';
                result.warnings.forEach(warning => {
                    message += `• ${warning}\n`;
                });
                message += '\n';
            }
            
            // 显示参数来源
            message += `参数来源: ${result.params_source}\n`;
            message += `使用策略: ${result.strategy_used || 'unknown'}\n`;
            
            // 使用Eagle通知显示
            if (window.eagle && window.eagle.notification) {
                eagle.notification.show({
                    title: '参数验证报告',
                    description: message,
                    duration: 5000
                });
            } else if (log) {
                log.info('ParamIntegrity', message);
            }
        }
        
        /**
         * 获取最近的验证结果
         * 
         * @param {number} count - 返回的结果数量
         * @returns {Array} 验证结果列表
         */
        getRecentResults(count = 10) {
            return this.validationResults.slice(-count);
        }
        
        /**
         * 清理旧的快照和结果
         * 
         * @param {number} maxAge - 最大保留时间（毫秒）
         */
        cleanup(maxAge = 3600000) {  // 默认1小时
            const now = Date.now();
            
            // 清理旧快照
            for (const [id, snapshot] of this.paramSnapshots.entries()) {
                if (now - snapshot.timestamp > maxAge) {
                    this.paramSnapshots.delete(id);
                }
            }
            
            // 清理旧结果
            this.validationResults = this.validationResults.filter(
                result => now - result.timestamp <= maxAge
            );
            
            if (log) {
                log.debug('ParamIntegrity', '清理完成', {
                    snapshots: this.paramSnapshots.size,
                    results: this.validationResults.length
                });
            }
        }
        
        /**
         * 启用/禁用验证器
         * 
         * @param {boolean} enabled - 是否启用
         */
        setEnabled(enabled) {
            this.enabled = enabled;
            if (log) {
                log.info('ParamIntegrity', `验证器${enabled ? '已启用' : '已禁用'}`);
            }
        }
        
        /**
         * 设置严格模式
         * 
         * @param {boolean} strict - 是否严格模式
         */
        setStrictMode(strict) {
            this.strictMode = strict;
            if (log) {
                log.info('ParamIntegrity', `严格模式: ${strict ? '开启' : '关闭'}`);
            }
        }
        
        /**
         * 生成完整验证报告
         * 
         * @returns {Object} 统计报告
         */
        generateStatistics() {
            const stats = {
                total: this.validationResults.length,
                passed: 0,
                failed: 0,
                with_warnings: 0,
                with_diffs: 0,
                param_sources: {
                    user: 0,
                    ai: 0,
                    hybrid: 0
                },
                avg_quality_change: 0,
                avg_speed_change: 0
            };
            
            let totalQualityChange = 0;
            let totalSpeedChange = 0;
            let qualityCount = 0;
            let speedCount = 0;
            
            this.validationResults.forEach(result => {
                if (result.valid) {
                    stats.passed++;
                } else {
                    stats.failed++;
                }
                
                if (result.warnings.length > 0) {
                    stats.with_warnings++;
                }
                
                if (result.diffs.length > 0) {
                    stats.with_diffs++;
                    
                    result.diffs.forEach(diff => {
                        if (diff.param === 'quality' && typeof diff.change === 'number') {
                            totalQualityChange += diff.change;
                            qualityCount++;
                        }
                        if (diff.param === 'speed' && typeof diff.change === 'number') {
                            totalSpeedChange += diff.change;
                            speedCount++;
                        }
                    });
                }
                
                // 统计参数来源
                const source = result.params_source || 'unknown';
                if (stats.param_sources[source] !== undefined) {
                    stats.param_sources[source]++;
                }
            });
            
            // 计算平均值
            if (qualityCount > 0) {
                stats.avg_quality_change = (totalQualityChange / qualityCount).toFixed(1);
            }
            if (speedCount > 0) {
                stats.avg_speed_change = (totalSpeedChange / speedCount).toFixed(1);
            }
            
            return stats;
        }
    }
    
    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ParamIntegrityValidator = ParamIntegrityValidator;
    
    // 创建全局实例
    window.paramIntegrityValidator = new ParamIntegrityValidator();
    
    // 定期清理（每30分钟）
    setInterval(() => {
        window.paramIntegrityValidator.cleanup();
    }, 1800000);
    
    if (log) {
        log.info('ParamIntegrity', '🔥 参数完整性验证器已加载');
    }
    
})(window);
