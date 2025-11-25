//! formatoptimizationstrategy
//!
//! at not formatdownoptimizationimage：
//! - PNG: oxipng/optipng (lossless)
//! - JPEG: mozjpeg/jpegtran (canconfigurationquality)
//! - Web P: cwebpheavycompression

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use std::fs;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// 🚀 performanceoptimization: cachedetectionresult
static TOOL_CACHE: Lazy<Mutex<HashMap<String, Option<String>>>> = Lazy::new(|| {
 Mutex::new(HashMap::new())
});

/// formatoptimization
pub struct SameFormatOptimizer {
 preserve_metadata: bool,
}

impl SameFormatOptimizer {
 pub fn new(preserve_metadata: bool) -> Self {
 Self { preserve_metadata }
 }

/// checkspecificformatisnoavailable
/// 🚀 performanceoptimization: cachedetectionresult，heavycall
 pub fn check_tools_for_format(format: &str) -> Result<String> {
// checkcache
 {
 let cache = TOOL_CACHE.lock()
 .map_err(|e| anyhow::anyhow!("Tool cache lock poisoned: {}", e))?;
 if let Some(cached) = cache.get(format) {
 return cached.clone()
 .ok_or_else(|| anyhow::anyhow!("No optimizer found for {}", format));
 }
 }

// executedetection
 let tool = match format {
 "png" => {
 if Command::new("oxipng").arg("--version").output().is_ok() {
 Some(String::from("oxipng"))
 } else if Command::new("optipng").arg("-v").output().is_ok() {
 Some(String::from("optipng"))
 } else {
 None
 }
 }
 "jpg" | "jpeg" => {
 if Command::new("mozjpeg").arg("--version").output().is_ok() {
 Some(String::from("mozjpeg"))
 } else if Command::new("jpegtran").arg("-version").output().is_ok() {
 Some(String::from("jpegtran"))
 } else {
 None
 }
 }
 "webp" => {
 if Command::new("cwebp").arg("-version").output().is_ok() {
 Some(String::from("cwebp"))
 } else {
 None
 }
 }
 _ => None,
 };

// cacheresult
 {
 let mut cache = TOOL_CACHE.lock()
 .map_err(|e| anyhow::anyhow!("Tool cache lock poisoned: {}", e))?;
 cache.insert(format.to_string(), tool.clone());
 }

 tool.ok_or_else(|| anyhow::anyhow!("No optimizer found for {}", format))
 }

/// optimizationPNG
 pub fn optimize_png(&self, input: &Path, output: &Path, tool: &str) -> Result<()> {
 let mut cmd = Command::new(tool);

 match tool {
 "oxipng" => {
 cmd.arg("-o3") // optimizedlevel3
 .arg("--strip") // removeelementdata（）
 .arg("--preserve") // fileproperty
 .arg(input)
 .arg("-out")
 .arg(output);
 }
 "optipng" => {
 cmd.arg("-o3") // optimizedlevel3
 .arg("-preserve") // fileproperty
 .arg("-out")
 .arg(output)
 .arg(input);
 }
 _ => anyhow::bail!("Unknown PNG tool: {}", tool),
 }

 let output_result = cmd.output()
 .with_context(|| format!("Failed to execute {}", tool))?;

 if !output_result.status.success() {
 anyhow::bail!(
 "{} failed: {}",
 tool,
 String::from_utf8_lossy(&output_result.stderr)
 );
 }

 Ok(())
 }

/// optimizationJPEG
 pub fn optimize_jpeg(&self, input: &Path, output: &Path, quality: u8, tool: &str) -> Result<()> {
 let mut cmd = Command::new(tool);

 match tool {
 "mozjpeg" => {
 cmd.arg("-quality")
 .arg(quality.to_string())
 .arg("-progressive")
 .arg("-outfile")
 .arg(output)
 .arg(input);
 }
 "jpegtran" => {
 cmd.arg("-copy")
 .arg(if self.preserve_metadata { "all" } else { "none" })
 .arg("-optimize")
 .arg("-progressive")
 .arg("-outfile")
 .arg(output)
 .arg(input);
 }
 _ => anyhow::bail!("Unknown JPEG tool: {}", tool),
 }

 let output_result = cmd.output()
 .with_context(|| format!("Failed to execute {}", tool))?;

 if !output_result.status.success() {
 anyhow::bail!(
 "{} failed: {}",
 tool,
 String::from_utf8_lossy(&output_result.stderr)
 );
 }

 Ok(())
 }

/// optimizationWebP
 pub fn optimize_webp(&self, input: &Path, output: &Path, quality: u8) -> Result<()> {
 let mut cmd = Command::new("cwebp");

 cmd.arg(input)
 .arg("-o")
 .arg(output)
 .arg("-q")
 .arg(quality.to_string())
 .arg("-m")
 .arg("6") // maximumeffort
 .arg("-mt"); // multithread

 if self.preserve_metadata {
 cmd.arg("-metadata").arg("all");
 }

 let output_result = cmd.output()
 .with_context(|| "Failed to execute cwebp")?;

 if !output_result.status.success() {
 anyhow::bail!(
 "cwebp failed: {}",
 String::from_utf8_lossy(&output_result.stderr)
 );
 }

 Ok(())
 }

/// executeformatoptimization
 pub fn optimize(&self, input: &Path, output: &Path, format: &str, quality: u8) -> Result<OptimizationResult> {
 let start = Instant::now();
 let input_size = fs::metadata(input)?.len();

 let tool = Self::check_tools_for_format(format)?;

 match format {
 "png" => self.optimize_png(input, output, &tool)?,
 "jpg" | "jpeg" => self.optimize_jpeg(input, output, quality, &tool)?,
 "webp" => self.optimize_webp(input, output, quality)?,
 _ => anyhow::bail!("Unsupported format: {}", format),
 }

 let output_size = fs::metadata(output)?.len();
 let processing_time = start.elapsed();

 Ok(OptimizationResult {
 input_size,
 output_size,
 compression_ratio: output_size as f64 / input_size as f64,
 processing_time_ms: processing_time.as_millis() as u64,
 tool_used: tool,
 })
 }
}

/// optimizationresult
#[derive(Debug, Clone)]
pub struct OptimizationResult {
 pub input_size: u64,
 pub output_size: u64,
 pub compression_ratio: f64,
 pub processing_time_ms: u64,
 pub tool_used: String,
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_tool_detection() {
// test PNGdetection
 let png_result = SameFormatOptimizer::check_tools_for_format("png");
 if png_result.is_ok() {
 log::debug!("PNG tool available: {:?}", png_result.unwrap());
 }

// test JPEGdetection
 let jpeg_result = SameFormatOptimizer::check_tools_for_format("jpeg");
 if jpeg_result.is_ok() {
 log::debug!("JPEG tool available: {:?}", jpeg_result.unwrap());
 }

// test Web Pdetection
 let webp_result = SameFormatOptimizer::check_tools_for_format("webp");
 if webp_result.is_ok() {
 log::debug!("WebP tool available: {:?}", webp_result.unwrap());
 }
 }

 #[test]
 fn test_optimizer_creation() {
 let optimizer = SameFormatOptimizer::new(true);
 assert!(optimizer.preserve_metadata);

 let optimizer2 = SameFormatOptimizer::new(false);
 assert!(!optimizer2.preserve_metadata);
 }
}
