/**
 * Audio Command - 音频转换命令处理
 * 
 * 负责:
 * - 音频格式转换 (MP3/AAC/Opus/FLAC)
 * - Python AI 参数预测集成
 * - FFmpeg 音频转换执行
 * - 编码器升级策略
 */

use std::path::Path;
use std::process::{self, Command, Stdio};
use std::time::Instant;
use serde_json;

// 🔧 统一日志系统
use tracing::{info, warn, error, debug};

/// 处理 audio 命令
pub fn handle(args: &[String]) {
    if args.len() < 2 {
        eprintln!("❌ Error: audio command requires input and output paths");
        eprintln!("Usage: pixly-rust audio <input> <output> [options]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --codec <codec>      目标编码器 (aac, opus, flac, mp3)");
        eprintln!("  --bitrate <kbps>     比特率 (64-320)");
        eprintln!("  --ai                 使用AI预测参数");
        eprintln!("  --mode <mode>        优化模式 (balanced, quality)");
        eprintln!("  --sample-rate <hz>   采样率 (22050, 44100, 48000)");
        process::exit(1);
    }
    
    let input = &args[0];
    let output = &args[1];
    
    // 解析参数
    let mut codec: Option<String> = None;
    let mut bitrate: Option<u32> = None;
    let mut use_ai = false;
    let mut mode = String::from("balanced");
    let mut sample_rate: Option<u32> = None;
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--codec" => {
                if i + 1 < args.len() {
                    codec = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("❌ Error: --codec requires a value");
                    process::exit(1);
                }
            }
            "--bitrate" => {
                if i + 1 < args.len() {
                    bitrate = Some(args[i + 1].parse().expect("Invalid bitrate value"));
                    i += 2;
                } else {
                    eprintln!("❌ Error: --bitrate requires a value");
                    process::exit(1);
                }
            }
            "--ai" => {
                use_ai = true;
                i += 1;
            }
            "--mode" => {
                if i + 1 < args.len() {
                    mode = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("❌ Error: --mode requires a value");
                    process::exit(1);
                }
            }
            "--sample-rate" => {
                if i + 1 < args.len() {
                    sample_rate = Some(args[i + 1].parse().expect("Invalid sample rate"));
                    i += 2;
                } else {
                    eprintln!("❌ Error: --sample-rate requires a value");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("❌ Unknown option: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    // 输入文件验证
    let input_path = Path::new(input);
    if !input_path.exists() {
        eprintln!("❌ Input file does not exist: {}", input);
        process::exit(1);
    }
    
    println!("🎵 Converting audio...");
    println!("   Input: {}", input);
    println!("   Output: {}", output);
    
    // 🔥 AI集成：获取转换参数
    let conversion_params = if use_ai {
        println!("🤖 AI prediction: enabled");
        match get_ai_audio_params(input, &mode) {
            Ok(params) => {
                println!("✅ AI prediction successful");
                println!("   Recommended codec: {}", params.encoder);
                println!("   Recommended bitrate: {}kbps", params.bitrate);
                println!("   Confidence: {:.1}%", params.confidence * 100.0);
                params
            }
            Err(e) => {
                eprintln!("❌ CRITICAL: AI prediction FAILED: {}", e);
                eprintln!("   🔥 NO FALLBACK AVAILABLE - Audio conversion REQUIRES AI prediction!");
                eprintln!("");
                eprintln!("   Required action:");
                eprintln!("   1. Start Go AI service: cd core/go && go run cmd/pixly-ai/main.go");
                eprintln!("   2. Verify service: curl http://localhost:50052/api/v1/version");
                eprintln!("");
                eprintln!("   📋 Project Quality Manifesto: 'Fallback代码（最高危害）- 绝对禁止！'");
                eprintln!("   📋 Reason: Fallback掩盖真实问题，让AI服务成为摆设");
                std::process::exit(1);
            }
        }
    } else {
        let output_codec = determine_output_codec(output, codec.as_deref());
        println!("✅ Using specified/default parameters");
        println!("   Codec: {}", output_codec);
        AudioConversionParams::default_for_format(&output_codec)
    };
    
    // 应用用户覆盖参数
    let final_params = AudioConversionParams {
        encoder: codec.unwrap_or(conversion_params.encoder),
        bitrate: bitrate.unwrap_or(conversion_params.bitrate),
        sample_rate: sample_rate.unwrap_or(conversion_params.sample_rate),
        channels: conversion_params.channels,
        confidence: conversion_params.confidence,
    };
    
    println!("   Final codec: {}", final_params.encoder);
    println!("   Final bitrate: {}kbps", final_params.bitrate);
    if final_params.sample_rate > 0 {
        println!("   Sample rate: {}Hz", final_params.sample_rate);
    }
    
    // 执行转换
    match execute_audio_conversion(input_path, Path::new(output), &final_params) {
        Ok(result) => {
            println!("\n✅ Audio conversion complete!");
            println!("   Original: {:.2} MB", result.original_size as f64 / 1_048_576.0);
            println!("   Converted: {:.2} MB", result.converted_size as f64 / 1_048_576.0);
            println!("   Duration: {:.2}s", result.duration);
            if result.compression_ratio > 0.0 {
                println!("   Size change: {:.1}%", (result.compression_ratio - 1.0) * 100.0);
            }
        }
        Err(e) => {
            eprintln!("\n❌ Audio conversion failed: {}", e);
            process::exit(1);
        }
    }
}

/// 音频转换参数
#[derive(Debug, Clone)]
struct AudioConversionParams {
    encoder: String,
    bitrate: u32,
    sample_rate: u32,
    channels: u32,
    confidence: f32,
}

impl AudioConversionParams {
    /// 为指定格式创建默认参数
    fn default_for_format(codec: &str) -> Self {
        match codec {
            "aac" => Self {
                encoder: "aac".to_string(),
                bitrate: 160,
                sample_rate: 44100,
                channels: 2,
                confidence: 0.5,
            },
            "opus" => Self {
                encoder: "opus".to_string(),
                bitrate: 112,
                sample_rate: 48000,
                channels: 2,
                confidence: 0.5,
            },
            "flac" => Self {
                encoder: "flac".to_string(),
                bitrate: 1000, // FLAC 无损，比特率仅参考
                sample_rate: 44100,
                channels: 2,
                confidence: 0.5,
            },
            "mp3" => Self {
                encoder: "mp3".to_string(),
                bitrate: 192,
                sample_rate: 44100,
                channels: 2,
                confidence: 0.5,
            },
            _ => Self {
                encoder: "aac".to_string(),
                bitrate: 160,
                sample_rate: 44100,
                channels: 2,
                confidence: 0.5,
            },
        }
    }
}

/// 音频转换结果
#[derive(Debug)]
struct AudioConversionResult {
    success: bool,
    original_size: u64,
    converted_size: u64,
    compression_ratio: f32,
    duration: f32,
    error: Option<String>,
}

/// 调用 Python AI 音频预测服务
fn get_ai_audio_params(input_path: &str, mode: &str) -> Result<AudioConversionParams, Box<dyn std::error::Error>> {
    // 构建 HTTP 请求到 Python 服务
    let url = "http://localhost:50052/api/v1/predict/audio";
    let payload = serde_json::json!({
        "audio_path": input_path,
        "mode": mode,
        "options": {
            "enable_format_recommendation": true,
            "enable_preprocessing": true,
            "aggressive_mode": false,
            "enable_magika": true
        }
    });
    
    info!("🔗 Connecting to Python AI audio service: {}", url);
    debug!("📋 Request payload: {}", payload);
    
    // 使用 curl 调用 (简化实现，生产环境应使用 reqwest)
    let output = Command::new("curl")
        .args([
            "-s", 
            "-X", "POST",
            "-H", "Content-Type: application/json",
            "-d", &payload.to_string(),
            url
        ])
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("HTTP request failed: {}", stderr).into());
    }
    
    let response_text = String::from_utf8_lossy(&output.stdout);
    let response: serde_json::Value = serde_json::from_str(&response_text)?;
    
    if let Some(error) = response.get("error") {
        return Err(format!("AI service error: {}", error).into());
    }
    
    // 解析响应
    let encoder = response["encoder"].as_str().unwrap_or("aac").to_string();
    let bitrate = response["bitrate"].as_u64().unwrap_or(160) as u32;
    let sample_rate = response["sample_rate"].as_u64().unwrap_or(44100) as u32;
    let channels = response["channels"].as_u64().unwrap_or(2) as u32;
    let confidence = response["confidence"].as_f64().unwrap_or(0.8) as f32;
    
    Ok(AudioConversionParams {
        encoder,
        bitrate,
        sample_rate,
        channels,
        confidence,
    })
}

/// 确定输出编码器
fn determine_output_codec(output_path: &str, user_codec: Option<&str>) -> String {
    if let Some(codec) = user_codec {
        return codec.to_string();
    }
    
    // 根据输出文件扩展名推断
    let extension = output_path.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "aac" | "m4a" => "aac".to_string(),
        "opus" | "ogg" => "opus".to_string(),
        "flac" => "flac".to_string(),
        "mp3" => "mp3".to_string(),
        _ => "aac".to_string(), // 默认 AAC
    }
}

/// 执行音频转换 (使用 FFmpeg)
fn execute_audio_conversion(
    input: &Path,
    output: &Path,
    params: &AudioConversionParams,
) -> Result<AudioConversionResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let original_size = std::fs::metadata(input)?.len();
    
    info!("🎵 Converting audio: {:?} → {:?}", input.file_name(), output.file_name());
    info!("   Codec: {} @ {}kbps", params.encoder, params.bitrate);
    
    // 构建 FFmpeg 命令
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(input)
        .arg("-y") // 覆盖输出文件
        .arg("-hide_banner")
        .arg("-loglevel").arg("error");
    
    // 设置编码器
    match params.encoder.as_str() {
        "aac" => {
            cmd.arg("-c:a").arg("aac")
               .arg("-b:a").arg(format!("{}k", params.bitrate));
        }
        "opus" => {
            cmd.arg("-c:a").arg("libopus")
               .arg("-b:a").arg(format!("{}k", params.bitrate));
        }
        "flac" => {
            cmd.arg("-c:a").arg("flac");
            // FLAC 是无损的，不需要比特率参数
        }
        "mp3" => {
            cmd.arg("-c:a").arg("libmp3lame")
               .arg("-b:a").arg(format!("{}k", params.bitrate));
        }
        _ => {
            return Err(format!("Unsupported codec: {}", params.encoder).into());
        }
    }
    
    // 设置采样率（如果指定）
    if params.sample_rate > 0 {
        cmd.arg("-ar").arg(params.sample_rate.to_string());
    }
    
    // 设置声道数
    if params.channels > 0 {
        cmd.arg("-ac").arg(params.channels.to_string());
    }
    
    // 输出文件
    cmd.arg(output);
    
    // 执行转换
    info!("🔧 Executing: ffmpeg with {} codec", params.encoder);
    let cmd_output = cmd.output()?;
    
    if !cmd_output.status.success() {
        let stderr = String::from_utf8_lossy(&cmd_output.stderr);
        return Err(format!("FFmpeg failed: {}", stderr).into());
    }
    
    let converted_size = std::fs::metadata(output)?.len();
    let compression_ratio = converted_size as f32 / original_size as f32;
    
    info!("✅ Audio conversion completed: {:.2}MB → {:.2}MB (ratio: {:.2})", 
          original_size as f32 / 1024.0 / 1024.0,
          converted_size as f32 / 1024.0 / 1024.0,
          compression_ratio);
    
    Ok(AudioConversionResult {
        success: true,
        original_size,
        converted_size,
        compression_ratio,
        duration: start_time.elapsed().as_secs_f32(),
        error: None,
    })
}
