/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * AI Bridge - AI系统桥接
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 职责:
 * - 调用Rust AI系统
 * - 获取参数预测
 * - 获取格式推荐
 * - 显示AI建议
 * 
 * 🚫 不做:
 * - 实现AI算法
 * - 参数计算
 * - 格式推荐逻辑
 */
(function(window) {
    'use strict';
    
    const log = window.pixlyLog;
    
    /**
     * AI桥接类
     */
    class AIBridge {
        constructor() {
            this.kernelBridge = window.PIXLY.kernelBridge;
        }
        
        /**
         * 🤖 获取AI参数预测
         * 
         * @param {Object} imageInfo - 图像信息
         * @param {string} targetFormat - 目标格式
         * @param {string} qualityMode - 质量模式 (speed/balanced/quality/lossless)
         * @returns {Promise<Object>} AI预测结果
         */
        async predictParameters(imageInfo, targetFormat, qualityMode = 'balanced') {
            log.info('AIBridge', '请求AI参数预测', { 
                format: targetFormat, 
                mode: qualityMode,
                imageSize: imageInfo.size 
            });
            
            try {
                const result = await this.kernelBridge.predictParameters(
                    imageInfo,
                    targetFormat,
                    qualityMode
                );
                
                log.info('AIBridge', 'AI预测完成', { 
                    quality: result.quality,
                    speed: result.speed,
                    lossless: result.lossless,
                    confidence: result.confidence
                });
                
                return result;
            } catch (error) {
                log.error('AIBridge', 'AI预测失败', { error: error.message });
                throw error;
            }
        }
        
        /**
         * 📊 获取格式推荐
         * 
         * @param {Object} imageInfo - 图像信息
         * @returns {Promise<Object>} 格式推荐结果
         */
        async recommendFormat(imageInfo) {
            log.info('AIBridge', '请求格式推荐', { 
                currentFormat: imageInfo.format,
                hasAlpha: imageInfo.hasAlpha,
                isAnimated: imageInfo.isAnimated
            });
            
            try {
                // 调用Rust的格式推荐
                const result = await this.kernelBridge.executeCommand('recommend_format', {
                    features: {
                        width: imageInfo.width,
                        height: imageInfo.height,
                        file_size: imageInfo.size,
                        format: imageInfo.format,
                        has_alpha: imageInfo.hasAlpha || false,
                        is_animated: imageInfo.isAnimated || false
                    }
                });
                
                log.info('AIBridge', '格式推荐完成', { 
                    recommended: result.format,
                    reason: result.reason
                });
                
                return result;
            } catch (error) {
                log.error('AIBridge', '格式推荐失败', { error: error.message });
                throw error;
            }
        }
        
        /**
         * 🎯 智能模式：自动获取最佳参数
         * 
         * @param {Object} imageInfo - 图像信息
         * @param {string} targetFormat - 目标格式（可选，不提供则自动推荐）
         * @returns {Promise<Object>} 完整的转换配置
         */
        async getSmartConfig(imageInfo, targetFormat = null) {
            log.info('AIBridge', '智能模式：获取最佳配置');
            
            try {
                // 如果没有指定格式，先推荐格式
                if (!targetFormat) {
                    const recommendation = await this.recommendFormat(imageInfo);
                    targetFormat = recommendation.format;
                    log.info('AIBridge', `AI推荐格式: ${targetFormat}`, { reason: recommendation.reason });
                }
                
                // 获取该格式的最佳参数
                const prediction = await this.predictParameters(imageInfo, targetFormat, 'balanced');
                
                return {
                    format: targetFormat,
                    quality: prediction.quality,
                    speed: prediction.speed,
                    lossless: prediction.lossless,
                    confidence: prediction.confidence,
                    estimatedSize: prediction.estimated_size,
                    estimatedRatio: prediction.estimated_ratio
                };
            } catch (error) {
                log.error('AIBridge', '智能配置获取失败', { error: error.message });
                throw error;
            }
        }
        
        /**
         * 💡 显示AI建议
         */
        displayAISuggestion(config) {
            if (!window.addLog) return;
            
            window.addLog('🤖 AI建议:', 'info');
            window.addLog(`  格式: ${config.format}`, 'info');
            window.addLog(`  质量: ${config.quality}`, 'info');
            window.addLog(`  速度: ${config.speed}`, 'info');
            window.addLog(`  无损: ${config.lossless ? '是' : '否'}`, 'info');
            
            if (config.confidence) {
                const confidencePercent = (config.confidence * 100).toFixed(0);
                window.addLog(`  置信度: ${confidencePercent}%`, 'info');
            }
            
            if (config.estimatedSize) {
                const sizeMB = (config.estimatedSize / (1024 * 1024)).toFixed(2);
                window.addLog(`  预计大小: ${sizeMB} MB`, 'info');
            }
            
            if (config.estimatedRatio) {
                const ratioPercent = (config.estimatedRatio * 100).toFixed(0);
                window.addLog(`  压缩比: ${ratioPercent}%`, 'info');
            }
        }
        
        /**
         * 🎨 创建AI建议UI
         */
        createAISuggestionUI(config) {
            const container = document.createElement('div');
            container.className = 'ai-suggestion';
            container.style.cssText = `
                margin: 10px 0;
                padding: 15px;
                border-radius: 8px;
                background: #e3f2fd;
                border-left: 4px solid #2196f3;
            `;
            
            // 标题
            const title = document.createElement('div');
            title.style.cssText = 'font-weight: bold; margin-bottom: 10px; color: #1976d2;';
            title.textContent = '🤖 AI建议';
            container.appendChild(title);
            
            // 参数列表
            const params = [
                { label: '格式', value: config.format },
                { label: '质量', value: config.quality },
                { label: '速度', value: config.speed },
                { label: '无损', value: config.lossless ? '是' : '否' }
            ];
            
            if (config.confidence) {
                params.push({ 
                    label: '置信度', 
                    value: `${(config.confidence * 100).toFixed(0)}%` 
                });
            }
            
            if (config.estimatedSize) {
                params.push({ 
                    label: '预计大小', 
                    value: `${(config.estimatedSize / (1024 * 1024)).toFixed(2)} MB` 
                });
            }
            
            if (config.estimatedRatio) {
                params.push({ 
                    label: '压缩比', 
                    value: `${(config.estimatedRatio * 100).toFixed(0)}%` 
                });
            }
            
            const list = document.createElement('div');
            list.style.cssText = 'font-size: 14px;';
            
            params.forEach(param => {
                const item = document.createElement('div');
                item.style.cssText = 'margin: 5px 0;';
                item.innerHTML = `<strong>${param.label}:</strong> ${param.value}`;
                list.appendChild(item);
            });
            
            container.appendChild(list);
            
            return container;
        }
        
        /**
         * 📍 在页面中显示AI建议
         */
        showAISuggestionInPage(config, containerId = 'ai-suggestion-container') {
            const container = document.getElementById(containerId);
            if (!container) {
                log.warn('AIBridge', 'AI建议容器不存在', { containerId });
                return;
            }
            
            // 清空现有内容
            container.innerHTML = '';
            
            // 添加AI建议UI
            const ui = this.createAISuggestionUI(config);
            container.appendChild(ui);
        }
        
        /**
         * 🔄 应用AI建议到UI
         */
        applyAISuggestionToUI(config) {
            log.info('AIBridge', '应用AI建议到UI', config);
            
            // 设置格式
            const formatSelect = document.getElementById('format');
            if (formatSelect && config.format) {
                formatSelect.value = config.format;
                // 触发change事件
                formatSelect.dispatchEvent(new Event('change'));
            }
            
            // 设置质量
            const qualitySlider = document.getElementById('quality');
            if (qualitySlider && config.quality) {
                qualitySlider.value = config.quality;
                // 触发input事件
                qualitySlider.dispatchEvent(new Event('input'));
            }
            
            // 设置速度
            const speedSlider = document.getElementById('speed');
            if (speedSlider && config.speed) {
                speedSlider.value = config.speed;
                // 触发input事件
                speedSlider.dispatchEvent(new Event('input'));
            }
            
            // 设置无损
            const losslessCheckbox = document.getElementById('lossless');
            if (losslessCheckbox && config.lossless !== undefined) {
                losslessCheckbox.checked = config.lossless;
                // 触发change事件
                losslessCheckbox.dispatchEvent(new Event('change'));
            }
            
            log.info('AIBridge', 'AI建议已应用到UI');
        }
    }
    
    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.AIBridge = AIBridge;
    
    // 创建单例
    window.PIXLY.aiBridge = new AIBridge();
    
    if (log) {
        log.info('AIBridge', '✅ AI Bridge已加载');
    }
    
})(window);
