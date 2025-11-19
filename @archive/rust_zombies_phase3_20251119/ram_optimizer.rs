// 🚀 RAM优化器 - 基于分辨率的智能并发控制
// 从 @archive/rust_broken/src/converter/ram_optimizer.rs 提取并增强
//
// 核心功能:
// - 基于图片分辨率的智能并发控制
// - 多工具分辨率检测（image库、ImageMagick、exiftool、ffprobe）
// - 内存使用估算
// - 分级并发规则（超高分辨率1线程，低分辨率16线程）
// - 系统内存自适应调整

use std::process::Command;
use std::path::Path;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

/// 分辨率规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionRule {
    pub threshold_mp: f64,
    pub max_concurrency: usize,
    pub description: String,
}

/// RAM优化器
pub struct RAMOptimizer {
    rules: Vec<ResolutionRule>,
}

impl RAMOptimizer {
    pub fn new() -> Self {
        Self {
            rules: vec![
                ResolutionRule {
                    threshold_mp: 50.0,
                    max_concurrency: 1,
                    description: "超高分辨率(>50MP) - 单线程处理".to_string(),
                },
                ResolutionRule {
                    threshold_mp: 25.0,
                    max_concurrency: 2,
                    description: "高分辨率(25-50MP) - 最多2并发".to_string(),
                },
                ResolutionRule {
                    threshold_mp: 10.0,
                    max_concurrency: 4,
                    description: "中高分辨率(10-25MP) - 最多4并发".to_string(),
                },
                ResolutionRule {
                    threshold_mp: 5.0,
                    max_concurrency: 8,
                    description: "标准分辨率(5-10MP) - 最多8并发".to_string(),
                },
                ResolutionRule {
                    threshold_mp: 0.0,
                    max_concurrency: 16,
                    description: "低分辨率(<5MP) - 最多16并发".to_string(),
                },
            ],
        }
    }

    /// 基于图片分辨率优化工作线程数
    pub fn optimize_worker_count<P: AsRef<Path>>(&self, image_path: P, base_workers: usize) -> Result<usize, Box<dyn std::error::Error>> {
        let res_mp = match self.get_image_resolution_mp(&image_path) {
            Ok(mp) => mp,
            Err(_) => return Ok(base_workers),
        };

        let optimized_workers = self.apply_resolution_rules(res_mp, base_workers);
        Ok(optimized_workers)
    }

    fn apply_resolution_rules(&self, res_mp: f64, base_workers: usize) -> usize {
        for rule in &self.rules {
            if res_mp >= rule.threshold_mp {
                if base_workers > rule.max_concurrency {
                    return rule.max_concurrency;
                }
                return base_workers;
            }
        }
        base_workers
    }

    #[allow(dead_code)]
    fn get_applied_rule(&self, res_mp: f64) -> &str {
        for rule in &self.rules {
            if res_mp >= rule.threshold_mp {
                return &rule.description;
            }
        }
        "默认规则"
    }

    fn get_image_resolution_mp<P: AsRef<Path>>(&self, image_path: P) -> Result<f64, Box<dyn std::error::Error>> {
        let path_str = image_path.as_ref().to_string_lossy();

        if let Ok(mp) = self.get_resolution_with_image_crate(&image_path) {
            return Ok(mp);
        }

        if let Ok(mp) = self.get_resolution_with_identify(&path_str) {
            return Ok(mp);
        }

        if let Ok(mp) = self.get_resolution_with_exiftool(&path_str) {
            return Ok(mp);
        }

        if let Ok(mp) = self.get_resolution_with_ffprobe(&path_str) {
            return Ok(mp);
        }

        Err("无法获取图片分辨率".into())
    }

    fn get_resolution_with_image_crate<P: AsRef<Path>>(&self, image_path: P) -> Result<f64, Box<dyn std::error::Error>> {
        let img = image::open(image_path)?;
        let (width, height) = img.dimensions();
        let megapixels = (width as f64 * height as f64) / 1_000_000.0;
        Ok(megapixels)
    }

    fn get_resolution_with_identify(&self, image_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let output = Command::new("identify")
            .args(["-format", "%w %h", image_path])
            .output()?;

        if !output.status.success() {
            return Err("Failed to execute identify command".into());
        }

        let output_str = String::from_utf8(output.stdout)?;
        self.parse_resolution(&output_str)
    }

    fn get_resolution_with_exiftool(&self, image_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let output = Command::new("exiftool")
            .args(["-ImageWidth", "-ImageHeight", "-s3", image_path])
            .output()?;

        if !output.status.success() {
            return Err("Failed to execute exiftool command".into());
        }

        let output_str = String::from_utf8(output.stdout)?;
        self.parse_resolution(&output_str)
    }

    fn get_resolution_with_ffprobe(&self, image_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let output = Command::new("ffprobe")
            .args([
                "-v", "error",
                "-select_streams", "v:0",
                "-show_entries", "stream=width,height",
                "-of", "csv=s=x:p=0",
                image_path
            ])
            .output()?;

        if !output.status.success() {
            return Err("Failed to execute ffprobe command".into());
        }

        let output_str = String::from_utf8(output.stdout)?;
        self.parse_resolution_from_ffprobe(&output_str)
    }

    fn parse_resolution(&self, res_str: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = res_str.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(format!("Invalid resolution format: {}", res_str).into());
        }

        let width: f64 = parts[0].parse()?;
        let height: f64 = parts[1].parse()?;
        let megapixels = (width * height) / 1_000_000.0;
        Ok(megapixels)
    }

    fn parse_resolution_from_ffprobe(&self, res_str: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = res_str.trim().split('x').collect();
        if parts.len() < 2 {
            return Err(format!("Invalid resolution format: {}", res_str).into());
        }

        let width: f64 = parts[0].parse()?;
        let height: f64 = parts[1].parse()?;
        let megapixels = (width * height) / 1_000_000.0;
        Ok(megapixels)
    }

    pub fn get_recommended_worker_count<P: AsRef<Path>>(&self, image_path: P) -> Result<(usize, f64), Box<dyn std::error::Error>> {
        let res_mp = self.get_image_resolution_mp(image_path)?;

        for rule in &self.rules {
            if res_mp >= rule.threshold_mp {
                return Ok((rule.max_concurrency, res_mp));
            }
        }

        self.rules.last()
            .map(|rule| (rule.max_concurrency, res_mp))
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "No RAM optimization rules configured")) as Box<dyn std::error::Error>)
    }

    pub fn estimate_memory_usage<P: AsRef<Path>>(&self, image_path: P) -> Result<f64, Box<dyn std::error::Error>> {
        let res_mp = self.get_image_resolution_mp(image_path)?;
        let estimated_mb = res_mp * 4.0 * 3.0;
        Ok(estimated_mb)
    }

    pub fn get_rules(&self) -> &[ResolutionRule] {
        &self.rules
    }

    pub fn add_rule(&mut self, rule: ResolutionRule) {
        self.rules.push(rule);
        self.rules.sort_by(|a, b| b.threshold_mp.partial_cmp(&a.threshold_mp).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn adjust_rules_for_system_memory(&mut self, available_memory_gb: f64) {
        if available_memory_gb < 8.0 {
            for rule in &mut self.rules {
                rule.max_concurrency = (rule.max_concurrency / 2).max(1);
            }
        } else if available_memory_gb > 32.0 {
            for rule in &mut self.rules {
                if rule.threshold_mp < 10.0 {
                    rule.max_concurrency = (rule.max_concurrency * 3 / 2).min(32);
                }
            }
        }
    }

    pub fn get_memory_info(&self, image_paths: &[impl AsRef<Path>]) -> RAMOptimizerStats {
        let mut stats = RAMOptimizerStats::default();
        
        for path in image_paths {
            if let Ok(mp) = self.get_image_resolution_mp(path) {
                stats.total_images += 1;
                stats.total_megapixels += mp;
                
                if let Ok(mb) = self.estimate_memory_usage(path) {
                    stats.estimated_memory_mb += mb;
                }

                if mp >= 50.0 {
                    stats.ultra_high_res_count += 1;
                } else if mp >= 25.0 {
                    stats.high_res_count += 1;
                } else if mp >= 10.0 {
                    stats.medium_res_count += 1;
                } else if mp >= 5.0 {
                    stats.standard_res_count += 1;
                } else {
                    stats.low_res_count += 1;
                }
            }
        }

        if stats.total_images > 0 {
            stats.average_megapixels = stats.total_megapixels / stats.total_images as f64;
        }

        stats
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RAMOptimizerStats {
    pub total_images: usize,
    pub total_megapixels: f64,
    pub average_megapixels: f64,
    pub estimated_memory_mb: f64,
    pub ultra_high_res_count: usize,
    pub high_res_count: usize,
    pub medium_res_count: usize,
    pub standard_res_count: usize,
    pub low_res_count: usize,
}

impl Default for RAMOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_rules() {
        let optimizer = RAMOptimizer::new();
        
        assert_eq!(optimizer.apply_resolution_rules(60.0, 8), 1);
        assert_eq!(optimizer.apply_resolution_rules(30.0, 8), 2);
        assert_eq!(optimizer.apply_resolution_rules(15.0, 8), 4);
        assert_eq!(optimizer.apply_resolution_rules(7.0, 8), 8);
        assert_eq!(optimizer.apply_resolution_rules(2.0, 20), 16);
    }

    #[test]
    fn test_parse_resolution() {
        let optimizer = RAMOptimizer::new();
        
        let result = optimizer.parse_resolution("1920 1080").unwrap();
        assert!((result - 2.073).abs() < 0.001);
        
        let result = optimizer.parse_resolution("3840 2160").unwrap();
        assert!((result - 8.294).abs() < 0.001);
    }
}
