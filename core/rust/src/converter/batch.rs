/**
 * ==========================================
 * PIXLY Batch Converter
 * ==========================================
 *
 * 批量转换系统，支持:
 * - 多文件批量处理
 * - 进度回调
 * - 多线程并行
 * - 错误恢复
 *
 * @module BatchConverter
 * @version 1.0.0
 * @date 2025-11-06
 */
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
// 🔧 统一日志系统
use tracing::{info, warn, debug};

use crate::converter::{ImageConverter, ConversionConfig, ConversionResult};

/// 进度信息
#[derive(Debug, Clone)]
pub struct Progress {
    /// 当前处理的文件索引（从0开始）
    pub current: usize,
    
    /// 总文件数
    pub total: usize,
    
    /// 当前文件名
    pub current_file: String,
    
    /// 当前进度百分比 (0-100)
    pub percentage: f32,
    
    /// 已完成的文件数
    pub completed: usize,
    
    /// 失败的文件数
    pub failed: usize,
}

impl Progress {
    /// 创建新的进度信息
    pub fn new(current: usize, total: usize, current_file: String) -> Self {
        let percentage = if total > 0 {
            (current as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        Self {
            current,
            total,
            current_file,
            percentage,
            completed: 0,
            failed: 0,
        }
    }
}

/// 进度回调函数类型
pub type ProgressCallback = Arc<dyn Fn(Progress) + Send + Sync>;

/// 批量转换任务
#[derive(Debug, Clone)]
pub struct BatchTask {
    /// 输入文件路径
    pub input: PathBuf,
    
    /// 输出文件路径
    pub output: PathBuf,
    
    /// 目标格式
    pub format: String,
}

impl BatchTask {
    /// 创建新的批量任务
    pub fn new<P: Into<PathBuf>>(input: P, output: P, format: &str) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            format: format.to_string(),
        }
    }
}

/// 批量转换结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// 总任务数
    pub total: usize,
    
    /// 成功数量
    pub succeeded: usize,
    
    /// 失败数量
    pub failed: usize,
    
    /// 失败的任务详情
    pub failures: Vec<(String, String)>,  // (文件名, 错误信息)
    
    /// 总耗时（毫秒）
    pub total_time_ms: u64,
}

/// 批量转换器
pub struct BatchConverter {
    config: ConversionConfig,
    converter: ImageConverter,
    progress_callback: Option<ProgressCallback>,
    parallel: bool,
    max_threads: usize,
}

impl BatchConverter {
    /// 创建新的批量转换器
    pub fn new(config: ConversionConfig) -> Self {
        let converter = ImageConverter::new(config.clone());
        
        Self {
            config,
            converter,
            progress_callback: None,
            parallel: true,
            max_threads: num_cpus::get(),
        }
    }
    
    /// 获取批量转换配置
    /// 
    /// 用于检查当前配置、传递到子任务等
    pub fn get_config(&self) -> &ConversionConfig {
        &self.config
    }
    
    /// 设置进度回调
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(Progress) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(callback));
        self
    }
    
    /// 设置是否并行处理
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }
    
    /// 设置最大线程数
    pub fn with_max_threads(mut self, max_threads: usize) -> Self {
        self.max_threads = max_threads.max(1);
        self
    }
    
    /// 批量转换文件
    pub fn convert_batch(&self, tasks: Vec<BatchTask>) -> Result<BatchResult> {
        let start = std::time::Instant::now();
        let total = tasks.len();
        
        info!("🚀 Starting batch conversion: {} files", total);
        
        if total == 0 {
            return Ok(BatchResult {
                total: 0,
                succeeded: 0,
                failed: 0,
                failures: Vec::new(),
                total_time_ms: 0,
            });
        }
        
        let result = if self.parallel {
            self.convert_parallel(tasks)?
        } else {
            self.convert_sequential(tasks)?
        };
        
        let elapsed = start.elapsed().as_millis() as u64;
        
        info!(
            "✅ Batch conversion completed: {}/{} succeeded, {}/{} failed, {}ms",
            result.succeeded,
            total,
            result.failed,
            total,
            elapsed
        );
        
        Ok(BatchResult {
            total_time_ms: elapsed,
            ..result
        })
    }
    
    /// 串行转换
    fn convert_sequential(&self, tasks: Vec<BatchTask>) -> Result<BatchResult> {
        let total = tasks.len();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut failures = Vec::new();
        
        for (idx, task) in tasks.iter().enumerate() {
            // 发送进度
            if let Some(ref callback) = self.progress_callback {
                let mut progress = Progress::new(
                    idx,
                    total,
                    task.input.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                );
                progress.completed = succeeded;
                progress.failed = failed;
                callback(progress);
            }
            
            // 执行转换
            match self.convert_single(task) {
                Ok(_) => {
                    succeeded += 1;
                    debug!("✓ Converted: {:?}", task.input);
                }
                Err(e) => {
                    failed += 1;
                    let filename = task.input.to_string_lossy().to_string();
                    let error_msg = e.to_string();
                    failures.push((filename.clone(), error_msg.clone()));
                    warn!("✗ Failed to convert {}: {}", filename, error_msg);
                }
            }
        }
        
        Ok(BatchResult {
            total,
            succeeded,
            failed,
            failures,
            total_time_ms: 0,  // 由外层设置
        })
    }
    
    /// 并行转换
    fn convert_parallel(&self, tasks: Vec<BatchTask>) -> Result<BatchResult> {
        use rayon::prelude::*;
        
        let total = tasks.len();
        let succeeded = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicUsize::new(0));
        let failures = Arc::new(std::sync::Mutex::new(Vec::new()));
        
        // 配置线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.max_threads)
            .build()
            .context("Failed to create thread pool")?;
        
        pool.install(|| {
            tasks.par_iter().enumerate().for_each(|(idx, task)| {
                // 发送进度
                if let Some(ref callback) = self.progress_callback {
                    let mut progress = Progress::new(
                        idx,
                        total,
                        task.input.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                    );
                    progress.completed = succeeded.load(Ordering::Relaxed);
                    progress.failed = failed.load(Ordering::Relaxed);
                    callback(progress);
                }
                
                // 执行转换
                match self.convert_single(task) {
                    Ok(_) => {
                        succeeded.fetch_add(1, Ordering::Relaxed);
                        debug!("✓ Converted: {:?}", task.input);
                    }
                    Err(e) => {
                        failed.fetch_add(1, Ordering::Relaxed);
                        let filename = task.input.to_string_lossy().to_string();
                        let error_msg = e.to_string();
                        
                        if let Ok(mut f) = failures.lock() {
                            f.push((filename.clone(), error_msg.clone()));
                        }
                        
                        warn!("✗ Failed to convert {}: {}", filename, error_msg);
                    }
                }
            });
        });
        
        let succeeded_count = succeeded.load(Ordering::Relaxed);
        let failed_count = failed.load(Ordering::Relaxed);
        let failures_vec = failures.lock()
            .expect("Failures mutex poisoned")
            .clone();
        
        Ok(BatchResult {
            total,
            succeeded: succeeded_count,
            failed: failed_count,
            failures: failures_vec,
            total_time_ms: 0,  // 由外层设置
        })
    }
    
    /// 转换单个文件
    fn convert_single(&self, task: &BatchTask) -> Result<ConversionResult> {
        self.converter.convert(
            &task.input,
            &task.output,
            &task.format,
        )
    }
}

/// 从目录创建批量任务
pub fn create_batch_from_directory<P: AsRef<Path>>(
    input_dir: P,
    output_dir: P,
    format: &str,
    recursive: bool,
) -> Result<Vec<BatchTask>> {
    use walkdir::WalkDir;
    
    let input_path = input_dir.as_ref();
    let output_path = output_dir.as_ref();
    
    let mut tasks = Vec::new();
    
    let walker = if recursive {
        WalkDir::new(input_path)
    } else {
        WalkDir::new(input_path).max_depth(1)
    };
    
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        
        let input_file = entry.path();
        
        // 检查是否为图像文件
        if let Some(ext) = input_file.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if !matches!(ext_str.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "avif" | "bmp" | "tiff") {
                continue;
            }
        } else {
            continue;
        }
        
        // 构建输出路径
        let relative_path = input_file.strip_prefix(input_path)
            .context("Failed to get relative path")?;
        
        let mut output_file = output_path.join(relative_path);
        output_file.set_extension(format);
        
        // 确保输出目录存在
        if let Some(parent) = output_file.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create output directory")?;
        }
        
        tasks.push(BatchTask::new(input_file.to_path_buf(), output_file, format));
    }
    
    Ok(tasks)
}
