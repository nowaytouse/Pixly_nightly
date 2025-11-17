/**
 *   ==========================================
 * Eagle Adapter - Eagle图库集成适配器
 *   ==========================================
 * 
 * 实现与Eagle图片管理软件的深度集成:
 * - 自动检测Eagle图库结构
 * - 生成预览图和缩略图
 * - 保留Eagle元数据和标签
 * - 支持.info目录结构
 * - 🔥 Phase 40.15: 并发批量处理
 * 
 * Eagle目录结构:
 * - images/xxx.library/
 *   - xxx.png (原图)
 *   - .info/
 *     - xxx.json (元数据)
 *     - xxx_thumbnail.png (缩略图)
 *       ==========================================
 */
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use rayon::prelude::*;  // 🔥 Phase 40.15: 并发处理
use std::sync::atomic::{AtomicUsize, Ordering};  // 🔥 Phase 40.15: 线程安全计数器
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};

/// Eagle图像元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EagleImageMetadata {
    /// 图像ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 文件大小
    pub size: u64,
    /// 创建时间
    #[serde(rename = "btime")]
    pub birth_time: u64,
    /// 修改时间
    #[serde(rename = "mtime")]
    pub modification_time: u64,
    /// 文件扩展名
    pub ext: String,
    /// 标签
    pub tags: Vec<String>,
    /// 所属文件夹
    pub folders: Vec<String>,
    /// 是否删除
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    /// URL
    pub url: String,
    /// 注释
    pub annotation: String,
    /// 高度 (非图片资源可能没有此字段)
    pub height: Option<u32>,
    /// 宽度 (非图片资源可能没有此字段)
    pub width: Option<u32>,
    /// 最后修改时间
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
    /// 调色板
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palettes: Option<Vec<ColorPalette>>,
}

/// 颜色调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// RGB颜色
    pub color: [u8; 3],
    /// 占比
    pub ratio: f32,
}

/// Eagle资源库适配器
pub struct EagleAdapter {
    library_path: PathBuf,
}

impl EagleAdapter {
    /// 创建新的Eagle适配器
    pub fn new<P: AsRef<Path>>(library_path: P) -> Self {
        Self {
            library_path: library_path.as_ref().to_path_buf(),
        }
    }
    
    /// 获取资源库路径
    /// 
    /// 用于验证路径、访问库级配置等
    pub fn get_library_path(&self) -> &Path {
        &self.library_path
    }
    
    /// 获取XMP sidecar文件路径
    /// 
    /// XMP文件命名规则：
    /// - 标准方式：`image.xmp` (去掉原扩展名，直接加.xmp)
    /// - 例如：`photo.jpg` → `photo.xmp`
    /// 
    /// # 参数
    /// - `file_path`: 图像文件路径
    /// 
    /// # 返回
    /// XMP sidecar文件的完整路径
    fn get_xmp_sidecar_path(file_path: &Path) -> PathBuf {
        let mut xmp_path = file_path.to_path_buf();
        // 🔥 修复：直接替换扩展名为.xmp，而不是追加
        // 错误：image.jpg → image.jpg.xmp
        // 正确：image.jpg → image.xmp
        xmp_path.set_extension("xmp");
        xmp_path
    }
    
    /// 验证路径是否在当前资源库内
    /// 
    /// # 参数
    /// - `path`: 待验证的路径
    /// 
    /// # 返回
    /// - `true`: 路径在库内
    /// - `false`: 路径在库外
    pub fn is_path_in_library<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().starts_with(&self.library_path)
    }
    
    /// 解析.info目录
    /// 
    /// # 参数
    /// - `info_dir`: .info目录路径 (如 images/XXXXX.info/)
    /// 
    /// # 返回
    /// - `Ok(EagleImageMetadata)`: 解析成功
    /// - `Err`: 解析失败
    pub fn parse_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<EagleImageMetadata> {
        let info_path = info_dir.as_ref();
        
        // 读取metadata.json
        let metadata_path = info_path.join("metadata.json");
        let metadata_content = fs::read_to_string(&metadata_path)
            .with_context(|| format!("Failed to read metadata: {:?}", metadata_path))?;
        
        let metadata: EagleImageMetadata = serde_json::from_str(&metadata_content)
            .with_context(|| format!("Failed to parse metadata: {:?}", metadata_path))?;
        
        Ok(metadata)
    }
    
    /// 更新.info目录的metadata.json
    /// 
    /// # 参数
    /// - `info_dir`: .info目录路径
    /// - `metadata`: 新的元数据
    /// 
    /// 注意: Eagle使用压缩格式的JSON (minified), 不使用pretty格式
    /// 这样可以减少文件大小，加快读写速度
    pub fn update_metadata<P: AsRef<Path>>(
        &self,
        info_dir: P,
        metadata: &EagleImageMetadata,
    ) -> Result<()> {
        let metadata_path = info_dir.as_ref().join("metadata.json");
        
        // Eagle使用压缩格式的JSON (不使用pretty格式)
        let json = serde_json::to_string(metadata)
            .context("Failed to serialize metadata")?;
        
        fs::write(&metadata_path, json)
            .with_context(|| format!("Failed to write metadata: {:?}", metadata_path))?;
        
        info!("✅ Updated Eagle metadata: {:?}", metadata_path);
        Ok(())
    }
    
    /// 查找.info目录中的原始文件
    /// 
    /// # 参数
    /// - `info_dir`: .info目录路径
    /// 
    /// # 返回
    /// - `Ok(PathBuf)`: 原始文件路径
    /// - `Err`: 未找到原始文件
    pub fn find_original_file<P: AsRef<Path>>(&self, info_dir: P) -> Result<PathBuf> {
        let info_path = info_dir.as_ref();
        
        // 排除特殊文件的模式
        let exclude_patterns = [
            "_thumbnail.",
            "_pixly_viewer_",
            "metadata.json",
            ".DS_Store",
        ];
        
        // 遍历目录查找原始文件
        for entry in fs::read_dir(info_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            // 检查是否是排除的文件
            if exclude_patterns.iter().any(|p| filename.contains(p)) {
                continue;
            }
            
            return Ok(path);
        }
        
        anyhow::bail!("Original file not found in {:?}", info_path)
    }
    
    /// 获取缩略图路径
    /// 
    /// Eagle生成的缩略图命名规则: {original_name}_thumbnail.png
    /// 
    /// 参考: <https://cn.eagle.cool/support/article/why-do-some-images-generate-extra-thumbnails>
    /// 
    /// 注意: 部分图片会额外产生_thumbnail缩略图，这是Eagle的自动优化机制
    /// 用于快速预览和列表显示，避免加载大图
    pub fn get_thumbnail_path<P: AsRef<Path>>(&self, info_dir: P, original_name: &str) -> PathBuf {
        let thumbnail_name = format!("{}_thumbnail.png", 
            Path::new(original_name).file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("thumbnail")
        );
        info_dir.as_ref().join(thumbnail_name)
    }
    
    /// 生成Eagle缩略图
    /// 
    /// 为大图或特定格式生成优化的缩略图
    /// 
    /// # 参数
    /// - `source`: 源图片路径
    /// - `info_dir`: .info目录路径
    /// - `size`: 缩略图尺寸 (默认800px)
    pub fn generate_thumbnail<P: AsRef<Path>>(
        &self,
        source: P,
        info_dir: P,
        size: Option<u32>,
    ) -> Result<PathBuf> {
        use image::GenericImageView;
        
        let source = source.as_ref();
        let info_dir = info_dir.as_ref();
        let size = size.unwrap_or(800);
        
        // 读取源图片
        let img = image::open(source)
            .context("Failed to open source image for thumbnail generation")?;
        
        let (width, height) = img.dimensions();
        
        // 如果图片已经很小，不需要缩略图
        if width <= size && height <= size {
            debug!("📸 Image too small for thumbnail: {}x{}", width, height);
            return Ok(source.to_path_buf());
        }
        
        // 计算缩略图尺寸 (保持宽高比)
        let (thumb_width, thumb_height) = if width > height {
            let ratio = size as f32 / width as f32;
            (size, (height as f32 * ratio) as u32)
        } else {
            let ratio = size as f32 / height as f32;
            ((width as f32 * ratio) as u32, size)
        };
        
        info!("📸 Generating thumbnail: {}x{} -> {}x{}", 
                   width, height, thumb_width, thumb_height);
        
        // 生成缩略图
        let thumbnail = img.thumbnail(thumb_width, thumb_height);
        
        // 保存缩略图
        let original_name = source.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image");
        let thumbnail_path = self.get_thumbnail_path(info_dir, original_name);
        
        thumbnail.save(&thumbnail_path)
            .context("Failed to save thumbnail")?;
        
        info!("✅ Thumbnail generated: {:?}", thumbnail_path);
        Ok(thumbnail_path)
    }
    
    /// 判断是否需要生成缩略图
    /// 
    /// 根据Eagle规则:
    /// - 大图 (>2000px)
    /// - 特定格式 (PSD, AI, etc.)
    /// - 复杂格式需要优化预览
    pub fn should_generate_thumbnail<P: AsRef<Path>>(&self, image_path: P) -> Result<bool> {
        use image::GenericImageView;
        
        let path = image_path.as_ref();
        
        // 检查文件格式
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            
            // 这些格式总是需要缩略图
            if matches!(ext_lower.as_str(), "psd" | "ai" | "svg" | "pdf") {
                return Ok(true);
            }
        }
        
        // 检查图片尺寸
        if let Ok(img) = image::open(path) {
            let (width, height) = img.dimensions();
            
            // 大图需要缩略图
            if width > 2000 || height > 2000 {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// 生成预览图路径
    pub fn generate_preview_path<P: AsRef<Path>>(&self, info_dir: P) -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis();
        
        info_dir.as_ref().join(format!("_pixly_viewer_{}.png", timestamp))
    }
    
    /// 清理旧的预览图
    /// 
    /// 保留最新的N个预览图，删除其他的
    pub fn cleanup_old_previews<P: AsRef<Path>>(
        &self,
        info_dir: P,
        keep_count: usize,
    ) -> Result<usize> {
        let info_path = info_dir.as_ref();
        
        // 查找所有预览图
        let mut previews: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
        
        for entry in fs::read_dir(info_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|n| n.to_str())
                && filename.starts_with("_pixly_viewer_") && filename.ends_with(".png")
                && let Ok(metadata) = fs::metadata(&path)
                && let Ok(modified) = metadata.modified()
            {
                previews.push((path, modified));
            }
        }
        
        // 按修改时间排序 (最新的在前)
        previews.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 删除超过keep_count的预览图
        let mut deleted = 0;
        for (path, _) in previews.iter().skip(keep_count) {
            if let Err(e) = fs::remove_file(path) {
                warn!("⚠️  Failed to delete old preview {:?}: {}", path, e);
            } else {
                deleted += 1;
                debug!("🗑️  Deleted old preview: {:?}", path);
            }
        }
        
        Ok(deleted)
    }
    
    /// 验证.info目录结构
    pub fn validate_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<()> {
        let info_path = info_dir.as_ref();
        
        // 检查目录存在
        if !info_path.exists() {
            anyhow::bail!(".info directory does not exist: {:?}", info_path);
        }
        
        if !info_path.is_dir() {
            anyhow::bail!("Not a directory: {:?}", info_path);
        }
        
        // 检查metadata.json存在
        let metadata_path = info_path.join("metadata.json");
        if !metadata_path.exists() {
            anyhow::bail!("metadata.json not found: {:?}", info_path);
        }
        
        // 尝试解析metadata.json
        self.parse_info_dir(info_path)?;
        
        // 检查原始文件存在
        self.find_original_file(info_path)?;
        
        Ok(())
    }

    /// 🔥 Phase 40.11: 扫描Eagle库中的所有图像
    /// 
    /// 遍历库目录，查找所有包含metadata.json的子目录
    pub fn scan_library(&self) -> Result<Vec<PathBuf>> {
        let mut image_dirs = Vec::new();
        
        let images_dir = self.library_path.join("images");
        if !images_dir.exists() {
            bail!("Eagle library images directory does not exist: {:?}", images_dir);
        }
        
        // 递归扫描images目录
        self.scan_directory(&images_dir, &mut image_dirs)?;
        
        info!("📂 Eagle library scan completed: found {} images", image_dirs.len());
        Ok(image_dirs)
    }
    
    /// 🔥 Phase 40.26: 查找Eagle库中同名的XMP资源
    /// 
    /// Eagle库结构：每个资源（图片/XMP）都有独立的.info目录
    /// images/
    ///   ├── XXXX.info/ (图片资源)
    ///   │   ├── photo.jpg
    ///   │   └── metadata.json (name: "photo", ext: "jpg")
    ///   └── YYYY.info/ (XMP资源)
    ///       ├── photo.xmp
    ///       └── metadata.json (name: "photo", ext: "xmp")
    /// 
    /// 通过metadata.json中的name字段关联同名资源
    /// 
    /// 参数：
    /// - current_info_dir: 当前图片的.info目录
    /// - image_name: 图片的name字段（不含扩展名）
    /// 
    /// 返回：XMP文件的完整路径（如果找到）
    pub fn find_xmp_resource<P: AsRef<Path>>(
        &self,
        current_info_dir: P,
        image_name: &str
    ) -> Option<PathBuf> {
        let current_path = current_info_dir.as_ref();
        
        // 获取images/目录
        let images_dir = current_path.parent()?;
        
        println!("   🔍 Searching for XMP resource with same name in Eagle library: {}", image_name);
        println!("   🔍 Current .info directory: {:?}", current_path);
        println!("   🔍 images directory: {:?}", images_dir);
        
        // 遍历images/目录下的所有.info目录
        let entries = match std::fs::read_dir(images_dir) {
            Ok(e) => e,
            Err(e) => {
                println!("   ⚠️  Cannot read Eagle images directory: {}", e);
                return None;
            }
        };
        
        println!("   🔍 Starting to traverse .info directories in images directory");
        let mut found_count = 0;
        
        for entry in entries.flatten() {
            found_count += 1;
            let path = entry.path();
            
            // 只处理.info目录，跳过当前图片的.info目录
            if !path.is_dir() || !path.file_name()?.to_str()?.ends_with(".info") {
                continue;
            }
            
            if path == current_path {
                continue;  // 跳过自己
            }
            
            // 读取并解析metadata.json
            match self.parse_info_dir(&path) {
                Ok(metadata) => {
                    // 🔥 Phase 40.33: 只在找到匹配时输出日志，减少日志噪音
                    
                    // 检查是否是XMP资源且name匹配
                    if metadata.ext == "xmp" && metadata.name == image_name {
                        // 找到匹配的XMP资源！
                        let xmp_file = path.join(format!("{}.xmp", metadata.name));
                        if xmp_file.exists() {
                            println!("   ✅ Found XMP resource in Eagle library: {}/{}.xmp", 
                                     path.file_name()?.to_str()?, metadata.name);
                            return Some(xmp_file);
                        }
                    }
                },
                Err(e) => {
                    // ✅ 响亮警告，不静默处理
                    warn!("⚠️ Failed to parse Eagle metadata: {:?}, error: {}", path, e);
                    warn!("   This may be a non-standard resource or test file, skipping");
                    continue;
                }
            }
        }
        
        println!("   🔍 Traversal completed: checked {} entries in total", found_count);
        println!("   ℹ️  XMP resource with same name not found in Eagle library");
        None
    }
    
    /// 递归扫描目录
    #[allow(clippy::only_used_in_recursion)]
    fn scan_directory(&self, dir: &Path, image_dirs: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // 检查是否包含metadata.json（Eagle图像目录标志）
                let metadata_path = path.join("metadata.json");
                if metadata_path.exists() {
                    image_dirs.push(path.clone());
                }
                
                // 继续递归扫描子目录
                self.scan_directory(&path, image_dirs)?;
            }
        }
        
        Ok(())
    }
    
    /// 🔥 Phase 40.11: 批量优化Eagle库中的图像
    /// 
    /// 参数：
    /// - format: 目标格式 (avif, webp, jxl等)
    /// - quality: 质量参数
    /// - dry_run: 是否仅测试（不实际转换）
    ///
    /// 批量优化图库（顺序处理）
    pub fn batch_optimize(
        &self,
        format: &str,
        quality: u8,
        dry_run: bool,
    ) -> Result<BatchOptimizeReport> {
        self.batch_optimize_with_concurrency(format, quality, dry_run, 1)
    }
    
    /// 🔥 Phase 40.15: 并发批量优化
    /// 
    /// 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
    /// - 真实的并发处理，不是模拟
    /// - 线程安全的进度跟踪
    /// - 错误不会被吞噬
    /// 
    /// # 参数
    /// - `format`: 目标格式
    /// - `quality`: 质量参数
    /// - `dry_run`: 是否仅模拟
    /// - `max_threads`: 最大线程数（0=自动检测）
    pub fn batch_optimize_with_concurrency(
        &self,
        format: &str,
        quality: u8,
        dry_run: bool,
        max_threads: usize,
    ) -> Result<BatchOptimizeReport> {
        let image_dirs = self.scan_library()?;
        let total = image_dirs.len();
        
        // 配置线程数
        let num_threads = if max_threads == 0 {
            num_cpus::get()
        } else {
            max_threads
        };
        
        info!("🚀 Starting batch optimization of {} images to {} format", total, format.to_uppercase());
        info!("⚙️  Concurrent threads: {}", num_threads);
        if dry_run {
            info!("⚠️  DRY RUN mode: files will not be actually converted");
        }
        
        // 线程安全的计数器（用于实时进度）
        let processed = AtomicUsize::new(0);
        // 注意: 统计数据（failed, saved_bytes）在结果收集阶段计算
        
        // 配置Rayon线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .context("Failed to create thread pool")?;
        
        // 并发处理
        let results: Vec<(PathBuf, std::result::Result<OptimizeResult, anyhow::Error>)> = pool.install(|| {
            image_dirs
                .par_iter()
                .map(|info_dir| {
                    let current = processed.fetch_add(1, Ordering::SeqCst) + 1;
                    let progress = current as f32 / total as f32 * 100.0;
                    
                    if current.is_multiple_of(10) || current == total {
                        info!("📊 Progress: {}/{} ({:.1}%)", current, total, progress);
                    }
                    
                    let result = self.optimize_single_image(info_dir, format, quality, dry_run);
                    (info_dir.clone(), result)
                })
                .collect()
        });
        
        // 收集统计数据和错误
        let mut errors = Vec::new();
        let mut total_processed = 0;
        let mut total_skipped = 0;
        let mut total_failed = 0;
        let mut total_original = 0u64;
        let mut total_optimized = 0u64;
        let mut total_saved = 0u64;
        
        for (path, result) in results {
            match result {
                Ok(opt_result) => {
                    total_processed += 1;
                    total_original += opt_result.original_size;
                    total_optimized += opt_result.optimized_size;
                    total_saved += opt_result.saved_bytes;
                    
                    if opt_result.skipped {
                        total_skipped += 1;
                    }
                }
                Err(e) => {
                    total_failed += 1;
                    errors.push(OptimizeError {
                        path,
                        error: e.to_string(),
                    });
                }
            }
        }
        
        // 计算压缩率
        let compression_ratio = if total_original > 0 {
            ((total_original - total_optimized) as f64 / total_original as f64) * 100.0
        } else {
            0.0
        };
        
        // 输出详细报告
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📊 Batch Optimization Completion Report");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📁 Total files: {}", total);
        info!("✅ Successfully processed: {}", total_processed);
        info!("⏭️  Skipped: {}", total_skipped);
        info!("❌ Failed: {}", total_failed);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("📦 Original total size: {:.2} MB", total_original as f64 / 1024.0 / 1024.0);
        info!("📦 Optimized size: {:.2} MB", total_optimized as f64 / 1024.0 / 1024.0);
        info!("💾 Space saved: {:.2} MB", total_saved as f64 / 1024.0 / 1024.0);
        info!("📊 Compression ratio: {:.2}%", compression_ratio);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        Ok(BatchOptimizeReport {
            total,
            processed: total_processed,
            skipped: total_skipped,
            failed: total_failed,
            total_original_size: total_original,
            total_optimized_size: total_optimized,
            saved_bytes: total_saved,
            compression_ratio,
            errors,
        })
    }
    
    /// 优化单个图像
    fn optimize_single_image(
        &self,
        info_dir: &Path,
        format: &str,
        _quality: u8,
        dry_run: bool,
    ) -> Result<OptimizeResult> {
        // 1. 查找原始文件
        let original_file = self.find_original_file(info_dir)?;
        let original_size = std::fs::metadata(&original_file)?.len();
        
        // 2. 检查是否需要转换
        let current_format = original_file
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if current_format == format {
            debug!("⏭️  Skipping: already in {} format", format.to_uppercase());
            return Ok(OptimizeResult {
                original_size,
                optimized_size: original_size,
                saved_bytes: 0,
                skipped: true,
            });
        }
        
        if dry_run {
            info!("🔍 DRY RUN: Will convert {:?} ({} → {})", 
                      original_file.file_name().unwrap_or_default(),
                      current_format.to_uppercase(),
                      format.to_uppercase());
            return Ok(OptimizeResult {
                original_size,
                optimized_size: 0,  // Dry run 不实际转换
                saved_bytes: 0,
                skipped: false,
            });
        }
        
        // 3. 执行转换（简化实现 - 使用image crate）
        // 创建临时输出路径
        let output_file = original_file.with_extension(format);
        
        info!("🔄 Converting: {:?} → {:?}", 
                  original_file.file_name().unwrap_or_default(),
                  output_file.file_name().unwrap_or_default());
        
        // 简化的图像转换
        let img = image::open(&original_file)
            .context("Failed to open image")?;
        
        img.save(&output_file)
            .context("Failed to save converted image")?;
        
        // 获取转换后的文件大小
        let optimized_size = std::fs::metadata(&output_file)?.len();
        let saved_bytes = original_size.saturating_sub(optimized_size);
        
        // 4. 更新Eagle元数据
        // 读取并更新metadata.json中的文件信息
        if let Ok(mut metadata) = self.parse_info_dir(info_dir) {
            // 更新文件扩展名
            if let Some(name) = metadata.name.strip_suffix(&format!(".{}", current_format)) {
                metadata.name = format!("{}.{}", name, format);
            }
            
            // 更新文件大小
            metadata.size = optimized_size;
            
            // 写回metadata.json
            if let Err(e) = self.update_metadata(info_dir, &metadata) {
                warn!("⚠️  Failed to update Eagle metadata: {}", e);
            }
        }
        
        // 🔥 4.5. 处理XMP sidecar文件（如果存在）
        // Eagle的XMP处理：合并到目标文件，然后删除sidecar
        let original_xmp = Self::get_xmp_sidecar_path(&original_file);
        
        if original_xmp.exists() {
            info!("📄 Found XMP sidecar: {:?}", original_xmp.file_name());
            
            // 使用exiftool将XMP合并到转换后的文件
            use std::process::Command;
            let merge_result = Command::new("exiftool")
                .arg("-tagsFromFile")
                .arg(&original_xmp)
                .arg("-XMP:all")
                .arg("-overwrite_original")
                .arg(&output_file)
                .output();
            
            match merge_result {
                Ok(output) if output.status.success() => {
                    info!("✅ XMP merged into target file: {:?}", output_file.file_name());
                    
                    // 验证合并成功后删除原始XMP
                    if let Err(e) = std::fs::remove_file(&original_xmp) {
                        warn!("⚠️  Failed to delete original XMP: {}", e);
                    } else {
                        info!("🗑️  Original XMP sidecar deleted");
                    }
                },
                Ok(output) => {
                    error!("❌ XMP merge failed: {}", String::from_utf8_lossy(&output.stderr));
                    warn!("⚠️  Keeping original XMP sidecar: {:?}", original_xmp);
                },
                Err(e) => {
                    error!("❌ Cannot execute exiftool: {}", e);
                    warn!("💡 Please install exiftool: brew install exiftool");
                    warn!("⚠️  Keeping original XMP sidecar: {:?}", original_xmp);
                }
            }
        }
        
        // 5. 删除原始文件（已被新格式替换，XMP已处理）
        // 🔥 重要：只有当输入和输出文件不同时才删除原始文件
        // 防止同格式转换时误删输出文件（如 JXL → JXL）
        if original_file != output_file {
            if let Err(e) = std::fs::remove_file(&original_file) {
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                error!("❌ [CRITICAL] Failed to delete original file!");
                error!("📁 File: {:?}", original_file);
                error!("💥 Error: {}", e);
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                error!("⚠️  Original file may still exist, please check manually");
            } else {
                info!("🗑️  Original file deleted: {:?}", original_file.file_name());
            }
        } else {
            info!("ℹ️  Input and output have same name, skipping original file deletion");
        }
        
        info!("✅ Conversion successful: {} MB → {} MB, saved {:.2} MB", 
                  original_size as f64 / 1024.0 / 1024.0,
                  optimized_size as f64 / 1024.0 / 1024.0,
                  saved_bytes as f64 / 1024.0 / 1024.0);
        
        Ok(OptimizeResult {
            original_size,
            optimized_size,
            saved_bytes,
            skipped: false,
        })
    }
}

/// 批量优化报告
#[derive(Debug)]
pub struct BatchOptimizeReport {
    /// 总图像数
    pub total: usize,
    /// 成功处理数
    pub processed: usize,
    /// 跳过数（已是目标格式）
    pub skipped: usize,
    /// 失败数
    pub failed: usize,
    /// 原始总大小（字节）
    pub total_original_size: u64,
    /// 优化后总大小（字节）
    pub total_optimized_size: u64,
    /// 节省的总字节数
    pub saved_bytes: u64,
    /// 压缩率（百分比）
    pub compression_ratio: f64,
    /// 错误列表
    pub errors: Vec<OptimizeError>,
}

/// 优化错误
#[derive(Debug)]
pub struct OptimizeError {
    /// 路径
    pub path: PathBuf,
    /// 错误信息
    pub error: String,
}

/// 单个优化结果
#[derive(Debug)]
struct OptimizeResult {
    /// 原始文件大小（字节）
    original_size: u64,
    /// 优化后文件大小（字节）
    optimized_size: u64,
    /// 节省的字节数
    saved_bytes: u64,
    /// 是否跳过处理
    skipped: bool,
}
