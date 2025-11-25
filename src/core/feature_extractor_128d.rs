//! 🔬 128dimensionalstandardizefeatureextraction
//!
//! fullimplementation Python ML Bridgeneed128dimensionalfeature
//!
//! featuregroup：
//! - Basic (16dimensional): property
//! - Color (16dimensional): color and statistics
//! - Texture (16dimensional): texture and edgeinformation
//! - Shape (16dimensional): how manywhat and structure
//! - Quality (16dimensional): quality
//! - Metadata (32dimensional): EXIF and fileproperty
//! - Context (16dimensional): contextinformation

use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;

/// full128dimensionalfeatureextraction
pub fn extract_128d_features(
 img: &DynamicImage,
 file_path: &Path,
 basic_features: &crate::ImageFeatures,
) -> Vec<f64> {
 let mut features = Vec::with_capacity(128);

// Basic Features (16dimensional)
 features.extend(extract_basic_features(basic_features));

// Color Features (16dimensional)
 features.extend(extract_color_features(img));

// Texture Features (16dimensional)
 features.extend(extract_texture_features(img));

// Shape Features (16dimensional)
 features.extend(extract_shape_features(img, basic_features));

// Quality Features (16dimensional)
 features.extend(extract_quality_features(img));

// Metadata Features (32dimensional)
 features.extend(extract_metadata_features(file_path));

// Context Features (16dimensional)
 features.extend(extract_context_features(basic_features));

 assert_eq!(features.len(), 128, "Feature vector must be 128 dimensions");
 features
}

/// Basic Features (16dimensional)
fn extract_basic_features(features: &crate::ImageFeatures) -> Vec<f64> {
 let mut vec = Vec::with_capacity(16);

 vec.push(features.width as f64);
 vec.push(features.height as f64);
 vec.push(features.pixels() as f64);
 vec.push(features.size_mb());
 vec.push(features.aspect_ratio());
 vec.push(if features.has_alpha { 1.0 } else { 0.0 });
 vec.push(if features.is_animated { 1.0 } else { 0.0 });
 vec.push(features.complexity);
 vec.push(if features.is_high_resolution() { 1.0 } else { 0.0 });
 vec.push(if features.is_large_image() { 1.0 } else { 0.0 });

// formatencoding
 let format_code = match features.format.as_str() {
 "png" => 1.0,
 "jpeg" | "jpg" => 2.0,
 "webp" => 3.0,
 "gif" => 4.0,
 "avif" => 5.0,
 "jxl" => 6.0,
 _ => 0.0,
 };
 vec.push(format_code);

// to16dimensional
 while vec.len() < 16 {
 vec.push(0.0);
 }

 vec
}

/// Color Features (16dimensional) - 🚀 performanceoptimizationversion
fn extract_color_features(img: &DynamicImage) -> Vec<f64> {
 let rgba = img.to_rgba8();
 let width = rgba.width() as usize;
 let height = rgba.height() as usize;
 let total_pixels = width * height;

 if total_pixels == 0 {
 return vec![0.0; 16];
 }

// 🚀 optimization1: intelligentsampling - largesampling，small全
 let step = if total_pixels > 1_000_000 { 20 } else if total_pixels > 100_000 { 10 } else { 1 };

// 🚀 optimization2: singletimescalculation has statistics
 let mut r_sum = 0u64;
 let mut g_sum = 0u64;
 let mut b_sum = 0u64;
 let mut sample_count = 0usize;

// HSVvalue
 let estimated_samples = (total_pixels / step).max(100);
 let mut hsv_values = Vec::with_capacity(estimated_samples);

// singletimes
 for y in (0..height).step_by(step) {
 for x in (0..width).step_by(step) {
 let pixel = rgba.get_pixel(x as u32, y as u32);
 let channels = pixel.channels();

 r_sum += channels[0] as u64;
 g_sum += channels[1] as u64;
 b_sum += channels[2] as u64;
 sample_count += 1;

// HSVconversion（every10pixelsamplingatimes）
 if sample_count.is_multiple_of(10) {
 let (h, s, v) = rgb_to_hsv(
 channels[0] as f64 / 255.0,
 channels[1] as f64 / 255.0,
 channels[2] as f64 / 255.0,
 );
 hsv_values.push((h, s, v));
 }
 }
 }

 let count = sample_count as f64;
 let r_mean = r_sum as f64 / count / 255.0;
 let g_mean = g_sum as f64 / count / 255.0;
 let b_mean = b_sum as f64 / count / 255.0;

// 🚀 optimization3: secondtimescalculationvariance（onlyatneed when ）
 let mut r_var = 0.0;
 let mut g_var = 0.0;
 let mut b_var = 0.0;

 for y in (0..height).step_by(step) {
 for x in (0..width).step_by(step) {
 let pixel = rgba.get_pixel(x as u32, y as u32);
 let channels = pixel.channels();
 let r = channels[0] as f64 / 255.0;
 let g = channels[1] as f64 / 255.0;
 let b = channels[2] as f64 / 255.0;

 r_var += (r - r_mean).powi(2);
 g_var += (g - g_mean).powi(2);
 b_var += (b - b_mean).powi(2);
 }
 }

 let r_std = (r_var / count).sqrt();
 let g_std = (g_var / count).sqrt();
 let b_std = (b_var / count).sqrt();

// HSVstatistics
 let mut h_sum = 0.0;
 let mut s_sum = 0.0;
 let mut v_sum = 0.0;

 for (h, s, v) in &hsv_values {
 h_sum += h;
 s_sum += s;
 v_sum += v;
 }

 let hsv_count = hsv_values.len().max(1) as f64;
 let h_mean = h_sum / hsv_count;
 let s_mean = s_sum / hsv_count;
 let v_mean = v_sum / hsv_count;

// HSVstandard
 let mut h_var = 0.0;
 let mut s_var = 0.0;
 let mut v_var = 0.0;

 for (h, s, v) in &hsv_values {
 h_var += (h - h_mean).powi(2);
 s_var += (s - s_mean).powi(2);
 v_var += (v - v_mean).powi(2);
 }

 let h_std = (h_var / hsv_count).sqrt();
 let s_std = (s_var / hsv_count).sqrt();
 let v_std = (v_var / hsv_count).sqrt();

// 🚀 colorcount - usesamplingdata
 let color_count = estimate_unique_colors(&rgba, step);

 vec![
 r_mean, g_mean, b_mean,
 r_std, g_std, b_std,
 h_mean, s_mean, v_mean,
 h_std, s_std, v_std, // ✅ truerealHSVstandard
 h_mean, // main
 color_count as f64 / 64.0, // a
 s_mean, //  and degree
 v_mean, // brightness
 ]
}

/// Texture Features (16dimensional) - realimplementation
fn extract_texture_features(img: &DynamicImage) -> Vec<f64> {
 let gray = img.to_luma8();
 let (width, height) = gray.dimensions();

 if width < 3 || height < 3 {
 return vec![0.0; 16];
 }

// Sobeledgedetection
 let mut edge_count = 0;
 let mut sobel_x_sum = 0.0;
 let mut sobel_y_sum = 0.0;

// edgestatistics（0°, 45°, 90°, 135°）
 let mut dir_0 = 0; // 
 let mut dir_45 = 0; // paircornerline↗
 let mut dir_90 = 0; // 
 let mut dir_135 = 0; // paircornerline↖

 for y in 1..(height - 1) {
 for x in 1..(width - 1) {
// Sobel X
 let gx =
 -(gray.get_pixel(x - 1, y - 1)[0] as f64) +
 1.0 * gray.get_pixel(x + 1, y - 1)[0] as f64 +
 -2.0 * gray.get_pixel(x - 1, y)[0] as f64 +
 2.0 * gray.get_pixel(x + 1, y)[0] as f64 +
 -(gray.get_pixel(x - 1, y + 1)[0] as f64) +
 1.0 * gray.get_pixel(x + 1, y + 1)[0] as f64;

// Sobel Y
 let gy =
 -(gray.get_pixel(x - 1, y - 1)[0] as f64) +
 -2.0 * gray.get_pixel(x, y - 1)[0] as f64 +
 -(gray.get_pixel(x + 1, y - 1)[0] as f64) +
 1.0 * gray.get_pixel(x - 1, y + 1)[0] as f64 +
 2.0 * gray.get_pixel(x, y + 1)[0] as f64 +
 1.0 * gray.get_pixel(x + 1, y + 1)[0] as f64;

 let magnitude = (gx * gx + gy * gy).sqrt();

 if magnitude > 30.0 {
 edge_count += 1;

// calculationedge（cornerdegree）
 let angle = gy.atan2(gx).to_degrees();
 let normalized_angle = if angle < 0.0 { angle + 180.0 } else { angle };

// classificationto4
 if !(22.5..157.5).contains(&normalized_angle) {
 dir_0 += 1; // 0° ()
 } else if (22.5..67.5).contains(&normalized_angle) {
 dir_45 += 1; // 45°
 } else if (67.5..112.5).contains(&normalized_angle) {
 dir_90 += 1; // 90° ()
 } else {
 dir_135 += 1; // 135°
 }
 }

 sobel_x_sum += gx.abs();
 sobel_y_sum += gy.abs();
 }
 }

 let pixel_count = ((width - 2) * (height - 2)) as f64;
 let edge_density = edge_count as f64 / pixel_count;
 let sobel_x_mean = sobel_x_sum / pixel_count / 255.0;
 let sobel_y_mean = sobel_y_sum / pixel_count / 255.0;

// edgenormalize
 let total_edges = (dir_0 + dir_45 + dir_90 + dir_135).max(1) as f64;
 let dir_0_norm = dir_0 as f64 / total_edges;
 let dir_45_norm = dir_45 as f64 / total_edges;
 let dir_90_norm = dir_90 as f64 / total_edges;
 let dir_135_norm = dir_135 as f64 / total_edges;

// localvariance（3x3）
 let local_var = calculate_local_variance(&gray);

 vec![
 edge_density,
 sobel_x_mean,
 sobel_y_mean,
 dir_0_norm, dir_45_norm, dir_90_norm, dir_135_norm, // ✅ truerealedge
 local_var.0, // value
 local_var.1, // standard
 edge_density * 2.0, // texturecomplexitynear
 sobel_x_mean + sobel_y_mean, // high
 0.0, 0.0, 0.0, 0.0, 0.0, // 
 ]
}

/// Shape Features (16dimensional) - realimplementation
fn extract_shape_features(img: &DynamicImage, features: &crate::ImageFeatures) -> Vec<f64> {
 let (width, height) = img.dimensions();
 let rgba = img.to_rgba8();

// how manywhatfeature
 let aspect_ratio = features.aspect_ratio();
 let log_width = (width as f64).ln();
 let log_height = (height as f64).ln();
 let log_pixels = (features.pixels() as f64).ln();

// boundarydensity（detectionnotinside）
 let bbox_tightness = calculate_bbox_tightness(&rgba);

// inside（detectioninsideisnoatcenter）
 let (center_mass_x, center_mass_y) = calculate_center_of_mass(&rgba);

// pairdetection
 let (h_symmetry, v_symmetry) = calculate_symmetry(&rgba);

// edge（insideisnoside）
 let edge_content = calculate_edge_content(&rgba);

// empty
 let blank_ratio = calculate_blank_ratio(&rgba);

 vec![
 aspect_ratio,
 log_width,
 log_height,
 log_pixels,
 bbox_tightness,
 center_mass_x,
 center_mass_y,
 h_symmetry,
 v_symmetry,
 edge_content,
 blank_ratio,
 0.0, 0.0, 0.0, 0.0, 0.0, // 
 ]
}

/// Quality Features (16dimensional) - realimplementation
fn extract_quality_features(img: &DynamicImage) -> Vec<f64> {
 let gray = img.to_luma8();

// noiseestimated（high）
 let noise_level = estimate_noise(&gray);

// cleardegree（variance）
 let sharpness = calculate_laplacian_variance(&gray);

// contrast（standard）
 let contrast = calculate_contrast(&gray);

// dynamicrange
 let dynamic_range = calculate_dynamic_range(&gray);

 vec![
 noise_level,
 sharpness,
 contrast,
 dynamic_range,
 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
 ]
}

/// Metadata Features (32dimensional) - realimplementation
fn extract_metadata_features(path: &Path) -> Vec<f64> {
 let mut features = vec![0.0; 32];

// fileproperty
 if let Ok(metadata) = std::fs::metadata(path) {
 features[0] = 1.0; // file
 features[1] = (metadata.len() as f64).ln(); // logfilesize

// modifiedtime（Unixtimenormalize）
 if let Ok(modified) = metadata.modified()
 && let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
 features[2] = (duration.as_secs() as f64 / 1e9).ln(); // atorange
 }
 }

// fileextensionencoding
 if let Some(ext) = path.extension()
 && let Some(ext_str) = ext.to_str() {
 features[3] = match ext_str.to_lowercase().as_str() {
 "jpg" | "jpeg" => 1.0,
 "png" => 2.0,
 "webp" => 3.0,
 "gif" => 4.0,
 "avif" => 5.0,
 "jxl" => 6.0,
 "bmp" => 7.0,
 "tiff" | "tif" => 8.0,
 _ => 0.0,
 };
 }

// filelength
 if let Some(filename) = path.file_name()
 && let Some(name_str) = filename.to_str() {
 features[4] = (name_str.len() as f64).ln();
 }

// EXIFparse（ifavailable）
// note：thisuseimplementation，fullEXIFneedoutsidedependency
// features[5..15]  to EXIFdata

 features
}

/// Context Features (16dimensional) - realimplementation
fn extract_context_features(features: &crate::ImageFeatures) -> Vec<f64> {
 let mut vec = Vec::with_capacity(16);

// imagetypeclassification
 let is_photo = features.complexity > 0.6 && !features.has_alpha;
 let is_graphic = features.complexity < 0.4 || features.has_alpha;
 let is_screenshot = features.width >= 1024 && features.aspect_ratio() > 1.3 && features.aspect_ratio() < 2.0;
 let is_icon = features.width <= 512 && features.height <= 512 && features.has_alpha;

 vec.push(if is_photo { 1.0 } else { 0.0 });
 vec.push(if is_graphic { 1.0 } else { 0.0 });
 vec.push(if is_screenshot { 1.0 } else { 0.0 });
 vec.push(if is_icon { 1.0 } else { 0.0 });

// resolutionclass
 let pixels = features.pixels();
 let res_category = if pixels < 500_000 {
 1.0 // lowresolution
 } else if pixels < 2_000_000 {
 2.0 // resolution
 } else if pixels < 8_000_000 {
 3.0 // highresolution
 } else {
 4.0 // highresolution
 };
 vec.push(res_category);

// filesizeclass
 let size_mb = features.size_mb();
 let size_category = if size_mb < 0.5 {
 1.0 // smallfile
 } else if size_mb < 2.0 {
 2.0 // etcfile
 } else if size_mb < 10.0 {
 3.0 // largefile
 } else {
 4.0 // largefile
 };
 vec.push(size_category);

// compression
 let compression_potential = if features.format == "png" && !features.has_alpha {
 0.8 // PNGtransparency，highcompress
 } else if features.format == "bmp" {
 0.9 // BMP，extremelyhighcompress
 } else if features.format == "jpeg" || features.format == "jpg" {
 0.3 // JPEG，lowcompress
 } else {
 0.5 // itsformat
 };
 vec.push(compression_potential);

// recommendedformatencoding
 let recommended_format = if features.has_alpha {
 if features.is_animated {
 6.0 // + → WebP/AVIF
 } else {
 5.0 //  → AVIF/WebP
 }
 } else if features.is_animated {
 4.0 //  → WebP/video
 } else if is_photo {
 3.0 // photo → AVIF/JXL
 } else {
 2.0 // graphic → WebP/AVIF
 };
 vec.push(recommended_format);

// to16dimensional
 while vec.len() < 16 {
 vec.push(0.0);
 }

 vec
}

// ========== helperfunction ==========

/// RGBHSV
fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
 let max = r.max(g).max(b);
 let min = r.min(g).min(b);
 let delta = max - min;

 let h = if delta == 0.0 {
 0.0
 } else if max == r {
 60.0 * (((g - b) / delta) % 6.0)
 } else if max == g {
 60.0 * (((b - r) / delta) + 2.0)
 } else {
 60.0 * (((r - g) / delta) + 4.0)
 };

 let s = if max == 0.0 { 0.0 } else { delta / max };
 let v = max;

 (h / 360.0, s, v)
}

/// 🚀 optimization：唯acolorcount（from Image Buffersampling）
fn estimate_unique_colors(rgba: &image::RgbaImage, step: usize) -> usize {
 use std::collections::HashSet;

 let mut colors = HashSet::new();
 let (width, height) = rgba.dimensions();
 let quantize = 64;
 let q_step = 256 / quantize;

 for y in (0..height).step_by(step) {
 for x in (0..width).step_by(step) {
 let pixel = rgba.get_pixel(x, y);
 let channels = pixel.channels();
 let r = (channels[0] / q_step as u8) * q_step as u8;
 let g = (channels[1] / q_step as u8) * q_step as u8;
 let b = (channels[2] / q_step as u8) * q_step as u8;
 colors.insert((r, g, b));
 }
 }

 colors.len()
}

/// colorcount（oldversioncompatibility）
#[allow(dead_code)]
fn estimate_color_count(pixels: &[&image::Rgba<u8>], quantize: usize) -> usize {
 use std::collections::HashSet;

 let mut colors = HashSet::new();
 let step = 256 / quantize;

 for pixel in pixels.iter().step_by(10) {
 let channels = pixel.channels();
 let r = (channels[0] / step as u8) * step as u8;
 let g = (channels[1] / step as u8) * step as u8;
 let b = (channels[2] / step as u8) * step as u8;
 colors.insert((r, g, b));
 }

 colors.len()
}

/// calculationlocalvariance
fn calculate_local_variance(gray: &image::GrayImage) -> (f64, f64) {
 let (width, height) = gray.dimensions();

 if width < 3 || height < 3 {
 return (0.0, 0.0);
 }

 let mut variances = Vec::new();

 for y in 1..(height - 1) {
 for x in 1..(width - 1) {
 let mut sum = 0.0;
 let mut count = 0;

 for dy in -1..=1 {
 for dx in -1..=1 {
 let px = (x as i32 + dx) as u32;
 let py = (y as i32 + dy) as u32;
 sum += gray.get_pixel(px, py)[0] as f64;
 count += 1;
 }
 }

 let mean = sum / count as f64;

 let mut var = 0.0;
 for dy in -1..=1 {
 for dx in -1..=1 {
 let px = (x as i32 + dx) as u32;
 let py = (y as i32 + dy) as u32;
 let val = gray.get_pixel(px, py)[0] as f64;
 var += (val - mean).powi(2);
 }
 }

 variances.push(var / count as f64);
 }
 }

 if variances.is_empty() {
 return (0.0, 0.0);
 }

 let mean = variances.iter().sum::<f64>() / variances.len() as f64;
 let std = (variances.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / variances.len() as f64).sqrt();

 (mean / 255.0, std / 255.0)
}

/// noise
fn estimate_noise(gray: &image::GrayImage) -> f64 {
 let (width, height) = gray.dimensions();

 if width < 2 || height < 2 {
 return 0.0;
 }

 let mut diff_sum = 0.0;
 let mut count = 0;

 for y in 0..(height - 1) {
 for x in 0..(width - 1) {
 let curr = gray.get_pixel(x, y)[0] as f64;
 let right = gray.get_pixel(x + 1, y)[0] as f64;
 let down = gray.get_pixel(x, y + 1)[0] as f64;

 diff_sum += (curr - right).abs();
 diff_sum += (curr - down).abs();
 count += 2;
 }
 }

 (diff_sum / count as f64) / 255.0
}

/// calculationvariance（cleardegree）
fn calculate_laplacian_variance(gray: &image::GrayImage) -> f64 {
 let (width, height) = gray.dimensions();

 if width < 3 || height < 3 {
 return 0.0;
 }

 let mut laplacian_sum = 0.0;
 let mut count = 0;

 for y in 1..(height - 1) {
 for x in 1..(width - 1) {
 let center = gray.get_pixel(x, y)[0] as f64;
 let top = gray.get_pixel(x, y - 1)[0] as f64;
 let bottom = gray.get_pixel(x, y + 1)[0] as f64;
 let left = gray.get_pixel(x - 1, y)[0] as f64;
 let right = gray.get_pixel(x + 1, y)[0] as f64;

 let laplacian = (4.0 * center - top - bottom - left - right).abs();
 laplacian_sum += laplacian;
 count += 1;
 }
 }

 (laplacian_sum / count as f64) / 255.0
}

/// calculationcontrast
fn calculate_contrast(gray: &image::GrayImage) -> f64 {
 let pixels: Vec<_> = gray.pixels().map(|p| p[0] as f64).collect();

 if pixels.is_empty() {
 return 0.0;
 }

 let mean = pixels.iter().sum::<f64>() / pixels.len() as f64;
 let variance = pixels.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / pixels.len() as f64;

 variance.sqrt() / 255.0
}

/// calculationdynamicrange
fn calculate_dynamic_range(gray: &image::GrayImage) -> f64 {
 let pixels: Vec<_> = gray.pixels().map(|p| p[0]).collect();

 if pixels.is_empty() {
 return 0.0;
 }

// Safe: pixels is non-empty, min/max will always return Some
 let min = *pixels.iter().min().expect("pixels is non-empty") as f64;
 let max = *pixels.iter().max().expect("pixels is non-empty") as f64;

 (max - min) / 255.0
}

/// calculationboundarydensity
fn calculate_bbox_tightness(rgba: &image::RgbaImage) -> f64 {
 let (width, height) = rgba.dimensions();

// findnottransparencypixelboundary
 let mut min_x = width;
 let mut max_x = 0;
 let mut min_y = height;
 let mut max_y = 0;
 let mut opaque_count = 0;

 for y in 0..height {
 for x in 0..width {
 let pixel = rgba.get_pixel(x, y);
 if pixel[3] > 128 { // not
 min_x = min_x.min(x);
 max_x = max_x.max(x);
 min_y = min_y.min(y);
 max_y = max_y.max(y);
 opaque_count += 1;
 }
 }
 }

 if opaque_count == 0 {
 return 1.0; // 全，densityfor1
 }

 let bbox_area = ((max_x - min_x + 1) * (max_y - min_y + 1)) as f64;
 let total_area = (width * height) as f64;

 bbox_area / total_area
}

/// calculationcenter
fn calculate_center_of_mass(rgba: &image::RgbaImage) -> (f64, f64) {
 let (width, height) = rgba.dimensions();

 let mut sum_x = 0.0;
 let mut sum_y = 0.0;
 let mut total_weight = 0.0;

 for y in 0..height {
 for x in 0..width {
 let pixel = rgba.get_pixel(x, y);
 let weight = pixel[3] as f64 / 255.0; // usingalphaforweight

 sum_x += x as f64 * weight;
 sum_y += y as f64 * weight;
 total_weight += weight;
 }
 }

 if total_weight == 0.0 {
 return (0.5, 0.5);
 }

 let center_x = sum_x / total_weight / width as f64;
 let center_y = sum_y / total_weight / height as f64;

 (center_x, center_y)
}

/// calculationpair
fn calculate_symmetry(rgba: &image::RgbaImage) -> (f64, f64) {
 let (width, height) = rgba.dimensions();

// pair
 let mut h_diff = 0.0;
 let mut h_count = 0;

 for y in 0..height {
 for x in 0..(width / 2) {
 let left = rgba.get_pixel(x, y);
 let right = rgba.get_pixel(width - 1 - x, y);

 for i in 0..4 {
 h_diff += (left[i] as f64 - right[i] as f64).abs();
 }
 h_count += 4;
 }
 }

// pair
 let mut v_diff = 0.0;
 let mut v_count = 0;

 for y in 0..(height / 2) {
 for x in 0..width {
 let top = rgba.get_pixel(x, y);
 let bottom = rgba.get_pixel(x, height - 1 - y);

 for i in 0..4 {
 v_diff += (top[i] as f64 - bottom[i] as f64).abs();
 }
 v_count += 4;
 }
 }

 let h_symmetry = 1.0 - (h_diff / h_count as f64 / 255.0);
 let v_symmetry = 1.0 - (v_diff / v_count as f64 / 255.0);

 (h_symmetry.max(0.0), v_symmetry.max(0.0))
}

/// calculationedgeinside
fn calculate_edge_content(rgba: &image::RgbaImage) -> f64 {
 let (width, height) = rgba.dimensions();
 let border = 10.min(width / 10).min(height / 10);

 let mut edge_pixels = 0;
 let mut total_pixels = 0;

// updownedge
 for y in 0..border {
 for x in 0..width {
 if rgba.get_pixel(x, y)[3] > 128 {
 edge_pixels += 1;
 }
 if rgba.get_pixel(x, height - 1 - y)[3] > 128 {
 edge_pixels += 1;
 }
 total_pixels += 2;
 }
 }

// leftrightedge
 for x in 0..border {
 for y in border..(height - border) {
 if rgba.get_pixel(x, y)[3] > 128 {
 edge_pixels += 1;
 }
 if rgba.get_pixel(width - 1 - x, y)[3] > 128 {
 edge_pixels += 1;
 }
 total_pixels += 2;
 }
 }

 edge_pixels as f64 / total_pixels as f64
}

/// calculationempty
fn calculate_blank_ratio(rgba: &image::RgbaImage) -> f64 {
 let (width, height) = rgba.dimensions();
 let mut blank_count = 0;

 for y in 0..height {
 for x in 0..width {
 let pixel = rgba.get_pixel(x, y);

// detectionisnoforempty（色ortransparency）
 let is_white = pixel[0] > 250 && pixel[1] > 250 && pixel[2] > 250;
 let is_transparent = pixel[3] < 5;

 if is_white || is_transparent {
 blank_count += 1;
 }
 }
 }

 blank_count as f64 / (width * height) as f64
}

#[cfg(test)]
mod tests {
 use super::*;
 use image::RgbaImage;

 #[test]
 fn test_rgb_to_hsv() {
 let (h, s, v) = rgb_to_hsv(1.0, 0.0, 0.0);
 assert!((h - 0.0).abs() < 0.01);
 assert!((s - 1.0).abs() < 0.01);
 assert!((v - 1.0).abs() < 0.01);
 }

 #[test]
 fn test_feature_dimensions() {
 let img = DynamicImage::ImageRgba8(RgbaImage::new(100, 100));
 let features = crate::ImageFeatures {
 width: 100,
 height: 100,
 file_size: 10000,
 format: "png".to_string(),
 has_alpha: false,
 is_animated: false,
 complexity: 0.5,
 };

 let vec = extract_128d_features(&img, Path::new("test.png"), &features);
 assert_eq!(vec.len(), 128);
 }
}
