/**
 * 🎛️ Pixly本地化配置UI
 * 
 * 为插件用户提供直观的功能开关控制：
 * - AI服务开关
 * - Rust加速开关  
 * - 优化模式选择
 * - 性能级别调节
 */

class PixlyLocalConfigUI {
    constructor(aiClient) {
        this.aiClient = aiClient;
        this.config = null;
        this.isInitialized = false;
        
        this.init();
    }

    async init() {
        try {
            // 加载当前配置
            this.config = await this.aiClient.getConfig();
            this.isInitialized = true;
            
            // 创建配置UI
            this.createConfigUI();
            
            // 定期更新状态
            setInterval(() => this.updateStatus(), 5000);
            
            console.log('✅ 本地化配置UI已初始化');
        } catch (error) {
            console.error('❌ 配置UI初始化失败:', error);
        }
    }

    /**
     * 创建配置UI面板
     */
    createConfigUI() {
        // 查找插入位置（在optimization mode之后）
        const modeSection = document.getElementById('modeSection');
        if (!modeSection) return;

        // 创建配置面板
        const configSection = document.createElement('section');
        configSection.className = 'config-section';
        configSection.id = 'localConfigSection';
        configSection.style.cssText = 'display: none; margin-top: 20px;';
        
        configSection.innerHTML = `
            <div class="local-config-panel">
                <h3 style="margin: 0 0 15px 0; font-size: 16px; color: #333;">
                    ⚙️ 本地化AI配置
                    <span id="configStatus" style="font-size: 12px; margin-left: 10px;"></span>
                </h3>
                
                <!-- AI服务状态 -->
                <div class="config-group" style="margin-bottom: 20px; padding: 15px; background: #f8f9fa; border-radius: 8px;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                        <h4 style="margin: 0; font-size: 14px;">🤖 AI服务</h4>
                        <div id="aiServiceStatus" class="status-indicator">加载中...</div>
                    </div>
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <label style="font-size: 13px;">服务状态</label>
                        <span id="aiServiceValue">-</span>
                    </div>
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center;">
                        <label style="font-size: 13px;">已加载模型</label>
                        <span id="modelsLoadedValue">-</span>
                    </div>
                </div>

                <!-- 性能优化 -->
                <div class="config-group" style="margin-bottom: 20px; padding: 15px; background: #f0f8ff; border-radius: 8px;">
                    <h4 style="margin: 0 0 10px 0; font-size: 14px;">🚀 性能优化</h4>
                    
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                        <label style="font-size: 13px;">
                            <span>Rust加速</span>
                            <small style="display: block; color: #666; font-size: 11px;">SIMD并行计算</small>
                        </label>
                        <label class="switch">
                            <input type="checkbox" id="rustAccelSwitch">
                            <span class="slider"></span>
                        </label>
                    </div>
                    
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                        <label style="font-size: 13px;">性能级别</label>
                        <select id="performanceLevelSelect" style="padding: 4px 8px; border-radius: 4px; border: 1px solid #ddd;">
                            <option value="power_saver">节能模式</option>
                            <option value="balanced">平衡模式</option>
                            <option value="high_performance">高性能</option>
                            <option value="maximum">极限模式</option>
                        </select>
                    </div>
                </div>

                <!-- 功能开关 -->
                <div class="config-group" style="margin-bottom: 20px; padding: 15px; background: #fff8f0; border-radius: 8px;">
                    <h4 style="margin: 0 0 10px 0; font-size: 14px;">⚡ 功能开关</h4>
                    
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <label style="font-size: 13px;">智能格式推荐</label>
                        <label class="switch">
                            <input type="checkbox" id="formatRecommendationSwitch">
                            <span class="slider"></span>
                        </label>
                    </div>
                    
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <label style="font-size: 13px;">批量处理</label>
                        <label class="switch">
                            <input type="checkbox" id="batchProcessingSwitch">
                            <span class="slider"></span>
                        </label>
                    </div>
                    
                    <div class="config-row" style="display: flex; justify-content: space-between; align-items: center;">
                        <label style="font-size: 13px;">预测缓存</label>
                        <label class="switch">
                            <input type="checkbox" id="predictionCacheSwitch">
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <!-- 性能统计 -->
                <div class="config-group" style="padding: 15px; background: #f0fff0; border-radius: 8px;">
                    <h4 style="margin: 0 0 10px 0; font-size: 14px;">📊 实时统计</h4>
                    <div id="performanceStats" style="font-size: 12px; color: #666;">
                        加载统计数据...
                    </div>
                </div>

                <!-- 操作按钮 -->
                <div style="margin-top: 20px; display: flex; gap: 10px; justify-content: flex-end;">
                    <button id="resetConfigBtn" class="btn btn-secondary" style="font-size: 13px;">
                        重置默认
                    </button>
                    <button id="applyConfigBtn" class="btn btn-primary" style="font-size: 13px;">
                        应用配置
                    </button>
                </div>
            </div>

            <style>
                .switch {
                    position: relative;
                    display: inline-block;
                    width: 44px;
                    height: 24px;
                }
                
                .switch input {
                    opacity: 0;
                    width: 0;
                    height: 0;
                }
                
                .slider {
                    position: absolute;
                    cursor: pointer;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    background-color: #ccc;
                    transition: 0.3s;
                    border-radius: 24px;
                }
                
                .slider:before {
                    position: absolute;
                    content: "";
                    height: 18px;
                    width: 18px;
                    left: 3px;
                    bottom: 3px;
                    background-color: white;
                    transition: 0.3s;
                    border-radius: 50%;
                }
                
                input:checked + .slider {
                    background-color: #4CAF50;
                }
                
                input:checked + .slider:before {
                    transform: translateX(20px);
                }
                
                .status-indicator {
                    font-size: 12px;
                    padding: 4px 8px;
                    border-radius: 12px;
                    font-weight: bold;
                }
                
                .status-online { background: #d4edda; color: #155724; }
                .status-offline { background: #f8d7da; color: #721c24; }
                .status-loading { background: #fff3cd; color: #856404; }
            </style>
        `;

        // 插入到页面中
        modeSection.parentNode.insertBefore(configSection, modeSection.nextSibling);
        
        // 绑定事件
        this.bindConfigEvents();
        
        // 初始化UI状态
        this.updateConfigUI();
    }

    /**
     * 绑定配置事件
     */
    bindConfigEvents() {
        // Rust加速开关
        const rustSwitch = document.getElementById('rustAccelSwitch');
        if (rustSwitch) {
            rustSwitch.addEventListener('change', (e) => {
                this.updateFeature('rust_acceleration', e.target.checked);
            });
        }

        // 性能级别选择
        const levelSelect = document.getElementById('performanceLevelSelect');
        if (levelSelect) {
            levelSelect.addEventListener('change', (e) => {
                this.updatePerformanceLevel(e.target.value);
            });
        }

        // 功能开关
        const switches = [
            { id: 'formatRecommendationSwitch', feature: 'format_recommendation' },
            { id: 'batchProcessingSwitch', feature: 'batch_processing' },
            { id: 'predictionCacheSwitch', feature: 'prediction_cache' }
        ];

        switches.forEach(({ id, feature }) => {
            const element = document.getElementById(id);
            if (element) {
                element.addEventListener('change', (e) => {
                    this.updateFeature(feature, e.target.checked);
                });
            }
        });

        // 按钮事件
        const resetBtn = document.getElementById('resetConfigBtn');
        if (resetBtn) {
            resetBtn.addEventListener('click', () => this.resetConfig());
        }

        const applyBtn = document.getElementById('applyConfigBtn');
        if (applyBtn) {
            applyBtn.addEventListener('click', () => this.applyConfig());
        }
    }

    /**
     * 更新配置UI显示
     */
    async updateConfigUI() {
        if (!this.config) return;

        try {
            // 更新开关状态
            const rustSwitch = document.getElementById('rustAccelSwitch');
            if (rustSwitch) {
                rustSwitch.checked = this.config.performance?.enable_simd || false;
            }

            const levelSelect = document.getElementById('performanceLevelSelect');
            if (levelSelect) {
                levelSelect.value = this.config.ai_service?.performance_level || 'balanced';
            }

            // 更新功能开关
            const formatSwitch = document.getElementById('formatRecommendationSwitch');
            if (formatSwitch) {
                formatSwitch.checked = this.config.features?.format_recommendation !== false;
            }

            const batchSwitch = document.getElementById('batchProcessingSwitch');
            if (batchSwitch) {
                batchSwitch.checked = this.config.features?.batch_processing !== false;
            }

            const cacheSwitch = document.getElementById('predictionCacheSwitch');
            if (cacheSwitch) {
                cacheSwitch.checked = this.config.performance?.enable_caching !== false;
            }
        } catch (error) {
            console.warn('更新配置UI失败:', error);
        }
    }

    /**
     * 更新状态显示
     */
    async updateStatus() {
        try {
            const health = await this.aiClient.checkHealth();
            const stats = await this.aiClient.getPerformanceStats();

            // 更新AI服务状态
            const serviceStatus = document.getElementById('aiServiceStatus');
            const serviceValue = document.getElementById('aiServiceValue');
            const modelsValue = document.getElementById('modelsLoadedValue');

            if (health.status === 'healthy') {
                if (serviceStatus) {
                    serviceStatus.textContent = '在线';
                    serviceStatus.className = 'status-indicator status-online';
                }
                if (serviceValue) serviceValue.textContent = '正常运行';
                if (modelsValue) modelsValue.textContent = `${health.models_loaded || 0} 个`;
            } else {
                if (serviceStatus) {
                    serviceStatus.textContent = '离线';
                    serviceStatus.className = 'status-indicator status-offline';
                }
                if (serviceValue) serviceValue.textContent = '服务不可用';
                if (modelsValue) modelsValue.textContent = '0 个';
            }

            // 更新性能统计
            const statsElement = document.getElementById('performanceStats');
            if (statsElement && stats) {
                statsElement.innerHTML = `
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                        <div>总请求: ${stats.total_requests}</div>
                        <div>成功率: ${(stats.success_rate * 100).toFixed(1)}%</div>
                        <div>平均响应: ${stats.avg_response_time_ms.toFixed(1)}ms</div>
                        <div>Rust加速: ${stats.rust_acceleration ? '✅' : '❌'}</div>
                    </div>
                    <div style="margin-top: 8px; font-size: 11px; color: #999;">
                        运行时间: ${Math.floor(stats.uptime_seconds / 60)} 分钟
                    </div>
                `;
            }

        } catch (error) {
            console.warn('更新状态失败:', error);
        }
    }

    /**
     * 更新功能开关
     */
    async updateFeature(featureName, enabled) {
        try {
            await this.aiClient.toggleFeature(featureName, enabled);
            
            // 显示成功提示
            this.showNotification(`${featureName} ${enabled ? '已启用' : '已禁用'}`, 'success');
        } catch (error) {
            console.error(`更新功能开关失败:`, error);
            this.showNotification(`更新 ${featureName} 失败`, 'error');
        }
    }

    /**
     * 更新性能级别
     */
    async updatePerformanceLevel(level) {
        try {
            const updates = {
                ai_service: {
                    performance_level: level
                }
            };

            await this.aiClient.updateConfig(updates);
            this.showNotification(`性能级别已设置为: ${level}`, 'success');
        } catch (error) {
            console.error('更新性能级别失败:', error);
            this.showNotification('更新性能级别失败', 'error');
        }
    }

    /**
     * 重置配置
     */
    async resetConfig() {
        if (!confirm('确定要重置为默认配置吗？')) return;

        try {
            // 重置为默认值
            const defaultConfig = {
                performance: {
                    enable_simd: true,
                    enable_gpu: false
                },
                features: {
                    format_recommendation: true,
                    batch_processing: true,
                    prediction_cache: true
                },
                ai_service: {
                    performance_level: 'balanced'
                }
            };

            await this.aiClient.updateConfig(defaultConfig);
            this.config = await this.aiClient.getConfig();
            
            this.updateConfigUI();
            this.showNotification('配置已重置为默认值', 'success');
        } catch (error) {
            console.error('重置配置失败:', error);
            this.showNotification('重置配置失败', 'error');
        }
    }

    /**
     * 应用配置
     */
    async applyConfig() {
        try {
            // 获取当前UI状态并应用
            const updates = this.collectUIConfig();
            await this.aiClient.updateConfig(updates);
            
            this.showNotification('配置已应用', 'success');
        } catch (error) {
            console.error('应用配置失败:', error);
            this.showNotification('应用配置失败', 'error');
        }
    }

    /**
     * 收集UI配置
     */
    collectUIConfig() {
        const rustSwitch = document.getElementById('rustAccelSwitch');
        const levelSelect = document.getElementById('performanceLevelSelect');
        const formatSwitch = document.getElementById('formatRecommendationSwitch');
        const batchSwitch = document.getElementById('batchProcessingSwitch');
        const cacheSwitch = document.getElementById('predictionCacheSwitch');

        return {
            performance: {
                enable_simd: rustSwitch?.checked || false,
                enable_gpu: rustSwitch?.checked || false,
                enable_caching: cacheSwitch?.checked !== false
            },
            features: {
                format_recommendation: formatSwitch?.checked !== false,
                batch_processing: batchSwitch?.checked !== false
            },
            ai_service: {
                performance_level: levelSelect?.value || 'balanced'
            }
        };
    }

    /**
     * 显示通知
     */
    showNotification(message, type = 'info') {
        // 简单的通知实现
        console.log(`[${type.toUpperCase()}] ${message}`);
        
        // 可以集成到现有的通知系统
        if (window.showNotification) {
            window.showNotification(message, type);
        }
    }

    /**
     * 显示/隐藏配置面板
     */
    toggle() {
        const section = document.getElementById('localConfigSection');
        if (section) {
            const isVisible = section.style.display !== 'none';
            section.style.display = isVisible ? 'none' : 'block';
            return !isVisible;
        }
        return false;
    }

    /**
     * 检查是否已初始化
     */
    isReady() {
        return this.isInitialized;
    }
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PixlyLocalConfigUI;
} else {
    window.PixlyLocalConfigUI = PixlyLocalConfigUI;
}
