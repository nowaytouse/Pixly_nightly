/**
 * HTTP 处理器
 */
use actix_web::{web, HttpResponse, Responder};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::path::Path;
// 🔧 统一日志系统
use tracing::{info, warn, error};

use crate::converter::strategy::{ConversionConfig, StrategyType};
use crate::converter::validation::{FileValidator, ValidationLevel};
use crate::converter::metadata::MetadataHandler;
use crate::server::models::{ConvertRequest, ConvertResponse, HealthResponse, ActualParams};

/// 健康检查处理器
/// GET /health
pub async fn health_handler() -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 获取全局策略管理器
    let available_strategies = if let Some(manager_lock) = crate::converter::strategy::get_global_manager() {
        match manager_lock.lock() {
            Ok(manager) => Some(manager.available_strategies().iter().map(|s| s.to_string()).collect()),
            Err(_) => None,
        }
    } else {
        None
    };
    
    let supported_formats = if let Some(manager_lock) = crate::converter::strategy::get_global_manager() {
        match manager_lock.lock() {
            Ok(manager) => Some(manager.supported_formats()),
            Err(_) => None,
        }
    } else {
        None
    };
    
    let response = HealthResponse {
        status: "ok".to_string(),
        rust_available: true,
        timestamp,
        available_strategies,
        supported_formats,
    };
    
    HttpResponse::Ok().json(response)
}

/// 转换处理器
/// POST /api/rust/convert
pub async fn convert_handler(req: web::Json<ConvertRequest>) -> impl Responder {
    let start = Instant::now();
    
    info!("🔄 Conversion request: {:?} -> {:?} (format: {})", 
               req.input, req.output, req.format);
    
    // ═══════════════════════════════════════════════════════════
    // Phase 1: 输入验证 (多级验证系统)
    // ═══════════════════════════════════════════════════════════
    let mut validator = FileValidator::new();
    validator.set_level(ValidationLevel::Security); // Level 5 (默认不开启防作弊)
    
    match validator.validate_input(&req.input) {
        Ok(validation_result) => {
            if !validation_result.passed {
                let response = ConvertResponse {
                    success: false,
                    error: Some(format!("Input validation failed: {:?}", validation_result.errors)),
                    output_path: req.output.clone(),
                    file_size: 0,
                    duration: start.elapsed().as_millis() as u64,
                    strategy: None,
                    compression_ratio: None,
                    actual_params: None,
                };
                return HttpResponse::BadRequest().json(response);
            }
            
            // 记录验证警告
            if !validation_result.warnings.is_empty() {
                warn!("⚠️  Validation warnings: {:?}", validation_result.warnings);
            }
            
            info!("✅ Input validation passed: {} bytes, {:?}", 
                       validation_result.file_size,
                       validation_result.detected_format);
        }
        Err(e) => {
            let response = ConvertResponse {
                success: false,
                error: Some(format!("Validation error: {}", e)),
                output_path: req.output.clone(),
                file_size: 0,
                duration: start.elapsed().as_millis() as u64,
                strategy: None,
                compression_ratio: None,
                actual_params: None,
            };
            return HttpResponse::BadRequest().json(response);
        }
    }
    
    // 创建转换配置
    let config = ConversionConfig {
        quality: req.quality,
        speed: req.speed,
        preserve_metadata: req.preserve_metadata,
        keep_animated: req.keep_animated,
        strategy: StrategyType::Auto, // 自动选择策略
        lossless: req.lossless,  // 🔥 Phase 39: lossless参数
        normalize_filenames: None,  // 🔥 Phase 40.9: HTTP服务器默认不规范化文件名
        prediction_data: None,  // 🔥 Phase 40.21: HTTP服务器暂不使用AI预测
    };
    
    // 获取全局策略管理器
    let manager_lock = match crate::converter::strategy::get_global_manager() {
        Some(lock) => lock,
        None => {
            let response = ConvertResponse {
                success: false,
                error: Some("Strategy manager not initialized".to_string()),
                output_path: req.output.clone(),
                file_size: 0,
                duration: 0,
                strategy: None,
                compression_ratio: None,
                actual_params: None,
            };
            return HttpResponse::InternalServerError().json(response);
        }
    };
    
    let manager = match manager_lock.lock() {
        Ok(m) => m,
        Err(e) => {
            let response = ConvertResponse {
                success: false,
                error: Some(format!("Failed to lock strategy manager: {}", e)),
                output_path: req.output.clone(),
                file_size: 0,
                duration: 0,
                strategy: None,
                compression_ratio: None,
                actual_params: None,
            };
            return HttpResponse::InternalServerError().json(response);
        }
    };
    
    // 执行转换
    let input_path = Path::new(&req.input);
    let output_path = Path::new(&req.output);
    
    match manager.convert(input_path, output_path, &req.format, &config) {
        Ok(result) => {
            let duration = start.elapsed().as_millis() as u64;
            
            info!(
                "✅ Conversion successful: {} -> {} ({} bytes, {}ms, strategy: {})",
                req.input,
                req.output,
                result.output_size,
                duration,
                result.strategy_used
            );
            
            // ═══════════════════════════════════════════════════════════
            // Phase 2: 元数据处理 (如果启用)
            // ═══════════════════════════════════════════════════════════
            if req.preserve_metadata {
                let metadata_handler = MetadataHandler::new();
                
                // 复制EXIF/XMP/ICC元数据
                if let Err(e) = metadata_handler.copy_metadata(&req.input, &req.output) {
                    warn!("⚠️  Failed to copy metadata: {}", e);
                    // 不要因为元数据失败而整体失败
                }
                
                // 处理XMP sidecar文件
                if let Err(e) = metadata_handler.process_xmp_sidecar(&req.input, &req.output) {
                    warn!("⚠️  Failed to process XMP sidecar: {}", e);
                }
                
                // 保留时间戳
                if let Err(e) = metadata_handler.preserve_timestamps(&req.input, &req.output) {
                    warn!("⚠️  Failed to preserve timestamps: {}", e);
                }
                
                info!("📋 Metadata preservation completed");
            }
            
            // ═══════════════════════════════════════════════════════════
            // Phase 3: 输出验证
            // ═══════════════════════════════════════════════════════════
            match validator.validate_output(&req.output) {
                Ok(output_validation) => {
                    if !output_validation.passed {
                        error!("❌ Output validation failed: {:?}", output_validation.errors);
                        // 输出验证失败，删除输出文件
                        let _ = std::fs::remove_file(&req.output);
                        
                        let response = ConvertResponse {
                            success: false,
                            error: Some(format!("Output validation failed: {:?}", output_validation.errors)),
                            output_path: req.output.clone(),
                            file_size: 0,
                            duration,
                            strategy: Some(result.strategy_used),
                            compression_ratio: None,
                            actual_params: None,
                        };
                        return HttpResponse::InternalServerError().json(response);
                    }
                    
                    info!("✅ Output validation passed: {} bytes", output_validation.file_size);
                }
                Err(e) => {
                    warn!("⚠️  Output validation error (non-fatal): {}", e);
                }
            }
            
            // 🔥 Phase 46.8: 参数来源透明化
            // 优先使用UI明确标记的params_source，否则根据参数变化推断
            let params_source = req.params_source.clone().unwrap_or_else(|| {
                if req.quality == config.quality && req.speed == config.speed {
                    "user".to_string()  // 用户手动参数未被修改
                } else {
                    "unknown".to_string()  // 参数被修改但来源未知
                }
            });
            
            // 🔥 AI置信度验证（如果是AI推荐参数）
            if params_source == "ai" {
                if let Some(confidence) = req.ai_confidence {
                    if confidence < 0.5 {
                        warn!("⚠️  AI confidence low: {} (threshold: 0.5)", confidence);
                    } else if confidence < 0.7 {
                        info!("ℹ️  AI confidence moderate: {} (threshold: 0.7)", confidence);
                    } else {
                        info!("✅ AI confidence high: {}", confidence);
                    }
                }
            }
            
            let actual_params = ActualParams {
                quality: config.quality,
                speed: config.speed,
                lossless: config.lossless,
                format: req.format.clone(),
                preserve_metadata: req.preserve_metadata,
                keep_animated: req.keep_animated,
                strategy_type: result.strategy_used.clone(),
                params_source,
                ai_confidence: req.ai_confidence,  // 🔥 透传AI置信度
            };
            
            let response = ConvertResponse {
                success: true,
                error: None,
                output_path: result.output_path,
                file_size: result.output_size,
                duration,
                strategy: Some(result.strategy_used),
                compression_ratio: Some(result.compression_ratio),
                actual_params: Some(actual_params),  // 🔥 参数回显
            };
            
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let duration = start.elapsed().as_millis() as u64;
            
            error!("❌ Conversion failed: {}", e);
            
            let response = ConvertResponse {
                success: false,
                error: Some(e.to_string()),
                output_path: req.output.clone(),
                file_size: 0,
                duration,
                strategy: None,
                compression_ratio: None,
                actual_params: None,  // 转换失败，不返回参数
            };
            
            HttpResponse::InternalServerError().json(response)
        }
    }
}
