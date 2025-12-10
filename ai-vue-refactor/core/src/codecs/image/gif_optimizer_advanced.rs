//! highlevelGIFoptimization
//!
//! providemultisegmentiterationoptimization、frameoptimization、optimization etc feature

use serde::{Deserialize, Serialize};

/// GIFoptimizationconfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GifOptimizationConfig {
 pub color_optimization: u8,
 pub frame_optimization: FrameOptimization,
 pub lossy_compression: bool,
 pub lossy_quality: u8,
 pub strip_metadata: bool,
 pub max_fps: Option<u8>,
 pub max_width: Option<u32>,
}

impl Default for GifOptimizationConfig {
 fn default() -> Self {
 Self {
 color_optimization: 2,
 frame_optimization: FrameOptimization::Balanced,
 lossy_compression: false,
 lossy_quality: 80,
 strip_metadata: true,
 max_fps: None,
 max_width: None,
 }
 }
}

/// frameoptimizationlevel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameOptimization {
 None,
 Basic,
 Balanced,
 Aggressive,
}

/// GIFoptimization
pub struct GifOptimizer {
 #[allow(dead_code)]
 config: GifOptimizationConfig,
}

impl GifOptimizer {
 pub fn new(config: GifOptimizationConfig) -> Self {
 Self { config }
 }

 pub fn with_defaults() -> Self {
 Self::new(GifOptimizationConfig::default())
 }
}

/// optimizationresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
 pub original_size: u64,
 pub optimized_size: u64,
 pub reduction_percent: f64,
 pub elapsed_ms: u64,
}
