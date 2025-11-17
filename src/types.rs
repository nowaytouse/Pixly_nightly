use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 质量模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityMode {
    /// 速度优先
    Speed,
    /// 平衡模式
    Balanced,
    /// 质量优先
    Quality,
    /// 无损模式
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

/// 图像特征数据 (标准化结构)
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
    /// 像素总数
    pub fn pixels(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }

    /// 文件大小(MB)
    pub fn size_mb(&self) -> f64 {
        self.file_size as f64 / (1024.0 * 1024.0)
    }

    /// 宽高比
    pub fn aspect_ratio(&self) -> f64 {
        if self.height > 0 {
            self.width as f64 / self.height as f64
        } else {
            1.0
        }
    }

    /// 是否为高分辨率图像 (>= 2K)
    pub fn is_high_resolution(&self) -> bool {
        self.width >= 2048 || self.height >= 2048
    }

    /// 是否为大图像 (>4MP)
    pub fn is_large_image(&self) -> bool {
        self.pixels() > 4_000_000
    }

    /// 是否为中等图像 (1MP-4MP)
    pub fn is_medium_image(&self) -> bool {
        let pixels = self.pixels();
        (1_000_000..=4_000_000).contains(&pixels)
    }

    /// 是否为小图像 (<1MP)
    pub fn is_small_image(&self) -> bool {
        self.pixels() < 1_000_000
    }

    /// 归一化后的压缩复杂度（从 Python ExtendedImageFeatures.compression_complexity 迁移）
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

/// 预测结果
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

/// 预测请求（方便跨语言调用）
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
