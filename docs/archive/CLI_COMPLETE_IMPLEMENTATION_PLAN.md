# 🎯 CLI完整实现计划

**状态**: 📋 待实现  
**优先级**: P0 - 核心功能  
**符合**: PROJECT_QUALITY_MANIFESTO.md v3.0.0

---

## 当前问题

### 现有CLI的局限性
```rust
// ❌ 当前实现：只支持单文件
Convert {
    input: PathBuf,  // 单个文件路径
    output: Option<PathBuf>,
    // ...
}
```

**缺失功能**：
1. ❌ 批量文件处理
2. ❌ 目录递归处理
3. ❌ 通配符支持
4. ❌ 进度报告
5. ❌ 并发控制
6. ❌ 错误恢复

---

## 完整实现方案

### 1. CLI参数设计

```rust
#[derive(Subcommand)]
enum Commands {
    /// Convert single file
    Convert {
        /// Input file
        input: PathBuf,
        
        /// Output file or directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        // ... format-specific params ...
    },
    
    /// Convert multiple files
    ConvertBatch {
        /// Input files (supports wildcards)
        #[arg(required = true)]
        inputs: Vec<PathBuf>,
        
        /// Output directory
        #[arg(short, long, required = true)]
        output_dir: PathBuf,
        
        /// Parallel jobs
        #[arg(short, long, default_value = "4")]
        jobs: usize,
        
        // ... format-specific params ...
    },
    
    /// Convert directory recursively
    ConvertDir {
        /// Input directory
        input_dir: PathBuf,
        
        /// Output directory
        #[arg(short, long, required = true)]
        output_dir: PathBuf,
        
        /// Recursive
        #[arg(short, long)]
        recursive: bool,
        
        /// File pattern (e.g., "*.jpg")
        #[arg(short, long)]
        pattern: Option<String>,
        
        /// Parallel jobs
        #[arg(short, long, default_value = "4")]
        jobs: usize,
        
        // ... format-specific params ...
    },
}
```

### 2. 文件收集模块

```rust
/// File collection with filtering
pub struct FileCollector {
    patterns: Vec<String>,
    recursive: bool,
    max_depth: Option<usize>,
}

impl FileCollector {
    /// Collect files from directory
    pub fn collect_from_dir(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if self.recursive {
            self.collect_recursive(dir, 0, &mut files)?;
        } else {
            self.collect_flat(dir, &mut files)?;
        }
        
        Ok(files)
    }
    
    /// Collect with glob pattern
    pub fn collect_with_glob(&self, pattern: &str) -> Result<Vec<PathBuf>> {
        // Use glob crate
        glob::glob(pattern)?
            .filter_map(Result::ok)
            .collect()
    }
    
    /// Filter by extension
    fn matches_pattern(&self, path: &Path) -> bool {
        // Check against patterns
    }
}
```

### 3. 批量处理模块

```rust
/// Batch conversion with progress tracking
pub struct BatchConverter {
    config: ConversionConfig,
    max_parallel: usize,
    progress_callback: Option<Box<dyn Fn(usize, usize)>>,
}

impl BatchConverter {
    /// Convert multiple files
    pub fn convert_batch(
        &self,
        inputs: Vec<PathBuf>,
        output_dir: &Path,
        format: &str,
    ) -> Result<BatchResult> {
        // Create output directory
        std::fs::create_dir_all(output_dir)?;
        
        let total = inputs.len();
        let mut results = Vec::new();
        
        // Use rayon for parallel processing
        use rayon::prelude::*;
        
        let results: Vec<_> = inputs
            .par_iter()
            .enumerate()
            .map(|(i, input)| {
                // Report progress
                if let Some(cb) = &self.progress_callback {
                    cb(i + 1, total);
                }
                
                // Convert file
                let output = self.determine_output_path(input, output_dir, format);
                execute_conversion(input, &output, format, &self.config)
            })
            .collect();
        
        // Aggregate results
        Ok(BatchResult::from_results(results))
    }
    
    /// Determine output path
    fn determine_output_path(
        &self,
        input: &Path,
        output_dir: &Path,
        format: &str,
    ) -> PathBuf {
        let filename = input.file_stem().unwrap();
        output_dir.join(format!("{}.{}", filename.to_string_lossy(), format))
    }
}

/// Batch conversion result
pub struct BatchResult {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub errors: Vec<(PathBuf, String)>,
    pub total_time: Duration,
}
```

### 4. 进度报告

```rust
/// Progress reporter
pub trait ProgressReporter {
    fn report(&self, current: usize, total: usize, file: &Path);
    fn complete(&self, result: &BatchResult);
}

/// Console progress reporter
pub struct ConsoleProgress {
    start_time: Instant,
}

impl ProgressReporter for ConsoleProgress {
    fn report(&self, current: usize, total: usize, file: &Path) {
        let percent = (current as f64 / total as f64 * 100.0) as u8;
        let elapsed = self.start_time.elapsed();
        let eta = if current > 0 {
            elapsed / current as u32 * (total - current) as u32
        } else {
            Duration::from_secs(0)
        };
        
        println!(
            "[{}/{}] {}% - {} - ETA: {:?}",
            current,
            total,
            percent,
            file.display(),
            eta
        );
    }
    
    fn complete(&self, result: &BatchResult) {
        println!("\n✅ Batch conversion complete!");
        println!("   Total: {}", result.total);
        println!("   Success: {}", result.success);
        println!("   Failed: {}", result.failed);
        println!("   Time: {:?}", result.total_time);
        
        if !result.errors.is_empty() {
            println!("\n❌ Errors:");
            for (file, error) in &result.errors {
                println!("   {}: {}", file.display(), error);
            }
        }
    }
}
```

### 5. 错误处理

```rust
/// Error handling strategy
pub enum ErrorStrategy {
    /// Stop on first error
    StopOnError,
    
    /// Continue on error, collect all errors
    ContinueOnError,
    
    /// Retry failed conversions
    RetryOnError { max_retries: usize },
}

impl BatchConverter {
    /// Convert with error handling
    pub fn convert_with_error_handling(
        &self,
        inputs: Vec<PathBuf>,
        output_dir: &Path,
        format: &str,
        strategy: ErrorStrategy,
    ) -> Result<BatchResult> {
        match strategy {
            ErrorStrategy::StopOnError => {
                // Stop on first error
                self.convert_batch_stop_on_error(inputs, output_dir, format)
            }
            ErrorStrategy::ContinueOnError => {
                // Continue, collect errors
                self.convert_batch(inputs, output_dir, format)
            }
            ErrorStrategy::RetryOnError { max_retries } => {
                // Retry failed conversions
                self.convert_batch_with_retry(inputs, output_dir, format, max_retries)
            }
        }
    }
}
```

---

## 实现步骤

### Phase 1: 文件收集 (1-2小时)
- [ ] 实现FileCollector
- [ ] 支持递归目录遍历
- [ ] 支持glob模式
- [ ] 支持扩展名过滤
- [ ] 单元测试

### Phase 2: 批量处理 (2-3小时)
- [ ] 实现BatchConverter
- [ ] 并发处理（rayon）
- [ ] 输出路径管理
- [ ] 结果聚合
- [ ] 单元测试

### Phase 3: 进度报告 (1小时)
- [ ] 实现ProgressReporter trait
- [ ] ConsoleProgress实现
- [ ] ETA计算
- [ ] 美化输出

### Phase 4: CLI集成 (1-2小时)
- [ ] 添加ConvertBatch命令
- [ ] 添加ConvertDir命令
- [ ] 参数验证
- [ ] 帮助文档

### Phase 5: 错误处理 (1-2小时)
- [ ] 实现ErrorStrategy
- [ ] 错误收集
- [ ] 重试逻辑
- [ ] 错误报告

### Phase 6: 测试 (2-3小时)
- [ ] 单文件转换测试
- [ ] 批量转换测试
- [ ] 目录递归测试
- [ ] 错误处理测试
- [ ] 性能测试

---

## 使用示例

### 单文件转换
```bash
pixly-converter convert input.png --format jxl --quality 90
```

### 批量转换
```bash
pixly-converter convert-batch *.png --output-dir ./output --format jxl --quality 90 --jobs 8
```

### 目录递归转换
```bash
pixly-converter convert-dir ./photos --output-dir ./converted --recursive --pattern "*.jpg" --format avif --quality 85
```

### 通配符支持
```bash
pixly-converter convert-batch "photos/**/*.{jpg,png}" --output-dir ./output --format webp
```

---

## 依赖添加

```toml
[dependencies]
glob = "0.3"
rayon = "1.10"  # 已有
indicatif = "0.17"  # 进度条（可选）
```

---

## 质量保证

### 遵循质量宣言
- ✅ 失败响亮报错（不静默fallback）
- ✅ 真实错误信息
- ✅ 无硬编码规则
- ✅ 完整测试覆盖

### 性能目标
- 并发处理：4-8个文件同时
- 内存控制：不超过2GB
- 进度更新：每秒1次

---

## 下一步行动

1. **立即开始**: Phase 1 - 文件收集
2. **预计完成**: 10-15小时工作量
3. **里程碑**: 完整的批量处理CLI

---

**创建时间**: 2024-11-17  
**预计完成**: 2024-11-18  
**负责人**: 待分配
