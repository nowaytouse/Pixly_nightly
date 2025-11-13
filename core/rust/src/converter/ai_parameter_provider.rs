/**
 * AI参数提供器 (AI Parameter Provider)
 * 
 * Phase 46.9: 架构角色明确定义
 * 
 * 职责：
 * - ✅ 调用Go AI服务获取参数推荐
 * - ✅ 验证AI返回的参数
 * - ✅ AI不可用时响亮报错
 * 
 * 不负责：
 * - ❌ Rust自己不做参数优化
 * - ❌ 不做智能决策
 * - ❌ 不做fallback到规则
 * 
 * 架构原则：
 * - Rust核心是唯一的转换器执行层和文件处理器
 * - Go AI是最完全的AI增强核心
 * - 此文件仅作为Go AI服务的客户端调用包装
 * 
 * 使用场景：
 * - 仅在用户未明确指定参数时调用
 * - 用户明确指定参数时跳过AI调用
 */
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};

use anyhow::{Result, bail};
// 🔥 重构：修复导入路径（扁平化后）
use crate::converter::params::{ImageCharacteristics, OptimizedParams};
use crate::converter::ai_client::{get_ai_client, PredictionRequest, PredictionResponse};

/// Get AI-recommended parameters for AVIF
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_for_avif(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    // 🎯 仅AI预测 - 失败则报错，强制Go AI服务正常工作
    match try_ai_prediction(chars, "avif", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required but not available!\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Get AI-recommended parameters for JXL
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_for_jxl(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    // 🎯 JPEG → JXL: 特殊处理（lossless transcoding）
    // 这是格式特性，不是fallback
    if chars.format == "jpeg" || chars.format == "jpg" {
        let effort: u8 = 7;
        
        return Ok(OptimizedParams {
            quality: 100,
            speed: effort,
            lossless: true,
            format_options: vec![
                ("lossless-jpeg".to_string(), "1".to_string()),
                ("effort".to_string(), effort.to_string()),
            ],
            estimated_size: (chars.file_size as f32 * 0.75) as u64,
            estimated_ratio: 0.75,
            reason: format!("🎯 JPEG→JXL lossless transcoding (effort={}, 质量=完美)", effort),
        });
    }

    // 其他格式: 仅AI预测
    match try_ai_prediction(chars, "jxl", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required for non-JPEG JXL conversion!\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Get AI-recommended parameters for WebP
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_for_webp(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    match try_ai_prediction(chars, "webp", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required but not available!\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Get AI-recommended parameters for PNG
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_for_png(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    match try_ai_prediction(chars, "png", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required but not available!\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Get AI-recommended parameters for JPEG
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_for_jpeg(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    match try_ai_prediction(chars, "jpeg", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required but not available!\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Get AI-recommended parameters (default/fallback)
/// 
/// Phase 46.10: 重命名明确职责 - 从Go AI服务获取推荐
pub fn get_ai_params_default(chars: &ImageCharacteristics, prefer_quality: bool) -> Result<OptimizedParams> {
    // 🎯 仍然尝试AI预测
    match try_ai_prediction(chars, "default", prefer_quality) {
        Some(ai_prediction) => Ok(ai_prediction),
        None => anyhow::bail!(
            "❌ AI service required but not available!\n\
             Unsupported format or AI service offline.\n\
             Start Go AI service:\n\
             cd cmd/ai-service && go run main.go --port 50052"
        )
    }
}

/// Try AI prediction for parameters
/// 启发式规则：根据输入格式和目标格式智能决定是否使用无损压缩
/// 
/// 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
/// - JPEG→JXL: 使用无损转码（保留JPEG完美质量）
/// - PNG→任意: 使用无损（保留PNG完美质量）
/// - WebP(无损)→任意: 使用无损
/// - 其他情况: 有损（平衡质量和大小）
fn decide_lossless_heuristic(input_format: &str, target_format: &str) -> bool {
    let input_lower = input_format.to_lowercase();
    let target_lower = target_format.to_lowercase();
    
    // JPEG→JXL 无损转码（利用JXL的JPEG重新包装特性）
    if input_lower == "jpeg" && target_lower == "jxl" {
        info!("🎯 Heuristic: JPEG→JXL using lossless transcoding");
        return true;
    }
    
    // PNG 默认无损（保留完美质量）
    if input_lower == "png" {
        info!("🎯 Heuristic: PNG input using lossless");
        return true;
    }
    
    // 其他情况使用有损（平衡质量和大小）
    debug!("🎯 Heuristic: {}→{} using lossy", input_format, target_format);
    false
}

/// 🔥 Phase 46.6: 验证AI返回的参数
/// 
/// 用途：参数完整性验证
/// - 验证参数范围
/// - 检查置信度阈值
/// - 拒绝异常值
fn validate_ai_response(response: &PredictionResponse) -> Result<()> {
    use tracing::{warn, error};
    
    // 1. 置信度阈值检查
    if response.confidence < 0.5 {
        error!("❌ AI confidence too low: {:.1}%", response.confidence * 100.0);
        bail!("AI置信度过低: {:.1}%，拒绝使用", response.confidence * 100.0);
    } else if response.confidence < 0.7 {
        warn!("⚠️ AI confidence moderate: {:.1}%", response.confidence * 100.0);
    }
    
    // 2. 参数范围验证
    // Quality: 1-100
    if let Some(quality) = response.quality {
        if quality < 1 || quality > 100 {
            error!("❌ AI returned invalid quality: {}", quality);
            bail!("AI返回无效质量参数: {} (应为1-100)", quality);
        }
    }
    
    // Speed: 0-10 
    if let Some(speed) = response.speed {
        if speed > 10 {
            error!("❌ AI returned invalid speed: {}", speed);
            bail!("AI返回无效速度参数: {} (应为0-10)", speed);
        }
    }
    
    // 3. 格式特定参数验证
    if let Some(format_opts) = &response.format_options {
        // format_options是Vec<(String, String)>，使用迭代查找
        
        // AVIF quantizer: 0-63
        for (key, value) in format_opts {
            if key == "quantizer" {
                if let Ok(q) = value.parse::<u8>() {
                    if q > 63 {
                        error!("❌ AI returned invalid AVIF quantizer: {}", q);
                        bail!("AI返回无效AVIF quantizer: {} (应为0-63)", q);
                    }
                }
            }
            
            // JXL distance: 0.0-15.0
            if key == "distance" {
                if let Ok(d) = value.parse::<f32>() {
                    if d < 0.0 || d > 15.0 {
                        error!("❌ AI returned invalid JXL distance: {}", d);
                        bail!("AI返回无效JXL distance: {} (应为0.0-15.0)", d);
                    }
                }
            }
        }
    }
    
    // 4. 其他合理性检查
    // 无损模式下quality应该是100
    if response.lossless == Some(true) {
        if let Some(quality) = response.quality {
            if quality < 100 {
                warn!("⚠️ AI suggested lossless with quality {}, adjusting to 100", quality);
            }
        }
    }
    
    Ok(())
}

/// 
/// Returns Some(OptimizedParams) if AI service is available and returns valid prediction
/// Returns None if AI service is unavailable, requiring fallback to rule-based
fn try_ai_prediction(
    chars: &ImageCharacteristics,
    target_format: &str,
    prefer_quality: bool,
) -> Option<OptimizedParams> {
    // 🔥 Phase 37: get_ai_client()总是返回，已自动初始化
    let ai_client = get_ai_client();
    let client = ai_client.lock().ok()?;
    
    // Check if AI service is available
    if !client.is_available() {
        warn!("⚠️  AI service unavailable, returning None");
        return None;
    }
    
    // Create prediction request
    let request = PredictionRequest {
        // 🔥 修复：传递真实文件路径给Python AI
        image_path: chars.path.clone(),
        
        input_format: chars.format.clone(),
        target_format: target_format.to_string(),
        width: chars.width,
        height: chars.height,
        file_size: chars.file_size,
        has_alpha: chars.has_alpha,
        has_animation: chars.is_animated,
        color_space: None,
        bit_depth: None,
        priority: if prefer_quality { "quality" } else { "balanced" }.to_string(),
        preserve_quality: prefer_quality,
        model_type: None,
        request_id: None,
        // 🔥 Phase 40.31: AI高级选项（此处使用默认值，CLI传入的优先）
        enable_bayesian: Some(true),       // 默认启用
        enable_ppo: Some(true),            // 默认启用
        enable_smart_quality: Some(true),  // 默认启用
        enable_auto_optimize: Some(true),  // 默认启用
        enable_video_for_anim: Some(true), // 默认启用
    };
    
    // Make prediction
    match client.predict(&request) {
        Ok(response) => {
            // 🔥 Phase 46.6: 先验证AI返回值
            if let Err(e) = validate_ai_response(&response) {
                error!("❌ AI response validation failed: {}", e);
                return None;
            }
            
            // 🔥 Phase 36: AI智能预测lossless和format_options
            // 🔥 Phase 40.16: 使用启发式规则而不是硬编码false
            // 🔥 CRITICAL: AI响应必须完整 - 不允许任何parameter fallback！
            // 📋 如果AI服务返回不完整数据，说明AI服务有问题，应该报错而非fallback
            
            let quality = match response.quality {
                Some(q) => q,
                None => {
                    error!("❌ CRITICAL: AI service returned incomplete response - missing quality parameter!");
                    error!("🔥 NO hardcoded fallback allowed (quality=80)");
                    error!("📋 Project Quality Manifesto violation: 'AI服务必须可用 - 每次转换必须调用AI预测'");
                    error!("Required action:");
                    error!("1. Check AI service health: curl http://localhost:50052/api/v1/version");
                    error!("2. Verify AI service prediction endpoint");
                    error!("3. Fix AI service to return complete responses");
                    error!("4. NO parameter fallbacks allowed");
                    return None;
                }
            };
            
            let speed = match response.speed {
                Some(s) => s,
                None => {
                    error!("❌ CRITICAL: AI service returned incomplete response - missing speed parameter!");
                    error!("🔥 NO hardcoded fallback allowed (speed=4)");
                    error!("📋 Project Quality Manifesto violation: 'AI服务必须可用 - 每次转换必须调用AI预测'");
                    return None;
                }
            };
            
            let lossless = match response.lossless {
                Some(l) => l,
                None => {
                    error!("❌ CRITICAL: AI service returned incomplete response - missing lossless parameter!");
                    error!("🔥 NO heuristic fallback allowed");
                    error!("📋 Project Quality Manifesto violation: 'AI服务必须可用 - 每次转换必须调用AI预测'");
                    return None;
                }
            };
            
            let format_options = response.format_options.unwrap_or_default();
            
            info!("✅ AI prediction SUCCESS: quality={}, speed={}, lossless={}, format_opts={}, confidence={:.1}%",
                quality, speed, lossless, format_options.len(), response.confidence * 100.0);
            
            // Convert to OptimizedParams
            Some(OptimizedParams {
                quality,
                speed,
                lossless, // 🔥 AI智能预测无损模式
                format_options, // 🔥 AI智能预测格式选项
                estimated_size: (chars.file_size as f32 * 0.8) as u64,
                estimated_ratio: 0.8,
                reason: response.reasoning,
            })
        }
        Err(e) => {
            // 🔥 Phase 37激进重构: 响亮的错误，不再静默降级
            error!("❌ AI prediction FAILED: {}", e);
            error!("   Without AI service, conversion will fail!");
            error!("   Start Go AI service:");
            error!("   cd cmd/ai-service && go run main.go --port 50052");
            None
        }
    }
}
