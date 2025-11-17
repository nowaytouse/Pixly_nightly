//! 🎓 格式知识库
//! 
//! 最全面、最现代的图像格式知识
//! 用于ML训练和智能推荐

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 格式特性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatKnowledge {
    pub name: String,
    pub full_name: String,
    pub year_released: u16,
    pub media_type: MediaType,
    pub capabilities: FormatCapabilities,
    pub performance: FormatPerformance,
    pub use_cases: Vec<UseCase>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub browser_support: BrowserSupport,
    pub compression_efficiency: f64, // 0-1, 相对于基准格式
}

/// 格式能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCapabilities {
    pub supports_alpha: bool,
    pub supports_animation: bool,
    pub supports_lossless: bool,
    pub supports_lossy: bool,
    pub supports_hdr: bool,
    pub max_dimensions: u32,
    pub bit_depth_max: u8,
    pub color_space_support: Vec<String>,
}

/// 格式性能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatPerformance {
    pub encode_speed: Speed,
    pub decode_speed: Speed,
    pub compression_ratio: f64, // vs JPEG baseline
    pub quality_retention: f64, // 0-1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Speed {
    VerySlow,
    Slow,
    Medium,
    Fast,
    VeryFast,
}

/// 使用场景
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UseCase {
    WebPhotography,
    WebGraphics,
    Archival,
    Printing,
    Animation,
    Screenshots,
    Medical,
    Scientific,
}

/// 浏览器支持
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSupport {
    pub chrome: bool,
    pub firefox: bool,
    pub safari: bool,
    pub edge: bool,
    pub global_support_percentage: f64,
}

/// 媒体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

/// 格式知识库
pub struct FormatKnowledgeBase {
    formats: HashMap<String, FormatKnowledge>,
}

impl FormatKnowledgeBase {
    /// 创建知识库
    pub fn new() -> Self {
        let mut kb = Self {
            formats: HashMap::new(),
        };
        kb.load_modern_formats();
        kb.load_video_formats();
        kb.load_audio_formats();
        kb
    }
    
    /// 加载现代图像格式知识
    fn load_modern_formats(&mut self) {
        // AVIF - 最现代的格式
        self.formats.insert("avif".to_string(), FormatKnowledge {
            name: "avif".to_string(),
            full_name: "AV1 Image File Format".to_string(),
            year_released: 2019,
            media_type: MediaType::Image,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: true,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 65536,
                bit_depth_max: 12,
                color_space_support: vec!["sRGB".to_string(), "P3".to_string(), "Rec2020".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Slow,
                decode_speed: Speed::Medium,
                compression_ratio: 0.5, // 50% of JPEG size
                quality_retention: 0.98,
            },
            use_cases: vec![UseCase::WebPhotography, UseCase::Archival],
            strengths: vec![
                "Best compression efficiency".to_string(),
                "HDR support".to_string(),
                "Wide color gamut".to_string(),
                "Royalty-free".to_string(),
            ],
            weaknesses: vec![
                "Slow encoding".to_string(),
                "Limited browser support (85%)".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 85.0,
            },
            compression_efficiency: 0.95,
        });
        
        // JXL - 最先进的技术
        self.formats.insert("jxl".to_string(), FormatKnowledge {
            name: "jxl".to_string(),
            full_name: "JPEG XL".to_string(),
            year_released: 2021,
            media_type: MediaType::Image,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: true,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 1073741824,
                bit_depth_max: 32,
                color_space_support: vec!["sRGB".to_string(), "P3".to_string(), "Rec2020".to_string(), "XYZ".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Medium,
                decode_speed: Speed::Fast,
                compression_ratio: 0.45, // 45% of JPEG size
                quality_retention: 0.99,
            },
            use_cases: vec![UseCase::Archival, UseCase::Printing, UseCase::Scientific],
            strengths: vec![
                "Best quality retention".to_string(),
                "Fastest decode speed".to_string(),
                "Progressive decoding".to_string(),
                "Lossless JPEG recompression".to_string(),
                "32-bit float support".to_string(),
            ],
            weaknesses: vec![
                "Very limited browser support (5%)".to_string(),
                "Requires external tools".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: false,
                firefox: false,
                safari: false,
                edge: false,
                global_support_percentage: 5.0,
            },
            compression_efficiency: 0.98,
        });
        
        // WebP - 最广泛支持
        self.formats.insert("webp".to_string(), FormatKnowledge {
            name: "webp".to_string(),
            full_name: "WebP".to_string(),
            year_released: 2010,
            media_type: MediaType::Image,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: true,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 16383,
                bit_depth_max: 8,
                color_space_support: vec!["sRGB".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::Fast,
                compression_ratio: 0.7, // 70% of JPEG size
                quality_retention: 0.95,
            },
            use_cases: vec![UseCase::WebPhotography, UseCase::WebGraphics, UseCase::Animation],
            strengths: vec![
                "Excellent browser support (97%)".to_string(),
                "Fast encode/decode".to_string(),
                "Good compression".to_string(),
                "Animation support".to_string(),
            ],
            weaknesses: vec![
                "No HDR support".to_string(),
                "8-bit only".to_string(),
                "Not ideal for archival".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 97.0,
            },
            compression_efficiency: 0.85,
        });
        
        // PNG - 无损标准
        self.formats.insert("png".to_string(), FormatKnowledge {
            name: "png".to_string(),
            full_name: "Portable Network Graphics".to_string(),
            year_released: 1996,
            media_type: MediaType::Image,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: false,
                supports_lossless: true,
                supports_lossy: false,
                supports_hdr: false,
                max_dimensions: 2147483647,
                bit_depth_max: 16,
                color_space_support: vec!["sRGB".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 1.0, // baseline
                quality_retention: 1.0,
            },
            use_cases: vec![UseCase::WebGraphics, UseCase::Screenshots, UseCase::Archival],
            strengths: vec![
                "Universal support (100%)".to_string(),
                "Lossless compression".to_string(),
                "Simple and reliable".to_string(),
                "16-bit support".to_string(),
            ],
            weaknesses: vec![
                "Large file sizes".to_string(),
                "No animation (use APNG)".to_string(),
                "No HDR".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 100.0,
            },
            compression_efficiency: 0.5,
        });
        
        // JPEG - 传统标准
        self.formats.insert("jpeg".to_string(), FormatKnowledge {
            name: "jpeg".to_string(),
            full_name: "Joint Photographic Experts Group".to_string(),
            year_released: 1992,
            media_type: MediaType::Image,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: false,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 65535,
                bit_depth_max: 8,
                color_space_support: vec!["sRGB".to_string(), "CMYK".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::VeryFast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 1.0, // baseline
                quality_retention: 0.90,
            },
            use_cases: vec![UseCase::WebPhotography, UseCase::Printing],
            strengths: vec![
                "Universal support (100%)".to_string(),
                "Very fast encode/decode".to_string(),
                "Mature ecosystem".to_string(),
                "Hardware acceleration".to_string(),
            ],
            weaknesses: vec![
                "No transparency".to_string(),
                "Lossy only".to_string(),
                "Compression artifacts".to_string(),
                "Outdated technology".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 100.0,
            },
            compression_efficiency: 0.6,
        });
    }
    
    /// 加载视频格式知识
    fn load_video_formats(&mut self) {
        // H.266/VVC - 最新一代视频编码标准
        self.formats.insert("h266".to_string(), FormatKnowledge {
            name: "h266".to_string(),
            full_name: "Versatile Video Coding (VVC)".to_string(),
            year_released: 2020,
            media_type: MediaType::Video,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 16384,
                bit_depth_max: 12,
                color_space_support: vec!["BT.709".to_string(), "BT.2020".to_string(), "BT.2100".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::VerySlow,
                decode_speed: Speed::Slow,
                compression_ratio: 0.35, // 35% of H.264 - 最佳压缩
                quality_retention: 0.99,
            },
            use_cases: vec![UseCase::Archival, UseCase::WebPhotography],
            strengths: vec![
                "Best compression efficiency (30-50% better than H.265)".to_string(),
                "Highest quality retention".to_string(),
                "HDR and wide color gamut".to_string(),
                "Future-proof technology".to_string(),
            ],
            weaknesses: vec![
                "Very slow encoding".to_string(),
                "Limited hardware support".to_string(),
                "New standard, limited adoption".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: false,
                firefox: false,
                safari: false,
                edge: false,
                global_support_percentage: 5.0,
            },
            compression_efficiency: 0.99,
        });
        
        // H.265/HEVC - 现代视频标准
        self.formats.insert("h265".to_string(), FormatKnowledge {
            name: "h265".to_string(),
            full_name: "High Efficiency Video Coding (HEVC)".to_string(),
            year_released: 2013,
            media_type: MediaType::Video,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 8192,
                bit_depth_max: 10,
                color_space_support: vec!["BT.709".to_string(), "BT.2020".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Slow,
                decode_speed: Speed::Medium,
                compression_ratio: 0.5, // 50% of H.264
                quality_retention: 0.96,
            },
            use_cases: vec![UseCase::WebPhotography, UseCase::Archival],
            strengths: vec![
                "50% better compression than H.264".to_string(),
                "HDR support".to_string(),
                "4K/8K ready".to_string(),
                "Wide device support".to_string(),
            ],
            weaknesses: vec![
                "Slow encoding".to_string(),
                "Patent licensing issues".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: false,
                safari: true,
                edge: true,
                global_support_percentage: 75.0,
            },
            compression_efficiency: 0.92,
        });
        
        // AV1 - 最先进的视频编码
        self.formats.insert("av1".to_string(), FormatKnowledge {
            name: "av1".to_string(),
            full_name: "AOMedia Video 1".to_string(),
            year_released: 2018,
            media_type: MediaType::Video,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 65536,
                bit_depth_max: 12,
                color_space_support: vec!["BT.709".to_string(), "BT.2020".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::VerySlow,
                decode_speed: Speed::Medium,
                compression_ratio: 0.4, // 40% of H.264
                quality_retention: 0.98,
            },
            use_cases: vec![UseCase::WebPhotography, UseCase::Archival],
            strengths: vec![
                "Best compression efficiency".to_string(),
                "Royalty-free".to_string(),
                "HDR and wide color gamut".to_string(),
                "Growing browser support".to_string(),
            ],
            weaknesses: vec![
                "Very slow encoding".to_string(),
                "Limited hardware support".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: false,
                edge: true,
                global_support_percentage: 70.0,
            },
            compression_efficiency: 0.96,
        });
        
        // VP9 - Google开源编码
        self.formats.insert("vp9".to_string(), FormatKnowledge {
            name: "vp9".to_string(),
            full_name: "VP9".to_string(),
            year_released: 2013,
            media_type: MediaType::Video,
            capabilities: FormatCapabilities {
                supports_alpha: true,
                supports_animation: true,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: true,
                max_dimensions: 16384,
                bit_depth_max: 12,
                color_space_support: vec!["BT.709".to_string(), "BT.2020".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Slow,
                decode_speed: Speed::Medium,
                compression_ratio: 0.55, // 55% of H.264
                quality_retention: 0.95,
            },
            use_cases: vec![UseCase::WebPhotography],
            strengths: vec![
                "Royalty-free".to_string(),
                "Good compression".to_string(),
                "Wide browser support".to_string(),
            ],
            weaknesses: vec![
                "Slower than H.265".to_string(),
                "Being replaced by AV1".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: false,
                edge: true,
                global_support_percentage: 95.0,
            },
            compression_efficiency: 0.88,
        });
        
        // H.264 - 传统标准 (仅在用户明确要求时使用)
        self.formats.insert("h264".to_string(), FormatKnowledge {
            name: "h264".to_string(),
            full_name: "Advanced Video Coding (AVC) - LEGACY".to_string(),
            year_released: 2003,
            media_type: MediaType::Video,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: true,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 4096,
                bit_depth_max: 8,
                color_space_support: vec!["BT.709".to_string()],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 1.0, // baseline
                quality_retention: 0.92,
            },
            use_cases: vec![],  // 不推荐用于任何场景
            strengths: vec![
                "Universal support (100%)".to_string(),
                "Hardware acceleration everywhere".to_string(),
                "Fast encode/decode".to_string(),
            ],
            weaknesses: vec![
                "OUTDATED - Use H.266/AV1/H.265 instead".to_string(),
                "Poor compression efficiency".to_string(),
                "No HDR support".to_string(),
                "No modern features".to_string(),
                "Patent licensing issues".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 100.0,
            },
            compression_efficiency: 0.50,  // 降低效率分数，不推荐使用
        });
    }
    
    /// 加载音频格式知识
    fn load_audio_formats(&mut self) {
        // Opus - 最先进的音频编码
        self.formats.insert("opus".to_string(), FormatKnowledge {
            name: "opus".to_string(),
            full_name: "Opus Interactive Audio Codec".to_string(),
            year_released: 2012,
            media_type: MediaType::Audio,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: false,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 0,
                bit_depth_max: 24,
                color_space_support: vec![],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 0.6, // vs AAC
                quality_retention: 0.98,
            },
            use_cases: vec![UseCase::WebPhotography],
            strengths: vec![
                "Best audio quality at low bitrates".to_string(),
                "Royalty-free".to_string(),
                "Low latency".to_string(),
                "Wide bitrate range (6-510 kbps)".to_string(),
            ],
            weaknesses: vec![
                "Limited hardware support".to_string(),
                "Not supported in MP4 container".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 97.0,
            },
            compression_efficiency: 0.95,
        });
        
        // AAC - 现代音频标准
        self.formats.insert("aac".to_string(), FormatKnowledge {
            name: "aac".to_string(),
            full_name: "Advanced Audio Coding".to_string(),
            year_released: 1997,
            media_type: MediaType::Audio,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: false,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 0,
                bit_depth_max: 24,
                color_space_support: vec![],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 1.0, // baseline
                quality_retention: 0.95,
            },
            use_cases: vec![UseCase::WebPhotography],
            strengths: vec![
                "Universal support".to_string(),
                "Hardware acceleration".to_string(),
                "Good quality".to_string(),
                "MP4 compatible".to_string(),
            ],
            weaknesses: vec![
                "Patent licensing".to_string(),
                "Not as efficient as Opus".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 100.0,
            },
            compression_efficiency: 0.85,
        });
        
        // MP3 - 传统标准
        self.formats.insert("mp3".to_string(), FormatKnowledge {
            name: "mp3".to_string(),
            full_name: "MPEG-1 Audio Layer III".to_string(),
            year_released: 1993,
            media_type: MediaType::Audio,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: false,
                supports_lossless: false,
                supports_lossy: true,
                supports_hdr: false,
                max_dimensions: 0,
                bit_depth_max: 16,
                color_space_support: vec![],
            },
            performance: FormatPerformance {
                encode_speed: Speed::VeryFast,
                decode_speed: Speed::VeryFast,
                compression_ratio: 1.2, // worse than AAC
                quality_retention: 0.88,
            },
            use_cases: vec![UseCase::WebPhotography],
            strengths: vec![
                "Universal support (100%)".to_string(),
                "Very fast".to_string(),
                "Simple".to_string(),
            ],
            weaknesses: vec![
                "Outdated compression".to_string(),
                "Lower quality than modern codecs".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 100.0,
            },
            compression_efficiency: 0.65,
        });
        
        // FLAC - 无损音频
        self.formats.insert("flac".to_string(), FormatKnowledge {
            name: "flac".to_string(),
            full_name: "Free Lossless Audio Codec".to_string(),
            year_released: 2001,
            media_type: MediaType::Audio,
            capabilities: FormatCapabilities {
                supports_alpha: false,
                supports_animation: false,
                supports_lossless: true,
                supports_lossy: false,
                supports_hdr: false,
                max_dimensions: 0,
                bit_depth_max: 32,
                color_space_support: vec![],
            },
            performance: FormatPerformance {
                encode_speed: Speed::Fast,
                decode_speed: Speed::Fast,
                compression_ratio: 0.5, // 50% of WAV
                quality_retention: 1.0,
            },
            use_cases: vec![UseCase::Archival],
            strengths: vec![
                "Lossless compression".to_string(),
                "Royalty-free".to_string(),
                "Fast encode/decode".to_string(),
                "High-res audio support".to_string(),
            ],
            weaknesses: vec![
                "Large file sizes".to_string(),
                "Limited browser support".to_string(),
            ],
            browser_support: BrowserSupport {
                chrome: true,
                firefox: true,
                safari: true,
                edge: true,
                global_support_percentage: 85.0,
            },
            compression_efficiency: 0.75,
        });
    }
    
    /// 获取格式知识
    pub fn get_format(&self, format: &str) -> Option<&FormatKnowledge> {
        self.formats.get(format)
    }
    
    /// 根据需求推荐格式
    pub fn recommend_for_requirements(&self, requirements: &FormatRequirements) -> Vec<String> {
        let mut scored_formats: Vec<(String, f64)> = self.formats.iter()
            .map(|(name, knowledge)| {
                let score = self.calculate_match_score(knowledge, requirements);
                (name.clone(), score)
            })
            .collect();
        
        scored_formats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_formats.into_iter().map(|(name, _)| name).collect()
    }
    
    /// 计算匹配分数
    fn calculate_match_score(&self, knowledge: &FormatKnowledge, req: &FormatRequirements) -> f64 {
        let mut score = 0.0;
        
        // 必需能力检查
        if req.needs_alpha && !knowledge.capabilities.supports_alpha {
            return 0.0;
        }
        if req.needs_animation && !knowledge.capabilities.supports_animation {
            return 0.0;
        }
        if req.needs_hdr && !knowledge.capabilities.supports_hdr {
            return 0.0;
        }
        
        // 压缩效率权重
        score += knowledge.compression_efficiency * req.compression_weight * 100.0;
        
        // 质量保持权重
        score += knowledge.performance.quality_retention * req.quality_weight * 100.0;
        
        // 浏览器支持权重
        score += knowledge.browser_support.global_support_percentage * req.compatibility_weight;
        
        // 编码速度权重
        let speed_score = match knowledge.performance.encode_speed {
            Speed::VeryFast => 1.0,
            Speed::Fast => 0.8,
            Speed::Medium => 0.6,
            Speed::Slow => 0.4,
            Speed::VerySlow => 0.2,
        };
        score += speed_score * req.speed_weight * 100.0;
        
        score
    }
    
    /// 获取所有格式
    pub fn list_all_formats(&self) -> Vec<&FormatKnowledge> {
        self.formats.values().collect()
    }
    
    /// 获取特定媒体类型的格式
    pub fn get_formats_by_type(&self, media_type: MediaType) -> Vec<&FormatKnowledge> {
        self.formats.values()
            .filter(|f| f.media_type == media_type)
            .collect()
    }
    
    /// 将格式知识转换为ML特征向量 (32维)
    /// 这些特征将被喂给ML模型，让它完全理解每种格式的特性
    pub fn get_ml_features(&self, format: &str) -> Vec<f32> {
        if let Some(knowledge) = self.get_format(format) {
            vec![
                // 基础能力 (8维)
                if knowledge.capabilities.supports_alpha { 1.0 } else { 0.0 },
                if knowledge.capabilities.supports_animation { 1.0 } else { 0.0 },
                if knowledge.capabilities.supports_lossless { 1.0 } else { 0.0 },
                if knowledge.capabilities.supports_lossy { 1.0 } else { 0.0 },
                if knowledge.capabilities.supports_hdr { 1.0 } else { 0.0 },
                knowledge.capabilities.bit_depth_max as f32 / 32.0,
                (knowledge.capabilities.max_dimensions as f32).log2() / 20.0,
                match knowledge.media_type {
                    MediaType::Image => 0.0,
                    MediaType::Video => 0.5,
                    MediaType::Audio => 1.0,
                },
                
                // 性能特征 (8维)
                match knowledge.performance.encode_speed {
                    Speed::VerySlow => 0.2,
                    Speed::Slow => 0.4,
                    Speed::Medium => 0.6,
                    Speed::Fast => 0.8,
                    Speed::VeryFast => 1.0,
                },
                match knowledge.performance.decode_speed {
                    Speed::VerySlow => 0.2,
                    Speed::Slow => 0.4,
                    Speed::Medium => 0.6,
                    Speed::Fast => 0.8,
                    Speed::VeryFast => 1.0,
                },
                knowledge.performance.compression_ratio as f32,
                knowledge.performance.quality_retention as f32,
                knowledge.compression_efficiency as f32,
                knowledge.browser_support.global_support_percentage as f32 / 100.0,
                (knowledge.year_released as f32 - 1990.0) / 35.0, // 归一化年份
                if knowledge.browser_support.chrome { 1.0 } else { 0.0 },
                
                // 兼容性和生态 (8维)
                if knowledge.browser_support.firefox { 1.0 } else { 0.0 },
                if knowledge.browser_support.safari { 1.0 } else { 0.0 },
                if knowledge.browser_support.edge { 1.0 } else { 0.0 },
                knowledge.strengths.len() as f32 / 10.0,
                knowledge.weaknesses.len() as f32 / 10.0,
                knowledge.use_cases.len() as f32 / 10.0,
                knowledge.capabilities.color_space_support.len() as f32 / 5.0,
                0.0, // 保留
                
                // 高级特征 (8维)
                // 计算格式的"现代性"分数
                if knowledge.year_released >= 2015 { 1.0 } else { 0.5 },
                // 计算格式的"通用性"分数
                if knowledge.browser_support.global_support_percentage >= 95.0 { 1.0 } else { 0.5 },
                // 计算格式的"效率"分数
                knowledge.compression_efficiency as f32 * knowledge.performance.quality_retention as f32,
                // 计算格式的"速度"分数
                (match knowledge.performance.encode_speed {
                    Speed::VeryFast => 1.0,
                    Speed::Fast => 0.8,
                    Speed::Medium => 0.6,
                    Speed::Slow => 0.4,
                    Speed::VerySlow => 0.2,
                } + match knowledge.performance.decode_speed {
                    Speed::VeryFast => 1.0,
                    Speed::Fast => 0.8,
                    Speed::Medium => 0.6,
                    Speed::Slow => 0.4,
                    Speed::VerySlow => 0.2,
                }) / 2.0,
                0.0, // 保留
                0.0, // 保留
                0.0, // 保留
                0.0, // 保留
            ]
        } else {
            vec![0.0; 32]
        }
    }
    
    /// 判断是否应该使用同格式优化
    /// 当用户禁用格式转换时，强制使用同格式优化
    pub fn should_use_same_format_optimization(
        &self,
        source_format: &str,
        target_format: Option<&str>,
        format_conversion_enabled: bool,
    ) -> bool {
        // 如果禁用了格式转换，必须使用同格式优化
        if !format_conversion_enabled {
            return true;
        }
        
        // 如果没有指定目标格式，使用同格式优化
        if target_format.is_none() {
            return true;
        }
        
        // 如果目标格式与源格式相同，使用同格式优化
        if let Some(target) = target_format
            && target == source_format {
                return true;
            }
        
        false
    }
    
    /// 建议格式升级 - 从旧格式升级到现代格式
    /// 返回: (是否应该升级, 推荐的新格式列表, 升级原因)
    pub fn suggest_format_upgrade(
        &self,
        source_format: &str,
    ) -> (bool, Vec<String>, String) {
        let source = match self.get_format(source_format) {
            Some(f) => f,
            None => return (false, vec![], String::new()),
        };
        
        // 检查是否是过时格式
        let is_outdated = source.year_released < 2010 || 
                         source.compression_efficiency < 0.70;
        
        if !is_outdated {
            return (false, vec![], String::new());
        }
        
        // 根据媒体类型推荐现代格式
        let (recommendations, reason) = match source.media_type {
            MediaType::Image => {
                if source_format == "jpeg" {
                    (
                        vec!["avif".to_string(), "jxl".to_string(), "webp".to_string()],
                        "JPEG is outdated. Modern formats offer 40-60% better compression with higher quality.".to_string()
                    )
                } else if source_format == "png" {
                    (
                        vec!["avif".to_string(), "jxl".to_string(), "webp".to_string()],
                        "PNG has poor compression. Modern formats offer lossless compression with 50-70% smaller files.".to_string()
                    )
                } else {
                    (vec![], String::new())
                }
            },
            MediaType::Video => {
                if source_format == "h264" {
                    (
                        vec!["h266".to_string(), "av1".to_string(), "h265".to_string()],
                        "H.264 is OUTDATED. Modern codecs offer 40-60% better compression with HDR support.".to_string()
                    )
                } else {
                    (vec![], String::new())
                }
            },
            MediaType::Audio => {
                if source_format == "mp3" {
                    (
                        vec!["opus".to_string(), "aac".to_string()],
                        "MP3 is outdated. Modern codecs offer 30-40% better compression with higher quality.".to_string()
                    )
                } else {
                    (vec![], String::new())
                }
            },
        };
        
        (!recommendations.is_empty(), recommendations, reason)
    }
    
    /// 获取推荐的现代格式（按优先级排序）
    /// 完全基于格式知识，零硬编码规则
    pub fn get_modern_format_recommendations(
        &self,
        media_type: MediaType,
        needs_hdr: bool,
        needs_alpha: bool,
    ) -> Vec<String> {
        let mut formats: Vec<_> = self.formats.values()
            .filter(|f| f.media_type == media_type)
            .filter(|f| !needs_hdr || f.capabilities.supports_hdr)
            .filter(|f| !needs_alpha || f.capabilities.supports_alpha)
            .collect();
        
        // 按现代性和效率排序
        formats.sort_by(|a, b| {
            let score_a = a.compression_efficiency * 
                         (if a.year_released >= 2015 { 1.2 } else { 1.0 });
            let score_b = b.compression_efficiency * 
                         (if b.year_released >= 2015 { 1.2 } else { 1.0 });
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        formats.into_iter()
            .map(|f| f.name.clone())
            .collect()
    }
}

/// 格式需求
#[derive(Debug, Clone)]
pub struct FormatRequirements {
    pub needs_alpha: bool,
    pub needs_animation: bool,
    pub needs_hdr: bool,
    pub compression_weight: f64,
    pub quality_weight: f64,
    pub compatibility_weight: f64,
    pub speed_weight: f64,
}

impl Default for FormatRequirements {
    fn default() -> Self {
        Self {
            needs_alpha: false,
            needs_animation: false,
            needs_hdr: false,
            compression_weight: 0.4,
            quality_weight: 0.4,
            compatibility_weight: 0.1,
            speed_weight: 0.1,
        }
    }
}

impl Default for FormatKnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_base_creation() {
        let kb = FormatKnowledgeBase::new();
        assert!(kb.get_format("avif").is_some());
        assert!(kb.get_format("jxl").is_some());
        assert!(kb.get_format("webp").is_some());
    }

    #[test]
    fn test_format_recommendation() {
        let kb = FormatKnowledgeBase::new();
        let req = FormatRequirements {
            needs_alpha: true,
            needs_animation: false,
            needs_hdr: false,
            compression_weight: 0.5,
            quality_weight: 0.5,
            compatibility_weight: 0.0,
            speed_weight: 0.0,
        };
        
        let recommendations = kb.recommend_for_requirements(&req);
        assert!(!recommendations.is_empty());
    }
}
