/**
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * AI Parameter Client - Go AI服务参数客户端
 * ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 * 
 * Phase 46.10: 架构角色明确
 * 
 * 🎯 核心定位:
 * 
 * 【Rust职责】
 * - ✅ 调用Go AI服务获取参数推荐
 * - ✅ 传递图像特征给AI服务
 * - ✅ 接收并验证AI返回的参数
 * - ❌ 不做AI决策和智能优化
 * - ❌ 不做参数选择逻辑
 * 
 * 【Go AI服务职责】
 * - ✅ AI模型推理和参数优化
 * - ✅ 图像特征分析 (SWT, 复杂度)
 * - ✅ 智能参数推荐 (Quality, Speed, Distance, Effort)
 * - ✅ 格式和工具选择建议
 * 
 * 【架构原则】
 * - Rust = 唯一执行层（文件处理+格式转换）
 * - Go AI = 完整AI服务（参数优化+智能决策）
 * - 此文件 = Go AI服务的Rust客户端
 * 
 * 【使用场景】
 * - 用户未明确指定参数时，调用AI获取推荐
 * - 用户明确指定参数时，跳过AI调用
 * 
 * 参考:
 * - core/go/ai/ (Go AI服务实现)
 * - core/rust/src/converter/ai_client.rs (HTTP客户端)
 */
use anyhow::{Context, Result};
use std::path::Path;
use serde::{Deserialize, Serialize};
use image::GenericImageView;
// 🔧 统一日志系统
use tracing::debug;

// 🔥 重构：optimizers 已移至 param_optimizers.rs
use crate::converter::ai_parameter_provider as optimizers;  // Phase 46.9: 重命名

/// Image characteristics for parameter optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCharacteristics {
    /// Image dimensions
    pub width: u32,
    pub height: u32,
    /// File size in bytes
    pub file_size: u64,
    /// Input format
    pub format: String,
    /// Has transparency
    pub has_alpha: bool,
    /// Is animated
    pub is_animated: bool,
    /// Estimated complexity (0.0-1.0)
    pub complexity: f32,
    /// 🔥 修复：真实文件路径（用于AI预测）
    pub path: Option<String>,
}

/// Optimized conversion parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedParams {
    /// Recommended quality (0-100)
    pub quality: u8,
    /// Recommended speed (0-10)
    pub speed: u8,
    /// Use lossless mode
    pub lossless: bool,
    /// Format-specific options
    pub format_options: Vec<(String, String)>,
    /// Estimated output size (bytes)
    pub estimated_size: u64,
    /// Estimated compression ratio
    pub estimated_ratio: f32,
    /// Optimization strategy reason
    pub reason: String,
}

/// AI Parameter Client - Go AI服务的客户端
/// 
/// Phase 46.10: 重命名以明确职责
/// - 旧名: ParamOptimizer (误导，暗示Rust做优化)
/// - 新名: AIParameterClient (明确，这是AI服务客户端)
pub struct AIParameterClient {
    /// Target format
    target_format: String,
    /// User preferences
    prefer_quality: bool,
}

impl AIParameterClient {
    /// Create new AI parameter client
    pub fn new(target_format: &str) -> Self {
        Self {
            target_format: target_format.to_lowercase(),
            prefer_quality: false,
        }
    }

    /// Set quality preference
    pub fn set_prefer_quality(&mut self, prefer: bool) {
        self.prefer_quality = prefer;
    }

    /// Get parameters from AI service based on image characteristics
    /// 
    /// Phase 46.10: 重命名以明确职责
    /// - 旧名: optimize() (误导，暗示Rust做优化)
    /// - 新名: get_parameters_from_ai() (明确，从AI服务获取参数)
    pub fn get_parameters_from_ai(&self, chars: &ImageCharacteristics) -> Result<OptimizedParams> {
        match self.target_format.as_str() {
            "avif" => optimizers::get_ai_params_for_avif(chars, self.prefer_quality),
            "jxl" | "jpegxl" => optimizers::get_ai_params_for_jxl(chars, self.prefer_quality),
            "webp" => optimizers::get_ai_params_for_webp(chars, self.prefer_quality),
            "png" => optimizers::get_ai_params_for_png(chars, self.prefer_quality),
            "jpeg" | "jpg" => optimizers::get_ai_params_for_jpeg(chars, self.prefer_quality),
            _ => optimizers::get_ai_params_default(chars, self.prefer_quality),
        }
    }

    /// Analyze image to get characteristics
    pub fn analyze_image<P: AsRef<Path>>(path: P) -> Result<ImageCharacteristics> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to read file metadata: {}", path.display()))?;

        let file_size = metadata.len();

        // Detect format from extension
        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Try to open and analyze image
        let img = image::open(path)
            .with_context(|| format!("Failed to open image: {}", path.display()))?;

        let (width, height) = img.dimensions();
        let has_alpha = img.color().has_alpha();

        // 🔥 实现真实的复杂度分析
        let complexity = Self::calculate_image_complexity(&img);
        
        // 🔥 检测动画（基于格式）
        let is_animated = Self::detect_animation(path)?;

        Ok(ImageCharacteristics {
            width,
            height,
            file_size,
            format,
            has_alpha,
            is_animated,
            complexity: complexity as f32, // 转换为f32
            path: Some(path.to_string_lossy().to_string()), // 🔥 修复：真实路径
        })
    }
    
    /// 计算图像复杂度 (0.0 - 1.0)
    /// 基于颜色多样性、边缘密度和纹理复杂度
    fn calculate_image_complexity(img: &image::DynamicImage) -> f64 {
        use std::collections::HashSet;
        
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let total_pixels = (width * height) as usize;
        
        // 采样以提高性能（大图片采样1000个像素）
        let sample_size = total_pixels.min(1000);
        let step = (total_pixels / sample_size).max(1);
        
        // 1. 颜色多样性分析 (0.0 - 0.4权重)
        let mut unique_colors = HashSet::new();
        
        for (sampled, (_, _, pixel)) in rgba.enumerate_pixels().enumerate() {
            if sampled >= sample_size {
                break;
            }
            if sampled % step == 0 {
                // 量化颜色以减少唯一值（RGB各取高4位）
                let quantized = (
                    pixel[0] & 0xF0,
                    pixel[1] & 0xF0,
                    pixel[2] & 0xF0,
                );
                unique_colors.insert(quantized);
            }
        }
        
        let color_diversity = (unique_colors.len() as f64 / sample_size as f64).min(1.0);
        let color_score = color_diversity * 0.4;
        
        // 2. 边缘密度分析 (0.0 - 0.3权重)
        // 使用简化的Sobel算子检测边缘
        let gray = img.to_luma8();
        let mut edge_count = 0;
        let edge_sample = 200.min(total_pixels);
        
        for i in 0..edge_sample {
            let x = (i * width as usize / edge_sample) as u32;
            let y = (i * height as usize / edge_sample) as u32;
            
            if x > 0 && y > 0 && x < width - 1 && y < height - 1 {
                let center = gray.get_pixel(x, y)[0] as i32;
                let right = gray.get_pixel(x + 1, y)[0] as i32;
                let bottom = gray.get_pixel(x, y + 1)[0] as i32;
                
                let gx = (right - center).abs();
                let gy = (bottom - center).abs();
                let gradient = (gx + gy) / 2;
                
                if gradient > 30 {
                    edge_count += 1;
                }
            }
        }
        
        let edge_density = edge_count as f64 / edge_sample as f64;
        let edge_score = edge_density * 0.3;
        
        // 3. 亮度方差分析 (0.0 - 0.3权重)
        let mut luminance_sum = 0u64;
        let mut luminance_sq_sum = 0u64;
        let luma_sample = 500.min(total_pixels);
        
        for i in 0..luma_sample {
            let x = (i * width as usize / luma_sample) as u32;
            let y = (i * height as usize / luma_sample) as u32;
            
            if x < width && y < height {
                let luma = gray.get_pixel(x, y)[0] as u64;
                luminance_sum += luma;
                luminance_sq_sum += luma * luma;
            }
        }
        
        let mean = luminance_sum as f64 / luma_sample as f64;
        let variance = (luminance_sq_sum as f64 / luma_sample as f64) - (mean * mean);
        let std_dev = variance.sqrt();
        
        // 标准差归一化 (0-128范围)
        let texture_score = (std_dev / 128.0).min(1.0) * 0.3;
        
        // 综合评分
        let final_complexity = color_score + edge_score + texture_score;
        
        debug!(
            "🎨 Image complexity: {:.3} (colors={:.3}, edges={:.3}, texture={:.3})",
            final_complexity, color_score, edge_score, texture_score
        );
        
        final_complexity.min(1.0)
    }
    
    /// 检测是否为动画
    fn detect_animation(image_path: &Path) -> Result<bool> {
        let extension = image_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // GIF可能是动画
        if extension == "gif" {
            // 通过文件大小启发式判断
            // 通常动画GIF比静态GIF大
            if let Ok(metadata) = std::fs::metadata(image_path) {
                let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
                // 大于100KB的GIF很可能是动画
                return Ok(size_mb > 0.1);
            }
        }
        
        // WebP可能是动画（需要进一步检测）
        if extension == "webp" {
            // 简单检测：读取文件头检查VP8L/VP8X标记
            if let Ok(mut file) = std::fs::File::open(image_path) {
                use std::io::Read;
                let mut header = [0u8; 16];
                if file.read(&mut header).is_ok() {
                    // 检查RIFF签名和WEBP标记
                    if &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP" {
                        // VP8X表示扩展格式（可能包含动画）
                        if &header[12..16] == b"VP8X" {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        
        // APNG检测
        if extension == "png" || extension == "apng" {
            if let Ok(mut file) = std::fs::File::open(image_path) {
                use std::io::{Read, Seek, SeekFrom};
                let mut sig = [0u8; 8];
                if file.read_exact(&mut sig).is_ok() && &sig == b"\x89PNG\r\n\x1a\n" {
                    // 查找acTL块（动画控制块）
                    let mut chunk_type = [0u8; 4];
                    loop {
                        let mut len_buf = [0u8; 4];
                        if file.read_exact(&mut len_buf).is_err() {
                            break;
                        }
                        if file.read_exact(&mut chunk_type).is_err() {
                            break;
                        }
                        
                        if &chunk_type == b"acTL" {
                            return Ok(true); // 发现动画块
                        }
                        
                        let chunk_len = u32::from_be_bytes(len_buf);
                        if file.seek(SeekFrom::Current((chunk_len + 4) as i64)).is_err() {
                            break;
                        }
                    }
                }
            }
        }
        
        Ok(false)
    }
}
