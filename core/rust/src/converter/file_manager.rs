/**
 * File Manager - 文件管理核心模块
 * 
 * 🔥 Rust作为唯一文件处理内核的扩展功能
 * 
 * 功能：
 * - 文件移动/复制/重命名
 * - 目录扫描
 * - 文件搜索
 * - 批量操作
 * - 安全删除
 * 
 * @module file_manager
 * @version 1.0.0
 */
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};

use anyhow::{Result, Context, bail};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 文件操作类型
#[derive(Debug, Clone, Copy)]
pub enum FileOperation {
    Move,
    Copy,
    Rename,
    Delete,
}

/// 文件搜索选项
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SearchOptions {
    pub pattern: String,
    pub case_sensitive: bool,
    pub max_depth: Option<usize>,
    pub file_types: Vec<String>,
}

/// 文件管理器
pub struct FileManager;

impl FileManager {
    /// 移动文件
    /// 
    /// # 参数
    /// - `source`: 源文件路径
    /// - `dest`: 目标路径
    /// - `overwrite`: 是否覆盖已存在的文件
    pub fn move_file<P: AsRef<Path>>(source: P, dest: P, overwrite: bool) -> Result<()> {
        let source = source.as_ref();
        let dest = dest.as_ref();
        
        // 检查源文件是否存在
        if !source.exists() {
            error!("❌ Source file not found: {:?}", source);
            bail!("Source file does not exist: {:?}", source);
        }
        
        // 检查目标文件是否已存在
        if dest.exists() && !overwrite {
            error!("❌ Destination file already exists: {:?}", dest);
            bail!("Destination file already exists (use overwrite=true to force): {:?}", dest);
        }
        
        // 确保目标目录存在
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create destination directory: {:?}", parent))?;
        }
        
        // 执行移动
        fs::rename(source, dest)
            .with_context(|| format!("Failed to move file from {:?} to {:?}", source, dest))?;
        
        info!("✅ File moved: {:?} → {:?}", source, dest);
        Ok(())
    }
    
    /// 复制文件
    /// 
    /// # 参数
    /// - `source`: 源文件路径
    /// - `dest`: 目标路径
    /// - `overwrite`: 是否覆盖已存在的文件
    pub fn copy_file<P: AsRef<Path>>(source: P, dest: P, overwrite: bool) -> Result<()> {
        let source = source.as_ref();
        let dest = dest.as_ref();
        
        // 检查源文件是否存在
        if !source.exists() {
            error!("❌ Source file not found: {:?}", source);
            bail!("Source file does not exist: {:?}", source);
        }
        
        // 检查目标文件是否已存在
        if dest.exists() && !overwrite {
            error!("❌ Destination file already exists: {:?}", dest);
            bail!("Destination file already exists (use overwrite=true to force): {:?}", dest);
        }
        
        // 确保目标目录存在
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create destination directory: {:?}", parent))?;
        }
        
        // 执行复制
        fs::copy(source, dest)
            .with_context(|| format!("Failed to copy file from {:?} to {:?}", source, dest))?;
        
        info!("✅ File copied: {:?} → {:?}", source, dest);
        Ok(())
    }
    
    /// 重命名文件
    /// 
    /// # 参数
    /// - `path`: 文件路径
    /// - `new_name`: 新文件名（不含路径）
    pub fn rename_file<P: AsRef<Path>, S: AsRef<str>>(path: P, new_name: S) -> Result<PathBuf> {
        let path = path.as_ref();
        let new_name = new_name.as_ref();
        
        // 检查文件是否存在
        if !path.exists() {
            error!("❌ File not found: {:?}", path);
            bail!("File does not exist: {:?}", path);
        }
        
        // 构造新路径
        let parent = path.parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine parent directory"))?;
        let new_path = parent.join(new_name);
        
        // 检查新文件名是否已存在
        if new_path.exists() {
            error!("❌ Target name already exists: {:?}", new_path);
            bail!("Target name already exists: {:?}", new_path);
        }
        
        // 执行重命名
        fs::rename(path, &new_path)
            .with_context(|| format!("Failed to rename {:?} to {:?}", path, new_path))?;
        
        info!("✅ File renamed: {:?} → {}", path, new_name);
        Ok(new_path)
    }
    
    /// 安全删除文件
    /// 
    /// # 参数
    /// - `path`: 文件路径
    /// - `confirm`: 确认删除（必须为true）
    pub fn delete_file<P: AsRef<Path>>(path: P, confirm: bool) -> Result<()> {
        let path = path.as_ref();
        
        if !confirm {
            warn!("⚠️ Delete operation cancelled: confirmation required");
            bail!("Delete operation requires explicit confirmation (confirm=true)");
        }
        
        // 检查文件是否存在
        if !path.exists() {
            warn!("⚠️ File does not exist: {:?}", path);
            return Ok(()); // 已经不存在，视为成功
        }
        
        // 执行删除
        fs::remove_file(path)
            .with_context(|| format!("Failed to delete file: {:?}", path))?;
        
        info!("✅ File deleted: {:?}", path);
        Ok(())
    }
    
    /// 扫描目录
    /// 
    /// # 参数
    /// - `dir`: 目录路径
    /// - `max_depth`: 最大递归深度（None表示无限制）
    /// - `include_dirs`: 是否包含目录
    pub fn scan_directory<P: AsRef<Path>>(
        dir: P,
        max_depth: Option<usize>,
        include_dirs: bool,
    ) -> Result<Vec<PathBuf>> {
        let dir = dir.as_ref();
        
        if !dir.exists() {
            error!("❌ Directory not found: {:?}", dir);
            bail!("Directory does not exist: {:?}", dir);
        }
        
        if !dir.is_dir() {
            error!("❌ Path is not a directory: {:?}", dir);
            bail!("Path is not a directory: {:?}", dir);
        }
        
        let mut walker = WalkDir::new(dir);
        if let Some(depth) = max_depth {
            walker = walker.max_depth(depth);
        }
        
        let mut results = Vec::new();
        for entry in walker {
            let entry = entry.with_context(|| "Failed to read directory entry")?;
            let path = entry.path().to_path_buf();
            
            // 跳过根目录
            if path == dir {
                continue;
            }
            
            // 根据选项过滤
            if !include_dirs && path.is_dir() {
                continue;
            }
            
            results.push(path);
        }
        
        info!("✅ Scanned directory: {:?} ({} items)", dir, results.len());
        Ok(results)
    }
    
    /// 搜索文件
    /// 
    /// # 参数
    /// - `dir`: 搜索目录
    /// - `options`: 搜索选项
    pub fn search_files<P: AsRef<Path>>(
        dir: P,
        options: &SearchOptions,
    ) -> Result<Vec<PathBuf>> {
        let dir = dir.as_ref();
        
        if !dir.exists() {
            bail!("Directory does not exist: {:?}", dir);
        }
        
        if !dir.is_dir() {
            bail!("Path is not a directory: {:?}", dir);
        }
        
        let mut walker = WalkDir::new(dir);
        if let Some(depth) = options.max_depth {
            walker = walker.max_depth(depth);
        }
        
        // 🚀 优化：单次遍历，直接过滤
        let pattern_lower = if !options.case_sensitive {
            Some(options.pattern.to_lowercase())
        } else {
            None
        };
        
        let results: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                
                // 跳过根目录
                if path == dir {
                    return false;
                }
                
                // 只要文件
                if path.is_dir() {
                    return false;
                }
                
                // 检查文件类型
                if !options.file_types.is_empty() {
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        let ext_lower = ext.to_lowercase();
                        if !options.file_types.iter().any(|t| t.to_lowercase() == ext_lower) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // 检查文件名模式
                if !options.pattern.is_empty() {
                    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                        let matches = if options.case_sensitive {
                            filename.contains(&options.pattern)
                        } else {
                            let filename_lower = filename.to_lowercase();
                            if let Some(ref pattern) = pattern_lower {
                                filename_lower.contains(pattern)
                            } else {
                                false
                            }
                        };
                        
                        if !matches {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                true
            })
            .map(|entry| entry.path().to_path_buf())
            .collect();
        
        info!(
            "✅ Search complete: {:?} (pattern={}, found={})",
            dir,
            options.pattern,
            results.len()
        );
        
        Ok(results)
    }
    
    /// 批量操作
    /// 
    /// # 参数
    /// - `files`: 文件列表
    /// - `operation`: 操作类型
    /// - `dest_dir`: 目标目录（用于Move/Copy）
    pub fn batch_operation<P: AsRef<Path>>(
        files: &[PathBuf],
        operation: FileOperation,
        dest_dir: Option<P>,
    ) -> Result<Vec<PathBuf>> {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for (idx, file) in files.iter().enumerate() {
            info!("📦 Processing [{}/{}]: {:?}", idx + 1, files.len(), file);
            
            let result = match operation {
                FileOperation::Move | FileOperation::Copy => {
                    let dest_dir = dest_dir.as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Destination directory required for Move/Copy"))?;
                    
                    let filename = file.file_name()
                        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
                    let dest = dest_dir.as_ref().join(filename);
                    
                    match operation {
                        FileOperation::Move => Self::move_file(file, &dest, false),
                        FileOperation::Copy => Self::copy_file(file, &dest, false),
                        _ => unreachable!(),
                    }
                    .map(|_| dest)
                }
                FileOperation::Delete => {
                    Self::delete_file(file, true)?;
                    Ok(file.clone())
                }
                FileOperation::Rename => {
                    bail!("Rename operation requires individual file handling");
                }
            };
            
            match result {
                Ok(path) => results.push(path),
                Err(e) => {
                    error!("❌ Failed to process {:?}: {}", file, e);
                    errors.push((file.clone(), e));
                }
            }
        }
        
        if !errors.is_empty() {
            error!(
                "❌ Batch operation completed with {} errors out of {} files",
                errors.len(),
                files.len()
            );
            bail!(
                "Batch operation had {} errors. First error: {}",
                errors.len(),
                errors[0].1
            );
        }
        
        info!("✅ Batch operation complete: {} files processed", results.len());
        Ok(results)
    }
}

/// Eagle路径解析和自动修复
pub struct EaglePathResolver;

impl EaglePathResolver {
    /// 解析Eagle文件路径，自动修复常见问题
    /// 
    /// # 功能
    /// 1. 从Eagle metadata构建路径
    /// 2. 自动修复metadata.json中的错误name字段
    /// 3. 处理特殊字符和URL编码
    /// 4. 智能路径修复（重复扩展名、特殊字符替换）
    /// 
    /// # 参数
    /// - `library_path`: Eagle资源库路径
    /// - `item_id`: 文件ID
    /// - `name`: 文件名（不含扩展名）
    /// - `ext`: 文件扩展名
    /// 
    /// # 返回
    /// 解析后的有效文件路径
    pub fn resolve_eagle_path<P: AsRef<Path>>(
        library_path: P,
        item_id: &str,
        name: &str,
        ext: &str,
    ) -> Result<PathBuf> {
        let library_path = library_path.as_ref();
        
        // 1. 自动修复Eagle metadata.json中的错误name字段
        let corrected_name = if name.ends_with(&format!(".{}", ext)) {
            warn!("⚠️ Detected corrupted metadata: name={} contains ext={}", name, ext);
            let correct_name = &name[..name.len() - ext.len() - 1];
            
            // 尝试修复metadata.json
            let metadata_path = library_path.join("images")
                .join(format!("{}.info", item_id))
                .join("metadata.json");
            
            if let Err(e) = Self::fix_metadata_name(&metadata_path, correct_name) {
                error!("❌ Failed to auto-fix metadata: {}", e);
            } else {
                info!("✅ Auto-fixed metadata: {} → {}", name, correct_name);
            }
            
            correct_name.to_string()
        } else {
            name.to_string()
        };
        
        // 2. 构建Eagle标准路径
        let mut path = library_path.join("images")
            .join(format!("{}.info", item_id))
            .join(format!("{}.{}", corrected_name, ext));
        
        // 3. 快速检查：路径存在直接返回
        if path.exists() {
            debug!("✅ File exists: {}", path.display());
            return Ok(path);
        }
        
        warn!("⚠️ File not found: {}, trying fixes...", path.display());
        
        // 4. 尝试修复路径
        let dir = path.parent().context("No parent directory")?.to_path_buf();
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .context("Invalid file name")?
            .to_string();
        
        // 修复1: 去除重复扩展名 (gif.gif → gif)
        let fixed_name = if let Some(base) = file_name.strip_suffix(&format!(".{}.{}", ext, ext)) {
            info!("🔧 Removed duplicate extension");
            format!("{}.{}", base, ext)
        } else {
            file_name
        };
        
        // 修复2: Eagle特殊字符替换规则
        let fixed_name = fixed_name
            .replace("(", "_")
            .replace(")", "");
        
        path = dir.join(&fixed_name);
        
        if path.exists() {
            info!("✅ Fixed path found: {}", path.display());
            return Ok(path);
        }
        
        // 修复3: 模糊匹配（列出目录中的文件）
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let entry_name = entry.file_name();
                let entry_str = entry_name.to_string_lossy();
                
                // 不区分大小写匹配
                if entry_str.eq_ignore_ascii_case(&format!("{}.{}", corrected_name, ext)) {
                    info!("✅ Found by fuzzy match: {}", entry_str);
                    return Ok(entry.path());
                }
            }
        }
        
        // 所有修复尝试都失败
        bail!("All path resolution attempts failed for: {}.{}", corrected_name, ext)
    }
    
    /// 修复Eagle metadata.json中的name字段
    fn fix_metadata_name(metadata_path: &Path, correct_name: &str) -> Result<()> {
        let content = fs::read_to_string(metadata_path)
            .context("Failed to read metadata.json")?;
        
        let mut metadata: serde_json::Value = serde_json::from_str(&content)
            .context("Failed to parse metadata.json")?;
        
        if let Some(obj) = metadata.as_object_mut() {
            obj.insert("name".to_string(), serde_json::Value::String(correct_name.to_string()));
            obj.insert("lastModified".to_string(), serde_json::Value::Number(
                serde_json::Number::from(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_millis() as u64)
            ));
        }
        
        // 使用Eagle默认格式：2空格缩进
        let fixed_content = serde_json::to_string_pretty(&metadata)?;
        fs::write(metadata_path, fixed_content)
            .context("Failed to write metadata.json")?;
        
        Ok(())
    }
}

/// 文件类型图标映射
pub struct FileIconMapper;

impl FileIconMapper {
    /// 根据文件扩展名获取对应的图标
    /// 
    /// # 参数
    /// - `ext`: 文件扩展名（不含.）
    /// 
    /// # 返回
    /// Unicode图标字符串
    pub fn get_icon(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
            // 图片格式
            "jpg" | "jpeg" | "jpe" | "jfif" | "jfi" => "🖼️",
            "png" => "🎨",
            "gif" => "🎞️",
            "webp" => "🌐",
            "avif" => "🎬",
            "jxl" => "📦",
            "heic" | "heif" => "📱",
            "bmp" => "🖌️",
            "tiff" | "tif" => "📄",
            
            // 视频格式
            "mp4" | "m4v" => "🎥",
            "mov" => "🎬",
            "avi" => "📹",
            "mkv" => "🎞️",
            "webm" => "🌐",
            
            // 默认
            _ => "📄",
        }
    }
}
