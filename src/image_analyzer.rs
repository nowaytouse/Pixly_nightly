// 图像分析器 - 基于ffprobe的质量分析
// 提取自: @archive/go/quality/image_analyzer.go

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFProbeStream {
    pub width: u32,
    pub height: u32,
    pub pix_fmt: String,
    pub color_space: Option<String>,
    pub codec_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFProbeData {
    pub streams: Vec<FFProbeStream>,
}

#[derive(Debug, Clone)]
pub struct ImageQualityMetrics {
    pub file_path: String,
    pub file_size: i64,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub pixel_count: i64,
    pub pixel_format: String,
    pub bit_depth: u8,
    pub color_space: String,
    pub has_alpha: bool,
    pub bytes_per_pixel: f64,
    pub quality_class: String,
    pub estimated_quality: u8,
    pub content_type: String,
    pub compression_potential: f64,
    pub is_already_compressed: bool,
    pub size_class: String,
    pub analysis_time_ms: u64,
}

pub struct ImageAnalyzer {
    ffprobe_path: String,
}

impl Default for ImageAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageAnalyzer {
    pub fn new() -> Self {
        let ffprobe_path = which::which("ffprobe")
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "ffprobe".to_string());

        Self { ffprobe_path }
    }

    /// 分析图像文件并返回质量指标
    pub fn analyze_image<P: AsRef<Path>>(&self, file_path: P) -> Result<ImageQualityMetrics, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        let path = file_path.as_ref();

        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len() as i64;

        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let probe_data = self.ffprobe_image(path)?;

        let mut metrics = ImageQualityMetrics {
            file_path: path.to_string_lossy().to_string(),
            file_size,
            format: format.clone(),
            width: 0,
            height: 0,
            pixel_count: 0,
            pixel_format: String::new(),
            bit_depth: 8,
            color_space: "sRGB".to_string(),
            has_alpha: false,
            bytes_per_pixel: 0.0,
            quality_class: String::new(),
            estimated_quality: 0,
            content_type: String::new(),
            compression_potential: 0.0,
            is_already_compressed: false,
            size_class: String::new(),
            analysis_time_ms: 0,
        };

        if let Some(stream) = probe_data.streams.first() {
            metrics.width = stream.width;
            metrics.height = stream.height;
            metrics.pixel_format = stream.pix_fmt.clone();
            metrics.bit_depth = self.estimate_bit_depth(&stream.pix_fmt);
            metrics.color_space = self.detect_color_space(stream);
            metrics.has_alpha = stream.pix_fmt.contains('a') || stream.pix_fmt.contains("rgba");
        }

        if metrics.width > 0 && metrics.height > 0 {
            metrics.pixel_count = (metrics.width as i64) * (metrics.height as i64);
        }

        if metrics.pixel_count > 0 {
            metrics.bytes_per_pixel = file_size as f64 / metrics.pixel_count as f64;
        }

        metrics.quality_class = self.classify_quality(metrics.bytes_per_pixel, &format);
        metrics.estimated_quality = self.estimate_quality(&metrics);
        metrics.content_type = self.detect_content_type(&metrics);
        metrics.compression_potential = self.assess_compression_potential(&metrics);
        metrics.is_already_compressed = metrics.bytes_per_pixel < 0.5;
        metrics.size_class = self.classify_size(file_size);
        metrics.analysis_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(metrics)
    }

    fn ffprobe_image<P: AsRef<Path>>(&self, file_path: P) -> Result<FFProbeData, Box<dyn std::error::Error>> {
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_streams",
                file_path.as_ref().to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            return Err("ffprobe执行失败".into());
        }

        let data: FFProbeData = serde_json::from_slice(&output.stdout)?;
        Ok(data)
    }

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

    fn estimate_quality(&self, metrics: &ImageQualityMetrics) -> u8 {
        let bpp = metrics.bytes_per_pixel;

        let quality = match metrics.format.as_str() {
            "jpg" | "jpeg" => (bpp / 3.0) * 100.0,
            "png" => (bpp / 5.0) * 100.0,
            _ => (bpp / 4.0) * 100.0,
        };

        quality.clamp(0.0, 100.0) as u8
    }

    fn detect_content_type(&self, metrics: &ImageQualityMetrics) -> String {
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

    fn assess_compression_potential(&self, metrics: &ImageQualityMetrics) -> f64 {
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

    fn classify_size(&self, file_size: i64) -> String {
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

    fn detect_color_space(&self, stream: &FFProbeStream) -> String {
        if let Some(ref color_space) = stream.color_space {
            if color_space.contains("bt709") {
                return "BT.709".to_string();
            } else if color_space.contains("bt2020") {
                return "BT.2020".to_string();
            }
        }
        "sRGB".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_quality() {
        let analyzer = ImageAnalyzer::new();
        
        assert_eq!(analyzer.classify_quality(4.0, "png"), "Extremely High");
        assert_eq!(analyzer.classify_quality(2.5, "png"), "High");
        assert_eq!(analyzer.classify_quality(1.0, "png"), "Medium");
        assert_eq!(analyzer.classify_quality(0.3, "png"), "Low");
    }

    #[test]
    fn test_classify_size() {
        let analyzer = ImageAnalyzer::new();
        
        assert_eq!(analyzer.classify_size(60 * 1024 * 1024), "Extremely Large");
        assert_eq!(analyzer.classify_size(15 * 1024 * 1024), "Large");
        assert_eq!(analyzer.classify_size(2 * 1024 * 1024), "Medium");
        assert_eq!(analyzer.classify_size(500 * 1024), "Small");
        assert_eq!(analyzer.classify_size(50 * 1024), "Extremely Small");
    }

    #[test]
    fn test_estimate_bit_depth() {
        let analyzer = ImageAnalyzer::new();
        
        assert_eq!(analyzer.estimate_bit_depth("yuv420p10le"), 10);
        assert_eq!(analyzer.estimate_bit_depth("yuv420p12le"), 12);
        assert_eq!(analyzer.estimate_bit_depth("yuv420p16le"), 16);
        assert_eq!(analyzer.estimate_bit_depth("yuv420p"), 8);
    }
}
