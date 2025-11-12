/**
 * Animation Detector
 * 
 * 检测GIF是否为动画
 */
use anyhow::{Result, Context};
use std::path::Path;
use std::fs::File;

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
    
    // 创建GIF解码器
    let decoder = GifDecoder::new(file)
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
    let decoder = GifDecoder::new(file)?;
    Ok(decoder.into_frames().count())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gif_detection() {
        // 这里可以添加测试用例
        // 需要准备测试GIF文件
    }
}
