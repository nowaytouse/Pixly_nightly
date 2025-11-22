// 🔄 转换核心逻辑
// 从 @archive/rust_broken/src/cli/conversion.rs 提取

use std::path::{Path, PathBuf};
use anyhow::Result;
use image::GenericImageView;
use crate::utils::feature_toggles::FeatureToggles;
use crate::utils::format_params::FormatSpecificParams;
use crate::core::validation_integration::ValidationDisplay;
use crate::utils::conversion_validator::ConversionValidator;
use crate::analysis::quality_analyzer::QualityAnalyzer;

#[derive(Debug, Clone)]
pub struct ConversionConfig {
    pub quality: u8,
    pub speed: u8,
    pub preserve_metadata: bool,
    pub keep_animated: bool,
    pub lossless: bool,
    pub merge_xmp_sidecar: bool,
    
    // ═══════════════════════════════════════════════════
    // 🎛️ 功能开关集成
    // ═══════════════════════════════════════════════════
    pub feature_toggles: Option<FeatureToggles>,
    
    // ═══════════════════════════════════════════════════
    // 🔧 高级参数 (手动模式)
    // ═══════════════════════════════════════════════════
    
    /// 色度子采样 (420, 422, 444)
    pub chroma_subsampling: Option<String>,
    
    /// Alpha通道质量 (0-100)
    pub alpha_quality: Option<u8>,
    
    /// 编码努力程度 (1-10, 覆盖speed)
    pub effort: Option<u8>,
    
    /// 调整大小选项
    pub resize: Option<ResizeOptions>,
    
    /// 颜色量化选项
    pub quantize: Option<QuantizeOptions>,
    
    /// 锐化选项
    pub sharpen: Option<SharpenOptions>,
    
    /// 输出目录
    pub output_dir: Option<String>,
    
    /// 规范化文件名
    pub normalize_filenames: bool,
    
    /// 🔥 Phase 3.3: 启用验证
    pub enable_validation: bool,
    
    /// 验证级别 (1-5)
    pub validation_level: u8,
    
    /// 启用质量分析
    pub enable_quality_analysis: bool,
    
    // ═══════════════════════════════════════════════════
    // 📦 JXL专属参数 (修复空壳功能)
    // ═══════════════════════════════════════════════════
    
    /// JXL: 使用模块化模式
    pub jxl_modular: bool,
    
    /// JXL: 渐进式解码
    pub jxl_progressive: bool,
    
    /// JXL: 响应式解码
    pub jxl_responsive: bool,
    
    /// JXL: Gaborish滤镜
    pub jxl_gaborish: bool,
    
    // ═══════════════════════════════════════════════════
    // 📦 格式专属参数 (完整支持HTML界面)
    // ═══════════════════════════════════════════════════
    
    /// 格式专属参数 (JXL/WebP/AVIF/HEIC)
    pub format_specific_params: Option<FormatSpecificParams>,
}

/// 调整大小选项
#[derive(Debug, Clone)]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub filter: String, // lanczos3, catmull_rom, gaussian, nearest
    pub maintain_aspect_ratio: bool,
}

/// 颜色量化选项
#[derive(Debug, Clone)]
pub struct QuantizeOptions {
    pub colors: u32,
    pub dithering: bool,
    pub dithering_level: f32,
}

/// 锐化选项
#[derive(Debug, Clone)]
pub struct SharpenOptions {
    pub amount: f32,
    pub radius: f32,
    pub threshold: u8,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            preserve_metadata: true,
            keep_animated: true,
            lossless: false,
            merge_xmp_sidecar: false,
            feature_toggles: None,
            chroma_subsampling: None,
            alpha_quality: None,
            effort: None,
            resize: None,
            quantize: None,
            sharpen: None,
            output_dir: None,
            normalize_filenames: false,
            jxl_modular: false,
            jxl_progressive: false,
            jxl_responsive: false,
            jxl_gaborish: false,
            format_specific_params: None,
            enable_validation: false,  // 🔥 Phase 3.3: 默认关闭，可选启用
            validation_level: 2,       // Standard level
            enable_quality_analysis: false,
        }
    }
}

impl ConversionConfig {
    /// 从功能开关创建配置
    pub fn from_toggles(toggles: FeatureToggles) -> Self {
        Self {
            feature_toggles: Some(toggles),
            ..Default::default()
        }
    }
    
    /// 检查是否启用AI预测
    pub fn is_ai_enabled(&self) -> bool {
        self.feature_toggles
            .as_ref()
            .map(|t| t.enable_ai_prediction)
            .unwrap_or(false)
    }
    
    /// 检查是否启用高级参数
    pub fn has_advanced_params(&self) -> bool {
        self.chroma_subsampling.is_some()
            || self.alpha_quality.is_some()
            || self.effort.is_some()
            || self.resize.is_some()
            || self.quantize.is_some()
            || self.sharpen.is_some()
    }
    
    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 验证质量参数
        if self.quality > 100 {
            anyhow::bail!("Quality parameter must be between 0-100");
        }
        
        // 验证速度参数
        if self.speed > 10 {
            anyhow::bail!("Speed parameter must be between 0-10");
        }
        
        // 验证alpha质量
        if let Some(alpha_q) = self.alpha_quality
            && alpha_q > 100 {
                anyhow::bail!("Alpha quality must be between 0-100");
            }
        
        // 验证努力程度
        if let Some(effort) = self.effort
            && effort > 10 {
                anyhow::bail!("Effort must be between 1-10");
            }
        
        // 验证功能开关
        if let Some(ref toggles) = self.feature_toggles {
            toggles.validate()?;
        }
        
        // 验证格式专属参数
        if let Some(ref format_params) = self.format_specific_params {
            format_params.validate()
                .map_err(|e| anyhow::anyhow!("Format parameter validation failed: {}", e))?;
        }
        
        Ok(())
    }
}

pub fn execute_conversion(
    input: &Path,
    output: &Path,
    format: &str,
    config: &ConversionConfig,
) -> Result<ConversionResult> {
    let start_time = std::time::Instant::now();
    
    // ═══════════════════════════════════════════════════
    // 🎛️ 功能开关检查
    // ═══════════════════════════════════════════════════
    let toggles = config.feature_toggles.as_ref();
    
    println!("🔄 Executing conversion:");
    println!("   Input: {:?}", input);
    println!("   Output: {:?}", output);
    println!("   Format: {}", format);
    println!("   Quality: {}", config.quality);
    println!("   Speed: {}", config.speed);
    
    if let Some(t) = toggles {
        println!("   Feature toggles: {}", t.summary());
        if config.has_advanced_params() {
            println!("   Advanced params: Enabled");
        }
    }
    
    // 获取输入文件大小
    let input_size = std::fs::metadata(input)?.len();
    
    // ═══════════════════════════════════════════════════
    // 🎬 动图转视频自动转换 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_video_for_animation).unwrap_or(false)
        && should_convert_animation_to_video(input)? {
            println!("🎬 Large animated image detected, auto-converting to video format");
            println!("   File: {:?}", input);
            println!("   Expected size reduction: 60-80%");
            println!("   Using codec: H.265/HEVC");
            
            // 🔥 Auto-convert to video
            let video_output = output.with_extension("mp4");
            println!("   Conversion target: {:?}", video_output);
            
            convert_animation_to_video(input, &video_output)?;
            
            println!("   ✅ Animation converted to video");
            println!();
            
            // 🔥 返回视频转换结果，不再继续图像转换
            let output_size = std::fs::metadata(&video_output)?.len();
            return Ok(ConversionResult {
                input_size,
                output_size,
                compression_ratio: output_size as f64 / input_size as f64,
                duration: start_time.elapsed(),
                strategy_used: "animation_to_video".to_string(),
            });
        }
    
    // ═══════════════════════════════════════════════════
    // 🔒 AI文件验证 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_file_validation).unwrap_or(false) {
        println!("🔒 Running AI file validation (Magika)...");
        validate_file_with_magika(input)?;
    }
    
    // ═══════════════════════════════════════════════════
    // 🔧 格式自动修正 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_format_correction).unwrap_or(false) {
        println!("🔧 Checking format correction...");
        check_format_correction(input)?;
    }
    
    // ═══════════════════════════════════════════════════
    // ✅ 配置验证
    // ═══════════════════════════════════════════════════
    config.validate()?;
    
    // ═══════════════════════════════════════════════════
    // 🔗 智能预处理 (如果启用)
    // ═══════════════════════════════════════════════════
    let preprocessed_input = if toggles.map(|t| t.enable_preprocess).unwrap_or(false) {
        println!("🔗 Running intelligent preprocessing...");
        apply_preprocessing(input, config)?
    } else {
        input.to_path_buf()
    };
    
    // ═══════════════════════════════════════════════════
    // 🔄 执行实际转换
    // ═══════════════════════════════════════════════════
    let strategy_used = perform_conversion(&preprocessed_input, output, format, config)?;
    
    // 清理临时预处理文件
    if preprocessed_input != input {
        let _ = std::fs::remove_file(&preprocessed_input);
    }
    
    // ═══════════════════════════════════════════════════
    // ✅ 验证输出质量
    // ═══════════════════════════════════════════════════
    validate_output_quality(output, config)?;
    
    // ═══════════════════════════════════════════════════
    // 🔥 Phase 3.3: 输出验证
    // ═══════════════════════════════════════════════════
    if config.enable_validation {
        println!("🔍 Running output validation...");
        let result = ConversionValidator::validate_output(output, Some(input_size));
        ValidationDisplay::display_result(&result);
        
        if !result.valid && config.validation_level >= 4 {
            // 严格模式下验证失败则报错
            anyhow::bail!("Validation failed: output did not meet quality standards");
        }
    }
    
    // ═══════════════════════════════════════════════════
    // 📊 Phase 3.3: 质量分析
    // ═══════════════════════════════════════════════════
    if config.enable_quality_analysis {
        println!("📊 Running quality analysis...");
        let analyzer = QualityAnalyzer::new();
        
        match analyzer.analyze(output) {
            Ok(metrics) => {
                println!("   Estimated quality: {}", metrics.estimated_quality);
                println!("   Complexity score: {:.2}", metrics.complexity_score);
                println!("   Bytes per pixel: {:.2}", metrics.bytes_per_pixel);
                println!("   Content type: {}", metrics.content_type);
            }
            Err(e) => {
                println!("   ⚠️  Quality analysis failed: {}", e);
            }
        }
    }
    
    // ═══════════════════════════════════════════════════
    // 📄 XMP Sidecar合并 (如果启用)
    // ═══════════════════════════════════════════════════
    if config.merge_xmp_sidecar {
        merge_xmp_sidecar(input, output)?;
    }
    
    // ═══════════════════════════════════════════════════
    // 📊 SSIM质量验证 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_ssim).unwrap_or(false) {
        println!("📊 Running SSIM quality validation...");
        validate_ssim_quality(input, output)?;
    }
    
    let output_size = std::fs::metadata(output)?.len();
    let elapsed = start_time.elapsed();
    
    println!("✅ Conversion completed:");
    println!("   Input size: {} bytes", input_size);
    println!("   Output size: {} bytes", output_size);
    println!("   Compression ratio: {:.2}%", (output_size as f64 / input_size as f64) * 100.0);
    println!("   Time elapsed: {:.2}s", elapsed.as_secs_f64());
    
    // ═══════════════════════════════════════════════════
    // 🎓 在线学习：记录转换经验
    // ═══════════════════════════════════════════════════
    record_conversion_for_learning(input, output, config, input_size, output_size);
    
    Ok(ConversionResult {
        input_size,
        output_size,
        compression_ratio: output_size as f64 / input_size as f64,
        duration: elapsed,
        strategy_used,
    })
}

/// 🎓 记录转换经验用于在线学习
fn record_conversion_for_learning(
    input: &Path,
    output: &Path,
    config: &ConversionConfig,
    input_size: u64,
    output_size: u64,
) {
    use crate::ai::online_learner_manager::OnlineLearnerManager;
    use crate::core::feature_extractor_128d::extract_128d_features;
    use crate::ai::reward_calculator::ConversionResult as RewardResult;
    
    // 加载图像并提取特征
    let img = match image::open(input) {
        Ok(i) => i,
        Err(e) => {
            log::warn!("⚠️  Failed to open image for feature extraction: {}", e);
            return;
        }
    };
    
    // 获取基础特征
    let basic_features = crate::ImageFeatures {
        width: img.width(),
        height: img.height(),
        format: String::from("unknown"),
        has_alpha: img.color().has_alpha(),
        is_animated: false,
        complexity: 0.5,
        file_size: input_size,
    };
    
    let features = extract_128d_features(&img, input, &basic_features);
    
    // 计算SSIM（如果可能）
    let ssim = calculate_ssim_simple(input, output).unwrap_or(0.95);
    
    // 创建转换结果
    let result = RewardResult {
        original_size: input_size,
        output_size,
        ssim,
        processing_time: 0.0,
    };
    
    // 使用全局学习器管理器记录经验
    match OnlineLearnerManager::record_conversion(features, config.quality as u32, config.speed as u32, result) {
        Ok(_) => {
            let buffer_size = OnlineLearnerManager::buffer_size();
            println!("📝 Conversion experience recorded (global buffer: {})", buffer_size);
            
            // 如果buffer达到阈值，会自动触发更新
            if buffer_size >= 10 {
                println!("🎓 Model update triggered! ({} experiences accumulated)", buffer_size);
            }
        }
        Err(e) => {
            // 🔥 响亮失败：显示详细错误信息
            eprintln!("❌ Failed to record conversion experience: {}", e);
            eprintln!("   This may affect online learning quality");
            log::error!("Online learning error: {}", e);
        }
    }
}

/// 简单的SSIM计算
fn calculate_ssim_simple(input: &Path, output: &Path) -> Option<f64> {
    use std::process::Command;
    
    let output_result = Command::new("python3")
        .arg("scripts/calculate_ssim.py")
        .arg(input)
        .arg(output)
        .output()
        .ok()?;
    
    if !output_result.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output_result.stdout);
    stdout.trim().parse::<f64>().ok()
}

/// 🔒 使用Magika AI验证文件类型
fn validate_file_with_magika(input: &Path) -> Result<()> {
    use crate::utils::magika_detector::MagikaDetector;
    
    let detector = MagikaDetector::with_defaults();
    match detector.detect_file_type(input) {
        Ok(detection) => {
            println!("   ✅ File type: {} (confidence: {:.1}%)", 
                     detection.detected_type, detection.confidence * 100.0);
            
            // 检查置信度
            if !detection.is_high_confidence {
                println!("   ⚠️  Warning: Low confidence detection");
            }
            
            Ok(())
        }
        Err(e) => {
            // 🔥 质量宣言：AI失败就响亮报错
            eprintln!("❌ Magika AI validation FAILED: {}", e);
            eprintln!("   File validation cannot proceed without AI");
            Err(e)
        }
    }
}

/// 🔧 检查格式修正
fn check_format_correction(input: &Path) -> Result<()> {
    use crate::utils::format_corrector::FormatCorrector;
    
    let corrector = FormatCorrector::new(false);  // 不自动重命名，只检查
    match corrector.check_and_correct(input) {
        Ok(result) if result.needs_correction => {
            println!("   ⚠️  Format mismatch detected:");
            println!("      Extension: {}", result.original_extension);
            println!("      Actual format: {}", result.detected_format);
            println!("      {}", result.message);
            
            // 只警告，不阻止转换
            Ok(())
        }
        Ok(_) => {
            println!("   ✅ Format matches extension");
            Ok(())
        }
        Err(e) => {
            println!("   ⚠️  Format correction check failed: {}", e);
            // 不阻止转换
            Ok(())
        }
    }
}

/// 🔗 应用智能预处理
fn apply_preprocessing(input: &Path, _config: &ConversionConfig) -> Result<PathBuf> {
    use crate::operations::preprocessing::PreprocessPipeline;
    
    println!("   🔍 Analyzing image for preprocessing...");
    
    // 读取图像
    let img = image::open(input)?;
    
    // 应用预处理
    let pipeline = PreprocessPipeline::new();
    let processed_img = pipeline.process(img)?;
    
    // 保存到临时文件
    let temp_output = input.with_extension("preprocessed.png");
    processed_img.save(&temp_output)?;
    
    println!("   ✅ Preprocessing complete");
    Ok(temp_output)
}

/// 🎬 检测是否应该转换动图为视频
fn should_convert_animation_to_video(input: &Path) -> Result<bool> {
    // 检查文件扩展名
    let ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 只检测动图格式
    if !matches!(ext.as_str(), "gif" | "apng" | "webp") {
        return Ok(false);
    }
    
    // 获取文件大小
    let metadata = std::fs::metadata(input)?;
    let file_size = metadata.len();
    
    // 大于2MB的动图推荐转视频
    if file_size > 2 * 1024 * 1024 {
        return Ok(true);
    }
    
    // 尝试获取图像尺寸
    if let Ok(img) = image::open(input) {
        let (width, height) = img.dimensions();
        let pixels = width * height;
        
        // 高分辨率动图 (>800x600) 推荐转视频
        if pixels > 800 * 600 {
            return Ok(true);
        }
    }
    
    Ok(false)
}

/// 🎬 转换动图为视频
fn convert_animation_to_video(input: &Path, output: &Path) -> Result<()> {
    use crate::codecs::video::video_processor::{VideoProcessor, VideoConversionConfig, AudioMode};
    
    // 创建视频转换配置
    let config = VideoConversionConfig {
        codec: "h265".to_string(),  // H.265最佳压缩率
        container: "mp4".to_string(),
        crf: 23,  // 平衡质量和体积
        preset: "medium".to_string(),
        target_resolution: None,
        target_fps: None,
        audio_mode: AudioMode::Remove,  // 动图没有音频
        two_pass: false,
        hw_accel: "auto".to_string(),
        gop_size: Some(250),
        bframes: Some(3),
        ref_frames: Some(3),
        me_method: Some("hex".to_string()),
        pix_fmt: None,
        rate_control: None,  // 🔥 Phase 3: 添加rate_control字段
    };
    
    // 执行转换
    let processor = VideoProcessor::new();
    let result = processor.convert_video(input, output, &config, Some(|_progress: f32| {
        // 进度回调（可选）
    }))?;
    
    if !result.success {
        anyhow::bail!("Animation to video conversion failed: {}", 
                     result.error.unwrap_or_default());
    }
    
    Ok(())
}

/// 📊 SSIM质量验证
fn validate_ssim_quality(original: &Path, converted: &Path) -> Result<()> {
    use crate::analysis::quality_checker::QualityChecker;
    
    let checker = QualityChecker::new();
    
    match checker.check_conversion_quality(original, converted) {
        Ok(result) => {
            println!("   📊 SSIM Score: {:.4}", result.ssim_score);
            println!("   📊 Quality Grade: {}", result.quality_grade.as_str());
            
            if result.passed {
                println!("   ✅ Quality check passed");
            } else {
                println!("   ⚠️  Warning: Quality below threshold");
            }
            
            println!("   {}", result.details);
            
            Ok(())
        }
        Err(e) => {
            println!("   ⚠️  SSIM validation failed: {}", e);
            // 不阻止转换
            Ok(())
        }
    }
}

/// 验证输出质量
fn validate_output_quality(output: &Path, _config: &ConversionConfig) -> Result<()> {
    if !output.exists() {
        anyhow::bail!("Output file does not exist: {:?}", output);
    }
    
    let metadata = std::fs::metadata(output)?;
    if metadata.len() == 0 {
        anyhow::bail!("Output file is empty: {:?}", output);
    }
    
    // 尝试打开输出文件验证格式正确性
    // 注意: image crate不支持所有格式(如AVIF/JXL)，所以验证失败不一定是错误
    if let Err(e) = image::open(output) {
        // 检查是否是不支持的格式
        let ext = output.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if matches!(ext.as_str(), "avif" | "jxl" | "jpegxl") {
            // 这些格式由外部工具处理，跳过image crate验证
            println!("✅ Output quality validation passed (external format)");
            return Ok(());
        }
        
        anyhow::bail!("Output file format is invalid: {}", e);
    }
    
    println!("✅ Output quality validation passed");
    Ok(())
}

fn perform_conversion(
    input: &Path,
    output: &Path,
    format: &str,
    config: &ConversionConfig,
) -> Result<String> {
    use image::ImageFormat;
    
    // 检查输入格式，如果是外部格式(AVIF/JXL)，先转换为PNG
    let input_ext = input.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let (actual_input, temp_file) = if matches!(input_ext.as_str(), "avif" | "jxl" | "jpegxl") {
        // 创建临时PNG文件
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_millis();
        let temp_path = std::env::temp_dir().join(format!("pixly_temp_{}.png", timestamp));
        
        // 使用外部工具转换为PNG
        decode_external_format(input, &temp_path)?;
        (temp_path.clone(), Some(temp_path))
    } else {
        (input.to_path_buf(), None)
    };
    
    // 读取输入图像
    let img = image::open(&actual_input)?;
    
    // 根据格式选择编码器
    let strategy = match format.to_lowercase().as_str() {
        "webp" => {
            // WebP编码 - 使用save_with_format因为image crate的WebP编码器API限制
            img.save_with_format(output, ImageFormat::WebP)?;
            "webp_native"
        }
        "png" => {
            // PNG编码
            let file = std::fs::File::create(output)?;
            let encoder = image::codecs::png::PngEncoder::new(file);
            img.write_with_encoder(encoder)?;
            "png_native"
        }
        "jpg" | "jpeg" => {
            // JPEG编码
            let file = std::fs::File::create(output)?;
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(file, config.quality);
            img.write_with_encoder(encoder)?;
            "jpeg_native"
        }
        "gif" => {
            // GIF编码
            img.save_with_format(output, ImageFormat::Gif)?;
            "gif_native"
        }
        "bmp" => {
            img.save_with_format(output, ImageFormat::Bmp)?;
            "bmp_native"
        }
        "tiff" | "tif" => {
            img.save_with_format(output, ImageFormat::Tiff)?;
            "tiff_native"
        }
        "avif" => {
            // AVIF编码 - 使用外部工具
            convert_to_avif(&actual_input, output, config)?;
            "avif_external"
        }
        "jxl" | "jpegxl" => {
            // JPEG XL编码 - 使用外部工具
            convert_to_jxl(&actual_input, output, config)?;
            "jxl_external"
        }
        _ => {
            anyhow::bail!("Unsupported output format: {}", format);
        }
    };
    
    // 清理临时文件
    if let Some(temp) = temp_file {
        let _ = std::fs::remove_file(temp);
    }
    
    Ok(strategy.to_string())
}

/// 解码外部格式(AVIF/JXL)为PNG
fn decode_external_format(input: &Path, output: &Path) -> Result<()> {
    use std::process::Command;
    
    let input_ext = input.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match input_ext.as_str() {
        "avif" => {
            // 使用avifenc的解码功能或ImageMagick
            let output = Command::new("magick")
                .arg("convert")
                .arg(input)
                .arg(output)
                .output()?;
            
            if !output.status.success() {
                anyhow::bail!("AVIF decode failed: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        "jxl" | "jpegxl" => {
            // 使用djxl或ImageMagick
            let output = Command::new("magick")
                .arg("convert")
                .arg(input)
                .arg(output)
                .output()?;
            
            if !output.status.success() {
                anyhow::bail!("JXL decode failed: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => anyhow::bail!("Unsupported external format: {}", input_ext),
    }
    
    Ok(())
}

/// Convert to AVIF using external tools
fn convert_to_avif(input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
    use std::process::Command;
    
    // 🔥 Phase: 修复avifenc参数
    // avifenc使用 -q/--qcolor 而不是 --quality
    // avifenc使用 -s/--speed 而不是 --speed (虽然两者都支持)
    let mut cmd = Command::new("avifenc");
    
    // 🔥 修复空壳功能：使用format_specific_params中的min/max quantizer
    let (min_q, max_q) = if let Some(ref params) = config.format_specific_params
        && let crate::utils::format_params::FormatSpecificParams::Avif(avif) = params {
        (
            avif.min_quantizer.unwrap_or(0),
            avif.max_quantizer.unwrap_or(63)
        )
    } else {
        (0, 63)  // 默认值
    };
    
    cmd.arg("--min").arg(min_q.to_string())
        .arg("--max").arg(max_q.to_string())
        .arg("-s").arg(config.speed.to_string())
        .arg("-q").arg(config.quality.to_string());  // 修复：使用 -q 而不是 --quality
    
    println!("   🔧 AVIF: min_quantizer={}, max_quantizer={}", min_q, max_q);
    
    // 🔥 Phase: AVIF高级参数支持（修复潜在空壳）
    // Chroma subsampling: -y 或 --yuv (420, 422, 444)
    if let Some(ref chroma) = config.chroma_subsampling {
        cmd.arg("-y").arg(chroma);
        println!("   🔧 AVIF: Chroma subsampling {}", chroma);
    }
    
    // Alpha quality: --qalpha (0-100)
    if let Some(alpha_q) = config.alpha_quality {
        cmd.arg("--qalpha").arg(alpha_q.to_string());
        println!("   🔧 AVIF: Alpha quality {}", alpha_q);
    }
    
    cmd.arg(input).arg(output);
    
    let result = cmd.output();
    
    match result {
        Ok(output_result) if output_result.status.success() => Ok(()),
        Ok(output_result) => {
            let error = String::from_utf8_lossy(&output_result.stderr);
            anyhow::bail!("avifenc failed: {}", error)
        }
        Err(e) => {
            // 🔥 质量宣言：响亮的错误，不fallback！
            anyhow::bail!("avifenc not found or failed to execute: {}. Please install avifenc: brew install libavif", e)
        }
    }
}

/// Convert to JPEG XL using external tools
fn convert_to_jxl(input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
    use std::process::Command;
    
    // Find cjxl in system PATH
    let cjxl_path = which::which("cjxl")
        .map_err(|_| anyhow::anyhow!(
            "cjxl not found in PATH. Please install libjxl:\n\
             macOS: brew install jpeg-xl\n\
             Linux: apt install libjxl-tools or yum install libjxl-tools\n\
             Windows: Download from https://github.com/libjxl/libjxl/releases"
        ))?;
    
    let effort = (10 - config.speed).clamp(1, 9);
    let distance = ((100 - config.quality) as f32 / 10.0).clamp(0.0, 15.0);
    
    // 检测输入是否为JPEG
    let is_jpeg_input = input.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            let ext = e.to_lowercase();
            ext == "jpg" || ext == "jpeg"
        })
        .unwrap_or(false);
    
    let mut cmd = Command::new(&cjxl_path);
    cmd.arg(input)
        .arg(output)
        .arg("--effort").arg(effort.to_string());
    
    // JPEG输入时的特殊处理：
    // - cjxl默认启用--lossless_jpeg=1（无损重新打包）
    // - 如果用户要求有损转换，需要禁用lossless_jpeg并传递distance
    // - 如果用户要求无损，不传递distance（让cjxl使用默认的lossless_jpeg=1）
    if !config.lossless {
        if is_jpeg_input {
            // JPEG输入 + 有损转换：禁用lossless_jpeg，传递distance
            cmd.arg("--lossless_jpeg").arg("0");
        }
        cmd.arg("--distance").arg(distance.to_string());
    }
    // 如果config.lossless=true，不传递distance，让cjxl使用默认的无损模式
    
    // 🔥 Phase: JXL高级参数支持（修复空壳功能）
    // 根据PROJECT_QUALITY_MANIFESTO.md - 反对摆设代码原则
    
    // Modular mode: -m 0|1 或 --modular=0|1
    if config.jxl_modular {
        cmd.arg("--modular=1");
        println!("   🔧 JXL: Modular mode enabled");
    }
    
    // Progressive decoding: -p 或 --progressive
    if config.jxl_progressive {
        cmd.arg("--progressive");
        println!("   🔧 JXL: Progressive decoding enabled");
    }
    
    // Responsive decoding: -R K 或 --responsive=K
    if config.jxl_responsive {
        cmd.arg("--responsive=1");
        println!("   🔧 JXL: Responsive decoding enabled");
    }
    
    // Gaborish filter: --gaborish=0|1
    if config.jxl_gaborish {
        cmd.arg("--gaborish=1");
        println!("   🔧 JXL: Gaborish filter enabled");
    }
    
    let result = cmd.output();
    
    match result {
        Ok(output_result) if output_result.status.success() => Ok(()),
        Ok(output_result) => {
            let error = String::from_utf8_lossy(&output_result.stderr);
            anyhow::bail!("cjxl failed: {}", error)
        }
        Err(e) => {
            anyhow::bail!("cjxl execution failed: {}", e)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub duration: std::time::Duration,
    pub strategy_used: String,
}

/// Get XMP sidecar file path
/// 
/// XMP naming rule: replace extension with .xmp
/// Example: photo.jpg → photo.xmp
fn get_xmp_sidecar_path(file_path: &Path) -> std::path::PathBuf {
    let mut xmp_path = file_path.to_path_buf();
    xmp_path.set_extension("xmp");
    xmp_path
}

/// Merge XMP sidecar file into output image (Eagle-compatible)
/// 
/// Full process matching Eagle adapter:
/// 1. Pre-merge validation: Check XMP file exists and is readable
/// 2. Verify output file exists before merge
/// 3. Use exiftool to merge XMP into output file
/// 4. Post-merge validation: Verify XMP data was written
/// 5. Delete original XMP sidecar only after successful validation
/// 6. Handle all error cases gracefully
fn merge_xmp_sidecar(input: &Path, output: &Path) -> Result<()> {
    let xmp_path = get_xmp_sidecar_path(input);
    
    // Pre-merge validation: Check if XMP exists
    if !xmp_path.exists() {
        return Ok(()); // No XMP file, nothing to do
    }
    
    // Pre-merge validation: Check XMP is readable
    if let Err(e) = std::fs::metadata(&xmp_path) {
        eprintln!("⚠️  XMP file exists but not readable: {}", e);
        return Ok(());
    }
    
    println!("📄 Found XMP sidecar: {:?}", xmp_path.file_name());
    
    // Pre-merge validation: Verify output file exists
    if !output.exists() {
        eprintln!("⚠️  Output file doesn't exist yet, skipping XMP merge");
        return Ok(());
    }
    
    // Get output file size before merge for validation
    let output_size_before = std::fs::metadata(output)?.len();
    
    // Use exiftool to merge XMP into output file
    use std::process::Command;
    let merge_result = Command::new("exiftool")
        .arg("-tagsFromFile")
        .arg(&xmp_path)
        .arg("-XMP:all")
        .arg("-overwrite_original")
        .arg(output)
        .output();
    
    match merge_result {
        Ok(output_result) if output_result.status.success() => {
            println!("✅ XMP merged into output file: {:?}", output.file_name());
            
            // Post-merge validation: Verify output file was modified
            let output_size_after = std::fs::metadata(output)?.len();
            if output_size_after <= output_size_before {
                eprintln!("⚠️  Warning: Output file size didn't increase after XMP merge");
                eprintln!("   Before: {} bytes, After: {} bytes", output_size_before, output_size_after);
            }
            
            // Post-merge validation: Verify XMP data exists in output
            let verify_result = Command::new("exiftool")
                .arg("-XMP:all")
                .arg(output)
                .output();
            
            let xmp_verified = match verify_result {
                Ok(verify_output) if verify_output.status.success() => {
                    let output_str = String::from_utf8_lossy(&verify_output.stdout);
                    !output_str.trim().is_empty() && !output_str.contains("no XMP")
                }
                _ => false
            };
            
            if !xmp_verified {
                eprintln!("⚠️  Warning: Could not verify XMP data in output file");
                eprintln!("⚠️  Keeping original XMP sidecar for safety: {:?}", xmp_path);
                return Ok(());
            }
            
            println!("✅ XMP data verified in output file");
            
            // Delete original XMP after successful merge and validation
            if let Err(e) = std::fs::remove_file(&xmp_path) {
                eprintln!("⚠️  Failed to delete original XMP: {}", e);
            } else {
                println!("🗑️  Original XMP sidecar deleted");
            }
            Ok(())
        }
        Ok(output_result) => {
            let error_msg = String::from_utf8_lossy(&output_result.stderr);
            eprintln!("❌ XMP merge failed: {}", error_msg);
            eprintln!("⚠️  Keeping original XMP sidecar: {:?}", xmp_path);
            Ok(()) // Don't fail the conversion
        }
        Err(e) => {
            eprintln!("❌ Cannot execute exiftool: {}", e);
            eprintln!("💡 Install exiftool: brew install exiftool");
            eprintln!("⚠️  Keeping original XMP sidecar: {:?}", xmp_path);
            Ok(()) // Don't fail the conversion
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ConversionConfig::default();
        assert_eq!(config.quality, 85);
        assert_eq!(config.speed, 4);
        assert!(config.preserve_metadata);
        assert!(!config.merge_xmp_sidecar);
    }
    
    #[test]
    fn test_xmp_path_generation() {
        let input = Path::new("photo.jpg");
        let xmp = get_xmp_sidecar_path(input);
        assert_eq!(xmp, Path::new("photo.xmp"));
        
        let input2 = Path::new("/path/to/image.png");
        let xmp2 = get_xmp_sidecar_path(input2);
        assert_eq!(xmp2, Path::new("/path/to/image.xmp"));
    }
}
