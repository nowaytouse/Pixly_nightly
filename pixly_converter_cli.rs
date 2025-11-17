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
        
        /// Normalize filenames (handle special characters)
        #[arg(long, default_value = "false")]
        normalize_filenames: bool,
        
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
        Commands::Convert {
            input,
            format,
            quality,
            output,
            // JXL
            jpeg_lossless,
            effort,
            distance: _,
            modular: _,
            progressive: _,
            responsive: _,
            gaborish: _,
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
            normalize_filenames,
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
            println!("🎯 Quality: {}", quality);
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
            config.quality = quality;
            
            // 根据格式设置参数
            match target_format.as_str() {
                "jxl" => {
                    if jpeg_lossless {
                        config.lossless = true;
                    }
                    if let Some(e) = effort {
                        config.effort = Some(e);
                    }
                    // JXL参数通过format_specific_params传递
                }
                "avif" => {
                    if let Some(s) = speed {
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
            
            // 🔥 XMP Sidecar 合并处理
            if merge_xmp {
                merge_xmp_sidecar(&actual_input, &output_path)?;
            }
            
            // 🔥 Eagle 原地替换处理
            handle_eagle_in_place_replacement(&actual_input, &output_path)?;
            
            Ok(())
        }
    }
}

/// Merge XMP sidecar file into the output file
/// 
/// XMP Sidecar处理规则（参考 PROJECT_QUALITY_MANIFESTO.md）:
/// 1. XMP文件命名: image.xmp (去掉原扩展名，直接加.xmp)
/// 2. 使用exiftool合并XMP到目标文件
/// 3. 验证合并成功（至少2个XMP标签）
/// 4. 删除原XMP sidecar
fn merge_xmp_sidecar(input_path: &Path, output_path: &Path) -> Result<()> {
    use std::process::Command;
    use std::fs;
    
    // 1. 检测XMP sidecar (标准命名: photo.jpg -> photo.xmp)
    let xmp_path = input_path.with_extension("xmp");
    
    if !xmp_path.exists() {
        // 没有XMP文件，直接返回
        return Ok(());
    }
    
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
        Ok(merge_output) if merge_output.status.success() || {
            // 允许exiftool的[minor]警告
            let stderr_str = String::from_utf8_lossy(&merge_output.stderr);
            !merge_output.status.success() && 
            stderr_str.contains("[minor]") && 
            !stderr_str.to_lowercase().contains("error")
        } => {
            let stderr_str = String::from_utf8_lossy(&merge_output.stderr);
            
            // 记录警告但继续执行
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
                    
                    if xmp_tag_count >= 2 {
                        println!("   ✅ XMP merge verified ({} tags found)", xmp_tag_count);
                        
                        // 6. 删除原XMP sidecar
                        if let Err(e) = fs::remove_file(&xmp_path) {
                            println!("   ⚠️  Failed to delete XMP sidecar: {}", e);
                        } else {
                            println!("   🗑️  XMP sidecar deleted: {:?}", xmp_path);
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
        Ok(merge_output) => {
            let stderr = String::from_utf8_lossy(&merge_output.stderr);
            println!("   ❌ XMP merge failed: {}", stderr);
            println!("   Keeping XMP sidecar");
        }
        Err(e) => {
            println!("   ❌ XMP merge failed: {}", e);
            println!("   Keeping XMP sidecar");
        }
    }
    
    Ok(())
}

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
    
    // 1. 检测是否在 Eagle .info 目录中
    let parent = match output.parent() {
        Some(p) => p,
        None => return Ok(()), // 没有父目录，跳过
    };
    
    let parent_name = match parent.file_name().and_then(|n| n.to_str()) {
        Some(name) => name,
        None => return Ok(()), // 无法获取目录名，跳过
    };
    
    if !parent_name.ends_with(".info") {
        // 不在 .info 目录中，跳过
        return Ok(());
    }
    
    println!("📦 Detected Eagle .info directory");
    
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
    if input != output && input.exists() {
        if let Err(e) = fs::remove_file(input) {
            println!("   ⚠️  Failed to delete original file: {}", e);
        } else {
            println!("   🗑️  Original file deleted: {:?}", input.file_name());
        }
    }
    
    Ok(())
}
