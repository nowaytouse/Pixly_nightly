use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crate::{ImageFeatures, PredictionWithConfidence, QualityMode, UnifiedAIPredictor};

#[derive(Debug, Clone)]
pub struct LocalPredictorStats {
    pub prediction_count: u64,
    pub total_prediction_time_ms: u64,
    pub avg_prediction_time_ms: f64,
}

/// 统一本地AI预测器（基于根内核 UnifiedAIPredictor，提取自 @archive rust_broken unified_local_ai.rs）
pub struct KernelLocalAIPredictor {
    inner: UnifiedAIPredictor,
    prediction_count: AtomicU64,
    total_prediction_time_ms: AtomicU64,
}

impl KernelLocalAIPredictor {
    pub fn new() -> Self {
        Self {
            inner: UnifiedAIPredictor::new(),
            prediction_count: AtomicU64::new(0),
            total_prediction_time_ms: AtomicU64::new(0),
        }
    }

    pub fn predict_with_stats(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> PredictionWithConfidence {
        let start = Instant::now();
        let result = self
            .inner
            .predict_with_confidence(features, target_format, quality_mode);
        let elapsed_ms = start.elapsed().as_millis() as u64;

        self.prediction_count.fetch_add(1, Ordering::Relaxed);
        self.total_prediction_time_ms
            .fetch_add(elapsed_ms, Ordering::Relaxed);

        result
    }

    pub fn stats(&self) -> LocalPredictorStats {
        let count = self.prediction_count.load(Ordering::Relaxed);
        let total = self.total_prediction_time_ms.load(Ordering::Relaxed);
        let avg = if count > 0 {
            total as f64 / count as f64
        } else {
            0.0
        };

        LocalPredictorStats {
            prediction_count: count,
            total_prediction_time_ms: total,
            avg_prediction_time_ms: avg,
        }
    }
}

impl Default for KernelLocalAIPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_local_ai_stats_grow() {
        let predictor = KernelLocalAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        let before = predictor.stats();
        assert_eq!(before.prediction_count, 0);

        let _ = predictor.predict_with_stats(&features, "webp", QualityMode::Balanced);
        let after = predictor.stats();

        assert_eq!(after.prediction_count, 1);
        assert!(after.total_prediction_time_ms >= before.total_prediction_time_ms);
    }
}
