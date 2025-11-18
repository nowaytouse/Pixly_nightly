// Phase 4: 智能格式选择器
// 
// 目标：解决JPEG→WebP可能增大文件的问题
// 原则：基于输入格式特征，智能判断最佳目标格式

use anyhow::Result;
use std::path::Path;

/// 格式选择建议
#[derive(Debug, Clone)]
pub struct FormatRecommendation {
    /// 推荐的目标格式
    pub recommended_format: String,
    /// 置信度 (0.0-1.0)
    pub confidence: f64,
    /// 推荐原因
    pub reason: String,
    /// 备选格式
    pub alternatives: Vec<String>,
    /// 预估文件大小变化 (负数表示减小)
    pub estimated_size_change: f64,
}

/// 智能格式选择器
pub struct FormatSelector {
    /// 是否启用激进模式（尝试更多格式）
    #[allow(dead_code)]  // Phase 4: 保留用于未来扩展
    aggressive: bool,
}

impl FormatSelector {
    pub fn new(aggressive: bool) -> Self {
        Self { aggressive }
    }
    
    /// 选择最佳目标格式
    pub fn select_best_format(
        &self,
        input_path: &Path,
        user_target: Option<&str>,
    ) -> Result<FormatRecommendation> {
        // 获取输入格式
        let input_ext = input_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // 如果用户指定了格式，验证是否合理
        if let Some(target) = user_target {
            return self.validate_user_choice(&input_ext, target);
        }
        
        // 智能选择
        self.auto_select_format(&input_ext, input_path)
    }
    
    /// 验证用户选择的格式是否合理
    fn validate_user_choice(
        &self,
        input_format: &str,
        target_format: &str,
    ) -> Result<FormatRecommendation> {
        let target = target_format.to_lowercase();
        
        // 检查是否是已知的问题组合
        let (is_risky, reason) = self.check_risky_conversion(input_format, &target);
        
        if is_risky {
            // 警告但不阻止
            Ok(FormatRecommendation {
                recommended_format: target.clone(),
                confidence: 0.5,
                reason: format!("⚠️ {}", reason),
                alternatives: self.suggest_alternatives(input_format),
                estimated_size_change: 0.1, // 可能增大10%
            })
        } else {
            Ok(FormatRecommendation {
                recommended_format: target.clone(),
                confidence: 0.9,
                reason: "用户指定格式，验证通过".to_string(),
                alternatives: vec![],
                estimated_size_change: -0.3, // 预估减小30%
            })
        }
    }
    
    /// 检查是否是风险转换
    fn check_risky_conversion(&self, input: &str, target: &str) -> (bool, String) {
        match (input, target) {
            // JPEG → WebP: 可能增大
            ("jpg" | "jpeg", "webp") => (
                true,
                "JPEG已经是有损压缩，转WebP可能增大文件。建议保持JPEG或转JXL".to_string()
            ),
            
            // JPEG → AVIF: 通常OK，但需要高质量
            ("jpg" | "jpeg", "avif") => (
                false,
                "JPEG→AVIF通常有效，但建议使用quality≥80".to_string()
            ),
            
            // PNG → JPEG: 丢失透明度
            ("png", "jpg" | "jpeg") if self.has_transparency_risk() => (
                true,
                "PNG可能包含透明度，转JPEG会丢失。建议转WebP/AVIF/JXL".to_string()
            ),
            
            // WebP → JPEG: 可能质量损失
            ("webp", "jpg" | "jpeg") => (
                true,
                "WebP→JPEG可能损失质量。建议保持WebP或转AVIF/JXL".to_string()
            ),
            
            _ => (false, String::new())
        }
    }
    
    /// 检查是否有透明度风险（简化版）
    fn has_transparency_risk(&self) -> bool {
        // 简化：假设PNG可能有透明度
        true
    }
    
    /// 自动选择最佳格式
    fn auto_select_format(
        &self,
        input_format: &str,
        _input_path: &Path,  // 未来可能用于检测透明度/动画
    ) -> Result<FormatRecommendation> {
        match input_format {
            // PNG: 优先AVIF（最佳压缩）
            "png" => Ok(FormatRecommendation {
                recommended_format: "avif".to_string(),
                confidence: 0.95,
                reason: "PNG→AVIF: 最佳压缩率（60-80%），保留透明度".to_string(),
                alternatives: vec!["webp".to_string(), "jxl".to_string()],
                estimated_size_change: -0.7, // 减小70%
            }),
            
            // JPEG: 优先JXL（无损重新包装）
            "jpg" | "jpeg" => Ok(FormatRecommendation {
                recommended_format: "jxl".to_string(),
                confidence: 0.9,
                reason: "JPEG→JXL: 无损重新包装（减小20-30%），无质量损失".to_string(),
                alternatives: vec!["avif".to_string()],
                estimated_size_change: -0.25, // 减小25%
            }),
            
            // WebP: 优先AVIF（更好的压缩）
            "webp" => Ok(FormatRecommendation {
                recommended_format: "avif".to_string(),
                confidence: 0.85,
                reason: "WebP→AVIF: 更好的压缩率（20-40%）".to_string(),
                alternatives: vec!["jxl".to_string()],
                estimated_size_change: -0.3, // 减小30%
            }),
            
            // GIF: 优先WebP（动画支持）
            "gif" => Ok(FormatRecommendation {
                recommended_format: "webp".to_string(),
                confidence: 0.9,
                reason: "GIF→WebP: 保留动画，大幅减小文件（70-90%）".to_string(),
                alternatives: vec!["avif".to_string(), "mp4".to_string()],
                estimated_size_change: -0.8, // 减小80%
            }),
            
            // AVIF: 已经是最佳格式
            "avif" => Ok(FormatRecommendation {
                recommended_format: "avif".to_string(),
                confidence: 1.0,
                reason: "AVIF已经是最佳格式，建议保持或优化参数".to_string(),
                alternatives: vec![],
                estimated_size_change: -0.1, // 优化可能减小10%
            }),
            
            // JXL: 已经是最佳格式
            "jxl" => Ok(FormatRecommendation {
                recommended_format: "jxl".to_string(),
                confidence: 1.0,
                reason: "JXL已经是最佳格式，建议保持或优化参数".to_string(),
                alternatives: vec![],
                estimated_size_change: -0.1,
            }),
            
            // 未知格式: 默认AVIF
            _ => Ok(FormatRecommendation {
                recommended_format: "avif".to_string(),
                confidence: 0.7,
                reason: format!("未知格式'{}'，默认推荐AVIF", input_format),
                alternatives: vec!["webp".to_string(), "jxl".to_string()],
                estimated_size_change: -0.5,
            }),
        }
    }
    
    /// 建议备选格式
    fn suggest_alternatives(&self, input_format: &str) -> Vec<String> {
        match input_format {
            "jpg" | "jpeg" => vec!["jxl".to_string(), "avif".to_string()],
            "png" => vec!["avif".to_string(), "webp".to_string(), "jxl".to_string()],
            "webp" => vec!["avif".to_string(), "jxl".to_string()],
            "gif" => vec!["webp".to_string(), "mp4".to_string()],
            _ => vec!["avif".to_string(), "webp".to_string()],
        }
    }
}

/// 格式兼容性检查器
pub struct FormatCompatibilityChecker;

impl FormatCompatibilityChecker {
    /// 检查格式是否支持透明度
    pub fn supports_transparency(format: &str) -> bool {
        matches!(format, "png" | "webp" | "avif" | "jxl")
    }
    
    /// 检查格式是否支持动画
    pub fn supports_animation(format: &str) -> bool {
        matches!(format, "gif" | "webp" | "avif" | "jxl")
    }
    
    /// 检查格式是否支持无损
    pub fn supports_lossless(format: &str) -> bool {
        matches!(format, "png" | "webp" | "avif" | "jxl")
    }
    
    /// 获取格式的压缩效率评分 (0-10)
    pub fn compression_efficiency(format: &str) -> u8 {
        match format {
            "avif" => 10,  // 最佳
            "jxl" => 9,
            "webp" => 8,
            "heic" => 8,
            "png" => 5,
            "jpg" | "jpeg" => 6,
            "gif" => 3,
            "bmp" => 1,
            _ => 5,
        }
    }
    
    /// 获取格式的兼容性评分 (0-10)
    pub fn compatibility_score(format: &str) -> u8 {
        match format {
            "jpg" | "jpeg" => 10,  // 最广泛支持
            "png" => 10,
            "webp" => 8,
            "gif" => 9,
            "avif" => 6,  // 较新，支持度中等
            "jxl" => 4,   // 很新，支持度较低
            "heic" => 5,
            _ => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_png_to_avif() {
        let selector = FormatSelector::new(false);
        let path = Path::new("test.png");
        let result = selector.select_best_format(path, None).unwrap();
        
        assert_eq!(result.recommended_format, "avif");
        assert!(result.confidence > 0.9);
        assert!(result.estimated_size_change < 0.0); // 应该减小
    }
    
    #[test]
    fn test_jpeg_to_jxl() {
        let selector = FormatSelector::new(false);
        let path = Path::new("test.jpg");
        let result = selector.select_best_format(path, None).unwrap();
        
        assert_eq!(result.recommended_format, "jxl");
        assert!(result.confidence > 0.8);
    }
    
    #[test]
    fn test_risky_jpeg_to_webp() {
        let selector = FormatSelector::new(false);
        let path = Path::new("test.jpg");
        let result = selector.select_best_format(path, Some("webp")).unwrap();
        
        assert_eq!(result.recommended_format, "webp");
        assert!(result.confidence < 0.7); // 低置信度
        assert!(result.reason.contains("⚠️")); // 包含警告
    }
    
    #[test]
    fn test_format_compatibility() {
        assert!(FormatCompatibilityChecker::supports_transparency("png"));
        assert!(FormatCompatibilityChecker::supports_transparency("webp"));
        assert!(!FormatCompatibilityChecker::supports_transparency("jpeg"));
        
        assert_eq!(FormatCompatibilityChecker::compression_efficiency("avif"), 10);
        assert_eq!(FormatCompatibilityChecker::compatibility_score("jpeg"), 10);
    }
}
