// 冲突检测器 - 在ML预测前检测并预警可能的转换冲突
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/predictor/conflict_detector.go

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FileFeatures {
    pub format: String,
    pub file_size: u64,
    pub width: u32,
    pub height: u32,
    pub page_count: u32,
    pub has_alpha: bool,
    pub bit_depth: u8,
    pub color_type: String,
    pub pixel_format: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub conflict_type: String,
    pub severity: ConflictSeverity,
    pub message: String,
    pub suggestion: String,
}

#[derive(Debug)]
pub struct ConflictReport {
    pub conflicts: Vec<Conflict>,
    pub has_critical: bool,
    pub can_proceed: bool,
}

#[derive(Default)]
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn new() -> Self {
        Self
    }

    /// 检测转换冲突
    pub fn check_conflicts(
        &self,
        features: &FileFeatures,
        target_format: &str,
        is_animated: bool,
        downscale_mode: Option<&str>,
        quality: u8,
        lossless: bool,
    ) -> ConflictReport {
        let mut conflicts = Vec::new();

        // 1. 格式组合冲突检测
        conflicts.extend(self.check_format_combination(features, target_format));

        // 2. 多页图片检测
        conflicts.extend(self.check_multipage(features));

        // 3. 动画降采样冲突
        conflicts.extend(self.check_animation_downscale(is_animated, downscale_mode));

        // 4. 资源消耗预警
        conflicts.extend(self.check_resource_usage(features, quality));

        // 5. 质量参数冲突
        conflicts.extend(self.check_quality_params(features, target_format, quality, lossless));

        let has_critical = conflicts.iter().any(|c| c.severity == ConflictSeverity::Critical);
        let can_proceed = conflicts.is_empty() || !has_critical;

        ConflictReport {
            conflicts,
            has_critical,
            can_proceed,
        }
    }

    /// 检测格式组合冲突
    fn check_format_combination(&self, features: &FileFeatures, target_format: &str) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // 定义有效的格式转换路径
        let mut valid_routes: HashMap<&str, Vec<&str>> = HashMap::new();
        valid_routes.insert("gif", vec!["jxl", "webp"]);
        valid_routes.insert("apng", vec!["jxl"]);

        if let Some(valid_formats) = valid_routes.get(features.format.as_str())
            && !valid_formats.contains(&target_format)
        {
            conflicts.push(Conflict {
                conflict_type: "InvalidFormatCombination".to_string(),
                severity: ConflictSeverity::Critical,
                message: format!("{} -> {} conversion not supported", features.format, target_format),
                suggestion: format!("Recommended formats: {:?}", valid_formats),
            });
        }

        conflicts
    }

    /// 检测多页图片
    fn check_multipage(&self, features: &FileFeatures) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        if features.page_count > 1 {
            conflicts.push(Conflict {
                conflict_type: "MultiPageImage".to_string(),
                severity: ConflictSeverity::Warning,
                message: format!("Detected {} page image", features.page_count),
                suggestion: "Multi-page images may only convert the first page or fail".to_string(),
            });
        }

        conflicts
    }

    /// 检测动画降采样冲突
    fn check_animation_downscale(&self, is_animated: bool, downscale_mode: Option<&str>) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        if is_animated
            && let Some(mode) = downscale_mode
            && mode != "none" && !mode.is_empty()
        {
            conflicts.push(Conflict {
                conflict_type: "AnimationDownscaleConflict".to_string(),
                severity: ConflictSeverity::Critical,
                message: "动画文件不支持降采样".to_string(),
                suggestion: "请禁用降采样或选择静态图片".to_string(),
            });
        }

        conflicts
    }

    /// 检测资源消耗预警
    fn check_resource_usage(&self, features: &FileFeatures, quality: u8) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // 超大文件 + 高质量 = OOM风险
        let file_size_mb = features.file_size as f64 / (1024.0 * 1024.0);
        if file_size_mb > 100.0 && quality > 90 {
            conflicts.push(Conflict {
                conflict_type: "OOMRisk".to_string(),
                severity: ConflictSeverity::Warning,
                message: format!(
                    "超大文件({:.1}MB) + 高质量({})可能导致内存溢出",
                    file_size_mb, quality
                ),
                suggestion: "建议降低质量参数或启用降采样".to_string(),
            });
        }

        // 超高分辨率预警
        let megapixels = (features.width * features.height) as f64 / 1_000_000.0;
        if megapixels > 50.0 {
            conflicts.push(Conflict {
                conflict_type: "HighResolution".to_string(),
                severity: ConflictSeverity::Info,
                message: format!("Ultra-high resolution image ({:.1}MP), conversion may take longer", megapixels),
                suggestion: "Consider enabling downsampling to improve conversion speed".to_string(),
            });
        }

        conflicts
    }

    /// 检测质量参数冲突
    fn check_quality_params(
        &self,
        features: &FileFeatures,
        target_format: &str,
        quality: u8,
        lossless: bool,
    ) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        // 无损模式 + 质量参数
        if lossless && quality < 100 {
            conflicts.push(Conflict {
                conflict_type: "LosslessQualityConflict".to_string(),
                severity: ConflictSeverity::Warning,
                message: "无损模式下质量参数将被忽略".to_string(),
                suggestion: "无损模式会自动使用100%质量".to_string(),
            });
        }

        // JPEG转JXL lossless特殊处理
        if (features.format == "jpeg" || features.format == "jpg") && target_format == "jxl" && !lossless {
            conflicts.push(Conflict {
                conflict_type: "JPEGLosslessOpportunity".to_string(),
                severity: ConflictSeverity::Info,
                message: "JPEG可使用--lossless_jpeg=1实现完美无损转换".to_string(),
                suggestion: "建议启用lossless_jpeg模式,可节省16-22%空间".to_string(),
            });
        }

        conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_combination_conflict() {
        let detector = ConflictDetector::new();
        let features = FileFeatures {
            format: "gif".to_string(),
            file_size: 1024,
            width: 100,
            height: 100,
            page_count: 1,
            has_alpha: false,
            bit_depth: 8,
            color_type: "rgb".to_string(),
            pixel_format: "rgb8".to_string(),
        };

        let report = detector.check_conflicts(&features, "png", false, None, 85, false);
        assert!(report.has_critical);
        assert!(!report.can_proceed);
    }

    #[test]
    fn test_animation_downscale_conflict() {
        let detector = ConflictDetector::new();
        let features = FileFeatures {
            format: "gif".to_string(),
            file_size: 1024,
            width: 100,
            height: 100,
            page_count: 1,
            has_alpha: false,
            bit_depth: 8,
            color_type: "rgb".to_string(),
            pixel_format: "rgb8".to_string(),
        };

        let report = detector.check_conflicts(&features, "jxl", true, Some("auto"), 85, false);
        assert!(report.has_critical);
    }

    #[test]
    fn test_oom_risk_warning() {
        let detector = ConflictDetector::new();
        let features = FileFeatures {
            format: "png".to_string(),
            file_size: 150 * 1024 * 1024, // 150MB
            width: 10000,
            height: 10000,
            page_count: 1,
            has_alpha: true,
            bit_depth: 8,
            color_type: "rgba".to_string(),
            pixel_format: "rgba8".to_string(),
        };

        let report = detector.check_conflicts(&features, "jxl", false, None, 95, false);
        assert!(!report.has_critical);
        assert!(report.can_proceed);
        assert!(!report.conflicts.is_empty());
    }
}
