// 🎯 Pixly统一常量配置
//
// Phase 46.8: 三端共享的常量定义
// 确保Go/Rust/JS使用相同的参数范围和阈值

/// 参数范围常量
pub mod param_ranges {
    /// Quality参数范围 (1-100)
    pub const QUALITY_MIN: u8 = 1;
    pub const QUALITY_MAX: u8 = 100;
    pub const QUALITY_DEFAULT: u8 = 85;

    /// Speed/Effort参数范围 (0-10)
    pub const SPEED_MIN: u8 = 0;
    pub const SPEED_MAX: u8 = 10;
    pub const SPEED_DEFAULT: u8 = 6;

    /// AVIF Quantizer范围 (0-63)
    pub const AVIF_QUANTIZER_MIN: u8 = 0;
    pub const AVIF_QUANTIZER_MAX: u8 = 63;

    /// JXL Distance范围 (0.0-15.0)
    pub const JXL_DISTANCE_MIN: f64 = 0.0;
    pub const JXL_DISTANCE_MAX: f64 = 15.0;

    /// WebP Method范围 (0-6)
    pub const WEBP_METHOD_MIN: u8 = 0;
    pub const WEBP_METHOD_MAX: u8 = 6;

    /// FFmpeg CRF范围 (0-51)
    pub const FFMPEG_CRF_MIN: u8 = 0;
    pub const FFMPEG_CRF_MAX: u8 = 51;

    /// 图像尺寸范围
    pub const IMAGE_DIM_MIN: u32 = 1;
    pub const IMAGE_DIM_MAX: u32 = 65535;

    /// 图像总像素数限制（10亿像素）
    pub const IMAGE_MAX_PIXELS: u64 = 1_000_000_000;

    /// 文件大小限制 (100MB)
    pub const FILE_SIZE_MAX: u64 = 100 * 1024 * 1024;
}

/// AI相关常量
pub mod ai {
    /// AI置信度阈值
    pub const CONFIDENCE_REJECT_THRESHOLD: f64 = 0.5;  // 低于此值拒绝
    pub const CONFIDENCE_WARN_THRESHOLD: f64 = 0.7;    // 低于此值警告

    /// AI服务超时（秒）
    pub const AI_TIMEOUT_SECONDS: u64 = 30;

    /// AI最大重试次数
    pub const AI_MAX_RETRIES: u8 = 3;
}

/// 支持的格式列表
pub mod formats {
    /// 输入格式
    pub const SUPPORTED_INPUT_FORMATS: &[&str] = &[
        "jpg", "jpeg", "png", "webp", "gif", "bmp",
        "tiff", "tif", "avif", "jxl", "heic", "heif",
    ];

    /// 输出格式
    pub const SUPPORTED_OUTPUT_FORMATS: &[&str] = &[
        "jpg", "jpeg", "png", "webp", "avif", "jxl", "gif", "heic",
    ];

    /// 支持无损模式的格式
    pub const LOSSLESS_FORMATS: &[&str] = &[
        "png", "webp", "avif", "jxl",
    ];
}

/// 工具名称常量
pub mod tools {
    pub const CJXL: &str = "cjxl";
    pub const DJXL: &str = "djxl";
    pub const AVIFENC: &str = "avifenc";
    pub const AVIFDEC: &str = "avifdec";
    pub const CWEBP: &str = "cwebp";
    pub const DWEBP: &str = "dwebp";
    pub const FFMPEG: &str = "ffmpeg";
    pub const MAGICK: &str = "magick";

    /// 所有支持的工具
    pub const ALL_TOOLS: &[&str] = &[
        CJXL, DJXL, AVIFENC, AVIFDEC,
        CWEBP, DWEBP, FFMPEG, MAGICK,
    ];
}

/// 验证消息常量
pub mod validation_messages {
    pub const QUALITY_OUT_OF_RANGE: &str = "Quality参数超出范围";
    pub const SPEED_OUT_OF_RANGE: &str = "Speed参数超出范围";
    pub const IMAGE_TOO_LARGE: &str = "图像尺寸超限";
    pub const FILE_NOT_FOUND: &str = "文件不存在";
    pub const FORMAT_UNSUPPORTED: &str = "格式不支持";
    pub const AI_CONFIDENCE_LOW: &str = "AI置信度过低";
}

/// 性能相关常量
pub mod performance {
    /// 批量处理默认并发数
    pub const DEFAULT_CONCURRENCY: usize = 4;

    /// 批量处理最大并发数
    pub const MAX_CONCURRENCY: usize = 16;

    /// 单个图像处理超时（秒）
    pub const PROCESSING_TIMEOUT_SECONDS: u64 = 300;
}

/// 验证辅助函数
pub mod validators {
    use super::*;

    /// 验证quality参数
    pub fn validate_quality(value: u8) -> Result<(), String> {
        if value < param_ranges::QUALITY_MIN || value > param_ranges::QUALITY_MAX {
            Err(format!("{} (应为{}-{})", 
                validation_messages::QUALITY_OUT_OF_RANGE,
                param_ranges::QUALITY_MIN, 
                param_ranges::QUALITY_MAX
            ))
        } else {
            Ok(())
        }
    }

    /// 验证speed参数
    pub fn validate_speed(value: u8) -> Result<(), String> {
        if value < param_ranges::SPEED_MIN || value > param_ranges::SPEED_MAX {
            Err(format!("{} (应为{}-{})",
                validation_messages::SPEED_OUT_OF_RANGE,
                param_ranges::SPEED_MIN,
                param_ranges::SPEED_MAX
            ))
        } else {
            Ok(())
        }
    }

    /// 验证图像尺寸
    pub fn validate_image_dimensions(width: u32, height: u32) -> Result<(), String> {
        if width < param_ranges::IMAGE_DIM_MIN || width > param_ranges::IMAGE_DIM_MAX ||
           height < param_ranges::IMAGE_DIM_MIN || height > param_ranges::IMAGE_DIM_MAX {
            return Err(format!("{} (应为{}x{} - {}x{})",
                validation_messages::IMAGE_TOO_LARGE,
                param_ranges::IMAGE_DIM_MIN, param_ranges::IMAGE_DIM_MIN,
                param_ranges::IMAGE_DIM_MAX, param_ranges::IMAGE_DIM_MAX
            ));
        }

        let total_pixels = width as u64 * height as u64;
        if total_pixels > param_ranges::IMAGE_MAX_PIXELS {
            return Err(format!("{} (总像素: {}, 最大: {})",
                validation_messages::IMAGE_TOO_LARGE,
                total_pixels,
                param_ranges::IMAGE_MAX_PIXELS
            ));
        }

        Ok(())
    }

    /// 检查输入格式是否支持
    pub fn is_input_format_supported(format: &str) -> bool {
        formats::SUPPORTED_INPUT_FORMATS.contains(&format.to_lowercase().as_str())
    }

    /// 检查输出格式是否支持
    pub fn is_output_format_supported(format: &str) -> bool {
        formats::SUPPORTED_OUTPUT_FORMATS.contains(&format.to_lowercase().as_str())
    }

    /// 检查格式是否支持无损模式
    pub fn supports_lossless(format: &str) -> bool {
        formats::LOSSLESS_FORMATS.contains(&format.to_lowercase().as_str())
    }

    /// 检查工具名称是否有效
    pub fn is_valid_tool(tool: &str) -> bool {
        tools::ALL_TOOLS.contains(&tool.to_lowercase().as_str())
    }
}
