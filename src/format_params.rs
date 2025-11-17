/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 格式专属参数 (Format-Specific Parameters)
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * 🎯 目标:
 * - 完整支持HTML界面的所有格式专属参数
 * - 与Eagle插件HTML界面完全对应
 * - 类型安全的参数验证
 */

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════
// 📦 JXL (JPEG XL) 参数
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JxlParams {
    /// 努力程度 (1-9) - 对应HTML: jxlEffort
    pub effort: Option<u8>,
    
    /// 距离值 (0-15) - 对应HTML: jxlDistance
    pub distance: Option<f32>,
    
    /// 位深度 (8/10/12/16) - 对应HTML: jxlBitDepth
    pub bit_depth: Option<u8>,
    
    /// 色彩空间 - 对应HTML: jxlColorSpace
    pub color_space: Option<String>,
    
    /// 边缘增强 (0-4) - 对应HTML: jxlPatches
    pub patches: Option<u8>,
}

impl Default for JxlParams {
    fn default() -> Self {
        Self {
            effort: Some(7),
            distance: Some(1.0),
            bit_depth: Some(8),
            color_space: Some("sRGB".to_string()),
            patches: Some(1),
        }
    }
}

impl JxlParams {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(effort) = self.effort {
            if !(1..=9).contains(&effort) {
                return Err(format!("JXL effort must be 1-9, got {}", effort));
            }
        }
        
        if let Some(distance) = self.distance {
            if !(0.0..=15.0).contains(&distance) {
                return Err(format!("JXL distance must be 0-15, got {}", distance));
            }
        }
        
        if let Some(bit_depth) = self.bit_depth {
            if ![8, 10, 12, 16].contains(&bit_depth) {
                return Err(format!("JXL bit_depth must be 8/10/12/16, got {}", bit_depth));
            }
        }
        
        if let Some(patches) = self.patches {
            if patches > 4 {
                return Err(format!("JXL patches must be 0-4, got {}", patches));
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════
// 🌐 WebP 参数
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebPParams {
    /// 压缩方法 (0-6) - 对应HTML: webpMethod
    pub method: Option<u8>,
    
    /// 滤波强度 (0-100) - 对应HTML: webpFilterStrength
    pub filter_strength: Option<u8>,
    
    /// 锐化级别 (0-7) - 对应HTML: webpSharpness
    pub sharpness: Option<u8>,
    
    /// 分段数量 (1-4) - 对应HTML: webpSegments
    pub segments: Option<u8>,
    
    /// SNS强度 (0-100) - 对应HTML: webpSnsStrength
    pub sns_strength: Option<u8>,
    
    /// 自动滤波 - 对应HTML: webpAutoFilter
    pub auto_filter: bool,
    
    /// 精确模式 - 对应HTML: webpExactMode
    pub exact_mode: bool,
    
    /// 移除Alpha - 对应HTML: webpNoAlpha
    pub no_alpha: bool,
    
    /// 低内存模式 - 对应HTML: webpLowMemory
    pub low_memory: bool,
    
    /// 编码遍数 (1-10) - 对应HTML: webpPass
    pub pass: Option<u8>,
}

impl Default for WebPParams {
    fn default() -> Self {
        Self {
            method: Some(4),
            filter_strength: Some(60),
            sharpness: Some(0),
            segments: Some(4),
            sns_strength: Some(50),
            auto_filter: true,
            exact_mode: false,
            no_alpha: false,
            low_memory: false,
            pass: Some(1),
        }
    }
}

impl WebPParams {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(method) = self.method {
            if method > 6 {
                return Err(format!("WebP method must be 0-6, got {}", method));
            }
        }
        
        if let Some(filter_strength) = self.filter_strength {
            if filter_strength > 100 {
                return Err(format!("WebP filter_strength must be 0-100, got {}", filter_strength));
            }
        }
        
        if let Some(sharpness) = self.sharpness {
            if sharpness > 7 {
                return Err(format!("WebP sharpness must be 0-7, got {}", sharpness));
            }
        }
        
        if let Some(segments) = self.segments {
            if !(1..=4).contains(&segments) {
                return Err(format!("WebP segments must be 1-4, got {}", segments));
            }
        }
        
        if let Some(sns_strength) = self.sns_strength {
            if sns_strength > 100 {
                return Err(format!("WebP sns_strength must be 0-100, got {}", sns_strength));
            }
        }
        
        if let Some(pass) = self.pass {
            if !(1..=10).contains(&pass) {
                return Err(format!("WebP pass must be 1-10, got {}", pass));
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════
// 🎬 AVIF 参数
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvifParams {
    /// 编码速度 (0-10) - 对应HTML: avifSpeed
    pub speed: Option<u8>,
    
    /// 最小量化器 (0-63) - 对应HTML: avifMinQuantizer
    pub min_quantizer: Option<u8>,
    
    /// 最大量化器 (0-63) - 对应HTML: avifMaxQuantizer
    pub max_quantizer: Option<u8>,
    
    /// 色度子采样 - 对应HTML: avifChromaSubsampling
    pub chroma_subsampling: Option<String>,
    
    /// 位深度 (8/10/12) - 对应HTML: avifBitDepth
    pub bit_depth: Option<u8>,
    
    /// Tiles行数 (1-8) - 对应HTML: avifTilesRows
    pub tiles_rows: Option<u8>,
    
    /// Tiles列数 (1-8) - 对应HTML: avifTilesCols
    pub tiles_cols: Option<u8>,
    
    /// 预乘Alpha - 对应HTML: avifPremultiply
    pub premultiply_alpha: bool,
}

impl Default for AvifParams {
    fn default() -> Self {
        Self {
            speed: Some(6),
            min_quantizer: Some(0),
            max_quantizer: Some(63),
            chroma_subsampling: Some("420".to_string()),
            bit_depth: Some(8),
            tiles_rows: Some(1),
            tiles_cols: Some(1),
            premultiply_alpha: false,
        }
    }
}

impl AvifParams {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(speed) = self.speed {
            if speed > 10 {
                return Err(format!("AVIF speed must be 0-10, got {}", speed));
            }
        }
        
        if let Some(min_q) = self.min_quantizer {
            if min_q > 63 {
                return Err(format!("AVIF min_quantizer must be 0-63, got {}", min_q));
            }
        }
        
        if let Some(max_q) = self.max_quantizer {
            if max_q > 63 {
                return Err(format!("AVIF max_quantizer must be 0-63, got {}", max_q));
            }
        }
        
        if let Some(bit_depth) = self.bit_depth {
            if ![8, 10, 12].contains(&bit_depth) {
                return Err(format!("AVIF bit_depth must be 8/10/12, got {}", bit_depth));
            }
        }
        
        if let Some(rows) = self.tiles_rows {
            if !(1..=8).contains(&rows) {
                return Err(format!("AVIF tiles_rows must be 1-8, got {}", rows));
            }
        }
        
        if let Some(cols) = self.tiles_cols {
            if !(1..=8).contains(&cols) {
                return Err(format!("AVIF tiles_cols must be 1-8, got {}", cols));
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════
// 🍎 HEIC 参数
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeicParams {
    /// 压缩质量 (1-100) - 对应HTML: heicQuality
    pub quality: Option<u8>,
    
    /// 编码器 - 对应HTML: heicEncoder
    pub encoder: Option<String>,
    
    /// 色度子采样 - 对应HTML: heicChromaSubsampling
    pub chroma_subsampling: Option<String>,
    
    /// 无损编码 - 对应HTML: heicLossless
    pub lossless: bool,
    
    /// 嵌入缩略图 - 对应HTML: heicThumbEmbed
    pub embed_thumbnail: bool,
}

impl Default for HeicParams {
    fn default() -> Self {
        Self {
            quality: Some(85),
            encoder: Some("x265".to_string()),
            chroma_subsampling: Some("444".to_string()),
            lossless: false,
            embed_thumbnail: true,
        }
    }
}

impl HeicParams {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(quality) = self.quality {
            if !(1..=100).contains(&quality) {
                return Err(format!("HEIC quality must be 1-100, got {}", quality));
            }
        }
        
        if let Some(ref encoder) = self.encoder {
            if !["x265", "libheif"].contains(&encoder.as_str()) {
                return Err(format!("HEIC encoder must be x265 or libheif, got {}", encoder));
            }
        }
        
        if let Some(ref chroma) = self.chroma_subsampling {
            if !["420", "422", "444"].contains(&chroma.as_str()) {
                return Err(format!("HEIC chroma_subsampling must be 420/422/444, got {}", chroma));
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════
// 🎯 统一格式参数枚举
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", content = "params")]
pub enum FormatSpecificParams {
    Jxl(JxlParams),
    WebP(WebPParams),
    Avif(AvifParams),
    Heic(HeicParams),
    None,
}

impl FormatSpecificParams {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            FormatSpecificParams::Jxl(params) => params.validate(),
            FormatSpecificParams::WebP(params) => params.validate(),
            FormatSpecificParams::Avif(params) => params.validate(),
            FormatSpecificParams::Heic(params) => params.validate(),
            FormatSpecificParams::None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jxl_params_default() {
        let params = JxlParams::default();
        assert_eq!(params.effort, Some(7));
        assert_eq!(params.distance, Some(1.0));
        assert!(params.validate().is_ok());
    }
    
    #[test]
    fn test_jxl_params_validation() {
        let mut params = JxlParams::default();
        params.effort = Some(10);
        assert!(params.validate().is_err());
        
        params.effort = Some(7);
        params.distance = Some(20.0);
        assert!(params.validate().is_err());
    }
    
    #[test]
    fn test_webp_params_default() {
        let params = WebPParams::default();
        assert_eq!(params.method, Some(4));
        assert!(params.auto_filter);
        assert!(params.validate().is_ok());
    }
    
    #[test]
    fn test_avif_params_default() {
        let params = AvifParams::default();
        assert_eq!(params.speed, Some(6));
        assert_eq!(params.chroma_subsampling, Some("420".to_string()));
        assert!(params.validate().is_ok());
    }
    
    #[test]
    fn test_heic_params_default() {
        let params = HeicParams::default();
        assert_eq!(params.quality, Some(85));
        assert_eq!(params.encoder, Some("x265".to_string()));
        assert!(params.validate().is_ok());
    }
}
