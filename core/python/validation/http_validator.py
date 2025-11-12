"""
HTTP验证器 - API参数严格验证系统

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/http_validator.go 重新实现

核心功能:
- HTTP API参数严格验证(响亮报错原则)
- 图像路径和格式验证  
- 工具名称和质量参数校验
- 请求选项完整性检查
- 多格式扩展名支持

EX-024实现: 从Go废弃代码价值提取 + 架构增强
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Optional, Any, Union, Tuple
from dataclasses import dataclass, field
from enum import Enum
import logging
import time
import hashlib


class ValidationLevel(Enum):
    """验证级别"""
    STRICT = "strict"      # 严格验证
    NORMAL = "normal"      # 普通验证  
    LOOSE = "loose"        # 宽松验证


class ValidationError(Exception):
    """验证错误异常"""
    
    def __init__(self, 
                 message: str, 
                 field: str = "", 
                 value: Any = None,
                 error_code: str = "VALIDATION_ERROR"):
        """
        初始化验证错误
        
        Args:
            message: 错误消息
            field: 错误字段名
            value: 错误值
            error_code: 错误码
        """
        super().__init__(message)
        self.field = field
        self.value = value
        self.error_code = error_code
        self.timestamp = time.time()


@dataclass  
class ValidationResult:
    """验证结果"""
    is_valid: bool = True
    errors: List[ValidationError] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    
    # 架构增强：性能和元数据
    validation_time_ms: float = 0.0
    fields_validated: List[str] = field(default_factory=list)
    suggestions: List[str] = field(default_factory=list)
    
    def add_error(self, message: str, field: str = "", value: Any = None, error_code: str = "VALIDATION_ERROR"):
        """添加错误"""
        self.is_valid = False
        self.errors.append(ValidationError(message, field, value, error_code))
    
    def add_warning(self, message: str):
        """添加警告"""
        self.warnings.append(message)
    
    def add_suggestion(self, message: str):
        """添加建议"""
        self.suggestions.append(message)


@dataclass
class PredictRequest:
    """预测请求数据结构"""
    image_path: str = ""
    tool: str = ""
    target_quality: int = 0
    optimize_mode: str = ""
    options: Optional[Dict[str, Any]] = None
    
    # 架构增强：扩展字段
    user_id: str = ""
    session_id: str = ""
    priority: int = 5
    timeout_seconds: int = 30


@dataclass  
class PredictResponse:
    """预测响应数据结构"""
    params: Optional[Dict[str, Any]] = None
    confidence: float = 0.0
    inference_time_ms: float = 0.0
    
    # 架构增强：扩展字段
    model_version: str = ""
    alternatives: List[Dict[str, Any]] = field(default_factory=list)
    debug_info: Dict[str, Any] = field(default_factory=dict)


class HTTPValidator:
    """
    HTTP验证器 - API参数严格验证系统
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, 
                 validation_level: ValidationLevel = ValidationLevel.NORMAL,
                 debug: bool = False):
        """
        初始化HTTP验证器
        
        Args:
            validation_level: 验证级别
            debug: 调试模式
        """
        self.validation_level = validation_level
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
        
        # 支持的格式和工具
        self.supported_extensions = {
            '.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp', 
            '.tiff', '.tif', '.avif', '.jxl', '.heic', '.heif'
        }
        
        self.supported_tools = {
            'jxl', 'cjxl', 'avif', 'avifenc', 'webp', 'cwebp', 
            'ffmpeg', 'magick'
        }
        
        self.supported_formats = {
            'jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'heic'
        }
        
        self.optimize_modes = {
            'size', 'balanced', 'quality', 'universal', 'fast', 'extreme'
        }
        
        # 架构增强：验证缓存和性能追踪
        self._validation_cache: Dict[str, ValidationResult] = {}
        self._validation_stats: Dict[str, int] = {
            'total_validations': 0,
            'cache_hits': 0,
            'errors': 0,
            'warnings': 0
        }
        
        if debug:
            self.logger.debug("HTTP验证器初始化完成")
    
    def validate_predict_request(self, 
                               request: Union[PredictRequest, Dict[str, Any]],
                               use_cache: bool = True) -> ValidationResult:
        """
        验证预测请求
        
        Args:
            request: 预测请求对象或字典
            use_cache: 是否使用缓存
            
        Returns:
            验证结果
        """
        start_time = time.time()
        result = ValidationResult()
        
        try:
            # 转换为标准对象
            if isinstance(request, dict):
                req_obj = PredictRequest(
                    image_path=request.get('image_path', ''),
                    tool=request.get('tool', ''),
                    target_quality=request.get('target_quality', 0),
                    optimize_mode=request.get('optimize_mode', ''),
                    options=request.get('options')
                )
            else:
                req_obj = request
            
            # 检查缓存
            cache_key = self._generate_cache_key(req_obj)
            if use_cache and cache_key in self._validation_cache:
                self._validation_stats['cache_hits'] += 1
                return self._validation_cache[cache_key]
            
            # 执行验证
            self._validate_image_path(req_obj.image_path, result)
            
            if req_obj.tool:
                self._validate_tool_name(req_obj.tool, result)
            
            if req_obj.target_quality > 0:
                self._validate_target_quality(req_obj.target_quality, result)
            
            if req_obj.optimize_mode:
                self._validate_optimize_mode(req_obj.optimize_mode, result)
            
            if req_obj.options:
                self._validate_request_options(req_obj.options, result)
            
            # 性能统计
            validation_time = (time.time() - start_time) * 1000
            result.validation_time_ms = validation_time
            result.fields_validated = ['image_path', 'tool', 'target_quality', 'optimize_mode', 'options']
            
            # 更新统计
            self._update_stats(result)
            
            # 缓存结果
            if use_cache:
                self._validation_cache[cache_key] = result
            
            if self.debug:
                self.logger.debug(f"请求验证完成: {'成功' if result.is_valid else '失败'}, "
                                f"耗时: {validation_time:.2f}ms")
            
            return result
            
        except Exception as e:
            result.add_error(f"验证过程异常: {e}", error_code="VALIDATION_EXCEPTION")
            self.logger.error(f"验证异常: {e}")
            return result
    
    def _validate_image_path(self, path: str, result: ValidationResult) -> None:
        """验证图像路径"""
        if not path or not path.strip():
            result.add_error("图像路径为空", "image_path", path, "EMPTY_PATH")
            return
        
        path = path.strip()
        
        # 检查文件是否存在
        if not os.path.exists(path):
            result.add_error(f"图像文件不存在: {path}", "image_path", path, "FILE_NOT_FOUND")
            return
        
        # 检查是否为文件
        if not os.path.isfile(path):
            result.add_error(f"路径不是文件: {path}", "image_path", path, "NOT_A_FILE")
            return
        
        # 检查文件扩展名
        ext = Path(path).suffix.lower()
        if ext not in self.supported_extensions:
            supported_list = ', '.join(sorted(self.supported_extensions))
            result.add_error(
                f"不支持的图像格式: {ext} (支持: {supported_list})",
                "image_path", ext, "UNSUPPORTED_FORMAT"
            )
            return
        
        # 架构增强：文件大小检查
        try:
            file_size = os.path.getsize(path)
            if file_size == 0:
                result.add_error("图像文件为空", "image_path", path, "EMPTY_FILE")
            elif file_size > 100 * 1024 * 1024:  # 100MB限制
                result.add_warning(f"图像文件较大: {file_size / 1024 / 1024:.1f}MB")
                result.add_suggestion("考虑先压缩大型图像文件")
        except OSError as e:
            result.add_warning(f"无法获取文件大小: {e}")
    
    def _validate_tool_name(self, tool: str, result: ValidationResult) -> None:
        """验证工具名称"""
        if not tool or not tool.strip():
            result.add_error("工具名称为空", "tool", tool, "EMPTY_TOOL")
            return
        
        tool_lower = tool.strip().lower()
        if tool_lower not in self.supported_tools:
            supported_list = ', '.join(sorted(self.supported_tools))
            result.add_error(
                f"不支持的工具: {tool} (支持: {supported_list})",
                "tool", tool, "UNSUPPORTED_TOOL"
            )
        
        # 架构增强：工具建议
        if tool_lower in ['cjxl', 'jxl']:
            result.add_suggestion("JXL格式提供最佳压缩效率")
        elif tool_lower in ['avifenc', 'avif']:
            result.add_suggestion("AVIF格式在现代浏览器中表现优异")
    
    def _validate_target_quality(self, quality: int, result: ValidationResult) -> None:
        """验证目标质量"""
        if not isinstance(quality, int):
            result.add_error(f"质量参数类型错误: {type(quality)}", "target_quality", quality, "INVALID_TYPE")
            return
        
        if quality < 1 or quality > 100:
            result.add_error(
                f"目标质量超出范围: {quality} (应为1-100)",
                "target_quality", quality, "OUT_OF_RANGE"
            )
            return
        
        # 架构增强：质量建议
        if quality < 50:
            result.add_warning("质量设置较低，可能影响图像效果")
        elif quality > 95:
            result.add_suggestion("极高质量可能导致文件过大")
    
    def _validate_optimize_mode(self, mode: str, result: ValidationResult) -> None:
        """验证优化模式"""
        if not mode or not mode.strip():
            return  # 空值允许，会使用默认值
        
        mode_lower = mode.strip().lower()
        if mode_lower not in self.optimize_modes:
            modes_list = ', '.join(sorted(self.optimize_modes))
            result.add_error(
                f"无效的优化模式: {mode} (支持: {modes_list})",
                "optimize_mode", mode, "INVALID_MODE"
            )
            return
        
        # 架构增强：模式建议
        mode_suggestions = {
            'size': "文件大小优先，适合网络传输",
            'quality': "质量优先，适合存档保存",  
            'balanced': "平衡模式，适合大多数场景",
            'fast': "快速处理，适合批量操作",
            'extreme': "极限压缩，处理时间较长"
        }
        
        if mode_lower in mode_suggestions:
            result.add_suggestion(mode_suggestions[mode_lower])
    
    def _validate_request_options(self, options: Dict[str, Any], result: ValidationResult) -> None:
        """验证请求选项"""
        if not options:
            return
        
        # 验证期望格式
        if 'expected_format' in options:
            format_val = options['expected_format']
            if format_val and format_val.strip():
                format_lower = format_val.strip().lower()
                if format_lower not in self.supported_formats:
                    formats_list = ', '.join(sorted(self.supported_formats))
                    result.add_error(
                        f"无效的期望格式: {format_val} (支持: {formats_list})",
                        "options.expected_format", format_val, "INVALID_FORMAT"
                    )
        
        # 架构增强：选项完整性检查
        valid_option_keys = {
            'expected_format', 'lossless', 'preserve_metadata', 
            'progressive', 'optimize_coding', 'auto_orient'
        }
        
        for key in options:
            if key not in valid_option_keys:
                result.add_warning(f"未知的选项: {key}")
    
    def validate_predict_response(self, 
                                response: Union[PredictResponse, Dict[str, Any]]) -> ValidationResult:
        """
        验证预测响应
        
        Args:
            response: 预测响应对象或字典
            
        Returns:
            验证结果
        """
        start_time = time.time()
        result = ValidationResult()
        
        try:
            # 转换为标准对象
            if isinstance(response, dict):
                resp_obj = PredictResponse(
                    params=response.get('params'),
                    confidence=response.get('confidence', 0.0),
                    inference_time_ms=response.get('inference_time_ms', 0.0)
                )
            else:
                resp_obj = response
            
            # 验证置信度
            if not (0.0 <= resp_obj.confidence <= 1.0):
                result.add_error(
                    f"置信度超出范围: {resp_obj.confidence} (应为0.0-1.0)",
                    "confidence", resp_obj.confidence, "OUT_OF_RANGE"
                )
            
            # 验证推理时间
            if resp_obj.inference_time_ms < 0:
                result.add_error(
                    f"推理时间无效: {resp_obj.inference_time_ms} (必须 >= 0)",
                    "inference_time_ms", resp_obj.inference_time_ms, "INVALID_TIME"
                )
            
            # 验证参数
            if resp_obj.params:
                self._validate_predict_params(resp_obj.params, result)
            
            # 性能统计
            validation_time = (time.time() - start_time) * 1000
            result.validation_time_ms = validation_time
            
            return result
            
        except Exception as e:
            result.add_error(f"响应验证异常: {e}", error_code="RESPONSE_VALIDATION_EXCEPTION")
            self.logger.error(f"响应验证异常: {e}")
            return result
    
    def _validate_predict_params(self, params: Dict[str, Any], result: ValidationResult) -> None:
        """验证预测参数"""
        # 验证质量
        if 'quality' in params:
            quality = params['quality']
            if not isinstance(quality, (int, float)) or not (0 <= quality <= 100):
                result.add_error(
                    f"质量参数超出范围: {quality} (应为0-100)",
                    "params.quality", quality, "QUALITY_OUT_OF_RANGE"
                )
        
        # 验证effort/speed
        if 'effort' in params:
            effort = params['effort']
            if not isinstance(effort, (int, float)) or not (0 <= effort <= 10):
                result.add_error(
                    f"Effort参数超出范围: {effort} (应为0-10)",
                    "params.effort", effort, "EFFORT_OUT_OF_RANGE"
                )
        
        # 验证距离 (JXL)
        if 'distance' in params:
            distance = params['distance']
            if not isinstance(distance, (int, float)) or not (0.0 <= distance <= 15.0):
                result.add_error(
                    f"Distance参数超出范围: {distance} (应为0.0-15.0)",
                    "params.distance", distance, "DISTANCE_OUT_OF_RANGE"
                )
    
    def _generate_cache_key(self, request: PredictRequest) -> str:
        """生成缓存键"""
        key_data = f"{request.image_path}_{request.tool}_{request.target_quality}_{request.optimize_mode}"
        return hashlib.md5(key_data.encode()).hexdigest()[:16]
    
    def _update_stats(self, result: ValidationResult) -> None:
        """更新统计信息"""
        self._validation_stats['total_validations'] += 1
        if not result.is_valid:
            self._validation_stats['errors'] += 1
        if result.warnings:
            self._validation_stats['warnings'] += 1
    
    def get_validation_stats(self) -> Dict[str, Any]:
        """
        获取验证统计
        
        Returns:
            统计信息字典
        """
        cache_hit_rate = 0.0
        if self._validation_stats['total_validations'] > 0:
            cache_hit_rate = self._validation_stats['cache_hits'] / self._validation_stats['total_validations']
        
        return {
            **self._validation_stats,
            'cache_hit_rate': cache_hit_rate,
            'cache_size': len(self._validation_cache),
            'validation_level': self.validation_level.value,
            'supported_extensions': sorted(self.supported_extensions),
            'supported_tools': sorted(self.supported_tools)
        }
    
    def clear_cache(self) -> None:
        """清空验证缓存"""
        self._validation_cache.clear()
        if self.debug:
            self.logger.debug("验证缓存已清空")


# 便捷函数
def validate_request(request: Union[PredictRequest, Dict[str, Any]], 
                    strict: bool = False) -> ValidationResult:
    """
    快速验证请求的便捷函数
    
    Args:
        request: 请求对象
        strict: 是否使用严格模式
        
    Returns:
        验证结果
    """
    level = ValidationLevel.STRICT if strict else ValidationLevel.NORMAL
    validator = HTTPValidator(validation_level=level)
    return validator.validate_predict_request(request)


def validate_response(response: Union[PredictResponse, Dict[str, Any]]) -> ValidationResult:
    """
    快速验证响应的便捷函数
    
    Args:
        response: 响应对象
        
    Returns:
        验证结果
    """
    validator = HTTPValidator()
    return validator.validate_predict_response(response)


if __name__ == "__main__":
    # 测试代码
    print("=== HTTP验证器测试 ===")
    
    # 创建验证器
    validator = HTTPValidator(debug=True)
    
    # 测试请求验证
    test_request = {
        'image_path': '/nonexistent/image.jpg',
        'tool': 'jxl',
        'target_quality': 85,
        'optimize_mode': 'balanced',
        'options': {'expected_format': 'jxl'}
    }
    
    print("\n📝 测试请求验证...")
    result = validator.validate_predict_request(test_request)
    print(f"验证结果: {'✅ 通过' if result.is_valid else '❌ 失败'}")
    print(f"错误数: {len(result.errors)}")
    print(f"警告数: {len(result.warnings)}")
    print(f"建议数: {len(result.suggestions)}")
    print(f"验证耗时: {result.validation_time_ms:.2f}ms")
    
    if result.errors:
        print("\n错误详情:")
        for error in result.errors:
            print(f"  - {error.field}: {error}")
    
    # 测试响应验证  
    test_response = {
        'params': {'quality': 85, 'effort': 7},
        'confidence': 0.85,
        'inference_time_ms': 120.5
    }
    
    print("\n📤 测试响应验证...")
    resp_result = validator.validate_predict_response(test_response)
    print(f"响应验证: {'✅ 通过' if resp_result.is_valid else '❌ 失败'}")
    
    # 查看统计
    stats = validator.get_validation_stats()
    print(f"\n📊 验证统计:")
    print(f"  总验证次数: {stats['total_validations']}")
    print(f"  错误次数: {stats['errors']}")
    print(f"  缓存命中率: {stats['cache_hit_rate']:.2%}")
    
    print("🎯 HTTP验证器测试完成！")
