//! 动画检测模块
//! 
//! 检测GIF和其他格式是否为动画

use anyhow::{Result, Context};
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

/// 检测GIF是否为动画（包含多帧）
pub fn is_animated_gif<P: AsRef<Path>>(path: P) -> Result<bool> {
    use image::codecs::gif::GifDecoder;
    use image::AnimationDecoder;
    
    let path = path.as_ref();
    
    // 检查文件扩展名
    if let Some(ext) = path.extension() {
        if ext.to_string_lossy().to_lowercase() != "gif" {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }
    
    // 打开文件
    let file = File::open(path)
        .with_context(|| format!("Failed to open GIF file: {:?}", path))?;
    let reader = BufReader::new(file);
    
    // 创建GIF解码器
    let decoder = GifDecoder::new(reader)
        .with_context(|| format!("Failed to decode GIF: {:?}", path))?;
    
    // 获取帧数
    let frames = decoder.into_frames();
    let frame_count = frames.count();
    
    // 多于1帧即为动画
    Ok(frame_count > 1)
}

/// 获取GIF帧数
pub fn get_gif_frame_count<P: AsRef<Path>>(path: P) -> Result<usize> {
    use image::codecs::gif::GifDecoder;
    use image::AnimationDecoder;
    
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;
    Ok(decoder.into_frames().count())
}

/// 获取GIF帧率
pub fn get_gif_fps<P: AsRef<Path>>(path: P) -> Result<f32> {
    use image::codecs::gif::GifDecoder;
    use image::AnimationDecoder;
    
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;
    
    let mut total_delay = 0u32;
    let mut frame_count = 0;
    
    for frame in decoder.into_frames().flatten() {
        let delay = frame.delay();
        total_delay += delay.numer_denom_ms().0;
        frame_count += 1;
    }
    
    if frame_count > 0 && total_delay > 0 {
        // 计算平均帧率
        let avg_delay_ms = total_delay as f32 / frame_count as f32;
        Ok(1000.0 / avg_delay_ms)
    } else {
        Ok(10.0) // 默认10fps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_detection() {
        // 测试基本功能
        assert!(is_animated_gif("nonexistent.gif").is_err());
    }
}
