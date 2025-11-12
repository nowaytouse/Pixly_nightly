/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * AI Client - Rust↔Python AI服务集成
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🔥 Phase 47: 架构简化 - 移除Go，双端架构
 * 
 * 🎯 新架构 (2端):
 * 
 * 1. 【职责分离 - 双端架构】
 *    🦀 Rust核心: 转换执行
 *       - CLI工具 (cjxl, avifenc, cwebp, ffmpeg)
 *       - Native编码器 (rav1e, webp, image)
 *       - 批量处理 + 并行编码
 *    
 *    🐍 Python AI服务: AI预测（可选）
 *       - HTTP API (端口50052)
 *       - 模型管理 (LightGBM)
 *       - 视频分析 (ffprobe)
 *       - Flask轻量级框架
 *    
 *    🔗 JS插件: 可选UI (不做转换)
 *       - 调用Rust CLI
 *       - 不实现任何转换逻辑
 * 
 * 2. 【通信协议】
 *    - 端口: 固定50052 (Python HTTP服务)
 *    - 协议: HTTP JSON
 *    - 超时: 5秒 (PIXLY_AI_TIMEOUT)
 *    - 降级: AI失败 → Mock预测 (不中断转换)
 * 
 * 3. 【AI预测字段 (Phase 36 - 完整版)】
 *    必须字段:
 *    - quality: Option<u8>                 // 质量参数 (0-100)
 *    - speed: Option<u8>                   // 速度参数 (format-specific)
 *    - lossless: Option<bool>              // 🔥 AI智能预测无损模式
 *    - lossless_jpeg: Option<bool>         // 🔥 JPEG→JXL特殊处理
 *    - format_options: Option<Vec<...>>    // 🔥 格式特定参数
 *    
 *    元数据:
 *    - confidence: f32                     // 置信度
 *    - reasoning: String                   // 推理原因
 *    - model_used: Option<String>          // 使用的模型
 * 
 * 4. 【禁止事项 - 必须遵守】
 *    ❌ 不要在JS插件实现转换 (04-conversion.js已删除)
 *    ❌ 不要硬编码lossless=false (必须用AI预测或启发式)
 *    ❌ 不要硬编码format_options=[] (必须用AI预测)
 *    ❌ 不要修改端口为8080 (固定50052)
 *    ❌ 不要在Python服务实现转换 (仅AI预测)
 * 
 * 5. 【测试要求】
 *    ✅ 命令行测试优先: pixly-rust convert input.jpg output.jxl
 *    ✅ AI服务独立测试: curl localhost:50052/api/v1/health
 *    ✅ 集成测试: Rust + Python联调
 *    ✅ 反馈收集: 自动反馈 + 训练队列
 * 
 * API端点 (Python服务):
 * - GET  /api/v1/health - 健康检查
 * - POST /api/v1/predict - 图像AI预测
 * - POST /api/v1/predict/video - 视频AI预测
 * 
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 */
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use lazy_static::lazy_static;

// 🔧 统一日志系统 - 使用 tracing 宏
use tracing::{info, warn, error, debug};

/// AI服务配置
#[derive(Debug, Clone)]
pub struct AIServiceConfig {
    /// AI服务URL
    pub base_url: String,
    /// 超时时间
    pub timeout: Duration,
    /// 是否启用
    pub enabled: bool,
}

impl Default for AIServiceConfig {
    fn default() -> Self {
        Self {
            // 🔧 Phase 47: Python AI服务端口 (tools/pixly_http_server.py: port 50052)
            base_url: "http://localhost:50052".to_string(),
            timeout: Duration::from_secs(5),
            enabled: true,
        }
    }
}

/// AI预测请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    // 🔥 修复：真实文件路径（用于Python AI分析）
    pub image_path: Option<String>,
    
    // 输入图像metadata
    pub input_format: String,
    pub target_format: String,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub has_alpha: bool,
    pub has_animation: bool,
    pub color_space: Option<String>,
    pub bit_depth: Option<u8>,
    
    // 用户偏好
    pub priority: String,  // "size", "balanced", "quality"
    pub preserve_quality: bool,
    
    // 🆕 Phase 34: 模型选择
    pub model_type: Option<String>,  // "lightgbm", "ppo", "transformer", "ensemble", "baseline", "auto"
    pub request_id: Option<String>,  // 用于A/B测试分流
    
    // 🔥 Phase 40.31: AI高级选项（UI→JS→Rust→GO完整链路）
    pub enable_bayesian: Option<bool>,       // 贝叶斯优化
    pub enable_ppo: Option<bool>,            // PPO强化学习
    pub enable_smart_quality: Option<bool>,  // 智能质量预测
    pub enable_auto_optimize: Option<bool>,  // 自动参数优化
    pub enable_video_for_anim: Option<bool>, // 动图转视频推荐
}

/// AI预测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResponse {
    // 推荐参数
    pub quality: Option<u8>,
    pub speed: Option<u8>,
    pub effort: Option<u8>,
    pub method: Option<String>,
    pub distance: Option<f32>,
    
    // AI metadata
    pub confidence: f32,      // 0.0-1.0
    pub reasoning: String,    // AI推理explanation
    pub format_recommendation: Option<String>,
    
    // 🆕 Phase 34: 模型信息
    pub model_used: Option<String>,          // 实际使用的模型
    pub model_version: Option<String>,       // 模型版本
    pub inference_time_ms: Option<f64>,      // 推理时间（毫秒）
    
    // 🔥 Phase 36: AI智能参数预测 (完整版)
    pub lossless: Option<bool>,              // AI预测：是否使用无损模式
    pub lossless_jpeg: Option<bool>,         // AI预测：JPEG→JXL无损转码
    pub format_options: Option<Vec<(String, String)>>, // AI预测：格式特定选项
    
    // 🆕 Phase 46.14+: AI预处理建议（自动应用）
    pub preprocessing_steps: Option<Vec<PreprocessStepSuggestion>>, // 预处理步骤建议
    pub optimization_path: Option<String>,   // 优化路径说明
}

/// AI推荐的预处理步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessStepSuggestion {
    pub step: String,                    // "resize", "quantize", "sharpen"
    pub params: serde_json::Value,       // 步骤参数（JSON）
    pub reason: String,                  // 推荐原因
}

/// 反馈数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackData {
    // 原始request
    pub request: PredictionRequest,
    
    // AI预测
    pub predicted_params: PredictionResponse,
    
    // 实际结果
    pub actual_quality: Option<f32>,  // SSIM或其他质量指标
    pub actual_size: u64,
    pub actual_time: f64,             // 转换时间（秒）
    
    // 用户反馈（可选）
    pub user_rating: Option<i32>,    // 1-5星
    pub user_comment: Option<String>,
    
    // 🆕 Phase 34: 性能指标
    pub compression_ratio: Option<f32>,  // 压缩比
    pub success: bool,                   // 转换是否成功
    pub error_message: Option<String>,   // 错误信息（如果失败）
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub status: String,
    pub priority: i32,
    pub ab_weight: f64,
}

/// AI客户端
pub struct AIClient {
    config: AIServiceConfig,
    #[cfg(feature = "ai-client")]
    client: reqwest::blocking::Client,
}

impl AIClient {
    /// 创建新的AI客户端
    pub fn new(config: AIServiceConfig) -> Self {
        #[cfg(feature = "ai-client")]
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");
        
        Self { 
            config,
            #[cfg(feature = "ai-client")]
            client,
        }
    }

    /// 使用默认配置创建
    pub fn with_default() -> Self {
        Self::new(AIServiceConfig::default())
    }

    /// 检查AI服务是否可用
    /// 
    /// 🔥 修复：实际测试连接，不仅检查配置
    pub fn is_available(&self) -> bool {
        if !self.config.enabled {
            eprintln!("[DEBUG] is_available: config.enabled = false");
            return false;
        }
        
        #[cfg(feature = "ai-client")]
        {
            // 🔥 实际测试health endpoint
            let health_url = format!("{}/api/v1/health", self.config.base_url);
            eprintln!("[DEBUG] is_available: Testing connection to {}", health_url);
            debug!("🔍 Testing AI service connection: {}", health_url);
            
            match self.client.get(&health_url)
                .timeout(Duration::from_secs(2))  // 短超时用于快速检测
                .send()
            {
                Ok(response) => {
                    let is_ok = response.status().is_success();
                    if is_ok {
                        eprintln!("[DEBUG] is_available: ✅ Service reachable");
                        debug!("✅ AI service is reachable");
                    } else {
                        eprintln!("[DEBUG] is_available: ⚠️ Status: {}", response.status());
                        warn!("⚠️ AI service returned status: {}", response.status());
                    }
                    is_ok
                }
                Err(e) => {
                    eprintln!("[DEBUG] is_available: ❌ Connection failed: {}", e);
                    error!("❌ AI service connection failed: {}", e);
                    error!("   URL: {}", health_url);
                    false
                }
            }
        }
        
        #[cfg(not(feature = "ai-client"))]
        {
            warn!("⚠️ AI client feature not enabled");
            false
        }
    }

    /// 预测转换参数
    pub fn predict(&self, request: &PredictionRequest) -> Result<PredictionResponse> {
        if !self.config.enabled {
            anyhow::bail!("AI service is disabled");
        }

        info!("🤖 AI Prediction: {}→{} ({}x{})", 
            request.input_format, request.target_format, request.width, request.height);
        
        #[cfg(feature = "ai-client")]
        {
            // 🔥 响亮报错：真实调用AI服务，不降级
            match self.http_predict(request) {
                Ok(response) => {
                    info!("✅ AI prediction success (confidence: {:.1}%)", 
                        response.confidence * 100.0);
                    Ok(response)
                }
                Err(e) => {
                    // ✅ 响亮报错，不fallback
                    error!("❌ AI service FAILED: {}", e);
                    error!("   Without AI service, conversion cannot proceed!");
                    error!("   Start Go AI service:");
                    error!("      cd core/go && go run cmd/pixly-ai/main.go --port 50052");
                    anyhow::bail!("🚨 AI service required for conversion! Error: {}", e);
                }
            }
        }
        
        #[cfg(not(feature = "ai-client"))]
        {
            // ✅ Feature未启用时也响亮报错
            error!("❌ AI client feature not enabled!");
            error!("   Rebuild with: cargo build --release --features ai-client");
            anyhow::bail!("🚨 AI client feature not enabled! Cannot perform AI prediction.");
        }
    }
    
    #[cfg(feature = "ai-client")]
    /// HTTP调用AI服务
    fn http_predict(&self, request: &PredictionRequest) -> Result<PredictionResponse> {
        // 🔥 Phase 40.6: 修复endpoint路径 + API格式适配
        let url = format!("{}/api/v1/predict", self.config.base_url);
        
        eprintln!("[DEBUG] http_predict: URL = {}", url);
        eprintln!("[DEBUG] http_predict: Request = {}→{} ({}x{})", 
            request.input_format, request.target_format, request.width, request.height);
        
        info!("🔗 Connecting to AI service: {}", url);
        debug!("📋 Request: {}→{} ({}x{})", 
            request.input_format, request.target_format, request.width, request.height);
        
        // 🔥 Phase 40.31: 适配Go AI服务的请求格式（含UI选项）
        // 🔥 修复：image_path是必需参数，不使用fallback（符合质量宣言）
        let image_path = request.image_path.as_ref()
            .ok_or_else(|| {
                error!("❌ image_path is REQUIRED for AI prediction");
                error!("   Cannot proceed without real image file");
                anyhow::anyhow!("image_path is required but was not provided")
            })?
            .clone();
        
        let adapted_request = serde_json::json!({
            "image_path": image_path,
            "tool": &request.target_format,
            "target_quality": request.preserve_quality.then_some(95).or(Some(85)),
            "optimize_mode": if request.preserve_quality { "quality" } else { "balanced" },
            // 🔥 Phase 40.31: 传递UI AI高级选项给GO Service
            "enable_bayesian": request.enable_bayesian.unwrap_or(true),       // 默认启用
            "enable_ppo": request.enable_ppo.unwrap_or(true),                 // 默认启用
            "enable_smart_quality": request.enable_smart_quality.unwrap_or(true),  // 默认启用
            "enable_auto_optimize": request.enable_auto_optimize.unwrap_or(true),  // 默认启用
            "enable_video_for_anim": request.enable_video_for_anim.unwrap_or(true), // 默认启用
        });
        
        // 重试逻辑：最多3次
        let mut retries = 3;
        let mut last_error = None;
        
        while retries > 0 {
            eprintln!("[DEBUG] Attempt {} of 3", 4 - retries);
            debug!("🔄 Attempt {} of 3...", 4 - retries);
            
            match self.client
                .post(&url)
                .json(&adapted_request)
                .send()
            {
                Ok(response) => {
                    eprintln!("[DEBUG] Response status: {}", response.status());
                    info!("📥 Response status: {}", response.status());
                    if response.status().is_success() {
                        // 🔥 Phase 40.6: 解析Go AI服务的响应格式并适配
                        let go_response: serde_json::Value = response.json()
                            .context("Failed to parse Go AI response")?;
                        
                        if go_response["success"].as_bool().unwrap_or(false) {
                            let params = &go_response["params"];
                            
                            // 🆕 Phase 46.14+: 解析AI预处理建议
                            let preprocessing_steps = go_response["preprocessing_steps"].as_array()
                                .map(|steps| {
                                    steps.iter().filter_map(|step| {
                                        Some(PreprocessStepSuggestion {
                                            step: step["step"].as_str()?.to_string(),
                                            params: step["params"].clone(),
                                            reason: step["reason"].as_str().unwrap_or("").to_string(),
                                        })
                                    }).collect()
                                });
                            
                            return Ok(PredictionResponse {
                                quality: params["quality"].as_i64().map(|v| v as u8),
                                speed: params["speed"].as_i64().map(|v| v as u8),
                                effort: params["effort"].as_i64().map(|v| v as u8),
                                method: params["method"].as_i64().map(|v| format!("{}", v)),
                                distance: params["distance"].as_f64().map(|v| v as f32),
                                confidence: go_response["confidence"].as_f64().unwrap_or(params["confidence"].as_f64().unwrap_or(0.8)) as f32,
                                reasoning: "Go AI prediction".to_string(),
                                format_recommendation: go_response["recommended_format"].as_str().map(|s| s.to_string()),
                                model_used: go_response["model_used"].as_str().map(|s| s.to_string()),
                                model_version: go_response["model_version"].as_str().map(|s| s.to_string()),
                                inference_time_ms: go_response["inference_time_ms"].as_f64(),
                                lossless: Some(false), // Go服务暂不支持lossless预测
                                lossless_jpeg: Some(false),
                                format_options: None,
                                preprocessing_steps,
                                optimization_path: go_response["optimization_path"].as_str().map(|s| s.to_string()),
                            });
                        } else {
                            let error_msg = go_response["error"].as_str().unwrap_or("Unknown error");
                            last_error = Some(anyhow::anyhow!("Go AI service error: {}", error_msg));
                        }
                    } else {
                        let status = response.status();
                        let body = response.text().unwrap_or_default();
                        last_error = Some(anyhow::anyhow!(
                            "AI service HTTP error: {} - {}", status, body
                        ));
                    }
                }
                Err(e) => {
                    error!("❌ HTTP request error: {}", e);
                    error!("   URL: {}", url);
                    error!("   Error type: {:?}", e);
                    last_error = Some(anyhow::anyhow!("HTTP request failed: {}", e));
                }
            }
            
            retries -= 1;
            if retries > 0 {
                debug!("Retrying AI prediction... ({} attempts left)", retries);
                std::thread::sleep(Duration::from_millis(500));
            }
        }
        
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Unknown error")))
    }

    // ❌ DELETED: mock_prediction
    //
    // 【质量宣言执行】
    // - ❌ Mock代码（假装功能正常）
    // - ❌ Fallback代码（让AI成为摆设）
    // - ❌ 硬编码参数（quality: 85/80）
    // - ✅ 响亮报错 > 静默降级
    //
    // AI不可用时，应该直接返回错误，而不是静默降级到hardcode参数。
    // 所有转换必须使用真实的AI预测，不接受Mock/演示数据。

    /// �� Phase 34: 带模型选择的预测
    pub fn predict_with_options(
        &self,
        request: &PredictionRequest,
        model_type: Option<&str>,
        request_id: Option<&str>,
    ) -> Result<PredictionResponse> {
        if !self.config.enabled {
            anyhow::bail!("AI service is disabled");
        }

        let mut req = request.clone();
        req.model_type = model_type.map(|s| s.to_string());
        req.request_id = request_id.map(|s| s.to_string());

        info!(
            "🤖 AI Prediction with options: {}→{} ({}x{}) [model: {:?}, request_id: {:?}]",
            req.input_format,
            req.target_format,
            req.width,
            req.height,
            model_type,
            request_id
        );

        #[cfg(feature = "ai-client")]
        {
            // 🔥 响亮报错：真实调用AI服务，不降级
            match self.http_predict(&req) {
                Ok(response) => {
                    info!(
                        "✅ AI prediction success (confidence: {:.1}%, model: {:?}, time: {:?}ms)",
                        response.confidence * 100.0,
                        response.model_used,
                        response.inference_time_ms
                    );
                    Ok(response)
                }
                Err(e) => {
                    // ✅ 响亮报错，不fallback
                    error!("❌ AI service FAILED: {}", e);
                    error!("   Without AI service, conversion cannot proceed!");
                    error!("   Start Go AI service:");
                    error!("      cd core/go && go run cmd/pixly-ai/main.go --port 50052");
                    anyhow::bail!("🚨 AI service required for conversion! Error: {}", e);
                }
            }
        }

        #[cfg(not(feature = "ai-client"))]
        {
            // ✅ Feature未启用时也响亮报错
            error!("❌ AI client feature not enabled!");
            error!("   Rebuild with: cargo build --release --features ai-client");
            anyhow::bail!("🚨 AI client feature not enabled! Cannot perform AI prediction.");
        }
    }

    /// 🆕 Phase 34: 自动反馈上报
    pub fn auto_feedback(
        &self,
        request: &PredictionRequest,
        predicted: &PredictionResponse,
        actual_result: ConversionResult,
    ) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let feedback = FeedbackData {
            request: request.clone(),
            predicted_params: predicted.clone(),
            actual_quality: actual_result.quality_score,
            actual_size: actual_result.output_size,
            actual_time: actual_result.duration_secs,
            compression_ratio: Some(actual_result.input_size as f32 / actual_result.output_size as f32),
            success: actual_result.success,
            error_message: actual_result.error,
            user_rating: None,
            user_comment: None,
        };

        debug!("📊 Auto-reporting feedback to AI service...");
        self.send_feedback(&feedback)
    }

    /// 发送反馈（增强版）
    // 🔥 Phase 40.21: 发送原始JSON反馈
    pub fn send_raw_feedback(&self, feedback_json: &str) -> Result<()> {
        if !self.config.enabled {
            // AI服务未启用时，只记录日志不返回错误
            debug!("AI service disabled, feedback not sent");
            return Ok(());
        }

        #[cfg(feature = "ai-client")]
        {
            let url = format!("{}/api/v1/feedback/record", self.config.base_url);
            
            match self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(feedback_json.to_string())
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        let status = response.status();
                        anyhow::bail!("Feedback failed: {}", status);
                    }
                }
                Err(e) => {
                    anyhow::bail!("HTTP error: {}", e);
                }
            }
        }

        #[cfg(not(feature = "ai-client"))]
        {
            debug!("AI client feature disabled, feedback not sent");
            Ok(())
        }
    }

    pub fn send_feedback(&self, feedback: &FeedbackData) -> Result<()> {
        if !self.config.enabled {
            anyhow::bail!("AI service is disabled");
        }

        info!("📤 Sending feedback: {} (compression: {:.2}x, quality: {:?})",
            if feedback.success { "SUCCESS" } else { "FAILED" },
            feedback.compression_ratio.unwrap_or(1.0),
            feedback.actual_quality
        );

        #[cfg(feature = "ai-client")]
        {
            let url = format!("{}/api/v1/feedback/record", self.config.base_url);
            
            match self.client
                .post(&url)
                .json(&feedback)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Feedback sent successfully");
                        Ok(())
                    } else {
                        let status = response.status();
                        let body = response.text().unwrap_or_default();
                        anyhow::bail!("Feedback failed: {} - {}", status, body);
                    }
                }
                Err(e) => {
                    anyhow::bail!("HTTP error sending feedback: {}", e);
                }
            }
        }

        #[cfg(not(feature = "ai-client"))]
        {
            warn!("⚠️ AI client feature not enabled, feedback not sent");
            Ok(())
        }
    }

    /// 获取活跃模型列表
    pub fn get_active_models(&self) -> Result<std::collections::HashMap<String, ModelInfo>> {
        if !self.config.enabled {
            anyhow::bail!("AI service is disabled");
        }

        debug!("🔍 Fetching active models from AI service");

        #[cfg(feature = "ai-client")]
        {
            let url = format!("{}/api/v1/models/active", self.config.base_url);
            
            match self.client.get(&url)
                .timeout(self.config.timeout)
                .send()
            {
                Ok(response) if response.status().is_success() => {
                    match response.json::<serde_json::Value>() {
                        Ok(json) => {
                            if let Some(active) = json.get("active") {
                                let models: std::collections::HashMap<String, ModelInfo> = 
                                    serde_json::from_value(active.clone())?;
                                info!("✅ Retrieved {} active models", models.len());
                                return Ok(models);
                            } else {
                                anyhow::bail!("AI service response missing 'active' field");
                            }
                        }
                        Err(e) => {
                            anyhow::bail!("Failed to parse AI service response: {}", e);
                        }
                    }
                }
                Ok(response) => {
                    anyhow::bail!("AI service returned error status: {}", response.status());
                }
                Err(e) => {
                    anyhow::bail!("Failed to connect to AI service: {}", e);
                }
            }
        }

        // ✅ 不再fallback到空map，响亮报错（符合质量宣言）
        #[cfg(not(feature = "ai-client"))]
        anyhow::bail!("AI client feature is not enabled")
    }

    /// 使用指定模型进行预测
    pub fn predict_with_model(
        &self,
        request: &PredictionRequest,
        model_name: &str,
        model_version: Option<&str>,
    ) -> Result<PredictionResponse> {
        if !self.config.enabled {
            anyhow::bail!("AI service is disabled");
        }

        info!("🤖 Predicting with model: {} v{}", 
            model_name, 
            model_version.unwrap_or("latest"));

        #[cfg(feature = "ai-client")]
        {
            let url = format!("{}/api/v1/predict/with-model", self.config.base_url);
            
            let req_body = serde_json::json!({
                "model_name": model_name,
                "model_version": model_version,
                "request": request
            });

            // 重试机制
            let mut retries = 3;
            while retries > 0 {
                match self.client.post(&url)
                    .json(&req_body)
                    .timeout(self.config.timeout)
                    .send()
                {
                    Ok(response) if response.status().is_success() => {
                        match response.json::<PredictionResponse>() {
                            Ok(pred) => {
                                info!("✅ Model prediction success (confidence: {:.1}%)", 
                                    pred.confidence * 100.0);
                                return Ok(pred);
                            }
                            Err(e) => {
                                warn!("⚠️ Failed to parse response: {}", e);
                            }
                        }
                    }
                    Ok(response) => {
                        warn!("⚠️ Model prediction failed: {}", response.status());
                    }
                    Err(e) => {
                        warn!("⚠️ Request failed: {}, retries left: {}", e, retries - 1);
                    }
                }
                
                retries -= 1;
                if retries > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
            
            // ✅ 响亮报错，不fallback
            error!("❌ All retries failed for model prediction");
            error!("   Model: {} v{}", model_name, model_version.unwrap_or("latest"));
            error!("   Without AI service, conversion cannot proceed!");
            anyhow::bail!("🚨 Model prediction failed after all retries");
        }

        #[cfg(not(feature = "ai-client"))]
        {
            // ✅ Feature未启用时也响亮报错
            error!("❌ AI client feature not enabled!");
            anyhow::bail!("🚨 AI client feature required for model prediction");
        }
    }

    /// 预测视频编码参数 (blocking版本)
    /// 
    /// # 参数
    /// - `width`: 视频宽度
    /// - `height`: 视频高度
    /// - `fps`: 帧率
    /// - `codec`: 目标编码器 (h264, h265, av1)
    /// 
    /// # 返回
    /// - `crf`: CRF质量值 (0-51)
    /// - `preset`: 编码预设 (ultrafast, fast, medium, slow, veryslow)
    #[cfg(feature = "ai-client")]
    pub fn predict_video_params(
        &self,
        width: u32,
        height: u32,
        fps: f32,
        codec: &str,
    ) -> Result<VideoParams> {
        let url = format!("{}/api/v1/video/predict", self.config.base_url);
        
        let request_body = serde_json::json!({
            "width": width,
            "height": height,
            "fps": fps,
            "codec": codec
        });
        
        info!("🤖 Requesting video AI prediction for {}x{}@{}fps ({})", 
            width, height, fps, codec);
        
        let response = self.client
            .post(&url)
            .json(&request_body)
            .timeout(self.config.timeout)
            .send()
            .context("Failed to send video prediction request")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            error!("❌ Video AI prediction failed: {} - {}", status, error_text);
            anyhow::bail!("Video AI prediction failed with status {}: {}", status, error_text);
        }
        
        let params: VideoParams = response.json()
            .context("Failed to parse video prediction response")?;
        
        info!("✅ Video AI prediction: CRF={}, preset={}", 
            params.crf, params.preset);
        
        Ok(params)
    }
    
    /// 预测视频编码参数 (fallback版本)
    #[cfg(not(feature = "ai-client"))]
    pub fn predict_video_params(
        &self,
        _width: u32,
        _height: u32,
        _fps: f32,
        _codec: &str,
    ) -> Result<VideoParams> {
        warn!("⚠️ AI client not enabled, returning default video params");
        Ok(VideoParams {
            crf: 23,
            preset: "medium".to_string(),
        })
    }
}

// �� Phase 37激进重构: 强制初始化AI Client
// 
// 使用lazy_static确保总是初始化，启动时检查连接性
lazy_static! {
    static ref AI_CLIENT: std::sync::Mutex<AIClient> = {
        let client = AIClient::with_default();
        
        // 🎯 启动时强制检查AI服务
        if !client.is_available() {
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            error!("❌ AI SERVICE NOT AVAILABLE!");
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            error!("   Expected at: {}", client.config.base_url);
            error!("   Without AI service, all conversions will FAIL!");
            error!("");
            error!("   Start Go AI service NOW:");
            error!("   cd cmd/ai-service && go run main.go --port 50052");
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        } else {
            info!("✅ AI service connected: {}", client.config.base_url);
        }
        
        std::sync::Mutex::new(client)
    };
}

/// 获取全局AI客户端（总是返回，已自动初始化）
pub fn get_ai_client() -> &'static std::sync::Mutex<AIClient> {
    &AI_CLIENT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_client_creation() {
        let client = AIClient::with_default();
        assert_eq!(client.config.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_jpeg_jxl_prediction() {
        let client = AIClient::with_default();
        let request = PredictionRequest {
            image_path: None,
            input_format: "jpeg".to_string(),
            target_format: "jxl".to_string(),
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            has_alpha: false,
            has_animation: false,
            color_space: None,
            bit_depth: None,
            priority: "quality".to_string(),
            preserve_quality: true,
            model_type: None,
            request_id: None,
            enable_bayesian: None,
            enable_ppo: None,
            enable_smart_quality: None,
            enable_auto_optimize: None,
            enable_video_for_anim: None,
        };

        let response = client.predict(&request).unwrap();
        assert_eq!(response.quality, Some(100));
        assert_eq!(response.speed, Some(7));
        assert!(response.confidence > 0.9);
    }

    #[test]
    fn test_default_prediction() {
        let client = AIClient::with_default();
        let request = PredictionRequest {
            image_path: None,
            input_format: "png".to_string(),
            target_format: "avif".to_string(),
            width: 1920,
            height: 1080,
            file_size: 3_000_000,
            has_alpha: false,
            has_animation: false,
            color_space: None,
            bit_depth: None,
            priority: "quality".to_string(),
            preserve_quality: true,
            model_type: None,
            request_id: None,
            enable_bayesian: None,
            enable_ppo: None,
            enable_smart_quality: None,
            enable_auto_optimize: None,
            enable_video_for_anim: None,
        };

        let response = client.predict(&request).unwrap();
        assert_eq!(response.quality, Some(85));
    }
}

// �� Phase 34: 转换结果结构体
#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub success: bool,
    pub input_size: u64,
    pub output_size: u64,
    pub duration_secs: f64,
    pub quality_score: Option<f32>,  // SSIM等
    pub error: Option<String>,
}

/// 视频编码参数
// ========================================================================
// 🎬 Phase 40.42b: 视频AI预测
// ========================================================================
/// 视频预测请求
#[derive(Debug, Clone, Serialize)]
pub struct VideoPredictRequest {
    pub video_path: String,
    pub optimize_mode: String,  // size, balanced, quality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VideoPredictOptions>,
}

/// 视频预测选项
#[derive(Debug, Clone, Serialize)]
pub struct VideoPredictOptions {
    pub use_advanced_ai: bool,
    pub enable_transformer: bool,
    pub enable_vmaf: bool,
    pub allow_hevc: bool,
    pub prefer_speed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_encoder: Option<String>,
}

/// 视频预测响应
#[derive(Debug, Clone, Deserialize)]
pub struct VideoPredictResponse {
    pub success: bool,
    #[serde(default)]
    pub params: Option<VideoParams>,
    #[serde(default)]
    pub confidence: f64,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub error: Option<String>,
}

/// 视频编码参数
#[derive(Debug, Clone, Deserialize)]
pub struct VideoParams {
    pub encoder: String,  // h264, h265, av1, vp9
    pub crf: u8,
    pub preset: String,
    pub fps: Option<u32>,
    pub scale: Option<String>,
}

impl AIClient {
    /// 🎬 视频AI预测
    pub fn video_predict(&self, video_path: &str, optimize_mode: &str) -> Result<VideoPredictResponse> {
        #[cfg(feature = "ai-client")]
        {
            let url = format!("{}/api/v1/predict/video", self.config.base_url);
            
            let request = VideoPredictRequest {
                video_path: video_path.to_string(),
                optimize_mode: optimize_mode.to_string(),
                options: Some(VideoPredictOptions {
                    use_advanced_ai: true,
                    enable_transformer: false,
                    enable_vmaf: false,
                    allow_hevc: true,
                    prefer_speed: optimize_mode == "size",
                    target_encoder: None,
                }),
            };
            
            info!("📡 Calling video AI prediction: {} (mode: {})", video_path, optimize_mode);
            
            let response = self.client
                .post(&url)
                .json(&request)
                .send()
                .with_context(|| format!("Failed to send video prediction request to {}", url))?;
            
            let status = response.status();
            if !status.is_success() {
                bail!("Video prediction API returned status {}", status);
            }
            
            let result: VideoPredictResponse = response.json()
                .context("Failed to parse video prediction response")?;
            
            if !result.success {
                bail!("Video prediction failed: {:?}", result.error);
            }
            
            info!("✅ Video AI prediction succeeded | Confidence: {:.1}%", result.confidence * 100.0);
            
            Ok(result)
        }
        
        #[cfg(not(feature = "ai-client"))]
        {
            bail!("AI client feature not enabled")
        }
    }
}
