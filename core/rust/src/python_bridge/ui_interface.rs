// 🧠 PIXLY v3.0 UI友好的Rust-Python桥接接口
//
// 为插件UI端提供简化的调用接口：
// - 一键配置预设
// - 状态查询接口
// - 实时性能监控
// - 错误处理与恢复

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use serde_json::Value;
use anyhow::Result;

use super::{RustPythonBridge, ImageFeatures, PredictionResult};
use super::config::{BridgeConfig, PerformanceLevel, ConfigBuilder};
use super::prediction_interface::PythonPredictionInterface;

/// UI接口状态
#[derive(Debug, Clone)]
pub enum UIStatus {
    /// 初始化中
    Initializing,
    /// 就绪
    Ready,
    /// 忙碌处理中
    Busy,
    /// 错误状态
    Error(String),
    /// 已停止
    Stopped,
}

/// 性能统计
#[derive(Debug, Clone, Default)]
pub struct PerformanceStats {
    pub total_predictions: u64,
    pub successful_predictions: u64,
    pub failed_predictions: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub avg_inference_time_ms: f32,
    pub min_inference_time_ms: f32,
    pub max_inference_time_ms: f32,
    pub total_inference_time_ms: f32,
}

/// UI友好的桥接接口
pub struct UIBridgeInterface {
    /// 核心桥接器
    bridge: Arc<Mutex<Option<RustPythonBridge>>>,
    
    /// 当前配置
    config: Arc<RwLock<BridgeConfig>>,
    
    /// 当前状态
    status: Arc<RwLock<UIStatus>>,
    
    /// 性能统计
    stats: Arc<RwLock<PerformanceStats>>,
    
    /// 错误历史
    error_history: Arc<Mutex<Vec<String>>>,
}

impl UIBridgeInterface {
    /// 创建新的UI接口
    pub fn new() -> Self {
        Self {
            bridge: Arc::new(Mutex::new(None)),
            config: Arc::new(RwLock::new(BridgeConfig::default())),
            status: Arc::new(RwLock::new(UIStatus::Initializing)),
            stats: Arc::new(RwLock::new(PerformanceStats::default())),
            error_history: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 使用预设配置初始化
    pub fn initialize_with_preset(&self, preset: &str) -> Result<()> {
        let config = match preset {
            "power_saver" => ConfigBuilder::new()
                .performance_level(PerformanceLevel::PowerSaver)
                .build()?,
            "balanced" => ConfigBuilder::new()
                .performance_level(PerformanceLevel::Balanced)
                .build()?,
            "high_performance" => ConfigBuilder::new()
                .performance_level(PerformanceLevel::HighPerformance)
                .gpu(true)
                .build()?,
            "maximum" => ConfigBuilder::new()
                .performance_level(PerformanceLevel::Maximum)
                .gpu(true)
                .debug(true)
                .build()?,
            _ => return Err(anyhow::anyhow!("未知预设: {}", preset)),
        };
        
        self.initialize_with_config(config)
    }
    
    /// 使用自定义配置初始化
    pub fn initialize_with_config(&self, config: BridgeConfig) -> Result<()> {
        *self.status.write().unwrap() = UIStatus::Initializing;
        
        match RustPythonBridge::new(&config.models_directory) {
            Ok(bridge) => {
                *self.bridge.lock().unwrap() = Some(bridge);
                *self.config.write().unwrap() = config;
                *self.status.write().unwrap() = UIStatus::Ready;
                println!("✅ UI桥接接口初始化成功");
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("初始化失败: {}", e);
                *self.status.write().unwrap() = UIStatus::Error(error_msg.clone());
                self.error_history.lock().unwrap().push(error_msg.clone());
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }
    
    /// 快速预测 (UI友好接口)
    pub fn predict_image_simple(&self, image_path: &str, quality: u8, tool: &str) -> Result<HashMap<String, Value>> {
        let start_time = std::time::Instant::now();
        
        // 检查状态
        if !matches!(*self.status.read().unwrap(), UIStatus::Ready) {
            return Err(anyhow::anyhow!("桥接器未就绪"));
        }
        
        *self.status.write().unwrap() = UIStatus::Busy;
        
        let result = {
            let mut bridge_guard = self.bridge.lock().unwrap();
            if let Some(bridge) = bridge_guard.as_mut() {
                bridge.predict_image(image_path, quality, tool)
            } else {
                Err(anyhow::anyhow!("桥接器未初始化"))
            }
        };
        
        *self.status.write().unwrap() = UIStatus::Ready;
        
        // 更新统计
        let inference_time = start_time.elapsed().as_secs_f32() * 1000.0;
        self.update_stats(result.is_ok(), inference_time);
        
        match result {
            Ok(pred_result) => {
                let mut ui_result = HashMap::new();
                ui_result.insert("quality".to_string(), Value::Number(pred_result.quality.into()));
                ui_result.insert("distance".to_string(), Value::Number(
                    serde_json::Number::from_f64(pred_result.distance as f64).unwrap()
                ));
                ui_result.insert("confidence".to_string(), Value::Number(
                    serde_json::Number::from_f64(pred_result.confidence as f64).unwrap()
                ));
                ui_result.insert("model_used".to_string(), Value::String(pred_result.model_used));
                ui_result.insert("inference_time_ms".to_string(), Value::Number(
                    serde_json::Number::from_f64(pred_result.inference_time_ms as f64).unwrap()
                ));
                ui_result.insert("reasoning".to_string(), Value::String(pred_result.reasoning));
                Ok(ui_result)
            },
            Err(e) => {
                let error_msg = format!("预测失败: {}", e);
                self.error_history.lock().unwrap().push(error_msg.clone());
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }
    
    /// 批量预测 (UI友好接口)
    pub fn predict_batch_simple(&self, requests: Vec<HashMap<String, Value>>) -> Result<Vec<HashMap<String, Value>>> {
        if !matches!(*self.status.read().unwrap(), UIStatus::Ready) {
            return Err(anyhow::anyhow!("桥接器未就绪"));
        }
        
        *self.status.write().unwrap() = UIStatus::Busy;
        
        let mut results = Vec::new();
        
        for (i, request) in requests.iter().enumerate() {
            let image_path = request.get("image_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("请求{}缺少image_path", i))?;
            
            let quality = request.get("quality")
                .and_then(|v| v.as_u64())
                .unwrap_or(85) as u8;
            
            let tool = request.get("tool")
                .and_then(|v| v.as_str())
                .unwrap_or("webp");
            
            match self.predict_image_simple(image_path, quality, tool) {
                Ok(result) => results.push(result),
                Err(e) => {
                    let mut error_result = HashMap::new();
                    error_result.insert("error".to_string(), Value::String(e.to_string()));
                    error_result.insert("image_path".to_string(), Value::String(image_path.to_string()));
                    results.push(error_result);
                }
            }
        }
        
        *self.status.write().unwrap() = UIStatus::Ready;
        Ok(results)
    }
    
    /// 获取当前状态
    pub fn get_status(&self) -> HashMap<String, Value> {
        let mut status = HashMap::new();
        
        let current_status = self.status.read().unwrap().clone();
        status.insert("status".to_string(), Value::String(match current_status {
            UIStatus::Initializing => "initializing".to_string(),
            UIStatus::Ready => "ready".to_string(),
            UIStatus::Busy => "busy".to_string(),
            UIStatus::Error(e) => format!("error: {}", e),
            UIStatus::Stopped => "stopped".to_string(),
        }));
        
        // 配置摘要
        let config = self.config.read().unwrap();
        for (key, value) in config.get_ui_summary() {
            status.insert(key, value);
        }
        
        status
    }
    
    /// 获取性能统计
    pub fn get_performance_stats(&self) -> HashMap<String, Value> {
        let stats = self.stats.read().unwrap();
        let mut result = HashMap::new();
        
        result.insert("total_predictions".to_string(), Value::Number(stats.total_predictions.into()));
        result.insert("successful_predictions".to_string(), Value::Number(stats.successful_predictions.into()));
        result.insert("failed_predictions".to_string(), Value::Number(stats.failed_predictions.into()));
        result.insert("success_rate".to_string(), {
            let rate = if stats.total_predictions > 0 {
                stats.successful_predictions as f64 / stats.total_predictions as f64 * 100.0
            } else {
                0.0
            };
            Value::Number(serde_json::Number::from_f64(rate).unwrap())
        });
        result.insert("cache_hit_rate".to_string(), {
            let total_cache_ops = stats.cache_hits + stats.cache_misses;
            let rate = if total_cache_ops > 0 {
                stats.cache_hits as f64 / total_cache_ops as f64 * 100.0
            } else {
                0.0
            };
            Value::Number(serde_json::Number::from_f64(rate).unwrap())
        });
        result.insert("avg_inference_time_ms".to_string(), 
                     Value::Number(serde_json::Number::from_f64(stats.avg_inference_time_ms as f64).unwrap()));
        result.insert("min_inference_time_ms".to_string(), 
                     Value::Number(serde_json::Number::from_f64(stats.min_inference_time_ms as f64).unwrap()));
        result.insert("max_inference_time_ms".to_string(), 
                     Value::Number(serde_json::Number::from_f64(stats.max_inference_time_ms as f64).unwrap()));
        
        result
    }
    
    /// 获取健康状态
    pub fn get_health_status(&self) -> HashMap<String, Value> {
        let mut health = HashMap::new();
        
        // 基本健康检查
        let is_ready = matches!(*self.status.read().unwrap(), UIStatus::Ready);
        health.insert("is_ready".to_string(), Value::Bool(is_ready));
        
        // 实际内存监控
        let memory_usage = self.get_memory_usage_mb();
        health.insert("memory_usage_mb".to_string(), Value::Number(memory_usage.into()));
        
        // 错误率
        let stats = self.stats.read().unwrap();
        let error_rate = if stats.total_predictions > 0 {
            stats.failed_predictions as f64 / stats.total_predictions as f64 * 100.0
        } else {
            0.0
        };
        health.insert("error_rate_percent".to_string(), 
                     Value::Number(serde_json::Number::from_f64(error_rate).unwrap()));
        
        // 最近错误
        let recent_errors: Vec<String> = self.error_history.lock().unwrap()
            .iter().rev().take(5).cloned().collect();
        health.insert("recent_errors".to_string(), 
                     Value::Array(recent_errors.into_iter().map(Value::String).collect()));
        
        health
    }
    
    /// 更新配置 (热更新)
    pub fn update_config(&self, updates: HashMap<String, Value>) -> Result<()> {
        let mut config = self.config.write().unwrap();
        
        // 更新功能开关
        if let Some(simd) = updates.get("enable_simd").and_then(|v| v.as_bool()) {
            config.features.enable_simd_features = simd;
        }
        
        if let Some(gpu) = updates.get("enable_gpu").and_then(|v| v.as_bool()) {
            config.features.enable_gpu_acceleration = gpu;
        }
        
        if let Some(debug) = updates.get("debug_mode").and_then(|v| v.as_bool()) {
            config.features.enable_debug_mode = debug;
        }
        
        if let Some(cache_size) = updates.get("cache_size").and_then(|v| v.as_u64()) {
            config.cache.max_entries = cache_size as usize;
        }
        
        // 验证配置
        config.validate()?;
        
        println!("✅ 配置已更新");
        Ok(())
    }
    
    /// 重启桥接器
    pub fn restart(&self) -> Result<()> {
        *self.status.write().unwrap() = UIStatus::Initializing;
        
        // 重置统计
        *self.stats.write().unwrap() = PerformanceStats::default();
        
        // 重新初始化
        let config = self.config.read().unwrap().clone();
        self.initialize_with_config(config)
    }
    
    /// 停止桥接器
    pub fn stop(&self) {
        *self.bridge.lock().unwrap() = None;
        *self.status.write().unwrap() = UIStatus::Stopped;
        println!("✅ 桥接器已停止");
    }
    
    /// 清空缓存和统计
    pub fn clear_cache_and_stats(&self) {
        *self.stats.write().unwrap() = PerformanceStats::default();
        self.error_history.lock().unwrap().clear();
        println!("✅ 缓存和统计已清空");
    }
    
    
    /// 获取实际内存使用情况 (MB)
    fn get_memory_usage_mb(&self) -> u64 {
        #[cfg(target_os = "linux")]
        {
            // Linux: 读取/proc/self/status
            use std::fs;
            if let Ok(status) = fs::read_to_string("/proc/self/status") {
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                return kb / 1024; // KB转MB
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS: 使用mach系统调用
            use std::mem;
            extern "C" {
                fn mach_task_self() -> u32;
                fn task_info(target_task: u32, flavor: u32, task_info_out: *mut u8, task_info_outCnt: *mut u32) -> i32;
            }
            
            const TASK_BASIC_INFO: u32 = 5;
            const TASK_BASIC_INFO_COUNT: u32 = 5;
            
            #[repr(C)]
            struct TaskBasicInfo {
                suspend_count: u32,
                virtual_size: u64,
                resident_size: u64,
                user_time: u64,
                system_time: u64,
            }
            
            unsafe {
                let mut info = mem::zeroed::<TaskBasicInfo>();
                let mut count = TASK_BASIC_INFO_COUNT;
                let result = task_info(
                    mach_task_self(),
                    TASK_BASIC_INFO,
                    &mut info as *mut _ as *mut u8,
                    &mut count,
                );
                
                if result == 0 {
                    return info.resident_size / (1024 * 1024); // Bytes转MB
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows: 使用GetProcessMemoryInfo
            use std::mem;
            extern "system" {
                fn GetCurrentProcess() -> *mut std::ffi::c_void;
                fn GetProcessMemoryInfo(
                    hProcess: *mut std::ffi::c_void,
                    ppsmemCounters: *mut ProcessMemoryCounters,
                    cb: u32,
                ) -> i32;
            }
            
            #[repr(C)]
            struct ProcessMemoryCounters {
                cb: u32,
                PageFaultCount: u32,
                PeakWorkingSetSize: u64,
                WorkingSetSize: u64,
                QuotaPeakPagedPoolUsage: u64,
                QuotaPagedPoolUsage: u64,
                QuotaPeakNonPagedPoolUsage: u64,
                QuotaNonPagedPoolUsage: u64,
                PagefileUsage: u64,
                PeakPagefileUsage: u64,
            }
            
            unsafe {
                let mut pmc = mem::zeroed::<ProcessMemoryCounters>();
                pmc.cb = mem::size_of::<ProcessMemoryCounters>() as u32;
                
                let result = GetProcessMemoryInfo(
                    GetCurrentProcess(),
                    &mut pmc,
                    mem::size_of::<ProcessMemoryCounters>() as u32,
                );
                
                if result != 0 {
                    return pmc.WorkingSetSize / (1024 * 1024); // Bytes转MB
                }
            }
        }
        
        // 回退：估算内存使用
        0
    }
    fn update_stats(&self, success: bool, inference_time: f32) {
        let mut stats = self.stats.write().unwrap();
        
        stats.total_predictions += 1;
        if success {
            stats.successful_predictions += 1;
        } else {
            stats.failed_predictions += 1;
        }
        
        stats.total_inference_time_ms += inference_time;
        stats.avg_inference_time_ms = stats.total_inference_time_ms / stats.total_predictions as f32;
        
        if stats.total_predictions == 1 {
            stats.min_inference_time_ms = inference_time;
            stats.max_inference_time_ms = inference_time;
        } else {
            stats.min_inference_time_ms = stats.min_inference_time_ms.min(inference_time);
            stats.max_inference_time_ms = stats.max_inference_time_ms.max(inference_time);
        }
    }
}

impl Default for UIBridgeInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局UI接口实例
static mut GLOBAL_UI_INTERFACE: Option<UIBridgeInterface> = None;
static UI_INIT_ONCE: std::sync::Once = std::sync::Once::new();

/// 获取全局UI接口
pub fn get_global_ui_interface() -> &'static UIBridgeInterface {
    unsafe {
        UI_INIT_ONCE.call_once(|| {
            GLOBAL_UI_INTERFACE = Some(UIBridgeInterface::new());
        });
        GLOBAL_UI_INTERFACE.as_ref().unwrap()
    }
}

/// UI友好的便捷函数
pub mod ui_helpers {
    use super::*;
    
    /// 一键预测 (最简接口)
    pub fn predict(image_path: &str, quality: u8, tool: &str) -> Result<HashMap<String, Value>> {
        let ui = get_global_ui_interface();
        ui.predict_image_simple(image_path, quality, tool)
    }
    
    /// 快速状态检查
    pub fn is_ready() -> bool {
        let ui = get_global_ui_interface();
        matches!(*ui.status.read().unwrap(), UIStatus::Ready)
    }
    
    /// 快速初始化 (默认配置)
    pub fn quick_init() -> Result<()> {
        let ui = get_global_ui_interface();
        ui.initialize_with_preset("balanced")
    }
    
    /// 获取简化状态
    pub fn get_simple_status() -> String {
        let ui = get_global_ui_interface();
        match &*ui.status.read().unwrap() {
            UIStatus::Initializing => "正在初始化...".to_string(),
            UIStatus::Ready => "就绪".to_string(),
            UIStatus::Busy => "处理中...".to_string(),
            UIStatus::Error(e) => format!("错误: {}", e),
            UIStatus::Stopped => "已停止".to_string(),
        }
    }
}
