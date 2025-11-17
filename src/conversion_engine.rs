// 🚀 转换引擎核心
// 从 @archive/rust_broken/src/conversion_engine/mod.rs 提取

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRequest {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub source_format: String,
    pub target_format: String,
    pub quality: u8,
    pub distance: Option<f32>,
    pub effort: Option<u8>,
    pub lossless: bool,
    pub advanced_options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub output_size: u64,
    pub compression_ratio: f32,
    pub processing_time_ms: u64,
    pub tool_used: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConversionEngineConfig {
    pub max_concurrent_conversions: usize,
    pub conversion_timeout_seconds: u64,
    pub enable_simd: bool,
    pub temp_dir: PathBuf,
    pub preserve_metadata: bool,
    pub merge_xmp_sidecar: bool,
    
    // ═══════════════════════════════════════════════════
    // 🎛️ 功能开关集成
    // ═══════════════════════════════════════════════════
    pub feature_toggles: Option<crate::feature_toggles::FeatureToggles>,
}

impl Default for ConversionEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_conversions: 4,
            conversion_timeout_seconds: 300,
            enable_simd: true,
            temp_dir: std::env::temp_dir().join("pixly_conversions"),
            preserve_metadata: true,
            merge_xmp_sidecar: false,
            feature_toggles: None,
        }
    }
}

impl ConversionEngineConfig {
    /// 从功能开关创建配置
    pub fn from_toggles(toggles: crate::feature_toggles::FeatureToggles) -> Self {
        Self {
            max_concurrent_conversions: toggles.max_concurrent_conversions,
            enable_simd: toggles.enable_simd,
            preserve_metadata: toggles.preserve_metadata,
            merge_xmp_sidecar: toggles.merge_xmp_sidecar,
            feature_toggles: Some(toggles),
            ..Default::default()
        }
    }
}

pub struct ConversionEngine {
    config: ConversionEngineConfig,
}

impl ConversionEngine {
    pub fn new(config: ConversionEngineConfig) -> Self {
        Self { config }
    }
    
    pub fn config(&self) -> &ConversionEngineConfig {
        &self.config
    }
    
    pub fn convert(&self, request: &ConversionRequest) -> anyhow::Result<ConversionResult> {
        let start_time = std::time::Instant::now();
        
        let original_size = std::fs::metadata(&request.input_path)?.len();
        
        // 使用config中的设置
        let _timeout = std::time::Duration::from_secs(self.config.conversion_timeout_seconds);
        
        // 执行实际转换
        use crate::conversion_core::{execute_conversion, ConversionConfig};
        
        // ═══════════════════════════════════════════════════
        // 🎛️ 从请求和引擎配置构建转换配置
        // ═══════════════════════════════════════════════════
        let mut config = ConversionConfig {
            quality: request.quality,
            speed: request.effort.unwrap_or(4),
            preserve_metadata: self.config.preserve_metadata,
            keep_animated: true,
            lossless: request.lossless,
            merge_xmp_sidecar: self.config.merge_xmp_sidecar,
            feature_toggles: self.config.feature_toggles.clone(),
            ..Default::default()
        };
        
        // ═══════════════════════════════════════════════════
        // 🔧 应用高级参数 (从advanced_options)
        // ═══════════════════════════════════════════════════
        if let Some(toggles) = &self.config.feature_toggles {
            if toggles.enable_advanced_params {
                // 从advanced_options提取参数
                if let Some(chroma) = request.advanced_options.get("chroma_subsampling") {
                    if let Some(chroma_str) = chroma.as_str() {
                        config.chroma_subsampling = Some(chroma_str.to_string());
                    }
                }
                
                if let Some(alpha_q) = request.advanced_options.get("alpha_quality") {
                    if let Some(alpha_val) = alpha_q.as_u64() {
                        config.alpha_quality = Some(alpha_val as u8);
                    }
                }
                
                if let Some(effort) = request.advanced_options.get("effort") {
                    if let Some(effort_val) = effort.as_u64() {
                        config.effort = Some(effort_val as u8);
                    }
                }
                
                if let Some(normalize) = request.advanced_options.get("normalize_filenames") {
                    if let Some(normalize_bool) = normalize.as_bool() {
                        config.normalize_filenames = normalize_bool;
                    }
                }
            }
        }
        
        let result = execute_conversion(
            &request.input_path,
            &request.output_path,
            &request.target_format,
            &config,
        );
        
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        
        match result {
            Ok(conv_result) => {
                let output_size = conv_result.output_size;
                Ok(ConversionResult {
                    success: true,
                    input_path: request.input_path.clone(),
                    output_path: request.output_path.clone(),
                    original_size,
                    output_size,
                    compression_ratio: output_size as f32 / original_size as f32,
                    processing_time_ms,
                    tool_used: conv_result.strategy_used,
                    error_message: None,
                })
            }
            Err(e) => {
                Ok(ConversionResult {
                    success: false,
                    input_path: request.input_path.clone(),
                    output_path: request.output_path.clone(),
                    original_size,
                    output_size: 0,
                    compression_ratio: 0.0,
                    processing_time_ms,
                    tool_used: "failed".to_string(),
                    error_message: Some(e.to_string()),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ConversionEngineConfig::default();
        assert_eq!(config.max_concurrent_conversions, 4);
        assert!(config.enable_simd);
    }
}
