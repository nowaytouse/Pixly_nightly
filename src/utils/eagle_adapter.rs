/**
 * ==========================================
 * Eagle Adapter - Eagle integration module
 * ==========================================
 *
 * Implementation for deep integration with Eagle software:
 * - Auto-detect Eagle library structure
 * - Generate preview and thumbnails
 * - Manage Eagle element data and labels
 * - Support .info directory structure
 * - Phase 40.15: Parallel batch processing
 *
 * Eagle directory structure:
 * - images/xxx.library/
 *   - xxx.png (original)
 *   - .info/
 *     - xxx.json (element data)
 *     - xxx_thumbnail.png (thumbnail)
 * ==========================================
 */
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use rayon::prelude::*; // Phase 40.15: Parallel processing
use std::sync::atomic::{AtomicUsize, Ordering}; // Phase 40.15: Thread-safe counters
// Unified logging system
use tracing::{info, warn, error, debug};

/// Eagle image element metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EagleImageMetadata {
    /// Image ID
    pub id: String,
    /// Name
    pub name: String,
    /// File size
    pub size: u64,
    /// Creation time
    #[serde(rename = "btime")]
    pub birth_time: u64,
    /// Modification time
    #[serde(rename = "mtime")]
    pub modification_time: u64,
    /// File extension
    pub ext: String,
    /// Tags/labels
    pub tags: Vec<String>,
    /// Folders
    pub folders: Vec<String>,
    /// Whether deleted
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    /// URL
    pub url: String,
    /// Annotation/comment
    pub annotation: String,
    /// Height (non-image resources may not have this field)
    pub height: Option<u32>,
    /// Width (non-image resources may not have this field)
    pub width: Option<u32>,
    /// Last modified time
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
    /// Color palettes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palettes: Option<Vec<ColorPalette>>,
}

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// RGB color
    pub color: [u8; 3],
    /// Ratio
    pub ratio: f32,
}

/// Eagle resource library adapter
pub struct EagleAdapter {
    library_path: PathBuf,
}

impl EagleAdapter {
    /// Create new Eagle adapter
    pub fn new<P: AsRef<Path>>(library_path: P) -> Self {
        Self {
            library_path: library_path.as_ref().to_path_buf(),
        }
    }

    /// Get resource library path
    ///
    /// For validating path, library-level configuration, etc.
    pub fn get_library_path(&self) -> &Path {
        &self.library_path
    }

    /// Get XMP sidecar file path
    ///
    /// XMP file convention:
    /// - Standard: `image.xmp` (replace original extension with .xmp)
    /// - Example: `photo.jpg` -> `photo.xmp`
    ///
    /// # Parameters
    /// - `file_path`: Image file path
    ///
    /// # Returns
    /// XMP sidecar file full path
    fn get_xmp_sidecar_path(file_path: &Path) -> PathBuf {
        let mut xmp_path = file_path.to_path_buf();
        // Replace extension with .xmp, not append
        // Wrong: image.jpg -> image.jpg.xmp
        // Correct: image.jpg -> image.xmp
        xmp_path.set_extension("xmp");
        xmp_path
    }

    /// Validate path is inside current resource library
    ///
    /// # Parameters
    /// - `path`: Path to validate
    ///
    /// # Returns
    /// - `true`: Path is inside library
    /// - `false`: Path is outside library
    pub fn is_path_in_library<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().starts_with(&self.library_path)
    }

    /// Parse .info directory
    ///
    /// # Parameters
    /// - `info_dir`: .info directory path (e.g., images/XXXXX.info/)
    ///
    /// # Returns
    /// - `Ok(EagleImageMetadata)`: Parse success
    /// - `Err`: Parse failure
    pub fn parse_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<EagleImageMetadata> {
        let info_path = info_dir.as_ref();

        // Read metadata.json
        let metadata_path = info_path.join("metadata.json");
        let metadata_content = fs::read_to_string(&metadata_path)
            .with_context(|| format!("Failed to read metadata: {:?}", metadata_path))?;

        let metadata: EagleImageMetadata = serde_json::from_str(&metadata_content)
            .with_context(|| format!("Failed to parse metadata: {:?}", metadata_path))?;

        Ok(metadata)
    }

    /// Update .info directory metadata.json
    ///
    /// # Parameters
    /// - `info_dir`: .info directory path
    /// - `metadata`: New element data
    ///
    /// Note: Eagle uses compressed JSON format (minified), not pretty format
    /// This reduces file size and improves speed
    pub fn update_metadata<P: AsRef<Path>>(
        &self,
        info_dir: P,
        metadata: &EagleImageMetadata,
    ) -> Result<()> {
        let metadata_path = info_dir.as_ref().join("metadata.json");

        // Eagle uses compressed JSON format (not pretty format)
        let json = serde_json::to_string(metadata)
            .context("Failed to serialize metadata")?;

        fs::write(&metadata_path, json)
            .with_context(|| format!("Failed to write metadata: {:?}", metadata_path))?;

        info!("Updated Eagle metadata: {:?}", metadata_path);
        Ok(())
    }

    /// Find original file in .info directory
    ///
    /// # Parameters
    /// - `info_dir`: .info directory path
    ///
    /// # Returns
    /// - `Ok(PathBuf)`: Original file path
    /// - `Err`: Original file not found
    pub fn find_original_file<P: AsRef<Path>>(&self, info_dir: P) -> Result<PathBuf> {
        let info_path = info_dir.as_ref();

        // Exclude file patterns
        let exclude_patterns = [
            "_thumbnail.",
            "_pixly_viewer_",
            "metadata.json",
            ".DS_Store",
        ];

        // Search directory for original file
        for entry in fs::read_dir(info_path)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Check if file should be excluded
            if exclude_patterns.iter().any(|p| filename.contains(p)) {
                continue;
            }

            return Ok(path);
        }

        anyhow::bail!("Original file not found in {:?}", info_path)
    }

    /// Get thumbnail path
    ///
    /// Eagle generates thumbnails as: {original_name}_thumbnail.png
    ///
    /// Reference: <https://cn.eagle.cool/support/article/why-do-some-images-generate-extra-thumbnails>
    ///
    /// Note: Some images will have extra _thumbnail files, this is Eagle's auto-optimization
    /// for quick preview and list display, avoiding loading large files
    pub fn get_thumbnail_path<P: AsRef<Path>>(&self, info_dir: P, original_name: &str) -> PathBuf {
        let thumbnail_name = format!("{}_thumbnail.png",
            Path::new(original_name).file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("thumbnail")
        );
        info_dir.as_ref().join(thumbnail_name)
    }

    /// Generate Eagle thumbnail
    ///
    /// For large images or specific formats, generate optimized thumbnails
    ///
    /// # Parameters
    /// - `source`: Source image path
    /// - `info_dir`: .info directory path
    /// - `size`: Thumbnail dimension (default 800px)
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

        // Read source image
        let img = image::open(source)
            .context("Failed to open source image for thumbnail generation")?;

        let (width, height) = img.dimensions();

        // If image is already small, no thumbnail needed
        if width <= size && height <= size {
            debug!("Image too small for thumbnail: {}x{}", width, height);
            return Ok(source.to_path_buf());
        }

        // Calculate thumbnail dimensions (keep aspect ratio)
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

        // Save thumbnail
        let original_name = source.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image");
        let thumbnail_path = self.get_thumbnail_path(info_dir, original_name);

        thumbnail.save(&thumbnail_path)
            .context("Failed to save thumbnail")?;

        info!("Thumbnail generated: {:?}", thumbnail_path);
        Ok(thumbnail_path)
    }

    /// Check if thumbnail generation is needed
    ///
    /// Based on Eagle conventions:
    /// - Large images (>2000px)
    /// - Specific formats (PSD, AI, etc.)
    /// - Formats needing optimized preview
    pub fn should_generate_thumbnail<P: AsRef<Path>>(&self, image_path: P) -> Result<bool> {
        use image::GenericImageView;

        let path = image_path.as_ref();

        // Check file format
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();

            // These formats need thumbnails
            if matches!(ext_lower.as_str(), "psd" | "ai" | "svg" | "pdf") {
                return Ok(true);
            }
        }

        // Check image dimensions
        if let Ok(img) = image::open(path) {
            let (width, height) = img.dimensions();

            // Large images need thumbnails
            if width > 2000 || height > 2000 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Generate preview path
    pub fn generate_preview_path<P: AsRef<Path>>(&self, info_dir: P) -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis();

        info_dir.as_ref().join(format!("_pixly_viewer_{}.png", timestamp))
    }

    /// Cleanup old previews
    ///
    /// Keep latest N previews, delete the rest
    pub fn cleanup_old_previews<P: AsRef<Path>>(
        &self,
        info_dir: P,
        keep_count: usize,
    ) -> Result<usize> {
        let info_path = info_dir.as_ref();

        // Find all previews
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

        // Sort by modification time (latest first)
        previews.sort_by(|a, b| b.1.cmp(&a.1));

        // Delete previews exceeding keep_count
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

    /// Validate .info directory structure
    pub fn validate_info_dir<P: AsRef<Path>>(&self, info_dir: P) -> Result<()> {
        let info_path = info_dir.as_ref();

        // Check directory exists
        if !info_path.exists() {
            anyhow::bail!(".info directory does not exist: {:?}", info_path);
        }

        if !info_path.is_dir() {
            anyhow::bail!("Not a directory: {:?}", info_path);
        }

        // Check metadata.json exists
        let metadata_path = info_path.join("metadata.json");
        if !metadata_path.exists() {
            anyhow::bail!("metadata.json not found: {:?}", info_path);
        }

        // Try parsing metadata.json
        self.parse_info_dir(info_path)?;

        // Check original file exists
        self.find_original_file(info_path)?;

        Ok(())
    }

    /// Phase 40.11: Scan Eagle library for all images
    ///
    /// Scan library directory, find all subdirectories containing metadata.json
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

    /// Phase 40.26: Find XMP resource in Eagle library
    ///
    /// Eagle library structure: every resource (image/XMP) has its own .info directory
    /// images/
    /// ├── XXXX.info/ (image resource)
    /// │   ├── photo.jpg
    /// │   └── metadata.json (name: "photo", ext: "jpg")
    /// └── YYYY.info/ (XMP resource)
    ///     ├── photo.xmp
    ///     └── metadata.json (name: "photo", ext: "xmp")
    ///
    /// Associate resources via metadata.json name field
    ///
    /// Parameters:
    /// - current_info_dir: Current image's .info directory
    /// - image_name: Image name field (without extension)
    ///
    /// Returns: XMP file full path (if found)
    pub fn find_xmp_resource<P: AsRef<Path>>(
        &self,
        current_info_dir: P,
        image_name: &str
    ) -> Option<PathBuf> {
        let current_path = current_info_dir.as_ref();

        // Get images/ directory
        let images_dir = current_path.parent()?;

        log::debug!("Searching for XMP resource with same name in Eagle library: {}", image_name);
        log::debug!("Current .info directory: {:?}", current_path);
        log::debug!("images directory: {:?}", images_dir);

        // Iterate all .info directories under images/
        let entries = match std::fs::read_dir(images_dir) {
            Ok(e) => e,
            Err(e) => {
                log::debug!("Cannot read Eagle images directory: {}", e);
                return None;
            }
        };

        log::debug!("Starting to traverse .info directories in images directory");
        let mut found_count = 0;

        for entry in entries.flatten() {
            found_count += 1;
            let path = entry.path();

            // Only process .info directories, skip current image's .info directory
            if !path.is_dir() || !path.file_name()?.to_str()?.ends_with(".info") {
                continue;
            }

            if path == current_path {
                continue; // Skip self
            }

            // Read and parse metadata.json
            match self.parse_info_dir(&path) {
                Ok(metadata) => {
                    // Phase 40.33: Only log output when match found, reduce logging

                    // Check if XMP resource with matching name
                    if metadata.ext == "xmp" && metadata.name == image_name {
                        // Found matching XMP resource!
                        let xmp_file = path.join(format!("{}.xmp", metadata.name));
                        if xmp_file.exists() {
                            log::debug!("Found XMP resource in Eagle library: {}/{}.xmp",
                                path.file_name()?.to_str()?, metadata.name);
                            return Some(xmp_file);
                        }
                    }
                },
                Err(e) => {
                    // Loud warning, not silent
                    warn!("Failed to parse Eagle metadata: {:?}, error: {}", path, e);
                    warn!("This may be a non-standard resource or test file, skipping");
                    continue;
                }
            }
        }

        log::debug!("Traversal completed: checked {} entries in total", found_count);
        log::debug!("XMP resource with same name not found in Eagle library");
        None
    }

    /// Recursively scan directory
    #[allow(clippy::only_used_in_recursion)]
    fn scan_directory(&self, dir: &Path, image_dirs: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Check if contains metadata.json (Eagle image directory)
                let metadata_path = path.join("metadata.json");
                if metadata_path.exists() {
                    image_dirs.push(path.clone());
                }

                // Continue recursively scanning subdirectories
                self.scan_directory(&path, image_dirs)?;
            }
        }

        Ok(())
    }

    /// Phase 40.11: Batch optimize Eagle library images
    ///
    /// Parameters:
    /// - format: Target format (avif, webp, jxl, etc.)
    /// - quality: Quality parameter
    /// - dry_run: Whether to only simulate (no actual conversion)
    ///
    /// Batch optimize library (sequential processing)
    pub fn batch_optimize(
        &self,
        format: &str,
        quality: u8,
        dry_run: bool,
    ) -> Result<BatchOptimizeReport> {
        self.batch_optimize_with_concurrency(format, quality, dry_run, 1)
    }

    /// Phase 40.15: Concurrent batch optimization
    ///
    /// Original design (@PROJECT_QUALITY_MANIFESTO.md):
    /// - Real concurrent processing, not simulated
    /// - Thread-safe progress tracking
    /// - Errors won't crash the process
    ///
    /// # Parameters
    /// - `format`: Target format
    /// - `quality`: Quality parameter
    /// - `dry_run`: Whether to only simulate
    /// - `max_threads`: Maximum threads (0=auto-detect)
    pub fn batch_optimize_with_concurrency(
        &self,
        format: &str,
        quality: u8,
        dry_run: bool,
        max_threads: usize,
    ) -> Result<BatchOptimizeReport> {
        let image_dirs = self.scan_library()?;
        let total = image_dirs.len();

        // Configure threads
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

        // Thread-safe counter (for real-time progress)
        let processed = AtomicUsize::new(0);
        // Note: Statistics data (failed, saved_bytes) calculated in result segment

        // Configure Rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .context("Failed to create thread pool")?;

        // Concurrent processing
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

        // Collect statistics and errors
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

        // Calculate compression ratio
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

    /// Optimize single image
    fn optimize_single_image(
        &self,
        info_dir: &Path,
        format: &str,
        _quality: u8,
        dry_run: bool,
    ) -> Result<OptimizeResult> {
        // 1. Find original file
        let original_file = self.find_original_file(info_dir)?;
        let original_size = std::fs::metadata(&original_file)?.len();

        // 2. Check if conversion is needed
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
            info!("DRY RUN: Will convert {:?} ({} -> {})",
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

        info!("Converting: {:?} -> {:?}",
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
        // Prevents accidentally deleting output during same-format conversion (e.g. JXL -> JXL)
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

        info!("Conversion successful: {} MB -> {} MB, saved {:.2} MB",
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

/// Batch optimization report
#[derive(Debug)]
pub struct BatchOptimizeReport {
    /// Total images
    pub total: usize,
    /// Successfully processed
    pub processed: usize,
    /// Skipped (already target format)
    pub skipped: usize,
    /// Failed
    pub failed: usize,
    /// Original total size (bytes)
    pub total_original_size: u64,
    /// Optimized total size (bytes)
    pub total_optimized_size: u64,
    /// Saved bytes
    pub saved_bytes: u64,
    /// Compression ratio (percentage)
    pub compression_ratio: f64,
    /// Error list
    pub errors: Vec<OptimizeError>,
}

/// Optimization error
#[derive(Debug)]
pub struct OptimizeError {
    /// File path
    pub path: PathBuf,
    /// Error message
    pub error: String,
}

/// Single optimization result
#[derive(Debug)]
struct OptimizeResult {
    /// Original file size (bytes)
    original_size: u64,
    /// Optimized file size (bytes)
    optimized_size: u64,
    /// Saved bytes
    saved_bytes: u64,
    /// Whether skipped
    skipped: bool,
}
