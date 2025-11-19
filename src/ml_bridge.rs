/**
 * ML桥接层 - Python训练 ↔ Rust推理
 * 
 * 统一特征定义和数据流
 */
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



/// 标准化特征向量 (128维)
/// 与Python训练保持完全一致
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandardFeatures {
    /// 基础特征 (16维) - 图像基本属性
    pub basic: [f64; 16],
    
    /// 颜色特征 (16维) - 颜色分布和复杂度
    pub color: [f64; 16],
    
    /// 纹理特征 (16维) - 边缘和纹理信息
    pub texture: [f64; 16],
    
    /// 形状特征 (16维) - 几何和结构
    pub shape: [f64; 16],
    
    /// 质量特征 (16维) - 噪声和清晰度
    pub quality: [f64; 16],
    
    /// 元数据特征 (32维) - EXIF和文件属性
    pub metadata: [f64; 32],
    
    /// 上下文特征 (16维) - 处理历史和环境
    pub context: [f64; 16],
}

impl StandardFeatures {
    /// 转换为128维向量 (与Python一致)
    pub fn to_vector(&self) -> Vec<f64> {
        let mut vec = Vec::with_capacity(128);
        vec.extend_from_slice(&self.basic);
        vec.extend_from_slice(&self.color);
        vec.extend_from_slice(&self.texture);
        vec.extend_from_slice(&self.shape);
        vec.extend_from_slice(&self.quality);
        vec.extend_from_slice(&self.metadata);
        vec.extend_from_slice(&self.context);
        vec
    }
    
    /// 从128维向量创建 (Python输出)
    pub fn from_vector(vec: &[f64]) -> Result<Self, String> {
        if vec.len() != 128 {
            return Err(format!("Invalid feature vector length: {} (expected 128)", vec.len()));
        }
        
        // 安全：长度已验证为128
        Ok(Self {
            basic: vec[0..16].try_into().expect("slice length verified"),
            color: vec[16..32].try_into().expect("slice length verified"),
            texture: vec[32..48].try_into().expect("slice length verified"),
            shape: vec[48..64].try_into().expect("slice length verified"),
            quality: vec[64..80].try_into().expect("slice length verified"),
            metadata: vec[80..112].try_into().expect("slice length verified"),
            context: vec[112..128].try_into().expect("slice length verified"),
        })
    }
    
    /// 转换为JSON (Python通信)
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
    
    /// 从JSON创建 (Python通信)
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

/// 标准化预测结果
/// 与Python模型输出保持一致
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardPrediction {
    /// 推荐质量 (0-100)
    pub quality: u32,
    
    /// 推荐速度/effort (0-9)
    pub effort: u32,
    
    /// 是否推荐无损
    pub lossless: bool,
    
    /// 推荐格式
    pub format: String,
    
    /// 预测置信度 (0-1)
    pub confidence: f64,
    
    /// 预估文件大小 (bytes)
    pub estimated_size: u64,
    
    /// 预估质量 (SSIM)
    pub estimated_quality: f64,
    
    /// 模型版本
    pub model_version: String,
}

/// 训练样本 (用于反馈)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    /// 特征向量
    pub features: StandardFeatures,
    
    /// 实际使用的参数
    pub actual_quality: u32,
    pub actual_effort: u32,
    pub actual_lossless: bool,
    pub actual_format: String,
    
    /// 实际结果
    pub result_size: u64,
    pub result_quality: f64, // SSIM
    pub processing_time: f64,
    
    /// 用户反馈 (可选)
    pub user_rating: Option<f64>,
    
    /// 时间戳
    pub timestamp: i64,
}

impl TrainingSample {
    /// 转换为Python训练格式
    /// 
    /// 🔥 使用安全的序列化，不会panic
    pub fn to_training_format(&self) -> HashMap<String, serde_json::Value> {
        let mut data = HashMap::new();
        
        // 🔥 安全的序列化宏 - 失败时记录警告并跳过
        macro_rules! safe_insert {
            ($key:expr, $value:expr) => {
                match serde_json::to_value($value) {
                    Ok(v) => { data.insert($key.to_string(), v); },
                    Err(e) => {
                        log::warn!("⚠️  Failed to serialize {}: {}", $key, e);
                    }
                }
            };
        }
        
        // 特征
        safe_insert!("features", self.features.to_vector());
        
        // 标签
        safe_insert!("quality", self.actual_quality);
        safe_insert!("effort", self.actual_effort);
        safe_insert!("lossless", self.actual_lossless);
        safe_insert!("format", &self.actual_format);
        
        // 结果
        safe_insert!("result_size", self.result_size);
        safe_insert!("result_quality", self.result_quality);
        safe_insert!("processing_time", self.processing_time);
        
        if let Some(rating) = self.user_rating {
            safe_insert!("user_rating", rating);
        }
        
        safe_insert!("timestamp", self.timestamp);
        
        data
    }
}

// 🔥 MLBridge结构体已删除 - 完全未被使用
// 保留的类型：StandardFeatures, StandardPrediction, TrainingSample (被大量使用)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_features_vector_conversion() {
        let features = StandardFeatures {
            basic: [1.0; 16],
            color: [2.0; 16],
            texture: [3.0; 16],
            shape: [4.0; 16],
            quality: [5.0; 16],
            metadata: [6.0; 32],
            context: [7.0; 16],
        };
        
        let vec = features.to_vector();
        assert_eq!(vec.len(), 128);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[16], 2.0);
        assert_eq!(vec[112], 7.0);
    }

    #[test]
    fn test_features_json_serialization() {
        let features = StandardFeatures {
            basic: [1.0; 16],
            color: [2.0; 16],
            texture: [3.0; 16],
            shape: [4.0; 16],
            quality: [5.0; 16],
            metadata: [6.0; 32],
            context: [7.0; 16],
        };
        
        let json = features.to_json().unwrap();
        let restored = StandardFeatures::from_json(&json).unwrap();
        
        assert_eq!(restored.basic[0], 1.0);
        assert_eq!(restored.color[0], 2.0);
    }

    #[test]
    fn test_training_sample_format() {
        let features = StandardFeatures {
            basic: [1.0; 16],
            color: [2.0; 16],
            texture: [3.0; 16],
            shape: [4.0; 16],
            quality: [5.0; 16],
            metadata: [6.0; 32],
            context: [7.0; 16],
        };
        
        let sample = TrainingSample {
            features,
            actual_quality: 75,
            actual_effort: 6,
            actual_lossless: false,
            actual_format: "avif".to_string(),
            result_size: 50000,
            result_quality: 0.96,
            processing_time: 1.5,
            user_rating: Some(4.5),
            timestamp: 1700000000,
        };
        
        let format = sample.to_training_format();
        assert!(format.contains_key("features"));
        assert!(format.contains_key("quality"));
        assert!(format.contains_key("result_size"));
    }


}
