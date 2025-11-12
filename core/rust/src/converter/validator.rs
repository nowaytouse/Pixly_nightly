//! 质量验证模块

use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub ssim: Option<f64>,
    pub psnr: Option<f64>,
}

pub struct QualityValidator;

impl QualityValidator {
    pub fn validate(_original: &Path, _converted: &Path) -> Result<ValidationResult> {
        // ✅ SSIM验证已实现在 media_analyzer.rs::OutputValidator::calculate_ssim()
        // 本模块保留用于未来扩展（PSNR, VMAF等）
        // 当前核心质量验证使用 media_analyzer.rs
        Ok(ValidationResult {
            valid: true,
            ssim: None,
            psnr: None,
        })
    }
}
