//! 🤖 AI模型路由系统
//! 
//! 多模型版本管理和A/B测试

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 模型版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: String,
    pub name: String,
    pub version: String,
    pub model_type: ModelType,
    pub path: PathBuf,
    pub accuracy: f64,
    pub latency_ms: f64,
    pub enabled: bool,
    pub weight: f64, // A/B测试权重
}

/// 模型类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    LightGBM,
    PPO,
    LinearRegression,
    BayesianOptimizer,
}

/// 路由策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    /// 最佳性能（选择准确率最高的）
    BestAccuracy,
    /// 最快响应（选择延迟最低的）
    FastestResponse,
    /// A/B测试（按权重随机选择）
    ABTesting,
    /// 轮询
    RoundRobin,
    /// 指定模型
    Specific(String),
}

/// 模型路由器
pub struct ModelRouter {
    models: HashMap<String, ModelVersion>,
    strategy: RoutingStrategy,
    current_index: usize,
}

impl ModelRouter {
    /// 创建新的路由器
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            models: HashMap::new(),
            strategy,
            current_index: 0,
        }
    }
    
    /// 注册模型
    pub fn register_model(&mut self, model: ModelVersion) {
        self.models.insert(model.id.clone(), model);
    }
    
    /// 选择模型
    pub fn select_model(&mut self) -> Option<&ModelVersion> {
        let enabled_models: Vec<_> = self.models.values()
            .filter(|m| m.enabled)
            .collect();
        
        if enabled_models.is_empty() {
            return None;
        }
        
        match &self.strategy {
            RoutingStrategy::BestAccuracy => {
                enabled_models.iter()
                    .max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap())
                    .copied()
            }
            RoutingStrategy::FastestResponse => {
                enabled_models.iter()
                    .min_by(|a, b| a.latency_ms.partial_cmp(&b.latency_ms).unwrap())
                    .copied()
            }
            RoutingStrategy::ABTesting => {
                // 按权重随机选择（简化版：使用时间戳作为随机源）
                use std::time::{SystemTime, UNIX_EPOCH};
                let total_weight: f64 = enabled_models.iter().map(|m| m.weight).sum();
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let random = ((timestamp % 10000) as f64 / 10000.0) * total_weight;
                
                let mut accumulated = 0.0;
                for model in &enabled_models {
                    accumulated += model.weight;
                    if random <= accumulated {
                        return Some(model);
                    }
                }
                enabled_models.first().copied()
            }
            RoutingStrategy::RoundRobin => {
                let model = enabled_models.get(self.current_index % enabled_models.len()).copied();
                self.current_index += 1;
                model
            }
            RoutingStrategy::Specific(id) => {
                self.models.get(id).filter(|m| m.enabled)
            }
        }
    }
    
    /// 更新模型统计
    pub fn update_model_stats(&mut self, model_id: &str, accuracy: f64, latency_ms: f64) {
        if let Some(model) = self.models.get_mut(model_id) {
            model.accuracy = accuracy;
            model.latency_ms = latency_ms;
        }
    }
    
    /// 启用/禁用模型
    pub fn set_model_enabled(&mut self, model_id: &str, enabled: bool) {
        if let Some(model) = self.models.get_mut(model_id) {
            model.enabled = enabled;
        }
    }
    
    /// 获取所有模型
    pub fn list_models(&self) -> Vec<&ModelVersion> {
        self.models.values().collect()
    }
}

impl Default for ModelRouter {
    fn default() -> Self {
        Self::new(RoutingStrategy::BestAccuracy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registration() {
        let mut router = ModelRouter::default();
        
        let model = ModelVersion {
            id: "lgbm_v1".to_string(),
            name: "LightGBM v1".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::LightGBM,
            path: PathBuf::from("models/lgbm_v1.txt"),
            accuracy: 0.95,
            latency_ms: 5.0,
            enabled: true,
            weight: 1.0,
        };
        
        router.register_model(model);
        assert_eq!(router.list_models().len(), 1);
    }

    #[test]
    fn test_best_accuracy_selection() {
        let mut router = ModelRouter::new(RoutingStrategy::BestAccuracy);
        
        router.register_model(ModelVersion {
            id: "model1".to_string(),
            name: "Model 1".to_string(),
            version: "1.0".to_string(),
            model_type: ModelType::LightGBM,
            path: PathBuf::from("model1"),
            accuracy: 0.90,
            latency_ms: 10.0,
            enabled: true,
            weight: 1.0,
        });
        
        router.register_model(ModelVersion {
            id: "model2".to_string(),
            name: "Model 2".to_string(),
            version: "2.0".to_string(),
            model_type: ModelType::PPO,
            path: PathBuf::from("model2"),
            accuracy: 0.95,
            latency_ms: 15.0,
            enabled: true,
            weight: 1.0,
        });
        
        let selected = router.select_model().unwrap();
        assert_eq!(selected.id, "model2");
        assert_eq!(selected.accuracy, 0.95);
    }
}
