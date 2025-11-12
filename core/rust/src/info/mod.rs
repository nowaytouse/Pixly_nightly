//! 阶段 1: 图像信息读取模块
//! 
//! 纯只读操作，零风险

pub mod image;

pub use image::{ImageInfo, read_image_info, detect_format, is_animated};
