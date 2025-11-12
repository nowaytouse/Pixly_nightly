"""
🔍 PIXLY v3.0 企业级验证引擎

基于Go废弃模块http_validator.go的验证架构：
- 响亮报错原则 (不静默降级)  
- 多层验证策略
- 插件化验证器
- 详细错误上下文
- 性能优化缓存

迁移自：Go http_validator.go 的验证逻辑
"""

import time
import json
import hashlib
import threading
from enum import Enum
from dataclasses import dataclass, asdict
from typing import Dict, List, Optional, Any, Callable, Union, Type
from pathlib import Path
import traceback

from .error_collector import ValidationErrorCollector, ValidationError, ErrorSeverity


class ValidationType(Enum):
    """验证类型"""
    PARAMETER = "parameter"
    DATA = "data"
    IMAGE = "image"
    BUSINESS = "business"
    PLUGIN = "plugin"
    SCHEMA = "schema"


@dataclass
class ValidationRule:
    """验证规则"""
    name: str
    validator_func: Callable
    error_message: str
    severity: ErrorSeverity = ErrorSeverity.ERROR
    enabled: bool = True
    depends_on: List[str] = None
    
    def __post_init__(self):
        if self.depends_on is None:
            self.depends_on = []


@dataclass
class ValidationResult:
    """验证结果"""
    success: bool
    validation_type: ValidationType
    rules_passed: int
    rules_failed: int
    total_duration_ms: float
    errors: List[ValidationError]
    warnings: List[ValidationError] 
    context: Dict[str, Any] = None
    
    def __post_init__(self):
        if self.context is None:
            self.context = {}


class ValidationCache:
    """验证结果缓存"""
    
    def __init__(self, max_size: int = 1000, ttl_seconds: int = 300):
        self.max_size = max_size
        self.ttl_seconds = ttl_seconds
        self.cache: Dict[str, Dict[str, Any]] = {}
        self._lock = threading.RLock()
    
    def _generate_key(self, data: Any) -> str:
        """生成缓存键"""
        if isinstance(data, (dict, list)):
            data_str = json.dumps(data, sort_keys=True)
        else:
            data_str = str(data)
        
        return hashlib.md5(data_str.encode()).hexdigest()
    
    def get(self, data: Any) -> Optional[ValidationResult]:
        """获取缓存的验证结果"""
        key = self._generate_key(data)
        
        with self._lock:
            if key not in self.cache:
                return None
            
            cached_item = self.cache[key]
            
            # 检查过期
            if time.time() - cached_item["timestamp"] > self.ttl_seconds:
                del self.cache[key]
                return None
            
            return cached_item["result"]
    
    def put(self, data: Any, result: ValidationResult):
        """缓存验证结果"""
        key = self._generate_key(data)
        
        with self._lock:
            # 清理过期缓存
            if len(self.cache) >= self.max_size:
                self._cleanup_expired()
            
            # 如果还是超出限制，删除最老的
            if len(self.cache) >= self.max_size:
                oldest_key = min(self.cache.keys(), 
                               key=lambda k: self.cache[k]["timestamp"])
                del self.cache[oldest_key]
            
            self.cache[key] = {
                "result": result,
                "timestamp": time.time()
            }
    
    def _cleanup_expired(self):
        """清理过期缓存"""
        current_time = time.time()
        expired_keys = [
            key for key, item in self.cache.items()
            if current_time - item["timestamp"] > self.ttl_seconds
        ]
        
        for key in expired_keys:
            del self.cache[key]
    
    def clear(self):
        """清空缓存"""
        with self._lock:
            self.cache.clear()


class ValidationEngine:
    """
    🔍 企业级验证引擎
    
    核心特性：
    - 响亮报错原则
    - 多层验证策略  
    - 插件化扩展
    - 性能缓存优化
    """
    
    def __init__(self, enable_cache: bool = True, cache_size: int = 1000):
        # 验证规则注册表
        self.rules: Dict[ValidationType, List[ValidationRule]] = {
            validation_type: [] for validation_type in ValidationType
        }
        
        # 错误收集器
        self.error_collector = ValidationErrorCollector()
        
        # 验证缓存
        self.cache = ValidationCache(max_size=cache_size) if enable_cache else None
        
        # 统计信息
        self.validation_stats: Dict[str, Any] = {
            "total_validations": 0,
            "successful_validations": 0,
            "failed_validations": 0,
            "cache_hits": 0,
            "cache_misses": 0,
            "avg_duration_ms": 0.0
        }
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 预定义验证器
        self._register_builtin_validators()
    
    def _register_builtin_validators(self):
        """注册内置验证器"""
        
        # 参数验证
        self.add_rule(
            ValidationType.PARAMETER,
            "not_none",
            lambda value, **ctx: value is not None,
            "参数不能为空"
        )
        
        self.add_rule(
            ValidationType.PARAMETER,
            "not_empty_string", 
            lambda value, **ctx: not isinstance(value, str) or len(value.strip()) > 0,
            "字符串参数不能为空"
        )
        
        # 数据验证
        self.add_rule(
            ValidationType.DATA,
            "valid_json",
            lambda data, **ctx: self._validate_json(data),
            "数据必须是有效的JSON格式"
        )
        
        # 图像验证 
        self.add_rule(
            ValidationType.IMAGE,
            "valid_image_path",
            lambda path, **ctx: self._validate_image_path(path),
            "图像路径必须存在且格式有效"
        )
        
        self.add_rule(
            ValidationType.IMAGE,
            "image_size_limit",
            lambda path, **ctx: self._validate_image_size(path, ctx.get("max_size_mb", 100)),
            "图像文件大小超出限制"
        )
    
    def _validate_json(self, data: Any) -> bool:
        """验证JSON数据"""
        if isinstance(data, (dict, list)):
            return True
        
        if isinstance(data, str):
            try:
                json.loads(data)
                return True
            except json.JSONDecodeError:
                return False
        
        return False
    
    def _validate_image_path(self, path: Any) -> bool:
        """验证图像路径"""
        if not isinstance(path, (str, Path)):
            return False
        
        path_obj = Path(path)
        
        # 检查文件是否存在
        if not path_obj.exists() or not path_obj.is_file():
            return False
        
        # 检查文件扩展名
        valid_extensions = {
            '.jpg', '.jpeg', '.png', '.webp', '.gif', 
            '.bmp', '.tiff', '.tif', '.avif', '.jxl', 
            '.heic', '.heif'
        }
        
        return path_obj.suffix.lower() in valid_extensions
    
    def _validate_image_size(self, path: Any, max_size_mb: float) -> bool:
        """验证图像大小"""
        if not isinstance(path, (str, Path)):
            return False
        
        try:
            path_obj = Path(path)
            if not path_obj.exists():
                return False
            
            size_mb = path_obj.stat().st_size / (1024 * 1024)
            return size_mb <= max_size_mb
            
        except Exception:
            return False
    
    def add_rule(self, validation_type: ValidationType, name: str,
                validator_func: Callable, error_message: str,
                severity: ErrorSeverity = ErrorSeverity.ERROR,
                depends_on: List[str] = None):
        """添加验证规则"""
        
        rule = ValidationRule(
            name=name,
            validator_func=validator_func,
            error_message=error_message,
            severity=severity,
            depends_on=depends_on or []
        )
        
        with self._lock:
            self.rules[validation_type].append(rule)
    
    def remove_rule(self, validation_type: ValidationType, name: str):
        """移除验证规则"""
        with self._lock:
            self.rules[validation_type] = [
                rule for rule in self.rules[validation_type]
                if rule.name != name
            ]
    
    def validate(self, validation_type: ValidationType, data: Any,
                context: Dict[str, Any] = None, use_cache: bool = True) -> ValidationResult:
        """执行验证"""
        
        start_time = time.time()
        context = context or {}
        
        # 检查缓存
        if use_cache and self.cache:
            cached_result = self.cache.get((validation_type, data, context))
            if cached_result:
                with self._lock:
                    self.validation_stats["cache_hits"] += 1
                return cached_result
        
        # 执行验证
        result = self._perform_validation(validation_type, data, context)
        result.total_duration_ms = (time.time() - start_time) * 1000
        
        # 更新统计
        with self._lock:
            self.validation_stats["total_validations"] += 1
            if result.success:
                self.validation_stats["successful_validations"] += 1
            else:
                self.validation_stats["failed_validations"] += 1
            
            # 更新平均时间
            total = self.validation_stats["total_validations"]
            current_avg = self.validation_stats["avg_duration_ms"]
            self.validation_stats["avg_duration_ms"] = (
                (current_avg * (total - 1) + result.total_duration_ms) / total
            )
            
            if use_cache and self.cache:
                self.validation_stats["cache_misses"] += 1
        
        # 缓存结果
        if use_cache and self.cache and result.success:
            self.cache.put((validation_type, data, context), result)
        
        return result
    
    def _perform_validation(self, validation_type: ValidationType, 
                          data: Any, context: Dict[str, Any]) -> ValidationResult:
        """执行实际验证"""
        
        rules = self.rules.get(validation_type, [])
        enabled_rules = [rule for rule in rules if rule.enabled]
        
        if not enabled_rules:
            return ValidationResult(
                success=True,
                validation_type=validation_type,
                rules_passed=0,
                rules_failed=0,
                total_duration_ms=0.0,
                errors=[],
                warnings=[],
                context=context
            )
        
        # 按依赖排序规则
        sorted_rules = self._sort_rules_by_dependency(enabled_rules)
        
        errors = []
        warnings = []
        rules_passed = 0
        rules_failed = 0
        
        for rule in sorted_rules:
            try:
                # 检查依赖是否满足
                if not self._check_dependencies(rule, errors):
                    continue
                
                # 执行验证
                is_valid = rule.validator_func(data, **context)
                
                if is_valid:
                    rules_passed += 1
                else:
                    rules_failed += 1
                    
                    error = ValidationError(
                        code=f"{validation_type.value}_{rule.name}",
                        message=rule.error_message,
                        severity=rule.severity,
                        context={
                            "rule_name": rule.name,
                            "validation_type": validation_type.value,
                            "data_type": type(data).__name__,
                            **context
                        }
                    )
                    
                    if rule.severity in [ErrorSeverity.ERROR, ErrorSeverity.CRITICAL]:
                        errors.append(error)
                    else:
                        warnings.append(error)
                        
            except Exception as e:
                rules_failed += 1
                
                error = ValidationError(
                    code=f"{validation_type.value}_{rule.name}_exception",
                    message=f"验证规则执行异常: {str(e)}",
                    severity=ErrorSeverity.CRITICAL,
                    context={
                        "rule_name": rule.name,
                        "exception": str(e),
                        "traceback": traceback.format_exc(),
                        **context
                    }
                )
                errors.append(error)
        
        success = len(errors) == 0
        
        return ValidationResult(
            success=success,
            validation_type=validation_type,
            rules_passed=rules_passed,
            rules_failed=rules_failed,
            total_duration_ms=0.0,  # 将在外层设置
            errors=errors,
            warnings=warnings,
            context=context
        )
    
    def _sort_rules_by_dependency(self, rules: List[ValidationRule]) -> List[ValidationRule]:
        """按依赖关系排序规则"""
        # 简单的拓扑排序实现
        sorted_rules = []
        remaining_rules = rules.copy()
        
        while remaining_rules:
            # 找到没有未满足依赖的规则
            ready_rules = []
            for rule in remaining_rules:
                dependencies_satisfied = all(
                    any(r.name == dep for r in sorted_rules)
                    for dep in rule.depends_on
                )
                if dependencies_satisfied:
                    ready_rules.append(rule)
            
            if not ready_rules:
                # 如果有循环依赖，按原顺序添加
                ready_rules = remaining_rules
            
            # 添加就绪的规则
            for rule in ready_rules:
                sorted_rules.append(rule)
                remaining_rules.remove(rule)
        
        return sorted_rules
    
    def _check_dependencies(self, rule: ValidationRule, 
                          current_errors: List[ValidationError]) -> bool:
        """检查规则依赖"""
        if not rule.depends_on:
            return True
        
        # 检查是否有依赖的规则失败
        failed_dependencies = [
            dep for dep in rule.depends_on
            if any(dep in error.context.get("rule_name", "") for error in current_errors)
        ]
        
        return len(failed_dependencies) == 0
    
    def validate_parameters(self, params: Dict[str, Any], 
                          context: Dict[str, Any] = None) -> ValidationResult:
        """验证参数"""
        return self.validate(ValidationType.PARAMETER, params, context)
    
    def validate_data(self, data: Any, 
                     context: Dict[str, Any] = None) -> ValidationResult:
        """验证数据"""
        return self.validate(ValidationType.DATA, data, context)
    
    def validate_image(self, image_path: str, 
                      context: Dict[str, Any] = None) -> ValidationResult:
        """验证图像"""
        return self.validate(ValidationType.IMAGE, image_path, context)
    
    def validate_business_rules(self, data: Any,
                               context: Dict[str, Any] = None) -> ValidationResult:
        """验证业务规则"""
        return self.validate(ValidationType.BUSINESS, data, context)
    
    def validate_plugin(self, plugin: Any,
                       context: Dict[str, Any] = None) -> ValidationResult:
        """验证插件"""
        return self.validate(ValidationType.PLUGIN, plugin, context)
    
    def validate_multiple(self, validations: List[Dict[str, Any]]) -> List[ValidationResult]:
        """批量验证"""
        results = []
        
        for validation in validations:
            validation_type = validation["type"]
            data = validation["data"]
            context = validation.get("context", {})
            
            result = self.validate(validation_type, data, context)
            results.append(result)
        
        return results
    
    def get_validation_summary(self) -> Dict[str, Any]:
        """获取验证统计摘要"""
        with self._lock:
            stats = self.validation_stats.copy()
        
        # 计算成功率
        total = stats["total_validations"]
        if total > 0:
            stats["success_rate"] = stats["successful_validations"] / total
            stats["failure_rate"] = stats["failed_validations"] / total
        else:
            stats["success_rate"] = 0.0
            stats["failure_rate"] = 0.0
        
        # 缓存命中率
        total_cache_attempts = stats["cache_hits"] + stats["cache_misses"]
        if total_cache_attempts > 0:
            stats["cache_hit_rate"] = stats["cache_hits"] / total_cache_attempts
        else:
            stats["cache_hit_rate"] = 0.0
        
        # 规则统计
        stats["total_rules"] = sum(len(rules) for rules in self.rules.values())
        stats["enabled_rules"] = sum(
            len([r for r in rules if r.enabled]) for rules in self.rules.values()
        )
        
        return stats
    
    def clear_cache(self):
        """清空验证缓存"""
        if self.cache:
            self.cache.clear()
    
    def reset_stats(self):
        """重置统计信息"""
        with self._lock:
            self.validation_stats = {
                "total_validations": 0,
                "successful_validations": 0,
                "failed_validations": 0,
                "cache_hits": 0,
                "cache_misses": 0,
                "avg_duration_ms": 0.0
            }
    
    def export_rules(self) -> Dict[str, Any]:
        """导出验证规则配置"""
        exported = {}
        
        for validation_type, rules in self.rules.items():
            exported[validation_type.value] = []
            
            for rule in rules:
                rule_data = {
                    "name": rule.name,
                    "error_message": rule.error_message,
                    "severity": rule.severity.value,
                    "enabled": rule.enabled,
                    "depends_on": rule.depends_on
                }
                exported[validation_type.value].append(rule_data)
        
        return exported
    
    def get_failed_validations_report(self) -> Dict[str, Any]:
        """获取失败验证报告"""
        return self.error_collector.get_error_summary()
