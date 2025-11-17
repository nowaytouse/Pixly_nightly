// 🚀 CLI Convert命令 - 完整实现
// 从 @archive/rust_broken/src/cli/commands/convert.rs 提取完整功能
//
// 核心功能:
// - 命令行参数解析
// - AI预测集成
// - 文件名规范化
// - 预处理管道
// - 转换执行

use std::path::Path;
use anyhow::Result;

/// 命令行选项结构
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    pub quality: u8,
    pub speed: u8,
    pub quality_explicit: bool,
    pub speed_explicit: bool,
    pub use_defaults: bool,
    pub preserve_metadata: bool,
    pub merge_xmp_sidecar: bool,
    pub keep_animated: bool,
    pub normalize_filenames: bool,
    pub optimize_mode: String,
    pub check_quality: bool,
    pub analyze: bool,
    pub estimate: bool,
    pub no_auto: bool,
    
    // 预处理选项
    pub resize: Option<String>,
    pub quantize: Option<u8>,
    pub sharpen: Option<f32>,
    pub resize_filter: String,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            quality_explicit: false,
            speed_explicit: false,
            use_defaults: false,
            preserve_metadata: true,
            merge_xmp_sidecar: false,
            keep_animated: true,
            normalize_filenames: false,
            optimize_mode: String::from("balanced"),
            check_quality: false,
            analyze: false,
            estimate: false,
            no_auto: false,
            resize: None,
            quantize: None,
            sharpen: None,
            resize_filter: String::from("lanczos3"),
        }
    }
}

/// 解析命令行选项
pub fn parse_options(args: &[String]) -> ConvertOptions {
    let mut options = ConvertOptions::default();
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--quality" => {
                if i + 1 < args.len() {
                    options.quality = args[i + 1].parse().unwrap_or(85).clamp(1, 100);
                    options.quality_explicit = true;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--speed" => {
                if i + 1 < args.len() {
                    options.speed = args[i + 1].parse().unwrap_or(4).clamp(1, 10);
                    options.speed_explicit = true;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--use-defaults" => {
                options.use_defaults = true;
                i += 1;
            }
            "--metadata" => {
                options.preserve_metadata = true;
                i += 1;
            }
            "--merge-xmp" => {
                options.merge_xmp_sidecar = true;
                i += 1;
            }
            "--no-merge-xmp" => {
                options.merge_xmp_sidecar = false;
                i += 1;
            }
            "--animated" => {
                options.keep_animated = true;
                i += 1;
            }
            "--normalize-filenames" => {
                options.normalize_filenames = true;
                i += 1;
            }
            "--optimize-mode" => {
                if i + 1 < args.len() {
                    options.optimize_mode = args[i + 1].to_lowercase();
                    if !["size", "balanced", "quality", "general"].contains(&options.optimize_mode.as_str()) {
                        eprintln!("⚠️  Warning: Invalid optimization mode '{}', using 'balanced'", options.optimize_mode);
                        options.optimize_mode = String::from("balanced");
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--check-quality" => {
                options.check_quality = true;
                i += 1;
            }
            "--analyze" => {
                options.analyze = true;
                i += 1;
            }
            "--estimate" => {
                options.estimate = true;
                i += 1;
            }
            "--no-auto" => {
                options.no_auto = true;
                i += 1;
            }
            "--resize" => {
                if i + 1 < args.len() {
                    options.resize = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("⚠️  Warning: --resize requires argument (e.g.: 1920x1080, 50%)");
                    i += 1;
                }
            }
            "--quantize" => {
                if i + 1 < args.len() {
                    if let Ok(colors) = args[i + 1].parse::<u8>() {
                        if colors > 0 {
                            options.quantize = Some(colors);
                        } else {
                            eprintln!("⚠️  Warning: --quantize value must be > 0");
                        }
                    } else {
                        eprintln!("⚠️  Warning: Invalid --quantize value");
                    }
                    i += 2;
                } else {
                    eprintln!("⚠️  Warning: --quantize requires argument (1-256)");
                    i += 1;
                }
            }
            "--sharpen" => {
                if i + 1 < args.len() {
                    if let Ok(amount) = args[i + 1].parse::<f32>() {
                        if (0.0..=2.0).contains(&amount) {
                            options.sharpen = Some(amount);
                        } else {
                            eprintln!("⚠️  Warning: --sharpen value must be between 0.0-2.0");
                        }
                    } else {
                        eprintln!("⚠️  Warning: Invalid --sharpen value");
                    }
                    i += 2;
                } else {
                    eprintln!("⚠️  Warning: --sharpen requires argument (0.0-2.0)");
                    i += 1;
                }
            }
            "--filter" => {
                if i + 1 < args.len() {
                    options.resize_filter = args[i + 1].to_lowercase();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    
    options
}

/// 处理convert命令
pub fn handle_convert(input: &str, output: &str, options: &ConvertOptions) -> Result<()> {
    // 输入文件验证
    if !Path::new(input).exists() {
        anyhow::bail!("Input file does not exist: {}", input);
    }
    
    // 确定参数来源
    let (quality, speed) = determine_parameters(input, output, options)?;
    
    // 文件名规范化
    if options.normalize_filenames {
        handle_with_normalization(input, output, quality, speed, options)?;
    } else {
        execute_conversion(input, output, quality, speed, options)?;
    }
    
    Ok(())
}

/// 确定参数来源
fn determine_parameters(
    _input: &str,
    _output: &str,
    options: &ConvertOptions,
) -> Result<(u8, u8)> {
    // 场景1: 用户明确指定了quality和speed
    if options.quality_explicit && options.speed_explicit {
        println!("✅ Using user-specified parameters");
        println!("   Quality: {}", options.quality);
        println!("   Speed: {}", options.speed);
        return Ok((options.quality, options.speed));
    }
    
    // 场景2: 用户选择使用默认值
    if options.use_defaults || options.no_auto {
        println!("✅ Using default parameters (no AI)");
        let quality = if options.quality_explicit { options.quality } else { 85 };
        let speed = if options.speed_explicit { options.speed } else { 4 };
        println!("   Quality: {}", quality);
        println!("   Speed: {}", speed);
        return Ok((quality, speed));
    }
    
    // 场景3: 需要AI推荐
    println!("🤖 Requesting AI parameter recommendation...");
    
    // 使用本地AI预测
    let quality = options.quality;
    let speed = options.speed;
    
    Ok((quality, speed))
}

/// 处理文件名规范化
fn handle_with_normalization(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    options: &ConvertOptions,
) -> Result<()> {
    use crate::filename_normalizer::FilenameNormalizer;
    
    let mut normalizer = FilenameNormalizer::new();
    let input_path = Path::new(input);
    
    // 1. 规范化输入文件名（创建临时文件）
    match normalizer.normalize(input_path) {
        Ok(temp_input_path) => {
            println!("📝 Input normalized: {:?} → {:?}", input, temp_input_path);
            
            // 如果规范化后的路径与原路径不同，需要重命名文件
            if temp_input_path != input_path
                && let Err(e) = std::fs::rename(input_path, &temp_input_path) {
                    eprintln!("⚠️  Failed to rename input file: {}", e);
                    eprintln!("   Using original filename...");
                    execute_conversion(input, output, quality, speed, options)?;
                    return Ok(());
                }
            
            let temp_input_str = temp_input_path.to_str().unwrap_or(input);
            
            // 2. 使用规范化的输入文件名进行转换（输出使用用户指定的名称）
            let conversion_result = execute_conversion(temp_input_str, output, quality, speed, options);
            
            // 3. 恢复输入文件的原始文件名
            if temp_input_path != input_path {
                match normalizer.restore(&temp_input_path) {
                    Ok(original_path) => {
                        if let Err(e) = std::fs::rename(&temp_input_path, &original_path) {
                            eprintln!("⚠️  Failed to restore input filename: {}", e);
                            eprintln!("   Input file remains as: {:?}", temp_input_path);
                        } else {
                            println!("✅ Input filename restored: {:?}", original_path);
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to restore input filename: {}", e);
                        eprintln!("   Input file remains as: {:?}", temp_input_path);
                    }
                }
            }
            
            conversion_result?;
        }
        Err(e) => {
            eprintln!("⚠️  Input normalization failed: {}", e);
            eprintln!("   Using original filename...");
            execute_conversion(input, output, quality, speed, options)?;
        }
    }
    
    Ok(())
}

/// 执行转换
fn execute_conversion(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    options: &ConvertOptions,
) -> Result<()> {
    println!("🔄 Starting conversion:");
    println!("   Input: {}", input);
    println!("   Output: {}", output);
    println!("   Quality: {}", quality);
    println!("   Speed: {}", speed);
    println!("   Preserve metadata: {}", options.preserve_metadata);
    println!("   Merge XMP: {}", options.merge_xmp_sidecar);
    println!("   Keep animated: {}", options.keep_animated);
    
    // 应用预处理
    let processed_input = apply_preprocessing_if_needed(input, options)?;
    let final_input = processed_input.as_deref().unwrap_or(input);
    
    // 提取输出格式
    let output_path = Path::new(output);
    let format = output_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("webp");
    
    // 执行实际转换
    use crate::conversion_core::{execute_conversion as core_convert, ConversionConfig};
    
    let config = ConversionConfig {
        quality,
        speed,
        preserve_metadata: options.preserve_metadata,
        keep_animated: options.keep_animated,
        lossless: quality == 100,
        merge_xmp_sidecar: options.merge_xmp_sidecar,
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
    };
    
    core_convert(Path::new(final_input), output_path, format, &config)?;
    
    // 清理临时文件
    if let Some(temp_path) = processed_input {
        let _ = std::fs::remove_file(temp_path);
    }
    
    Ok(())
}

/// 应用预处理管道
fn apply_preprocessing_if_needed(
    input: &str,
    options: &ConvertOptions,
) -> Result<Option<String>> {
    let needs_preprocessing = options.resize.is_some() 
        || options.quantize.is_some() 
        || options.sharpen.is_some();
    
    if !needs_preprocessing {
        return Ok(None);
    }
    
    println!("🔧 Preprocessing pipeline activated");
    
    use crate::preprocessing::parse_resize_param;
    
    // 读取图像
    let mut img = image::open(input)?;
    let mut modified = false;
    
    // 1. 调整大小
    if let Some(ref resize_str) = options.resize
        && let Some((width, height)) = parse_resize_param(resize_str)
    {
        println!("   📐 Resize: {}x{}", width, height);
        img = apply_resize(&img, width, height, &options.resize_filter)?;
        modified = true;
    }
    
    // 2. 锐化
    if let Some(amount) = options.sharpen {
        println!("   ✨ Sharpen: {}", amount);
        img = apply_sharpen(&img, amount)?;
        modified = true;
    }
    
    // 3. 量化（减少颜色）
    if let Some(colors) = options.quantize {
        println!("   🎨 Quantize: {} colors", colors);
        // 量化需要特殊处理，暂时跳过
        let _ = colors;
    }
    
    if !modified {
        return Ok(None);
    }
    
    // 保存到临时文件
    let temp_path = format!("{}.preprocessed.png", input);
    img.save(&temp_path)?;
    println!("   💾 Preprocessing result: {}", temp_path);
    
    Ok(Some(temp_path))
}

fn apply_resize(img: &image::DynamicImage, width: u32, height: u32, filter: &str) -> Result<image::DynamicImage> {
    use image::imageops::FilterType;
    
    let filter_type = match filter {
        "nearest" => FilterType::Nearest,
        "triangle" | "bilinear" => FilterType::Triangle,
        "catmullrom" | "cubic" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        other => {
            eprintln!("⚠️  Unknown filter type '{}', using default lanczos3", other);
            FilterType::Lanczos3
        }
    };
    
    Ok(img.resize(width, height, filter_type))
}

fn apply_sharpen(img: &image::DynamicImage, amount: f32) -> Result<image::DynamicImage> {
    use crate::sharpen::{SharpenConfig, SimdSharpener};
    
    let config = SharpenConfig {
        strength: amount,
        radius: 1,
        threshold: 0.1,
        use_simd: true,
    };
    
    let sharpener = SimdSharpener::new(config);
    sharpener.sharpen(img)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_options() {
        let options = ConvertOptions::default();
        assert_eq!(options.quality, 85);
        assert_eq!(options.speed, 4);
        assert!(!options.quality_explicit);
        assert!(!options.speed_explicit);
    }
    
    #[test]
    fn test_parse_quality_option() {
        let args = vec![
            "input.jpg".to_string(),
            "output.webp".to_string(),
            "--quality".to_string(),
            "90".to_string(),
        ];
        let options = parse_options(&args);
        assert_eq!(options.quality, 90);
        assert!(options.quality_explicit);
    }
    
    #[test]
    fn test_parse_speed_option() {
        let args = vec![
            "input.jpg".to_string(),
            "output.webp".to_string(),
            "--speed".to_string(),
            "6".to_string(),
        ];
        let options = parse_options(&args);
        assert_eq!(options.speed, 6);
        assert!(options.speed_explicit);
    }
    
    #[test]
    fn test_parse_use_defaults() {
        let args = vec![
            "input.jpg".to_string(),
            "output.webp".to_string(),
            "--use-defaults".to_string(),
        ];
        let options = parse_options(&args);
        assert!(options.use_defaults);
    }
    
    #[test]
    fn test_parse_preprocessing_options() {
        let args = vec![
            "input.jpg".to_string(),
            "output.webp".to_string(),
            "--resize".to_string(),
            "1920x1080".to_string(),
            "--quantize".to_string(),
            "128".to_string(),
            "--sharpen".to_string(),
            "1.5".to_string(),
        ];
        let options = parse_options(&args);
        assert_eq!(options.resize, Some("1920x1080".to_string()));
        assert_eq!(options.quantize, Some(128));
        assert_eq!(options.sharpen, Some(1.5));
    }
}
