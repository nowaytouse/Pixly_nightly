// 🎨 GIF优化增强
// 从 @archive/rust_broken/src/converter/gif_optimizer.rs 提取

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameOptimization {
    None,
    Basic,
    Balanced,
    Aggressive,
}

impl FrameOptimization {
    pub fn gifsicle_level(&self) -> &str {
        match self {
            FrameOptimization::None => "O1",
            FrameOptimization::Basic => "O2",
            FrameOptimization::Balanced => "O3",
            FrameOptimization::Aggressive => "O3",
        }
    }
}

pub struct GifOptimizer {
    config: GifOptimizationConfig,
}

impl GifOptimizer {
    pub fn new(config: GifOptimizationConfig) -> Self {
        Self { config }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(GifOptimizationConfig::default())
    }
    
    pub fn for_web() -> Self {
        Self::new(GifOptimizationConfig {
            color_optimization: 3,
            frame_optimization: FrameOptimization::Aggressive,
            lossy_compression: true,
            lossy_quality: 80,
            strip_metadata: true,
            max_fps: Some(30),
            max_width: Some(800),
        })
    }
    
    pub fn config(&self) -> &GifOptimizationConfig {
        &self.config
    }
    
    pub fn build_gifsicle_args(&self) -> Vec<String> {
        let mut args = vec![
            format!("-{}", self.config.frame_optimization.gifsicle_level()),
            format!("--colors={}", 256 >> self.config.color_optimization),
        ];
        
        if self.config.lossy_compression {
            args.push(format!("--lossy={}", self.config.lossy_quality));
        }
        
        if let Some(fps) = self.config.max_fps {
            args.push(format!("--delay={}", 100 / fps));
        }
        
        if let Some(width) = self.config.max_width {
            args.push(format!("--resize-width={}", width));
        }
        
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = GifOptimizationConfig::default();
        assert_eq!(config.color_optimization, 2);
        assert!(!config.lossy_compression);
    }
    
    #[test]
    fn test_gifsicle_level() {
        assert_eq!(FrameOptimization::None.gifsicle_level(), "O1");
        assert_eq!(FrameOptimization::Balanced.gifsicle_level(), "O3");
    }
}
