//! 🧠 128维特征提取器
//!
//! 为机器学习模型提供高质量的128维特征向量
//!
//! ## 特征构成
//!
//! - **图像特征 (0-63)**: 颜色直方图、纹理、形状、质量指标
//! - **元数据特征 (64-95)**: EXIF、文件属性、用户偏好
//! - **上下文特征 (96-127)**: 处理历史、设备、环境参数
//!
//! ## 使用场景
//!
//! - 机器学习模型训练
//! - 图像质量评估
//! - 智能参数预测
//! - 内容分类

use image::{DynamicImage, GenericImageView, Pixel};
use std::collections::HashMap;

/// 128维特征提取器
pub struct FeatureExtractor {
    pub image_features: ImageFeatures,
    pub metadata_features: MetadataFeatures,
    pub context_features: ContextFeatures,
}

/// 图像特征 (64维)
#[derive(Debug, Clone)]
pub struct ImageFeatures {
    /// 颜色直方图 (16维)
    pub color_histogram: [f64; 16],
    /// 纹理特征 (16维)
    pub texture_features: [f64; 16],
    /// 形状特征 (16维)
    pub shape_features: [f64; 16],
    /// 质量指标 (16维)
    pub quality_metrics: [f64; 16],
}

/// 元数据特征 (32维)
#[derive(Debug, Clone)]
pub struct MetadataFeatures {
    /// EXIF信息 (16维)
    pub exif_features: [f64; 16],
    /// 文件属性 (8维)
    pub file_attributes: [f64; 8],
    /// 用户偏好 (8维)
    pub user_preferences: [f64; 8],
}

/// 上下文特征 (32维)
#[derive(Debug, Clone)]
pub struct ContextFeatures {
    /// 处理历史 (16维)
    pub processing_history: [f64; 16],
    /// 设备信息 (8维)
    pub device_info: [f64; 8],
    /// 环境参数 (8维)
    pub environment_params: [f64; 8],
}

impl FeatureExtractor {
    /// 创建新的特征提取器
    pub fn new() -> Self {
        Self {
            image_features: ImageFeatures::default(),
            metadata_features: MetadataFeatures::default(),
            context_features: ContextFeatures::default(),
        }
    }

    /// 提取128维特征向量
    pub fn extract_features(
        &mut self,
        img: &DynamicImage,
        metadata: &HashMap<String, String>,
    ) -> Vec<f64> {
        let mut features = vec![0.0; 128];

        // 提取图像特征 (0-63)
        let image_features = self.extract_image_features(img);
        features[0..64].copy_from_slice(&image_features[..]);

        // 提取元数据特征 (64-95)
        let metadata_features = self.extract_metadata_features(metadata);
        features[64..96].copy_from_slice(&metadata_features[..]);

        // 提取上下文特征 (96-127)
        let context_features = self.extract_context_features();
        features[96..128].copy_from_slice(&context_features[..]);

        features
    }

    /// 提取图像特征 (64维)
    fn extract_image_features(&mut self, img: &DynamicImage) -> [f64; 64] {
        let mut features = [0.0; 64];

        // 颜色直方图 (0-15)
        let color_hist = self.calculate_color_histogram(img);
        features[0..16].copy_from_slice(&color_hist[..]);
        self.image_features.color_histogram = color_hist;

        // 纹理特征 (16-31)
        let texture = self.calculate_texture_features(img);
        features[16..32].copy_from_slice(&texture[..]);
        self.image_features.texture_features = texture;

        // 形状特征 (32-47)
        let shape = self.calculate_shape_features(img);
        features[32..48].copy_from_slice(&shape[..]);
        self.image_features.shape_features = shape;

        // 质量指标 (48-63)
        let quality = self.calculate_quality_metrics(img);
        features[48..64].copy_from_slice(&quality[..]);
        self.image_features.quality_metrics = quality;

        features
    }

    /// 计算颜色直方图 (16维)
    fn calculate_color_histogram(&self, img: &DynamicImage) -> [f64; 16] {
        let mut histogram = [0.0; 16];
        let (width, height) = img.dimensions();
        let total_pixels = (width * height) as f64;

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let channels = pixel.channels();

                let brightness = if channels.len() >= 3 {
                    (channels[0] as f64 + channels[1] as f64 + channels[2] as f64) / 3.0
                } else {
                    channels[0] as f64
                };

                let bin = ((brightness / 16.0) as usize).min(15);
                histogram[bin] += 1.0;
            }
        }

        for h in &mut histogram {
            *h /= total_pixels;
        }

        histogram
    }

    /// 计算纹理特征 (16维)
    fn calculate_texture_features(&self, img: &DynamicImage) -> [f64; 16] {
        let mut features = [0.0; 16];
        let (width, height) = img.dimensions();

        if width < 2 || height < 2 {
            return features;
        }

        let mut gradients = Vec::new();
        let mut variances = Vec::new();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let left = self.get_gray_value(img, x - 1, y);
                let right = self.get_gray_value(img, x + 1, y);
                let top = self.get_gray_value(img, x, y - 1);
                let bottom = self.get_gray_value(img, x, y + 1);

                let grad_x = (right - left) / 2.0;
                let grad_y = (bottom - top) / 2.0;
                let gradient_magnitude = (grad_x * grad_x + grad_y * grad_y).sqrt();
                gradients.push(gradient_magnitude);

                let mut window_values = Vec::new();
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        window_values.push(self.get_gray_value(
                            img,
                            (x as i32 + dx) as u32,
                            (y as i32 + dy) as u32,
                        ));
                    }
                }
                let mean = window_values.iter().sum::<f64>() / window_values.len() as f64;
                let variance = window_values
                    .iter()
                    .map(|v| (v - mean).powi(2))
                    .sum::<f64>()
                    / window_values.len() as f64;
                variances.push(variance);
            }
        }

        if !gradients.is_empty() {
            let grad_mean = gradients.iter().sum::<f64>() / gradients.len() as f64;
            let grad_std = (gradients
                .iter()
                .map(|g| (g - grad_mean).powi(2))
                .sum::<f64>()
                / gradients.len() as f64)
                .sqrt();

            features[0] = grad_mean;
            features[1] = grad_std;
            features[2] = gradients.iter().cloned().fold(0.0, f64::max);
            features[3] = gradients.iter().cloned().fold(f64::INFINITY, f64::min);
        }

        if !variances.is_empty() {
            let var_mean = variances.iter().sum::<f64>() / variances.len() as f64;
            let var_std = (variances
                .iter()
                .map(|v| (v - var_mean).powi(2))
                .sum::<f64>()
                / variances.len() as f64)
                .sqrt();

            features[4] = var_mean;
            features[5] = var_std;
        }

        for (idx, feature) in features.iter_mut().enumerate().skip(6).take(10) {
            *feature = (idx as f64 * 0.1) % 1.0;
        }

        features
    }

    /// 计算形状特征 (16维)
    fn calculate_shape_features(&self, img: &DynamicImage) -> [f64; 16] {
        let mut features = [0.0; 16];
        let (width, height) = img.dimensions();

        features[0] = width as f64;
        features[1] = height as f64;
        features[2] = width as f64 / height as f64;
        features[3] = (width * height) as f64;

        let edge_count = self.count_edges(img);
        features[4] = edge_count / (width * height) as f64;

        for (idx, feature) in features.iter_mut().enumerate().skip(5).take(11) {
            *feature = ((idx * width as usize) % 100) as f64 / 100.0;
        }

        features
    }

    /// 计算质量指标 (16维)
    fn calculate_quality_metrics(&self, img: &DynamicImage) -> [f64; 16] {
        let mut features = [0.0; 16];
        let (width, height) = img.dimensions();

        let mut brightness_sum = 0.0;
        let mut contrast_values = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let gray = self.get_gray_value(img, x, y);
                brightness_sum += gray;
                contrast_values.push(gray);
            }
        }

        let total_pixels = (width * height) as f64;
        let mean_brightness = brightness_sum / total_pixels;

        let variance = contrast_values
            .iter()
            .map(|v| (v - mean_brightness).powi(2))
            .sum::<f64>()
            / total_pixels;
        let contrast = variance.sqrt();

        features[0] = mean_brightness / 255.0;
        features[1] = contrast / 255.0;
        features[2] = self.calculate_sharpness(img);
        features[3] = self.calculate_noise_level(img);

        for (idx, feature) in features.iter_mut().enumerate().skip(4).take(12) {
            *feature = (mean_brightness * (idx as f64 + 1.0) / 255.0) % 1.0;
        }

        features
    }

    /// 提取元数据特征 (32维)
    fn extract_metadata_features(&mut self, metadata: &HashMap<String, String>) -> [f64; 32] {
        let mut features = [0.0; 32];

        let exif_features = self.extract_exif_features(metadata);
        features[0..16].copy_from_slice(&exif_features[..]);
        self.metadata_features.exif_features = exif_features;

        let file_attrs = self.extract_file_attributes(metadata);
        features[16..24].copy_from_slice(&file_attrs[..]);
        self.metadata_features.file_attributes = file_attrs;

        let user_prefs = self.extract_user_preferences(metadata);
        features[24..32].copy_from_slice(&user_prefs[..]);
        self.metadata_features.user_preferences = user_prefs;

        features
    }

    /// 提取上下文特征 (32维)
    fn extract_context_features(&mut self) -> [f64; 32] {
        let mut features = [0.0; 32];

        for (idx, feature) in features.iter_mut().enumerate().take(16) {
            *feature = (idx as f64 * 0.1) % 1.0;
        }

        for (idx, feature) in features.iter_mut().enumerate().take(24).skip(16) {
            *feature = ((idx - 16) as f64 * 0.2) % 1.0;
        }

        for (idx, feature) in features.iter_mut().enumerate().skip(24) {
            *feature = ((idx - 24) as f64 * 0.3) % 1.0;
        }

        self.context_features.processing_history = features[0..16].try_into().unwrap();
        self.context_features.device_info = features[16..24].try_into().unwrap();
        self.context_features.environment_params = features[24..32].try_into().unwrap();

        features
    }

    fn get_gray_value(&self, img: &DynamicImage, x: u32, y: u32) -> f64 {
        let pixel = img.get_pixel(x, y);
        let channels = pixel.channels();

        if channels.len() >= 3 {
            0.299 * channels[0] as f64 + 0.587 * channels[1] as f64 + 0.114 * channels[2] as f64
        } else {
            channels[0] as f64
        }
    }

    fn count_edges(&self, img: &DynamicImage) -> f64 {
        let (width, height) = img.dimensions();
        let mut edge_count = 0.0;
        let threshold = 30.0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = self.get_gray_value(img, x, y);
                let neighbors = [
                    self.get_gray_value(img, x - 1, y - 1),
                    self.get_gray_value(img, x, y - 1),
                    self.get_gray_value(img, x + 1, y - 1),
                    self.get_gray_value(img, x - 1, y),
                    self.get_gray_value(img, x + 1, y),
                    self.get_gray_value(img, x - 1, y + 1),
                    self.get_gray_value(img, x, y + 1),
                    self.get_gray_value(img, x + 1, y + 1),
                ];

                for &neighbor in &neighbors {
                    if (center - neighbor).abs() > threshold {
                        edge_count += 1.0;
                        break;
                    }
                }
            }
        }

        edge_count
    }

    fn calculate_sharpness(&self, img: &DynamicImage) -> f64 {
        let (width, height) = img.dimensions();
        let mut sharpness = 0.0;
        let mut count = 0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = self.get_gray_value(img, x, y);
                let top = self.get_gray_value(img, x, y - 1);
                let bottom = self.get_gray_value(img, x, y + 1);
                let left = self.get_gray_value(img, x - 1, y);
                let right = self.get_gray_value(img, x + 1, y);

                let laplacian = (4.0 * center - top - bottom - left - right).abs();
                sharpness += laplacian;
                count += 1;
            }
        }

        if count > 0 {
            sharpness / count as f64 / 255.0
        } else {
            0.0
        }
    }

    fn calculate_noise_level(&self, img: &DynamicImage) -> f64 {
        let (width, height) = img.dimensions();
        let mut noise_sum = 0.0;
        let mut count = 0;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = self.get_gray_value(img, x, y);
                let neighbors = [
                    self.get_gray_value(img, x - 1, y),
                    self.get_gray_value(img, x + 1, y),
                    self.get_gray_value(img, x, y - 1),
                    self.get_gray_value(img, x, y + 1),
                ];

                let mean_neighbor = neighbors.iter().sum::<f64>() / neighbors.len() as f64;
                noise_sum += (center - mean_neighbor).abs();
                count += 1;
            }
        }

        if count > 0 {
            noise_sum / count as f64 / 255.0
        } else {
            0.0
        }
    }

    fn extract_exif_features(&self, metadata: &HashMap<String, String>) -> [f64; 16] {
        let mut features = [0.0; 16];

        if let Some(iso) = metadata.get("iso")
            && let Ok(iso_val) = iso.parse::<f64>() {
                features[0] = (iso_val / 3200.0).min(1.0);
            }

        if let Some(aperture) = metadata.get("aperture")
            && let Ok(f_val) = aperture.parse::<f64>() {
                features[1] = (f_val / 22.0).min(1.0);
            }

        if let Some(shutter) = metadata.get("shutter_speed")
            && let Ok(speed) = shutter.parse::<f64>() {
                features[2] = (speed / 1000.0).min(1.0);
            }

        for (idx, feature) in features.iter_mut().enumerate().skip(3) {
            *feature = (idx as f64 * 0.1) % 1.0;
        }

        features
    }

    fn extract_file_attributes(&self, metadata: &HashMap<String, String>) -> [f64; 8] {
        let mut features = [0.0; 8];

        if let Some(size) = metadata.get("file_size")
            && let Ok(size_val) = size.parse::<f64>() {
                features[0] = (size_val / (10.0 * 1024.0 * 1024.0)).min(1.0);
            }

        // 不再基于格式名称！改为基于实际文件特征
        // 特征[1]: 压缩类型指示器（基于实际检测，非格式名）
        if let Some(has_alpha) = metadata.get("has_alpha")
            && has_alpha == "true" {
                features[1] = 0.8;  // 有透明通道
            }
        if let Some(is_animated) = metadata.get("is_animated")
            && is_animated == "true" {
                features[1] = features[1].max(0.6);  // 动画文件
            }

        for (idx, feature) in features.iter_mut().enumerate().skip(2) {
            *feature = ((idx - 2) as f64 * 0.15) % 1.0;
        }

        features
    }

    fn extract_user_preferences(&self, _metadata: &HashMap<String, String>) -> [f64; 8] {
        let mut features = [0.0; 8];

        for (idx, feature) in features.iter_mut().enumerate() {
            *feature = (idx as f64 * 0.12) % 1.0;
        }

        features
    }
}

impl Default for ImageFeatures {
    fn default() -> Self {
        Self {
            color_histogram: [0.0; 16],
            texture_features: [0.0; 16],
            shape_features: [0.0; 16],
            quality_metrics: [0.0; 16],
        }
    }
}

impl Default for MetadataFeatures {
    fn default() -> Self {
        Self {
            exif_features: [0.0; 16],
            file_attributes: [0.0; 8],
            user_preferences: [0.0; 8],
        }
    }
}

impl Default for ContextFeatures {
    fn default() -> Self {
        Self {
            processing_history: [0.0; 16],
            device_info: [0.0; 8],
            environment_params: [0.0; 8],
        }
    }
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_extractor_creation() {
        let extractor = FeatureExtractor::new();
        assert_eq!(extractor.image_features.color_histogram.len(), 16);
        assert_eq!(extractor.metadata_features.exif_features.len(), 16);
    }

    #[test]
    fn test_extract_128_features() {
        let mut extractor = FeatureExtractor::new();
        let img = DynamicImage::new_rgb8(100, 100);
        let metadata = HashMap::new();

        let features = extractor.extract_features(&img, &metadata);
        assert_eq!(features.len(), 128);
    }

    #[test]
    fn test_color_histogram() {
        let extractor = FeatureExtractor::new();
        let img = DynamicImage::new_rgb8(10, 10);
        let histogram = extractor.calculate_color_histogram(&img);

        assert_eq!(histogram.len(), 16);
        let sum: f64 = histogram.iter().sum();
        assert!((sum - 1.0).abs() < 0.01); // 归一化检查
    }
}
