/*
🔄 PIXLY v3.1 格式转换器

专门的格式转换逻辑，支持：
- JXL ↔ 多种格式转换
- AVIF ↔ 多种格式转换  
- WebP ↔ 多种格式转换
- 智能参数映射
- 格式特定优化

确保每种格式都能获得最佳的转换参数
*/

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use super::tool_manager::{ConversionTool, ToolManager};
use super::{ConversionRequest, ConversionResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConverter {
    supported_conversions: HashMap<String, Vec<String>>,
    optimal_tools: HashMap<(String, String), ConversionTool>,
    format_defaults: HashMap<String, FormatDefaults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FormatDefaults {
    quality_range: (u8, u8),
    default_quality: u8,
    effort_range: (u8, u8),
    default_effort: u8,
    supports_lossless: bool,
    optimal_distance: Option<f32>,
}

impl FormatConverter {
    pub fn new() -> Self {
        let mut converter = Self {
            supported_conversions: HashMap::new(),
            optimal_tools: HashMap::new(),
            format_defaults: HashMap::new(),
        };
        
        converter.initialize_format_support();
        converter.initialize_optimal_tools();
        converter.initialize_format_defaults();
        
        converter
    }
    
    fn initialize_format_support(&mut self) {
        // JXL支持的转换
        self.supported_conversions.insert("jxl".to_string(), vec![
            "jpeg".to_string(), "png".to_string(), "webp".to_string(),
            "avif".to_string(), "bmp".to_string(), "tiff".to_string()
        ]);
        
        // AVIF支持的转换
        self.supported_conversions.insert("avif".to_string(), vec![
            "jpeg".to_string(), "png".to_string(), "webp".to_string(),
            "jxl".to_string(), "bmp".to_string()
        ]);
        
        // WebP支持的转换
        self.supported_conversions.insert("webp".to_string(), vec![
            "jpeg".to_string(), "png".to_string(), "jxl".to_string(),
            "avif".to_string(), "bmp".to_string()
        ]);
        
        // 传统格式
        for format in ["jpeg", "png", "bmp", "tiff"] {
            self.supported_conversions.insert(format.to_string(), vec![
                "jxl".to_string(), "avif".to_string(), "webp".to_string(),
                "png".to_string(), "jpeg".to_string()
            ]);
        }
    }
    
    fn initialize_optimal_tools(&mut self) {
        // JXL转换的最佳工具
        self.optimal_tools.insert(("jpeg".to_string(), "jxl".to_string()), ConversionTool::CJXL);
        self.optimal_tools.insert(("png".to_string(), "jxl".to_string()), ConversionTool::CJXL);
        
        // AVIF转换的最佳工具
        self.optimal_tools.insert(("jpeg".to_string(), "avif".to_string()), ConversionTool::AVIFENC);
        self.optimal_tools.insert(("png".to_string(), "avif".to_string()), ConversionTool::AVIFENC);
        
        // WebP转换的最佳工具
        self.optimal_tools.insert(("jpeg".to_string(), "webp".to_string()), ConversionTool::CWEBP);
        self.optimal_tools.insert(("png".to_string(), "webp".to_string()), ConversionTool::CWEBP);
        
        // 通用转换工具
        self.optimal_tools.insert(("jxl".to_string(), "jpeg".to_string()), ConversionTool::DJXL);
        self.optimal_tools.insert(("avif".to_string(), "jpeg".to_string()), ConversionTool::AVIFDEC);
        self.optimal_tools.insert(("webp".to_string(), "jpeg".to_string()), ConversionTool::DWEBP);
    }
    
    fn initialize_format_defaults(&mut self) {
        // JXL默认设置
        self.format_defaults.insert("jxl".to_string(), FormatDefaults {
            quality_range: (70, 100),
            default_quality: 90,
            effort_range: (1, 9),
            default_effort: 7,
            supports_lossless: true,
            optimal_distance: Some(1.0),
        });
        
        // AVIF默认设置
        self.format_defaults.insert("avif".to_string(), FormatDefaults {
            quality_range: (60, 100),
            default_quality: 85,
            effort_range: (0, 10),
            default_effort: 6,
            supports_lossless: true,
            optimal_distance: None,
        });
        
        // WebP默认设置
        self.format_defaults.insert("webp".to_string(), FormatDefaults {
            quality_range: (60, 100),
            default_quality: 85,
            effort_range: (0, 6),
            default_effort: 4,
            supports_lossless: true,
            optimal_distance: None,
        });
        
        // JPEG默认设置
        self.format_defaults.insert("jpeg".to_string(), FormatDefaults {
            quality_range: (60, 100),
            default_quality: 85,
            effort_range: (1, 5),
            default_effort: 3,
            supports_lossless: false,
            optimal_distance: None,
        });
        
        // PNG默认设置
        self.format_defaults.insert("png".to_string(), FormatDefaults {
            quality_range: (1, 9), // PNG compression level
            default_quality: 6,
            effort_range: (1, 9),
            default_effort: 6,
            supports_lossless: true,
            optimal_distance: None,
        });
    }
    
    pub fn is_conversion_supported(&self, source: &str, target: &str) -> bool {
        self.supported_conversions
            .get(source)
            .map(|targets| targets.contains(&target.to_string()))
            .unwrap_or(false)
    }
    
    pub fn get_optimal_tool(&self, source: &str, target: &str) -> Option<ConversionTool> {
        self.optimal_tools.get(&(source.to_string(), target.to_string())).cloned()
    }
    
    pub fn validate_conversion_request(&self, request: &ConversionRequest) -> Result<()> {
        let source_ext = self.extract_extension(&request.input_path)?;
        let target_ext = &request.target_format;
        
        // 仅验证转换支持，不修改任何参数
        if !self.is_conversion_supported(&source_ext, target_ext) {
            return Err(anyhow!("不支持的转换: {} -> {}", source_ext, target_ext));
        }
        
        // 验证参数范围（但不修改）
        if let Some(defaults) = self.format_defaults.get(target_ext) {
            if request.quality < defaults.quality_range.0 || request.quality > defaults.quality_range.1 {
                return Err(anyhow!(
                    "质量参数 {} 超出格式 {} 的有效范围 [{}, {}]",
                    request.quality, target_ext, defaults.quality_range.0, defaults.quality_range.1
                ));
            }
            
            if let Some(effort) = request.effort {
                if effort < defaults.effort_range.0 || effort > defaults.effort_range.1 {
                    return Err(anyhow!(
                        "effort参数 {} 超出格式 {} 的有效范围 [{}, {}]",
                        effort, target_ext, defaults.effort_range.0, defaults.effort_range.1
                    ));
                }
            }
            
            // 验证无损模式支持
            if request.lossless && !defaults.supports_lossless {
                return Err(anyhow!("格式 {} 不支持无损模式", target_ext));
            }
        }
        
        Ok(())
    }
    
    // Rust执行层不应预设参数 - 移除此函数，参数优化应由Python AI层负责
    
    fn extract_extension(&self, path: &std::path::PathBuf) -> Result<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .ok_or_else(|| anyhow!("无法提取文件扩展名: {}", path.display()))
    }
    
    pub fn get_format_info(&self, format: &str) -> Option<&FormatDefaults> {
        self.format_defaults.get(format)
    }
    
    pub fn list_supported_formats(&self) -> Vec<&String> {
        self.supported_conversions.keys().collect()
    }
    
    pub fn get_conversion_targets(&self, source: &str) -> Option<&Vec<String>> {
        self.supported_conversions.get(source)
    }
    
    pub fn calculate_conversion_complexity(&self, source: &str, target: &str, 
                                         quality: u8, lossless: bool) -> f32 {
        let base_complexity = match (source, target) {
            // 同类型转换 (容易)
            ("jpeg", "webp") | ("webp", "jpeg") => 1.0,
            ("png", "webp") | ("webp", "png") => 1.0,
            
            // 现代格式间转换 (中等)
            ("jxl", "avif") | ("avif", "jxl") => 2.0,
            ("webp", "avif") | ("avif", "webp") => 2.0,
            
            // 到现代格式 (中等到困难)
            (_, "jxl") => 2.5,
            (_, "avif") => 3.0,
            
            // 从现代格式 (容易到中等)
            ("jxl", _) => 1.5,
            ("avif", _) => 2.0,
            
            // 默认
            _ => 2.0,
        };
        
        // 质量调整
        let quality_factor = if quality > 90 { 1.5 } else if quality < 70 { 0.8 } else { 1.0 };
        
        // 无损调整
        let lossless_factor = if lossless { 1.3 } else { 1.0 };
        
        base_complexity * quality_factor * lossless_factor
    }
    
    pub fn estimate_conversion_time(&self, source: &str, target: &str, 
                                  file_size_mb: f32, quality: u8) -> f32 {
        let complexity = self.calculate_conversion_complexity(source, target, quality, false);
        let size_factor = (file_size_mb / 10.0).max(0.5); // 10MB为基准
        
        // 基础时间（秒）
        let base_time = match target {
            "jxl" => 2.0,   // JXL编码较快
            "avif" => 8.0,  // AVIF编码慢
            "webp" => 1.5,  // WebP编码快
            _ => 3.0,
        };
        
        base_time * complexity * size_factor
    }
    
    // 推荐设置应由Python AI层负责 - Rust只执行不推荐
}

impl Default for FormatConverter {
    fn default() -> Self {
        Self::new()
    }
}
