/**
 * 🧠 Pixly本地化AI客户端
 * 
 * 完全替代HTTP调用的本地化实现：
 * - 直接调用Python函数，零网络依赖
 * - 完全兼容现有插件代码
 * - 替代localhost:50052 HTTP服务
 */

class PixlyLocalAIClient {
    constructor() {
        this.pythonBridge = null;
        this.isInitialized = false;
        this.config = {
            ai_enabled: true,
            rust_acceleration: false,
            performance_level: 'balanced'
        };
        
        this._initializeBridge();
    }

    /**
     * 初始化Python桥接器
     */
    async _initializeBridge() {
        try {
            // 尝试加载Python桥接器
            if (typeof require !== 'undefined') {
                // Node.js环境或Eagle插件环境
                const { spawn } = require('child_process');
                const path = require('path');
                
                // 找到Python AI模块路径
                const pythonPath = this._findPythonPath();
                const modulePath = path.join(__dirname, '../../../core/python/ai/plugin_local_bridge.py');
                
                console.log('✅ 本地AI客户端：使用直接Python调用');
                this.isInitialized = true;
            } else {
                console.warn('⚠️ 本地AI客户端：Node.js环境不可用，使用模拟模式');
                this.isInitialized = false;
            }
        } catch (error) {
            console.error('❌ 本地AI客户端初始化失败:', error);
            this.isInitialized = false;
        }
    }

    /**
     * 找到Python路径
     */
    _findPythonPath() {
        const possiblePaths = [
            'python3',
            'python',
            '/usr/bin/python3',
            '/usr/local/bin/python3'
        ];
        return possiblePaths[0]; // 简化：使用第一个
    }

    /**
     * 执行Python函数调用
     */
    async _callPythonFunction(functionName, args = []) {
        if (!this.isInitialized) {
            throw new Error('本地AI客户端未初始化');
        }

        try {
            const { spawn } = require('child_process');
            const pythonPath = this._findPythonPath();
            
            // 构建Python命令
            const pythonScript = `
import sys
sys.path.append('${__dirname}/../../../core/python')
from ai.plugin_local_bridge import ${functionName}

try:
    result = ${functionName}(${args.map(arg => JSON.stringify(arg)).join(', ')})
    print(result)
except Exception as e:
    import json
    print(json.dumps({"success": false, "error": str(e)}))
`;

            return new Promise((resolve, reject) => {
                const python = spawn(pythonPath, ['-c', pythonScript]);
                let output = '';
                let error = '';

                python.stdout.on('data', (data) => {
                    output += data.toString();
                });

                python.stderr.on('data', (data) => {
                    error += data.toString();
                });

                python.on('close', (code) => {
                    if (code === 0) {
                        try {
                            const result = JSON.parse(output.trim());
                            resolve(result);
                        } catch (e) {
                            reject(new Error(`解析Python输出失败: ${output}`));
                        }
                    } else {
                        reject(new Error(`Python执行失败 (code ${code}): ${error}`));
                    }
                });

                // 超时处理
                setTimeout(() => {
                    python.kill();
                    reject(new Error('Python调用超时'));
                }, 30000);
            });

        } catch (error) {
            throw new Error(`Python函数调用失败: ${error.message}`);
        }
    }

    /**
     * 检查AI服务健康状态
     */
    async checkHealth() {
        try {
            if (!this.isInitialized) {
                return {
                    status: 'offline',
                    error: '本地AI客户端未初始化'
                };
            }

            const result = await this._callPythonFunction('plugin_health_check');
            
            // 更新本地配置
            this.config.ai_enabled = result.ai_enabled || false;
            this.config.rust_acceleration = result.rust_available || false;
            
            return result;
        } catch (error) {
            console.error('本地AI健康检查失败:', error);
            return {
                status: 'error',
                error: error.message
            };
        }
    }

    /**
     * 预测图像优化参数 - 完全本地化
     */
    async predictImage(imagePath, mode = 'balanced', options = {}) {
        try {
            const requestData = {
                image_path: imagePath,
                target_format: options.expected_format || 'auto',
                mode: mode,
                options: {
                    enable_format_recommendation: true,
                    enable_preprocessing: true,
                    enable_magika: options.enable_magika !== false,
                    enable_caching: options.enable_caching !== false
                }
            };

            const result = await this._callPythonFunction('plugin_predict_image', [JSON.stringify(requestData)]);
            
            if (!result.success) {
                throw new Error(result.error || 'Image prediction failed');
            }

            return result;
        } catch (error) {
            console.error('本地AI图像预测失败:', error);
            throw error;
        }
    }

    /**
     * 预测视频优化参数 - 复用图像预测逻辑
     */
    async predictVideo(videoPath, mode = 'balanced', options = {}) {
        // 标记为视频类型
        options.media_type = 'video';
        return this.predictImage(videoPath, mode, options);
    }

    /**
     * 预测音频优化参数 - 复用图像预测逻辑
     */
    async predictAudio(audioPath, mode = 'balanced', options = {}) {
        // 标记为音频类型
        options.media_type = 'audio';
        return this.predictImage(audioPath, mode, options);
    }

    /**
     * 自动检测并预测 - 兼容现有插件代码
     */
    async predictAuto(filePath, mode = 'balanced', options = {}) {
        const ext = filePath.split('.').pop().toLowerCase();
        
        // 图像格式
        const imageFormats = ['jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'bmp', 'tiff'];
        if (imageFormats.includes(ext)) {
            return await this.predictImage(filePath, mode, options);
        }
        
        // 视频格式
        const videoFormats = ['mp4', 'mkv', 'avi', 'mov', 'webm', 'flv', 'm4v'];
        if (videoFormats.includes(ext)) {
            return await this.predictVideo(filePath, mode, options);
        }
        
        // 音频格式
        const audioFormats = ['mp3', 'aac', 'm4a', 'opus', 'ogg', 'flac', 'wav'];
        if (audioFormats.includes(ext)) {
            return await this.predictAudio(filePath, mode, options);
        }
        
        throw new Error(`Unsupported file format: ${ext}`);
    }

    /**
     * 获取AI模型列表 - 本地化实现
     */
    async getModels() {
        try {
            const result = await this._callPythonFunction('plugin_get_models');
            return result;
        } catch (error) {
            console.error('获取模型列表失败:', error);
            return {
                models: {},
                total: 0,
                active: 0,
                error: error.message
            };
        }
    }

    /**
     * 获取配置和开关状态
     */
    async getConfig() {
        try {
            const result = await this._callPythonFunction('plugin_get_config');
            this.config = result; // 更新本地配置缓存
            return result;
        } catch (error) {
            console.error('获取配置失败:', error);
            return this.config; // 返回缓存的配置
        }
    }

    /**
     * 更新配置开关
     */
    async updateConfig(updates) {
        try {
            const result = await this._callPythonFunction('plugin_update_config', [JSON.stringify(updates)]);
            
            if (result.success) {
                // 更新本地配置缓存
                Object.assign(this.config, result.updated_config || {});
            }
            
            return result;
        } catch (error) {
            console.error('更新配置失败:', error);
            return {
                success: false,
                error: error.message
            };
        }
    }

    /**
     * 批量预测 - 本地化优化
     */
    async predictBatch(files, mode = 'balanced', options = {}) {
        const results = [];
        
        for (const file of files) {
            try {
                const result = await this.predictAuto(file.path || file.name, mode, options);
                results.push({
                    file: file,
                    prediction: result,
                    status: 'success'
                });
            } catch (error) {
                results.push({
                    file: file,
                    error: error.message,
                    status: 'error'
                });
            }
        }
        
        return results;
    }

    /**
     * 检查功能开关状态
     */
    isFeatureEnabled(featureName) {
        const config = this.config;
        
        switch (featureName) {
            case 'ai_service':
                return config.ai_service?.enabled || false;
            case 'rust_acceleration':
                return config.performance?.enable_simd || config.rust_acceleration || false;
            case 'format_recommendation':
                return config.features?.format_recommendation !== false;
            case 'batch_processing':
                return config.features?.batch_processing !== false;
            case 'smart_recommendation':
                return config.features?.smart_recommendation !== false;
            default:
                return false;
        }
    }

    /**
     * 启用/禁用功能开关
     */
    async toggleFeature(featureName, enabled) {
        const updates = {};
        
        switch (featureName) {
            case 'rust_acceleration':
                updates.performance = { enable_simd: enabled, enable_gpu: enabled };
                break;
            case 'format_recommendation':
                updates.features = { format_recommendation: enabled };
                break;
            case 'batch_processing':
                updates.features = { batch_processing: enabled };
                break;
            default:
                throw new Error(`Unknown feature: ${featureName}`);
        }
        
        return await this.updateConfig(updates);
    }

    /**
     * 获取性能统计
     */
    async getPerformanceStats() {
        try {
            const health = await this.checkHealth();
            return {
                total_requests: health.total_requests || 0,
                success_rate: health.success_rate || 0,
                avg_response_time_ms: health.avg_response_time_ms || 0,
                rust_acceleration: this.config.rust_acceleration || false,
                uptime_seconds: health.uptime_seconds || 0
            };
        } catch (error) {
            return {
                total_requests: 0,
                success_rate: 0,
                avg_response_time_ms: 0,
                rust_acceleration: false,
                error: error.message
            };
        }
    }
}

// 兼容现有代码：创建PixlyAIClient别名
class PixlyAIClient extends PixlyLocalAIClient {
    constructor(baseURL) {
        console.log('🔄 检测到HTTP AI客户端调用，自动切换到本地化实现');
        super(); // 忽略baseURL，使用本地化实现
    }
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { PixlyLocalAIClient, PixlyAIClient };
} else {
    window.PixlyLocalAIClient = PixlyLocalAIClient;
    window.PixlyAIClient = PixlyAIClient; // 向后兼容
}
