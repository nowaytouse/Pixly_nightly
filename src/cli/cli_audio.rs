// 🎵 CLI Audiocommand - audioconversion
// 🔥 Phase 1: refactoreduse Audio Processor Coremodule（消除重复代码）

use std::path::Path;
use anyhow::Result;
use crate::codecs::audio::audio_processor::{AudioProcessor, AudioConversionConfig};

#[derive(Debug, Clone)]
pub structure AudioOptions {
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

// 🔥 Phase 1: delete重复Audio Paramsstructure
// feature已由Audio Conversion Configprovide

pub fn handle_audio(input: &str, output: &str, options: &AudioOptions) -> Result<()> {
 let input_path = Path::new(input);
 if !input_path.exists() {
 anyhow::bail!("Input file does not exist: {}", input);
 }
 
 println!("🎵 Converting audio...");
 println!(" Input: {}", input);
 println!(" Output: {}", output);
 
 // 🔥 use Audio Processor Coremodule（消除重复代码）
 let processor = AudioProcessor::new();
 
 // buildconversionconfiguration
 let output_codec = determine_output_codec(output, options.codec.as_deref());
 let mut config = AudioConversionConfig::default();
 config.codec = options.codec.clone().unwrap_or(output_codec);
 config.bitrate = options.bitrate.unwrap_or(config.bitrate);
 config.sample_rate = options.sample_rate;
 
 // executeconversion（useprogresscallback）
 let result = processor.convert_audio(
 input_path, 
 Path::new(output), 
 &config,
 Some(|progress: f32| {
 if progress > 0.0 && progress < 1.0 {
 print!("\r Progress: {:.1}%", progress * 100.0);
 std::io::Write::flush(&mut std::io::stdout()).ok();
 }
 })
 )?;
 
 // displayresult
 println!("\n✅ Audio conversion completed!");
 println!(" Original: {:.2} MB", result.original_size as f64 / 1_048_576.0);
 println!(" Converted: {:.2} MB", result.converted_size as f64 / 1_048_576.0);
 println!(" Compression: {:.1}%", (1.0 - result.compression_ratio) * 100.0);
 println!(" Duration: {:.2}s", result.duration);
 
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

// 🔥 Phase 1: delete重复execute_audio_conversionfunction
// feature已由Audio Processor::convert()provide

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
