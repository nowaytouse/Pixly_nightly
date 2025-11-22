// 🔧 外部工具依赖管理
// 检查和管理所有外部工具依赖

use std::process::Command;
use serde::{Deserialize, Serialize};

/// 外部工具类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalTool {
    // 视频处理
    FFmpeg,
    FFprobe,
    
    // 图像格式工具
    Cjxl,      // JPEG XL 编码器
    Djxl,      // JPEG XL 解码器
    Cwebp,     // WebP 编码器
    Dwebp,     // WebP 解码器
    Avifenc,   // AVIF 编码器
    Avifdec,   // AVIF 解码器
    
    // 图像优化工具
    Oxipng,    // PNG 优化器 (Rust)
    Optipng,   // PNG 优化器 (C)
    Jpegtran,  // JPEG 优化器
    Cjpegli,   // JPEG XL 的 JPEG 编码器
    Gifsicle,  // GIF 优化器
    
    // 元数据工具
    Exiftool,  // EXIF 元数据处理
}

impl ExternalTool {
    /// 获取工具名称
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
    
    /// 获取工具描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::FFmpeg => "视频/音频转换工具",
            Self::FFprobe => "视频/音频分析工具",
            Self::Cjxl => "JPEG XL 编码器",
            Self::Djxl => "JPEG XL 解码器",
            Self::Cwebp => "WebP 编码器",
            Self::Dwebp => "WebP 解码器",
            Self::Avifenc => "AVIF 编码器",
            Self::Avifdec => "AVIF 解码器",
            Self::Oxipng => "PNG 优化器 (推荐)",
            Self::Optipng => "PNG 优化器 (备选)",
            Self::Jpegtran => "JPEG 无损优化器",
            Self::Cjpegli => "JPEG XL 的 JPEG 编码器",
            Self::Gifsicle => "GIF 优化器",
            Self::Exiftool => "EXIF 元数据处理工具",
        }
    }
    
    /// 是否必需
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Exiftool)
    }
    
    /// 获取安装建议
    pub fn install_hint(&self) -> &'static str {
        match self {
            Self::FFmpeg | Self::FFprobe => "brew install ffmpeg (macOS) 或 apt install ffmpeg (Linux)",
            Self::Cjxl | Self::Djxl => "brew install jpeg-xl (macOS) 或从 https://github.com/libjxl/libjxl 编译",
            Self::Cwebp | Self::Dwebp => "brew install webp (macOS) 或 apt install webp (Linux)",
            Self::Avifenc | Self::Avifdec => "brew install libavif (macOS) 或从 https://github.com/AOMediaCodec/libavif 编译",
            Self::Oxipng => "cargo install oxipng 或 brew install oxipng (macOS)",
            Self::Optipng => "brew install optipng (macOS) 或 apt install optipng (Linux)",
            Self::Jpegtran => "brew install libjpeg (macOS) 或 apt install libjpeg-turbo-progs (Linux)",
            Self::Cjpegli => "从 https://github.com/libjxl/libjxl 编译",
            Self::Gifsicle => "brew install gifsicle (macOS) 或 apt install gifsicle (Linux)",
            Self::Exiftool => "brew install exiftool (macOS) 或 apt install libimage-exiftool-perl (Linux)",
        }
    }
    
    /// 检查工具是否可用
    pub fn is_available(&self) -> bool {
        Command::new(self.name())
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// 获取工具版本
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

/// 工具状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub required: bool,
    pub version: Option<String>,
    pub description: String,
    pub install_hint: String,
}

/// 外部工具检查器
pub struct ExternalToolChecker;

impl ExternalToolChecker {
    /// 检查所有工具
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
    
    /// 检查必需工具
    pub fn check_required() -> Vec<ToolStatus> {
        Self::check_all()
            .into_iter()
            .filter(|status| status.required)
            .collect()
    }
    
    /// 检查可选工具
    pub fn check_optional() -> Vec<ToolStatus> {
        Self::check_all()
            .into_iter()
            .filter(|status| !status.required)
            .collect()
    }
    
    /// 检查特定格式的工具
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
    
    /// 生成安装报告
    pub fn generate_install_report() -> String {
        let all_tools = Self::check_all();
        let mut report = String::new();
        
        report.push_str("🔧 外部工具依赖检查报告\n");
        report.push_str("=" .repeat(50).as_str());
        report.push_str("\n\n");
        
        // 必需工具
        report.push_str("📌 必需工具:\n");
        for tool in all_tools.iter().filter(|t| t.required) {
            let status = if tool.available { "✅" } else { "❌" };
            report.push_str(&format!("  {} {}: {}\n", status, tool.name, tool.description));
            if let Some(version) = &tool.version {
                report.push_str(&format!("     Version: {}\n", version));
            }
            if !tool.available {
                report.push_str(&format!("     Install: {}\n", tool.install_hint));
            }
        }
        
        // 可选工具
        report.push_str("\n🔧 可选工具:\n");
        for tool in all_tools.iter().filter(|t| !t.required) {
            let status = if tool.available { "✅" } else { "⚪" };
            report.push_str(&format!("  {} {}: {}\n", status, tool.name, tool.description));
            if let Some(version) = &tool.version {
                report.push_str(&format!("     Version: {}\n", version));
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
