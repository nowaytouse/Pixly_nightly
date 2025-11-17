/**
 * 🤖 AI Integration Module - Phase 33 深度集成
 * 
 * 功能:
 * - AI模型选择UI
 * - 实时性能监控
 * - 智能格式推荐
 * - 用户反馈机制
 */

(function() {
    'use strict';

    // ===== 全局状态 =====
    window.PIXLY_AI = {
        enabled: false,
        activeModels: {},
        stats: null,
        lastUpdate: null,
    };

    // ===== AI服务测试 =====
    
    async function testAIService() {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', formatLog(LOG.AI_INTEGRATION_INIT, {}));
        }
        
        try {
            // 🔥 直接检测Go AI服务（端口50052），不依赖Rust CLI
            const http = require('http');
            const result = await new Promise((resolve) => {
                const timeout = setTimeout(() => {
                    if (log) {
                        log.error('AI', 'AI service detection timeout (2s)');
                    }
                    resolve(false);
                }, 2000);
                
                const req = http.get('http://localhost:50052/api/v1/version', (res) => {
                    clearTimeout(timeout);
                    if (res.statusCode === 200) {
                        if (log) {
                            log.info('AI', `AI service responded: HTTP ${res.statusCode}`);
                        }
                        resolve(true);
                    } else {
                        if (log) {
                            log.error('AI', `AI service responded with error: HTTP ${res.statusCode}`);
                        }
                        resolve(false);
                    }
                });
                
                req.on('error', (err) => {
                    clearTimeout(timeout);
                    if (log) {
                        log.error('AI', `AI service connection error: ${err.message}`);
                    }
                    resolve(false);
                });
                
                req.setTimeout(1500, () => {
                    req.destroy();
                    if (log) {
                        log.error('AI', 'AI service request timeout');
                    }
                    clearTimeout(timeout);
                    resolve(false);
                });
            });
            
            if (result) {
                if (log) {
                    log.info('AI', formatLog(LOG.AI_INTEGRATION_ENABLED, {}));
                }
                window.PIXLY_AI.enabled = true;
                return true;
            } else {
                if (log) {
                    log.error('AI', formatLog(LOG.AI_INTEGRATION_GO_UNAVAILABLE, {}));
                    log.error('AI', 'Please start Go AI service: cd core/go && go run cmd/pixly-ai/main.go');
                }
                window.PIXLY_AI.enabled = false;
                return false;
            }
        } catch (error) {
            if (log) {
                log.error('AI', `AI service test failed: ${error.message || error}`);
            }
            window.PIXLY_AI.enabled = false;
            return false;
        }
    }

    // ===== 获取活跃模型 =====
    
    async function getActiveModels() {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', 'Fetching active models...');
        }
        
        // 🔥 直接调用Go AI服务的API
        try {
            const http = require('http');
            const models = await new Promise((resolve) => {
                const timeout = setTimeout(() => resolve({}), 2000);
                
                // 🔥 Phase 40.17: 使用标准 RESTful 路径（Go 服务支持两种路径）
                const req = http.get('http://localhost:50052/api/v1/models', (res) => {
                    let data = '';
                    res.on('data', chunk => data += chunk);
                    res.on('end', () => {
                        clearTimeout(timeout);
                        try {
                            const json = JSON.parse(data);
                            resolve(json.models || {});
                        } catch {
                            resolve({});
                        }
                    });
                });
                
                req.on('error', () => {
                    clearTimeout(timeout);
                    resolve({});
                });
            });
            
            window.PIXLY_AI.activeModels = models;
            window.PIXLY_AI.lastUpdate = Date.now();
            
            if (log) {
                log.info('AI', `Retrieved models: ${JSON.stringify(models)}`);
            }
            return models;
        } catch (error) {
            if (log) {
                log.warn('AI', `Failed to get active models: ${error.message || error}`);
            }
            return {};
        }
    }

    // ===== 获取模型统计 =====
    
    async function getModelStats() {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', 'Fetching model stats...');
        }
        
        // 🔥 真实调用Go AI服务的stats API
        try {
            const http = require('http');
            const stats = await new Promise((resolve) => {
                const timeout = setTimeout(() => resolve(null), 2000);
                
                const req = http.get('http://localhost:50052/api/v1/training/stats', (res) => {
                    let data = '';
                    res.on('data', chunk => data += chunk);
                    res.on('end', () => {
                        clearTimeout(timeout);
                        try {
                            const json = JSON.parse(data);
                            resolve(json);
                        } catch {
                            resolve(null);
                        }
                    });
                });
                
                req.on('error', () => {
                    clearTimeout(timeout);
                    resolve(null);
                });
            });
            
            if (stats) {
                window.PIXLY_AI.stats = stats;
                if (log) {
                    log.info('AI', 'Retrieved real stats from Go service');
                }
                return stats;
            } else {
                if (log) {
                    log.warn('AI', 'Stats API not available');
                }
                return null;
            }
        } catch (error) {
            if (log) {
                log.error('AI', `Failed to get stats: ${error.message || error}`);
            }
            return null;
        }
    }

    // ===== 智能推荐 =====
    
    async function getSmartRecommendation(fileInfo) {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', formatLog(LOG.AI_INTEGRATION_PREDICTING, { file: fileInfo.name }));
        }
        
        const rustCLI = window.rustCLI;
        if (!rustCLI || !rustCLI.available) {
            return null;
        }
        
        try {
            const input = JSON.stringify({
                ext: fileInfo.ext.replace('.', ''),
                size: fileInfo.size,
                name: fileInfo.name
            });
            
            const result = await rustCLI.exec('--smart-recommend', [input]);
            const recommendation = JSON.parse(result.stdout || 'null');
            
            if (log) {
                log.info('AI', formatLog(LOG.AI_INTEGRATION_PREDICTION_SUCCESS, { result: JSON.stringify(recommendation) }));
            }
            return recommendation;
        } catch (error) {
            if (log) {
                log.warn('AI', formatLog(LOG.AI_INTEGRATION_PREDICTION_FAILED, { error: error.message || error }));
            }
            return null;
        }
    }

    // ===== 发送反馈 =====
    
    async function sendFeedback(feedbackData) {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', 'Sending feedback...');
        }
        
        const rustCLI = window.rustCLI;
        if (!rustCLI || !rustCLI.available) {
            return false;
        }
        
        try {
            const input = JSON.stringify(feedbackData);
            const result = await rustCLI.exec('--send-feedback', [input]);
            const response = JSON.parse(result.stdout || '{"success":false}');
            
            if (response.success) {
                if (log) {
                    log.info('AI', 'Feedback sent successfully');
                }
                return true;
            } else {
                if (log) {
                    log.warn('AI', `Feedback failed: ${response.error}`);
                }
                return false;
            }
        } catch (error) {
            if (log) {
                log.warn('AI', `Failed to send feedback: ${error.message || error}`);
            }
            return false;
        }
    }

    // ===== UI组件：模型选择器 =====
    
    function createModelSelectorUI() {
        const container = document.getElementById('aiModelContainer');
        if (!container) {
            const log = window.pixlyLog;
            if (log) {
                log.warn('AI', 'AI model container not found');
            }
            return;
        }
        
        container.innerHTML = `
            <div class="ai-model-selector" style="margin-bottom: 15px;">
                <label style="display: flex; align-items: center; gap: 8px; font-size: 13px;">
                    <span>🤖 AI模型:</span>
                    <select id="aiModelSelect" style="flex: 1; padding: 6px; border-radius: 4px; border: 1px solid #ddd;">
                        <option value="auto">自动选择 (A/B测试)</option>
                    </select>
                    <button id="aiRefreshBtn" style="padding: 6px 12px; border-radius: 4px; border: 1px solid #ddd; background: white; cursor: pointer;">
                        🔄
                    </button>
                </label>
                <div id="aiModelInfo" style="margin-top: 8px; font-size: 12px; color: #666;">
                    正在加载模型信息...
                </div>
            </div>
        `;
        
        // 加载模型列表
        loadModelSelector();
        
        // 绑定刷新按钮
        document.getElementById('aiRefreshBtn').addEventListener('click', loadModelSelector);
    }

    async function loadModelSelector() {
        const select = document.getElementById('aiModelSelect');
        const info = document.getElementById('aiModelInfo');
        
        if (!select || !info) return;
        
        const i18n = window.i18n || { t: (key) => key };
        info.textContent = i18n.t('common.loading');
        
        const models = await getActiveModels();
        
        // 清空现有选项（保留"自动选择"）
        while (select.options.length > 1) {
            select.remove(1);
        }
        
        // 添加模型选项
        for (const [name, modelInfo] of Object.entries(models)) {
            const option = document.createElement('option');
            option.value = `${name}:${modelInfo.version}`;
            option.textContent = `${name} v${modelInfo.version} (准确率: ${(modelInfo.metrics?.accuracy * 100 || 0).toFixed(1)}%)`;
            select.appendChild(option);
        }
        
        if (Object.keys(models).length > 0) {
            info.textContent = `✅ 已加载 ${Object.keys(models).length} 个模型`;
        } else {
            info.textContent = i18n.t('ai.serviceUnavailable');
        }
    }

    // ===== UI组件：性能监控面板 =====
    
    function createPerformanceMonitorUI() {
        const container = document.getElementById('aiStatsContainer');
        if (!container) {
            const log = window.pixlyLog;
            if (log) {
                log.warn('AI', 'AI stats container not found');
            }
            return;
        }
        
        container.innerHTML = `
            <div class="ai-performance-monitor" style="padding: 15px; background: #f8f9fa; border-radius: 8px; margin-bottom: 15px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                    <h4 style="margin: 0; font-size: 14px;">📊 AI性能监控</h4>
                    <button id="aiStatsRefreshBtn" style="padding: 4px 8px; font-size: 11px; border-radius: 4px; border: 1px solid #ddd; background: white; cursor: pointer;">
                        刷新
                    </button>
                </div>
                <div id="aiStatsContent" style="font-size: 12px;">
                    正在加载统计数据...
                </div>
            </div>
        `;
        
        // 加载统计数据
        loadPerformanceMonitor();
        
        // 绑定刷新按钮
        document.getElementById('aiStatsRefreshBtn').addEventListener('click', loadPerformanceMonitor);
    }

    async function loadPerformanceMonitor() {
        const content = document.getElementById('aiStatsContent');
        if (!content) return;
        
        content.textContent = i18n.t('common.loading');
        
        const stats = await getModelStats();
        
        if (!stats || stats.error) {
            content.innerHTML = `
                <div style="color: #ff6b6b;">
                    ⚠️ 无法获取统计数据
                    ${stats?.error ? `<br><small>${stats.error}</small>` : ''}
                </div>
            `;
            return;
        }
        
        // 显示统计信息
        const models = stats.models || {};
        let html = '';
        
        for (const [name, versions] of Object.entries(models)) {
            for (const model of (versions || [])) {
                const metrics = model.metrics || {};
                html += `
                    <div style="margin-bottom: 10px; padding: 10px; background: white; border-radius: 4px; border: 1px solid #e0e0e0;">
                        <div style="font-weight: bold; margin-bottom: 5px;">
                            ${name} v${model.version}
                            <span style="color: ${model.status === 'active' ? '#4caf50' : '#ff9800'}; font-size: 11px;">
                                [${model.status}]
                            </span>
                        </div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 5px; font-size: 11px; color: #666;">
                            <div>准确率: ${(metrics.accuracy * 100 || 0).toFixed(1)}%</div>
                            <div>延迟: ${(metrics.avg_latency || 0).toFixed(1)}ms</div>
                            <div>调用: ${metrics.total_calls || 0}次</div>
                            <div>错误率: ${((metrics.error_rate || 0) * 100).toFixed(1)}%</div>
                        </div>
                    </div>
                `;
            }
        }
        
        if (html) {
            content.innerHTML = html;
        } else {
            content.innerHTML = '<div style="color: #999;">暂无数据</div>';
        }
    }

    // ===== UI组件：智能推荐提示 =====
    
    async function showSmartRecommendation(fileInfo) {
        if (!window.PIXLY_AI.enabled) return;
        
        const recommendation = await getSmartRecommendation(fileInfo);
        if (!recommendation) return;
        
        // 显示推荐提示
        const message = `
            <div style="padding: 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border-radius: 8px; margin-bottom: 15px;">
                <div style="font-size: 14px; font-weight: bold; margin-bottom: 8px;">
                    🎯 AI智能推荐
                </div>
                <div style="font-size: 13px; margin-bottom: 8px;">
                    推荐格式: <strong>${recommendation.recommended_format.toUpperCase()}</strong>
                </div>
                <div style="font-size: 12px; opacity: 0.9; margin-bottom: 8px;">
                    ${recommendation.reason}
                </div>
                <div style="font-size: 11px; opacity: 0.8;">
                    置信度: ${(recommendation.confidence * 100).toFixed(0)}%
                </div>
            </div>
        `;
        
        // 插入到转换UI的顶部
        const conversionContainer = document.querySelector('.conversion-options');
        if (conversionContainer) {
            const existingRecommendation = conversionContainer.querySelector('.ai-recommendation');
            if (existingRecommendation) {
                existingRecommendation.remove();
            }
            
            const div = document.createElement('div');
            div.className = 'ai-recommendation';
            div.innerHTML = message;
            conversionContainer.insertBefore(div, conversionContainer.firstChild);
        }
    }

    // ===== UI组件：反馈对话框 =====
    
    function showFeedbackDialog(conversionResult) {
        const overlay = document.createElement('div');
        overlay.id = 'aiFeedbackOverlay';
        overlay.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.6);
            z-index: 1000000;
            display: flex;
            justify-content: center;
            align-items: center;
            backdrop-filter: blur(5px);
        `;
        
        const dialog = document.createElement('div');
        dialog.style.cssText = `
            max-width: 500px;
            padding: 30px;
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
        `;
        
        dialog.innerHTML = `
            <h3 style="margin: 0 0 15px 0; font-size: 18px;">💬 转换反馈</h3>
            <p style="margin: 0 0 15px 0; font-size: 14px; color: #666;">
                请为本次转换打分，帮助AI改进
            </p>
            <div style="margin-bottom: 20px;">
                <div style="margin-bottom: 10px; font-size: 13px; color: #666;">
                    转换: ${conversionResult.input_format} → ${conversionResult.output_format}
                </div>
                <div style="display: flex; gap: 10px; justify-content: center; margin-bottom: 15px;">
                    ${[1, 2, 3, 4, 5].map(star => `
                        <button class="ai-star-btn" data-rating="${star}" style="
                            font-size: 32px;
                            border: none;
                            background: none;
                            cursor: pointer;
                            transition: transform 0.2s;
                        ">
                            ⭐
                        </button>
                    `).join('')}
                </div>
                <textarea id="aiFeedbackComment" placeholder="可选：输入您的评论..." style="
                    width: 100%;
                    min-height: 80px;
                    padding: 10px;
                    border: 1px solid #ddd;
                    border-radius: 6px;
                    font-size: 13px;
                    resize: vertical;
                "></textarea>
            </div>
            <div style="display: flex; gap: 10px; justify-content: flex-end;">
                <button id="aiFeedbackSkip" style="
                    padding: 10px 20px;
                    border: 1px solid #ddd;
                    background: white;
                    border-radius: 6px;
                    cursor: pointer;
                    font-size: 13px;
                ">
                    跳过
                </button>
                <button id="aiFeedbackSubmit" disabled style="
                    padding: 10px 20px;
                    border: none;
                    background: #4caf50;
                    color: white;
                    border-radius: 6px;
                    cursor: pointer;
                    font-size: 13px;
                    opacity: 0.5;
                ">
                    提交反馈
                </button>
            </div>
        `;
        
        overlay.appendChild(dialog);
        document.body.appendChild(overlay);
        
        // 绑定事件
        let selectedRating = 0;
        
        dialog.querySelectorAll('.ai-star-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                selectedRating = parseInt(btn.dataset.rating);
                
                // 更新星星显示
                dialog.querySelectorAll('.ai-star-btn').forEach((b, idx) => {
                    b.textContent = idx < selectedRating ? '⭐' : '☆';
                    b.style.transform = idx < selectedRating ? 'scale(1.2)' : 'scale(1)';
                });
                
                // 启用提交按钮
                document.getElementById('aiFeedbackSubmit').disabled = false;
                document.getElementById('aiFeedbackSubmit').style.opacity = '1';
            });
            
            btn.addEventListener('mouseenter', () => {
                btn.style.transform = 'scale(1.3)';
            });
            
            btn.addEventListener('mouseleave', () => {
                const rating = parseInt(btn.dataset.rating);
                btn.style.transform = rating <= selectedRating ? 'scale(1.2)' : 'scale(1)';
            });
        });
        
        document.getElementById('aiFeedbackSkip').addEventListener('click', () => {
            overlay.remove();
        });
        
        document.getElementById('aiFeedbackSubmit').addEventListener('click', async () => {
            const comment = document.getElementById('aiFeedbackComment').value;
            
            const feedbackData = {
                input_format: conversionResult.input_format,
                output_format: conversionResult.output_format,
                quality: conversionResult.quality || 90,
                user_rating: selectedRating,
                comment: comment || null,
                timestamp: Date.now(),
            };
            
            const success = await sendFeedback(feedbackData);
            overlay.remove();
            
            if (success) {
                showNotification('✅ 感谢您的反馈！', 'success');
            }
        });
    }

    // ===== 初始化 =====
    
    async function initAIIntegration() {
        const log = window.pixlyLog;
        if (log) {
            log.info('AI', formatLog(LOG.AI_INTEGRATION_INIT, {}));
        }
        
        // 等待Rust CLI就绪
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // 测试AI服务
        const serviceAvailable = await testAIService();
        
        // 🔥 更新UI状态显示
        const imageStatus = document.getElementById('imageGoCoreStatus');
        const videoStatus = document.getElementById('videoGoCoreInline');
        const goCoreStatus = document.getElementById('goCoreStatus');
        
        if (serviceAvailable) {
            if (log) {
                log.info('AI', formatLog(LOG.AI_INTEGRATION_MODULE_LOADED, {}));
            }
            
            // ✅ Update status to online
            const i18n = window.i18n || { t: (key) => key };
            if (imageStatus) {
                imageStatus.innerHTML = `<span style="color: #10B981;">✅ ${i18n.t('ai.online')}</span>`;
            }
            if (videoStatus) {
                videoStatus.innerHTML = `<span style="color: #10B981;">✅ ${i18n.t('ai.online')}</span>`;
            }
            if (goCoreStatus) {
                goCoreStatus.innerHTML = `<span style="color: #10B981;">✅ ${i18n.t('common.online')}</span>`;
            }
            
            // 创建UI组件（如果容器存在）
            createModelSelectorUI();
            createPerformanceMonitorUI();
            
            // 定期刷新数据（每30秒）
            setInterval(async () => {
                if (window.PIXLY_AI.enabled) {
                    await getActiveModels();
                }
            }, 30000);
        } else {
            if (log) {
                log.error('AI', 'AI service not available, attempting auto-start...');
            }
            
            // 🔥 Phase 46.5.14: 尝试自动启动AI服务
            try {
                if (window.PIXLY && window.PIXLY.UIHandlers && window.PIXLY.UIHandlers.fixAICore) {
                    if (log) {
                        log.info('AI', 'Auto-starting AI service...');
                    }
                    // 延迟500ms后自动启动，避免与初始化冲突
                    setTimeout(async () => {
                        try {
                            await window.PIXLY.UIHandlers.fixAICore();
                            if (log) {
                                log.info('AI', 'Auto-start completed');
                            }
                        } catch (error) {
                            if (log) {
                                log.warn('AI', `Auto-start failed: ${error.message}`);
                            }
                        }
                    }, 500);
                }
            } catch (error) {
                if (log) {
                    log.warn('AI', `Cannot auto-start AI service: ${error.message || error}`);
                }
            }
            
            // ❌ 更新状态为离线（稍后会被fixAICore更新）
            const i18n = window.i18n || { t: (key) => key };
            if (imageStatus) {
                imageStatus.innerHTML = '<span style="color: #EF4444;">❌ AI离线</span>';
                imageStatus.title = i18n.t('title.aiOffline');
            }
            if (videoStatus) {
                videoStatus.innerHTML = '<span style="color: #EF4444;">❌ AI离线</span>';
                videoStatus.title = i18n.t('title.aiOfflineShort');
            }
            if (goCoreStatus) {
                goCoreStatus.innerHTML = '<span style="color: #EF4444;">❌ 离线</span>';
                goCoreStatus.parentElement.title = i18n.t('title.aiOfflineShort');
            }
            
            // 🔥 修复：禁用AI功能标志，避免误用
            window.PIXLY_AI.enabled = false;
            if (log) {
                log.info('AI', formatLog(LOG.AI_INTEGRATION_DISABLED, {}));
            }
        }
    }

    // ===== 导出API =====
    window.PIXLY_AI_INTEGRATION = {
        testService: testAIService,
        getActiveModels: getActiveModels,
        getModelStats: getModelStats,
        getSmartRecommendation: getSmartRecommendation,
        sendFeedback: sendFeedback,
        showSmartRecommendation: showSmartRecommendation,
        showFeedbackDialog: showFeedbackDialog,
        loadModelSelector: loadModelSelector,
        loadPerformanceMonitor: loadPerformanceMonitor,
    };

    // 自动初始化
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initAIIntegration);
    } else {
        initAIIntegration();
    }

})();
