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
            
            println!("🔄 Converting: {:?}", input);
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
                &input,
                &output_path,
                &target_format,
                &config,
            )?;
            
            println!("✅ Conversion complete!");
            println!("   Input size: {} bytes", result.input_size);
            println!("   Output size: {} bytes", result.output_size);
            println!("   Compression ratio: {:.2}%", result.compression_ratio * 100.0);
            println!("   Processing time: {:.2}s", result.duration.as_secs_f64());
            println!("   Strategy: {}", result.strategy_used);
            
            Ok(())
        }
    }
}
