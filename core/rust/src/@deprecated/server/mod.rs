//! 🚀 Pixly v3.1 本地处理核心
//! 
//! 完全本地化处理模块，不包含任何网络功能
//! 确保所有处理都在本地进行，无外部依赖

use anyhow::Result;
use log::{info, warn, error};
use serde::{Serialize, Deserialize};

// 核心本地处理服务
mod local_processor;

#[allow(clippy::module_inception)]
mod server;

// 仅导出本地处理功能，无HTTP功能
pub use local_processor::{LocalProcessor, ProcessingResult};
pub use server::{LocalProcessingConfig};
