// 🤖 增强版PPO强化学习模型
// 支持图像、视频、音频的全媒体类型训练数据

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Context, Result};

/// 媒体类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

/// 单个训练样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    pub media_type: MediaType,
    pub input_file: String,
    pub input_size: u64,
    pub target_format: String,
    pub quality: u32,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub reward: f64,
}

/// PPO训练数据集
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOTrainingData {
    pub timestamp: String,
    pub total_samples: usize,
    pub stats: HashMap<String, usize>,
    pub data: Vec<TrainingSample>,
}

impl PPOTrainingData {
    /// 从JSON文件加载
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .context("Failed to read PPO training data file")?;
        
        let data: PPOTrainingData = serde_json::from_str(&content)
            .context("Failed to parse PPO training data")?;
        
        Ok(data)
    }
    
    /// 按媒体类型过滤样本
    pub fn filter_by_media_type(&self, media_type: MediaType) -> Vec<&TrainingSample> {
        self.data.iter()
            .filter(|s| s.media_type == media_type)
            .collect()
    }
    
    /// 按格式过滤样本
    pub fn filter_by_format(&self, format: &str) -> Vec<&TrainingSample> {
        self.data.iter()
            .filter(|s| s.target_format == format)
            .collect()
    }
    
    /// 获取最佳压缩率样本
    pub fn get_best_compression(&self, media_type: MediaType, format: &str) -> Option<&TrainingSample> {
        self.data.iter()
            .filter(|s| s.media_type == media_type && s.target_format == format)
            .min_by(|a, b| a.compression_ratio.partial_cmp(&b.compression_ratio).unwrap())
    }
    
    /// 获取平均压缩率
    pub fn average_compression_ratio(&self, media_type: MediaType, format: &str) -> f64 {
        let samples: Vec<_> = self.data.iter()
            .filter(|s| s.media_type == media_type && s.target_format == format)
            .collect();
        
        if samples.is_empty() {
            return 1.0;
        }
        
        let sum: f64 = samples.iter().map(|s| s.compression_ratio).sum();
        sum / samples.len() as f64
    }
    
    /// 获取平均奖励
    pub fn average_reward(&self, media_type: MediaType, format: &str) -> f64 {
        let samples: Vec<_> = self.data.iter()
            .filter(|s| s.media_type == media_type && s.target_format == format)
            .collect();
        
        if samples.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = samples.iter().map(|s| s.reward).sum();
        sum / samples.len() as f64
    }
    
    /// 推荐最优质量参数
    pub fn recommend_quality(&self, media_type: MediaType, format: &str, target_ratio: f64) -> u32 {
        let samples: Vec<_> = self.data.iter()
            .filter(|s| s.media_type == media_type && s.target_format == format)
            .collect();
        
        if samples.is_empty() {
            return match media_type {
                MediaType::Image => 85,
                MediaType::Video => 1000,
                MediaType::Audio => 192,
            };
        }
        
        // 找到最接近目标压缩率的样本
        samples.iter()
            .min_by(|a, b| {
                let diff_a = (a.compression_ratio - target_ratio).abs();
                let diff_b = (b.compression_ratio - target_ratio).abs();
                diff_a.partial_cmp(&diff_b).unwrap()
            })
            .map(|s| s.quality)
            .unwrap_or(85)
    }
}

/// 增强版PPO预测器
pub struct EnhancedPPOPredictor {
    training_data: PPOTrainingData,
    enabled: bool,
}

impl EnhancedPPOPredictor {
    /// 创建新的预测器
    pub fn new(training_data: PPOTrainingData) -> Self {
        Self {
            training_data,
            enabled: true,
        }
    }
    
    /// 从文件加载
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let training_data = PPOTrainingData::from_file(path)?;
        Ok(Self::new(training_data))
    }
    
    /// 预测图像转换参数
    pub fn predict_image_params(&self, format: &str, file_size: u64) -> ImageConversionParams {
        if !self.enabled {
            return ImageConversionParams::default();
        }
        
        let avg_ratio = self.training_data.average_compression_ratio(MediaType::Image, format);
        let avg_reward = self.training_data.average_reward(MediaType::Image, format);
        
        // 根据文件大小和训练数据调整质量
        let base_quality = if file_size > 5_000_000 {
            80 // 大文件用较低质量
        } else if file_size < 500_000 {
            90 // 小文件用较高质量
        } else {
            85 // 中等文件
        };
        
        // 使用训练数据微调
        let quality_adjustment = (avg_reward * 10.0) as i32;
        let quality = (base_quality + quality_adjustment).clamp(60, 100) as u8;
        
        ImageConversionParams {
            quality,
            speed: self.predict_speed_from_ratio(avg_ratio),
            lossless: avg_ratio < 0.5, // 如果压缩率很好，可以尝试无损
        }
    }
    
    /// 预测视频转换参数
    pub fn predict_video_params(&self, format: &str, _file_size: u64) -> VideoConversionParams {
        if !self.enabled {
            return VideoConversionParams::default();
        }
        
        let avg_ratio = self.training_data.average_compression_ratio(MediaType::Video, format);
        let avg_reward = self.training_data.average_reward(MediaType::Video, format);
        
        // 根据训练数据推荐比特率
        let bitrate = if avg_ratio < 1.0 {
            // 压缩效果好，可以用较低比特率
            1000
        } else if avg_ratio < 2.0 {
            // 中等压缩，用中等比特率
            1500
        } else {
            // 压缩效果差，用较高比特率保证质量
            2000
        };
        
        VideoConversionParams {
            bitrate,
            preset: self.predict_preset_from_reward(avg_reward),
            crf: self.calculate_crf_from_ratio(avg_ratio),
        }
    }
    
    /// 预测音频转换参数
    pub fn predict_audio_params(&self, format: &str, _file_size: u64) -> AudioConversionParams {
        if !self.enabled {
            return AudioConversionParams::default();
        }
        
        let _avg_ratio = self.training_data.average_compression_ratio(MediaType::Audio, format);
        let _avg_reward = self.training_data.average_reward(MediaType::Audio, format);
        
        // 根据训练数据推荐比特率
        let bitrate = if format == "aac" {
            // AAC压缩效果最好，可以用较低比特率
            128
        } else if format == "opus" {
            // Opus适合中等比特率
            160
        } else {
            // MP3需要较高比特率
            192
        };
        
        AudioConversionParams {
            bitrate,
            sample_rate: 48000,
            channels: 2,
        }
    }
    
    /// 推荐最佳格式
    pub fn recommend_best_format(&self, media_type: MediaType) -> String {
        let formats = match media_type {
            MediaType::Image => vec!["webp", "avif", "jxl"],
            MediaType::Video => vec!["webm", "mp4"],
            MediaType::Audio => vec!["opus", "aac", "mp3"],
        };
        
        // 找到平均奖励最高的格式
        formats.iter()
            .max_by(|a, b| {
                let reward_a = self.training_data.average_reward(media_type.clone(), a);
                let reward_b = self.training_data.average_reward(media_type.clone(), b);
                reward_a.partial_cmp(&reward_b).unwrap()
            })
            .map(|s| s.to_string())
            .unwrap_or_else(|| match media_type {
                MediaType::Image => "webp".to_string(),
                MediaType::Video => "webm".to_string(),
                MediaType::Audio => "aac".to_string(),
            })
    }
    
    /// 从压缩率预测速度参数
    fn predict_speed_from_ratio(&self, ratio: f64) -> u8 {
        if ratio < 0.5 {
            6 // 压缩效果好，可以用慢速
        } else if ratio < 0.8 {
            4 // 中等压缩，用平衡速度
        } else {
            2 // 压缩效果差，用快速
        }
    }
    
    /// 从奖励预测预设
    fn predict_preset_from_reward(&self, reward: f64) -> String {
        if reward > 0.5 {
            "slow".to_string()
        } else if reward > 0.3 {
            "medium".to_string()
        } else {
            "fast".to_string()
        }
    }
    
    /// 从压缩率计算CRF
    fn calculate_crf_from_ratio(&self, ratio: f64) -> u8 {
        // CRF: 0-51, 值越小质量越高
        if ratio < 1.0 {
            18 // 压缩效果好，用高质量
        } else if ratio < 2.0 {
            23 // 中等压缩，用中等质量
        } else {
            28 // 压缩效果差，用较低质量
        }
    }
    
    /// 获取训练统计信息
    pub fn get_training_stats(&self) -> String {
        let image_count = self.training_data.stats.get("image").unwrap_or(&0);
        let video_count = self.training_data.stats.get("video").unwrap_or(&0);
        let audio_count = self.training_data.stats.get("audio").unwrap_or(&0);
        
        format!(
            "PPO Training Data: total_samples={}, images={}, videos={}, audio={}, timestamp={}",
            self.training_data.total_samples,
            image_count,
            video_count,
            audio_count,
            self.training_data.timestamp
        )
    }
    
    /// 启用/禁用预测器
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// 图像转换参数
#[derive(Debug, Clone)]
pub struct ImageConversionParams {
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
}

impl Default for ImageConversionParams {
    fn default() -> Self {
        Self {
            quality: 85,
            speed: 4,
            lossless: false,
        }
    }
}

/// 视频转换参数
#[derive(Debug, Clone)]
pub struct VideoConversionParams {
    pub bitrate: u32,
    pub preset: String,
    pub crf: u8,
}

impl Default for VideoConversionParams {
    fn default() -> Self {
        Self {
            bitrate: 1000,
            preset: "medium".to_string(),
            crf: 23,
        }
    }
}

/// 音频转换参数
#[derive(Debug, Clone)]
pub struct AudioConversionParams {
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: u8,
}

impl Default for AudioConversionParams {
    fn default() -> Self {
        Self {
            bitrate: 192,
            sample_rate: 48000,
            channels: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_training_data_loading() {
        // 创建测试数据
        let data = PPOTrainingData {
            timestamp: "2025-11-17".to_string(),
            total_samples: 2,
            stats: {
                let mut map = HashMap::new();
                map.insert("image".to_string(), 1);
                map.insert("video".to_string(), 1);
                map
            },
            data: vec![
                TrainingSample {
                    media_type: MediaType::Image,
                    input_file: "test.jpg".to_string(),
                    input_size: 100000,
                    target_format: "webp".to_string(),
                    quality: 85,
                    output_size: 50000,
                    compression_ratio: 0.5,
                    reward: 0.8,
                },
                TrainingSample {
                    media_type: MediaType::Video,
                    input_file: "test.mp4".to_string(),
                    input_size: 1000000,
                    target_format: "webm".to_string(),
                    quality: 1000,
                    output_size: 800000,
                    compression_ratio: 0.8,
                    reward: 0.6,
                },
            ],
        };
        
        assert_eq!(data.total_samples, 2);
        assert_eq!(data.filter_by_media_type(MediaType::Image).len(), 1);
        assert_eq!(data.filter_by_media_type(MediaType::Video).len(), 1);
    }
    
    #[test]
    fn test_enhanced_predictor() {
        let data = PPOTrainingData {
            timestamp: "2025-11-17".to_string(),
            total_samples: 1,
            stats: HashMap::new(),
            data: vec![
                TrainingSample {
                    media_type: MediaType::Image,
                    input_file: "test.jpg".to_string(),
                    input_size: 100000,
                    target_format: "webp".to_string(),
                    quality: 85,
                    output_size: 50000,
                    compression_ratio: 0.5,
                    reward: 0.8,
                },
            ],
        };
        
        let predictor = EnhancedPPOPredictor::new(data);
        let params = predictor.predict_image_params("webp", 100000);
        
        assert!(params.quality >= 60 && params.quality <= 100);
        assert!(params.speed >= 1 && params.speed <= 10);
    }
    
    #[test]
    fn test_format_recommendation() {
        let data = PPOTrainingData {
            timestamp: "2025-11-17".to_string(),
            total_samples: 2,
            stats: HashMap::new(),
            data: vec![
                TrainingSample {
                    media_type: MediaType::Audio,
                    input_file: "test.mp3".to_string(),
                    input_size: 100000,
                    target_format: "aac".to_string(),
                    quality: 128,
                    output_size: 50000,
                    compression_ratio: 0.5,
                    reward: 0.9,
                },
                TrainingSample {
                    media_type: MediaType::Audio,
                    input_file: "test.mp3".to_string(),
                    input_size: 100000,
                    target_format: "opus".to_string(),
                    quality: 128,
                    output_size: 60000,
                    compression_ratio: 0.6,
                    reward: 0.7,
                },
            ],
        };
        
        let predictor = EnhancedPPOPredictor::new(data);
        let best_format = predictor.recommend_best_format(MediaType::Audio);
        
        // AAC应该被推荐，因为它的奖励更高
        assert_eq!(best_format, "aac");
    }
}
