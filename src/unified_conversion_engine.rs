// 🚀 统一转换引擎
// 集成PPO模型、现代格式、质量评估的完整转换流程

use anyhow::{Context, Result};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::ppo_model_enhanced::{EnhancedPPOPredictor, MediaType};
use crate::modern_formats::{ModernFormatConverter, AVIFParams, JXLParams, FormatSupport};
use crate::quality_metrics::{QualityAssessor, QualityMetrics};
use crate::transparent_logger::{TransparentLogger, OperationTracker, LogLevel};

/// 统一转换配置
#[derive(Debug, Clone)]
pub struct UnifiedConversionConfig {
    pub use_ppo: bool,
    pub use_quality_assessment: bool,
    pub target_quality_threshold: f64,  // 最低可接受质量 (0-100)
    pub max_retries: u32,
    pub ppo_model_path: Option<PathBuf>,
}

impl Default for UnifiedConversionConfig {
    fn default() -> Self {
        Self {
            use_ppo: true,
            use_quality_assessment: true,
            target_quality_threshold: 70.0,
            max_retries: 3,
            ppo_model_path: Some(PathBuf::from("models/ppo_training_all_media_20251117_003038.json")),
        }
    }
}

/// 转换请求
#[derive(Debug, Clone)]
pub struct ConversionRequest {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub target_format: String,
    pub media_type: MediaType,
    pub quality: Option<u8>,  // None = 自动
}

/// 转换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub quality_metrics: Option<QualityMetrics>,
    pub used_ppo: bool,
    pub retries: u32,
    pub error: Option<String>,
}

/// 统一转换引擎
pub struct UnifiedConversionEngine {
    config: UnifiedConversionConfig,
    ppo_predictor: Option<EnhancedPPOPredictor>,
    format_converter: ModernFormatConverter,
    quality_assessor: QualityAssessor,
    format_support: FormatSupport,
    logger: TransparentLogger,
}

impl UnifiedConversionEngine {
    /// 创建新引擎
    pub fn new(config: UnifiedConversionConfig) -> Result<Self> {
        let logger = TransparentLogger::new();
        
        logger.log_header("Unified Conversion Engine Initialization");
        
        // 加载PPO模型
        logger.log(LogLevel::Info, "🤖 Loading PPO model...");
        let ppo_predictor = if config.use_ppo {
            if let Some(ref path) = config.ppo_model_path {
                if path.exists() {
                    logger.log_with_details(
                        LogLevel::Detail,
                        "PPO model file found",
                        &[("Path", path.display().to_string())]
                    );
                    
                    match EnhancedPPOPredictor::from_file(path) {
                        Ok(predictor) => {
                            logger.log(LogLevel::Info, "✅ PPO model loaded successfully");
                            logger.log_with_details(
                                LogLevel::Detail,
                                "Model info",
                                &[("Stats", predictor.get_training_stats())]
                            );
                            Some(predictor)
                        }
                        Err(e) => {
                            logger.log(LogLevel::Error, &format!("❌ Failed to load PPO model: {}", e));
                            None
                        }
                    }
                } else {
                    logger.log(LogLevel::Warning, &format!("⚠️  PPO model file not found: {:?}", path));
                    None
                }
            } else {
                logger.log(LogLevel::Warning, "⚠️  PPO model path not specified");
                None
            }
        } else {
            logger.log(LogLevel::Info, "ℹ️  PPO model disabled");
            None
        };
        
        // 初始化格式转换器
        logger.log(LogLevel::Info, "🎨 Initializing format converter...");
        let format_converter = ModernFormatConverter::new();
        let format_support = format_converter.check_format_support();
        
        logger.log_with_details(
            LogLevel::Info,
            "Format support detection completed",
            &[
                ("AVIF", if format_support.avif { "✅ Supported" } else { "❌ Not supported" }.to_string()),
                ("JXL (FFmpeg)", if format_support.jxl_ffmpeg { "✅ Supported" } else { "❌ Not supported" }.to_string()),
                ("JXL (Native)", if format_support.jxl_native { "✅ Supported" } else { "❌ Not supported" }.to_string()),
                ("WebP", if format_support.webp { "✅ Supported" } else { "❌ Not supported" }.to_string()),
            ]
        );
        
        // 初始化质量评估器
        logger.log(LogLevel::Info, "📊 Initializing quality assessor...");
        let quality_assessor = QualityAssessor::new();
        
        if quality_assessor.has_vmaf_support() {
            logger.log(LogLevel::Info, "✅ VMAF support enabled");
        } else {
            logger.log(LogLevel::Warning, "⚠️  VMAF not available, will use SSIM/PSNR");
        }
        
        logger.log_separator();
        logger.log(LogLevel::Info, "🚀 Engine initialization completed");
        
        Ok(Self {
            config,
            ppo_predictor,
            format_converter,
            quality_assessor,
            format_support,
            logger,
        })
    }
    
    /// 执行转换
    pub fn convert(&self, request: ConversionRequest) -> Result<ConversionResult> {
        let tracker = OperationTracker::start(
            self.logger.clone(),
            &format!("Convert {} -> {}", 
                request.input_path.display(), 
                request.target_format
            )
        );
        
        let input_size = std::fs::metadata(&request.input_path)?.len();
        
        tracker.log_details(&[
            ("Input file", request.input_path.display().to_string()),
            ("Output file", request.output_path.display().to_string()),
            ("Target format", request.target_format.clone()),
            ("Media type", format!("{:?}", request.media_type)),
            ("File size", format!("{} bytes ({:.2} MB)", input_size, input_size as f64 / 1_048_576.0)),
            ("Use PPO", if self.ppo_predictor.is_some() { "Yes" } else { "No" }.to_string()),
            ("Quality assessment", if self.config.use_quality_assessment { "Enabled" } else { "Disabled" }.to_string()),
        ]);
        
        // 检查格式支持
        if !self.format_support.supports(&request.target_format) {
            tracker.log_error(&format!("Unsupported format: {}", request.target_format));
            return Ok(ConversionResult {
                success: false,
                input_size,
                output_size: 0,
                compression_ratio: 0.0,
                quality_metrics: None,
                used_ppo: false,
                retries: 0,
                error: Some(format!("Unsupported format: {}", request.target_format)),
            });
        }
        
        tracker.log_step("Format support check passed");
        
        let mut retries = 0;
        let mut last_error = None;
        
        // 尝试转换，如果质量不达标则重试
        while retries < self.config.max_retries {
            match self.try_convert(&request, retries) {
                Ok(result) => {
                    // 检查质量
                    if self.config.use_quality_assessment {
                        if let Some(ref metrics) = result.quality_metrics {
                            if metrics.overall_score >= self.config.target_quality_threshold {
                                return Ok(result);
                            } else {
                                println!("⚠️  Quality below threshold ({:.1}/100), retry {}/{}",
                                    metrics.overall_score,
                                    retries + 1,
                                    self.config.max_retries
                                );
                                retries += 1;
                                continue;
                            }
                        }
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    retries += 1;
                }
            }
        }
        
        Ok(ConversionResult {
            success: false,
            input_size,
            output_size: 0,
            compression_ratio: 0.0,
            quality_metrics: None,
            used_ppo: self.ppo_predictor.is_some(),
            retries,
            error: last_error,
        })
    }
    
    /// 尝试单次转换
    fn try_convert(&self, request: &ConversionRequest, retry: u32) -> Result<ConversionResult> {
        let input_size = std::fs::metadata(&request.input_path)?.len();
        
        // 确定质量参数
        let quality = if let Some(q) = request.quality {
            q
        } else if let Some(ref ppo) = self.ppo_predictor {
            // 使用PPO预测
            self.predict_quality_with_ppo(ppo, request, retry)
        } else {
            // 默认质量
            85
        };
        
        // 执行转换
        let _conversion_result = match request.media_type {
            MediaType::Image => self.convert_image(request, quality)?,
            MediaType::Video => self.convert_video(request, quality)?,
            MediaType::Audio => self.convert_audio(request, quality)?,
        };
        
        let output_size = std::fs::metadata(&request.output_path)?.len();
        let compression_ratio = output_size as f64 / input_size as f64;
        
        // 质量评估
        let quality_metrics = if self.config.use_quality_assessment {
            Some(self.assess_quality(request)?)
        } else {
            None
        };
        
        Ok(ConversionResult {
            success: true,
            input_size,
            output_size,
            compression_ratio,
            quality_metrics,
            used_ppo: self.ppo_predictor.is_some(),
            retries: retry,
            error: None,
        })
    }
    
    /// 使用PPO预测质量参数
    fn predict_quality_with_ppo(
        &self,
        ppo: &EnhancedPPOPredictor,
        request: &ConversionRequest,
        retry: u32,
    ) -> u8 {
        let input_size = std::fs::metadata(&request.input_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        let base_quality = match request.media_type {
            MediaType::Image => {
                let params = ppo.predict_image_params(&request.target_format, input_size);
                params.quality
            }
            MediaType::Video => {
                let params = ppo.predict_video_params(&request.target_format, input_size);
                // 视频使用比特率，这里转换为质量值
                ((params.bitrate as f64 / 2000.0) * 100.0).min(100.0) as u8
            }
            MediaType::Audio => {
                let params = ppo.predict_audio_params(&request.target_format, input_size);
                // 音频使用比特率，这里转换为质量值
                ((params.bitrate as f64 / 320.0) * 100.0).min(100.0) as u8
            }
        };
        
        // 重试时提高质量
        let adjusted_quality = base_quality + (retry as u8 * 5);
        adjusted_quality.min(100)
    }
    
    /// 转换图像
    fn convert_image(&self, request: &ConversionRequest, quality: u8) -> Result<()> {
        match request.target_format.as_str() {
            "avif" => {
                let params = AVIFParams::from_quality(quality);
                self.format_converter.convert_to_avif(
                    &request.input_path,
                    &request.output_path,
                    &params,
                )?;
            }
            "jxl" => {
                let params = JXLParams::from_quality(quality);
                self.format_converter.convert_to_jxl(
                    &request.input_path,
                    &request.output_path,
                    &params,
                )?;
            }
            "webp" => {
                // 使用FFmpeg转换WebP
                self.convert_webp_ffmpeg(request, quality)?;
            }
            _ => {
                anyhow::bail!("Unsupported image format: {}", request.target_format);
            }
        }
        Ok(())
    }
    
    /// 使用FFmpeg转换WebP
    fn convert_webp_ffmpeg(&self, request: &ConversionRequest, quality: u8) -> Result<()> {
        use std::process::Command;
        
        let output = Command::new("ffmpeg")
            .args(&[
                "-i", request.input_path.to_str().unwrap(),
                "-c:v", "libwebp",
                "-quality", &quality.to_string(),
                "-y",
                request.output_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !output.status.success() {
            anyhow::bail!("WebP conversion failed");
        }
        
        Ok(())
    }
    
    /// 转换视频
    fn convert_video(&self, request: &ConversionRequest, _quality: u8) -> Result<()> {
        use std::process::Command;
        
        // 使用PPO预测的参数
        let params = if let Some(ref ppo) = self.ppo_predictor {
            ppo.predict_video_params(&request.target_format, 0)
        } else {
            Default::default()
        };
        
        let (video_codec, audio_codec) = match request.target_format.as_str() {
            "webm" => ("libvpx-vp9", "libopus"),
            "mp4" => ("libx264", "aac"),
            _ => anyhow::bail!("Unsupported video format: {}", request.target_format),
        };
        
        let output = Command::new("ffmpeg")
            .args(&[
                "-i", request.input_path.to_str().unwrap(),
                "-c:v", video_codec,
                "-b:v", &format!("{}k", params.bitrate),
                "-c:a", audio_codec,
                "-b:a", "128k",
                "-y",
                request.output_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !output.status.success() {
            anyhow::bail!("Video conversion failed");
        }
        
        Ok(())
    }
    
    /// 转换音频
    fn convert_audio(&self, request: &ConversionRequest, _quality: u8) -> Result<()> {
        use std::process::Command;
        
        // 使用PPO预测的参数
        let params = if let Some(ref ppo) = self.ppo_predictor {
            ppo.predict_audio_params(&request.target_format, 0)
        } else {
            Default::default()
        };
        
        let codec = match request.target_format.as_str() {
            "opus" => "libopus",
            "aac" => "aac",
            "mp3" => "libmp3lame",
            _ => anyhow::bail!("Unsupported audio format: {}", request.target_format),
        };
        
        let output = Command::new("ffmpeg")
            .args(&[
                "-i", request.input_path.to_str().unwrap(),
                "-c:a", codec,
                "-b:a", &format!("{}k", params.bitrate),
                "-y",
                request.output_path.to_str().unwrap(),
            ])
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !output.status.success() {
            anyhow::bail!("Audio conversion failed");
        }
        
        Ok(())
    }
    
    /// 评估质量
    fn assess_quality(&self, request: &ConversionRequest) -> Result<QualityMetrics> {
        match request.media_type {
            MediaType::Image => {
                self.quality_assessor.assess_image_quality(
                    &request.input_path,
                    &request.output_path,
                )
            }
            MediaType::Video => {
                self.quality_assessor.assess_video_quality(
                    &request.input_path,
                    &request.output_path,
                )
            }
            MediaType::Audio => {
                self.quality_assessor.assess_audio_quality(
                    &request.input_path,
                    &request.output_path,
                )
            }
        }
    }
    
    /// 推荐最佳格式
    pub fn recommend_format(&self, media_type: MediaType) -> String {
        if let Some(ref ppo) = self.ppo_predictor {
            ppo.recommend_best_format(media_type)
        } else {
            match media_type {
                MediaType::Image => "webp".to_string(),
                MediaType::Video => "webm".to_string(),
                MediaType::Audio => "aac".to_string(),
            }
        }
    }
    
    /// 获取引擎信息
    pub fn get_info(&self) -> String {
        let mut info = String::new();
        
        info.push_str("🚀 Unified Conversion Engine\n");
        info.push_str(&format!("PPO Model: {}\n", 
            if self.ppo_predictor.is_some() { "Loaded" } else { "Not loaded" }
        ));
        info.push_str(&format!("Quality Assessment: {}\n", 
            if self.config.use_quality_assessment { "Enabled" } else { "Disabled" }
        ));
        info.push_str(&format!("Supported Formats: {}\n", 
            self.format_support.supported_formats().join(", ")
        ));
        
        if let Some(ref ppo) = self.ppo_predictor {
            info.push_str(&format!("\n{}\n", ppo.get_training_stats()));
        }
        
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_creation() {
        let config = UnifiedConversionConfig::default();
        let engine = UnifiedConversionEngine::new(config);
        
        // 即使PPO模型不存在，引擎也应该能创建
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_format_recommendation() {
        let config = UnifiedConversionConfig {
            use_ppo: false,
            ..Default::default()
        };
        
        let engine = UnifiedConversionEngine::new(config).unwrap();
        
        let image_format = engine.recommend_format(MediaType::Image);
        let video_format = engine.recommend_format(MediaType::Video);
        let audio_format = engine.recommend_format(MediaType::Audio);
        
        assert_eq!(image_format, "webp");
        assert_eq!(video_format, "webm");
        assert_eq!(audio_format, "aac");
    }
}
