/**
 * Eagle插件错误处理增强
 * Phase 47.18 (O-005): 统一错误处理和用户提示
 */

(function() {
    'use strict';

    /**
     * 错误分类和处理策略
     */
    const ErrorHandler = {
        /**
         * 错误分类
         */
        ErrorTypes: {
            // 环境错误
            RUST_CLI_NOT_FOUND: 'RUST_CLI_NOT_FOUND',
            JXL_NOT_INSTALLED: 'JXL_NOT_INSTALLED',
            JXL_VERSION_MISMATCH: 'JXL_VERSION_MISMATCH',
            AI_SERVICE_UNAVAILABLE: 'AI_SERVICE_UNAVAILABLE',
            
            // 文件错误
            FILE_NOT_FOUND: 'FILE_NOT_FOUND',
            FILE_TOO_LARGE: 'FILE_TOO_LARGE',
            UNSUPPORTED_FORMAT: 'UNSUPPORTED_FORMAT',
            
            // 转换错误
            CONVERSION_FAILED: 'CONVERSION_FAILED',
            QUALITY_CHECK_FAILED: 'QUALITY_CHECK_FAILED',
            
            // 网络错误
            NETWORK_ERROR: 'NETWORK_ERROR',
            TIMEOUT_ERROR: 'TIMEOUT_ERROR',
        },

        /**
         * 处理错误并显示友好提示
         */
        async handle(error, context = {}) {
            const log = window.pixlyLog || console;
            const eagle = window.eagle;
            
            // 记录详细错误
            log.error('[ErrorHandler]', error.message, error.stack);
            
            // 获取错误类型和友好信息
            const { type, title, message, actions } = this.classify(error);
            
            // 显示用户提示
            if (eagle && eagle.modal) {
                await this.showUserDialog(title, message, actions);
            } else {
                console.error(`${title}\n${message}`);
            }
            
            // 上报错误统计
            this.reportError(type, error, context);
            
            return { type, handled: true };
        },

        /**
         * 错误分类
         */
        classify(error) {
            const code = error.code || '';
            const message = error.message || '';
            
            // JXL编码器问题
            if (code === 'JXL_NOT_INSTALLED' || message.includes('cjxl') && message.includes('not found')) {
                return {
                    type: this.ErrorTypes.JXL_NOT_INSTALLED,
                    title: '❌ JXL编码器未安装',
                    message: 'JXL是新一代图像格式，需要安装编码器才能使用。',
                    actions: [
                        { label: '查看安装指南', action: 'show_jxl_guide' },
                        { label: '暂时使用其他格式', action: 'use_alternative' }
                    ]
                };
            }
            
            // AI服务问题
            if (code === 'AI_SERVICE_UNAVAILABLE' || message.includes('AI服务')) {
                return {
                    type: this.ErrorTypes.AI_SERVICE_UNAVAILABLE,
                    title: '🤖 AI服务未启动',
                    message: 'AI服务能够智能优化转换参数，提升转换质量。',
                    actions: [
                        { label: '查看启动方法', action: 'show_ai_guide' },
                        { label: '使用默认参数', action: 'use_defaults' }
                    ]
                };
            }
            
            // Rust CLI问题
            if (message.includes('Rust CLI not available')) {
                return {
                    type: this.ErrorTypes.RUST_CLI_NOT_FOUND,
                    title: '⚠️ 核心转换器未找到',
                    message: 'Pixly需要Rust转换器才能正常工作。',
                    actions: [
                        { label: '检查安装', action: 'check_installation' },
                        { label: '重新安装', action: 'reinstall' }
                    ]
                };
            }
            
            // 文件过大
            if (message.includes('too large') || message.includes('File too large')) {
                return {
                    type: this.ErrorTypes.FILE_TOO_LARGE,
                    title: '📁 文件过大',
                    message: '该文件超过了处理限制，建议先缩小尺寸。',
                    actions: [
                        { label: '缩小后重试', action: 'resize_and_retry' },
                        { label: '跳过此文件', action: 'skip' }
                    ]
                };
            }
            
            // 超时错误
            if (message.includes('timeout') || code === 'TIMEOUT_ERROR') {
                return {
                    type: this.ErrorTypes.TIMEOUT_ERROR,
                    title: '⏱️ 处理超时',
                    message: '处理时间过长，可能是文件过大或服务器繁忙。',
                    actions: [
                        { label: '重试', action: 'retry' },
                        { label: '减小批量大小', action: 'reduce_batch' }
                    ]
                };
            }
            
            // 默认错误
            return {
                type: this.ErrorTypes.CONVERSION_FAILED,
                title: '❌ 转换失败',
                message: message.substring(0, 200),
                actions: [
                    { label: '重试', action: 'retry' },
                    { label: '查看日志', action: 'show_logs' }
                ]
            };
        },

        /**
         * 显示用户对话框
         */
        async showUserDialog(title, message, actions) {
            const eagle = window.eagle;
            if (!eagle || !eagle.modal) return;
            
            // 构建按钮
            const buttons = actions.map(action => ({
                text: action.label,
                value: action.action
            }));
            
            // 添加关闭按钮
            buttons.push({
                text: '关闭',
                value: 'close',
                style: 'secondary'
            });
            
            // 显示对话框
            const result = await eagle.modal.show({
                title,
                message,
                buttons
            });
            
            // 处理用户选择
            await this.handleUserAction(result.value);
        },

        /**
         * 处理用户操作
         */
        async handleUserAction(action) {
            const eagle = window.eagle;
            
            switch (action) {
                case 'show_jxl_guide':
                    // 打开JXL安装指南
                    if (eagle && eagle.shell) {
                        eagle.shell.openExternal('https://github.com/libjxl/libjxl#installation');
                    }
                    break;
                    
                case 'show_ai_guide':
                    // 显示AI服务启动指南
                    const aiGuide = `
启动AI服务步骤：

1. 打开终端
2. 进入Pixly目录：cd Pixly_Nightly
3. 启动服务：python3 tools/pixly_http_server.py
4. 等待服务启动完成（显示"Running on http://localhost:50052"）
5. 重新尝试转换
                    `.trim();
                    
                    if (eagle && eagle.modal) {
                        await eagle.modal.show({
                            title: 'AI服务启动指南',
                            message: aiGuide,
                            buttons: [{ text: '知道了', value: 'ok' }]
                        });
                    }
                    break;
                    
                case 'check_installation':
                    // 检查Rust CLI安装
                    if (window.PIXLY?.DependencyChecker) {
                        await window.PIXLY.DependencyChecker.checkAndReport();
                    }
                    break;
                    
                case 'show_logs':
                    // 显示日志
                    if (window.pixlyLog) {
                        window.pixlyLog.showLogs();
                    }
                    break;
                    
                case 'retry':
                    // 触发重试事件
                    if (window.PIXLY?.EventBus) {
                        window.PIXLY.EventBus.emit('conversion:retry');
                    }
                    break;
                    
                default:
                    // 其他操作
                    break;
            }
        },

        /**
         * 上报错误统计
         */
        reportError(type, error, context) {
            // 收集错误信息用于改进
            const errorInfo = {
                type,
                message: error.message,
                code: error.code,
                timestamp: new Date().toISOString(),
                context,
                version: window.PIXLY?.VERSION || 'unknown'
            };
            
            // 保存到本地统计
            if (window.PIXLY?.Stats) {
                window.PIXLY.Stats.recordError(errorInfo);
            }
            
            // 记录到日志
            const log = window.pixlyLog || console;
            log.debug('[ErrorHandler] Error recorded:', errorInfo);
        }
    };

    // 导出到全局
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.ErrorHandler = ErrorHandler;
    
    // 全局错误捕获
    window.addEventListener('unhandledrejection', event => {
        const log = window.pixlyLog || console;
        log.error('[Global] Unhandled promise rejection:', event.reason);
        ErrorHandler.handle(event.reason, { source: 'unhandledrejection' });
    });

    // 初始化成功
    const log = window.pixlyLog || console;
    log.info('[ErrorHandler] ✅ Error handler initialized');

})();
