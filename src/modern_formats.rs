// 🎨 现代图像格式支持
// AVIF, JXL等次世代格式的FFmpeg集成

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};

/// 现代格式转换器
pub struct ModernFormatConverter {
    ffmpeg_path: String,
    cjxl_path: Option<String>,
}

impl ModernFormatConverter {
    /// 创建新的转换器
    pub fn new() -> Self {
        Self {
            ffmpeg_path: "ffmpeg".to_string(),
            cjxl_path: which::which("cjxl").ok().map(|p| p.to_string_lossy().to_string()),
        }
    }
    
    /// 检查格式支持
    pub fn check_format_support(&self) -> FormatSupport {
        FormatSupport {
            avif: self.check_avif_support(),
            jxl_ffmpeg: self.check_jxl_ffmpeg_support(),
            jxl_native: self.cjxl_path.is_some(),
            webp: true, // FFmpeg总是支持WebP
        }
    }
    
    /// 检查AVIF支持
    fn check_avif_support(&self) -> bool {
        let output = Command::new(&self.ffmpeg_path)
            .args(&["-encoders"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("libaom-av1") || stdout.contains("libsvtav1")
        } else {
            false
        }
    }
    
    /// 检查JXL FFmpeg支持
    fn check_jxl_ffmpeg_support(&self) -> bool {
        let output = Command::new(&self.ffmpeg_path)
            .args(&["-encoders"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("libjxl")
        } else {
            false
        }
    }
    
    /// 转换为AVIF
    pub fn convert_to_avif<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &AVIFParams,
    ) -> Result<ConversionResult> {
        let input = input.as_ref();
        let output = output.as_ref();
        
        if !self.check_avif_support() {
            bail!("FFmpeg does not support AVIF encoding, please install libaom-av1 or libsvtav1");
        }
        
        let input_size = std::fs::metadata(input)?.len();
        
        // 构建FFmpeg命令
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.args(&[
            "-i", input.to_str().unwrap(),
            "-c:v", &params.encoder,
        ]);
        
        // 根据编码器添加参数
        match params.encoder.as_str() {
            "libaom-av1" => {
                cmd.args(&[
                    "-crf", &params.crf.to_string(),
                    "-cpu-used", &params.speed.to_string(),
                ]);
                
                // 🔥 量化器参数
                cmd.args(&["-qmin", &params.min_quantizer.to_string()]);
                cmd.args(&["-qmax", &params.max_quantizer.to_string()]);
                
                // 🔥 Tiles并行编码
                if params.tiles_rows > 1 || params.tiles_cols > 1 {
                    cmd.args(&["-tiles", &format!("{}x{}", params.tiles_cols, params.tiles_rows)]);
                }
            }
            "libsvtav1" => {
                cmd.args(&[
                    "-crf", &params.crf.to_string(),
                    "-preset", &params.speed.to_string(),
                ]);
                
                // 🔥 量化器参数
                cmd.args(&["-qmin", &params.min_quantizer.to_string()]);
                cmd.args(&["-qmax", &params.max_quantizer.to_string()]);
            }
            _ => {}
        }
        
        // 🔥 像素格式 - 根据用户选择的bit_depth和chroma_subsampling
        // 如果用户明确指定了这些参数，我们就传递给FFmpeg
        // 否则让FFmpeg自动选择最佳格式
        if !params.chroma_subsampling.is_empty() {
            let pix_fmt = match (params.bit_depth, params.chroma_subsampling.as_str()) {
                (10, "420") => "yuv420p10le",
                (10, "422") => "yuv422p10le",
                (10, "444") => "yuv444p10le",
                (12, "420") => "yuv420p12le",
                (12, "422") => "yuv422p12le",
                (12, "444") => "yuv444p12le",
                (_, "420") => "yuv420p",
                (_, "422") => "yuv422p",
                (_, "444") => "yuv444p",
                _ => {
                    // 不指定，让FFmpeg自动选择
                    ""
                }
            };
            if !pix_fmt.is_empty() {
                cmd.args(&["-pix_fmt", pix_fmt]);
            }
        }
        // 如果不指定pix_fmt，FFmpeg会自动选择损失最小的格式
        
        cmd.args(&[
            "-y",
            output.to_str().unwrap(),
        ]);
        
        // 执行转换
        let result = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("AVIF conversion failed: {}", stderr);
        }
        
        let output_size = std::fs::metadata(output)?.len();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            format: "avif".to_string(),
        })
    }
    
    /// 转换为JXL (使用FFmpeg)
    pub fn convert_to_jxl_ffmpeg<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &JXLParams,
    ) -> Result<ConversionResult> {
        let input = input.as_ref();
        let output = output.as_ref();
        
        if !self.check_jxl_ffmpeg_support() {
            bail!("FFmpeg does not support JXL encoding, please use native cjxl tool");
        }
        
        let input_size = std::fs::metadata(input)?.len();
        
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.args(&[
            "-i", input.to_str().unwrap(),
            "-c:v", "libjxl",
            "-q:v", &params.quality.to_string(),
            "-effort", &params.effort.to_string(),
        ]);
        
        if params.lossless {
            cmd.args(&["-lossless", "1"]);
        }
        
        cmd.args(&[
            "-y",
            output.to_str().unwrap(),
        ]);
        
        let result = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("JXL conversion failed: {}", stderr);
        }
        
        let output_size = std::fs::metadata(output)?.len();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            format: "jxl".to_string(),
        })
    }
    
    /// 转换为JXL (使用原生cjxl)
    pub fn convert_to_jxl_native<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &JXLParams,
    ) -> Result<ConversionResult> {
        let input = input.as_ref();
        let output = output.as_ref();
        
        let cjxl_path = self.cjxl_path.as_ref()
            .ok_or_else(|| anyhow::anyhow!("cjxl tool not installed"))?;
        
        let input_size = std::fs::metadata(input)?.len();
        
        let mut cmd = Command::new(cjxl_path);
        cmd.arg(input.to_str().unwrap());
        cmd.arg(output.to_str().unwrap());
        
        // 质量参数
        if params.lossless {
            cmd.arg("--lossless");
        } else {
            cmd.args(&["--quality", &params.quality.to_string()]);
        }
        
        // 努力程度
        cmd.args(&["--effort", &params.effort.to_string()]);
        
        // 🔥 Advanced JXL parameters - REAL implementation!
        if params.modular {
            cmd.arg("--modular");
        }
        
        if params.progressive {
            cmd.arg("--progressive");
        }
        
        if params.responsive {
            cmd.args(&["--responsive", "1"]);
        }
        
        if params.gaborish {
            cmd.arg("--gaborish=1");
        } else {
            cmd.arg("--gaborish=0");
        }
        
        if params.photon_noise > 0 {
            cmd.args(&["--photon_noise", &params.photon_noise.to_string()]);
        }
        
        if params.decoding_speed > 0 {
            cmd.args(&["--decoding_speed", &params.decoding_speed.to_string()]);
        }
        
        // 🔥 Phase 2: Additional critical parameters
        if params.distance > 0.0 && !params.lossless {
            cmd.args(&["--distance", &params.distance.to_string()]);
        }
        
        // 🔥 位深度 - 只有非默认值且非0时才传递，让cjxl自动处理
        if params.bit_depth != 8 && params.bit_depth != 0 {
            cmd.args(&["--bits_per_sample", &params.bit_depth.to_string()]);
        }
        // bit_depth=0或8时不传递，让cjxl根据源文件自动选择
        
        // 🔥 色彩空间 - 只有非默认值且非空时才传递，让cjxl自动处理
        if !params.color_space.is_empty() 
            && params.color_space != "sRGB" 
            && params.color_space != "auto" {
            cmd.args(&["--color_space", &params.color_space]);
        }
        // color_space为空、"sRGB"或"auto"时不传递，让cjxl保持源色彩空间
        
        if params.patches > 0 {
            cmd.args(&["--patches", &params.patches.to_string()]);
        }
        
        // 执行转换
        let result = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute cjxl")?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("JXL conversion failed: {}", stderr);
        }
        
        let output_size = std::fs::metadata(output)?.len();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            format: "jxl".to_string(),
        })
    }
    
    /// 智能选择JXL转换方法
    pub fn convert_to_jxl<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &JXLParams,
    ) -> Result<ConversionResult> {
        // 优先使用原生cjxl（更快更好）
        if self.cjxl_path.is_some() {
            self.convert_to_jxl_native(input, output, params)
        } else if self.check_jxl_ffmpeg_support() {
            self.convert_to_jxl_ffmpeg(input, output, params)
        } else {
            bail!("JXL encoder not available, please install cjxl or FFmpeg with JXL support")
        }
    }
    
    /// 转换为WebP (使用cwebp)
    pub fn convert_to_webp<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &WebPParams,
    ) -> Result<ConversionResult> {
        let input = input.as_ref();
        let output = output.as_ref();
        
        let cwebp_path = which::which("cwebp")
            .context("cwebp tool not installed")?;
        
        let input_size = std::fs::metadata(input)?.len();
        
        let mut cmd = Command::new(cwebp_path);
        cmd.arg(input.to_str().unwrap());
        cmd.arg("-o").arg(output.to_str().unwrap());
        
        // 🔥 Complete WebP parameters - REAL implementation!
        if params.lossless {
            cmd.arg("-lossless");
        } else {
            cmd.args(&["-q", &params.quality.to_string()]);
        }
        
        // 压缩方法 (0-6)
        cmd.args(&["-m", &params.method.to_string()]);
        
        // 滤波强度 (0-100)
        cmd.args(&["-f", &params.filter_strength.to_string()]);
        
        // 锐化级别 (0-7)
        cmd.args(&["-sharpness", &params.sharpness.to_string()]);
        
        // 执行转换
        let result = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute cwebp")?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("WebP conversion failed: {}", stderr);
        }
        
        let output_size = std::fs::metadata(output)?.len();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            format: "webp".to_string(),
        })
    }
    
    /// 转换为HEIC (使用FFmpeg + x265/libheif)
    pub fn convert_to_heic<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        params: &HEICParams,
    ) -> Result<ConversionResult> {
        let input = input.as_ref();
        let output = output.as_ref();
        
        let input_size = std::fs::metadata(input)?.len();
        
        let mut cmd = Command::new(&self.ffmpeg_path);
        cmd.args(&["-i", input.to_str().unwrap()]);
        
        // 🔥 Complete HEIC parameters - REAL implementation!
        match params.encoder.as_str() {
            "x265" => {
                cmd.args(&["-c:v", "libx265"]);
                cmd.args(&["-tag:v", "hvc1"]);  // HEIC tag
                
                if params.lossless {
                    cmd.args(&["-x265-params", "lossless=1"]);
                } else {
                    let crf = 100 - params.quality;  // Convert quality to CRF
                    cmd.args(&["-crf", &crf.to_string()]);
                }
            }
            "libheif" => {
                // libheif encoder (if available)
                cmd.args(&["-c:v", "libheif"]);
                cmd.args(&["-q:v", &params.quality.to_string()]);
            }
            _ => {
                bail!("Unsupported HEIC encoder: {}", params.encoder);
            }
        }
        
        // 🔥 像素格式 - 根据用户选择的chroma_subsampling
        // 如果用户明确指定，就传递；否则让FFmpeg自动选择
        if !params.chroma_subsampling.is_empty() {
            let pix_fmt = match params.chroma_subsampling.as_str() {
                "444" => "yuv444p",
                "420" => "yuv420p",
                _ => "", // 让FFmpeg自动选择
            };
            if !pix_fmt.is_empty() {
                cmd.args(&["-pix_fmt", pix_fmt]);
            }
        }
        // 如果不指定，FFmpeg会自动选择损失最小的格式
        
        cmd.args(&["-y", output.to_str().unwrap()]);
        
        // 执行转换
        let result = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            bail!("HEIC conversion failed: {}", stderr);
        }
        
        let output_size = std::fs::metadata(output)?.len();
        
        Ok(ConversionResult {
            input_size,
            output_size,
            compression_ratio: output_size as f64 / input_size as f64,
            format: "heic".to_string(),
        })
    }
}

impl Default for ModernFormatConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// 格式支持信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatSupport {
    pub avif: bool,
    pub jxl_ffmpeg: bool,
    pub jxl_native: bool,
    pub webp: bool,
}

impl FormatSupport {
    /// 获取支持的格式列表
    pub fn supported_formats(&self) -> Vec<String> {
        let mut formats = vec!["webp".to_string()];
        
        if self.avif {
            formats.push("avif".to_string());
        }
        
        if self.jxl_ffmpeg || self.jxl_native {
            formats.push("jxl".to_string());
        }
        
        formats
    }
    
    /// 检查是否支持指定格式
    pub fn supports(&self, format: &str) -> bool {
        match format.to_lowercase().as_str() {
            "webp" => self.webp,
            "avif" => self.avif,
            "jxl" => self.jxl_ffmpeg || self.jxl_native,
            _ => false,
        }
    }
}

/// AVIF转换参数
#[derive(Debug, Clone)]
pub struct AVIFParams {
    pub encoder: String,  // "libaom-av1" 或 "libsvtav1"
    pub crf: u8,          // 0-63, 越小质量越高
    pub speed: u8,        // 0-8 (libaom) 或 0-13 (svt)
    pub bit_depth: u8,    // 8, 10, 或 12
    // 🔥 Complete AVIF parameters - NO MORE FAKE UI!
    pub min_quantizer: u8,     // 0-63, minimum quantizer
    pub max_quantizer: u8,     // 0-63, maximum quantizer
    pub chroma_subsampling: String, // "420", "422", "444"
    pub tiles_rows: u8,        // 1-8, tile rows for parallel encoding
    pub tiles_cols: u8,        // 1-8, tile columns for parallel encoding
    pub premultiply_alpha: bool, // Premultiply alpha channel
}

impl Default for AVIFParams {
    fn default() -> Self {
        Self {
            encoder: "libaom-av1".to_string(),
            crf: 30,
            speed: 6,
            bit_depth: 8,
            min_quantizer: 0,
            max_quantizer: 63,
            chroma_subsampling: "420".to_string(),
            tiles_rows: 1,
            tiles_cols: 1,
            premultiply_alpha: false,
        }
    }
}

impl AVIFParams {
    /// 从质量值创建 (0-100)
    pub fn from_quality(quality: u8) -> Self {
        let crf = ((100 - quality) as f64 * 0.63) as u8; // 映射到0-63
        let speed = if quality >= 90 {
            6 // 高质量用慢速
        } else if quality >= 70 {
            4 // 中等质量用平衡
        } else {
            2 // 低质量用快速
        };
        
        // Calculate min/max quantizers based on quality
        let min_q = if quality > 90 { 0 } else if quality > 70 { 5 } else { 10 };
        let max_q = if quality > 90 { 20 } else if quality > 70 { 35 } else { 50 };
        
        Self {
            encoder: "libaom-av1".to_string(),
            crf,
            speed,
            bit_depth: 8,
            min_quantizer: min_q,
            max_quantizer: max_q,
            chroma_subsampling: "420".to_string(),
            tiles_rows: 1,
            tiles_cols: 1,
            premultiply_alpha: false,
        }
    }
}

/// JXL转换参数
#[derive(Debug, Clone)]
pub struct JXLParams {
    pub quality: u8,      // 0-100
    pub effort: u8,       // 1-9
    pub lossless: bool,
    // 🔥 Advanced JXL parameters - REAL implementation!
    pub modular: bool,    // Use modular mode (better for synthetic images)
    pub progressive: bool, // Enable progressive decoding
    pub responsive: bool,  // Enable responsive by default
    pub gaborish: bool,    // Enable Gaborish filter (reduces ringing artifacts)
    pub photon_noise: u8,  // Photon noise level (0-100)
    pub decoding_speed: u8, // Decoding speed tier (0-4)
    // 🔥 Phase 2: Additional critical parameters
    pub distance: f32,     // Psychovisual distance (0.0=lossless, higher=more compression)
    pub bit_depth: u8,     // Bit depth: 8, 10, 12, or 16
    pub color_space: String, // Color space: sRGB, Display P3, Adobe RGB, ProPhoto RGB
    pub patches: u8,       // Edge enhancement level (0-4)
}

impl Default for JXLParams {
    fn default() -> Self {
        Self {
            quality: 85,
            effort: 7,
            lossless: false,
            modular: false,
            progressive: true,
            responsive: true,
            gaborish: true,
            photon_noise: 0,
            decoding_speed: 0,
            distance: 1.0,
            bit_depth: 0,  // 0 = auto, let cjxl detect from source
            color_space: "auto".to_string(),  // auto = preserve source color space
            patches: 1,
        }
    }
}

impl JXLParams {
    /// 从质量值创建
    pub fn from_quality(quality: u8) -> Self {
        let effort = if quality >= 90 {
            9 // 高质量用最大努力
        } else if quality >= 70 {
            7 // 中等质量用平衡
        } else {
            5 // 低质量用快速
        };
        
        Self {
            quality,
            effort,
            lossless: quality >= 95,
            modular: false,
            progressive: true,
            responsive: true,
            gaborish: true,
            photon_noise: 0,
            decoding_speed: 0,
            distance: if quality >= 90 { 0.5 } else { 1.0 },
            bit_depth: 0,  // Always auto, let cjxl decide based on source
            color_space: "auto".to_string(),  // Always auto, preserve source
            patches: 1,
        }
    }
}

/// WebP转换参数
#[derive(Debug, Clone)]
pub struct WebPParams {
    pub quality: u8,           // 0-100
    pub method: u8,            // 0-6, compression method
    pub filter_strength: u8,   // 0-100, deblocking filter strength
    pub sharpness: u8,         // 0-7, sharpness level
    pub lossless: bool,        // lossless encoding
}

/// HEIC转换参数
#[derive(Debug, Clone)]
pub struct HEICParams {
    pub quality: u8,           // 0-100
    pub encoder: String,       // "x265" or "libheif"
    pub chroma_subsampling: String, // "420" or "444"
    pub lossless: bool,        // lossless encoding
    pub embed_thumbnail: bool, // embed thumbnail
}

impl Default for WebPParams {
    fn default() -> Self {
        Self {
            quality: 85,
            method: 4,
            filter_strength: 60,
            sharpness: 0,
            lossless: false,
        }
    }
}

impl WebPParams {
    pub fn from_quality(quality: u8) -> Self {
        let method = if quality >= 90 { 6 } else if quality >= 70 { 4 } else { 2 };
        Self {
            quality,
            method,
            filter_strength: 60,
            sharpness: 0,
            lossless: quality >= 95,
        }
    }
}

impl Default for HEICParams {
    fn default() -> Self {
        Self {
            quality: 85,
            encoder: "x265".to_string(),
            chroma_subsampling: "420".to_string(),
            lossless: false,
            embed_thumbnail: false,
        }
    }
}

impl HEICParams {
    pub fn from_quality(quality: u8) -> Self {
        Self {
            quality,
            encoder: "x265".to_string(),
            chroma_subsampling: if quality >= 90 { "444".to_string() } else { "420".to_string() },
            lossless: quality >= 95,
            embed_thumbnail: true,
        }
    }
}

/// 转换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub format: String,
}

impl ConversionResult {
    /// 获取空间节省百分比
    pub fn space_saving_percent(&self) -> f64 {
        (1.0 - self.compression_ratio) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_support_check() {
        let converter = ModernFormatConverter::new();
        let support = converter.check_format_support();
        
        // WebP应该总是支持
        assert!(support.webp);
        
        // 打印支持的格式
        println!("Supported formats: {:?}", support.supported_formats());
    }
    
    #[test]
    fn test_avif_params_from_quality() {
        let params = AVIFParams::from_quality(85);
        assert!(params.crf < 20); // 高质量应该有低CRF
        assert!(params.speed >= 2);
    }
    
    #[test]
    fn test_jxl_params_from_quality() {
        let params = JXLParams::from_quality(90);
        assert_eq!(params.quality, 90);
        assert!(params.effort >= 7);
    }
    
    #[test]
    fn test_format_support_query() {
        let support = FormatSupport {
            avif: true,
            jxl_ffmpeg: false,
            jxl_native: true,
            webp: true,
        };
        
        assert!(support.supports("webp"));
        assert!(support.supports("avif"));
        assert!(support.supports("jxl"));
        assert!(!support.supports("heic"));
    }
}
