// 质量报告器 - 生成完整的质量分析报告
// 提取自: @archive/go/quality/reporter.go

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub file_path: String,
    pub file_size: i64,
    pub format: String,
    pub media_type: String,
    pub width: u32,
    pub height: u32,
    pub pixel_count: i64,
    pub has_alpha: bool,
    pub pixel_format: String,
    pub bit_depth: u8,
    pub color_space: String,
    pub bytes_per_pixel: f64,
    pub estimated_quality: u8,
    pub complexity_score: f64,
    pub noise_level: f64,
    pub content_type: String,
    pub compression_potential: f64,
    pub is_already_compressed: bool,
    pub compression_ratio: f64,
    pub quality_class: String,
    pub size_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub extremely_high: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub extremely_low: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStats {
    pub format: String,
    pub file_count: usize,
    pub avg_saving: f64,
    pub best_saving: f64,
    pub worst_saving: f64,
    pub avg_bpp: f64,
    pub total_before: i64,
    pub total_after: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub session_id: String,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub duration: Option<Duration>,
    pub total_files: usize,
    pub quality_distribution: QualityDistribution,
    pub avg_bytes_per_pixel_before: f64,
    pub avg_bytes_per_pixel_after: f64,
    pub compression_effectiveness: HashMap<String, CompressionStats>,
    pub format_distribution_source: HashMap<String, usize>,
    pub format_distribution_target: HashMap<String, usize>,
    pub content_type_distribution: HashMap<String, usize>,
    pub quality_class_distribution: HashMap<String, usize>,
}

pub struct Reporter {
    report: QualityReport,
}

impl Reporter {
    /// 创建新的质量报告器
    pub fn new(session_id: String) -> Self {
        Self {
            report: QualityReport {
                session_id,
                start_time: SystemTime::now(),
                end_time: None,
                duration: None,
                total_files: 0,
                quality_distribution: QualityDistribution {
                    extremely_high: 0,
                    high: 0,
                    medium: 0,
                    low: 0,
                    extremely_low: 0,
                    total: 0,
                },
                avg_bytes_per_pixel_before: 0.0,
                avg_bytes_per_pixel_after: 0.0,
                compression_effectiveness: HashMap::new(),
                format_distribution_source: HashMap::new(),
                format_distribution_target: HashMap::new(),
                content_type_distribution: HashMap::new(),
                quality_class_distribution: HashMap::new(),
            },
        }
    }

    /// 添加质量指标
    pub fn add_metrics(&mut self, metrics: &QualityMetrics) {
        self.report.total_files += 1;

        // Update quality distribution
        match metrics.quality_class.as_str() {
            "Extremely High" | "极高" => self.report.quality_distribution.extremely_high += 1,
            "High" | "高" => self.report.quality_distribution.high += 1,
            "Medium" | "中" => self.report.quality_distribution.medium += 1,
            "Low" | "低" => self.report.quality_distribution.low += 1,
            "Extremely Low" | "极低" => self.report.quality_distribution.extremely_low += 1,
            _ => {}
        }
        self.report.quality_distribution.total += 1;

        // 更新格式分布
        *self.report.format_distribution_source.entry(metrics.format.clone()).or_insert(0) += 1;

        // 更新内容类型分布
        *self.report.content_type_distribution.entry(metrics.content_type.clone()).or_insert(0) += 1;

        // 更新质量类别分布
        *self.report.quality_class_distribution.entry(metrics.quality_class.clone()).or_insert(0) += 1;
    }

    /// 添加转换结果
    pub fn add_conversion_result(
        &mut self,
        format: String,
        size_before: i64,
        size_after: i64,
        bpp_before: f64,
        _bpp_after: f64,
    ) {
        let stats = self.report.compression_effectiveness
            .entry(format.clone())
            .or_insert(CompressionStats {
                format: format.clone(),
                file_count: 0,
                avg_saving: 0.0,
                best_saving: 0.0,
                worst_saving: 0.0,
                avg_bpp: 0.0,
                total_before: 0,
                total_after: 0,
            });

        stats.file_count += 1;
        stats.total_before += size_before;
        stats.total_after += size_after;

        // 计算节省率
        let saving = 1.0 - (size_after as f64 / size_before as f64);

        // 更新统计
        if stats.file_count == 1 {
            stats.avg_saving = saving;
            stats.best_saving = saving;
            stats.worst_saving = saving;
            stats.avg_bpp = bpp_before;
        } else {
            stats.avg_saving = (stats.avg_saving * (stats.file_count - 1) as f64 + saving) / stats.file_count as f64;
            if saving > stats.best_saving {
                stats.best_saving = saving;
            }
            if saving < stats.worst_saving {
                stats.worst_saving = saving;
            }
            stats.avg_bpp = (stats.avg_bpp * (stats.file_count - 1) as f64 + bpp_before) / stats.file_count as f64;
        }
    }

    /// 完成报告
    pub fn finalize(&mut self) {
        self.report.end_time = Some(SystemTime::now());
        if let Ok(duration) = self.report.end_time.unwrap().duration_since(self.report.start_time) {
            self.report.duration = Some(duration);
        }

        // 计算平均BytesPerPixel
        let mut total_bpp = 0.0;
        let count = self.report.compression_effectiveness.len();
        for stats in self.report.compression_effectiveness.values() {
            total_bpp += stats.avg_bpp;
        }
        if count > 0 {
            self.report.avg_bytes_per_pixel_before = total_bpp / count as f64;
        }
    }

    /// 保存为JSON
    pub fn save_json<P: AsRef<Path>>(&mut self, output_path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.finalize();

        // 创建目录
        if let Some(parent) = output_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        // 序列化
        let json = serde_json::to_string_pretty(&self.report)?;

        // 写入文件
        fs::write(output_path, json)?;

        Ok(())
    }

    /// 保存为文本
    pub fn save_text<P: AsRef<Path>>(&mut self, output_path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.finalize();

        // 创建目录
        if let Some(parent) = output_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        // 生成文本报告
        let text = self.generate_text_report();

        // 写入文件
        fs::write(output_path, text)?;

        Ok(())
    }

    /// 生成文本报告
    fn generate_text_report(&self) -> String {
        let mut text = String::new();

        text.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
        text.push_str("║                                                               ║\n");
        text.push_str("║   📊 Pixly Quality Analysis Report                           ║\n");
        text.push_str("║                                                               ║\n");
        text.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");

        text.push_str(&format!("Session ID: {}\n", self.report.session_id));
        
        if let Some(duration) = self.report.duration {
            text.push_str(&format!("Total time: {:?}\n", duration));
        }
        text.push_str(&format!("Total files: {}\n\n", self.report.total_files));

        // 质量分布
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text.push_str("Quality Distribution:\n");
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text.push_str(&format!("  Extremely High: {}\n", self.report.quality_distribution.extremely_high));
        text.push_str(&format!("  High:           {}\n", self.report.quality_distribution.high));
        text.push_str(&format!("  Medium:         {}\n", self.report.quality_distribution.medium));
        text.push_str(&format!("  Low:            {}\n", self.report.quality_distribution.low));
        text.push_str(&format!("  Extremely Low:  {}\n\n", self.report.quality_distribution.extremely_low));

        // Compression Effectiveness
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text.push_str("Compression Effectiveness:\n");
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        for (format, stats) in &self.report.compression_effectiveness {
            text.push_str(&format!("  {}:\n", format));
            text.push_str(&format!("    File count:   {}\n", stats.file_count));
            text.push_str(&format!("    Avg saving:   {:.1}%\n", stats.avg_saving * 100.0));
            text.push_str(&format!("    Best saving:  {:.1}%\n", stats.best_saving * 100.0));
            text.push_str(&format!("    Worst saving: {:.1}%\n", stats.worst_saving * 100.0));
            text.push_str(&format!("    Avg BPP:      {:.2}\n\n", stats.avg_bpp));
        }

        // Format Distribution
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text.push_str("Format Distribution:\n");
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text.push_str("  Source Formats:\n");
        for (format, count) in &self.report.format_distribution_source {
            text.push_str(&format!("    {}: {}\n", format, count));
        }

        text
    }

    /// 获取报告
    pub fn get_report(&self) -> &QualityReport {
        &self.report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporter_basic() {
        let mut reporter = Reporter::new("test-session".to_string());
        
        let metrics = QualityMetrics {
            file_path: "test.png".to_string(),
            file_size: 1024,
            format: "png".to_string(),
            media_type: "image".to_string(),
            width: 100,
            height: 100,
            pixel_count: 10000,
            has_alpha: true,
            pixel_format: "rgba8".to_string(),
            bit_depth: 8,
            color_space: "sRGB".to_string(),
            bytes_per_pixel: 0.1024,
            estimated_quality: 85,
            complexity_score: 0.5,
            noise_level: 0.1,
            content_type: "photo".to_string(),
            compression_potential: 0.7,
            is_already_compressed: false,
            compression_ratio: 1.0,
            quality_class: "High".to_string(),
            size_class: "Small".to_string(),
        };

        reporter.add_metrics(&metrics);
        assert_eq!(reporter.report.total_files, 1);
        assert_eq!(reporter.report.quality_distribution.high, 1);
    }

    #[test]
    fn test_compression_stats() {
        let mut reporter = Reporter::new("test-session".to_string());
        
        reporter.add_conversion_result("jxl".to_string(), 1000, 500, 0.1, 0.05);
        
        let stats = reporter.report.compression_effectiveness.get("jxl").unwrap();
        assert_eq!(stats.file_count, 1);
        assert_eq!(stats.avg_saving, 0.5);
    }
}
