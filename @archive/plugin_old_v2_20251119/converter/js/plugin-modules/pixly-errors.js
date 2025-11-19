/**
 * 🚨 Pixly统一错误码系统 (JavaScript)
 * 
 * Phase 46.8: 三端统一的错误处理
 * 核心原则：响亮报错 > 静默降级
 */

/**
 * 错误严重级别
 */
const ErrorSeverity = {
    CRITICAL: 'CRITICAL',
    ERROR: 'ERROR',
    WARNING: 'WARNING',
    INFO: 'INFO',
};

/**
 * Pixly统一错误类
 */
class PixlyError extends Error {
    /**
     * 创建Pixly错误
     * @param {string} code - 错误码 (PIXLY-UI-VAL-001)
     * @param {string} message - 错误消息（技术详情）
     * @param {string} userMessage - 用户友好消息
     * @param {string} severity - 错误严重级别
     */
    constructor(code, message, userMessage, severity = ErrorSeverity.ERROR) {
        super(`${code}: ${message}`);
        this.name = 'PixlyError';
        this.code = code;
        this.originalMessage = message;
        this.userMessage = userMessage;
        this.severity = severity;
        this.context = null;
        this.timestamp = new Date().toISOString();
    }

    /**
     * 添加上下文信息
     * @param {Object} context - 上下文数据
     * @returns {PixlyError}
     */
    withContext(context) {
        this.context = context;
        return this;
    }

    /**
     * 是否应该阻断流程
     * @returns {boolean}
     */
    shouldBlock() {
        return this.severity === ErrorSeverity.ERROR || 
               this.severity === ErrorSeverity.CRITICAL;
    }

    /**
     * 转换为日志格式
     * @returns {Object}
     */
    toLogFormat() {
        return {
            timestamp: this.timestamp,
            level: this.severity,
            layer: 'UI',
            component: 'validation',
            message: this.originalMessage,
            code: this.code,
            context: this.context,
            user_message: this.userMessage,
        };
    }

    /**
     * 转换为JSON
     * @returns {Object}
     */
    toJSON() {
        return {
            code: this.code,
            message: this.originalMessage,
            userMessage: this.userMessage,
            severity: this.severity,
            context: this.context,
            timestamp: this.timestamp,
        };
    }
}

/**
 * 错误码常量
 */
const ErrorCodes = {
    // ========== 参数验证错误 (VAL) ==========
    
    /** 参数超出范围 */
    VAL_OUT_OF_RANGE: 'PIXLY-UI-VAL-001',
    
    /** 参数类型错误 */
    VAL_TYPE_ERROR: 'PIXLY-UI-VAL-002',
    
    /** 必填参数缺失 */
    VAL_MISSING_REQUIRED: 'PIXLY-UI-VAL-003',
    
    /** 参数组合冲突 */
    VAL_CONFLICT: 'PIXLY-UI-VAL-004',
    
    /** AI置信度过低 */
    VAL_LOW_CONFIDENCE: 'PIXLY-UI-VAL-005',
    
    /** 参数完整性验证失败 */
    VAL_INTEGRITY_FAILED: 'PIXLY-UI-VAL-006',
    
    // ========== 网络错误 (NET) ==========
    
    /** Rust服务不可用 */
    NET_SERVICE_DOWN: 'PIXLY-UI-NET-003',
    
    /** 请求超时 */
    NET_TIMEOUT: 'PIXLY-UI-NET-004',
};

/**
 * 错误构建器
 */
class ErrorBuilder {
    /**
     * 参数超出范围错误
     */
    static valOutOfRange(param, value, expected) {
        return new PixlyError(
            ErrorCodes.VAL_OUT_OF_RANGE,
            `${param} 参数超出范围: ${value} (应为 ${expected})`,
            `参数设置有误，请检查 ${param} 的值（应为 ${expected}）`,
            ErrorSeverity.ERROR
        ).withContext({ parameter: param, value, expected });
    }

    /**
     * 必填参数缺失
     */
    static valMissingRequired(param) {
        return new PixlyError(
            ErrorCodes.VAL_MISSING_REQUIRED,
            `必填参数缺失: ${param}`,
            `缺少必要参数：${param}`,
            ErrorSeverity.ERROR
        ).withContext({ parameter: param });
    }

    /**
     * 参数组合冲突
     */
    static valConflict(reason) {
        return new PixlyError(
            ErrorCodes.VAL_CONFLICT,
            `参数冲突: ${reason}`,
            `参数设置冲突：${reason}`,
            ErrorSeverity.ERROR
        ).withContext({ reason });
    }

    /**
     * AI置信度过低（警告）
     */
    static valLowConfidence(confidence, threshold) {
        return new PixlyError(
            ErrorCodes.VAL_LOW_CONFIDENCE,
            `AI置信度过低: ${(confidence * 100).toFixed(1)}% (阈值: ${(threshold * 100).toFixed(1)}%)`,
            'AI预测不够确定，建议手动设置参数',
            ErrorSeverity.WARNING
        ).withContext({ confidence, threshold });
    }

    /**
     * 参数完整性验证失败
     */
    static valIntegrityFailed(details) {
        return new PixlyError(
            ErrorCodes.VAL_INTEGRITY_FAILED,
            '参数在传递过程中被修改',
            '参数传递异常，请重试',
            ErrorSeverity.ERROR
        ).withContext({ details });
    }

    /**
     * Rust服务不可用
     */
    static netServiceDown() {
        return new PixlyError(
            ErrorCodes.NET_SERVICE_DOWN,
            '无法连接到Rust转换服务',
            '转换服务未启动，请检查服务状态',
            ErrorSeverity.ERROR
        );
    }

    /**
     * 请求超时
     */
    static netTimeout(timeoutMs) {
        return new PixlyError(
            ErrorCodes.NET_TIMEOUT,
            `请求超时: ${timeoutMs}ms`,
            '请求超时，请重试',
            ErrorSeverity.ERROR
        ).withContext({ timeout_ms: timeoutMs });
    }
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        PixlyError,
        ErrorCodes,
        ErrorSeverity,
        ErrorBuilder,
    };
}

// 全局导出（用于Eagle插件）
if (typeof window !== 'undefined') {
    window.PIXLY = window.PIXLY || {};
    window.PIXLY.PixlyError = PixlyError;
    window.PIXLY.ErrorCodes = ErrorCodes;
    window.PIXLY.ErrorSeverity = ErrorSeverity;
    window.PIXLY.ErrorBuilder = ErrorBuilder;
}
