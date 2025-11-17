//! ⏱️ ML驱动的转换时间估算器
//!
//! 结合文件特征和历史数据预测转换时间
//!
//! ## 核心算法
//!
//! - **理论时间计算** - 基于文件特征的多因子计算
//! - **历史数据学习** - 从实际转换记录中学习
//! - **混合估算** - 70%历史 + 30%理论
//! - **格式特定预测** - 不同格式的复杂度因子
//!
//! ## 使用场景
//!
//! - 批量处理进度预测
//! - 用户等待时间提示
//! - 资源调度优化

use std::collections::HashMap;
use std::time::Duration;

/// ML驱动的转换时间估算器
pub struct TimeEstimator {
    /// 历史数据记录
    historical_records: HashMap<String, Vec<ConversionRecord>>,
    /// 格式复杂度因子
    format_factors: HashMap<String, f64>,
    /// 是否启用历史数据学习
    learning_enabled: bool,
}

/// 文件特征结构
#[derive(Debug, Clone)]
pub struct FileFeatures {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: String,
    pub is_animated: bool,
    pub frame_count: u32,
}

/// 转换参数
#[derive(Debug, Clone)]
pub struct ConversionParams {
    pub target_format: String,
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub scale_ratio: Option<f64>,
    pub threads: u32,
}

/// 转换记录（用于历史数据学习）
#[derive(Debug, Clone)]
pub struct ConversionRecord {
    pub file_size: u64,
    pub width: u32,
    pub height: u32,
    pub source_format: String,
    pub target_format: String,
    pub actual_time_ms: u64,
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub timestamp: std::time::SystemTime,
}

/// 时间估算结果
#[derive(Debug, Clone)]
pub struct TimeEstimate {
    pub estimated_time: Duration,
    pub base_time: Duration,
    pub historical_time: Option<Duration>,
    pub confidence: f64,
    pub estimation_method: EstimationMethod,
}

/// 估算方法
#[derive(Debug, Clone, PartialEq)]
pub enum EstimationMethod {
    Theoretical,
    Historical,
    Hybrid,
}

/// 时间估算器统计信息
#[derive(Debug, Default)]
pub struct TimeEstimatorStats {
    pub total_records: usize,
    pub conversion_pairs: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub learning_enabled: bool,
}

impl TimeEstimator {
    /// 创建时间估算器
    pub fn new() -> Self {
        let mut format_factors = HashMap::new();

        // 格式复杂度因子
        format_factors.insert("jpg".to_string(), 0.3);
        format_factors.insert("jpeg".to_string(), 0.3);
        format_factors.insert("webp".to_string(), 0.8);
        format_factors.insert("png".to_string(), 0.5);
        format_factors.insert("jxl".to_string(), 1.5);
        format_factors.insert("avif".to_string(), 2.5);
        format_factors.insert("heif".to_string(), 2.0);
        format_factors.insert("gif".to_string(), 0.6);

        Self {
            historical_records: HashMap::new(),
            format_factors,
            learning_enabled: true,
        }
    }

    /// 预测单个文件转换时间
    pub fn estimate_conversion_time(
        &self,
        features: &FileFeatures,
        params: &ConversionParams,
    ) -> TimeEstimate {
        let base_time = self.calculate_base_time(features, params);
        let historical_time = self.query_historical_time(features, params);

        let (estimated_time, method, confidence) = match historical_time {
            Some(hist_time) => {
                // 70%历史数据 + 30%理论估算
                let weighted_time = Duration::from_secs_f64(
                    hist_time.as_secs_f64() * 0.7 + base_time.as_secs_f64() * 0.3,
                );
                (weighted_time, EstimationMethod::Hybrid, 0.8)
            }
            None => (base_time, EstimationMethod::Theoretical, 0.5),
        };

        TimeEstimate {
            estimated_time,
            base_time,
            historical_time,
            confidence,
            estimation_method: method,
        }
    }

    /// 基于文件特征的理论时间计算
    fn calculate_base_time(&self, features: &FileFeatures, params: &ConversionParams) -> Duration {
        let mut megapixels = (features.width as f64 * features.height as f64) / 1_000_000.0;
        if megapixels < 0.1 {
            megapixels = 0.1;
        }

        let format_factor = self.get_format_factor(&params.target_format);
        let effort_factor = 1.0 + (params.effort as f64 * 0.15);
        let quality_factor = if params.quality > 95 {
            1.5
        } else if params.quality > 85 {
            1.2
        } else {
            1.0
        };
        let lossless_factor = if params.lossless { 1.3 } else { 1.0 };
        let downscale_factor = match params.scale_ratio {
            Some(ratio) if ratio < 100.0 => {
                let scale_ratio = ratio / 100.0;
                scale_ratio * scale_ratio
            }
            _ => 1.0,
        };
        let animation_factor = if features.is_animated {
            let frame_count = if features.frame_count < 1 {
                30.0
            } else {
                features.frame_count as f64
            };
            frame_count.sqrt()
        } else {
            1.0
        };
        let cpu_factor = 8.0 / params.threads as f64;

        let estimated_seconds = megapixels
            * format_factor
            * effort_factor
            * quality_factor
            * lossless_factor
            * downscale_factor
            * animation_factor
            * cpu_factor;

        let base_overhead = 0.5;
        let total_seconds = estimated_seconds + base_overhead;

        Duration::from_secs_f64(total_seconds)
    }

    /// 获取格式复杂度因子
    fn get_format_factor(&self, format: &str) -> f64 {
        self.format_factors.get(format).copied().unwrap_or(1.0)
    }

    /// 从历史数据查询相似转换的时间
    fn query_historical_time(
        &self,
        features: &FileFeatures,
        params: &ConversionParams,
    ) -> Option<Duration> {
        if !self.learning_enabled {
            return None;
        }

        let key = self.make_record_key(&features.format, &params.target_format);
        let records = self.historical_records.get(&key)?;

        if records.is_empty() {
            return None;
        }

        let tolerance = 0.1;
        let similar_records: Vec<&ConversionRecord> = records
            .iter()
            .filter(|record| self.is_similar_conversion(record, features, params, tolerance))
            .take(10)
            .collect();

        if similar_records.is_empty() {
            return None;
        }

        let total_time: u64 = similar_records
            .iter()
            .map(|record| record.actual_time_ms)
            .sum();

        let avg_time_ms = total_time / similar_records.len() as u64;
        Some(Duration::from_millis(avg_time_ms))
    }

    /// 判断是否为相似转换
    fn is_similar_conversion(
        &self,
        record: &ConversionRecord,
        features: &FileFeatures,
        params: &ConversionParams,
        tolerance: f64,
    ) -> bool {
        let size_diff = if record.file_size > features.file_size {
            (record.file_size - features.file_size) as f64 / features.file_size as f64
        } else {
            (features.file_size - record.file_size) as f64 / features.file_size as f64
        };

        if size_diff > tolerance {
            return false;
        }

        let pixels_record = record.width as u64 * record.height as u64;
        let pixels_features = features.width as u64 * features.height as u64;

        let pixel_diff = if pixels_record > pixels_features {
            (pixels_record - pixels_features) as f64 / pixels_features as f64
        } else {
            (pixels_features - pixels_record) as f64 / pixels_features as f64
        };

        if pixel_diff > tolerance {
            return false;
        }

        let quality_diff = ((record.quality as i16) - (params.quality as i16)).abs();
        if quality_diff > 10 {
            return false;
        }

        let effort_diff = ((record.effort as i16) - (params.effort as i16)).abs();
        if effort_diff > 2 {
            return false;
        }

        record.lossless == params.lossless
    }

    /// 记录实际转换时间到历史数据
    pub fn record_actual_time(
        &mut self,
        features: &FileFeatures,
        params: &ConversionParams,
        actual_time: Duration,
    ) {
        if !self.learning_enabled {
            return;
        }

        let record = ConversionRecord {
            file_size: features.file_size,
            width: features.width,
            height: features.height,
            source_format: features.format.clone(),
            target_format: params.target_format.clone(),
            actual_time_ms: actual_time.as_millis() as u64,
            quality: params.quality,
            effort: params.effort,
            lossless: params.lossless,
            timestamp: std::time::SystemTime::now(),
        };

        let key = self.make_record_key(&features.format, &params.target_format);
        self.historical_records
            .entry(key.clone())
            .or_default()
            .push(record);

        if let Some(records) = self.historical_records.get_mut(&key) && records.len() > 1000 {
            records.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            records.truncate(1000);
        }
    }

    /// 生成记录键
    fn make_record_key(&self, source_format: &str, target_format: &str) -> String {
        format!("{}:{}", source_format, target_format)
    }

    /// 获取估算器统计信息
    pub fn get_stats(&self) -> TimeEstimatorStats {
        let mut stats = TimeEstimatorStats::default();

        for records in self.historical_records.values() {
            stats.total_records += records.len();
            for record in records {
                stats.total_time += Duration::from_millis(record.actual_time_ms);
            }
        }

        if stats.total_records > 0 {
            stats.average_time = stats.total_time / stats.total_records as u32;
        }

        stats.conversion_pairs = self.historical_records.len();
        stats.learning_enabled = self.learning_enabled;

        stats
    }

    /// 启用/禁用学习功能
    pub fn set_learning_enabled(&mut self, enabled: bool) {
        self.learning_enabled = enabled;
    }

    /// 清理过期的历史记录
    pub fn cleanup_old_records(&mut self, max_age: Duration) {
        let cutoff_time = std::time::SystemTime::now() - max_age;

        for records in self.historical_records.values_mut() {
            records.retain(|record| record.timestamp >= cutoff_time);
        }

        self.historical_records
            .retain(|_, records| !records.is_empty());
    }

    /// 批量估算多个文件的转换时间
    pub fn estimate_batch_time(&self, files: &[(FileFeatures, ConversionParams)]) -> Duration {
        let total_time: Duration = files
            .iter()
            .map(|(features, params)| {
                self.estimate_conversion_time(features, params)
                    .estimated_time
            })
            .sum();

        let parallel_efficiency = 0.85;
        Duration::from_secs_f64(total_time.as_secs_f64() * parallel_efficiency)
    }
}

impl Default for TimeEstimator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_estimation() {
        let estimator = TimeEstimator::new();

        let features = FileFeatures {
            file_path: "test.jpg".to_string(),
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpg".to_string(),
            is_animated: false,
            frame_count: 1,
        };

        let params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 80,
            effort: 4,
            lossless: false,
            scale_ratio: None,
            threads: 4,
        };

        let estimate = estimator.estimate_conversion_time(&features, &params);

        assert!(estimate.estimated_time.as_secs_f64() > 0.0);
        assert_eq!(estimate.estimation_method, EstimationMethod::Theoretical);
    }

    #[test]
    fn test_format_factors() {
        let estimator = TimeEstimator::new();

        assert_eq!(estimator.get_format_factor("jxl"), 1.5);
        assert_eq!(estimator.get_format_factor("avif"), 2.5);
        assert_eq!(estimator.get_format_factor("webp"), 0.8);
        assert_eq!(estimator.get_format_factor("unknown"), 1.0);
    }

    #[test]
    fn test_historical_learning() {
        let mut estimator = TimeEstimator::new();

        let features = FileFeatures {
            file_path: "test.png".to_string(),
            width: 800,
            height: 600,
            file_size: 1_000_000,
            format: "png".to_string(),
            is_animated: false,
            frame_count: 1,
        };

        let params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 80,
            effort: 4,
            lossless: false,
            scale_ratio: None,
            threads: 4,
        };

        estimator.record_actual_time(&features, &params, Duration::from_secs(5));

        let estimate = estimator.estimate_conversion_time(&features, &params);
        assert_eq!(estimate.estimation_method, EstimationMethod::Hybrid);
        assert!(estimate.confidence > 0.7);
    }

    #[test]
    fn test_batch_estimation() {
        let estimator = TimeEstimator::new();

        let files = vec![
            (
                FileFeatures {
                    file_path: "test1.jpg".to_string(),
                    width: 1920,
                    height: 1080,
                    file_size: 2_000_000,
                    format: "jpg".to_string(),
                    is_animated: false,
                    frame_count: 1,
                },
                ConversionParams {
                    target_format: "webp".to_string(),
                    quality: 80,
                    effort: 4,
                    lossless: false,
                    scale_ratio: None,
                    threads: 4,
                },
            ),
            (
                FileFeatures {
                    file_path: "test2.png".to_string(),
                    width: 800,
                    height: 600,
                    file_size: 1_000_000,
                    format: "png".to_string(),
                    is_animated: false,
                    frame_count: 1,
                },
                ConversionParams {
                    target_format: "jxl".to_string(),
                    quality: 90,
                    effort: 6,
                    lossless: true,
                    scale_ratio: None,
                    threads: 4,
                },
            ),
        ];

        let batch_time = estimator.estimate_batch_time(&files);
        assert!(batch_time.as_secs_f64() > 0.0);
    }
}
