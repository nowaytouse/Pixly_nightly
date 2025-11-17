//! 🔍 质量分析器
//!
//! 综合图像和视频质量分析，提供详细质量指标
//!
//! ## 核心功能
//!
//! - **多格式支持** - JPEG、PNG、WebP、AVIF、JXL等
//! - **FFprobe集成** - 深度分析图像和视频
//! - **BPP评估** - 字节每像素质量评估
//! - **内容检测** - 照片、图形、屏幕截图分类
//! - **压缩潜力** - 评估进一步压缩的可能性

use anyhow::Result;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

/// 质量指标 - 完整的媒体文件质量评估
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    // 基础信息
    pub file_path: PathBuf,
    pub file_size: u64,
    pub format: String,
    pub media_type: String,

    // 图像特征
    pub width: u32,
    pub height: u32,
    pub pixel_count: u64,
    pub has_alpha: bool,
    pub pixel_format: String,
    pub bit_depth: u8,
    pub color_space: String,

    // 质量评估
    pub bytes_per_pixel: f64,
    pub estimated_quality: u8,
    pub complexity_score: f64,
    pub content_type: String,

    // 压缩评估
    pub compression_potential: f64,
    pub is_already_compressed: bool,

    // 分类
    pub quality_class: String,
    pub size_class: String,

    // 分析元数据
    pub analyzed_at: SystemTime,
    pub analysis_time: Duration,
    pub analysis_error: Option<String>,
}

/// FFprobe数据结构
#[derive(Debug, Deserialize)]
struct FFProbeData {
    streams: Vec<FFProbeStream>,
}

/// FFprobe流信息
#[derive(Debug, Deserialize)]
struct FFProbeStream {
    width: Option<u32>,
    height: Option<u32>,
    pix_fmt: Option<String>,
    color_space: Option<String>,
}

/// 质量分布统计
#[derive(Debug, Clone, Default)]
pub struct QualityDistribution {
    pub extremely_high: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub extremely_low: usize,
    pub total: usize,
}

/// 图像质量分析器
pub struct ImageAnalyzer {
    ffprobe_path: String,
}

/// 主质量分析器
pub struct QualityAnalyzer {
    image_analyzer: ImageAnalyzer,
    supported_image_formats: HashMap<String, bool>,
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            file_path: PathBuf::new(),
            file_size: 0,
            format: String::new(),
            media_type: String::new(),
            width: 0,
            height: 0,
            pixel_count: 0,
            has_alpha: false,
            pixel_format: String::new(),
            bit_depth: 8,
            color_space: String::new(),
            bytes_per_pixel: 0.0,
            estimated_quality: 0,
            complexity_score: 0.0,
            content_type: String::new(),
            compression_potential: 0.0,
            is_already_compressed: false,
            quality_class: String::new(),
            size_class: String::new(),
            analyzed_at: SystemTime::UNIX_EPOCH,
            analysis_time: Duration::ZERO,
            analysis_error: None,
        }
    }
}

impl Default for ImageAnalyzer {
    fn default() -> Self {
        let ffprobe_path = which::which("ffprobe")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "ffprobe".to_string());

        Self { ffprobe_path }
    }
}

impl ImageAnalyzer {
    /// 创建新的图像分析器
    pub fn new() -> Self {
        Self::default()
    }

    /// 分析图像文件并返回质量指标
    pub fn analyze_image<P: AsRef<Path>>(&self, file_path: P) -> Result<QualityMetrics> {
        let start_time = SystemTime::now();
        let path = file_path.as_ref();

        let mut metrics = QualityMetrics {
            file_path: path.to_path_buf(),
            media_type: "image".to_string(),
            analyzed_at: start_time,
            ..Default::default()
        };

        let metadata = std::fs::metadata(path)?;
        metrics.file_size = metadata.len();
        metrics.format = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        // 尝试使用ffprobe分析
        match self.ffprobe_image(path) {
            Ok(probe_data) => {
                if let Some(stream) = probe_data.streams.first() {
                    metrics.width = stream.width.unwrap_or(0);
                    metrics.height = stream.height.unwrap_or(0);
                    metrics.pixel_format = stream.pix_fmt.clone().unwrap_or_default();
                    metrics.bit_depth = self.estimate_bit_depth(&metrics.pixel_format);
                    metrics.color_space = self.detect_color_space(stream);
                    metrics.has_alpha = metrics.pixel_format.contains('a')
                        || metrics.pixel_format.contains("rgba");
                }
            }
            Err(_) => {
                // 使用image库作为fallback
                if let Err(e) = self.analyze_with_image_crate(path, &mut metrics) {
                    metrics.analysis_error = Some(format!("Analysis failed: {}", e));
                }
            }
        }

        // 计算派生指标
        if metrics.width > 0 && metrics.height > 0 {
            metrics.pixel_count = metrics.width as u64 * metrics.height as u64;
        }

        if metrics.pixel_count > 0 {
            metrics.bytes_per_pixel = metrics.file_size as f64 / metrics.pixel_count as f64;
        }

        metrics.quality_class = self.classify_quality(metrics.bytes_per_pixel, &metrics.format);
        metrics.estimated_quality = self.estimate_quality(&metrics);
        metrics.content_type = self.detect_content_type(&metrics);
        metrics.compression_potential = self.assess_compression_potential(&metrics);
        metrics.is_already_compressed = metrics.bytes_per_pixel < 0.5;
        metrics.size_class = Self::classify_size_static(metrics.file_size);
        metrics.analysis_time = start_time.elapsed().unwrap_or(Duration::ZERO);

        Ok(metrics)
    }

    /// 使用image库分析（fallback方法）
    fn analyze_with_image_crate<P: AsRef<Path>>(
        &self,
        path: P,
        metrics: &mut QualityMetrics,
    ) -> Result<()> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();

        metrics.width = width;
        metrics.height = height;
        metrics.pixel_format = match img.color() {
            image::ColorType::Rgb8 => "rgb24".to_string(),
            image::ColorType::Rgba8 => "rgba".to_string(),
            image::ColorType::L8 => "gray".to_string(),
            image::ColorType::La8 => "ya8".to_string(),
            image::ColorType::Rgb16 => "rgb48".to_string(),
            image::ColorType::Rgba16 => "rgba64".to_string(),
            image::ColorType::L16 => "gray16".to_string(),
            image::ColorType::La16 => "ya16".to_string(),
            _ => "unknown".to_string(),
        };

        metrics.has_alpha = matches!(
            img.color(),
            image::ColorType::Rgba8
                | image::ColorType::La8
                | image::ColorType::Rgba16
                | image::ColorType::La16
        );
        metrics.bit_depth = if matches!(
            img.color(),
            image::ColorType::Rgb16
                | image::ColorType::Rgba16
                | image::ColorType::L16
                | image::ColorType::La16
        ) {
            16
        } else {
            8
        };
        metrics.color_space = "sRGB".to_string();

        Ok(())
    }

    /// 使用ffprobe分析图像
    fn ffprobe_image<P: AsRef<Path>>(&self, file_path: P) -> Result<FFProbeData> {
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_streams",
                file_path.as_ref().to_str().unwrap_or(""),
            ])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("ffprobe execution failed");
        }

        let data: FFProbeData = serde_json::from_slice(&output.stdout)?;
        Ok(data)
    }

    /// 分类质量等级
    fn classify_quality(&self, bpp: f64, format: &str) -> String {
        let (high_threshold, medium_threshold, low_threshold) = match format {
            "jpg" | "jpeg" => (3.0, 1.0, 0.2),
            "png" => (4.0, 2.0, 0.5),
            _ => (3.0, 1.0, 0.3),
        };

        if bpp >= high_threshold {
            "Extremely High".to_string()
        } else if bpp >= medium_threshold {
            "High".to_string()
        } else if bpp >= low_threshold {
            "Medium".to_string()
        } else if bpp >= 0.1 {
            "Low".to_string()
        } else {
            "Extremely Low".to_string()
        }
    }

    /// 估算质量分数
    fn estimate_quality(&self, metrics: &QualityMetrics) -> u8 {
        let bpp = metrics.bytes_per_pixel;

        let quality = match metrics.format.as_str() {
            "jpg" | "jpeg" => (bpp / 3.0) * 100.0,
            "png" => (bpp / 5.0) * 100.0,
            _ => (bpp / 4.0) * 100.0,
        };

        quality.clamp(0.0, 100.0) as u8
    }

    /// 检测内容类型
    fn detect_content_type(&self, metrics: &QualityMetrics) -> String {
        let bpp = metrics.bytes_per_pixel;

        match metrics.format.as_str() {
            "jpg" | "jpeg" => {
                if bpp > 2.0 {
                    "photo".to_string()
                } else if bpp > 0.5 {
                    "mixed".to_string()
                } else {
                    "graphic".to_string()
                }
            }
            "png" => {
                if bpp > 3.0 {
                    "screenshot".to_string()
                } else if bpp > 1.5 {
                    "photo".to_string()
                } else {
                    "graphic".to_string()
                }
            }
            _ => "mixed".to_string(),
        }
    }

    /// 评估压缩潜力
    fn assess_compression_potential(&self, metrics: &QualityMetrics) -> f64 {
        let bpp = metrics.bytes_per_pixel;

        match metrics.format.as_str() {
            "png" => {
                if bpp > 3.0 {
                    0.8
                } else if bpp > 1.5 {
                    0.6
                } else {
                    0.3
                }
            }
            "jpg" | "jpeg" => {
                if bpp > 2.0 {
                    0.5
                } else if bpp > 0.8 {
                    0.3
                } else {
                    0.1
                }
            }
            _ => 0.5,
        }
    }

    /// 分类文件大小
    pub fn classify_size_static(file_size: u64) -> String {
        let size_mb = file_size as f64 / (1024.0 * 1024.0);

        if size_mb >= 50.0 {
            "Extremely Large".to_string()
        } else if size_mb >= 10.0 {
            "Large".to_string()
        } else if size_mb >= 1.0 {
            "Medium".to_string()
        } else if size_mb >= 0.1 {
            "Small".to_string()
        } else {
            "Extremely Small".to_string()
        }
    }

    /// 估算位深度
    fn estimate_bit_depth(&self, pix_fmt: &str) -> u8 {
        if pix_fmt.contains("p10") || pix_fmt.contains("10le") {
            10
        } else if pix_fmt.contains("p12") {
            12
        } else if pix_fmt.contains("p16") || pix_fmt.contains("16le") {
            16
        } else {
            8
        }
    }

    /// 检测颜色空间
    fn detect_color_space(&self, stream: &FFProbeStream) -> String {
        if let Some(color_space) = &stream.color_space {
            if color_space.contains("bt709") {
                "BT.709".to_string()
            } else if color_space.contains("bt2020") {
                "BT.2020".to_string()
            } else {
                "sRGB".to_string()
            }
        } else {
            "sRGB".to_string()
        }
    }
}

impl QualityAnalyzer {
    /// 创建新的质量分析器
    pub fn new() -> Self {
        let mut supported_image_formats = HashMap::new();
        for ext in &[
            "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "heic", "heif", "jxl",
            "avif",
        ] {
            supported_image_formats.insert(ext.to_string(), true);
        }

        Self {
            image_analyzer: ImageAnalyzer::new(),
            supported_image_formats,
        }
    }

    /// 分析文件并返回质量指标
    pub fn analyze<P: AsRef<Path>>(&self, file_path: P) -> Result<QualityMetrics> {
        let path = file_path.as_ref();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if self.supported_image_formats.contains_key(&ext) {
            return self.image_analyzer.analyze_image(path);
        }

        anyhow::bail!("Unsupported file format: {}", ext)
    }

    /// 批量分析多个文件
    pub fn analyze_batch<P: AsRef<Path>>(&self, file_paths: &[P]) -> Vec<Result<QualityMetrics>> {
        file_paths.iter().map(|path| self.analyze(path)).collect()
    }

    /// 获取质量分布统计
    pub fn get_quality_distribution(&self, metrics_list: &[QualityMetrics]) -> QualityDistribution {
        let mut distribution = QualityDistribution::default();

        for metrics in metrics_list {
            distribution.total += 1;
            match metrics.quality_class.as_str() {
                "Extremely High" | "极高" => distribution.extremely_high += 1,
                "High" | "高" => distribution.high += 1,
                "Medium" | "中" => distribution.medium += 1,
                "Low" | "低" => distribution.low += 1,
                "Extremely Low" | "极低" => distribution.extremely_low += 1,
                _ => {}
            }
        }

        distribution
    }
}

impl Default for QualityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_classification() {
        let analyzer = ImageAnalyzer::new();

        assert_eq!(analyzer.classify_quality(3.5, "jpeg"), "Extremely High");
        assert_eq!(analyzer.classify_quality(1.5, "jpeg"), "High");
        assert_eq!(analyzer.classify_quality(0.5, "jpeg"), "Medium");
        assert_eq!(analyzer.classify_quality(0.15, "jpeg"), "Low");
        assert_eq!(analyzer.classify_quality(0.05, "jpeg"), "Extremely Low");

        assert_eq!(analyzer.classify_quality(4.5, "png"), "Extremely High");
        assert_eq!(analyzer.classify_quality(2.5, "png"), "High");
        assert_eq!(analyzer.classify_quality(1.0, "png"), "Medium");
    }

    #[test]
    fn test_size_classification() {
        assert_eq!(
            ImageAnalyzer::classify_size_static(100 * 1024 * 1024),
            "Extremely Large"
        );
        assert_eq!(ImageAnalyzer::classify_size_static(20 * 1024 * 1024), "Large");
        assert_eq!(ImageAnalyzer::classify_size_static(5 * 1024 * 1024), "Medium");
        assert_eq!(ImageAnalyzer::classify_size_static(500 * 1024), "Small");
        assert_eq!(ImageAnalyzer::classify_size_static(50 * 1024), "Extremely Small");
    }

    #[test]
    fn test_content_type_detection() {
        let analyzer = ImageAnalyzer::new();
        let mut metrics = QualityMetrics {
            format: "jpeg".to_string(),
            bytes_per_pixel: 2.5,
            ..Default::default()
        };
        assert_eq!(analyzer.detect_content_type(&metrics), "photo");

        metrics.bytes_per_pixel = 0.3;
        assert_eq!(analyzer.detect_content_type(&metrics), "graphic");

        metrics.format = "png".to_string();
        metrics.bytes_per_pixel = 3.5;
        assert_eq!(analyzer.detect_content_type(&metrics), "screenshot");
    }

    #[test]
    fn test_quality_analyzer_creation() {
        let analyzer = QualityAnalyzer::new();
        assert!(analyzer.supported_image_formats.contains_key("jpg"));
        assert!(analyzer.supported_image_formats.contains_key("png"));
        assert!(analyzer.supported_image_formats.contains_key("webp"));
    }
}
