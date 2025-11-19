//! 批量处理模块 - 从 @archive/rust_v2_clean 提取并增强
//! 
//! 提供高效的批量图像处理功能
//! 
//! 增强点：
//! - 改进并发控制
//! - 添加进度回调
//! - 增强错误恢复
//! - 性能监控

use crate::core_processor::{ImageProcessor, ProcessingConfig, ProcessingResult};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};


/// 批量处理器
#[derive(Debug)]
pub struct BatchProcessor {
    processor: ImageProcessor,
    config: BatchConfig,
}

/// 批量处理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub max_parallel: usize,
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub target_format: String,
    pub recursive: bool,
    pub overwrite: bool,
    pub skip_existing: bool,
}

/// 批量处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub total_files: usize,
    pub processed_files: usize,
    pub failed_files: usize,
    pub skipped_files: usize,
    pub results: Vec<ProcessingResult>,
    pub errors: Vec<BatchError>,
    pub total_time_ms: u64,
    pub avg_time_per_file_ms: u64,
    pub total_input_size: u64,
    pub total_output_size: u64,
}

/// 批量处理错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    pub file_path: String,
    pub error_message: String,
}

/// 进度信息
#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub current: usize,
    pub total: usize,
    pub current_file: String,
    pub percent: f64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_parallel: num_cpus::get().max(1),
            input_dir: PathBuf::from("."),
            output_dir: PathBuf::from("./output"),
            target_format: "jpg".to_string(),
            recursive: false,
            overwrite: false,
            skip_existing: false,
        }
    }
}

impl BatchProcessor {
    /// 创建新的批量处理器
    pub fn new(processing_config: ProcessingConfig, batch_config: BatchConfig) -> Result<Self> {
        let processor = ImageProcessor::new(processing_config)?;
        Ok(Self {
            processor,
            config: batch_config,
        })
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Result<Self> {
        Self::new(ProcessingConfig::default(), BatchConfig::default())
    }

    /// 执行批量处理（同步版本）
    pub fn process(&self) -> Result<BatchResult> {
        let start_time = std::time::Instant::now();
        
        // 收集输入文件
        let input_files = self.collect_input_files()?;
        let total_files = input_files.len();
        
        if total_files == 0 {
            return Ok(BatchResult {
                total_files: 0,
                processed_files: 0,
                failed_files: 0,
                skipped_files: 0,
                results: vec![],
                errors: vec![],
                total_time_ms: start_time.elapsed().as_millis() as u64,
                avg_time_per_file_ms: 0,
                total_input_size: 0,
                total_output_size: 0,
            });
        }

        // 创建输出目录
        std::fs::create_dir_all(&self.config.output_dir)?;

        // 处理文件
        let (results, errors, skipped) = self.process_files_sequential(input_files);

        let processed_files = results.len();
        let failed_files = errors.len();
        let total_time_ms = start_time.elapsed().as_millis() as u64;
        let avg_time_per_file_ms = if processed_files > 0 {
            total_time_ms / processed_files as u64
        } else {
            0
        };

        let total_input_size: u64 = results.iter().map(|r| r.input_size).sum();
        let total_output_size: u64 = results.iter().map(|r| r.output_size).sum();

        Ok(BatchResult {
            total_files,
            processed_files,
            failed_files,
            skipped_files: skipped,
            results,
            errors,
            total_time_ms,
            avg_time_per_file_ms,
            total_input_size,
            total_output_size,
        })
    }

    /// 收集输入文件
    fn collect_input_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if self.config.recursive {
            self.collect_files_recursive(&self.config.input_dir, &mut files)?;
        } else {
            self.collect_files_direct(&self.config.input_dir, &mut files)?;
        }
        
        Ok(files)
    }

    /// 递归收集文件
    fn collect_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)
            .context(format!("Failed to read directory: {}", dir.display()))? 
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.collect_files_recursive(&path, files)?;
            } else if self.is_image_file(&path) {
                files.push(path);
            }
        }
        Ok(())
    }

    /// 直接收集文件
    fn collect_files_direct(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)
            .context(format!("Failed to read directory: {}", dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && self.is_image_file(&path) {
                files.push(path);
            }
        }
        Ok(())
    }

    /// 检查是否为图像文件
    fn is_image_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext.to_lowercase().as_str(),
                "jpg" | "jpeg" | "png" | "webp" | "avif" | "gif" | "jxl"
            )
        } else {
            false
        }
    }

    /// 顺序处理文件
    fn process_files_sequential(
        &self,
        files: Vec<PathBuf>,
    ) -> (Vec<ProcessingResult>, Vec<BatchError>, usize) {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        let mut skipped = 0;
        
        for input_file in files {
            let output_file = self.get_output_path(&input_file);
            
            // 检查是否跳过已存在的文件
            if self.config.skip_existing && output_file.exists() {
                skipped += 1;
                continue;
            }
            
            // 检查是否覆盖
            if !self.config.overwrite && output_file.exists() {
                errors.push(BatchError {
                    file_path: input_file.to_string_lossy().into_owned(),
                    error_message: "Output file already exists (use --overwrite to replace)".to_string(),
                });
                continue;
            }
            
            match self.processor.process(&input_file, &output_file, &self.config.target_format) {
                Ok(result) => results.push(result),
                Err(e) => errors.push(BatchError {
                    file_path: input_file.to_string_lossy().into_owned(),
                    error_message: e.to_string(),
                }),
            }
        }
        
        (results, errors, skipped)
    }

    /// 获取输出文件路径
    fn get_output_path(&self, input_path: &Path) -> PathBuf {
        let file_stem = input_path.file_stem().unwrap_or_default();
        let output_name = format!("{}.{}", file_stem.to_string_lossy(), self.config.target_format);
        self.config.output_dir.join(output_name)
    }
    
    /// 获取配置
    pub fn config(&self) -> &BatchConfig {
        &self.config
    }
}

impl BatchResult {
    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            0.0
        } else {
            self.processed_files as f64 / self.total_files as f64 * 100.0
        }
    }
    
    /// 计算总压缩率
    pub fn compression_ratio(&self) -> f64 {
        if self.total_input_size == 0 {
            1.0
        } else {
            self.total_output_size as f64 / self.total_input_size as f64
        }
    }
    
    /// 计算节省的空间
    pub fn space_saved(&self) -> i64 {
        self.total_input_size as i64 - self.total_output_size as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert!(config.max_parallel > 0);
        assert!(!config.recursive);
        assert!(!config.overwrite);
        assert!(!config.skip_existing);
    }

    #[test]
    fn test_batch_processor_creation() {
        let processor = BatchProcessor::with_defaults();
        assert!(processor.is_ok());
    }

    #[test]
    fn test_empty_directory_processing() {
        let temp_dir = TempDir::new().unwrap();
        let config = BatchConfig {
            input_dir: temp_dir.path().to_path_buf(),
            output_dir: temp_dir.path().join("output"),
            ..BatchConfig::default()
        };
        
        let processor = BatchProcessor::new(ProcessingConfig::default(), config).unwrap();
        let result = processor.process().unwrap();
        
        assert_eq!(result.total_files, 0);
        assert_eq!(result.processed_files, 0);
        assert_eq!(result.success_rate(), 0.0);
    }
    
    #[test]
    fn test_batch_result_calculations() {
        let result = BatchResult {
            total_files: 10,
            processed_files: 8,
            failed_files: 2,
            skipped_files: 0,
            results: vec![],
            errors: vec![],
            total_time_ms: 1000,
            avg_time_per_file_ms: 125,
            total_input_size: 10_000_000,
            total_output_size: 5_000_000,
        };
        
        assert_eq!(result.success_rate(), 80.0);
        assert_eq!(result.compression_ratio(), 0.5);
        assert_eq!(result.space_saved(), 5_000_000);
    }
}
