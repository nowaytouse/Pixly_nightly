//! 阶段 1: 图像信息读取模块
//! 
//! 纯只读操作，零风险

pub mod image;

pub use image::{ImageInfo, read_image_info, detect_format, is_animated};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // 确保模块正确导出
        println!("info module loaded");
    }
}
