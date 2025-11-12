/**
 * CLI Commands Module - 命令模块管理
 * 
 * 将原1629行的commands.rs重构为独立的命令模块
 * 每个命令拥有单独的文件和职责
 * 
 * 模块结构:
 * - convert.rs: 图像转换命令 (~350行)
 * - batch.rs: 批量处理命令 (~240行)
 * - info.rs: 文件信息命令 (~70行)
 * - analyze.rs: 分析命令 (~220行)
 * - eagle.rs: Eagle图库命令 (~260行)
 * - video.rs: 视频处理命令 (~240行)
 * - gif.rs: GIF优化命令 (~210行)
 * - detect.rs: 文件检测命令 (~180行)
 */

// 导出命令模块
pub mod convert;
pub mod batch;
pub mod video;
pub mod audio;
pub mod detect;
pub mod info;
pub mod gif;
pub mod analyze;
pub mod eagle;

// 导出便捷函数（向后兼容）
// Phase 46.12: 添加_command后缀以匹配cli/mod.rs的导入
pub use convert::handle as handle_convert_command;
pub use batch::handle as handle_batch_command;
pub use video::handle as handle_video_command;
pub use audio::handle as handle_audio_command;
pub use detect::handle as handle_detect_command;
pub use info::handle as handle_info_command;
pub use gif::handle as handle_gif_optimize_command;
// Phase 46.13: 保留未使用的导出以备将来使用
#[allow(unused_imports)]
pub use analyze::handle as handle_analyze_command;
#[allow(unused_imports)]
pub use eagle::handle as handle_eagle_command;
