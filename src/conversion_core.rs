// 🔄 转换核心逻辑
// 从 @archive/rust_broken/src/cli/conversion.rs 提取

use std::path::Path;
use anyhow::Result;
use crate::feature_toggles::FeatureToggles;
use crate::format_params::FormatSpecificParams;

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
            format_specific_params: None,
        }
    }
}

impl ConversionConfig {
    /// 从功能开关创建配置
    pub fn from_toggles(toggles: FeatureToggles) -> Self {
        Self {
            preserve_metadata: toggles.preserve_metadata,
            keep_animated: toggles.keep_animated,
            merge_xmp_sidecar: toggles.merge_xmp_sidecar,
            normalize_filenames: toggles.normalize_filenames,
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
    
    /// 检查是否允许手动覆盖
    pub fn allows_manual_override(&self) -> bool {
        self.feature_toggles
            .as_ref()
            .map(|t| t.allow_manual_override)
            .unwrap_or(true)
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
        if let Some(alpha_q) = self.alpha_quality {
            if alpha_q > 100 {
                anyhow::bail!("Alpha quality must be between 0-100");
            }
        }
        
        // 验证努力程度
        if let Some(effort) = self.effort {
            if effort > 10 {
                anyhow::bail!("Effort must be between 1-10");
            }
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
    
    // ═══════════════════════════════════════════════════
    // ✅ 输入验证 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_file_validation).unwrap_or(true) {
        validate_input_file(input)?;
    }
    
    // ═══════════════════════════════════════════════════
    // ✅ 配置验证
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_validation).unwrap_or(true) {
        config.validate()?;
    }
    
    let input_size = std::fs::metadata(input)?.len();
    
    // ═══════════════════════════════════════════════════
    // 🔄 执行实际转换
    // ═══════════════════════════════════════════════════
    let strategy_used = perform_conversion(input, output, format, config)?;
    
    // ═══════════════════════════════════════════════════
    // 📄 XMP Sidecar合并 (如果启用)
    // ═══════════════════════════════════════════════════
    let should_merge_xmp = toggles
        .map(|t| t.merge_xmp_sidecar)
        .unwrap_or(config.merge_xmp_sidecar);
    
    if should_merge_xmp {
        merge_xmp_sidecar(input, output)?;
    }
    
    // ═══════════════════════════════════════════════════
    // ✅ 输出质量验证 (如果启用)
    // ═══════════════════════════════════════════════════
    if toggles.map(|t| t.enable_quality_validation).unwrap_or(false) {
        validate_output_quality(output, config)?;
    }
    
    let output_size = std::fs::metadata(output)?.len();
    let elapsed = start_time.elapsed();
    
    println!("✅ Conversion completed:");
    println!("   Input size: {} bytes", input_size);
    println!("   Output size: {} bytes", output_size);
    println!("   Compression ratio: {:.2}%", (output_size as f64 / input_size as f64) * 100.0);
    println!("   Time elapsed: {:.2}s", elapsed.as_secs_f64());
    
    Ok(ConversionResult {
        input_size,
        output_size,
        compression_ratio: output_size as f64 / input_size as f64,
        duration: elapsed,
        strategy_used,
    })
}

/// 验证输入文件
fn validate_input_file(input: &Path) -> Result<()> {
    if !input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", input);
    }
    
    let metadata = std::fs::metadata(input)?;
    if !metadata.is_file() {
        anyhow::bail!("Input path is not a file: {:?}", input);
    }
    
    if metadata.len() == 0 {
        anyhow::bail!("Input file is empty: {:?}", input);
    }
    
    Ok(())
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
    if let Err(e) = image::open(output) {
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
    
    // 读取输入图像
    let img = image::open(input)?;
    
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
            convert_to_avif(input, output, config)?;
            "avif_external"
        }
        "jxl" | "jpegxl" => {
            // JPEG XL编码 - 使用外部工具
            convert_to_jxl(input, output, config)?;
            "jxl_external"
        }
        _ => {
            anyhow::bail!("Unsupported output format: {}", format);
        }
    };
    
    Ok(strategy.to_string())
}

/// Convert to AVIF using external tools
fn convert_to_avif(input: &Path, output: &Path, config: &ConversionConfig) -> Result<()> {
    use std::process::Command;
    
    // Try avifenc first
    let result = Command::new("avifenc")
        .arg("--min").arg("0")
        .arg("--max").arg("63")
        .arg("--speed").arg(config.speed.to_string())
        .arg("--quality").arg(config.quality.to_string())
        .arg(input)
        .arg(output)
        .output();
    
    match result {
        Ok(output_result) if output_result.status.success() => Ok(()),
        Ok(output_result) => {
            let error = String::from_utf8_lossy(&output_result.stderr);
            anyhow::bail!("avifenc failed: {}", error)
        }
        Err(e) => {
            // 不再fallback！直接报错！
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
    
    let result = Command::new(&cjxl_path)
        .arg(input)
        .arg(output)
        .arg("--effort").arg(effort.to_string())
        .arg("--distance").arg(distance.to_string())
        .output();
    
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
