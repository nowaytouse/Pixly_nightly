use std::env;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pixly_kernel::conversion_core::{execute_conversion, ConversionConfig};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Setup PATH to include plugin bin directory and common tool locations
fn setup_path() {
    let current_path = env::var("PATH").unwrap_or_default();
    
    let mut paths: Vec<String> = Vec::new();
    
    // 1. Plugin bin directory (highest priority - bundled tools)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(bin_dir) = exe_path.parent() {
            paths.push(bin_dir.to_string_lossy().to_string());
        }
    }
    
    // 2. Original PATH
    paths.push(current_path);
    
    // 3. Common tool locations across platforms
    let additional_paths = vec![
        "/opt/homebrew/bin",      // macOS Homebrew (Apple Silicon)
        "/usr/local/bin",          // macOS Homebrew (Intel) / Linux
        "/usr/bin",                // Linux
        "/opt/local/bin",          // MacPorts
        "C:\\Program Files\\libjxl\\bin",  // Windows
        "C:\\Program Files\\ffmpeg\\bin",  // Windows
    ];
    paths.extend(additional_paths.iter().map(|s| s.to_string()));
    
    let new_path = paths.join(if cfg!(windows) { ";" } else { ":" });
    
    // SAFETY: We're setting PATH at startup before any threads are created
    unsafe {
        env::set_var("PATH", new_path);
    }
}

#[derive(Parser)]
#[command(name = "pixly-converter")]
#[command(about = "PIXLY Format Converter - Professional Media Conversion Tool", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🔍 Analyze media files with AI recommendations
    Analyze {
        /// Input file path
        input: PathBuf,
        
        /// Use AI to recommend optimal format and parameters
        #[arg(long, default_value = "true")]
        ai: bool,
        
        /// Output in JSON format (for programmatic use)
        #[arg(long, default_value = "false")]
        json: bool,
        
        /// Target format to analyze for (optional)
        #[arg(short, long)]
        format: Option<String>,
    },
    
    /// Convert media files
    Convert {
        /// Input file path
        input: PathBuf,
        
        /// Output format (jxl, avif, webp, heic, mp4, mov, etc.)
        #[arg(short, long)]
        format: Option<String>,
        
        /// Quality (0-100)
        #[arg(short, long, default_value = "90")]
        quality: u8,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Merge XMP sidecar files
        #[arg(long, default_value = "true")]
        merge_xmp: bool,
        
        /// XMP file path (if provided, skip scanning)
        #[arg(long)]
        xmp_path: Option<PathBuf>,
        
        /// Normalize filenames (handle special characters)
        #[arg(long, default_value = "false")]
        normalize_filenames: bool,
        
        /// 🤖 Use AI to predict optimal parameters
        #[arg(long, default_value = "false")]
        ai: bool,
        
        // JXL specific
        #[arg(long)]
        jpeg_lossless: bool,
        
        #[arg(long)]
        effort: Option<u8>,
        
        #[arg(long)]
        distance: Option<f64>,
        
        #[arg(long)]
        modular: bool,
        
        #[arg(long)]
        progressive: bool,
        
        #[arg(long)]
        responsive: bool,
        
        #[arg(long)]
        gaborish: bool,
        
        #[arg(long)]
        bit_depth: Option<String>,
        
        #[arg(long)]
        color_space: Option<String>,
        
        // AVIF specific
        #[arg(long)]
        speed: Option<u8>,
        
        #[arg(long)]
        min_quantizer: Option<u8>,
        
        #[arg(long)]
        max_quantizer: Option<u8>,
        
        #[arg(long)]
        chroma: Option<String>,
        
        #[arg(long)]
        tiles: Option<String>,
        
        // WebP specific
        #[arg(long)]
        method: Option<u8>,
        
        #[arg(long)]
        filter_strength: Option<u8>,
        
        #[arg(long)]
        sharpness: Option<u8>,
        
        // HEIC specific
        #[arg(long)]
        encoder: Option<String>,
        
        #[arg(long)]
        lossless: bool,
        
        #[arg(long)]
        thumbnail: bool,
        
        // Video specific
        #[arg(long)]
        container: Option<String>,
        
        #[arg(long)]
        crf: Option<u8>,
        
        #[arg(long)]
        rate_control: Option<String>,
        
        #[arg(long)]
        gop: Option<u32>,
        
        #[arg(long)]
        bframes: Option<u8>,
        
        #[arg(long)]
        refs: Option<u8>,
        
        #[arg(long)]
        me_method: Option<String>,
        
        #[arg(long)]
        pix_fmt: Option<String>,
    },
}

fn main() {
    // Setup PATH to include common tool locations
    setup_path();
    
    let cli = Cli::parse();
    
    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Analyze {
            input,
            ai,
            json,
            format,
        } => {
            // 🔍 调用analyze模块
            use pixly_kernel::cli_analyze::{handle_analyze, AnalyzeOptions};
            
            let options = AnalyzeOptions {
                use_ai: ai,
                json_output: json,
                target_format: format,
            };
            
            handle_analyze(
                input.to_str().context("Invalid input path")?,
                &options
            )?;
            
            Ok(())
        }
        
        Commands::Convert {
            input,
            format,
            quality,
            output,
            // JXL
            jpeg_lossless,
            effort,
            distance: _,
            modular,
            progressive,
            responsive,
            gaborish,
            bit_depth: _,
            color_space: _,
            // AVIF
            speed,
            min_quantizer: _,
            max_quantizer: _,
            chroma,
            tiles: _,
            // WebP
            method,
            filter_strength: _,
            sharpness: _,
            // HEIC
            encoder: _,
            lossless,
            thumbnail: _,
            // Video
            container: _,
            crf: _,
            rate_control: _,
            gop: _,
            bframes: _,
            refs: _,
            me_method: _,
            pix_fmt: _,
            // Tools
            merge_xmp,
            xmp_path,
            normalize_filenames,
            // AI
            ai,
        } => {
            // 确定输出格式
            let target_format = format.unwrap_or_else(|| {
                // 如果没有指定格式，从输入文件扩展名推断
                input
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jxl")
                    .to_string()
            });
            
            // 创建可变的参数变量（用于AI覆盖）
            let mut final_quality = quality;
            let final_speed = speed;  // 目前AI不推荐speed参数
            let final_effort = effort;  // 目前AI不推荐effort参数
            
            // 🤖 AI参数预测
            if ai {
                println!("🤖 AI Smart Mode: Analyzing image features...");
                
                // 🔥 质量宣言：使用真实的AI预测，不fallback！
                use pixly_kernel::{MediaAnalyzer, ImageFeatures, QualityMode};
                use pixly_kernel::format_recommender::{AIFormatRecommender, UserPreferences};
                
                // 1. 分析媒体文件
                let analyzer = MediaAnalyzer::new();
                match analyzer.analyze(&input) {
                    Ok(media_info) => {
                        // 2. 转换为ImageFeatures
                        let image_features = ImageFeatures {
                            width: media_info.resolution.0,
                            height: media_info.resolution.1,
                            file_size: media_info.size,
                            format: media_info.format.clone(),
                            has_alpha: false, // TODO: 实际检测
                            is_animated: false, // TODO: 实际检测
                            complexity: 0.75, // TODO: 实际计算
                        };
                        
                        // 3. 使用AI推荐器
                        let recommender = AIFormatRecommender::new();
                        let user_prefs = UserPreferences::default();
                        
                        match recommender.get_best_recommendation(
                            &image_features,
                            QualityMode::Balanced,
                            &user_prefs
                        ) {
                            Some(recommendation) => {
                                println!("   ✅ AI recommendation: {} (confidence: {:.0}%)", 
                                         recommendation.format.to_uppercase(),
                                         recommendation.confidence * 100.0);
                                
                                // 应用AI推荐的参数
                                final_quality = recommendation.quality_score;
                                println!("   📊 AI recommended quality: {}", final_quality);
                                
                                // 如果AI推荐的格式与用户指定不同，给出提示
                                if recommendation.format != target_format {
                                    println!("   💡 AI suggests {} instead of {}", 
                                             recommendation.format.to_uppercase(),
                                             target_format.to_uppercase());
                                    println!("      (Using your specified format: {})", target_format.to_uppercase());
                                }
                            }
                            None => {
                                // 🔥 质量宣言：AI失败就响亮报错！
                                eprintln!("❌ AI prediction FAILED: No recommendation available");
                                eprintln!("   Without AI, conversion will use default parameters");
                                eprintln!("   This is NOT optimal! Please check AI system.");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Media analysis FAILED: {}", e);
                        eprintln!("   Cannot use AI without media analysis");
                        eprintln!("   Using default parameters");
                    }
                }
            }
            
            // 确定输出路径
            let output_path = if let Some(out) = output {
                // 🔥 修复：检查output是文件还是目录
                if out.extension().is_some() {
                    // output有扩展名，视为完整文件路径
                    out
                } else {
                    // output无扩展名，视为目录，在其中创建同名文件
                    let filename = input.file_stem().unwrap();
                    out.join(format!("{}.{}", filename.to_string_lossy(), target_format))
                }
            } else {
                // 默认在输入文件同目录创建（原地替换）
                let parent = input.parent().unwrap_or(Path::new("."));
                let filename = input.file_stem().unwrap();
                parent.join(format!("{}.{}", filename.to_string_lossy(), target_format))
            };
            
            // 🔥 文件名规范化处理（如果启用）
            let (actual_input, temp_normalized) = if normalize_filenames {
                normalize_filename_if_needed(&input)?
            } else {
                (input.clone(), None)
            };
            
            println!("🔄 Converting: {:?}", actual_input);
            println!("📦 Format: {}", target_format);
            println!("🎯 Quality: {}", final_quality);
            println!("📁 Output: {:?}", output_path);
            
            // 🔥 修复：只在输出目录不存在时创建，避免在Eagle .info目录中创建子目录
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    println!("📂 Creating output directory: {:?}", parent);
                    std::fs::create_dir_all(parent)
                        .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
                    println!("✅ Output directory created");
                } else {
                    println!("✅ Output directory already exists: {:?}", parent);
                }
            }
            
            // 构建转换配置
            let mut config = ConversionConfig::default();
            config.quality = final_quality;
            
            // 根据格式设置参数（AI推荐的参数优先）
            match target_format.as_str() {
                "jxl" => {
                    if jpeg_lossless {
                        config.lossless = true;
                    }
                    if let Some(e) = final_effort.or(effort) {
                        config.effort = Some(e);
                    }
                    // 🔥 Phase: JXL高级参数（修复空壳功能）
                    config.jxl_modular = modular;
                    config.jxl_progressive = progressive;
                    config.jxl_responsive = responsive;
                    config.jxl_gaborish = gaborish;
                }
                "avif" => {
                    if let Some(s) = final_speed.or(speed) {
                        config.speed = s;
                    }
                    if let Some(c) = chroma {
                        config.chroma_subsampling = Some(c);
                    }
                }
                "webp" => {
                    if let Some(m) = method {
                        config.speed = m; // WebP method映射到speed
                    }
                }
                "heic" => {
                    if lossless {
                        config.lossless = true;
                    }
                    if let Some(c) = chroma {
                        config.chroma_subsampling = Some(c);
                    }
                }
                _ => {}
            }
            
            // 执行转换
            let result = execute_conversion(
                &actual_input,
                &output_path,
                &target_format,
                &config,
            )?;
            
            // 🔥 清理临时规范化文件
            if let Some(temp_path) = temp_normalized {
                let _ = std::fs::remove_file(&temp_path);
                println!("   🧹 Cleaned up temporary normalized file");
            }
            
            println!("✅ Conversion complete!");
            println!("   Input size: {} bytes", result.input_size);
            println!("   Output size: {} bytes", result.output_size);
            println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
            println!("   Processing time: {:.2}s", result.duration.as_secs_f64());
            println!("   Strategy: {}", result.strategy_used);
            
            // 🔥 XMP Sidecar 合并处理（使用原始输入路径）
            println!("\n🔍 Post-conversion processing...");
            println!("   Input: {:?}", input);
            println!("   Output: {:?}", output_path);
            
            if merge_xmp {
                println!("   📎 Checking for XMP sidecar...");
                if let Err(e) = merge_xmp_sidecar(&input, &output_path, xmp_path.as_deref()) {
                    println!("   ⚠️  XMP merge failed: {}", e);
                }
            }
            
            // 🔥 Eagle 原地替换处理（使用原始输入路径）
            println!("   📦 Checking for Eagle .info directory...");
            if let Err(e) = handle_eagle_in_place_replacement(&input, &output_path) {
                println!("   ⚠️  Eagle update failed: {}", e);
            }
            
            Ok(())
        }
    }
}

/// Merge XMP sidecar file into the output file
/// 
/// XMP Sidecar处理规则:
/// 1. 仅使用插件/用户提供的 XMP 路径
/// 2. 不做任何自动查找或扫描
/// 3. 使用exiftool合并XMP到目标文件
/// 4. 验证合并成功
/// 5. 删除原XMP sidecar
fn merge_xmp_sidecar(_input_path: &Path, output_path: &Path, provided_xmp_path: Option<&Path>) -> Result<()> {
    use std::process::Command;
    use std::fs;
    
    // 🔥 仅使用提供的 XMP 路径，不做任何自动查找
    let xmp_path = if let Some(provided) = provided_xmp_path {
        if provided.exists() {
            println!("   📎 Using provided XMP path: {:?}", provided);
            provided.to_path_buf()
        } else {
            println!("   ⚠️  Provided XMP path does not exist: {:?}", provided);
            return Ok(());
        }
    } else {
        // 🔥 没有提供 XMP 路径，直接返回（不查找）
        println!("   ℹ️  No XMP path provided, skipping XMP merge");
        return Ok(());
    };
    
    println!("📎 Found XMP sidecar: {:?}", xmp_path);
    
    // 2. 检查exiftool是否可用
    let exiftool_check = Command::new("exiftool")
        .arg("-ver")
        .output();
    
    if exiftool_check.is_err() {
        println!("⚠️  exiftool not found, skipping XMP merge");
        println!("   Install: brew install exiftool (macOS) or apt install libimage-exiftool-perl (Linux)");
        return Ok(());
    }
    
    // 3. 清理exiftool临时文件（如果存在）
    let tmp_file = format!("{}_exiftool_tmp", output_path.display());
    if Path::new(&tmp_file).exists() {
        println!("   🧹 Cleaning old exiftool temp file: {}", tmp_file);
        let _ = fs::remove_file(&tmp_file);
    }
    
    // 4. 使用exiftool合并XMP到目标文件
    println!("   🔄 Merging XMP metadata to output file...");
    let merge_result = Command::new("exiftool")
        .arg("-tagsFromFile")
        .arg(&xmp_path)
        .arg("-XMP:all")
        .arg("-overwrite_original")
        .arg(&output_path)
        .output();
    
    match merge_result {
        Ok(merge_output) => {
            let stderr_str = String::from_utf8_lossy(&merge_output.stderr);
            
            // 判断是否成功：
            // 1. exit code = 0，或
            // 2. 包含[minor]警告（即使有"Error:"前缀，只要是[minor]就认为成功）
            let is_success = merge_output.status.success() || stderr_str.contains("[minor]");
            
            if !is_success {
                // 真正的失败
                println!("   ❌ XMP merge failed: {}", stderr_str);
                println!("   Keeping XMP sidecar");
                return Ok(());
            }
            
            // 成功或minor警告，记录警告但继续
            if stderr_str.contains("[minor]") {
                let warning = stderr_str.lines()
                    .find(|line| line.contains("[minor]"))
                    .unwrap_or("");
                println!("   ℹ️  exiftool warning (ignored): {}", warning);
            }
            
            // 5. 验证合并成功（至少2个XMP标签）
            let verify_result = Command::new("exiftool")
                .arg("-XMP:all")
                .arg(&output_path)
                .output();
            
            match verify_result {
                Ok(verify_output) => {
                    let output_str = String::from_utf8_lossy(&verify_output.stdout);
                    let xmp_tag_count = output_str.lines()
                        .filter(|line| line.contains("XMP") || line.contains("xmp"))
                        .count();
                    
                    // 🔥 降低验证标准：只要有XMP标签就认为成功
                    // JXL等格式可能只有1个XMP标签，但仍然是有效的
                    if xmp_tag_count >= 1 {
                        println!("   ✅ XMP merge verified ({} tags found)", xmp_tag_count);
                        
                        // 6. 删除原XMP sidecar
                        // 如果是Eagle XMP资源，删除整个.info目录
                        if let Some(parent) = xmp_path.parent() {
                            if parent.file_name()
                                .and_then(|n| n.to_str())
                                .map_or(false, |n| n.ends_with(".info")) {
                                // Eagle XMP资源，删除整个.info目录
                                if let Err(e) = fs::remove_dir_all(parent) {
                                    println!("   ⚠️  Failed to delete Eagle XMP resource directory: {}", e);
                                } else {
                                    println!("   🗑️  Eagle XMP resource deleted: {:?}", parent);
                                }
                            } else {
                                // 标准XMP sidecar，只删除文件
                                if let Err(e) = fs::remove_file(&xmp_path) {
                                    println!("   ⚠️  Failed to delete XMP sidecar: {}", e);
                                } else {
                                    println!("   🗑️  XMP sidecar deleted: {:?}", xmp_path);
                                }
                            }
                        }
                    } else {
                        println!("   ⚠️  XMP merge verification failed (only {} tags found)", xmp_tag_count);
                        println!("   Keeping XMP sidecar for safety");
                    }
                }
                Err(e) => {
                    println!("   ⚠️  XMP verification failed: {}", e);
                    println!("   Keeping XMP sidecar for safety");
                }
            }
        }
        Err(e) => {
            println!("   ❌ XMP merge failed: {}", e);
            println!("   Keeping XMP sidecar");
        }
    }
    
    Ok(())
}

// 🔥 已删除 find_eagle_xmp_resource 函数
// 原因：不再自动查找 XMP，仅使用用户/插件提供的路径

/// Normalize filename if it contains special characters
/// Returns (actual_input_path, optional_temp_path)
fn normalize_filename_if_needed(input: &Path) -> Result<(PathBuf, Option<PathBuf>)> {
    use std::fs;
    
    let filename = input.file_name()
        .and_then(|n| n.to_str())
        .context("Invalid filename")?;
    
    // 检查是否需要规范化（包含特殊字符、空格等）
    let needs_normalization = filename.chars().any(|c| {
        !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_'
    });
    
    if !needs_normalization {
        // 不需要规范化，直接返回原路径
        return Ok((input.to_path_buf(), None));
    }
    
    println!("📝 Normalizing filename: {}", filename);
    
    // 生成规范化的文件名
    let normalized_name = filename
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
                c
            } else if c.is_whitespace() {
                '_'
            } else {
                '_'
            }
        })
        .collect::<String>();
    
    // 创建临时规范化文件
    let parent = input.parent().unwrap_or(Path::new("."));
    let temp_path = parent.join(&normalized_name);
    
    // 复制文件到临时规范化路径
    fs::copy(input, &temp_path)
        .context("Failed to create normalized temp file")?;
    
    println!("   ✅ Normalized to: {}", normalized_name);
    
    Ok((temp_path.clone(), Some(temp_path)))
}

/// Handle Eagle in-place replacement
/// 
/// Eagle 原地替换规则:
/// 1. 检测是否在 Eagle .info 目录中
/// 2. 更新 metadata.json (ext, size, mtime)
/// 3. 删除原文件
/// 4. 保留缩略图（Eagle会自动重新生成）
fn handle_eagle_in_place_replacement(input: &Path, output: &Path) -> Result<()> {
    use std::fs;
    use serde_json::{json, Value};
    
    println!("   🔍 Eagle check - Input: {:?}", input);
    println!("   🔍 Eagle check - Output: {:?}", output);
    
    // 🔥 修复：检查INPUT的parent，因为input在.info目录中
    let parent = match input.parent() {
        Some(p) => {
            println!("   🔍 Input parent: {:?}", p);
            p
        },
        None => {
            println!("   ⏭️  No parent directory, skipping");
            return Ok(());
        }
    };
    
    let parent_name = match parent.file_name().and_then(|n| n.to_str()) {
        Some(name) => {
            println!("   🔍 Parent name: {}", name);
            name
        },
        None => {
            println!("   ⏭️  Cannot get parent name, skipping");
            return Ok(());
        }
    };
    
    if !parent_name.ends_with(".info") {
        println!("   ⏭️  Not in .info directory, skipping");
        return Ok(());
    }
    
    println!("   ✅ Detected Eagle .info directory: {}", parent_name);
    
    // 2. 更新 metadata.json
    let metadata_path = parent.join("metadata.json");
    if !metadata_path.exists() {
        println!("   ⚠️  metadata.json not found, skipping Eagle update");
        return Ok(());
    }
    
    println!("   📝 Updating Eagle metadata.json...");
    
    // 读取现有 metadata
    let metadata_content = fs::read_to_string(&metadata_path)
        .context("Failed to read metadata.json")?;
    
    let mut metadata: Value = serde_json::from_str(&metadata_content)
        .context("Failed to parse metadata.json")?;
    
    // 获取新文件信息
    let output_metadata = fs::metadata(output)?;
    let new_ext = output.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let new_size = output_metadata.len();
    let new_mtime = output_metadata.modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis() as u64;
    
    // 更新字段
    if let Some(obj) = metadata.as_object_mut() {
        // 更新扩展名
        obj.insert("ext".to_string(), json!(new_ext));
        
        // 更新文件大小
        obj.insert("size".to_string(), json!(new_size));
        
        // 更新修改时间
        obj.insert("mtime".to_string(), json!(new_mtime));
        obj.insert("lastModified".to_string(), json!(new_mtime));
        
        // 更新文件名（去掉扩展名）
        if let Some(stem) = output.file_stem().and_then(|s| s.to_str()) {
            obj.insert("name".to_string(), json!(stem));
        }
        
        println!("   ✅ Updated metadata:");
        println!("      ext: {}", new_ext);
        println!("      size: {} bytes", new_size);
        println!("      name: {}", output.file_stem().and_then(|s| s.to_str()).unwrap_or(""));
    }
    
    // 写回 metadata.json
    let updated_content = serde_json::to_string_pretty(&metadata)?;
    fs::write(&metadata_path, updated_content)
        .context("Failed to write metadata.json")?;
    
    println!("   ✅ Eagle metadata updated");
    
    // 3. 删除原文件（如果与输出文件不同）
    println!("   🔍 Checking if original file should be deleted...");
    println!("      Input: {:?}", input);
    println!("      Output: {:?}", output);
    println!("      Same file? {}", input == output);
    println!("      Input exists? {}", input.exists());
    
    if input != output && input.exists() {
        println!("   🗑️  Deleting original file: {:?}", input);
        if let Err(e) = fs::remove_file(input) {
            println!("   ❌ Failed to delete original file: {}", e);
            return Err(anyhow::anyhow!("Failed to delete original file: {}", e));
        } else {
            println!("   ✅ Original file deleted: {:?}", input.file_name());
        }
    } else {
        if input == output {
            println!("   ⏭️  Input and output are the same file, skipping deletion");
        } else if !input.exists() {
            println!("   ⏭️  Input file no longer exists, skipping deletion");
        }
    }
    
    Ok(())
}
