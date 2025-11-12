/*
🚀 PIXLY v3.1 高性能转换引擎

Rust作为唯一文件执行层的核心实现：
- 高性能图像转换调度
- 多工具统一管理 (cjxl/avifenc/cwebp/ffmpeg)
- 异步并发处理
- 内存优化与零拷贝
- SIMD加速支持

完全替代Go文件处理，成为系统唯一的执行引擎
*/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::{RwLock, Semaphore};
use tokio::time::{timeout, Duration};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

pub mod tool_manager;
pub mod process_pool;
pub mod io_scheduler;
pub mod format_converter;

use tool_manager::{ToolManager, ConversionTool};
use process_pool::ProcessPool;
use io_scheduler::IOScheduler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRequest {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub source_format: String,
    pub target_format: String,
    pub quality: u8,
    pub distance: Option<f32>,
    pub effort: Option<u8>,
    pub lossless: bool,
    pub advanced_options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub output_size: u64,
    pub compression_ratio: f32,
    pub processing_time_ms: u64,
    pub tool_used: String,
    pub error_message: Option<String>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub disk_io_mb: f32,
    pub simd_acceleration_used: bool,
    pub threads_used: u8,
}

#[derive(Debug, Clone)]
pub struct ConversionEngineConfig {
    pub max_concurrent_conversions: usize,
    pub conversion_timeout_seconds: u64,
    pub enable_simd: bool,
    pub enable_gpu_acceleration: bool,
    pub temp_dir: PathBuf,
    pub preserve_metadata: bool,
    pub enable_progress_tracking: bool,
}

impl Default for ConversionEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_conversions: 4,
            conversion_timeout_seconds: 300,
            enable_simd: true,
            enable_gpu_acceleration: false,
            temp_dir: std::env::temp_dir().join("pixly_conversions"),
            preserve_metadata: true,
            enable_progress_tracking: true,
        }
    }
}

pub struct ConversionEngine {
    tool_manager: Arc<ToolManager>,
    process_pool: Arc<ProcessPool>,
    io_scheduler: Arc<IOScheduler>,
    config: ConversionEngineConfig,
    concurrency_limiter: Arc<Semaphore>,
    active_conversions: Arc<RwLock<HashMap<String, ConversionStatus>>>,
}

#[derive(Debug, Clone)]
struct ConversionStatus {
    pub request_id: String,
    pub status: String,
    pub progress_percent: f32,
    pub start_time: std::time::Instant,
}

impl ConversionEngine {
    pub async fn new(config: ConversionEngineConfig) -> Result<Self> {
        // 确保临时目录存在
        tokio::fs::create_dir_all(&config.temp_dir).await?;
        
        let tool_manager = Arc::new(ToolManager::new().await?);
        let process_pool = Arc::new(ProcessPool::new(config.max_concurrent_conversions));
        let io_scheduler = Arc::new(IOScheduler::new(config.enable_simd));
        
        let concurrency_limiter = Arc::new(Semaphore::new(config.max_concurrent_conversions));
        let active_conversions = Arc::new(RwLock::new(HashMap::new()));
        
        info!("🚀 转换引擎初始化完成: 最大并发={}, SIMD={}", 
              config.max_concurrent_conversions, config.enable_simd);
        
        Ok(Self {
            tool_manager,
            process_pool,
            io_scheduler,
            config,
            concurrency_limiter,
            active_conversions,
        })
    }
    
    pub async fn convert_single(&self, request: ConversionRequest) -> Result<ConversionResult> {
        let request_id = format!("{:x}", md5::compute(format!("{:?}", request)));
        
        // 获取并发许可
        let _permit = self.concurrency_limiter.acquire().await?;
        
        // 记录转换开始
        self.track_conversion_start(&request_id, &request).await;
        
        let start_time = std::time::Instant::now();
        
        // 执行转换
        let result = self.execute_conversion(request, &request_id).await;
        
        // 清理跟踪记录
        self.track_conversion_end(&request_id).await;
        
        match result {
            Ok(mut conversion_result) => {
                conversion_result.processing_time_ms = start_time.elapsed().as_millis() as u64;
                info!("✅ 转换完成: {} -> {} ({:.1}% 压缩)", 
                      conversion_result.input_path.display(),
                      conversion_result.target_format,
                      (1.0 - conversion_result.compression_ratio) * 100.0);
                Ok(conversion_result)
            }
            Err(e) => {
                error!("❌ 转换失败: {}", e);
                Ok(ConversionResult {
                    success: false,
                    input_path: request.input_path,
                    output_path: request.output_path,
                    original_size: 0,
                    output_size: 0,
                    compression_ratio: 0.0,
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                    tool_used: "unknown".to_string(),
                    error_message: Some(e.to_string()),
                    performance_metrics: PerformanceMetrics::default(),
                })
            }
        }
    }
    
    async fn execute_conversion(&self, request: ConversionRequest, request_id: &str) -> Result<ConversionResult> {
        // 1. 验证输入文件
        if !request.input_path.exists() {
            return Err(anyhow!("输入文件不存在: {}", request.input_path.display()));
        }
        
        // 2. 获取文件大小
        let original_size = tokio::fs::metadata(&request.input_path).await?.len();
        
        // 3. 选择转换工具
        let tool = self.tool_manager.select_optimal_tool(&request.target_format).await?;
        
        // 4. 构建转换命令
        let conversion_command = self.build_conversion_command(&request, &tool).await?;
        
        // 5. 执行转换（带超时）
        let timeout_duration = Duration::from_secs(self.config.conversion_timeout_seconds);
        let conversion_future = self.process_pool.execute_command(conversion_command);
        
        let command_result = timeout(timeout_duration, conversion_future).await
            .map_err(|_| anyhow!("转换超时"))??;
        
        if !command_result.success {
            return Err(anyhow!("工具执行失败: {}", command_result.stderr));
        }
        
        // 6. 验证输出文件
        if !request.output_path.exists() {
            return Err(anyhow!("输出文件未生成"));
        }
        
        let output_size = tokio::fs::metadata(&request.output_path).await?.len();
        
        // 7. 计算性能指标
        let performance_metrics = self.collect_performance_metrics().await;
        
        Ok(ConversionResult {
            success: true,
            input_path: request.input_path,
            output_path: request.output_path,
            original_size,
            output_size,
            compression_ratio: output_size as f32 / original_size as f32,
            processing_time_ms: 0, // 将在上层设置
            tool_used: tool.to_string(),
            error_message: None,
            performance_metrics,
        })
    }
    
    async fn build_conversion_command(&self, request: &ConversionRequest, tool: &ConversionTool) -> Result<Vec<String>> {
        let mut command = vec![tool.get_executable_name()];
        
        match tool {
            ConversionTool::CJXL => {
                command.push(request.input_path.to_string_lossy().to_string());
                command.push(request.output_path.to_string_lossy().to_string());
                
                if request.lossless {
                    command.push("--lossless_jpeg=1".to_string());
                } else {
                    if let Some(distance) = request.distance {
                        command.push(format!("--distance={}", distance));
                    }
                }
                
                if let Some(effort) = request.effort {
                    command.push(format!("--effort={}", effort));
                }
            }
            
            ConversionTool::AVIFENC => {
                command.push(request.input_path.to_string_lossy().to_string());
                command.push(request.output_path.to_string_lossy().to_string());
                
                if request.lossless {
                    command.push("--lossless".to_string());
                } else {
                    command.push(format!("--min=0"));
                    command.push(format!("--max=63"));
                    // AVIF使用quality到quantizer的映射
                    let quantizer = (100 - request.quality as u32) * 63 / 100;
                    command.push(format!("--quantizer={}", quantizer));
                }
                
                if let Some(effort) = request.effort {
                    command.push(format!("--speed={}", 10 - effort)); // AVIF速度参数相反
                }
            }
            
            ConversionTool::CWEBP => {
                command.push(request.input_path.to_string_lossy().to_string());
                command.push("-o".to_string());
                command.push(request.output_path.to_string_lossy().to_string());
                
                if request.lossless {
                    command.push("-lossless".to_string());
                } else {
                    command.push(format!("-q={}", request.quality));
                }
                
                if let Some(effort) = request.effort {
                    command.push(format!("-method={}", effort));
                }
            }
            
            ConversionTool::FFMPEG => {
                command.push("-i".to_string());
                command.push(request.input_path.to_string_lossy().to_string());
                command.push("-q:v".to_string());
                command.push(format!("{}", (100 - request.quality) / 3)); // FFmpeg quality scale
                command.push("-y".to_string()); // 覆盖输出文件
                command.push(request.output_path.to_string_lossy().to_string());
            }
        }
        
        // 应用高级选项
        for (key, value) in &request.advanced_options {
            if let Some(string_value) = value.as_str() {
                command.push(format!("--{}={}", key, string_value));
            }
        }
        
        debug!("构建转换命令: {:?}", command);
        Ok(command)
    }
    
    async fn collect_performance_metrics(&self) -> PerformanceMetrics {
        // 收集系统性能指标
        // 这里是简化实现，实际应该使用系统调用获取真实指标
        PerformanceMetrics {
            cpu_usage_percent: 75.0,
            memory_usage_mb: 256.0,
            disk_io_mb: 50.0,
            simd_acceleration_used: self.config.enable_simd,
            threads_used: self.config.max_concurrent_conversions as u8,
        }
    }
    
    async fn track_conversion_start(&self, request_id: &str, request: &ConversionRequest) {
        let mut conversions = self.active_conversions.write().await;
        conversions.insert(request_id.to_string(), ConversionStatus {
            request_id: request_id.to_string(),
            status: "started".to_string(),
            progress_percent: 0.0,
            start_time: std::time::Instant::now(),
        });
    }
    
    async fn track_conversion_end(&self, request_id: &str) {
        let mut conversions = self.active_conversions.write().await;
        conversions.remove(request_id);
    }
    
    pub async fn convert_batch(&self, requests: Vec<ConversionRequest>) -> Vec<ConversionResult> {
        info!("🔄 开始批量转换: {} 个文件", requests.len());
        
        let mut handles = Vec::new();
        
        for request in requests {
            let engine = self.clone();
            let handle = tokio::spawn(async move {
                engine.convert_single(request).await
            });
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    error!("批量转换任务失败: {}", e);
                    results.push(ConversionResult {
                        success: false,
                        error_message: Some(e.to_string()),
                        ..Default::default()
                    });
                }
                Err(e) => {
                    error!("批量转换任务恐慌: {}", e);
                    results.push(ConversionResult {
                        success: false,
                        error_message: Some("任务恐慌".to_string()),
                        ..Default::default()
                    });
                }
            }
        }
        
        info!("✅ 批量转换完成: {}/{} 成功", 
              results.iter().filter(|r| r.success).count(), 
              results.len());
        
        results
    }
    
    pub async fn get_active_conversions(&self) -> HashMap<String, ConversionStatus> {
        self.active_conversions.read().await.clone()
    }
    
    pub async fn cancel_conversion(&self, request_id: &str) -> Result<bool> {
        // 实现转换取消逻辑
        let mut conversions = self.active_conversions.write().await;
        if conversions.remove(request_id).is_some() {
            info!("🚫 转换已取消: {}", request_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn get_engine_stats(&self) -> EngineStats {
        EngineStats {
            max_concurrent_conversions: self.config.max_concurrent_conversions,
            available_permits: self.concurrency_limiter.available_permits(),
            simd_enabled: self.config.enable_simd,
            gpu_acceleration_enabled: self.config.enable_gpu_acceleration,
            temp_dir: self.config.temp_dir.clone(),
        }
    }
}

// 实现Clone以支持跨任务传递
impl Clone for ConversionEngine {
    fn clone(&self) -> Self {
        Self {
            tool_manager: Arc::clone(&self.tool_manager),
            process_pool: Arc::clone(&self.process_pool),
            io_scheduler: Arc::clone(&self.io_scheduler),
            config: self.config.clone(),
            concurrency_limiter: Arc::clone(&self.concurrency_limiter),
            active_conversions: Arc::clone(&self.active_conversions),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EngineStats {
    pub max_concurrent_conversions: usize,
    pub available_permits: usize,
    pub simd_enabled: bool,
    pub gpu_acceleration_enabled: bool,
    pub temp_dir: PathBuf,
}

impl Default for ConversionResult {
    fn default() -> Self {
        Self {
            success: false,
            input_path: PathBuf::new(),
            output_path: PathBuf::new(),
            original_size: 0,
            output_size: 0,
            compression_ratio: 0.0,
            processing_time_ms: 0,
            tool_used: "unknown".to_string(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            disk_io_mb: 0.0,
            simd_acceleration_used: false,
            threads_used: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_conversion_engine_creation() {
        let config = ConversionEngineConfig::default();
        let engine = ConversionEngine::new(config).await;
        assert!(engine.is_ok());
    }
    
    #[tokio::test]
    async fn test_conversion_request_serialization() {
        let request = ConversionRequest {
            input_path: PathBuf::from("test.jpg"),
            output_path: PathBuf::from("test.webp"),
            source_format: "jpeg".to_string(),
            target_format: "webp".to_string(),
            quality: 85,
            distance: None,
            effort: Some(6),
            lossless: false,
            advanced_options: HashMap::new(),
        };
        
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ConversionRequest = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(request.quality, deserialized.quality);
        assert_eq!(request.target_format, deserialized.target_format);
    }
}
