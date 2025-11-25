// Alpha quality predictor - Independent alpha quality control
//
// Inspired by Squoosh's independent alpha quality control.
// Principle: Alpha channel is usually simpler than RGB channels and can be encoded with lower quality.
// Independent control of alpha quality can save 5-15% file size.

#[derive(Default)]
pub structure AlphaQualityPredictor;

#[derive(Debug, Clone)]
pub structure FileFeatures {
 pub has_alpha: bool,
 pub format: String,
 pub file_size: u64,
 pub width: u32,
 pub height: u32,
 pub complexity: f64,
 pub noise_level: f64,
}

impl AlphaQualityPredictor {
 pub fn new() -> Self {
 Self
 }

 /// prediction独立Alphaquality
 /// return: None (跟随RGBquality) or Some(qualityvalue) (0-100)
 ///
 /// Alpha复杂度classification:
 /// - 简single蒙版 (0-0.3): 纯transparency/ not transparency,no渐变 → availablelowquality (RGB-15)
 /// - etc 复杂度 (0.3-0.7): 部分渐变 → 略lowquality (RGB-5)
 /// - high复杂度 (0.7-1.0): 复杂渐变/半transparency → 跟随RGBquality
 pub fn predict_alpha_quality(&self, features: &FileFeatures, rgb_quality: u8) -> Option<u8> {
 // 1. if没 has Alphachannel,return None
 if !features.has_alpha {
 return None;
 }

 // 2. 估算Alpha复杂度
 let alpha_complexity = self.estimate_alpha_complexity(features);

 // 3. based on复杂度决定Alphaquality
 if alpha_complexity < 0.3 {
 // 简singleAlpha (如纯蒙版, UI元素)
 // can用显著更lowquality
 let alpha_quality = (rgb_quality as i16 - 15).max(60) as u8;
 Some(alpha_quality)
 } else if alpha_complexity < 0.7 {
 // etc Alpha (部分渐变, 阴影效果)
 // can用略lowquality
 let alpha_quality = (rgb_quality as i16 - 5).max(75) as u8;
 Some(alpha_quality)
 } else {
 // 复杂Alpha (复杂渐变, 半transparency效果)
 // need跟随RGBquality
 None
 }
 }

 /// 估算Alpha复杂度
 /// return: 0.0-1.0 (0=简single蒙版, 1=复杂渐变)
 fn estimate_alpha_complexity(&self, features: &FileFeatures) -> f64 {
 let mut complexity = 0.3; // 基础复杂度 (保守估计)

 // 1. based onfileformat推断
 match features.format.as_str() {
 "png" => {
 // PNGAlpha通常forUI元素/图标
 // iffile很小 (<1MB) 且resolution etc ,mayis UI/图标
 if features.file_size < 1024 * 1024
 && (features.width * features.height) < 2_000_000
 {
 complexity -= 0.2;
 }
 }
 "gif" => {
 // GIFtransparency度is1bit (纯transparencyor纯 not transparency)
 complexity = 0.1;
 }
 _ => {}
 }

 // 2. based onimage复杂度 (权重提highto0.5)
 complexity += features.complexity * 0.5;

 // 3. based onnoise水平 (权重提highto0.3)
 complexity += features.noise_level * 0.3;

 // 4. normalizeto [0, 1]
 complexity.clamp(0.0, 1.0)
 }

 /// getAlphaqualitysuggested
 pub fn get_alpha_quality_recommendation(
 &self,
 features: &FileFeatures,
 rgb_quality: u8,
 ) -> String {
 if !features.has_alpha {
 return "No alpha channel".to_string();
 }

 match self.predict_alpha_quality(features, rgb_quality) {
 None => "Alpha quality follows RGB quality (complex alpha)".to_string(),
 Some(alpha_quality) => {
 let saving = (rgb_quality as f64 - alpha_quality as f64) / rgb_quality as f64 * 100.0;
 format!(
 "Independent alpha quality={} (RGB={}), expected saving {:.1}%",
 alpha_quality, rgb_quality, saving
 )
 }
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_simple_alpha() {
 let predictor = AlphaQualityPredictor::new();
 let features = FileFeatures {
 has_alpha: true,
 format: "png".to_string(),
 file_size: 500 * 1024, // 500KB
 width: 1000,
 height: 1000,
 complexity: 0.2,
 noise_level: 0.1,
 };

 let alpha_quality = predictor.predict_alpha_quality(&features, 85);
 assert!(alpha_quality.is_some());
 assert!(alpha_quality.unwrap() < 85);
 }

 #[test]
 fn test_no_alpha() {
 let predictor = AlphaQualityPredictor::new();
 let features = FileFeatures {
 has_alpha: false,
 format: "jpg".to_string(),
 file_size: 1024 * 1024,
 width: 2000,
 height: 2000,
 complexity: 0.5,
 noise_level: 0.3,
 };

 let alpha_quality = predictor.predict_alpha_quality(&features, 85);
 assert!(alpha_quality.is_none());
 }

 #[test]
 fn test_complex_alpha() {
 let predictor = AlphaQualityPredictor::new();
 let features = FileFeatures {
 has_alpha: true,
 format: "webp".to_string(), // 改用webp避免PNG-0.2调整
 file_size: 5 * 1024 * 1024,
 width: 4000,
 height: 4000,
 complexity: 0.9, // improve复杂度
 noise_level: 0.6, // improve噪声
 };

 let complexity = predictor.estimate_alpha_complexity(&features);
 // 0.3 + 0.9*0.5 + 0.6*0.3 = 0.3 + 0.45 + 0.18 = 0.93
 assert!(complexity >= 0.7, "complexity={}", complexity);
 
 let alpha_quality = predictor.predict_alpha_quality(&features, 90);
 assert!(alpha_quality.is_none()); // 复杂Alpha应该跟随RGB
 }
}
