/**
 * ═══════════════════════════════════════════════════════════════
 * 🗑️ DEPRECATED: Legacy JS Conversion Logic
 * ═══════════════════════════════════════════════════════════════
 * 
 * ⚠️ 此文件包含已弃用的 JavaScript 转换逻辑
 * 
 * 状态: DEPRECATED (v2.1.0+)
 * 原因: 已迁移到 GO/Rust 架构
 * 保留: 仅作为紧急 fallback 和参考
 * 
 * 新架构:
 *   - 智能模式: GO AI 预测 + Rust 转换
 *   - 手动模式: Rust 转换
 *   - Fallback: 提示用户启动服务，而非使用低效的 JS 实现
 * 
 * ⚠️ 不要在生产环境中使用此文件！
 * ═══════════════════════════════════════════════════════════════
 */

log.warn('[PIXLY] 🗑️ Loading DEPRECATED legacy conversion module');
log.warn('[PIXLY] ⚠️ This module should NOT be used in production');
log.warn('[PIXLY] 💡 Please ensure GO/Rust services are running');

// 导出提示函数
window.PIXLY_LEGACY_CONVERSION = {
    available: false,
    deprecated: true,
    message: 'Legacy JS conversion is deprecated. Please start GO/Rust services.',
    
    /**
     * 显示服务未运行的友好提示
     */
    showServiceDownMessage() {
        const message = `
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     ⚠️  转换服务未运行                                       ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

转换失败：GO/Rust 服务未运行

请启动服务：
  1. GO AI Service:  ./quick_start_go_core.sh
  2. Rust Service:   ./start-rust-service.sh

服务端口：
  • GO AI:   localhost:50052
  • Rust:    localhost:8080

如需帮助，请查看文档或联系支持。
        `.trim();
        
        log.error(message);
        
        if (window.addLog) {
            window.addLog('❌ 转换服务未运行，请启动 GO/Rust 服务', 'error');
        }
        
        // 显示 Eagle 通知
        if (window.eagle?.notification) {
            window.eagle.notification.show({
                title: '⚠️ 转换服务未运行',
                description: '请启动 GO AI Service 和 Rust Service',
            });
        }
        
        throw new Error('Conversion services not available. Please start GO/Rust services.');
    }
};

log.warn('[PIXLY] 🗑️ Legacy conversion module loaded (fallback only)');
