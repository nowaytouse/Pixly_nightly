//! RAM优化器
//! 
//! 根据图片分辨率动态调整并发数，防止大图OOM

use std::path::Path;
use image::GenericImageView;

/// 分辨率规则
#[derive(Debug, Clone)]
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

    pub fn optimize_worker_count<P: AsRef<Path>>(&self, image_path: P, base_workers: usize) -> Result<usize, Box<dyn std::error::Error>> {
        let res_mp = match self.get_image_resolution_mp(&image_path) {
            Ok(mp) => mp,
            Err(_) => {
                return Ok(base_workers);
            }
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

    fn get_image_resolution_mp<P: AsRef<Path>>(&self, image_path: P) -> Result<f64, Box<dyn std::error::Error>> {
        if let Ok(mp) = self.get_resolution_with_image_crate(&image_path) {
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
        self.rules.sort_by(|a, b| b.threshold_mp.partial_cmp(&a.threshold_mp).unwrap());
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
}
