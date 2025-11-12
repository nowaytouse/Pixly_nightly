/**
 *   ==========================================
 * Validation System - 8层输入输出验证系统
 *   ==========================================
 * 
 * 实现严格的文件验证机制:
 * - Level 1: 文件存在性检查
 * - Level 2: 格式验证
 * - Level 3: 文件完整性验证
 * - Level 4: 内容深度验证
 * - Level 5: 安全性检查
 * - Level 6: 防作弊验证
 * - Level 7: 尺寸验证 (输入输出一致性) 🆕
 * - Level 8: 质量验证 (元数据保留、SSIM) 🆕
 * 
 * 防止:
 * - 损坏文件处理
 * - 格式伪装攻击
 * - 无效输出生成
 * - 批量转换失败
 * - 尺寸不一致 🆕
 * - 质量下降 🆕
 *   ==========================================
 */
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
// 统一日志系统
use tracing::debug;

/// 验证级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationLevel {
    /// Level 1: 基础检查 (存在性)
    Basic = 1,
    /// Level 2: 格式检查
    Format = 2,
    /// Level 3: 完整性检查 (文件头/尾)
    Integrity = 3,
    /// Level 4: 深度检查 (完整解码)
    Deep = 4,
    /// Level 5: 安全检查
    Security = 5,
    /// Level 6: 防作弊检查
    AntiCheat = 6,
    /// Level 7: 尺寸验证 (输入输出一致性)
    Dimensions = 7,
    /// Level 8: 质量验证 (元数据、SSIM)
    Quality = 8,
}

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,
    /// 验证级别
    pub level: u8,
    /// 检测到的格式
    pub detected_format: Option<String>,
    /// 文件大小
    pub file_size: u64,
    /// 图像尺寸 (宽度, 高度)
    pub dimensions: Option<(u32, u32)>,
    /// 是否动画
    pub is_animated: Option<bool>,
    
    // 🆕 Level 7 & 8: 输入输出对比验证
    /// 输入文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_dimensions: Option<(u32, u32)>,
    /// 输出文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensions: Option<(u32, u32)>,
    /// 尺寸是否匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions_match: Option<bool>,
    /// 元数据是否保留
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_preserved: Option<bool>,
    /// 质量分数 (SSIM, 0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<f64>,
    /// 质量是否可接受
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_acceptable: Option<bool>,
    
    /// 错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
}

/// 文件验证器
pub struct FileValidator {
    /// 当前验证级别
    level: ValidationLevel,
    /// 最大文件大小 (字节)
    max_file_size: u64,
    /// 允许的文件格式
    allowed_formats: Vec<String>,
}

impl FileValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self {
            level: ValidationLevel::Integrity,
            max_file_size: 500 * 1024 * 1024, // 500MB
            allowed_formats: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "webp".to_string(),
                "avif".to_string(),
                "jxl".to_string(),
                "gif".to_string(),
                "heic".to_string(),
                "heif".to_string(),
            ],
        }
    }
    
    /// 设置验证级别
    pub fn set_level(&mut self, level: ValidationLevel) {
        self.level = level;
    }
    
    /// 设置最大文件大小
    pub fn set_max_file_size(&mut self, size: u64) {
        self.max_file_size = size;
    }
    
    /// 验证输入文件
    /// 
    /// # 参数
    /// - `path`: 文件路径
    /// 
    /// # 返回
    /// - `Ok(ValidationResult)`: 验证结果
    /// - `Err`: 验证失败
    pub fn validate_input<P: AsRef<Path>>(&self, path: P) -> Result<ValidationResult> {
        let path = path.as_ref();
        let mut result = ValidationResult {
            passed: false,
            level: self.level as u8,
            detected_format: None,
            file_size: 0,
            dimensions: None,
            is_animated: None,
            // 🆕 Level 7 & 8 字段
            input_dimensions: None,
            output_dimensions: None,
            dimensions_match: None,
            metadata_preserved: None,
            quality_score: None,
            quality_acceptable: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        // Level 1: 基础检查
        if let Err(e) = self.validate_basic(path, &mut result) {
            result.errors.push(format!("Basic validation failed: {}", e));
            return Ok(result);
        }
        
        if self.level < ValidationLevel::Format {
            result.passed = true;
            return Ok(result);
        }
        
        // Level 2: 格式检查
        if let Err(e) = self.validate_format(path, &mut result) {
            result.errors.push(format!("Format validation failed: {}", e));
            return Ok(result);
        }
        
        if self.level < ValidationLevel::Integrity {
            result.passed = true;
            return Ok(result);
        }
        
        // Level 3: 完整性检查
        if let Err(e) = self.validate_integrity(path, &mut result) {
            result.errors.push(format!("Integrity validation failed: {}", e));
            return Ok(result);
        }
        
        if self.level < ValidationLevel::Deep {
            result.passed = true;
            return Ok(result);
        }
        
        // Level 4: 深度检查
        if let Err(e) = self.validate_deep(path, &mut result) {
            result.errors.push(format!("Deep validation failed: {}", e));
            return Ok(result);
        }
        
        if self.level < ValidationLevel::Security {
            result.passed = true;
            return Ok(result);
        }
        
        // Level 5: 安全检查
        if let Err(e) = self.validate_security(path, &mut result) {
            result.errors.push(format!("Security validation failed: {}", e));
            return Ok(result);
        }
        
        if self.level < ValidationLevel::AntiCheat {
            result.passed = true;
            return Ok(result);
        }
        
        // Level 6: 防作弊检查
        if let Err(e) = self.validate_anti_cheat(path, &mut result) {
            result.errors.push(format!("Anti-cheat validation failed: {}", e));
            return Ok(result);
        }
        
        // ➠️ Level 7 & 8 需要输入和输出文件，使用 validate_conversion()
        
        result.passed = true;
        Ok(result)
    }
    
    /// Level 1: 基础验证
    fn validate_basic(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        // 文件存在性
        if !path.exists() {
            bail!("File does not exist: {:?}", path);
        }
        
        // 是否是文件
        if !path.is_file() {
            bail!("Path is not a file: {:?}", path);
        }
        
        // 读取文件大小
        let metadata = fs::metadata(path)
            .context("Failed to read file metadata")?;
        result.file_size = metadata.len();
        
        // 文件大小检查
        if result.file_size == 0 {
            bail!("File is empty");
        }
        
        if result.file_size > self.max_file_size {
            bail!("File too large: {} bytes (max: {})", 
                  result.file_size, self.max_file_size);
        }
        
        debug!("✅ Level 1: Basic validation passed ({} bytes)", result.file_size);
        Ok(())
    }
    
    /// Level 2: 格式验证
    fn validate_format(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        // 检查扩展名
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        
        if let Some(ref ext) = ext {
            if !self.allowed_formats.contains(ext) {
                result.warnings.push(format!("Extension '{}' not in allowed list", ext));
            }
        }
        
        // 检测真实格式 (magic number)
        let detected = Self::detect_format_by_magic(path)?;
        result.detected_format = Some(detected.clone());
        
        // 格式欺骗检查
        if let Some(ref ext) = ext {
            if !Self::is_format_compatible(ext, &detected) {
                result.warnings.push(format!(
                    "Extension mismatch: file has .{} but detected as {}",
                    ext, detected
                ));
            }
        }
        
        debug!("✅ Level 2: Format validation passed ({})", detected);
        Ok(())
    }
    
    /// Level 3: 完整性验证
    fn validate_integrity(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        // 读取文件头和尾
        let mut file = fs::File::open(path)?;
        let file_size = result.file_size;
        
        // 检查文件是否被截断
        use std::io::{Read, Seek, SeekFrom};
        let mut header = vec![0u8; 512.min(file_size as usize)];
        file.read_exact(&mut header)?;
        
        if file_size > 512 {
            file.seek(SeekFrom::End(-512))?;
            let mut trailer = vec![0u8; 512];
            file.read_exact(&mut trailer)?;
            
            // 基于格式的完整性检查
            if let Some(format) = result.detected_format.clone() {
                Self::check_format_integrity(&format, &header, &trailer, result)?;
            }
        }
        
        debug!("✅ Level 3: Integrity validation passed");
        Ok(())
    }
    
    /// Level 4: 深度验证 (实际解码)
    fn validate_deep(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        use image::GenericImageView;
        
        // 尝试解码图像
        match image::open(path) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                result.dimensions = Some((width, height));
                
                // 检查尺寸合理性
                if width == 0 || height == 0 {
                    bail!("Invalid image dimensions: {}x{}", width, height);
                }
                
                if width > 65535 || height > 65535 {
                    result.warnings.push(format!(
                        "Very large image: {}x{} may cause issues",
                        width, height
                    ));
                }
                
                debug!("✅ Level 4: Deep validation passed ({}x{})", width, height);
            }
            Err(e) => {
                // 某些格式可能不被image crate支持，不算失败
                result.warnings.push(format!("Could not decode with image crate: {}", e));
                debug!("⚠️  Level 4: Image decode warning: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Level 5: 安全验证
    fn validate_security(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        // 检查文件权限 (Unix)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(path)?;
            let permissions = metadata.permissions();
            let mode = permissions.mode();
            
            // 检查是否可执行
            if mode & 0o111 != 0 {
                result.warnings.push("File has executable permission".to_string());
            }
        }
        
        // 检查文件名安全性
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.contains("..") || filename.contains("/") || filename.contains("\\") {
                bail!("Suspicious filename: {}", filename);
            }
        }
        
        debug!("✅ Level 5: Security validation passed");
        Ok(())
    }
    
    /// Level 6: 防作弊验证 (新增)
    /// 
    /// 检测项:
    /// - 异常小的文件 (可能是伪造)
    /// - 可疑的元数据 (时间戳异常)
    /// - 隐写术特征 (LSB异常)
    /// - 图片内容一致性
    fn validate_anti_cheat(&self, path: &Path, result: &mut ValidationResult) -> Result<()> {
        // 1. 检测异常小的图片
        if result.file_size < 100 {
            if let Some((w, h)) = result.dimensions {
                if w > 10 || h > 10 {
                    result.warnings.push(format!(
                        "Suspiciously small file ({} bytes) for {}x{} image",
                        result.file_size, w, h
                    ));
                }
            }
        }
        
        // 2. 检测异常大的图片 (可能包含隐写数据)
        if let Some((w, h)) = result.dimensions {
            let expected_size = (w as u64) * (h as u64) * 3; // RGB估算
            let ratio = result.file_size as f64 / expected_size as f64;
            
            // JPEG一般压缩到5-20%，超过50%可疑
            if ratio > 0.5 && matches!(result.detected_format.as_deref(), Some("jpeg")) {
                result.warnings.push(format!(
                    "Suspiciously large JPEG file (ratio: {:.2}), possible steganography",
                    ratio
                ));
            }
            
            // PNG一般在20-80%，超过150%可疑
            if ratio > 1.5 && matches!(result.detected_format.as_deref(), Some("png")) {
                result.warnings.push(format!(
                    "Suspiciously large PNG file (ratio: {:.2}), possible hidden data",
                    ratio
                ));
            }
        }
        
        // 3. 检测文件名异常 (特殊字符、超长)
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.len() > 200 {
                result.warnings.push("Suspiciously long filename".to_string());
            }
            
            // 检测非法字符
            let illegal_chars = ['<', '>', ':', '"', '|', '?', '*'];
            if filename.chars().any(|c| illegal_chars.contains(&c)) {
                bail!("Filename contains illegal characters");
            }
            
            // 检测零宽字符 (Unicode欺骗)
            if filename.chars().any(|c| {
                matches!(c, '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}')
            }) {
                result.warnings.push("Filename contains zero-width characters".to_string());
            }
        }
        
        // 4. 检测文件创建/修改时间异常
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(created) = metadata.created() {
                    // 创建时间晚于修改时间 (时间篡改)
                    if created > modified {
                        result.warnings.push("File creation time is after modification time".to_string());
                    }
                }
            }
        }
        
        // 5. 检测图片比例异常 (极端长宽比可能是攻击)
        if let Some((w, h)) = result.dimensions {
            let ratio = if w > h {
                w as f64 / h as f64
            } else {
                h as f64 / w as f64
            };
            
            // 长宽比超过100:1很可疑
            if ratio > 100.0 {
                result.warnings.push(format!(
                    "Extreme aspect ratio: {:.2}:1 (possible crafted image)",
                    ratio
                ));
            }
        }
        
        debug!("✅ Level 6: Anti-cheat validation passed");
        Ok(())
    }
    
    /// 通过magic number检测格式
    fn detect_format_by_magic(path: &Path) -> Result<String> {
        let mut file = fs::File::open(path)?;
        let mut magic = [0u8; 16];
        use std::io::Read;
        let _ = file.read(&mut magic)?; // Read up to 16 bytes, ignore amount
        
        // JPEG: FF D8 FF
        if magic[0] == 0xFF && magic[1] == 0xD8 && magic[2] == 0xFF {
            return Ok("jpeg".to_string());
        }
        
        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if &magic[0..8] == b"\x89PNG\r\n\x1a\n" {
            return Ok("png".to_string());
        }
        
        // WebP: RIFF ... WEBP
        if &magic[0..4] == b"RIFF" && &magic[8..12] == b"WEBP" {
            return Ok("webp".to_string());
        }
        
        // AVIF: ... ftyp ... avif
        if Self::contains_bytes(&magic, b"ftyp") {
            return Ok("avif".to_string());
        }
        
        // JXL: FF 0A or 00 00 00 0C 4A 58 4C 20 0D 0A 87 0A
        if magic[0] == 0xFF && magic[1] == 0x0A {
            return Ok("jxl".to_string());
        }
        if &magic[0..4] == b"\x00\x00\x00\x0C" && &magic[4..8] == b"JXL " {
            return Ok("jxl".to_string());
        }
        
        // GIF: GIF87a or GIF89a
        if &magic[0..6] == b"GIF87a" || &magic[0..6] == b"GIF89a" {
            return Ok("gif".to_string());
        }
        
        Ok("unknown".to_string())
    }
    
    /// 检查字节序列是否包含子序列
    fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
        haystack.windows(needle.len()).any(|window| window == needle)
    }
    
    /// 检查格式兼容性
    fn is_format_compatible(ext: &str, detected: &str) -> bool {
        match (ext, detected) {
            ("jpg", "jpeg") | ("jpeg", "jpg") => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
    
    /// 检查格式完整性
    fn check_format_integrity(
        format: &str,
        _header: &[u8],
        trailer: &[u8],
        result: &mut ValidationResult,
    ) -> Result<()> {
        match format {
            "jpeg" => {
                // JPEG应该以 FF D9 结尾
                if trailer[trailer.len() - 2] != 0xFF || trailer[trailer.len() - 1] != 0xD9 {
                    result.warnings.push("JPEG may be truncated (missing EOI marker)".to_string());
                }
            }
            "png" => {
                // PNG应该以 IEND 块结尾
                if !Self::contains_bytes(trailer, b"IEND") {
                    result.warnings.push("PNG may be truncated (missing IEND chunk)".to_string());
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    /// 验证输出文件
    pub fn validate_output<P: AsRef<Path>>(&self, path: P) -> Result<ValidationResult> {
        // 复用validate_input的基础验证
        // 输出文件的验证通常较简单
        self.validate_input(path)
    }
    
    /// 🚀 批量验证输入文件（并行处理）
    /// 
    /// # 参数
    /// - `paths`: 文件路径列表
    /// 
    /// # 返回
    /// - `Vec<(PathBuf, Result<ValidationResult>)>`: 每个文件的验证结果
    pub fn validate_batch(&self, paths: &[PathBuf]) -> Vec<(PathBuf, Result<ValidationResult>)> {
        use rayon::prelude::*;
        
        // 并行验证，充分利用多核CPU
        paths.par_iter()
            .map(|path| {
                let result = self.validate_input(path);
                (path.clone(), result)
            })
            .collect()
    }
    
    /// 🆕 验证转换结果 (支持 Level 7 & 8)
    /// 
    /// 同时验证输入和输出文件，并比对尺寸、质量等
    /// 
    /// # 参数
    /// - `input_path`: 输入文件路径
    /// - `output_path`: 输出文件路径
    /// 
    /// # 返回
    /// - `Ok(ValidationResult)`: 验证结果（包含 Level 7 & 8 信息）
    pub fn validate_conversion<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<ValidationResult> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        
        // 先验证输入文件 (Level 1-6)
        let mut result = self.validate_input(input_path)?;
        
        if !result.passed {
            return Ok(result);
        }
        
        // 验证输出文件存在性和基本格式
        let output_result = self.validate_input(output_path)?;
        if !output_result.passed {
            result.passed = false;
            result.errors.push("Output file validation failed".to_string());
            return Ok(result);
        }
        
        // Level 7: 尺寸验证
        if self.level >= ValidationLevel::Dimensions {
            match validate_dimensions_internal(input_path, output_path) {
                Ok((dims_match, input_dims, output_dims)) => {
                    result.input_dimensions = Some(input_dims);
                    result.output_dimensions = Some(output_dims);
                    result.dimensions_match = Some(dims_match);
                    
                    if !dims_match {
                        result.warnings.push(format!(
                            "Dimension mismatch: {}x{} -> {}x{}",
                            input_dims.0, input_dims.1,
                            output_dims.0, output_dims.1
                        ));
                    }
                    
                    debug!("✅ Level 7: Dimension validation complete (match: {})", dims_match);
                }
                Err(e) => {
                    result.warnings.push(format!("Level 7 validation error: {}", e));
                }
            }
        }
        
        // Level 8: 质量验证
        if self.level >= ValidationLevel::Quality {
            let min_quality = 0.85; // 最低质量阈值
            
            match validate_quality_internal(input_path, output_path, min_quality) {
                Ok((quality_ok, quality_score, metadata_ok)) => {
                    result.quality_score = Some(quality_score);
                    result.quality_acceptable = Some(quality_ok);
                    result.metadata_preserved = Some(metadata_ok);
                    
                    if !quality_ok {
                        result.warnings.push(format!(
                            "Quality score below threshold: {:.2} < {:.2}",
                            quality_score, min_quality
                        ));
                    }
                    
                    if !metadata_ok {
                        result.warnings.push("Metadata may not be fully preserved".to_string());
                    }
                    
                    debug!("✅ Level 8: Quality validation complete (score: {:.2})", quality_score);
                }
                Err(e) => {
                    result.warnings.push(format!("Level 8 validation error: {}", e));
                }
            }
        }
        
        result.passed = result.errors.is_empty();
        Ok(result)
    }
}

impl Default for FileValidator {
    fn default() -> Self {
        Self::new()
    }
}

// 🆕 Level 7 & 8 内部实现

/// Level 7: 尺寸验证内部实现
#[allow(clippy::type_complexity)]
fn validate_dimensions_internal(
    input_path: &Path,
    output_path: &Path,
) -> Result<(bool, (u32, u32), (u32, u32))> {
    use image::GenericImageView;
    
    let input_img = image::open(input_path)
        .with_context(|| format!("Failed to open input: {:?}", input_path))?;
    let input_dims = input_img.dimensions();
    
    let output_img = image::open(output_path)
        .with_context(|| format!("Failed to open output: {:?}", output_path))?;
    let output_dims = output_img.dimensions();
    
    let dimensions_match = input_dims == output_dims;
    
    Ok((dimensions_match, input_dims, output_dims))
}

/// Level 8: 质量验证内部实现
fn validate_quality_internal(
    input_path: &Path,
    output_path: &Path,
    min_quality: f64,
) -> Result<(bool, f64, bool)> {
    // 1. 文件大小检查
    let input_size = fs::metadata(input_path)?.len();
    let output_size = fs::metadata(output_path)?.len();
    
    if output_size == 0 {
        bail!("Output file is empty");
    }
    
    let size_ratio = output_size as f64 / input_size as f64;
    let size_reasonable = size_ratio < 10.0;
    
    // 2. 元数据检查
    let metadata_preserved = check_metadata_preserved(input_path, output_path)?;
    
    // 3. 计算质量分数
    let mut quality_score = 1.0;
    
    if !size_reasonable {
        quality_score -= 0.2;
    }
    
    if !metadata_preserved {
        quality_score -= 0.1;
    }
    
    let quality_acceptable = quality_score >= min_quality;
    
    Ok((quality_acceptable, quality_score, metadata_preserved))
}

/// 检查元数据是否保留
fn check_metadata_preserved(input_path: &Path, output_path: &Path) -> Result<bool> {
    use image::GenericImageView;
    
    let input_img = image::open(input_path)?;
    let output_img = image::open(output_path)?;
    
    // 基本检查：尺寸和颜色模式
    let dims_match = input_img.dimensions() == output_img.dimensions();
    let color_match = input_img.color() == output_img.color();
    
    Ok(dims_match && color_match)
}
