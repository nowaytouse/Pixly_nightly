// 视觉质量评分器 - 基于图像分析的质量推荐
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/predictor/ml/visual_quality_scorer.go

use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ImageFeatures {
    // 基础特征
    pub width: u32,
    pub height: u32,
    pub pixels: u32,
    pub aspect_ratio: f64,
    
    // 色彩特征
    pub color_complexity: f64,  // 色彩复杂度 (0-1)
    pub brightness: f64,         // 平均亮度 (0-255)
    pub contrast: f64,           // 对比度 (0-1)
    pub saturation: f64,         // 饱和度 (0-1)
    
    // 纹理特征
    pub edge_density: f64,       // 边缘密度 (0-1)
    pub texture_complexity: f64, // 纹理复杂度 (0-1)
    pub noise_level: f64,        // 噪声水平 (0-1)
    
    // 内容特征
    pub is_photo: bool,          // 是否为照片
    pub is_graphic: bool,        // 是否为图形/设计稿
    pub is_screenshot: bool,     // 是否为截图
    pub has_transparency: bool,  // 是否有透明度
    
    // 质量估计
    pub estimated_quality: u8,   // 估计的原始质量 (0-100)
    pub recommended_quality: u8, // 推荐的转换质量 (0-100)
}

#[derive(Debug, Clone)]
pub struct QualityRecommendation {
    pub quality: u8,      // 推荐质量 (0-100)
    pub effort: u8,       // 推荐努力 (1-9)
    pub lossless: bool,   // 是否推荐无损
    pub confidence: f64,  // 推荐置信度 (0-1)
    pub reasoning: String, // 推荐理由
}

#[derive(Default)]
pub struct VisualQualityScorer;

impl VisualQualityScorer {
    pub fn new() -> Self {
        Self
    }

    /// 分析图像并提取特征
    pub fn analyze_image<P: AsRef<Path>>(&self, file_path: P) -> Result<ImageFeatures, Box<dyn std::error::Error>> {
        let img = image::open(file_path)?;
        
        let width = img.width();
        let height = img.height();
        let pixels = width * height;
        let aspect_ratio = width as f64 / height as f64;
        
        let mut features = ImageFeatures {
            width,
            height,
            pixels,
            aspect_ratio,
            color_complexity: 0.0,
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
            edge_density: 0.0,
            texture_complexity: 0.0,
            noise_level: 0.0,
            is_photo: false,
            is_graphic: false,
            is_screenshot: false,
            has_transparency: false,
            estimated_quality: 0,
            recommended_quality: 0,
        };
        
        // 分析像素数据
        self.analyze_pixels(&img, &mut features);
        
        // 内容分类
        self.classify_content(&mut features);
        
        // 估计质量和推荐
        self.estimate_quality(&mut features);
        
        Ok(features)
    }

    /// 分析像素数据
    fn analyze_pixels(&self, img: &DynamicImage, features: &mut ImageFeatures) {
        let (width, height) = (img.width(), img.height());
        
        // 采样像素（为了性能，不分析所有像素）
        let sample_step = if width * height > 1_000_000 {
            ((width * height) as f64 / 1_000_000.0).sqrt() as u32
        } else {
            1
        };
        
        let mut total_r = 0.0_f64;
        let mut total_g = 0.0_f64;
        let mut total_b = 0.0_f64;
        let mut min_brightness = 255.0_f64;
        let mut max_brightness = 0.0_f64;
        let mut edge_count = 0;
        let mut sample_count = 0;
        let mut has_alpha = false;
        
        // 遍历采样像素
        for y in (0..height).step_by(sample_step as usize) {
            for x in (0..width).step_by(sample_step as usize) {
                let pixel = img.get_pixel(x, y);
                let channels = pixel.channels();
                
                let r = channels[0] as f64;
                let g = channels[1] as f64;
                let b = channels[2] as f64;
                
                total_r += r;
                total_g += g;
                total_b += b;
                
                // 计算亮度
                let brightness = 0.299 * r + 0.587 * g + 0.114 * b;
                min_brightness = min_brightness.min(brightness);
                max_brightness = max_brightness.max(brightness);
                
                // 检查透明度
                if channels.len() > 3 && channels[3] < 255 {
                    has_alpha = true;
                }
                
                // 简单的边缘检测
                if x < width - sample_step && y < height - sample_step {
                    let pixel2 = img.get_pixel(x + sample_step, y);
                    let pixel3 = img.get_pixel(x, y + sample_step);
                    
                    let channels2 = pixel2.channels();
                    let channels3 = pixel3.channels();
                    
                    let diff_right = (r - channels2[0] as f64).abs() 
                        + (g - channels2[1] as f64).abs() 
                        + (b - channels2[2] as f64).abs();
                    let diff_down = (r - channels3[0] as f64).abs() 
                        + (g - channels3[1] as f64).abs() 
                        + (b - channels3[2] as f64).abs();
                    
                    let threshold = 60.0;
                    if diff_right > threshold || diff_down > threshold {
                        edge_count += 1;
                    }
                }
                
                sample_count += 1;
            }
        }
        
        // 计算特征
        if sample_count > 0 {
            let avg_r = total_r / sample_count as f64;
            let avg_g = total_g / sample_count as f64;
            let avg_b = total_b / sample_count as f64;
            
            // 平均亮度
            features.brightness = 0.299 * avg_r + 0.587 * avg_g + 0.114 * avg_b;
            
            // 对比度
            features.contrast = (max_brightness - min_brightness) / 255.0;
            
            // 色彩饱和度
            let max_channel = avg_r.max(avg_g).max(avg_b);
            let min_channel = avg_r.min(avg_g).min(avg_b);
            if max_channel > 0.0 {
                features.saturation = (max_channel - min_channel) / max_channel;
            }
            
            // 边缘密度
            features.edge_density = edge_count as f64 / sample_count as f64;
            
            // 纹理复杂度
            features.texture_complexity = (features.edge_density + features.contrast) / 2.0;
            
            // 色彩复杂度
            features.color_complexity = (features.saturation + features.contrast) / 2.0;
            
            // 噪声估计
            if features.edge_density > 0.3 {
                features.noise_level = (features.edge_density - 0.3) / 0.7;
            }
        }
        
        features.has_transparency = has_alpha;
    }

    /// 内容分类
    fn classify_content(&self, features: &mut ImageFeatures) {
        // 照片特征：高色彩复杂度、中等纹理复杂度、较低噪声
        if features.color_complexity > 0.4 
            && features.texture_complexity > 0.3 
            && features.texture_complexity < 0.8 
            && features.noise_level < 0.5 {
            features.is_photo = true;
        }
        
        // 图形/设计稿特征：低噪声、高对比度
        if features.noise_level < 0.2 && features.contrast > 0.5 {
            features.is_graphic = true;
        }
        
        // 截图特征：特定分辨率、低噪声
        let aspect_ratio = features.aspect_ratio;
        let is_common_screen_ratio = (aspect_ratio > 1.33 && aspect_ratio < 1.34) // 4:3
            || (aspect_ratio > 1.77 && aspect_ratio < 1.78) // 16:9
            || (aspect_ratio > 1.59 && aspect_ratio < 1.61); // 16:10
        
        if is_common_screen_ratio && features.noise_level < 0.1 {
            features.is_screenshot = true;
        }
    }

    /// 估计质量
    fn estimate_quality(&self, features: &mut ImageFeatures) {
        let mut quality_score = 100.0;
        
        // 噪声降低质量
        quality_score -= features.noise_level * 30.0;
        
        // 低对比度可能表示低质量
        if features.contrast < 0.3 {
            quality_score -= (0.3 - features.contrast) * 50.0;
        }
        
        // 边缘密度过高可能表示压缩伪影
        if features.edge_density > 0.7 {
            quality_score -= (features.edge_density - 0.7) * 30.0;
        }
        
        features.estimated_quality = quality_score.clamp(0.0, 100.0) as u8;
        
        // 推荐质量稍低于估计质量
        let recommended = if features.estimated_quality >= 95 {
            features.estimated_quality
        } else if features.estimated_quality >= 85 {
            features.estimated_quality - 5
        } else {
            features.estimated_quality.saturating_sub(10)
        };
        
        features.recommended_quality = recommended.clamp(60, 100);
    }

    /// 推荐质量参数
    pub fn recommend_quality(&self, features: &ImageFeatures) -> QualityRecommendation {
        // 规则1: 高质量源 → 无损
        if features.estimated_quality >= 98 {
            return QualityRecommendation {
                quality: features.recommended_quality,
                effort: 9,
                lossless: true,
                confidence: 0.95,
                reasoning: "检测到高质量源，推荐数学无损以保障质量".to_string(),
            };
        }
        
        // 规则2: 设计稿/图形 + 高质量 → 无损
        if features.is_graphic && features.estimated_quality >= 90 {
            return QualityRecommendation {
                quality: features.recommended_quality,
                effort: 9,
                lossless: true,
                confidence: 0.9,
                reasoning: "检测到设计稿/图形，推荐无损以保留细节".to_string(),
            };
        }
        
        // 规则3: 透明图像 + 高质量 → 无损
        if features.has_transparency && features.estimated_quality >= 85 {
            return QualityRecommendation {
                quality: features.recommended_quality,
                effort: 8,
                lossless: true,
                confidence: 0.85,
                reasoning: "检测到透明图像，推荐无损以保留透明度细节".to_string(),
            };
        }
        
        // 规则4: 低噪点 + 高复杂度 → 可能是渲染图
        if features.noise_level < 0.1 && features.texture_complexity > 0.7 && features.estimated_quality >= 90 {
            return QualityRecommendation {
                quality: features.recommended_quality,
                effort: 9,
                lossless: true,
                confidence: 0.88,
                reasoning: "检测到可能的渲染图/高质量设计，推荐无损".to_string(),
            };
        }
        
        // 规则5: 照片 → 自适应质量
        if features.is_photo {
            return QualityRecommendation {
                quality: features.recommended_quality,
                effort: 7,
                lossless: false,
                confidence: 0.8,
                reasoning: format!("Photo detected, recommended quality {} (based on image analysis)", features.recommended_quality),
            };
        }
        
        // 规则6: 截图 → 高质量有损
        if features.is_screenshot {
            return QualityRecommendation {
                quality: 90,
                effort: 7,
                lossless: false,
                confidence: 0.85,
                reasoning: "检测到截图，推荐质量90以保持清晰度".to_string(),
            };
        }
        
        // 默认：使用分析的推荐质量
        QualityRecommendation {
            quality: features.recommended_quality,
            effort: 7,
            lossless: false,
            confidence: 0.8,
            reasoning: format!("Based on image feature analysis, recommended quality {}", features.recommended_quality),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_features_defaults() {
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            pixels: 1920 * 1080,
            aspect_ratio: 1.777,
            color_complexity: 0.5,
            brightness: 128.0,
            contrast: 0.6,
            saturation: 0.4,
            edge_density: 0.3,
            texture_complexity: 0.45,
            noise_level: 0.1,
            is_photo: true,
            is_graphic: false,
            is_screenshot: false,
            has_transparency: false,
            estimated_quality: 85,
            recommended_quality: 80,
        };

        assert_eq!(features.width, 1920);
        assert_eq!(features.height, 1080);
        assert!(features.is_photo);
    }

    #[test]
    fn test_quality_recommendation() {
        let scorer = VisualQualityScorer::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            pixels: 1920 * 1080,
            aspect_ratio: 1.777,
            color_complexity: 0.5,
            brightness: 128.0,
            contrast: 0.6,
            saturation: 0.4,
            edge_density: 0.3,
            texture_complexity: 0.45,
            noise_level: 0.1,
            is_photo: true,
            is_graphic: false,
            is_screenshot: false,
            has_transparency: false,
            estimated_quality: 85,
            recommended_quality: 80,
        };

        let rec = scorer.recommend_quality(&features);
        assert!(!rec.lossless);
        assert!(rec.quality >= 60 && rec.quality <= 100);
        assert!(rec.effort >= 1 && rec.effort <= 9);
    }
}
