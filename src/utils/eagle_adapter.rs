/**
 * ==========================================
 * Eagle Adapter - Eagle图库integrated适配器
 * ==========================================
 * 
 * implementation与Eagle图片管理软件深度integrated:
 * - auto检测Eagle图库structure
 * - generatepreview图 and 缩略图
 * - 保留Eagle元data and 标签
 * - support.info目录structure
 * - 🔥 Phase 40.15: and发批量处理
 * 
 * Eagle目录structure:
 * - images/xxx.library/
 * - xxx.png (原图)
 * - .info/
 * - xxx.json (元data)
 * - xxx_thumbnail.png (缩略图)
 * ==========================================
 */
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use rayon::prelude::*; // 🔥 Phase 40.15: and发处理
use std::sync::atomic::{AtomicUsize, Ordering}; // 🔥 Phase 40.15: 线程安全计数器
// 🔧 UnifiedloggingSystem
use tracing::{info, warn, error, debug};

/// Eagleimage元data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure EagleImageMetadata {
 /// imageID
 pub id: String,
 /// name
 pub name: String,
 /// filesize
 pub size: u64,
 /// createtime
 #[serde(rename = "btime")]
 pub birth_time: u64,
 /// modifiedtime
 #[serde(rename = "mtime")]
 pub modification_time: u64,
 /// file扩展名
 pub ext: String,
 /// label
 pub tags: Vec<String>,
 /// 所属file夹
 pub folders: Vec<String>,
 /// is否delete
 #[serde(rename = "isDeleted")]
 pub is_deleted: bool,
 /// URL
 pub url: String,
 /// comment
 pub annotation: String,
 /// height (not图片resourcemay没 has 此field)
 pub height: Option<u32>,
 /// width (not图片resourcemay没 has 此field)
 pub width: Option<u32>,
 /// 最 after modifiedtime
 #[serde(rename = "lastModified")]
 pub last_modified: u64,
 /// palette
 #[serde(skip_serializing_if = "Option::is_none")]
 pub palettes: Option<Vec<ColorPalette>>,
}

/// colorpalette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure ColorPalette {
 /// RGBcolor
 pub color: [u8; 3],
 /// 占比
 pub ratio: f32,
}

/// Eagleresourcelibraryadapter
pub structure EagleAdapter {
 library_path: PathBuf,
}

impl EagleAdapter {
 /// createnewEagleadapter
 pub fn new<P: AsRef<Path>>(library_path: P) -> Self {
 Self {
 library_path: library_path.as_ref().to_path_buf(),
 }
 }
 
 /// getresourcelibrarypath
 /// 
 /// forvalidationpath、访问library级configuration etc 
 pub fn get_library_path(&self) -> &Path {
 &self.library_path
 }
 
 /// getXMP sidecarfilepath
 /// 
 /// XMPfile命名规则：
 /// - standard方式：`image.xmp` (去掉原扩展名，直接加.xmp)
 /// - 例如：`photo.jpg` → `photo.xmp`
 /// 
 /// # parameter
 /// - `file_path`: imagefilepath
 /// 
 /// # return
 /// XMP sidecarfilefullpath
 fn get_xmp_sidecar_path(file_path: &Path) -> PathBuf {
 let mut xmp_path = file_path.to_path_buf();
 // 🔥 fixed：直接replace扩展名for.xmp，而 not isappend
 // error：image.jpg → image.jpg.xmp
 // 正确：image.jpg → image.xmp
 xmp_path.set_extension("xmp");
 xmp_path
 }
 
 /// validationpathis否atcurrentresourcelibrary内
 /// 
 /// # parameter
 /// - `path`: 待validationpath
 /// 
 /// # return
 /// - `true`: pathatlibrary内
 /// - `false`: pathatlibrary外
 pub fn is_path_in_library<P: AsRef<Path>>(&self, path: P) -> bool {
 path.as_ref().starts_with(&self.library_path)
 }
 
 /// parse.infodirectory
 /// 
 /// # parameter
 /// - `info_dir`: .infodirectorypath (如 images/XXXXX.info/)
 /// 
 /// # return
 /// - `Ok(EagleImageMetadata)`: parsesuccess
 /// - `Err`: parsefailure
 pub fn parse_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<EagleImageMetadata> {
 let info_path = info_dir.as_ref();
 
 // readmetadata.json
 let metadata_path = info_path.join("metadata.json");
 let metadata_content = fs::read_to_string(&metadata_path)
 .with_context(|| format!("Failed to read metadata: {:?}", metadata_path))?;
 
 let metadata: EagleImageMetadata = serde_json::from_str(&metadata_content)
 .with_context(|| format!("Failed to parse metadata: {:?}", metadata_path))?;
 
 Ok(metadata)
 }
 
 /// update.infodirectorymetadata.json
 /// 
 /// # parameter
 /// - `info_dir`: .infodirectorypath
 /// - `metadata`: new元data
 /// 
 /// note: Eagleusecompressionformat JSON (minified), not useprettyformat
 /// this样canreducefilesize，加快读写speed
 pub fn update_metadata<P: AsRef<Path>>(
 &self,
 info_dir: P,
 metadata: &EagleImageMetadata,
 ) -> Result<()> {
 let metadata_path = info_dir.as_ref().join("metadata.json");
 
 // Eagleusecompressionformat JSON ( not useprettyformat)
 let json = serde_json::to_string(metadata)
 .context("Failed to serialize metadata")?;

 fs::write(&metadata_path, json)
 .with_context(|| format!("Failed to write metadata: {:?}", metadata_path))?;

 info!("Updated Eagle metadata: {:?}", metadata_path);
 Ok(())
 }
 
 /// find.infodirectoryoriginalfile
 /// 
 /// # parameter
 /// - `info_dir`: .infodirectorypath
 /// 
 /// # return
 /// - `Ok(Path Buf)`: originalfilepath
 /// - `Err`: 未findtooriginalfile
 pub fn find_original_file<P: AsRef<Path>>(&self, info_dir: P) -> Result<PathBuf> {
 let info_path = info_dir.as_ref();
 
 // 排除特殊filemode
 let exclude_patterns = [
 "_thumbnail.",
 "_pixly_viewer_",
 "metadata.json",
 ".DS_Store",
 ];
 
 // 遍历directoryfindoriginalfile
 for entry in fs::read_dir(info_path)? {
 let entry = entry?;
 let path = entry.path();
 
 if !path.is_file() {
 continue;
 }
 
 let filename = path.file_name()
 .and_then(|n| n.to_str())
 .unwrap_or("");
 
 // checkis否is排除file
 if exclude_patterns.iter().any(|p| filename.contains(p)) {
 continue;
 }
 
 return Ok(path);
 }
 
 anyhow::bail!("Original file not found in {:?}", info_path)
 }
 
 /// get缩略图path
 /// 
 /// Eaglegenerate缩略图命名规则: {original_name}_thumbnail.png
 /// 
 /// 参考: <https://cn.eagle.cool/support/article/why-do-some-images-generate-extra-thumbnails>
 /// 
 /// note: 部分图片 will 额外产生_thumbnail缩略图，thisis Eagleautooptimization机制
 /// forquickpreview and listdisplay，避免load大图
 pub fn get_thumbnail_path<P: AsRef<Path>>(&self, info_dir: P, original_name: &str) -> PathBuf {
 let thumbnail_name = format!("{}_thumbnail.png", 
 Path::new(original_name).file_stem()
 .and_then(|s| s.to_str())
 .unwrap_or("thumbnail")
 );
 info_dir.as_ref().join(thumbnail_name)
 }
 
 /// generateEagle缩略图
 /// 
 /// for大图orspecificformatgenerateoptimization缩略图
 /// 
 /// # parameter
 /// - `source`: source图片path
 /// - `info_dir`: .infodirectorypath
 /// - `size`: 缩略图dimension (default800px)
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
 
 // readsource图片
 let img = image::open(source)
 .context("Failed to open source image for thumbnail generation")?;
 
 let (width, height) = img.dimensions();

 // If image is already small, no thumbnail needed
 if width <= size && height <= size {
 debug!("Image too small for thumbnail: {}x{}", width, height);
 return Ok(source.to_path_buf());
 }
 
 // calculation缩略图dimension (keep宽high比)
 let (thumb_width, thumb_height) = if width > height {
 let ratio = size as f32 / width as f32;
 (size, (height as f32 * ratio) as u32)
 } else {
 let ratio = size as f32 / height as f32;
 ((width as f32 * ratio) as u32, size)
 };

 info!("Generating thumbnail: {}x{} -> {}x{}",
 width, height, thumb_width, thumb_height);

 // Generate thumbnail
 let thumbnail = img.thumbnail(thumb_width, thumb_height);
 
 // save缩略图
 let original_name = source.file_name()
 .and_then(|n| n.to_str())
 .unwrap_or("image");
 let thumbnail_path = self.get_thumbnail_path(info_dir, original_name);
 
 thumbnail.save(&thumbnail_path)
 .context("Failed to save thumbnail")?;

 info!("Thumbnail generated: {:?}", thumbnail_path);
 Ok(thumbnail_path)
 }
 
 /// 判断is否needgenerate缩略图
 /// 
 /// based on Eagle规则:
 /// - 大图 (>2000px)
 /// - specificformat (PSD, AI, etc.)
 /// - 复杂formatneedoptimizationpreview
 pub fn should_generate_thumbnail<P: AsRef<Path>>(&self, image_path: P) -> Result<bool> {
 use image::GenericImageView;
 
 let path = image_path.as_ref();
 
 // checkfileformat
 if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
 let ext_lower = ext.to_lowercase();
 
 // this些format总isneed缩略图
 if matches!(ext_lower.as_str(), "psd" | "ai" | "svg" | "pdf") {
 return Ok(true);
 }
 }
 
 // check图片dimension
 if let Ok(img) = image::open(path) {
 let (width, height) = img.dimensions();
 
 // 大图need缩略图
 if width > 2000 || height > 2000 {
 return Ok(true);
 }
 }
 
 Ok(false)
 }
 
 /// generatepreview图path
 pub fn generate_preview_path<P: AsRef<Path>>(&self, info_dir: P) -> PathBuf {
 let timestamp = std::time::SystemTime::now()
 .duration_since(std::time::UNIX_EPOCH)
 .expect("System time before UNIX epoch")
 .as_millis();
 
 info_dir.as_ref().join(format!("_pixly_viewer_{}.png", timestamp))
 }
 
 /// cleanup旧preview图
 /// 
 /// 保留latest Npreview图，delete其他
 pub fn cleanup_old_previews<P: AsRef<Path>>(
 &self,
 info_dir: P,
 keep_count: usize,
 ) -> Result<usize> {
 let info_path = info_dir.as_ref();
 
 // find所 has preview图
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
 
 // 按modifiedtimesort (latestat before )
 previews.sort_by(|a, b| b.1.cmp(&a.1));
 
 // deleteexceedskeep_countpreview图
 let mut deleted = 0;
 for (path, _) in previews.iter().skip(keep_count) {
 if let Err(e) = fs::remove_file(path) {
 warn!("Failed to delete old preview {:?}: {}", path, e);
 } else {
 deleted += 1;
 debug!("Deleted old preview: {:?}", path);
 }
 }
 
 Ok(deleted)
 }
 
 /// validation.infodirectorystructure
 pub fn validate_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<()> {
 let info_path = info_dir.as_ref();
 
 // checkdirectoryexists
 if !info_path.exists() {
 anyhow::bail!(".info directory does not exist: {:?}", info_path);
 }
 
 if !info_path.is_dir() {
 anyhow::bail!("Not a directory: {:?}", info_path);
 }
 
 // checkmetadata.jsonexists
 let metadata_path = info_path.join("metadata.json");
 if !metadata_path.exists() {
 anyhow::bail!("metadata.json not found: {:?}", info_path);
 }
 
 // tryparsemetadata.json
 self.parse_info_dir(info_path)?;
 
 // checkoriginalfileexists
 self.find_original_file(info_path)?;
 
 Ok(())
 }

 /// 🔥 Phase 40.11: 扫描Eaglelibrary所 has image
 /// 
 /// 遍历librarydirectory，find所 has containsmetadata.json子directory
 pub fn scan_library(&self) -> Result<Vec<PathBuf>> {
 let mut image_dirs = Vec::new();
 
 let images_dir = self.library_path.join("images");
 if !images_dir.exists() {
 bail!("Eagle library images directory does not exist: {:?}", images_dir);
 }

 // Recursively scan images directory
 self.scan_directory(&images_dir, &mut image_dirs)?;

 info!("Eagle library scan completed: found {} images", image_dirs.len());
 Ok(image_dirs)
 }
 
 /// 🔥 Phase 40.26: find Eaglelibrary同名XMPresource
 /// 
 /// Eaglelibrarystructure：everyresource（图片/XMP）都 has 独立.infodirectory
 /// images/
 /// ├── XXXX.info/ (图片resource)
 /// │ ├── photo.jpg
 /// │ └── metadata.json (name: "photo", ext: "jpg")
 /// └── YYYY.info/ (XMPresource)
 /// ├── photo.xmp
 /// └── metadata.json (name: "photo", ext: "xmp")
 /// 
 /// passmetadata.jsonnamefield关联同名resource
 /// 
 /// parameter：
 /// - current_info_dir: current图片.infodirectory
 /// - image_name: 图片namefield（ not 含扩展名）
 /// 
 /// return：XMPfilefullpath（iffindto）
 pub fn find_xmp_resource<P: AsRef<Path>>(
 &self,
 current_info_dir: P,
 image_name: &str
 ) -> Option<PathBuf> {
 let current_path = current_info_dir.as_ref();
 
 // getimages/directory
 let images_dir = current_path.parent()?;
 
 log::debug!(" 🔍 Searching for XMP resource with same name in Eagle library: {}", image_name);
 log::debug!(" 🔍 Current .info directory: {:?}", current_path);
 log::debug!(" 🔍 images directory: {:?}", images_dir);
 
 // 遍历images/directory下所 has .infodirectory
 let entries = match std::fs::read_dir(images_dir) {
 Ok(e) => e,
 Err(e) => {
 log::debug!(" ⚠️ Cannot read Eagle images directory: {}", e);
 return None;
 }
 };
 
 log::debug!(" 🔍 Starting to traverse .info directories in images directory");
 let mut found_count = 0;
 
 for entry in entries.flatten() {
 found_count += 1;
 let path = entry.path();
 
 // 只processing.infodirectory，skipcurrent图片.infodirectory
 if !path.is_dir() || !path.file_name()?.to_str()?.ends_with(".info") {
 continue;
 }
 
 if path == current_path {
 continue; // 跳过自己
 }
 
 // readandparsemetadata.json
 match self.parse_info_dir(&path) {
 Ok(metadata) => {
 // 🔥 Phase 40.33: 只atfindtomatch when outputlogging，reducelogging噪音
 
 // Check if XMP resource with matching name
 if metadata.ext == "xmp" && metadata.name == image_name {
 // Found matching XMP resource!
 let xmp_file = path.join(format!("{}.xmp", metadata.name));
 if xmp_file.exists() {
 log::debug!(" Found XMP resource in Eagle library: {}/{}.xmp",
 path.file_name()?.to_str()?, metadata.name);
 return Some(xmp_file);
 }
 }
 },
 Err(e) => {
 // Loud warning, not silent
 warn!("Failed to parse Eagle metadata: {:?}, error: {}", path, e);
 warn!(" This may be a non-standard resource or test file, skipping");
 continue;
 }
 }
 }

 log::debug!(" Traversal completed: checked {} entries in total", found_count);
 log::debug!(" XMP resource with same name not found in Eagle library");
 None
 }
 
 /// recursion扫描directory
 #[allow(clippy::only_used_in_recursion)]
 fn scan_directory(&self, dir: &Path, image_dirs: &mut Vec<PathBuf>) -> Result<()> {
 for entry in std::fs::read_dir(dir)? {
 let entry = entry?;
 let path = entry.path();
 
 if path.is_dir() {
 // checkis否containsmetadata.json（Eagleimagedirectory标志）
 let metadata_path = path.join("metadata.json");
 if metadata_path.exists() {
 image_dirs.push(path.clone());
 }
 
 // continuerecursion扫描子directory
 self.scan_directory(&path, image_dirs)?;
 }
 }
 
 Ok(())
 }
 
 /// 🔥 Phase 40.11: batchoptimization Eaglelibraryimage
 /// 
 /// parameter：
 /// - format: targetformat (avif, webp, jxl etc )
 /// - quality: qualityparameter
 /// - dry_run: is否仅test（ not actualconversion）
 ///
 /// batchoptimization图library（sequentialprocessing）
 pub fn batch_optimize(
 &self,
 format: &str,
 quality: u8,
 dry_run: bool,
 ) -> Result<BatchOptimizeReport> {
 self.batch_optimize_with_concurrency(format, quality, dry_run, 1)
 }
 
 /// 🔥 Phase 40.15: concurrentbatchoptimization
 /// 
 /// 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
 /// - realconcurrentprocessing， not issimulated
 /// - threadsecurityprogress跟踪
 /// - error not will be 吞噬
 /// 
 /// # parameter
 /// - `format`: targetformat
 /// - `quality`: qualityparameter
 /// - `dry_run`: is否仅simulated
 /// - `max_threads`: maximumthread数（0=autodetection）
 pub fn batch_optimize_with_concurrency(
 &self,
 format: &str,
 quality: u8,
 dry_run: bool,
 max_threads: usize,
 ) -> Result<BatchOptimizeReport> {
 let image_dirs = self.scan_library()?;
 let total = image_dirs.len();
 
 // configurationthread数
 let num_threads = if max_threads == 0 {
 num_cpus::get()
 } else {
 max_threads
 };

 info!("Starting batch optimization of {} images to {} format", total, format.to_uppercase());
 info!("Concurrent threads: {}", num_threads);
 if dry_run {
 info!("DRY RUN mode: files will not be actually converted");
 }
 
 // threadsecurity计数（forreal-timeprogress）
 let processed = AtomicUsize::new(0);
 // note: statisticsdata（failed, saved_bytes）atresult收集阶段calculation
 
 // configuration Rayonthread池
 let pool = rayon::ThreadPoolBuilder::new()
 .num_threads(num_threads)
 .build()
 .context("Failed to create thread pool")?;
 
 // concurrentprocessing
 let results: Vec<(PathBuf, std::result::Result<OptimizeResult, anyhow::Error>)> = pool.install(|| {
 image_dirs
 .par_iter()
 .map(|info_dir| {
 let current = processed.fetch_add(1, Ordering::SeqCst) + 1;
 let progress = current as f32 / total as f32 * 100.0;

 if current.is_multiple_of(10) || current == total {
 info!("Progress: {}/{} ({:.1}%)", current, total, progress);
 }
 
 let result = self.optimize_single_image(info_dir, format, quality, dry_run);
 (info_dir.clone(), result)
 })
 .collect()
 });
 
 // 收集statisticsdata and error
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
 
 // calculationcompression率
 let compression_ratio = if total_original > 0 {
 ((total_original - total_optimized) as f64 / total_original as f64) * 100.0
 } else {
 0.0
 };

 // Output detailed report
 info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 info!("Batch Optimization Completion Report");
 info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 info!("Total files: {}", total);
 info!("Successfully processed: {}", total_processed);
 info!("Skipped: {}", total_skipped);
 info!("Failed: {}", total_failed);
 info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 info!("Original total size: {:.2} MB", total_original as f64 / 1024.0 / 1024.0);
 info!("Optimized size: {:.2} MB", total_optimized as f64 / 1024.0 / 1024.0);
 info!("Space saved: {:.2} MB", total_saved as f64 / 1024.0 / 1024.0);
 info!("Compression ratio: {:.2}%", compression_ratio);
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
 
 /// optimizationsingleimage
 fn optimize_single_image(
 &self,
 info_dir: &Path,
 format: &str,
 _quality: u8,
 dry_run: bool,
 ) -> Result<OptimizeResult> {
 // 1. findoriginalfile
 let original_file = self.find_original_file(info_dir)?;
 let original_size = std::fs::metadata(&original_file)?.len();
 
 // 2. checkis否needconversion
 let current_format = original_file
 .extension()
 .and_then(|s| s.to_str())
 .unwrap_or("")
 .to_lowercase();

 if current_format == format {
 debug!("Skipping: already in {} format", format.to_uppercase());
 return Ok(OptimizeResult {
 original_size,
 optimized_size: original_size,
 saved_bytes: 0,
 skipped: true,
 });
 }

 if dry_run {
 info!("DRY RUN: Will convert {:?} ({} → {})",
 original_file.file_name().unwrap_or_default(),
 current_format.to_uppercase(),
 format.to_uppercase());
 return Ok(OptimizeResult {
 original_size,
 optimized_size: 0, // Dry run does not actually convert
 saved_bytes: 0,
 skipped: false,
 });
 }

 // 3. Execute conversion (simplified implementation using image crate)
 // Create temporary output path
 let output_file = original_file.with_extension(format);

 info!("Converting: {:?} → {:?}",
 original_file.file_name().unwrap_or_default(),
 output_file.file_name().unwrap_or_default());

 // Simplified image conversion
 let img = image::open(&original_file)
 .context("Failed to open image")?;

 img.save(&output_file)
 .context("Failed to save converted image")?;

 // Get converted file size
 let optimized_size = std::fs::metadata(&output_file)?.len();
 let saved_bytes = original_size.saturating_sub(optimized_size);

 // 4. Update Eagle metadata
 // Read and update file info in metadata.json
 if let Ok(mut metadata) = self.parse_info_dir(info_dir) {
 // Update file extension
 if let Some(name) = metadata.name.strip_suffix(&format!(".{}", current_format)) {
 metadata.name = format!("{}.{}", name, format);
 }

 // Update file size
 metadata.size = optimized_size;

 // Write back metadata.json
 if let Err(e) = self.update_metadata(info_dir, &metadata) {
 warn!("Failed to update Eagle metadata: {}", e);
 }
 }

 // 4.5. Handle XMP sidecar file (if exists)
 // Eagle XMP handling: merge into target file, then delete sidecar
 let original_xmp = Self::get_xmp_sidecar_path(&original_file);

 if original_xmp.exists() {
 info!("Found XMP sidecar: {:?}", original_xmp.file_name());

 // Use exiftool to merge XMP into converted file
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
 info!("XMP merged into target file: {:?}", output_file.file_name());

 // Delete original XMP after successful merge
 if let Err(e) = std::fs::remove_file(&original_xmp) {
 warn!("Failed to delete original XMP: {}", e);
 } else {
 info!("Original XMP sidecar deleted");
 }
 },
 Ok(output) => {
 error!("XMP merge failed: {}", String::from_utf8_lossy(&output.stderr));
 warn!("Keeping original XMP sidecar: {:?}", original_xmp);
 },
 Err(e) => {
 error!("Cannot execute exiftool: {}", e);
 warn!("Please install exiftool: brew install exiftool");
 warn!("Keeping original XMP sidecar: {:?}", original_xmp);
 }
 }
 }

 // 5. Delete original file (replaced by new format, XMP already handled)
 // Important: Only delete original if input and output files are different
 // Prevents accidentally deleting output during same-format conversion (e.g. JXL → JXL)
 if original_file != output_file {
 if let Err(e) = std::fs::remove_file(&original_file) {
 error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 error!("[CRITICAL] Failed to delete original file!");
 error!("File: {:?}", original_file);
 error!("Error: {}", e);
 error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
 error!("Original file may still exist, please check manually");
 } else {
 info!("Original file deleted: {:?}", original_file.file_name());
 }
 } else {
 info!("Input and output have same name, skipping original file deletion");
 }

 info!("Conversion successful: {} MB → {} MB, saved {:.2} MB",
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

/// batchoptimizationreport
#[derive(Debug)]
pub structure BatchOptimizeReport {
 /// 总image数
 pub total: usize,
 /// successprocessing数
 pub processed: usize,
 /// skip数（已istargetformat）
 pub skipped: usize,
 /// failure数
 pub failed: usize,
 /// original总size（bytes）
 pub total_original_size: u64,
 /// optimized总size（bytes）
 pub total_optimized_size: u64,
 /// 节省总bytes数
 pub saved_bytes: u64,
 /// compression率（percentage）
 pub compression_ratio: f64,
 /// errorlist
 pub errors: Vec<OptimizeError>,
}

/// optimizationerror
#[derive(Debug)]
pub structure OptimizeError {
 /// path
 pub path: PathBuf,
 /// errorinformation
 pub error: String,
}

/// singleoptimizationresult
#[derive(Debug)]
structure OptimizeResult {
 /// originalfilesize（bytes）
 original_size: u64,
 /// optimizedfilesize（bytes）
 optimized_size: u64,
 /// 节省bytes数
 saved_bytes: u64,
 /// is否skipprocessing
 skipped: bool,
}
