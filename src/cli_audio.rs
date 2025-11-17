// 🎵 CLI Audio命令 - 音频转换
// 从 @archive/rust_broken/src/cli/commands/audio.rs 提取

use std::path::Path;
use std::process::Command;
use std::time::Instant;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AudioOptions {
    pub codec: Option<String>,
    pub bitrate: Option<u32>,
    pub use_ai: bool,
    pub mode: String,
    pub sample_rate: Option<u32>,
}

impl Default for AudioOptions {
    fn default() -> Self {
        Self {
            codec: None,
            bitrate: None,
            use_ai: false,
            mode: "balanced".to_string(),
            sample_rate: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioParams {
    pub encoder: String,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: u32,
}

impl AudioParams {
    pub fn default_for_format(codec: &str) -> Self {
        match codec {
            "aac" => Self { encoder: "aac".to_string(), bitrate: 160, sample_rate: 44100, channels: 2 },
            "opus" => Self { encoder: "opus".to_string(), bitrate: 112, sample_rate: 48000, channels: 2 },
            "flac" => Self { encoder: "flac".to_string(), bitrate: 1000, sample_rate: 44100, channels: 2 },
            "mp3" => Self { encoder: "mp3".to_string(), bitrate: 192, sample_rate: 44100, channels: 2 },
            _ => Self { encoder: "aac".to_string(), bitrate: 160, sample_rate: 44100, channels: 2 },
        }
    }
}

pub fn handle_audio(input: &str, output: &str, options: &AudioOptions) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        anyhow::bail!("Input file does not exist: {}", input);
    }
    
    println!("🎵 Converting audio...");
    println!("   Input: {}", input);
    println!("   Output: {}", output);
    
    let output_codec = determine_output_codec(output, options.codec.as_deref());
    let params = AudioParams::default_for_format(&output_codec);
    
    let final_params = AudioParams {
        encoder: options.codec.clone().unwrap_or(params.encoder),
        bitrate: options.bitrate.unwrap_or(params.bitrate),
        sample_rate: options.sample_rate.unwrap_or(params.sample_rate),
        channels: params.channels,
    };
    
    execute_audio_conversion(input_path, Path::new(output), &final_params)?;
    
    Ok(())
}

fn determine_output_codec(output_path: &str, user_codec: Option<&str>) -> String {
    if let Some(codec) = user_codec {
        return codec.to_string();
    }
    
    let extension = output_path.rsplit('.').next().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "aac" | "m4a" => "aac".to_string(),
        "opus" | "ogg" => "opus".to_string(),
        "flac" => "flac".to_string(),
        "mp3" => "mp3".to_string(),
        _ => "aac".to_string(),
    }
}

fn execute_audio_conversion(input: &Path, output: &Path, params: &AudioParams) -> Result<()> {
    let start_time = Instant::now();
    let original_size = std::fs::metadata(input)?.len();
    
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(input)
        .arg("-y")
        .arg("-hide_banner")
        .arg("-loglevel").arg("error");
    
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
        }
        "mp3" => {
            cmd.arg("-c:a").arg("libmp3lame")
               .arg("-b:a").arg(format!("{}k", params.bitrate));
        }
        _ => anyhow::bail!("Unsupported encoder: {}", params.encoder),
    }
    
    if params.sample_rate > 0 {
        cmd.arg("-ar").arg(params.sample_rate.to_string());
    }
    
    if params.channels > 0 {
        cmd.arg("-ac").arg(params.channels.to_string());
    }
    
    cmd.arg(output);
    
    let cmd_output = cmd.output()?;
    
    if !cmd_output.status.success() {
        let stderr = String::from_utf8_lossy(&cmd_output.stderr);
        anyhow::bail!("FFmpeg failed: {}", stderr);
    }
    
    let converted_size = std::fs::metadata(output)?.len();
    let duration = start_time.elapsed();
    
    println!("\n✅ Audio conversion completed!");
    println!("   Original: {:.2} MB", original_size as f64 / 1_048_576.0);
    println!("   Converted: {:.2} MB", converted_size as f64 / 1_048_576.0);
    println!("   Duration: {:.2}s", duration.as_secs_f64());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_options() {
        let options = AudioOptions::default();
        assert_eq!(options.mode, "balanced");
        assert!(!options.use_ai);
    }
    
    #[test]
    fn test_determine_codec() {
        assert_eq!(determine_output_codec("test.mp3", None), "mp3");
        assert_eq!(determine_output_codec("test.aac", None), "aac");
        assert_eq!(determine_output_codec("test.opus", None), "opus");
    }
}
