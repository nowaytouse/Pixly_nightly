/// 🔥 Pixly CLI - 主程序入口
/// 
/// Phase 3: CLI 集成
/// - 使用 Phase 2 批量处理系统
/// - 统一日志管理
/// - 完整的命令行参数解析

use anyhow::Result;

fn main() -> Result<()> {
    pixly_kernel::cli_main::run()
}
