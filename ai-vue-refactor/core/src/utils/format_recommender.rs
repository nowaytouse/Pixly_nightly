// 🤖 AIformatrecommended
// autorecommendedmosttargetformat
// 🔥 Phase: Real ML Integration - Uses python_ml_caller for genuine AI predictions

use crate::{ImageFeatures, QualityMode, UnifiedAIPredictor};
use crate::ai::python_ml_caller::{call_python_ml, is_python_ml_available, MLPredictRequest};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// formatrecommendedresult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFormatRecommendation {
 pub format: String,
 pub score: f64,
 pub space_saving: f64,
 pub quality_score: u8,
 pub confidence: f64,
 pub reason: String,
 pub estimated_size: u64,
 pub estimated_ratio: f64,
}

/// setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
 pub space_weight: f64, // emptyweight (0.0-1.0)
 pub quality_weight: f64, // qualityweight (0.0-1.0)
 pub confidence_weight: f64, // confidenceweight (0.0-1.0)
 pub format_bonus: f64, // formatrewardscore
 pub preferred_formats: Vec<String>,
}

impl Default for UserPreferences {
 fn default() -> Self {
 Self {
 space_weight: 0.4,
 quality_weight: 0.4,
 confidence_weight: 0.2,
 format_bonus: 10.0,
 preferred_formats: vec!["webp".to_string(), "avif".to_string()],
 }
 }
}

/// AIformatrecommended
/// 🔥 Real ML Integration: Uses Python ML service for genuine predictions
pub struct AIFormatRecommender {
 predictor: UnifiedAIPredictor, // Fallback only (deprecated)
 use_real_ml: bool, // Enable real ML predictions
}

impl AIFormatRecommender {
/// createnewrecommended
/// 🔥 Automatically detects and uses real ML if available
 pub fn new() -> Self {
 let use_real_ml = is_python_ml_available();
 if use_real_ml {
 log::info!("Real ML available - Using Python ML service for predictions");
 } else {
 log::warn!("Python ML unavailable - Using fallback heuristics (suboptimal)");
 }

 Self {
 predictor: UnifiedAIPredictor::new(),
 use_real_ml,
 }
 }

/// Create recommender (force heuristic mode)
/// For testing or when ML is intentionally disabled
 pub fn new_heuristic_only() -> Self {
 log::info!("Heuristic mode - Using rule-based predictions");
 Self {
 predictor: UnifiedAIPredictor::new(),
 use_real_ml: false,
 }
 }

/// recommendedmostformat
/// 🔥 Real ML Integration: Uses Python ML if available, falls back to heuristics
 pub fn recommend_best_format(
 &self,
 features: &ImageFeatures,
 quality_mode: QualityMode,
 user_preferences: &UserPreferences,
 ) -> Vec<AIFormatRecommendation> {
 let formats = self.get_candidate_formats(features);
 let mut recommendations = Vec::with_capacity(formats.len());

 for format in formats {
// 🔥 Try real ML first
 let prediction = if self.use_real_ml {
 match self.predict_with_real_ml(features, &format, quality_mode) {
 Ok(ml_pred) => ml_pred,
 Err(e) => {
 log::warn!("ML prediction failed for {}: {}, using fallback", format, e);
 self.predictor.predict_with_confidence(features, &format, quality_mode)
 }
 }
 } else {
// Fallback to heuristics
 self.predictor.predict_with_confidence(features, &format, quality_mode)
 };

 let space_saving = 100.0 * (1.0 - prediction.core.estimated_ratio);
 let quality_score = prediction.core.quality;

 let score = self.calculate_format_score(
 space_saving,
 quality_score as f64,
 prediction.confidence,
 user_preferences,
 &format
 );

 let reason = self.generate_reason(&format, space_saving, quality_score as u8, prediction.confidence);

 recommendations.push(AIFormatRecommendation {
 format: format.clone(),
 score,
 space_saving,
 quality_score: quality_score as u8,
 confidence: prediction.confidence,
 reason,
 estimated_size: prediction.core.estimated_size,
 estimated_ratio: prediction.core.estimated_ratio,
 });
 }

 recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
 recommendations
 }

/// 🔥 Real ML Prediction - Calls Python ML service
 fn predict_with_real_ml(
 &self,
 features: &ImageFeatures,
 target_format: &str,
 quality_mode: QualityMode,
 ) -> Result<crate::types::PredictionWithConfidence> {
// 1. Extract 128D features (if available from image data)
// For now, use ImageFeatures as-is (will be enhanced later)

// 2. Build ML request
 let ml_request = MLPredictRequest {
 features: vec![
 features.width as f64,
 features.height as f64,
 features.file_size as f64,
 if features.has_alpha { 1.0 } else { 0.0 },
 if features.is_animated { 1.0 } else { 0.0 },
 features.complexity,
 ],
 target_format: target_format.to_string(),
 quality_mode: match quality_mode {
 QualityMode::Speed => "size".to_string(),
 QualityMode::Balanced => "balanced".to_string(),
 QualityMode::Quality => "quality".to_string(),
 QualityMode::Lossless => "quality".to_string(),
 },
 };

// 3. Call Python ML
 let ml_response = call_python_ml(&ml_request)
 .context("Python ML call failed")?;

// 4. Convert to PredictionWithConfidence
 use crate::types::{PredictionResult, PredictionWithConfidence};

// Estimate size based on quality and format
 let estimated_ratio = match target_format {
 "avif" => 0.3,
 "jxl" => 0.4,
 "webp" => 0.5,
 _ => 0.6,
 } * (ml_response.quality as f64 / 100.0);

 let estimated_size = (features.file_size as f64 * estimated_ratio) as u64;

 let core = PredictionResult {
 quality: ml_response.quality as u32,
 speed: ml_response.effort as u32,
 lossless: ml_response.lossless,
 format_options: std::collections::HashMap::new(),
 estimated_size,
 estimated_ratio,
 algorithm_version: ml_response.model_version.clone(),
 predictor_version: "real-ml-v1".to_string(),
 };

 Ok(PredictionWithConfidence {
 core,
 confidence: ml_response.confidence,
 method: format!("python-ml-{}", ml_response.model_version),
 })
 }

/// getformatlist - based onsourcefileintelligentrecommended
/// positiveprocessinganimation+transparencydegreecomposite
 fn get_candidate_formats(&self, features: &ImageFeatures) -> Vec<String> {
// animationfile + transparencydegree (most)
 if features.is_animated && features.has_alpha {
 return vec![
 "webp".to_string(), // most动画+compress
 "avif".to_string(), // newa+format
 "png".to_string(), // APNG - +
 ];
 }

// animationfile (notransparencydegree)
 if features.is_animated {
 return vec![
 "webp".to_string(), // most动画compress
 "avif".to_string(), // newaformat
 "gif".to_string(), // compatiblefallback
 ];
 }

// statictransparencyimage
 if features.has_alpha {
 return vec![
 "avif".to_string(), // mostcompress
 "webp".to_string(), // +compress
 "jxl".to_string(), // highquality
 "png".to_string(), // 无损
 ];
 }

// highresolutionphoto (>4K)
 if features.width > 3840 || features.height > 2160 {
 return vec![
 "jxl".to_string(), // mostlargecompress
 "avif".to_string(), // highcompress
 "webp".to_string(), // 
 ];
 }

// singlegraphic/screenshot (lowcomplexity)
 if features.complexity < 0.3 {
 return vec![
 "webp".to_string(), // singlegraphicoptimal
 "png".to_string(), // 
 "avif".to_string(), // format
 ];
 }

// photo (highcomplexity)
 if features.complexity > 0.7 {
 return vec![
 "jxl".to_string(), // imageoptimal
 "avif".to_string(), // times
 "webp".to_string(), // compatible
 "jpeg".to_string(), // 传统
 ];
 }

// defaultrecommended ()
 vec![
 "avif".to_string(), // first
 "webp".to_string(), // support
 "jxl".to_string(), // highquality
 "jpeg".to_string(), // compatiblefallback
 ]
 }

/// calculationformat
 fn calculate_format_score(
 &self,
 space_saving: f64,
 quality: f64,
 confidence: f64,
 preferences: &UserPreferences,
 format: &str,
 ) -> f64 {
 let mut score = 0.0;

// emptyweight
 score += space_saving * preferences.space_weight;

// qualityweight
 score += quality * preferences.quality_weight;

// confidenceweight
 score += confidence * 100.0 * preferences.confidence_weight;

// formatreward
 if preferences.preferred_formats.contains(&format.to_string()) {
 score += preferences.format_bonus;
 }

 score
 }

/// generaterecommendedreason
 fn generate_reason(&self, format: &str, space_saving: f64, quality: u8, confidence: f64) -> String {
 let mut reasons = Vec::new();

 if space_saving > 50.0 {
 reasons.push(format!("Significant space saving of {}%", space_saving as u32));
 } else if space_saving > 30.0 {
 reasons.push(format!("Space saving of {}%", space_saving as u32));
 }

 if quality >= 90 {
 reasons.push("High quality guarantee".to_string());
 } else if quality >= 80 {
 reasons.push("Good quality".to_string());
 }

 if confidence >= 0.85 {
 reasons.push("High confidence".to_string());
 }

 match format {
 "avif" => reasons.push("Modern format, highest compression".to_string()),
 "webp" => reasons.push("Wide support, balanced choice".to_string()),
 "jxl" => reasons.push("Next-gen format, excellent performance".to_string()),
 "jpeg" => reasons.push("Best compatibility".to_string()),
 "png" => reasons.push("Lossless compression".to_string()),
 _ => {}
 }

 if reasons.is_empty() {
 format!("{} format", format.to_uppercase())
 } else {
 reasons.join(", ")
 }
 }

/// getmostrecommended
 pub fn get_best_recommendation(
 &self,
 features: &ImageFeatures,
 quality_mode: QualityMode,
 user_preferences: &UserPreferences,
 ) -> Option<AIFormatRecommendation> {
 let recommendations = self.recommend_best_format(features, quality_mode, user_preferences);
 recommendations.into_iter().next()
 }
}

impl Default for AIFormatRecommender {
 fn default() -> Self {
 Self::new()
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_format_recommendation() {
 let recommender = AIFormatRecommender::new();
 let features = ImageFeatures {
 width: 1920,
 height: 1080,
 file_size: 2_000_000,
 format: "jpeg".to_string(),
 has_alpha: false,
 is_animated: false,
 complexity: 0.6,
 };

 let preferences = UserPreferences::default();
 let recommendations = recommender.recommend_best_format(
 &features,
 QualityMode::Balanced,
 &preferences
 );

 assert!(!recommendations.is_empty());
 assert!(recommendations[0].score > 0.0);
 }

 #[test]
 fn test_best_recommendation() {
 let recommender = AIFormatRecommender::new();
 let features = ImageFeatures {
 width: 1920,
 height: 1080,
 file_size: 2_000_000,
 format: "jpeg".to_string(),
 has_alpha: false,
 is_animated: false,
 complexity: 0.6,
 };

 let preferences = UserPreferences::default();
 let best = recommender.get_best_recommendation(
 &features,
 QualityMode::Balanced,
 &preferences
 );

 assert!(best.is_some());
 let recommendation = best.unwrap();
 assert!(recommendation.space_saving > 0.0);
 }

 #[test]
 fn test_alpha_format_candidates() {
 let recommender = AIFormatRecommender::new();
 let features = ImageFeatures {
 width: 1920,
 height: 1080,
 file_size: 2_000_000,
 format: "png".to_string(),
 has_alpha: true,
 is_animated: false,
 complexity: 0.6,
 };

 let formats = recommender.get_candidate_formats(&features);
 assert!(formats.contains(&"png".to_string()));
 }
}
