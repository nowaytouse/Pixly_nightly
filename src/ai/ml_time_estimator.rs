//! ML-driven conversion time estimator
//! 
//! Predict conversion time using file features and historical data

use std::collections::HashMap;
use std::time::Duration;

/// ML-driven conversion time estimator
pub struct TimeEstimator {
    historical_records: HashMap<String, Vec<ConversionRecord>>,
    format_factors: HashMap<String, f64>,
    learning_enabled: bool,
}

/// filefeaturestruct
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

/// conversionparameter
#[derive(Debug, Clone)]
pub struct ConversionParams {
    pub target_format: String,
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub scale_ratio: Option<f64>,
    pub threads: u32,
}

/// Conversion record
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
}

/// Time estimation result
#[derive(Debug, Clone)]
pub struct TimeEstimate {
    pub estimated_time: Duration,
    pub base_time: Duration,
    pub historical_time: Option<Duration>,
    pub confidence: f64,
    pub estimation_method: EstimationMethod,
}

/// Estimation method
#[derive(Debug, Clone, PartialEq)]
pub enum EstimationMethod {
    Theoretical,
    Historical,
    Hybrid,
}

impl TimeEstimator {
    pub fn new() -> Self {
        let mut format_factors = HashMap::new();
        
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

    pub fn estimate_conversion_time(
        &self,
        features: &FileFeatures,
        params: &ConversionParams,
    ) -> TimeEstimate {
        let base_time = self.calculate_base_time(features, params);
        let historical_time = self.query_historical_time(features, params);

        let (estimated_time, method, confidence) = match historical_time {
            Some(hist_time) => {
                let weighted_time = Duration::from_secs_f64(
                    hist_time.as_secs_f64() * 0.7 + base_time.as_secs_f64() * 0.3
                );
                (weighted_time, EstimationMethod::Hybrid, 0.8)
            }
            None => {
                (base_time, EstimationMethod::Theoretical, 0.5)
            }
        };

        TimeEstimate {
            estimated_time,
            base_time,
            historical_time,
            confidence,
            estimation_method: method,
        }
    }

    fn calculate_base_time(
        &self,
        features: &FileFeatures,
        params: &ConversionParams,
    ) -> Duration {
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

    fn get_format_factor(&self, format: &str) -> f64 {
        self.format_factors
            .get(format)
            .copied()
            .unwrap_or(1.0)
    }

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
            .filter(|record| {
                self.is_similar_conversion(record, features, params, tolerance)
            })
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
        };

        let key = self.make_record_key(&features.format, &params.target_format);
        self.historical_records
            .entry(key.clone())
            .or_default()
            .push(record);

        if let Some(records) = self.historical_records.get_mut(&key) && records.len() > 1000 {
            records.truncate(1000);
        }
    }

    fn make_record_key(&self, source_format: &str, target_format: &str) -> String {
        format!("{}:{}", source_format, target_format)
    }

    pub fn batch_estimate_time(
        &self,
        files: &[(FileFeatures, ConversionParams)],
    ) -> Duration {
        let total_time: Duration = files
            .iter()
            .map(|(features, params)| {
                self.estimate_conversion_time(features, params).estimated_time
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

    fn create_test_features() -> FileFeatures {
        FileFeatures {
            file_path: "/tmp/test.jpg".to_string(),
            width: 1920,
            height: 1080,
            file_size: 1_000_000,
            format: "jpg".to_string(),
            is_animated: false,
            frame_count: 1,
        }
    }

    fn create_test_params() -> ConversionParams {
        ConversionParams {
            target_format: "webp".to_string(),
            quality: 85,
            effort: 5,
            lossless: false,
            scale_ratio: None,
            threads: 8,
        }
    }

    #[test]
    fn test_time_estimator_creation() {
        let estimator = TimeEstimator::new();
        assert!(estimator.learning_enabled);
        assert!(!estimator.format_factors.is_empty());
    }

    #[test]
    fn test_estimate_conversion_time() {
        let estimator = TimeEstimator::new();
        let features = create_test_features();
        let params = create_test_params();

        let estimate = estimator.estimate_conversion_time(&features, &params);

        assert!(estimate.estimated_time.as_secs_f64() > 0.0);
        assert!(estimate.confidence > 0.0 && estimate.confidence <= 1.0);
        assert_eq!(estimate.estimation_method, EstimationMethod::Theoretical);
    }

    #[test]
    fn test_format_factors() {
        let estimator = TimeEstimator::new();

        // AVIF should take longer than JPEG
        let avif_factor = estimator.get_format_factor("avif");
        let jpg_factor = estimator.get_format_factor("jpg");

        assert!(avif_factor > jpg_factor, "AVIF should have higher factor than JPG");
    }

    #[test]
    fn test_high_quality_takes_longer() {
        let estimator = TimeEstimator::new();
        let features = create_test_features();

        let low_quality_params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 70,
            effort: 3,
            lossless: false,
            scale_ratio: None,
            threads: 8,
        };

        let high_quality_params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 98,
            effort: 9,
            lossless: false,
            scale_ratio: None,
            threads: 8,
        };

        let low_estimate = estimator.estimate_conversion_time(&features, &low_quality_params);
        let high_estimate = estimator.estimate_conversion_time(&features, &high_quality_params);

        assert!(
            high_estimate.estimated_time > low_estimate.estimated_time,
            "Higher quality should take longer"
        );
    }

    #[test]
    fn test_larger_image_takes_longer() {
        let estimator = TimeEstimator::new();
        let params = create_test_params();

        let small_features = FileFeatures {
            file_path: "/tmp/small.jpg".to_string(),
            width: 640,
            height: 480,
            file_size: 100_000,
            format: "jpg".to_string(),
            is_animated: false,
            frame_count: 1,
        };

        let large_features = FileFeatures {
            file_path: "/tmp/large.jpg".to_string(),
            width: 4096,
            height: 2160,
            file_size: 10_000_000,
            format: "jpg".to_string(),
            is_animated: false,
            frame_count: 1,
        };

        let small_estimate = estimator.estimate_conversion_time(&small_features, &params);
        let large_estimate = estimator.estimate_conversion_time(&large_features, &params);

        assert!(
            large_estimate.estimated_time > small_estimate.estimated_time,
            "Larger images should take longer"
        );
    }

    #[test]
    fn test_animated_takes_longer() {
        let estimator = TimeEstimator::new();
        let params = create_test_params();

        let static_features = FileFeatures {
            file_path: "/tmp/static.gif".to_string(),
            width: 500,
            height: 500,
            file_size: 500_000,
            format: "gif".to_string(),
            is_animated: false,
            frame_count: 1,
        };

        let animated_features = FileFeatures {
            file_path: "/tmp/animated.gif".to_string(),
            width: 500,
            height: 500,
            file_size: 500_000,
            format: "gif".to_string(),
            is_animated: true,
            frame_count: 100,
        };

        let static_estimate = estimator.estimate_conversion_time(&static_features, &params);
        let animated_estimate = estimator.estimate_conversion_time(&animated_features, &params);

        assert!(
            animated_estimate.estimated_time > static_estimate.estimated_time,
            "Animated images should take longer"
        );
    }

    #[test]
    fn test_record_and_use_historical_data() {
        let mut estimator = TimeEstimator::new();
        let features = create_test_features();
        let params = create_test_params();

        // First estimate (no historical data)
        let first_estimate = estimator.estimate_conversion_time(&features, &params);
        assert_eq!(first_estimate.estimation_method, EstimationMethod::Theoretical);

        // Record actual time
        let actual_time = Duration::from_millis(500);
        estimator.record_actual_time(&features, &params, actual_time);

        // Second estimate should use hybrid method
        let second_estimate = estimator.estimate_conversion_time(&features, &params);
        assert_eq!(second_estimate.estimation_method, EstimationMethod::Hybrid);
        assert!(second_estimate.historical_time.is_some());
    }

    #[test]
    fn test_batch_estimate() {
        let estimator = TimeEstimator::new();
        let features = create_test_features();
        let params = create_test_params();

        let files = vec![
            (features.clone(), params.clone()),
            (features.clone(), params.clone()),
            (features.clone(), params.clone()),
        ];

        let single_estimate = estimator.estimate_conversion_time(&features, &params);
        let batch_estimate = estimator.batch_estimate_time(&files);

        // Batch should be less than 3x single due to parallel efficiency
        let three_times_single = single_estimate.estimated_time.as_secs_f64() * 3.0;
        assert!(
            batch_estimate.as_secs_f64() < three_times_single,
            "Batch estimate should account for parallel efficiency"
        );
    }

    #[test]
    fn test_lossless_takes_longer() {
        let estimator = TimeEstimator::new();
        let features = create_test_features();

        let lossy_params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 85,
            effort: 5,
            lossless: false,
            scale_ratio: None,
            threads: 8,
        };

        let lossless_params = ConversionParams {
            target_format: "webp".to_string(),
            quality: 85,
            effort: 5,
            lossless: true,
            scale_ratio: None,
            threads: 8,
        };

        let lossy_estimate = estimator.estimate_conversion_time(&features, &lossy_params);
        let lossless_estimate = estimator.estimate_conversion_time(&features, &lossless_params);

        assert!(
            lossless_estimate.estimated_time > lossy_estimate.estimated_time,
            "Lossless should take longer"
        );
    }

    #[test]
    fn test_unknown_format_uses_default_factor() {
        let estimator = TimeEstimator::new();
        let factor = estimator.get_format_factor("unknown_format");
        assert_eq!(factor, 1.0, "Unknown format should use default factor of 1.0");
    }
}
