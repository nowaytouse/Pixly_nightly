/**
 * ML数据流管理器
 * 
 * 统一Python训练 ↔ Rust推理的完整数据流
 */
use crate::ml_bridge::{StandardFeatures, StandardPrediction, TrainingSample};
use crate::feature_extractor_128d::FeatureExtractor128D;
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;

/// ML数据流管理器
pub struct MLDataFlow {
    /// 特征提取器
    extractor: FeatureExtractor128D,
    
    /// 训练样本缓冲区
    training_buffer: Vec<TrainingSample>,
    
    /// 最大缓冲区大小
    max_buffer_size: usize,
    
    /// 自动保存路径
    auto_save_path: Option<String>,
}

impl MLDataFlow {
    /// 创建新的数据流管理器
    pub fn new() -> Self {
        Self {
            extractor: FeatureExtractor128D::new(),
            training_buffer: Vec::new(),
            max_buffer_size: 1000,
            auto_save_path: None,
        }
    }
    
    /// 设置自动保存路径
    pub fn with_auto_save(mut self, path: String) -> Self {
        self.auto_save_path = Some(path);
        self
    }
    
    /// 设置缓冲区大小
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }
    
    /// 提取标准化特征 (Rust → Python训练)
    pub fn extract_standard_features(
        &mut self, 
        img: &DynamicImage, 
        metadata: &HashMap<String, String>
    ) -> StandardFeatures {
        let features_vec = self.extractor.extract_features(img, metadata);
        StandardFeatures::from_vector(&features_vec).expect("特征向量转换失败")
    }
    
    /// 记录训练样本 (收集反馈数据)
    pub fn record_training_sample(&mut self, sample: TrainingSample) -> Result<(), String> {
        // Validate features
        let vec = sample.features.to_vector();
        if vec.len() != 128 {
            return Err(format!("Invalid feature dimension: {}", vec.len()));
        }
        
        // 添加到缓冲区
        self.training_buffer.push(sample);
        
        // 检查是否需要自动保存
        if self.training_buffer.len() >= self.max_buffer_size
            && let Some(path) = self.auto_save_path.clone() {
                self.flush_training_buffer(&path)?;
            }
        
        Ok(())
    }
    
    /// 刷新训练缓冲区到文件
    pub fn flush_training_buffer(&mut self, path: &str) -> Result<(), String> {
        if self.training_buffer.is_empty() {
            return Ok(());
        }
        
        // 转换为JSON格式
        let samples: Vec<_> = self.training_buffer.iter()
            .map(|s| s.to_training_format())
            .collect();
        
        let json = serde_json::to_string_pretty(&samples)
            .map_err(|e| format!("JSON serialization failed: {}", e))?;
        
        // 保存到文件
        std::fs::write(path, json)
            .map_err(|e| format!("File write failed: {}", e))?;
        
        println!("Saved {} training samples to: {}", self.training_buffer.len(), path);
        
        // 清空缓冲区
        self.training_buffer.clear();
        
        Ok(())
    }
    
    /// 加载Python预测结果 (Python → Rust推理)
    pub fn load_prediction_from_json(&self, json: &str) -> Result<StandardPrediction, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("Prediction result parsing failed: {}", e))
    }
    
    /// 加载Python预测结果从文件
    pub fn load_prediction_from_file(&self, path: &Path) -> Result<StandardPrediction, String> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| format!("File read failed: {}", e))?;
        self.load_prediction_from_json(&json)
    }
    
    /// 导出特征到Python训练格式
    pub fn export_features_for_training(
        &self,
        features: &StandardFeatures,
        output_path: &str
    ) -> Result<(), String> {
        let json = features.to_json()?;
        std::fs::write(output_path, json)
            .map_err(|e| format!("Export failed: {}", e))?;
        Ok(())
    }
    
    /// 批量导出训练样本
    pub fn export_training_batch(
        &self,
        samples: &[TrainingSample],
        output_path: &str
    ) -> Result<(), String> {
        let batch: Vec<_> = samples.iter()
            .map(|s| s.to_training_format())
            .collect();
        
        let json = serde_json::to_string_pretty(&batch)
            .map_err(|e| format!("JSON serialization failed: {}", e))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| format!("File write failed: {}", e))?;
        
        Ok(())
    }
    
    /// 获取缓冲区状态
    pub fn buffer_status(&self) -> (usize, usize) {
        (self.training_buffer.len(), self.max_buffer_size)
    }
    
    /// 验证数据流一致性
    pub fn validate_data_flow(&self) -> Result<(), String> {
        // 创建测试特征
        let test_features = StandardFeatures {
            basic: [1.0; 16],
            color: [2.0; 16],
            texture: [3.0; 16],
            shape: [4.0; 16],
            quality: [5.0; 16],
            metadata: [6.0; 32],
            context: [7.0; 16],
        };
        
        // Test vector conversion
        let vec = test_features.to_vector();
        if vec.len() != 128 {
            return Err(format!("Invalid feature dimension: {}", vec.len()));
        }
        
        // Test JSON serialization
        let json = test_features.to_json()?;
        let restored = StandardFeatures::from_json(&json)?;
        
        // Verify consistency
        let restored_vec = restored.to_vector();
        for (i, (&original, &restored)) in vec.iter().zip(restored_vec.iter()).enumerate() {
            if (original - restored).abs() > 1e-10 {
                return Err(format!("Feature mismatch at index {}: {} vs {}", i, original, restored));
            }
        }
        
        Ok(())
    }
}

impl Default for MLDataFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_flow_validation() {
        let flow = MLDataFlow::new();
        assert!(flow.validate_data_flow().is_ok());
    }

    #[test]
    fn test_buffer_management() {
        let mut flow = MLDataFlow::new().with_buffer_size(10);
        
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
        
        assert!(flow.record_training_sample(sample).is_ok());
        assert_eq!(flow.buffer_status().0, 1);
    }
}
