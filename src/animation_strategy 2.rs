// 🎬 动画转换策略系统
// 智能选择最优的动画处理策略

use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// 动画转换策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationStrategy {
    /// FFmpeg处理（大帧数GIF）
    FFmpegBased,
    /// 帧拼接（小帧数动画）
    FrameStitching,
    /// 直接转换（静态或极少帧）
    DirectConversion,
    /// 输出为视频
    VideoOutput,
}

/// 动画信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub frame_count: u32,
    pub fps: f32,
    pub duration_secs: f32,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub has_alpha: bool,  // 是否有透明通道 - 关键！
}

/// 动画保留配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationPreservation {
    /// 保留原始帧数（默认true）
    pub preserve_frame_count: bool,
    /// 保留原始FPS（默认true）
    pub preserve_fps: bool,
    /// 原始帧数
    pub original_frame_count: u32,
    /// 原始FPS
    pub original_fps: f32,
    /// 目标FPS（None表示使用原始FPS）
    pub target_fps: Option<f32>,
}

impl Default for AnimationPreservation {
    fn default() -> Self {
        Self {
            preserve_frame_count: true,  // 默认保留帧数
            preserve_fps: true,           // 默认保留FPS
            original_frame_count: 0,
            original_fps: 0.0,
            target_fps: None,             // 默认使用原始FPS
        }
    }
}

/// 动画策略选择器
pub struct AnimationStrategySelector;

impl AnimationStrategySelector {
    /// 选择最优策略
    /// 基于实际文件特征，而非格式名称！
    /// 策略完全由文件内容决定：帧数、透明度、分辨率
    pub fn select_strategy(
        info: &AnimationInfo,
        _target_format: &str,  // 不再使用！格式不决定策略！
        output_as_video: bool,
    ) -> AnimationStrategy {
        // 如果明确要求输出为视频
        if output_as_video {
            return AnimationStrategy::VideoOutput;
        }
        
        // 静态图像或单帧 - 基于实际帧数
        if info.frame_count <= 1 {
            return AnimationStrategy::DirectConversion;
        }
        
        // 策略完全基于文件实际特征，不信任格式！
        
        // 动画+透明度：需要特殊处理保证透明度质量
        if info.has_alpha {
            if info.frame_count > 50 {
                // 大帧数透明动画：FFmpeg处理
                AnimationStrategy::FFmpegBased
            } else {
                // 小帧数透明动画：帧拼接保证质量
                AnimationStrategy::FrameStitching
            }
        } else {
            // 无透明度：按帧数和分辨率选择
            let pixel_count = info.width * info.height;
            let is_high_res = pixel_count > 1920 * 1080;
            
            if info.frame_count > 100 || (info.frame_count > 50 && is_high_res) {
                // 大帧数或高分辨率：FFmpeg处理
                AnimationStrategy::FFmpegBased
            } else if info.frame_count > 10 {
                // 中等帧数：帧拼接
                AnimationStrategy::FrameStitching
            } else {
                // 小帧数：直接转换
                AnimationStrategy::DirectConversion
            }
        }
    }
    
    /// 计算推荐的目标FPS
    pub fn recommend_target_fps(info: &AnimationInfo, strategy: AnimationStrategy) -> f32 {
        match strategy {
            AnimationStrategy::FFmpegBased => {
                // FFmpeg处理，保持原始FPS（不限制）
                info.fps
            }
            AnimationStrategy::FrameStitching => {
                // 帧拼接，保持原始FPS
                info.fps
            }
            AnimationStrategy::DirectConversion => {
                // 直接转换，保持原始FPS
                info.fps
            }
            AnimationStrategy::VideoOutput => {
                // 视频输出，保持原始FPS（不限制）
                info.fps
            }
        }
    }
    
    /// 估算处理时间
    pub fn estimate_processing_time(info: &AnimationInfo, strategy: AnimationStrategy) -> f64 {
        let base_time = (info.frame_count as f64) * 0.1; // 每帧0.1秒
        
        match strategy {
            AnimationStrategy::FFmpegBased => base_time * 0.5,      // FFmpeg快
            AnimationStrategy::FrameStitching => base_time * 1.5,   // 帧拼接慢
            AnimationStrategy::DirectConversion => base_time * 0.8, // 直接转换中等
            AnimationStrategy::VideoOutput => base_time * 0.6,      // 视频输出较快
        }
    }
    
    /// 生成策略说明
    pub fn get_strategy_description(strategy: AnimationStrategy) -> &'static str {
        match strategy {
            AnimationStrategy::FFmpegBased => 
                "使用FFmpeg处理大帧数动画，性能最优",
            AnimationStrategy::FrameStitching => 
                "使用帧拼接处理中等帧数动画，质量最优",
            AnimationStrategy::DirectConversion => 
                "直接转换小帧数动画，速度最快",
            AnimationStrategy::VideoOutput => 
                "输出为视频格式，兼容性最好",
        }
    }
}

/// 动画转视频转换器
pub struct AnimationToVideoConverter;

impl AnimationToVideoConverter {
    /// 将动画转换为视频
    pub fn convert_to_video(
        input: &Path,
        output: &Path,
        info: &AnimationInfo,
        preservation: &AnimationPreservation,
    ) -> Result<()> {
        use std::process::Command;
        
        let target_fps = preservation.target_fps.unwrap_or(info.fps);
        
        // 使用FFmpeg转换
        let output_result = Command::new("ffmpeg")
            .arg("-i").arg(input)
            .arg("-r").arg(target_fps.to_string())
            .arg("-c:v").arg("libx264")
            .arg("-preset").arg("medium")
            .arg("-crf").arg("23")
            .arg("-pix_fmt").arg("yuv420p")
            .arg("-y")
            .arg(output)
            .output()
            .context("Failed to execute FFmpeg")?;
        
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            anyhow::bail!("FFmpeg conversion failed: {}", stderr);
        }
        
        Ok(())
    }
    
    /// 检查FFmpeg是否可用
    pub fn is_ffmpeg_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strategy_selection() {
        let info = AnimationInfo {
            frame_count: 150,
            fps: 30.0,
            duration_secs: 5.0,
            width: 800,
            height: 600,
            file_size: 5_000_000,
            has_alpha: false,
        };
        
        let strategy = AnimationStrategySelector::select_strategy(&info, "avif", false);
        assert_eq!(strategy, AnimationStrategy::FFmpegBased);
    }
    
    #[test]
    fn test_small_frame_strategy() {
        let info = AnimationInfo {
            frame_count: 5,
            fps: 10.0,
            duration_secs: 0.5,
            width: 400,
            height: 300,
            file_size: 500_000,
            has_alpha: false,
        };
        
        let strategy = AnimationStrategySelector::select_strategy(&info, "webp", false);
        assert_eq!(strategy, AnimationStrategy::DirectConversion);
    }
    
    #[test]
    fn test_video_output_strategy() {
        let info = AnimationInfo {
            frame_count: 50,
            fps: 24.0,
            duration_secs: 2.0,
            width: 1920,
            height: 1080,
            file_size: 10_000_000,
            has_alpha: false,
        };
        
        // 明确要求输出为视频
        let strategy = AnimationStrategySelector::select_strategy(&info, "mp4", true);
        assert_eq!(strategy, AnimationStrategy::VideoOutput);
        
        // 不要求视频输出时，基于实际特征选择
        let strategy2 = AnimationStrategySelector::select_strategy(&info, "mp4", false);
        assert!(matches!(strategy2, AnimationStrategy::FFmpegBased | AnimationStrategy::FrameStitching));
    }
    
    #[test]
    fn test_fps_recommendation() {
        let info = AnimationInfo {
            frame_count: 100,
            fps: 60.0,
            duration_secs: 1.67,
            width: 1920,
            height: 1080,
            file_size: 8_000_000,
            has_alpha: false,
        };
        
        let fps = AnimationStrategySelector::recommend_target_fps(
            &info,
            AnimationStrategy::FFmpegBased
        );
        assert_eq!(fps, 60.0); // 保持原始FPS
    }
    
    #[test]
    fn test_static_image() {
        let info = AnimationInfo {
            frame_count: 1,
            fps: 0.0,
            duration_secs: 0.0,
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            has_alpha: false,
        };
        
        let strategy = AnimationStrategySelector::select_strategy(&info, "avif", false);
        assert_eq!(strategy, AnimationStrategy::DirectConversion);
    }
    
    #[test]
    fn test_transparent_animation_strategy() {
        // 测试透明动画的策略选择
        let info_with_alpha = AnimationInfo {
            frame_count: 60,
            fps: 30.0,
            duration_secs: 2.0,
            width: 800,
            height: 600,
            file_size: 3_000_000,
            has_alpha: true,  // 有透明通道
        };
        
        // 透明动画应该使用FFmpeg或FrameStitching
        let strategy = AnimationStrategySelector::select_strategy(&info_with_alpha, "webp", false);
        assert!(matches!(strategy, AnimationStrategy::FFmpegBased | AnimationStrategy::FrameStitching));
    }
}
