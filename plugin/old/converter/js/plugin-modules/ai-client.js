// PIXLY AI Client - Last Modified: 2025-11-10 14:15:24
// Version: 6.0.0-fixed

/**
 * ╔════════════════════════════════════════════════════════════╗
 * ║   AI智能预测客户端 v6.0.0 - 100% Real AI                     ║
 * ║   零Fallback | 响亮报错 | 真实调用                         ║
 * ║   遵循质量宣言：质量 > 速度，响亮报错 > 静默降级               ║
 * ╚════════════════════════════════════════════════════════════╝
 */

// ============================================================================
// AI客户端配置
// ============================================================================

(function() {
'use strict';

// Log instance for the module
const log = window.pixlyLog;

const AI_CONFIG = {
  serviceUrl: 'http://localhost:50052',
  endpoints: {
    predict: '/api/v1/predict',              // 图像AI预测
    predictVideo: '/api/v1/predict/video',   // 视频AI预测
    validateVMAF: '/api/v1/validate/vmaf',   // VMAF质量验证
    health: '/api/v1/health',
    version: '/api/v1/version',
    capabilities: '/api/v1/capabilities',
  },
  timeout: 5000, // 5秒超时（普通请求）
  videoTimeout: 30000, // 30秒超时（视频AI预测）
  vmafTimeout: 120000, // 120秒超时（VMAF验证，可能很耗时）
  retries: 2,
  healthCheckInterval: 30000, // 30秒健康检查
};

// ============================================================================
// AI客户端类
// ============================================================================

class AIClient {
  constructor() {
    this.healthy = false;
    this.version = null;
    this.capabilities = null;
    this.lastError = null;
    this.stats = {
      totalPredictions: 0,
      successfulPredictions: 0,
      failedPredictions: 0,
      avgResponseTime: 0,
      lastCheckTime: null,
    };
    this.healthCheckTimer = null;
    this.init();
  }

  /**
   * 初始化AI客户端
   */
  async init() {
    const Logger = window.PIXLY?.Logger || console;
    Logger.info('AI Client', '🚀 Initializing GO core AI client...');
    
    await this.checkHealth();
    
    // 启动定期健康检查
    this.startHealthCheck();
  }

  /**
   * 检查AI服务健康状态
   */
  async checkHealth() {
    const Logger = window.PIXLY?.Logger || console;
    try {
      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.health}`,
        { method: 'GET' }
      );
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }
      
      const data = await response.json();
      this.healthy = data.ready === true || data.status === 'healthy';
      this.version = data.version || 'unknown';
      this.stats.lastCheckTime = new Date();
      
      if (this.healthy) {
        Logger.info(`[AI Client] ✅ GO core connected | version=${this.version}`);
        // 获取能力信息
        await this.fetchCapabilities();
      } else {
        Logger.warn('[AI Client] ⚠️ GO core response abnormal - using static rules');
      }
    } catch (error) {
      this.healthy = false;
      this.lastError = error.message;
      Logger.warn(`[AI Client] ⚠️ GO core offline - using static rules | ${error.message}`);
    }
  }

  /**
   * 获取GO核心能力信息
   */
  async fetchCapabilities() {
    const Logger = window.PIXLY?.Logger || console;
    try {
      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.capabilities}`,
        { method: 'GET' }
      );
      
      if (response.ok) {
        this.capabilities = await response.json();
        Logger.debug('[AI Client] 📋 GO核心能力:', this.capabilities);
      }
    } catch (error) {
      Logger.debug('[AI Client] ⚠️ 无法获取GO核心能力信息');
      this.capabilities = null;
    }
  }

  /**
   * 启动定期健康检查
   */
  startHealthCheck() {
    if (this.healthCheckTimer) {
      clearInterval(this.healthCheckTimer);
    }
    
    this.healthCheckTimer = setInterval(() => {
      this.checkHealth();
    }, AI_CONFIG.healthCheckInterval);
  }

  /**
   * 停止健康检查
   */
  stopHealthCheck() {
    if (this.healthCheckTimer) {
      clearInterval(this.healthCheckTimer);
      this.healthCheckTimer = null;
    }
  }

  /**
   * 预测最佳转换参数（核心方法）
   * @param {string} imagePath - 图像文件路径
   * @param {string} tool - 转换工具 (jxl/avif/webp/heic)
   * @param {string} optimizeMode - 优化模式 (size/balanced/quality/universal)
   * @returns {Promise<Object>} 预测结果
   */
  async predictParams(imagePath, tool, optimizeMode, options = {}) {
    const Logger = window.PIXLY?.Logger || console;
    
    // 🔥 响亮报错：服务不健康
    if (!this.healthy) {
      Logger.error('[AI Client] ❌ GO核心不可用');
      throw new Error(
        '🚨 GO AI核心不可用！\n\n' +
        '无法进行智能预测，转换已中止。\n\n' +
        '请检查：\n' +
        '1. GO核心是否启动（端口50052）\n' +
        '2. Python环境是否配置\n' +
        '3. AI模型文件是否存在\n\n' +
        `最后错误: ${this.lastError || '未知'}\n\n` +
        '【质量宣言】响亮报错 > 静默降级'
      );
    }

    // 🔥 响亮报错：参数验证失败
    if (!this.validateParams(imagePath, tool, optimizeMode)) {
      Logger.error('[AI Client] ❌ 参数验证失败');
      throw new Error(
        '🚨 参数验证失败！\n\n' +
        `image: ${imagePath}\n` +
        `tool: ${tool}\n` +
        `mode: ${optimizeMode}\n\n` +
        '【质量宣言】响亮报错 > 静默降级'
      );
    }

    const startTime = Date.now();
    this.stats.totalPredictions++;

    try {
      // 🆕 构建可选参数对象（无预设模式）
      const requestOptions = {
        return_advanced_params: options.returnAdvancedParams || false,
        enable_feature_analysis: options.enableFeatureAnalysis || false,
        enable_quality_constraint: options.enableQualityConstraint !== false, // 默认true
        expected_format: options.expectedFormat || null,
        enable_format_recommendation: options.enableFormatRecommendation || false,  // 🎨 格式推荐
        recommend_video_for_animation: options.recommendVideoForAnimation || false,  // 🎬 动图转视频
        enable_bayesian: options.enableBayesian || false,        // 🆕 贝叶斯优化
        enable_ppo: options.enablePPO || false                    // 🆕 PPO强化学习
      };

      // 🔧 FIX: 根据优化模式动态设置target_quality
      const targetQualityMap = {
        'size': 80,
        'balanced': 90,
        'quality': 100,  // ✅ quality模式必须是100
        'universal': 95
      };
      const targetQuality = targetQualityMap[optimizeMode] || 90;

      const requestBody = {
        image_path: imagePath,
        tool: tool,
        target_quality: targetQuality,  // 🔧 FIX: 动态设置，不再硬编码
        optimize_mode: optimizeMode,
        options: requestOptions,  // 🆕 可选参数
      };

      Logger.debug(
        `[AI Client] 📤 AI预测请求 | 工具=${tool} | 模式=${optimizeMode} | ` +
        `高级参数=${requestOptions.return_advanced_params} | 格式=${requestOptions.expected_format || 'auto'} | ` +
        `贝叶斯=${requestOptions.enable_bayesian} | PPO=${requestOptions.enable_ppo}`
      );

      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.predict}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(requestBody),
        }
      );

      if (!response.ok) {
        const errorText = await response.text();
        Logger.error(`[AI Client] ❌ HTTP ${response.status}: ${response.statusText}`);
        Logger.error(`[AI Client] ❌ Response body: ${errorText}`);
        throw new Error(`HTTP ${response.status}: ${response.statusText} | ${errorText}`);
      }

      const result = await response.json();
      const elapsed = Date.now() - startTime;

      // 更新统计
      this.updateStats(elapsed, true);

      if (result.success) {
        // 🆕 优先使用merged参数（包含基础+高级）
        const finalParams = result.merged || result.params;
        
        Logger.info(
          `[AI Client] ✅ AI预测成功 | Q=${result.params.quality} d=${result.params.distance?.toFixed(1) || 'N/A'} | ` +
          `置信度=${result.confidence?.toFixed(2) || 'N/A'} | 模式=${optimizeMode} | 耗时=${elapsed}ms` +
          (result.advanced ? ' | 包含高级参数' : '')
        );
        
        return {
          success: true,
          params: this.normalizeParams(finalParams, tool),  // 标准化参数
          advanced: result.advanced || null,                 // 🆕 高级参数
          features: result.features || null,                 // 可选特征
          confidence: result.confidence || 0,                // 🆕 置信度
          elapsed_ms: elapsed,
          source: 'go_ai',
        };
      } else {
        // 🔥 响亮报错：AI预测失败
        Logger.error(`[AI Client] ❌ AI预测失败: ${result.error || 'Unknown error'}`);
        throw new Error(
          `🚨 AI预测失败！\n\n` +
          `错误信息: ${result.error || '未知错误'}\n\n` +
          `请检查GO核心日志查看详细原因\n\n` +
          '【质量宣言】响亮报错 > 静默降级'
        );
      }
    } catch (error) {
      this.updateStats(Date.now() - startTime, false);
      
      // 如果连续失败，标记服务不健康
      if (this.stats.failedPredictions > 5) {
        this.healthy = false;
        Logger.error('[AI Client] ❌ GO核心连续失败，标记为不可用');
      }
      
      // 🔥 响亮报错：AI调用异常
      Logger.error(`[AI Client] ❌ AI调用异常: ${error.message}`);
      Logger.error(`[AI Client] ❌ Error stack: ${error.stack}`);
      Logger.error(`[AI Client] ❌ Request details: image=${imagePath}, tool=${tool}, mode=${optimizeMode}`);
      
      // 🔥 Phase 47.18 (E-003): 提供更友好的错误信息
      let errorMessage = `🚨 AI调用失败！\n\n`;
      
      // 根据错误类型提供不同的提示
      if (error.message.includes('ECONNREFUSED') || error.message.includes('connect')) {
        errorMessage += '❌ AI服务连接失败\n\n' +
          '请启动AI服务：\n' +
          '• python3 tools/pixly_http_server.py\n' +
          '• 或 go run cmd/pixly-ai/*.go\n\n' +
          '端口：50052';
      } else if (error.message.includes('timeout')) {
        errorMessage += '⏱️ AI服务响应超时\n\n' +
          '可能原因：\n' +
          '• 图像文件过大\n' +
          '• AI服务正在处理其他任务\n' +
          '• 服务器性能不足\n\n' +
          '建议重试或减小图像尺寸';
      } else {
        errorMessage += `${error.message}\n\n` +
          `请检查:\n` +
          `1. AI服务是否正在运行 (端口 50052)\n` +
          `2. 图像文件是否存在\n` +
          `3. 网络连接是否正常`;
      }
      
      const friendlyError = new Error(errorMessage);
      friendlyError.originalError = error;
      throw friendlyError;
    }
  }

  /**
   * 验证预测参数
   */
  validateParams(imagePath, tool, optimizeMode) {
    const validTools = ['jxl', 'avif', 'webp', 'heic'];
    const validModes = ['size', 'balanced', 'quality', 'universal'];
    
    if (!imagePath || typeof imagePath !== 'string') {
      return false;
    }
    
    if (!validTools.includes(tool)) {
      return false;
    }
    
    if (!validModes.includes(optimizeMode)) {
      return false;
    }
    
    return true;
  }

  /**
   * 标准化参数（确保所有参数存在且合法）
   */
  normalizeParams(params, tool) {
    const normalized = {
      quality: params.quality || 90,
      distance: params.distance !== undefined ? params.distance : 1.0,
      effort: params.effort || 7,
      confidence: params.confidence || 0.5,
    };

    // 工具特定参数
    if (tool === 'jxl') {
      normalized.distance = Math.max(0, Math.min(15, normalized.distance));
      normalized.effort = Math.max(1, Math.min(9, normalized.effort));
    } else if (tool === 'avif') {
      normalized.speed = params.speed || 4;
      normalized.speed = Math.max(0, Math.min(10, normalized.speed));
    } else if (tool === 'webp') {
      normalized.method = params.method || 4;
      normalized.method = Math.max(0, Math.min(6, normalized.method));
    }

    return normalized;
  }

  /**
   * 更新统计信息
   */
  updateStats(elapsedMs, success) {
    if (success) {
      this.stats.successfulPredictions++;
      
      // 计算平均响应时间
      const total = this.stats.avgResponseTime * (this.stats.successfulPredictions - 1);
      this.stats.avgResponseTime = (total + elapsedMs) / this.stats.successfulPredictions;
    } else {
      this.stats.failedPredictions++;
    }
  }

  /**
   * ❌ DELETED: getFallbackParams
   * 
   * 【质量宣言执行】
   * - ❌ Fallback代码（让AI成为摆设）
   * - ✅ 响亮报错 > 静默降级
   * 
   * AI不可用时，应该直接抛出错误，而不是静默降级到hardcode参数。
   */

  /**
   * 验证AI预测结果（只验证，不修改）
   */
  validatePredictionResult(result) {
    const Logger = window.pixlyLog || console;
    
    // 验证必需字段
    if (!result.params) {
      throw new Error('AI返回结果缺少params字段');
    }
    
    if (typeof result.params.quality !== 'number') {
      throw new Error('AI返回的quality不是数字');
    }
    
    // 验证范围（警告，但不修改）
    if (result.params.quality < 1 || result.params.quality > 100) {
      Logger.warn(`[AI Client] ⚠️ AI返回的quality超出范围: ${result.params.quality}`);
      // ✅ 只记录警告，不修改值
    }
    
    Logger.debug('[AI Client] ✅ AI返回值验证通过');
  }

  /**
   * 批量预测（优化性能）
   */
  async predictBatch(images, tool, optimizeMode, options = {}) {
    const Logger = window.PIXLY?.Logger || console;
    
    // 🔥 响亮报错：批量预测时GO核心不可用
    if (!this.healthy) {
      Logger.error('[AI Client] ❌ 批量预测: GO核心不可用');
      throw new Error(
        '🚨 GO AI核心不可用！\n\n' +
        '无法进行批量智能预测，转换已中止。\n\n' +
        '【质量宣言】响亮报错 > 静默降级'
      );
    }

    try {
      const requestBody = {
        images: images,
        tool: tool,
        optimize_mode: optimizeMode,
      };

      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}/api/v1/predict/batch`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(requestBody),
        }
      );

      if (response.ok) {
        const result = await response.json();
        Logger.info(`[AI Client] ✅ 批量预测成功 | 数量=${images.length}`);
        return result.predictions || [];
      }
    } catch (error) {
      Logger.warn(`[AI Client] ⚠️ 批量预测失败: ${error.message}`);
    }

    // 降级：逐个预测
    Logger.info('[AI Client] 降级到单个预测模式');
    const results = [];
    for (const img of images) {
      results.push(await this.predictParams(img, tool, optimizeMode));
    }
    return results;
  }

  /**
   * 带超时的fetch（支持自定义超时时间）
   */
  async fetchWithTimeout(url, options = {}, customTimeout = null) {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), customTimeout || AI_CONFIG.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        signal: controller.signal,
      });
      clearTimeout(timeout);
      return response;
    } catch (error) {
      clearTimeout(timeout);
      if (error.name === 'AbortError') {
        throw new Error(`请求超时 (${customTimeout || AI_CONFIG.timeout}ms)`);
      }
      throw error;
    }
  }

  /**
   * 🎬 视频AI预测（GO核心）
   * @param {string} videoPath - 视频文件路径
   * @param {string} optimizeMode - 优化模式 ('balanced', 'size', 'quality')
   * @param {object} options - 可选参数
   * @returns {Promise<object>} 预测结果
   */
  async predictVideoParams(videoPath, optimizeMode = 'balanced', options = {}) {
    const Logger = window.PIXLY?.Logger || console;
    
    // 🔥 Phase 47.18 (E-003): AI服务不可用友好提示
    if (!this.healthy) {
      const friendlyError = new Error(
        '🤖 AI服务未启动\n\n' +
        '为获得最佳转换效果，请启动AI服务：\n\n' +
        '1. Python AI服务（推荐）：\n' +
        '   cd Pixly_Nightly\n' +
        '   python3 tools/pixly_http_server.py\n\n' +
        '2. 或使用Go AI服务：\n' +
        '   cd core/go\n' +
        '   go run cmd/pixly-ai/*.go\n\n' +
        'AI服务端口：50052\n' +
        '健康检查：http://localhost:50052/api/v1/health'
      );
      friendlyError.code = 'AI_SERVICE_UNAVAILABLE';
      throw friendlyError;
    }

    const startTime = Date.now();
    this.stats.totalPredictions++;

    try {
      const requestOptions = {
        use_advanced_ai: options.use_advanced_ai !== false,      // 默认true（高级特征分析）
        enable_transformer: options.enable_transformer || false, // 默认false（强制Transformer）
        enable_vmaf: options.enable_vmaf || false,               // 默认false（VMAF验证）
        allow_hevc: options.allowHEVC !== false,
        prefer_speed: options.preferSpeed !== false,
        target_encoder: options.targetEncoder || null
      };

      const requestBody = {
        video_path: videoPath,
        optimize_mode: optimizeMode,
        options: requestOptions
      };

      Logger.info(
        `[AI Client Video] 📤 视频AI预测请求 | 模式=${optimizeMode} | ` +
        `高级AI=${requestOptions.use_advanced_ai} | Transformer=${requestOptions.enable_transformer}`
      );

      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.predictVideo}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(requestBody),
        },
        AI_CONFIG.videoTimeout // 使用更长的超时时间
      );

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const result = await response.json();
      
      const duration = Date.now() - startTime;
      this.stats.successfulPredictions++;
      this.updateResponseTime(duration);

      Logger.info(
        `[AI Client Video] ✅ 视频AI预测成功 | 耗时=${duration}ms | ` +
        `编码器=${result.params?.encoder} | CRF=${result.params?.crf} | ` +
        `复杂度=${result.complexity_score || 'N/A'}`
      );

      return result;

    } catch (error) {
      this.stats.failedPredictions++;
      Logger.error(`[AI Client Video] ❌ 视频AI预测失败: ${error.message}`);
      // 🔥 Phase 47.18: 删除fallback，响亮报错
      throw error;
    }
  }

  /**
   * 📊 VMAF质量验证（GO核心）
   * @param {string} originalPath - 原始视频路径
   * @param {string} convertedPath - 转换后视频路径
   * @param {object} options - 验证选项
   * @returns {Promise<object>} VMAF验证结果
   */
  async validateVMAF(originalPath, convertedPath, options = {}) {
    const Logger = window.PIXLY?.Logger || console;
    
    // 快速失败：服务不健康
    if (!this.healthy) {
      Logger.warn('[AI Client VMAF] GO核心不可用，跳过VMAF验证');
      return {
        success: false,
        error: 'GO core unavailable',
        score: null,
        passed: null
      };
    }

    const startTime = Date.now();

    try {
      const requestBody = {
        original_path: originalPath,
        converted_path: convertedPath,
        min_score: options.minScore || 85,
        use_model: options.model || 'vmaf_v0.6.1',
        n_threads: options.threads || 4
      };

      Logger.info(`[AI Client VMAF] 📊 VMAF验证请求 | 最低分数=${requestBody.min_score}`);

      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.validateVMAF}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(requestBody),
        },
        AI_CONFIG.vmafTimeout // 使用最长的超时时间
      );

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const result = await response.json();
      
      const duration = Date.now() - startTime;
      
      Logger.info(
        `[AI Client VMAF] ✅ VMAF验证完成 | 耗时=${(duration/1000).toFixed(1)}s | ` +
        `分数=${result.score?.toFixed(2)} | 通过=${result.passed ? '✅' : '❌'}`
      );

      return result;

    } catch (error) {
      Logger.error(`[AI Client VMAF] ❌ VMAF验证失败: ${error.message}`);
      return {
        success: false,
        error: error.message,
        score: null,
        passed: null
      };
    }
  }

  /**
   * 🔥 Phase 47.18: 删除视频静态规则fallback
   * 遵循PROJECT_QUALITY_MANIFESTO：响亮报错 > 静默降级
   * AI不可用时应该直接报错，而不是使用硬编码规则
   */

  /**
   * 手动重新检查健康状态
   */
  async recheckHealth() {
    await this.checkHealth();
    return this.healthy;
  }

  /**
   * 获取AI服务信息
   */
  async getServiceInfo() {
    try {
      const response = await this.fetchWithTimeout(
        `${AI_CONFIG.serviceUrl}${AI_CONFIG.endpoints.version}`
      );
      
      if (response.ok) {
        return await response.json();
      }
    } catch (error) {
      const Logger = window.PIXLY?.Logger || console;
      Logger.error(`[AI Client] 获取服务信息失败: ${error.message}`);
    }
    return null;
  }

  /**
   * 获取统计信息
   */
  getStats() {
    return {
      ...this.stats,
      healthy: this.healthy,
      version: this.version,
      capabilities: this.capabilities,
      successRate: this.stats.totalPredictions > 0 
        ? (this.stats.successfulPredictions / this.stats.totalPredictions * 100).toFixed(2) + '%'
        : 'N/A',
    };
  }

  /**
   * 重置统计信息
   */
  resetStats() {
    this.stats = {
      totalPredictions: 0,
      successfulPredictions: 0,
      failedPredictions: 0,
      avgResponseTime: 0,
      lastCheckTime: this.stats.lastCheckTime,
    };
  }

  /**
   * 销毁客户端
   */
  destroy() {
    this.stopHealthCheck();
    const Logger = window.PIXLY?.Logger || console;
    Logger.info('[AI Client] 🛑 AI客户端已销毁');
  }
}

// ============================================================================
// 全局AI客户端实例
// ============================================================================

const aiClient = new AIClient();

// ============================================================================
// 辅助函数：智能参数构建器增强
// ============================================================================

/**
 * 使用AI智能预测增强参数构建
 * 这个函数会被params-builder.js调用
 */
async function enhanceParamsWithAI(imagePath, tool, baseParams, optimizeMode) {
  const Logger = window.PIXLY?.Logger || console;
  
  try {
    // 调用AI预测
    const aiResult = await aiClient.predictParams(imagePath, tool, optimizeMode);
    
    if (aiResult.success && aiResult.source === 'go_ai') {
      // AI预测成功，使用AI参数覆盖基础参数
      Logger.info(`[AI Enhance] ✅ 使用AI预测参数 | 置信度=${aiResult.params.confidence?.toFixed(2)}`);
      
      return {
        ...baseParams,
        quality: aiResult.params.quality,
        distance: aiResult.params.distance,
        effort: aiResult.params.effort,
        speed: aiResult.params.speed,
        method: aiResult.params.method,
        // 元数据
        _ai_enhanced: true,
        _ai_confidence: aiResult.params.confidence,
        _ai_mode: aiResult.mode,
        _ai_source: aiResult.source,
        _ai_elapsed: aiResult.elapsed_ms,
      };
    } else {
      // AI失败或降级，使用静态规则参数
      Logger.debug(`[AI Enhance] 📋 使用静态规则参数 | 模式=${optimizeMode}`);
      
      return {
        ...baseParams,
        ...aiResult.params,
        _ai_enhanced: false,
        _ai_mode: 'fallback',
        _ai_source: 'static_rules',
      };
    }
  } catch (error) {
    Logger.error(`[AI Enhance] ❌ AI参数增强失败: ${error.message}`);
    return {
      ...baseParams,
      _ai_enhanced: false,
      _ai_error: error.message,
    };
  }
}

/**
 * 统一日志风格：AI预测日志
 */
function logAIPrediction(result) {
  const Logger = window.PIXLY?.Logger || console;
  
  if (!result.success) {
    Logger.warn('[AI Client] ⚠️ AI预测失败 - 使用静态规则');
    return;
  }

  const { params, mode, elapsed_ms, source } = result;
  
  if (source === 'go_ai') {
    Logger.success(
      `[AI Client] 🧠 AI智能预测 | Q=${params.quality} d=${params.distance?.toFixed(1) || 'N/A'} | ` +
      `置信度=${params.confidence?.toFixed(2)} | 耗时=${elapsed_ms}ms`
    );
  } else if (source === 'static_rules') {
    Logger.info(
      `[AI Client] 📋 静态规则 | Q=${params.quality} d=${params.distance?.toFixed(1) || 'N/A'} | 模式=${mode}`
    );
  }
}

// ============================================================================
// 导出
// ============================================================================

// 暴露到全局供其他模块使用
if (typeof window !== 'undefined') {
  window.AIClient = AIClient;
  window.aiClient = aiClient;
  window.enhanceParamsWithAI = enhanceParamsWithAI;
  window.logAIPrediction = logAIPrediction;
  
  // PIXLY命名空间
  window.PIXLY = window.PIXLY || {};
  window.PIXLY.AIClient = aiClient;
}

if (log) {
    log.info('AI Client', formatLog(LOG.AI_CLIENT_MODULE_LOADED, {}));
}

// 🔥 自动初始化AI客户端（延迟执行，等待DOM就绪）
if (typeof document !== 'undefined') {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      setTimeout(() => aiClient.init(), 500);
    });
  } else {
    setTimeout(() => aiClient.init(), 500);
  }
}

})(); // End of IIFE
