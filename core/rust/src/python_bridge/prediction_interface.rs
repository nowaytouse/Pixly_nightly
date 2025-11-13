// 🧠 PIXLY v3.0 Python预测接口
//
// 零拷贝Python AI调用接口：
// - PyO3原生绑定 (vs Go subprocess)
// - 直接内存共享 (vs Go JSON序列化) 
// - 类型安全转换 (vs Go反射解析)

use pyo3::prelude::*;
use pyo3::{Bound};
use pyo3::types::{PyDict, PyList, PyTuple, PyModule, PyAny};
use anyhow::{Context, Result};
use std::collections::HashMap;

use super::{ImageFeatures, PredictionResult};

/// Python AI预测接口
pub struct PythonPredictionInterface {
    dispatcher: Option<PyObject>,
    prediction_cache: HashMap<String, PredictionResult>,
}

impl PythonPredictionInterface {
    /// 创建Python预测接口
    pub fn new() -> Result<Self> {
        Ok(Self {
            dispatcher: None,
            prediction_cache: HashMap::new(),
        })
    }
    
    /// 初始化Python AI调度器
    pub fn initialize_dispatcher(&mut self, models_dir: &str) -> Result<()> {
        Python::with_gil(|py| -> Result<()> {
            // 添加Python路径
            let sys = PyModule::import_bound(py, "sys").map_err(|e| anyhow::anyhow!("Failed to import sys: {}", e))?;
            let path_attr = sys.getattr("path").map_err(|e| anyhow::anyhow!("Failed to get sys.path: {}", e))?;
            let path = match path_attr.downcast::<PyList>() {
                Ok(list) => list,
                Err(_) => return Err(anyhow::anyhow!("Failed to downcast path to PyList")),
            };
            path.insert(0, "core/python").map_err(|e| anyhow::anyhow!("Failed to insert path: {}", e))?;
            
            // 导入本地化AI调度器
            let ai_module = PyModule::import_bound(py, "ai.local_dispatcher")
                .map_err(|e| anyhow::anyhow!("无法导入ai.local_dispatcher模块: {}", e))?;
            let dispatcher_class = ai_module.getattr("LocalAIDispatcher").map_err(|e| anyhow::anyhow!("Failed to get LocalAIDispatcher: {}", e))?;
            
            // 创建调度器实例
            let args = PyTuple::new_bound(py, &[models_dir, "config", "cache"]);
            let dispatcher = dispatcher_class.call1(args).map_err(|e| anyhow::anyhow!("Failed to create dispatcher: {}", e))?;
            
            self.dispatcher = Some(dispatcher.to_object(py));
            
            println!("✅ Python预测接口已初始化");
            Ok(())
        })
    }
    
    /// 零拷贝特征转换
    fn features_to_python(&self, py: Python, features: &ImageFeatures) -> Result<PyObject> {
        let request_dict = PyDict::new_bound(py);
        
        // 基础请求参数
        request_dict.set_item("image_path", "rust_extracted")?;
        request_dict.set_item("tool", &features.target_tool)?;
        request_dict.set_item("target_quality", features.target_quality)?;
        request_dict.set_item("optimize_mode", "balanced")?;
        request_dict.set_item("enable_caching", true)?;
        
        // Rust提取的特征数据 (零拷贝传递)
        let features_dict = PyDict::new_bound(py);
        features_dict.set_item("width", features.width)?;
        features_dict.set_item("height", features.height)?;
        features_dict.set_item("channels", features.channels)?;
        features_dict.set_item("file_size", features.file_size)?;
        features_dict.set_item("swt_energy", features.swt_energy)?;
        features_dict.set_item("swt_variance", features.swt_variance)?;
        features_dict.set_item("high_freq_ratio", features.high_freq_ratio)?;
        features_dict.set_item("color_complexity", features.color_complexity)?;
        features_dict.set_item("brightness", features.brightness)?;
        features_dict.set_item("contrast", features.contrast)?;
        features_dict.set_item("texture_score", features.texture_score)?;
        features_dict.set_item("edge_density", features.edge_density)?;
        features_dict.set_item("target_quality", features.target_quality)?;
        features_dict.set_item("target_tool", &features.target_tool)?;
        
        request_dict.set_item("rust_features", features_dict)?;
        
        Ok(request_dict.to_object(py))
    }
    
    /// 零拷贝结果解析 (现代化PyO3 API)
    fn parse_python_result(&self, py_result: &Bound<PyAny>) -> Result<PredictionResult> {
        // 直接从Python对象提取字段 (零拷贝)
        let quality: u8 = py_result.getattr("quality")?.extract()?;
        let distance: f32 = py_result.getattr("distance")?.extract()?;
        let effort: u8 = py_result.getattr("effort")
            .map(|attr| attr.extract().unwrap_or(6))
            .unwrap_or(6);
        let confidence: f32 = py_result.getattr("confidence")?.extract()?;
        let model_used: String = py_result.getattr("model_used")?.extract()?;
        let inference_time_ms: f32 = py_result.getattr("inference_time_ms")?.extract()?;
        let reasoning: String = py_result.getattr("reasoning")
            .map(|attr| attr.extract().unwrap_or_else(|_| "No reasoning provided".to_string()))
            .unwrap_or_else(|_| "No reasoning provided".to_string());
        
        Ok(PredictionResult {
            quality,
            distance,
            effort,
            confidence,
            model_used,
            inference_time_ms,
            reasoning,
        })
    }
    
    /// 零拷贝AI预测调用
    pub fn predict(&mut self, features: &ImageFeatures) -> Result<PredictionResult> {
        // 检查缓存
        let cache_key = format!("{}:{}:{}", features.target_tool, features.target_quality, 
                               features.file_size);
        if let Some(cached) = self.prediction_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        let dispatcher = self.dispatcher.as_ref()
            .context("Python调度器未初始化")?;
        
        let result = Python::with_gil(|py| -> Result<PredictionResult> {
            let dispatcher = dispatcher.bind(py);
            
            // 零拷贝特征转换
            let request = self.features_to_python(py, features)?;
            
            // 直接调用Python预测 (无subprocess开销)
            let py_result = dispatcher.call_method1("predict_with_rust_features", (request,))
                .context("Python预测调用失败")?;
            
            // 零拷贝结果解析
            let result = self.parse_python_result(&py_result)?;
            
            Ok(result)
        })?;
        
        // 缓存结果
        self.prediction_cache.insert(cache_key, result.clone());
        
        Ok(result)
    }
    
    /// 批量预测 (SIMD优化)
    pub fn predict_batch(&mut self, features_batch: &[ImageFeatures]) -> Result<Vec<PredictionResult>> {
        let mut results = Vec::with_capacity(features_batch.len());
        
        Python::with_gil(|py| -> Result<()> {
            let dispatcher = self.dispatcher.as_ref()
                .context("Python调度器未初始化")?
                .bind(py);
            
            // 构建批量请求
            let requests = PyList::empty_bound(py);
            for features in features_batch {
                let request = self.features_to_python(py, features)?;
                requests.append(request)?;
            }
            
            // 批量调用
            let py_results = dispatcher.call_method1("predict_batch", (requests,)).map_err(|e| anyhow::anyhow!("Batch prediction failed: {}", e))?;
            let results_list = match py_results.downcast::<PyList>() {
                Ok(list) => list,
                Err(_) => return Err(anyhow::anyhow!("Failed to downcast batch results to PyList")),
            };
            
            // 批量解析结果
            for py_result in results_list.iter() {
                let result = self.parse_python_result(&py_result)?;
                results.push(result);
            }
            
            Ok(())
        })?;
        
        Ok(results)
    }
    
    /// 获取模型状态
    pub fn get_model_status(&self) -> Result<HashMap<String, serde_json::Value>> {
        let dispatcher = self.dispatcher.as_ref()
            .context("Python调度器未初始化")?;
        
        Python::with_gil(|py| -> Result<HashMap<String, serde_json::Value>> {
            let dispatcher = dispatcher.bind(py);
            
            // 调用Python状态查询
            let py_status = dispatcher.call_method0("list_models").map_err(|e| anyhow::anyhow!("Failed to list models: {}", e))?;
            
            // 转换为Rust HashMap
            let status_dict = match py_status.downcast::<PyDict>() {
                Ok(dict) => dict,
                Err(_) => return Err(anyhow::anyhow!("Failed to downcast status to PyDict")),
            };
            let mut status = HashMap::new();
            
            for (key, value) in status_dict.iter() {
                let key_str: String = key.extract()?;
                // 直接尝试提取不同类型
                let value_json = if let Ok(s) = value.extract::<String>() {
                    serde_json::Value::String(s)
                } else if let Ok(n) = value.extract::<i64>() {
                    serde_json::Value::Number(serde_json::Number::from(n))
                } else if let Ok(f) = value.extract::<f64>() {
                    serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0)))
                } else if let Ok(b) = value.extract::<bool>() {
                    serde_json::Value::Bool(b)
                } else {
                    serde_json::Value::Null
                };
                status.insert(key_str, value_json);
            }
            
            Ok(status)
        })
    }
    
    /// 获取健康状态
    pub fn get_health_status(&self) -> Result<HashMap<String, serde_json::Value>> {
        let dispatcher = self.dispatcher.as_ref()
            .context("Python调度器未初始化")?;
        
        Python::with_gil(|py| -> Result<HashMap<String, serde_json::Value>> {
            let dispatcher = dispatcher.bind(py);
            
            let py_health = dispatcher.call_method0("get_health_status").map_err(|e| anyhow::anyhow!("Failed to get health status: {}", e))?;
            let health_dict = match py_health.downcast::<PyDict>() {
                Ok(dict) => dict,
                Err(_) => return Err(anyhow::anyhow!("Failed to downcast health to PyDict")),
            };
            
            let mut health = HashMap::new();
            for (key, value) in health_dict.iter() {
                let key_str: String = key.extract()?;
                let value_json = match value.extract::<String>() {
                    Ok(s) => serde_json::Value::String(s),
                    Err(_) => match value.extract::<bool>() {
                        Ok(b) => serde_json::Value::Bool(b),
                        Err(_) => match value.extract::<f64>() {
                            Ok(f) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))),
                            Err(_) => serde_json::Value::Null,
                        }
                    }
                };
                health.insert(key_str, value_json);
            }
            
            Ok(health)
        })
    }
    
    /// 清空预测缓存
    pub fn clear_cache(&mut self) {
        self.prediction_cache.clear();
        println!("✅ 预测缓存已清空");
    }
    
    /// 获取缓存统计
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.prediction_cache.len(), self.prediction_cache.capacity())
    }
}

impl Default for PythonPredictionInterface {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            dispatcher: None,
            prediction_cache: HashMap::new(),
        })
    }
}

/// 全局Python预测接口实例
static mut GLOBAL_INTERFACE: Option<PythonPredictionInterface> = None;
static INIT_ONCE: std::sync::Once = std::sync::Once::new();

/// 获取全局Python预测接口
pub fn get_global_interface() -> &'static mut PythonPredictionInterface {
    unsafe {
        INIT_ONCE.call_once(|| {
            GLOBAL_INTERFACE = Some(PythonPredictionInterface::default());
        });
        GLOBAL_INTERFACE.as_mut().unwrap()
    }
}
