#!/usr/bin/env python3
"""
Pixly 统一日志模块 (Python)
Version: 1.0.0

✅ Phase 46.14+ 三端统一日志规范 (2025-11-11)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
统一日志格式：
{
  "timestamp": "2025-11-11T12:00:00.000Z",
  "level": "INFO",
  "layer": "python-ml",          # ✅ 固定值
  "component": "predictor",
  "message": "Prediction completed",
  "code": "PIXLY-PY-BIZ-001",    # 可选
  "context": {...},              # 可选
  "trace_id": "req-123456"       # 可选
}

错误码格式：PIXLY-PY-{CATEGORY}-{NUMBER}
- VAL: 验证类错误
- FILE: 文件类错误
- NET: 网络类错误
- SYS: 系统类错误
- BIZ: 业务类错误

参见：docs/architecture/PROJECT_QUALITY_MANIFESTO.md
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

import json
import sys
from datetime import datetime
from typing import Optional, Dict, Any


# ============================================================================
# 日志级别
# ============================================================================

class LogLevel:
    """统一日志级别（与Go/Rust一致）"""
    ERROR = "ERROR"
    WARN = "WARN"
    INFO = "INFO"
    DEBUG = "DEBUG"
    TRACE = "TRACE"


# ============================================================================
# 核心日志函数
# ============================================================================

def log_structured(
    level: str,
    component: str,
    message: str,
    code: Optional[str] = None,
    context: Optional[Dict[str, Any]] = None,
    trace_id: Optional[str] = None,
    output_json: bool = True
) -> None:
    """
    输出结构化日志
    
    Args:
        level: 日志级别 (ERROR/WARN/INFO/DEBUG/TRACE)
        component: 组件名称 (predictor/trainer/feature_extractor)
        message: 日志消息
        code: 错误码 (可选，ERROR级别建议提供)
        context: 上下文信息字典 (可选)
        trace_id: 追踪ID (可选)
        output_json: 是否输出JSON格式 (默认True，False则人类可读格式)
    """
    entry = {
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "level": level,
        "layer": "python-ml",
        "component": component,
        "message": message
    }
    
    if code:
        entry["code"] = code
    if context:
        entry["context"] = context
    if trace_id:
        entry["trace_id"] = trace_id
    
    if output_json:
        # JSON格式输出（用于日志聚合）
        print(json.dumps(entry), file=sys.stderr)
    else:
        # 人类可读格式输出（开发调试）
        icon = {
            "ERROR": "❌",
            "WARN": "⚠️",
            "INFO": "✅",
            "DEBUG": "🔍",
            "TRACE": "📍"
        }.get(level, "ℹ️")
        
        msg = f"{icon} [{level}] {component}: {message}"
        if code:
            msg += f" [{code}]"
        if context:
            msg += f" | Context: {json.dumps(context)}"
        
        print(msg, file=sys.stderr)


def log_error(
    component: str,
    message: str,
    code: Optional[str] = None,
    **context
) -> None:
    """
    记录错误日志
    
    Example:
        log_error("predictor", "Model loading failed",
                 code="PIXLY-PY-BIZ-001",
                 model_path="models/lightgbm.txt")
    """
    log_structured(LogLevel.ERROR, component, message, code, context or None)


def log_warn(
    component: str,
    message: str,
    code: Optional[str] = None,
    **context
) -> None:
    """
    记录警告日志
    
    Example:
        log_warn("predictor", "Low confidence prediction",
                code="PIXLY-PY-VAL-005",
                confidence=0.45)
    """
    log_structured(LogLevel.WARN, component, message, code, context or None)


def log_info(
    component: str,
    message: str,
    **context
) -> None:
    """
    记录信息日志
    
    Example:
        log_info("predictor", "Prediction completed",
                quality=90, speed=4, duration_ms=125)
    """
    log_structured(LogLevel.INFO, component, message, context=context or None)


def log_debug(
    component: str,
    message: str,
    **context
) -> None:
    """
    记录调试日志
    
    Example:
        log_debug("feature_extractor", "Extracting features",
                 image_size=1920*1080, complexity=0.75)
    """
    log_structured(LogLevel.DEBUG, component, message, context=context or None)


def log_trace(
    component: str,
    message: str,
    **context
) -> None:
    """
    记录追踪日志
    
    Example:
        log_trace("predictor", "Function entered",
                 func="predict_params", args=["input.jpg"])
    """
    log_structured(LogLevel.TRACE, component, message, context=context or None)


# ============================================================================
# 错误码常量
# ============================================================================

class ErrorCode:
    """Pixly Python错误码常量"""
    
    # ========== 参数验证错误 (VAL) ==========
    VAL_OUT_OF_RANGE = "PIXLY-PY-VAL-001"        # 参数超出范围
    VAL_TYPE_ERROR = "PIXLY-PY-VAL-002"          # 参数类型错误
    VAL_MISSING_REQUIRED = "PIXLY-PY-VAL-003"    # 必填参数缺失
    VAL_CONFLICT = "PIXLY-PY-VAL-004"            # 参数组合冲突
    VAL_LOW_CONFIDENCE = "PIXLY-PY-VAL-005"      # AI置信度过低
    
    # ========== 文件错误 (FILE) ==========
    FILE_NOT_FOUND = "PIXLY-PY-FILE-001"         # 文件不存在
    FILE_READ_ERROR = "PIXLY-PY-FILE-002"        # 文件读取失败
    FILE_FORMAT_UNSUPPORTED = "PIXLY-PY-FILE-003" # 文件格式不支持
    FILE_CORRUPTED = "PIXLY-PY-FILE-004"         # 文件损坏
    
    # ========== 系统错误 (SYS) ==========
    SYS_OUT_OF_MEMORY = "PIXLY-PY-SYS-001"       # 内存不足
    SYS_DEPENDENCY_MISSING = "PIXLY-PY-SYS-002"  # 依赖包缺失
    
    # ========== 业务逻辑错误 (BIZ) ==========
    BIZ_MODEL_LOAD_FAILED = "PIXLY-PY-BIZ-001"   # 模型加载失败
    BIZ_PREDICT_FAILED = "PIXLY-PY-BIZ-002"      # 预测失败
    BIZ_TRAINING_FAILED = "PIXLY-PY-BIZ-003"     # 训练失败
    BIZ_FEATURE_EXTRACT_FAILED = "PIXLY-PY-BIZ-004" # 特征提取失败


# ============================================================================
# 性能追踪工具
# ============================================================================

class PerfLogger:
    """性能追踪日志记录器"""
    
    def __init__(self, component: str, operation: str, **context):
        """
        初始化性能追踪
        
        Example:
            with PerfLogger("predictor", "model_inference") as perf:
                # ... 执行操作 ...
                perf.add_context(model="lightgbm", features=50)
        """
        self.component = component
        self.operation = operation
        self.start_time = datetime.utcnow()
        self.context = context
    
    def __enter__(self):
        log_info(self.component, f"🚀 Starting: {self.operation}", **self.context)
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        duration_ms = (datetime.utcnow() - self.start_time).total_seconds() * 1000
        self.context["duration_ms"] = round(duration_ms, 2)
        
        if exc_type is None:
            log_info(self.component, f"✅ Completed: {self.operation}", **self.context)
        else:
            self.context["error"] = str(exc_val)
            log_error(self.component, f"❌ Failed: {self.operation}", **self.context)
        
        return False  # 不抑制异常
    
    def add_context(self, **kwargs):
        """添加额外的上下文信息"""
        self.context.update(kwargs)


# ============================================================================
# 便捷函数
# ============================================================================

def log_validation_error(
    component: str,
    param: str,
    value: Any,
    expected: str
) -> None:
    """
    记录参数验证错误
    
    Example:
        log_validation_error("predictor", "quality", 150, "1-100")
    """
    log_error(
        component,
        f"{param} 参数超出范围: {value} (应为 {expected})",
        code=ErrorCode.VAL_OUT_OF_RANGE,
        parameter=param,
        value=str(value),
        expected=expected
    )


def log_file_error(
    component: str,
    file_path: str,
    operation: str = "read"
) -> None:
    """
    记录文件错误
    
    Example:
        log_file_error("predictor", "/path/to/model.txt", "load")
    """
    log_error(
        component,
        f"文件{operation}失败: {file_path}",
        code=ErrorCode.FILE_NOT_FOUND,
        file_path=file_path,
        operation=operation
    )


def log_model_error(
    component: str,
    model_name: str,
    error_msg: str
) -> None:
    """
    记录模型错误
    
    Example:
        log_model_error("predictor", "lightgbm", "Model file corrupted")
    """
    log_error(
        component,
        f"模型错误: {model_name} - {error_msg}",
        code=ErrorCode.BIZ_MODEL_LOAD_FAILED,
        model=model_name,
        error=error_msg
    )


# ============================================================================
# 测试和示例
# ============================================================================

if __name__ == "__main__":
    print("=== Pixly Python日志系统测试 ===\n", file=sys.stderr)
    
    # 基本日志测试
    log_info("test", "基本信息日志")
    log_debug("test", "调试日志", value=42, status="ok")
    log_warn("test", "警告日志", code=ErrorCode.VAL_LOW_CONFIDENCE, confidence=0.45)
    log_error("test", "错误日志", code=ErrorCode.BIZ_PREDICT_FAILED, reason="Model not loaded")
    
    # 性能追踪测试
    print("\n性能追踪测试:", file=sys.stderr)
    with PerfLogger("test", "sample_operation", input="test.jpg") as perf:
        import time
        time.sleep(0.1)  # 模拟操作
        perf.add_context(result="success", output_size=1024)
    
    # 便捷函数测试
    print("\n便捷函数测试:", file=sys.stderr)
    log_validation_error("test", "quality", 150, "1-100")
    log_file_error("test", "/path/to/model.txt", "load")
    log_model_error("test", "lightgbm", "File not found")
    
    print("\n=== 测试完成 ===", file=sys.stderr)
