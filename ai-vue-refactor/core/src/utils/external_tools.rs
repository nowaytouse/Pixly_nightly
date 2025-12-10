// 🔧 externaldependencymanagement
// check and management has externaldependency

use std::process::Command;
use serde::{Deserialize, Serialize};

/// externaltype
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalTool {
// videoprocessing
 FFmpeg,
 FFprobe,

// imageformat
 Cjxl, // JPEG XL encodedevice
 Djxl, // JPEG XL decoder
 Cwebp, // WebP encodedevice
 Dwebp, // WebP decoder
 Avifenc, // AVIF encodedevice
 Avifdec, // AVIF decoder

// imageoptimization
 Oxipng, // PNG optimizer (Rust)
 Optipng, // PNG optimizer (C)
 Jpegtran, // JPEG optimizer
 Cjpegli, // JPEG XL JPEG encodedevice
 Gifsicle, // GIF optimizer

// elementdata
 Exiftool, // EXIF elementdataprocess
}

impl ExternalTool {
/// getname
 pub fn name(&self) -> &'static str {
 match self {
 Self::FFmpeg => "ffmpeg",
 Self::FFprobe => "ffprobe",
 Self::Cjxl => "cjxl",
 Self::Djxl => "djxl",
 Self::Cwebp => "cwebp",
 Self::Dwebp => "dwebp",
 Self::Avifenc => "avifenc",
 Self::Avifdec => "avifdec",
 Self::Oxipng => "oxipng",
 Self::Optipng => "optipng",
 Self::Jpegtran => "jpegtran",
 Self::Cjpegli => "cjpegli",
 Self::Gifsicle => "gifsicle",
 Self::Exiftool => "exiftool",
 }
 }

/// getdescription
 pub fn description(&self) -> &'static str {
 match self {
 Self::FFmpeg => "video/audioconvert",
 Self::FFprobe => "video/audioanalyze",
 Self::Cjxl => "JPEG XL encodedevice",
 Self::Djxl => "JPEG XL decoder",
 Self::Cwebp => "WebP encodedevice",
 Self::Dwebp => "WebP decoder",
 Self::Avifenc => "AVIF encodedevice",
 Self::Avifdec => "AVIF decoder",
 Self::Oxipng => "PNG optimizer (recommended)",
 Self::Optipng => "PNG optimizer ()",
 Self::Jpegtran => "JPEG optimizer",
 Self::Cjpegli => "JPEG XL JPEG encodedevice",
 Self::Gifsicle => "GIF optimizer",
 Self::Exiftool => "EXIF elementdataprocess",
 }
 }

/// isnorequired
 pub fn is_required(&self) -> bool {
 matches!(self, Self::Exiftool)
 }

/// getsuggested
 pub fn install_hint(&self) -> &'static str {
 match self {
 Self::FFmpeg | Self::FFprobe => "brew install ffmpeg (macOS) or apt install ffmpeg (Linux)",
 Self::Cjxl | Self::Djxl => "brew install jpeg-xl (macOS) orfrom https://github.com/libjxl/libjxl ",
 Self::Cwebp | Self::Dwebp => "brew install webp (macOS) or apt install webp (Linux)",
 Self::Avifenc | Self::Avifdec => "brew install libavif (macOS) orfrom https://github.com/AOMediaCodec/libavif ",
 Self::Oxipng => "cargo install oxipng or brew install oxipng (macOS)",
 Self::Optipng => "brew install optipng (macOS) or apt install optipng (Linux)",
 Self::Jpegtran => "brew install libjpeg (macOS) or apt install libjpeg-turbo-progs (Linux)",
 Self::Cjpegli => "from https://github.com/libjxl/libjxl ",
 Self::Gifsicle => "brew install gifsicle (macOS) or apt install gifsicle (Linux)",
 Self::Exiftool => "brew install exiftool (macOS) or apt install libimage-exiftool-perl (Linux)",
 }
 }

/// checkisnoavailable
 pub fn is_available(&self) -> bool {
 Command::new(self.name())
 .arg("--version")
 .output()
 .is_ok()
 }

/// getversion
 pub fn get_version(&self) -> Option<String> {
 let output = Command::new(self.name())
 .arg("--version")
 .output()
 .ok()?;

 if output.status.success() {
 let version_str = String::from_utf8_lossy(&output.stdout);
 Some(version_str.lines().next()?.to_string())
 } else {
 None
 }
 }
}

/// status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
 pub name: String,
 pub available: bool,
 pub required: bool,
 pub version: Option<String>,
 pub description: String,
 pub install_hint: String,
}

/// externalcheck
pub struct ExternalToolChecker;

impl ExternalToolChecker {
/// check has 
 pub fn check_all() -> Vec<ToolStatus> {
 let tools = [
 ExternalTool::FFmpeg,
 ExternalTool::FFprobe,
 ExternalTool::Cjxl,
 ExternalTool::Djxl,
 ExternalTool::Cwebp,
 ExternalTool::Dwebp,
 ExternalTool::Avifenc,
 ExternalTool::Avifdec,
 ExternalTool::Oxipng,
 ExternalTool::Optipng,
 ExternalTool::Jpegtran,
 ExternalTool::Cjpegli,
 ExternalTool::Gifsicle,
 ExternalTool::Exiftool,
 ];

 tools.iter().map(|tool| {
 ToolStatus {
 name: tool.name().to_string(),
 available: tool.is_available(),
 required: tool.is_required(),
 version: tool.get_version(),
 description: tool.description().to_string(),
 install_hint: tool.install_hint().to_string(),
 }
 }).collect()
 }

/// checkrequired
 pub fn check_required() -> Vec<ToolStatus> {
 Self::check_all()
 .into_iter()
 .filter(|status| status.required)
 .collect()
 }

/// checkoptional
 pub fn check_optional() -> Vec<ToolStatus> {
 Self::check_all()
 .into_iter()
 .filter(|status| !status.required)
 .collect()
 }

/// checkspecificformat
 pub fn check_format_tools(format: &str) -> Vec<ToolStatus> {
 let tools = match format.to_lowercase().as_str() {
 "jxl" | "jpegxl" => vec![ExternalTool::Cjxl, ExternalTool::Djxl],
 "webp" => vec![ExternalTool::Cwebp, ExternalTool::Dwebp],
 "avif" => vec![ExternalTool::Avifenc, ExternalTool::Avifdec],
 "png" => vec![ExternalTool::Oxipng, ExternalTool::Optipng],
 "jpeg" | "jpg" => vec![ExternalTool::Jpegtran, ExternalTool::Cjpegli],
 "gif" => vec![ExternalTool::Gifsicle],
 _ => vec![],
 };

 tools.iter().map(|tool| {
 ToolStatus {
 name: tool.name().to_string(),
 available: tool.is_available(),
 required: tool.is_required(),
 version: tool.get_version(),
 description: tool.description().to_string(),
 install_hint: tool.install_hint().to_string(),
 }
 }).collect()
 }

/// generatereport
 pub fn generate_install_report() -> String {
 let all_tools = Self::check_all();
 let mut report = String::new();

 report.push_str("🔧 externaldependencycheckreport\n");
 report.push_str("=" .repeat(50).as_str());
 report.push_str("\n\n");

// required
 report.push_str("📌 required:\n");
 for tool in all_tools.iter().filter(|t| t.required) {
 let status = if tool.available { "✅" } else { "❌" };
 report.push_str(&format!(" {} {}: {}\n", status, tool.name, tool.description));
 if let Some(version) = &tool.version {
 report.push_str(&format!(" Version: {}\n", version));
 }
 if !tool.available {
 report.push_str(&format!(" Install: {}\n", tool.install_hint));
 }
 }

// optional
 report.push_str("\n🔧 optional:\n");
 for tool in all_tools.iter().filter(|t| !t.required) {
 let status = if tool.available { "✅" } else { "⚪" };
 report.push_str(&format!(" {} {}: {}\n", status, tool.name, tool.description));
 if let Some(version) = &tool.version {
 report.push_str(&format!(" Version: {}\n", version));
 }
 }

 report
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_tool_names() {
 assert_eq!(ExternalTool::FFmpeg.name(), "ffmpeg");
 assert_eq!(ExternalTool::Cjxl.name(), "cjxl");
 assert_eq!(ExternalTool::Exiftool.name(), "exiftool");
 }

 #[test]
 fn test_required_tools() {
 assert!(ExternalTool::Exiftool.is_required());
 assert!(!ExternalTool::FFmpeg.is_required());
 }

 #[test]
 fn test_check_all() {
 let tools = ExternalToolChecker::check_all();
 assert_eq!(tools.len(), 14);
 }

 #[test]
 fn test_format_tools() {
 let webp_tools = ExternalToolChecker::check_format_tools("webp");
 assert_eq!(webp_tools.len(), 2);
 assert!(webp_tools.iter().any(|t| t.name == "cwebp"));
 assert!(webp_tools.iter().any(|t| t.name == "dwebp"));
 }
}
