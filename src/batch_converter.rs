/// 🔥 批量转换系统 - Phase 2
/// 
/// 架构原则：
/// - 完整实现单文件、多文件、目录处理
/// - 使用统一日志系统（无硬编码println）
/// - 并行处理支持
/// - 完整的错误处理和进度跟踪
/// - 支持重试策略

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use anyhow::{Context, Result};
use rayon::prelude::*;

use crate::conversion_core::{execute_conversion, ConversionConfig};
use crate::file_collector::FileCollector;
use crate::log_manager::{LogManager, LogLevel};
use crate::progress_tracker::{ProgressTracker, ProgressLevel};

/// 批量转换结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub skipped: usize,
    pub errors: Vec<(PathBuf, String)>,
    pub total_time: Duration,
    pub successful_files: Vec<PathBuf>,
}

impl BatchResult {
    pub fn new() -> Self {
        Self {
            total: 0,
            success: 0,
            failed: 0,
            skipped: 0,
            errors: Vec::new(),
            total_time: Duration::from_secs(0),
            successful_files: Vec::new(),
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.success as f64 / self.total as f64
        }
    }
    
    pub fn log_summary(&self) {
        let logger = LogManager::global();
        
        logger.header("Batch Conversion Summary");
        logger.log(LogLevel::Info, &format!("Total files: {}", self.total));
        logger.log(LogLevel::Info, &format!("✅ Success: {}", self.success));
        
        if self.failed > 0 {
            logger.log(LogLevel::Warning, &format!("❌ Failed: {}", self.failed));
        }
        
        if self.skipped > 0 {
            logger.log(LogLevel::Info, &format!("⏭️  Skipped: {}", self.skipped));
        }
        
        logger.log(LogLevel::Info, &format!("Success rate: {:.1}%", self.success_rate() * 100.0));
        logger.log(LogLevel::Info, &format!("Total time: {:.2}s", self.total_time.as_secs_f64()));
        
        if !self.errors.is_empty() {
            logger.separator();
            logger.log(LogLevel::Warning, "Failed files:");
            for (path, error) in &self.errors {
                logger.log(LogLevel::Error, &format!("  {} - {}", path.display(), error));
            }
        }
    }
}

impl Default for BatchResult {
    fn default() -> Self {
        Self::new()
    }
}

/// 错误处理策略
#[derive(Debug, Clone)]
pub enum ErrorStrategy {
    /// 遇到第一个错误就停止
    StopOnError,
    /// 继续处理，收集所有错误
    ContinueOnError,
    /// 重试失败的转换
    RetryOnError { max_retries: usize },
}

/// 批量转换器配置
#[derive(Debug, Clone)]
pub struct BatchConverterConfig {
    /// 转换配置
    pub conversion_config: ConversionConfig,
    /// 最大并行任务数
    pub max_parallel: usize,
    /// 错误处理策略
    pub error_strategy: ErrorStrategy,
    /// 是否覆盖已存在的文件
    pub overwrite: bool,
    /// 是否显示进度
    pub show_progress: bool,
}

impl Default for BatchConverterConfig {
    fn default() -> Self {
        Self {
            conversion_config: ConversionConfig::default(),
            max_parallel: num_cpus::get(),
            error_strategy: ErrorStrategy::ContinueOnError,
            overwrite: false,
            show_progress: true,
        }
    }
}

/// 批量转换器
pub struct BatchConverter {
    config: BatchConverterConfig,
    logger: &'static LogManager,
}

impl BatchConverter {
    /// 创建新的批量转换器
    pub fn new(config: BatchConverterConfig) -> Self {
        Self {
            config,
            logger: LogManager::global(),
        }
    }
    
    /// 转换单个文件
    pub fn convert_single(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
    ) -> Result<()> {
        self.logger.log(LogLevel::Info, &format!("Converting: {}", input.display()));
        self.logger.log(LogLevel::Verbose, &format!("  Output: {}", output.display()));
        self.logger.log(LogLevel::Verbose, &format!("  Format: {}", format));
        
        // 检查输入文件
        if !input.exists() {
            anyhow::bail!("Input file does not exist: {}", input.display());
        }
        
        // 检查输出文件
        if output.exists() && !self.config.overwrite {
            self.logger.log(LogLevel::Warning, "Output file already exists (use --overwrite to replace)");
            return Ok(());
        }
        
        // 执行转换
        let start = Instant::now();
        execute_conversion(input, output, format, &self.config.conversion_config)
            .with_context(|| format!("Failed to convert {}", input.display()))?;
        
        let elapsed = start.elapsed();
        self.logger.log(LogLevel::Info, &format!("✅ Completed in {:.2}s", elapsed.as_secs_f64()));
        
        Ok(())
    }
    
    /// 转换多个文件到输出目录
    pub fn convert_batch(
        &self,
        inputs: Vec<PathBuf>,
        output_dir: &Path,
        format: &str,
    ) -> Result<BatchResult> {
        if inputs.is_empty() {
            self.logger.log(LogLevel::Warning, "No files to convert");
            return Ok(BatchResult::new());
        }
        
        self.logger.header(&format!("Batch Conversion - {} files", inputs.len()));
        self.logger.log(LogLevel::Info, &format!("Output directory: {}", output_dir.display()));
        self.logger.log(LogLevel::Info, &format!("Target format: {}", format));
        self.logger.log(LogLevel::Info, &format!("Parallel jobs: {}", self.config.max_parallel));
        
        // 创建输出目录
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;
        
        let start_time = Instant::now();
        let total = inputs.len();
        
        // 设置线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.max_parallel)
            .build()
            .context("Failed to create thread pool")?;
        
        // 进度跟踪
        let progress = if self.config.show_progress {
            Some(Arc::new(ProgressTracker::new(
                ProgressLevel::Batch,
                total as u64,
                format!("Converting {} files", total),
            )))
        } else {
            None
        };
        
        // 共享状态
        let completed = Arc::new(Mutex::new(0usize));
        let errors = Arc::new(Mutex::new(Vec::new()));
        let successful_files = Arc::new(Mutex::new(Vec::new()));
        let skipped = Arc::new(Mutex::new(0usize));
        
        // 并行处理
        let results: Vec<_> = pool.install(|| {
            inputs
                .par_iter()
                .map(|input| {
                    let output_path = self.determine_output_path(input, output_dir, format);
                    
                    // 检查是否跳过
                    if output_path.exists() && !self.config.overwrite {
                        *skipped.lock().unwrap() += 1;
                        self.update_progress(&completed, total, &progress);
                        return Ok(());
                    }
                    
                    // 执行转换
                    let result = execute_conversion(
                        input,
                        &output_path,
                        format,
                        &self.config.conversion_config,
                    );
                    
                    // 更新进度
                    self.update_progress(&completed, total, &progress);
                    
                    // 处理结果
                    match result {
                        Ok(_) => {
                            successful_files.lock().unwrap().push(output_path.clone());
                            self.logger.log(LogLevel::Verbose, &format!("✅ {}", input.file_name().unwrap().to_string_lossy()));
                            Ok(())
                        }
                        Err(e) => {
                            let error_msg = format!("{:#}", e);
                            errors.lock().unwrap().push((input.clone(), error_msg.clone()));
                            self.logger.log(LogLevel::Error, &format!("❌ {} - {}", input.file_name().unwrap().to_string_lossy(), error_msg));
                            
                            // 检查错误策略
                            if matches!(self.config.error_strategy, ErrorStrategy::StopOnError) {
                                Err(e)
                            } else {
                                Ok(())
                            }
                        }
                    }
                })
                .collect()
        });
        
        // 汇总结果
        let total_time = start_time.elapsed();
        let errors = Arc::try_unwrap(errors).unwrap().into_inner().unwrap();
        let successful_files = Arc::try_unwrap(successful_files).unwrap().into_inner().unwrap();
        let skipped_count = *skipped.lock().unwrap();
        
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        let failed_count = errors.len();
        
        // 检查是否应该停止
        if matches!(self.config.error_strategy, ErrorStrategy::StopOnError) && !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Conversion stopped on first error: {}",
                errors[0].1
            ));
        }
        
        let result = BatchResult {
            total,
            success: success_count,
            failed: failed_count,
            skipped: skipped_count,
            errors,
            total_time,
            successful_files,
        };
        
        result.log_summary();
        
        Ok(result)
    }
    
    /// 转换目录（递归）
    pub fn convert_directory(
        &self,
        input_dir: &Path,
        output_dir: &Path,
        format: &str,
        extensions: Vec<String>,
        recursive: bool,
    ) -> Result<BatchResult> {
        self.logger.log(LogLevel::Info, &format!("Scanning directory: {}", input_dir.display()));
        
        // 收集文件
        let collector = FileCollector::new(extensions)
            .recursive(recursive);
        
        let files = collector.collect(input_dir)
            .context("Failed to collect files")?;
        
        self.logger.log(LogLevel::Info, &format!("Found {} files", files.len()));
        
        // 批量转换
        self.convert_batch(files, output_dir, format)
    }
    
    /// 带重试的转换
    pub fn convert_with_retry(
        &self,
        inputs: Vec<PathBuf>,
        output_dir: &Path,
        format: &str,
        max_retries: usize,
    ) -> Result<BatchResult> {
        let mut remaining_inputs = inputs.clone();
        let mut all_successful = Vec::new();
        let mut final_errors = Vec::new();
        let start_time = Instant::now();
        
        for attempt in 0..=max_retries {
            if remaining_inputs.is_empty() {
                break;
            }
            
            self.logger.log(
                LogLevel::Info,
                &format!("Attempt {} of {}, {} files remaining", 
                    attempt + 1, max_retries + 1, remaining_inputs.len())
            );
            
            let result = self.convert_batch(remaining_inputs.clone(), output_dir, format)?;
            
            // 收集成功的文件
            all_successful.extend(result.successful_files);
            
            // 准备重试失败的文件
            remaining_inputs = result.errors.iter()
                .map(|(path, _)| path.clone())
                .collect();
            
            // 如果是最后一次尝试，保存错误
            if attempt == max_retries {
                final_errors = result.errors;
            }
        }
        
        let total_time = start_time.elapsed();
        let original_total = inputs.len();
        let final_success = all_successful.len();
        let final_failed = final_errors.len();
        
        let result = BatchResult {
            total: original_total,
            success: final_success,
            failed: final_failed,
            skipped: 0,
            errors: final_errors,
            total_time,
            successful_files: all_successful,
        };
        
        result.log_summary();
        
        Ok(result)
    }
    
    /// 确定输出路径
    fn determine_output_path(
        &self,
        input: &Path,
        output_dir: &Path,
        format: &str,
    ) -> PathBuf {
        let filename = input.file_stem().unwrap_or_default();
        output_dir.join(format!("{}.{}", filename.to_string_lossy(), format))
    }
    
    /// 更新进度
    fn update_progress(
        &self,
        completed: &Arc<Mutex<usize>>,
        total: usize,
        progress: &Option<Arc<ProgressTracker>>,
    ) {
        let mut count = completed.lock().unwrap();
        *count += 1;
        let current = *count;
        drop(count);
        
        if let Some(tracker) = progress {
            tracker.set_completed(current as u64);
            
            if self.config.show_progress {
                let info = tracker.get_info();
                self.logger.log(
                    LogLevel::Info,
                    &format!("[{}/{}] {:.1}% complete", 
                        current, total, info.progress)
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::new();
        result.total = 10;
        result.success = 8;
        result.failed = 2;
        
        assert_eq!(result.success_rate(), 0.8);
    }
    
    #[test]
    fn test_determine_output_path() {
        let config = BatchConverterConfig::default();
        let converter = BatchConverter::new(config);
        
        let input = Path::new("/path/to/input.jpg");
        let output_dir = Path::new("/output");
        let format = "webp";
        
        let result = converter.determine_output_path(input, output_dir, format);
        assert_eq!(result, Path::new("/output/input.webp"));
    }
    
    #[test]
    fn test_batch_converter_creation() {
        let config = BatchConverterConfig {
            max_parallel: 2,
            error_strategy: ErrorStrategy::ContinueOnError,
            ..Default::default()
        };
        
        let converter = BatchConverter::new(config);
        assert_eq!(converter.config.max_parallel, 2);
    }
}
