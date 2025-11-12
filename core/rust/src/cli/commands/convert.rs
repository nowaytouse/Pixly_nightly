/**
 * Convert Command - 图像转换命令处理
 * 从cli/commands.rs拆分 (~270行)
 * 
 * 负责:
 * - 命令行参数解析
 * - AI预测集成
 * - 文件名规范化
 * - 转换执行
 */

use std::path::Path;
use std::process;
use pixly_converter::converter::params::{AIParameterClient, ImageCharacteristics};  // Phase 46.10: 重命名
use pixly_converter::converter::PredictionData;
use pixly_converter::converter::ai_client::{AIClient, PredictionRequest};
use image::{self, GenericImageView};  // Phase 46.14: 添加GenericImageView trait

use super::super::conversion::convert_image;
use super::super::help::show_quick_help;

/// 处理convert命令
pub fn handle(args: &[String]) {
    if args.len() < 2 {
        eprintln!("❌ Error: Missing arguments");
        show_quick_help();
        process::exit(1);
    }

    let input = &args[0];
    let output = &args[1];

    // 解析命令行选项
    let options = parse_options(args);
    
    // 输入文件验证
    if !Path::new(input).exists() {
        eprintln!("❌ Input file does not exist: {}", input);
        process::exit(1);
    }

    // Phase 46.9: 确定参数来源并相应处理
    let (quality, speed, prediction_data) = determine_parameters(
        input,
        output,
        &options
    );
    
    // 文件名规范化
    if options.normalize_filenames {
        handle_with_normalization(input, output, quality, speed, &options, prediction_data);
    } else {
        execute_conversion(input, output, quality, speed, &options, prediction_data);
    }
}

/// 命令行选项结构
/// 
/// Phase 46.9: 添加参数来源追踪
/// Phase 46.14: 添加预处理管道选项 (参考Rimage)
struct ConvertOptions {
    quality: u8,
    speed: u8,
    quality_explicit: bool,  // 用户是否明确指定quality
    speed_explicit: bool,    // 用户是否明确指定speed
    use_defaults: bool,      // 使用默认值，不调用AI
    preserve_metadata: bool,
    merge_xmp_sidecar: bool,
    keep_animated: bool,
    normalize_filenames: bool,
    optimize_mode: String,
    check_quality: bool,
    analyze: bool,
    estimate: bool,
    no_auto: bool,
    
    // Phase 46.14: 预处理选项
    resize: Option<String>,       // 例如 "1920x1080" 或 "50%"
    quantize: Option<u8>,         // 颜色数量 (1-256)
    sharpen: Option<f32>,         // 锐化强度 (0.0-2.0)
    resize_filter: String,        // 缩放滤镜: nearest/triangle/catmullrom/gaussian/lanczos3
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            quality_explicit: false,  // Phase 46.9: 默认未明确指定
            speed_explicit: false,    // Phase 46.9: 默认未明确指定
            use_defaults: false,      // Phase 46.9: 默认不使用defaults模式
            preserve_metadata: true,
            merge_xmp_sidecar: false,
            keep_animated: true,
            normalize_filenames: false,
            optimize_mode: String::from("balanced"),
            check_quality: false,
            analyze: false,
            estimate: false,
            no_auto: false,
            
            // Phase 46.14: 预处理默认值
            resize: None,
            quantize: None,
            sharpen: None,
            resize_filter: String::from("lanczos3"),  // 默认使用最高质量滤镜
        }
    }
}

/// 解析命令行选项
fn parse_options(args: &[String]) -> ConvertOptions {
    let mut options = ConvertOptions::default();
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--quality" => {
                if i + 1 < args.len() {
                    options.quality = args[i + 1].parse().unwrap_or(85).clamp(1, 100);
                    options.quality_explicit = true;  // Phase 46.9: 标记为明确指定
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--speed" => {
                if i + 1 < args.len() {
                    options.speed = args[i + 1].parse().unwrap_or(4).clamp(1, 10);
                    options.speed_explicit = true;    // Phase 46.9: 标记为明确指定
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--use-defaults" => {
                // Phase 46.9: 新选项 - 使用默认值，不调用AI
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
                        eprintln!("⚠️  Warning: Invalid optimize mode '{}', using 'balanced'", options.optimize_mode);
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
            // Phase 46.14: 预处理选项
            "--resize" => {
                if i + 1 < args.len() {
                    options.resize = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("⚠️  Warning: --resize requires a value (e.g., 1920x1080, 50%)");
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
                    eprintln!("⚠️  Warning: --quantize requires a value (1-256)");
                    i += 1;
                }
            }
            "--sharpen" => {
                if i + 1 < args.len() {
                    if let Ok(amount) = args[i + 1].parse::<f32>() {
                        if amount >= 0.0 && amount <= 2.0 {
                            options.sharpen = Some(amount);
                        } else {
                            eprintln!("⚠️  Warning: --sharpen value must be 0.0-2.0");
                        }
                    } else {
                        eprintln!("⚠️  Warning: Invalid --sharpen value");
                    }
                    i += 2;
                } else {
                    eprintln!("⚠️  Warning: --sharpen requires a value (0.0-2.0)");
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

/// Phase 46.9: 确定参数来源并获取最终参数
/// 
/// 架构原则：
/// - Rust是执行层，不做AI决策
/// - 用户明确指定参数时，直接使用（不调用AI）
/// - 用户选择使用默认值时，使用defaults（不调用AI）
/// - 只有用户未指定参数时，才调用Go AI服务
fn determine_parameters(
    input: &str,
    output: &str,
    options: &ConvertOptions,
) -> (u8, u8, Option<PredictionData>) {
    // 场景1: 用户明确指定了quality和speed
    if options.quality_explicit && options.speed_explicit {
        println!("✅ Using user-specified parameters");
        println!("   Quality: {}", options.quality);
        println!("   Speed: {}", options.speed);
        return (options.quality, options.speed, None);
    }
    
    // 场景2: 用户选择使用默认值
    if options.use_defaults || options.no_auto {
        println!("✅ Using default parameters (no AI)");
        let quality = if options.quality_explicit { options.quality } else { 85 };
        let speed = if options.speed_explicit { options.speed } else { 4 };
        println!("   Quality: {}", quality);
        println!("   Speed: {}", speed);
        return (quality, speed, None);
    }
    
    // 场景3: 需要AI推荐
    println!("🤖 Requesting AI parameter recommendation...");
    
    // 调用AI服务获取推荐
    let (quality, speed) = optimize_parameters(
        input,
        output,
        options.quality,
        options.speed,
        options.analyze,
        options.estimate,
    );
    
    let prediction_data = perform_ai_prediction(
        input,
        output,
        quality,
        speed,
        options.analyze,
    );
    
    (quality, speed, prediction_data)
}

/// 优化参数（调用Go AI服务）
/// 
/// Phase 46.9注释：此函数实际上是调用Go AI服务，不是Rust自己优化
fn optimize_parameters(
    input: &str,
    output: &str,
    mut quality: u8,
    mut speed: u8,
    analyze: bool,
    estimate: bool,
) -> (u8, u8) {
    let output_format = output.rsplit('.').next().unwrap_or("").to_lowercase();
    
    match std::fs::metadata(input) {
        Ok(metadata) => {
            let file_size = metadata.len();
            let input_format = input.rsplit('.').next().unwrap_or("").to_lowercase();
            
            let chars = ImageCharacteristics {
                width: 1920,
                height: 1080,
                file_size,
                format: input_format,
                has_alpha: false,
                is_animated: false,
                complexity: 0.5,
                path: Some(input.to_string()),
            };

            let optimizer = AIParameterClient::new(&output_format);
            if let Ok(params) = optimizer.get_parameters_from_ai(&chars) {
                if analyze {
                    println!("🔍 Optimization Analysis:");
                    println!("   Recommended Quality: {}", params.quality);
                    println!("   Recommended Speed: {}", params.speed);
                    println!("   Reason: {}", params.reason);
                    if estimate {
                        println!("   Estimated Size: {} bytes", params.estimated_size);
                        println!("   Estimated Ratio: {:.1}%", params.estimated_ratio * 100.0);
                    }
                    println!();
                }
                
                quality = params.quality;
                speed = params.speed;
            }
        }
        Err(e) => {
            if analyze {
                eprintln!("⚠️  Cannot read file metadata: {}", e);
            }
        }
    }
    
    (quality, speed)
}

/// 执行AI预测
fn perform_ai_prediction(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    analyze: bool,
) -> Option<PredictionData> {
    let input_format = input.rsplit('.').next().unwrap_or("").to_lowercase();
    let output_format = output.rsplit('.').next().unwrap_or("").to_lowercase();
    
    // 获取图像尺寸
    let (width, height) = if let Ok(img) = image::open(input) {
        (img.width(), img.height())
    } else {
        (1920, 1080)
    };
    
    // 调用AI服务
    let ai_client = AIClient::with_default();
    let file_size = std::fs::metadata(input)
        .map(|m| m.len())
        .unwrap_or(0);
    
    let prediction_request = PredictionRequest {
        image_path: Some(input.to_string()),
        input_format: input_format.clone(),
        target_format: output_format.clone(),
        width,
        height,
        file_size,
        has_alpha: false,
        has_animation: false,
        color_space: None,
        bit_depth: None,
        priority: "balanced".to_string(),
        preserve_quality: true,
        model_type: None,
        request_id: None,
        enable_bayesian: Some(true),
        enable_ppo: Some(true),
        enable_smart_quality: Some(true),
        enable_auto_optimize: Some(true),
        enable_video_for_anim: Some(true),
    };
    
    match ai_client.predict(&prediction_request) {
        Ok(response) => {
            println!("🤖 AI Prediction (confidence: {:.1}%):", response.confidence * 100.0);
            println!("   Quality: {}", response.quality.unwrap_or(quality));
            println!("   Speed: {}", response.speed.unwrap_or(speed));
            println!("   Lossless: {}", response.lossless.unwrap_or(false));
            
            Some(PredictionData {
                format: output_format,
                quality: response.quality.unwrap_or(quality),
                speed: response.speed.unwrap_or(speed),
                lossless: response.lossless.unwrap_or(false),
                predicted_size: None,
                confidence: Some(response.confidence),
                preprocessing_steps: response.preprocessing_steps.clone(),
                optimization_path: response.optimization_path.clone(),
            })
        }
        Err(e) => {
            if analyze {
                eprintln!("⚠️  AI prediction failed: {}", e);
                eprintln!("   Proceeding with manual parameters...");
            }
            None
        }
    }
}

/// 处理文件名规范化
fn handle_with_normalization(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    options: &ConvertOptions,
    prediction_data: Option<PredictionData>,
) {
    use pixly_converter::converter::filename_normalizer::FilenameNormalizer;
    
    let mut normalizer = FilenameNormalizer::new();
    let input_path = Path::new(input);
    
    match normalizer.normalize(input_path) {
        Ok(temp_path) => {
            println!("📝 Normalized filename: {:?} → {:?}", input, temp_path);
            let temp_input = temp_path.to_str().unwrap_or(input);
            
            // 执行转换
            execute_conversion(temp_input, output, quality, speed, options, prediction_data.clone());
            
            // 恢复文件名
            match normalizer.restore(&temp_path) {
                Ok(original_path) => {
                    println!("✅ Restored original filename: {:?}", original_path);
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to restore filename: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️  Filename normalization failed: {}", e);
            eprintln!("   Proceeding with original filename...");
            execute_conversion(input, output, quality, speed, options, prediction_data);
        }
    }
}

/// 执行转换
fn execute_conversion(
    input: &str,
    output: &str,
    quality: u8,
    speed: u8,
    options: &ConvertOptions,
    prediction_data: Option<PredictionData>,
) {
    // Phase 46.14: 当prediction_data为None时，表示使用默认值或用户指定参数，应跳过AI
    let skip_ai = prediction_data.is_none();
    
    // Phase 46.14+: 应用预处理管道（支持AI建议自动应用）
    let input_path = apply_preprocessing_if_needed(input, options, prediction_data.as_ref());
    let actual_input = input_path.as_deref().unwrap_or(input);
    
    convert_image(
        actual_input,
        output,
        quality,
        speed,
        options.preserve_metadata,
        options.merge_xmp_sidecar,
        options.keep_animated,
        options.check_quality,
        prediction_data,
        &options.optimize_mode,
        skip_ai,  // Phase 46.14: 传递skip_ai标志
    );
    
    // Phase 46.14: 清理临时预处理文件
    if let Some(temp_path) = input_path {
        let _ = std::fs::remove_file(&temp_path);
    }
}

/// Phase 46.14+: 应用预处理管道（如果需要）
/// 支持AI预处理建议自动应用（R-003）
fn apply_preprocessing_if_needed(
    input: &str,
    options: &ConvertOptions,
    prediction_data: Option<&PredictionData>,
) -> Option<String> {
    use pixly_converter::preprocessing::{PreprocessPipeline, PreprocessStep, FilterType};
    
    // 🆕 R-003: 检查AI预处理建议
    let has_ai_suggestions = prediction_data
        .and_then(|pd| pd.preprocessing_steps.as_ref())
        .map(|steps| !steps.is_empty())
        .unwrap_or(false);
    
    // 检查是否需要预处理（用户指定 或 AI建议）
    let needs_preprocessing = options.resize.is_some() 
        || options.quantize.is_some() 
        || options.sharpen.is_some()
        || has_ai_suggestions;
    
    if !needs_preprocessing {
        return None;
    }
    
    if has_ai_suggestions {
        println!("🤖 Applying AI-suggested preprocessing steps");
    }
    
    println!("🔧 Preprocessing pipeline activated");
    
    // 读取图像
    let image = match image::open(input) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("⚠️  Failed to open image for preprocessing: {}", e);
            return None;
        }
    };
    
    // 构建预处理管道
    let mut pipeline = PreprocessPipeline::new();
    
    // 🆕 R-003: 优先应用AI建议的预处理步骤
    if let Some(pd) = prediction_data {
        if let Some(steps) = &pd.preprocessing_steps {
            for suggestion in steps {
                match suggestion.step.as_str() {
                    "resize" => {
                        if let (Some(w), Some(h)) = (
                            suggestion.params["width"].as_u64(),
                            suggestion.params["height"].as_u64(),
                        ) {
                            println!("    🤖 AI suggests resize: {}x{} ({})", w, h, suggestion.reason);
                            let filter = FilterType::from_str(
                                suggestion.params["filter"].as_str().unwrap_or("lanczos3")
                            ).unwrap_or(FilterType::Lanczos3);
                            pipeline = pipeline.add_step(PreprocessStep::Resize {
                                width: w as u32,
                                height: h as u32,
                                filter,
                            });
                        }
                    }
                    "quantize" => {
                        if let Some(colors) = suggestion.params["colors"].as_u64() {
                            println!("    🤖 AI suggests quantize: {} colors ({})", colors, suggestion.reason);
                            pipeline = pipeline.add_step(PreprocessStep::Quantization {
                                colors: colors as u8,
                                dithering: false,
                            });
                        }
                    }
                    "sharpen" => {
                        if let Some(amount) = suggestion.params["amount"].as_f64() {
                            println!("    🤖 AI suggests sharpen: {:.2} ({})", amount, suggestion.reason);
                            pipeline = pipeline.add_step(PreprocessStep::Sharpen {
                                amount: amount as f32,
                            });
                        }
                    }
                    _ => {
                        eprintln!("    ⚠️  Unknown preprocessing step: {}", suggestion.step);
                    }
                }
            }
        }
    }
    
    // 用户指定的预处理步骤（用户参数优先级更高，会覆盖AI建议）
    // 添加resize步骤
    if let Some(ref resize_param) = options.resize {
        if let Some((width, height)) = pixly_converter::preprocessing::parse_resize_param(resize_param) {
            let filter = FilterType::from_str(&options.resize_filter)
                .unwrap_or(FilterType::Lanczos3);
            
            // Phase 46.14: 处理百分比缩放
            let (actual_width, actual_height) = if width < 200 && height == 0 {
                // 百分比缩放模式（parse_resize_param返回(percent, 0)）
                let percent = width as f64 / 100.0;
                let (img_width, img_height) = image.dimensions();
                let new_width = (img_width as f64 * percent) as u32;
                let new_height = (img_height as f64 * percent) as u32;
                println!("    📐 Percentage resize: {}% → {}x{}", width, new_width, new_height);
                (new_width, new_height)
            } else {
                (width, height)
            };
            
            pipeline = pipeline.add_step(PreprocessStep::Resize {
                width: actual_width,
                height: actual_height,
                filter,
            });
        } else {
            eprintln!("⚠️  Invalid resize parameter: {}", resize_param);
        }
    }
    
    // 添加quantization步骤
    if let Some(colors) = options.quantize {
        pipeline = pipeline.add_step(PreprocessStep::Quantization {
            colors,
            dithering: false,
        });
    }
    
    // 添加sharpen步骤
    if let Some(amount) = options.sharpen {
        pipeline = pipeline.add_step(PreprocessStep::Sharpen { amount });
    }
    
    // 执行预处理
    let preprocessed = match pipeline.process(image) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("⚠️  Preprocessing failed: {}", e);
            return None;
        }
    };
    
    // 保存到临时文件
    let temp_path = format!("{}.preprocessed.png", input);
    if let Err(e) = preprocessed.save(&temp_path) {
        eprintln!("⚠️  Failed to save preprocessed image: {}", e);
        return None;
    }
    
    println!("✅ Preprocessing complete, saved to temporary file");
    Some(temp_path)
}
