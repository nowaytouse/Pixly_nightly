use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quality mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityMode {
    /// Speed priority
    Speed,
    /// Balanced mode
    Balanced,
    /// Quality priority
    Quality,
    /// Lossless mode
    Lossless,
}

impl QualityMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            QualityMode::Speed => "speed",
            QualityMode::Balanced => "balanced",
            QualityMode::Quality => "quality",
            QualityMode::Lossless => "lossless",
        }
    }

    pub fn parse_mode(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "speed" => Some(QualityMode::Speed),
            "balanced" => Some(QualityMode::Balanced),
            "quality" => Some(QualityMode::Quality),
            "lossless" => Some(QualityMode::Lossless),
            _ => None,
        }
    }
}

/// Image feature data (standardized structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFeatures {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: String,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub complexity: f64,
}

impl ImageFeatures {
    /// Total pixel count
    pub fn pixels(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }

    /// File size in MB
    pub fn size_mb(&self) -> f64 {
        self.file_size as f64 / (1024.0 * 1024.0)
    }

    /// Aspect ratio
    pub fn aspect_ratio(&self) -> f64 {
        if self.height > 0 {
            self.width as f64 / self.height as f64
        } else {
            1.0
        }
    }

    /// Whether this is a high resolution image (>= 2K)
    pub fn is_high_resolution(&self) -> bool {
        self.width >= 2048 || self.height >= 2048
    }

    /// Whether this is a large image (>4MP)
    pub fn is_large_image(&self) -> bool {
        self.pixels() > 4_000_000
    }

    /// Whether this is a medium image (1MP-4MP)
    pub fn is_medium_image(&self) -> bool {
        let pixels = self.pixels();
        (1_000_000..=4_000_000).contains(&pixels)
    }

    /// Whether this is a small image (<1MP)
    pub fn is_small_image(&self) -> bool {
        self.pixels() < 1_000_000
    }

    /// Normalized compression complexity (migrated from Python ExtendedImageFeatures.compression_complexity)
    pub fn effective_complexity(&self) -> f64 {
        let mut base = self.complexity;

        if self.is_large_image() {
            base *= 1.3;
        }

        if self.is_high_resolution() {
            base *= 1.2;
        }

        if self.is_animated {
            base *= 1.5;
        }

        if self.has_alpha {
            base *= 1.1;
        }

        base.min(1.0)
    }
}

/// Prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub quality: u32,
    pub speed: u32,
    pub lossless: bool,
    pub format_options: HashMap<String, String>,
    pub estimated_size: u64,
    pub estimated_ratio: f64,
    pub algorithm_version: String,
    pub predictor_version: String,
}

/// Prediction request (for cross-language invocation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub features: ImageFeatures,
    pub target_format: String,
    pub quality_mode: QualityMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionWithConfidence {
    pub core: PredictionResult,
    pub confidence: f64,
    pub method: String,
}
