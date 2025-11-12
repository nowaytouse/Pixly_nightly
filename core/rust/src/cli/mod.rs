/**
 * CLI Module - 命令行接口模块
 * 
 * 提供完整的CLI功能:
 * - 命令解析和分发
 * - 转换逻辑
 * - 帮助文档
 * - AI集成命令 (Phase 33)
 */

pub mod commands;
pub mod help;
pub mod conversion;
pub mod ai_commands;  // Phase 33: AI集成
pub mod analyze; // 🔥 Phase 40.7.15: 文件分析命令
pub mod dependency_checker;  // 🔥 Phase 40.19: 外部依赖检查
// 注意：Eagle适配器位于 converter::eagle_adapter (避免重复)

pub use commands::{
    handle_convert_command, 
    handle_batch_command, 
    handle_info_command, 
    handle_analyze_command, 
    handle_eagle_command, 
    handle_gif_optimize_command,
    handle_video_command,  // 🎬 Phase 40.24: 视频转换命令
    handle_detect_command,  // 🔥 Phase 45.3: AI文件类型检测命令
};
pub use help::{show_tui_manual, show_quick_help};
