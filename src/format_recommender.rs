// 🤖 AI格式推荐器
// 自动推荐最佳目标格式

use crate::{ImageFeatures, QualityMode, UnifiedAIPredictor};
use serde::{Deserialize, Serialize};

/// 格式推荐结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatRecommendation {
    pub format: String,
    pub score: f64,
    pub space_saving: f64,
    pub quality_score: u8,
    pub confidence: f64,
    pub reason: String,
    pub estimated_size: u64,
    pub estimated_ratio: f64,
}

/// 用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub space_weight: f64,      // 空间节省权重 (0.0-1.0)
    pub quality_weight: f64,    // 质量权重 (0.0-1.0)
    pub confidence_weight: f64, // 置信度权重 (0.0-1.0)
    pub format_bonus: f64,      // 偏好格式奖励分数
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

/// AI格式推荐器
pub struct AIFormatRecommender {
    predictor: UnifiedAIPredictor,
}

impl AIFormatRecommender {
    /// 创建新的推荐器
    pub fn new() -> Self {
        Self {
            predictor: UnifiedAIPredictor::new(),
        }
    }
    
    /// 推荐最佳格式
    pub fn recommend_best_format(
        &self,
        features: &ImageFeatures,
        quality_mode: QualityMode,
        user_preferences: &UserPreferences,
    ) -> Vec<FormatRecommendation> {
        let formats = self.get_candidate_formats(features);
        let mut recommendations = Vec::new();
        
        for format in formats {
            let prediction = self.predictor.predict_with_confidence(
                features, &format, quality_mode
            );
            
            let space_saving = 100.0 * (1.0 - prediction.core.estimated_ratio);
            let quality_score = prediction.core.quality;
            
            // 计算综合评分
            let score = self.calculate_format_score(
                space_saving,
                quality_score as f64,
                prediction.confidence,
                user_preferences,
                &format
            );
            
            let reason = self.generate_reason(&format, space_saving, quality_score as u8, prediction.confidence);
            
            recommendations.push(FormatRecommendation {
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
        
        // 按评分排序
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        recommendations
    }
    
    /// 获取候选格式列表 - 基于源文件智能推荐
    /// 正确处理动画+透明度的组合
    fn get_candidate_formats(&self, features: &ImageFeatures) -> Vec<String> {
        // 动画文件 + 透明度 (最复杂的情况)
        if features.is_animated && features.has_alpha {
            return vec![
                "webp".to_string(),  // 最佳动画+透明压缩
                "avif".to_string(),  // 新一代动画+透明格式
                "png".to_string(),   // APNG - 无损动画+透明
            ];
        }
        
        // 动画文件 (无透明度)
        if features.is_animated {
            return vec![
                "webp".to_string(),  // 最佳动画压缩
                "avif".to_string(),  // 新一代动画格式
                "gif".to_string(),   // 兼容性fallback
            ];
        }
        
        // 静态透明图像
        if features.has_alpha {
            return vec![
                "avif".to_string(),  // 最佳透明压缩
                "webp".to_string(),  // 透明+压缩
                "jxl".to_string(),   // 高质量透明
                "png".to_string(),   // 无损透明
            ];
        }
        
        // 高分辨率照片 (>4K)
        if features.width > 3840 || features.height > 2160 {
            return vec![
                "jxl".to_string(),   // 最佳大图压缩
                "avif".to_string(),  // 高效压缩
                "webp".to_string(),  // 平衡选择
            ];
        }
        
        // 简单图形/截图 (低复杂度)
        if features.complexity < 0.3 {
            return vec![
                "webp".to_string(),  // 简单图形最优
                "png".to_string(),   // 无损选择
                "avif".to_string(),  // 现代格式
            ];
        }
        
        // 复杂照片 (高复杂度)
        if features.complexity > 0.7 {
            return vec![
                "jxl".to_string(),   // 复杂图像最优
                "avif".to_string(),  // 次优选择
                "webp".to_string(),  // 兼容选择
                "jpeg".to_string(),  // 传统选择
            ];
        }
        
        // 默认推荐 (通用场景)
        vec![
            "avif".to_string(),  // 现代首选
            "webp".to_string(),  // 广泛支持
            "jxl".to_string(),   // 高质量选择
            "jpeg".to_string(),  // 兼容fallback
        ]
    }
    
    /// 计算格式评分
    fn calculate_format_score(
        &self,
        space_saving: f64,
        quality: f64,
        confidence: f64,
        preferences: &UserPreferences,
        format: &str,
    ) -> f64 {
        let mut score = 0.0;
        
        // 空间节省权重
        score += space_saving * preferences.space_weight;
        
        // 质量权重
        score += quality * preferences.quality_weight;
        
        // 置信度权重
        score += confidence * 100.0 * preferences.confidence_weight;
        
        // 格式偏好奖励
        if preferences.preferred_formats.contains(&format.to_string()) {
            score += preferences.format_bonus;
        }
        
        score
    }
    
    /// 生成推荐理由
    fn generate_reason(&self, format: &str, space_saving: f64, quality: u8, confidence: f64) -> String {
        let mut reasons = Vec::new();
        
        if space_saving > 50.0 {
            reasons.push(format!("Significant space saving of {}%", space_saving as u32));
        } else if space_saving > 30.0 {
            reasons.push(format!("Space saving of {}%", space_saving as u32));
        }
        
        if quality >= 90 {
            reasons.push("高质量保证".to_string());
        } else if quality >= 80 {
            reasons.push("良好质量".to_string());
        }
        
        if confidence >= 0.85 {
            reasons.push("高置信度".to_string());
        }
        
        match format {
            "avif" => reasons.push("现代格式，最高压缩率".to_string()),
            "webp" => reasons.push("广泛支持，平衡选择".to_string()),
            "jxl" => reasons.push("次世代格式，优秀性能".to_string()),
            "jpeg" => reasons.push("兼容性最好".to_string()),
            "png" => reasons.push("无损压缩".to_string()),
            _ => {}
        }
        
        if reasons.is_empty() {
            format!("{} format", format.to_uppercase())
        } else {
            reasons.join(", ")
        }
    }
    
    /// 获取最佳推荐
    pub fn get_best_recommendation(
        &self,
        features: &ImageFeatures,
        quality_mode: QualityMode,
        user_preferences: &UserPreferences,
    ) -> Option<FormatRecommendation> {
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
