/**
 * Phase 47.20 (R-006): WASM绑定
 * 提供浏览器环境下的图像处理能力
 */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};

// 使用lol_alloc作为全局分配器（减小WASM体积）- 替代已废弃的wee_alloc
#[cfg(feature = "lol_alloc")]
#[global_allocator]
static ALLOC: lol_alloc::AssumeSingleThreaded<lol_alloc::FreeListAllocator> = 
    unsafe { lol_alloc::AssumeSingleThreaded::new(lol_alloc::FreeListAllocator::new()) };

// 设置panic hook
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    console::log_1(&"🚀 Pixly WASM initialized".into());
}

/// 图像转换参数
#[wasm_bindgen]
#[derive(Clone)]
pub struct ConvertOptions {
    quality: u8,
    format: String,
    preserve_metadata: bool,
}

#[wasm_bindgen]
impl ConvertOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            quality: 85,
            format: "webp".to_string(),
            preserve_metadata: false,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn quality(&self) -> u8 {
        self.quality
    }

    #[wasm_bindgen(setter)]
    pub fn set_quality(&mut self, value: u8) {
        self.quality = value.min(100);
    }

    #[wasm_bindgen(getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_format(&mut self, value: String) {
        self.format = value;
    }

    #[wasm_bindgen(getter)]
    pub fn preserve_metadata(&self) -> bool {
        self.preserve_metadata
    }

    #[wasm_bindgen(setter)]
    pub fn set_preserve_metadata(&mut self, value: bool) {
        self.preserve_metadata = value;
    }
}

/// 主要的WASM图像转换器
#[wasm_bindgen]
pub struct PixlyWasm {
    image: Option<DynamicImage>,
}

#[wasm_bindgen]
impl PixlyWasm {
    /// 创建新实例
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { image: None }
    }

    /// 从Uint8Array加载图像
    #[wasm_bindgen]
    pub fn load_from_bytes(&mut self, data: &[u8]) -> Result<(), JsValue> {
        console::log_1(&format!("📁 Loading image from {} bytes", data.len()).into());
        
        let img = image::load_from_memory(data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
        
        let (width, height) = img.dimensions();
        console::log_1(&format!("✅ Image loaded: {}x{}", width, height).into());
        
        self.image = Some(img);
        Ok(())
    }

    /// 从Base64字符串加载图像
    #[wasm_bindgen]
    pub fn load_from_base64(&mut self, base64_str: &str) -> Result<(), JsValue> {
        let data = general_purpose::STANDARD
            .decode(base64_str.trim_start_matches("data:image/").split(',').nth(1).unwrap_or(base64_str))
            .map_err(|e| JsValue::from_str(&format!("Failed to decode base64: {}", e)))?;
        
        self.load_from_bytes(&data)
    }

    /// 转换图像
    #[wasm_bindgen]
    pub fn convert(&self, options: &ConvertOptions) -> Result<Vec<u8>, JsValue> {
        let img = self.image.as_ref()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("🔄 Converting to {} with quality {}", options.format, options.quality).into());
        
        let format = match options.format.as_str() {
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "png" => ImageFormat::Png,
            "webp" => ImageFormat::WebP,
            _ => return Err(JsValue::from_str(&format!("Unsupported format: {}", options.format))),
        };
        
        let mut buffer = Cursor::new(Vec::new());
        
        // 根据格式和质量进行转换
        match format {
            ImageFormat::Jpeg => {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, options.quality);
                img.write_with_encoder(encoder)
                    .map_err(|e| JsValue::from_str(&format!("JPEG encoding failed: {}", e)))?;
            }
            ImageFormat::WebP => {
                // WebP编码
                let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut buffer);
                img.write_with_encoder(encoder)
                    .map_err(|e| JsValue::from_str(&format!("WebP encoding failed: {}", e)))?;
            }
            _ => {
                img.write_to(&mut buffer, format)
                    .map_err(|e| JsValue::from_str(&format!("Encoding failed: {}", e)))?;
            }
        }
        
        let result = buffer.into_inner();
        console::log_1(&format!("✅ Conversion complete: {} bytes", result.len()).into());
        
        Ok(result)
    }

    /// 转换为Base64
    #[wasm_bindgen]
    pub fn convert_to_base64(&self, options: &ConvertOptions) -> Result<String, JsValue> {
        let bytes = self.convert(options)?;
        let base64 = general_purpose::STANDARD.encode(&bytes);
        let mime = match options.format.as_str() {
            "jpeg" | "jpg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            _ => "image/unknown",
        };
        Ok(format!("data:{};base64,{}", mime, base64))
    }

    /// 调整图像大小
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("📐 Resizing to {}x{}", width, height).into());
        
        *img = img.resize(width, height, image::imageops::FilterType::Lanczos3);
        
        Ok(())
    }

    /// 裁剪图像
    #[wasm_bindgen]
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("✂️ Cropping: x={}, y={}, w={}, h={}", x, y, width, height).into());
        
        *img = img.crop_imm(x, y, width, height);
        
        Ok(())
    }

    /// 旋转图像
    #[wasm_bindgen]
    pub fn rotate(&mut self, degrees: i32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("🔄 Rotating {} degrees", degrees).into());
        
        *img = match degrees {
            90 | -270 => img.rotate90(),
            180 | -180 => img.rotate180(),
            270 | -90 => img.rotate270(),
            _ => return Err(JsValue::from_str(&format!("Unsupported rotation: {}", degrees))),
        };
        
        Ok(())
    }

    /// 获取图像尺寸
    #[wasm_bindgen]
    pub fn get_dimensions(&self) -> Result<Vec<u32>, JsValue> {
        let img = self.image.as_ref()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        let (width, height) = img.dimensions();
        Ok(vec![width, height])
    }

    /// 应用模糊
    #[wasm_bindgen]
    pub fn blur(&mut self, sigma: f32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("🌫️ Applying blur with sigma={}", sigma).into());
        
        *img = img.blur(sigma);
        
        Ok(())
    }

    /// 调整亮度
    #[wasm_bindgen]
    pub fn adjust_brightness(&mut self, value: i32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("☀️ Adjusting brightness by {}", value).into());
        
        *img = img.brighten(value);
        
        Ok(())
    }

    /// 调整对比度
    #[wasm_bindgen]
    pub fn adjust_contrast(&mut self, value: f32) -> Result<(), JsValue> {
        let img = self.image.as_mut()
            .ok_or_else(|| JsValue::from_str("No image loaded"))?;
        
        console::log_1(&format!("🎨 Adjusting contrast by {}", value).into());
        
        *img = img.adjust_contrast(value);
        
        Ok(())
    }
}

/// 批量处理器
#[wasm_bindgen]
pub struct BatchProcessor {
    images: Vec<DynamicImage>,
}

#[wasm_bindgen]
impl BatchProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
        }
    }

    /// 添加图像到批处理队列
    #[wasm_bindgen]
    pub fn add_image(&mut self, data: &[u8]) -> Result<usize, JsValue> {
        let img = image::load_from_memory(data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
        
        self.images.push(img);
        Ok(self.images.len() - 1)
    }

    /// 批量转换所有图像
    #[wasm_bindgen]
    pub fn convert_all(&self, options: &ConvertOptions) -> Result<Vec<Vec<u8>>, JsValue> {
        console::log_1(&format!("🔄 Batch converting {} images", self.images.len()).into());
        
        let mut results = Vec::new();
        
        for (i, img) in self.images.iter().enumerate() {
            console::log_1(&format!("  Processing image {}/{}", i + 1, self.images.len()).into());
            
            let format = match options.format.as_str() {
                "jpeg" | "jpg" => ImageFormat::Jpeg,
                "png" => ImageFormat::Png,
                "webp" => ImageFormat::WebP,
                _ => return Err(JsValue::from_str(&format!("Unsupported format: {}", options.format))),
            };
            
            let mut buffer = Cursor::new(Vec::new());
            img.write_to(&mut buffer, format)
                .map_err(|e| JsValue::from_str(&format!("Encoding failed: {}", e)))?;
            
            results.push(buffer.into_inner());
        }
        
        console::log_1(&"✅ Batch conversion complete".into());
        Ok(results)
    }

    /// 清空批处理队列
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.images.clear();
        console::log_1(&"🗑️ Batch queue cleared".into());
    }

    /// 获取队列中的图像数量
    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.images.len()
    }
}
