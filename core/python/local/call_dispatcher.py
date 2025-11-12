"""
直接调用分派器 - 纯本地函数调用架构

彻底摆脱网络概念，实现真正的本地化：
- 没有HTTP请求/响应
- 没有网络序列化
- 没有网关概念
- 只有直接函数调用

架构原则: 函数调用 > 一切网络概念
"""

import time
import threading
from typing import Dict, List, Optional, Any, Callable, Union
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
import logging
import inspect
from concurrent.futures import ThreadPoolExecutor, Future
import numpy as np


class CallMode(Enum):
    """调用模式"""
    SYNC = "sync"              # 同步直接调用
    ASYNC = "async"            # 异步调用
    BATCH = "batch"            # 批量调用
    PARALLEL = "parallel"      # 并行调用


@dataclass
class DirectCall:
    """直接调用定义"""
    function_name: str
    args: tuple
    kwargs: dict
    call_mode: CallMode = CallMode.SYNC
    memory_blocks: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    # 性能追踪
    created_at: float = field(default_factory=time.perf_counter)
    call_id: str = field(default="")
    
    def __post_init__(self):
        if not self.call_id:
            self.call_id = f"{self.function_name}_{int(time.time() * 1000000)}"


@dataclass 
class CallResult:
    """调用结果"""
    call_id: str
    success: bool
    result: Any = None
    error: Optional[str] = None
    duration_ms: float = 0.0
    memory_used_mb: float = 0.0
    
    # 元数据
    function_name: str = ""
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())


class CallDispatcher:
    """
    直接调用分派器 - 纯本地函数调用架构
    
    完全摆脱网络概念，只有函数直接调用
    """
    
    def __init__(self, 
                 max_workers: int = 10,
                 enable_metrics: bool = True,
                 debug: bool = False):
        """
        初始化直接调用分派器
        
        Args:
            max_workers: 最大并发工作线程
            enable_metrics: 启用性能指标
            debug: 调试模式
        """
        self.max_workers = max_workers
        self.enable_metrics = enable_metrics
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        # 本地函数注册表
        self._local_functions: Dict[str, Callable] = {}
        self._function_metadata: Dict[str, Dict[str, Any]] = {}
        
        # 线程池 (仅用于异步调用)
        self._thread_pool = ThreadPoolExecutor(max_workers=max_workers)
        
        # 性能统计
        self._call_metrics: Dict[str, Dict[str, float]] = {}
        self._call_counts: Dict[str, int] = {}
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 注册核心本地函数
        self._register_core_functions()
        
        if debug:
            self.logger.debug("直接调用分派器初始化完成")
    
    def register_function(self, 
                         name: str, 
                         func: Callable,
                         description: str = "",
                         expected_latency_ms: float = 1.0,
                         memory_intensive: bool = False) -> bool:
        """
        注册本地函数
        
        Args:
            name: 函数名称
            func: 可调用函数
            description: 函数描述
            expected_latency_ms: 预期延迟
            memory_intensive: 是否内存密集
        """
        try:
            with self._lock:
                # 注册函数
                self._local_functions[name] = func
                
                # 保存元数据
                self._function_metadata[name] = {
                    'description': description,
                    'expected_latency_ms': expected_latency_ms,
                    'memory_intensive': memory_intensive,
                    'signature': str(inspect.signature(func)),
                    'registered_at': datetime.now().isoformat()
                }
                
                # 初始化统计
                self._call_metrics[name] = {
                    'total_time': 0.0,
                    'min_time': float('inf'),
                    'max_time': 0.0,
                    'avg_time': 0.0
                }
                self._call_counts[name] = 0
            
            if self.debug:
                self.logger.debug(f"注册本地函数: {name}")
            
            return True
            
        except Exception as e:
            self.logger.error(f"函数注册失败: {name}, {e}")
            return False
    
    def call(self, call_request: DirectCall) -> CallResult:
        """
        执行直接函数调用
        
        Args:
            call_request: 调用请求
        """
        start_time = time.perf_counter()
        
        try:
            # 检查函数是否存在
            if call_request.function_name not in self._local_functions:
                return CallResult(
                    call_id=call_request.call_id,
                    success=False,
                    error=f"函数未注册: {call_request.function_name}",
                    function_name=call_request.function_name
                )
            
            func = self._local_functions[call_request.function_name]
            
            # 根据调用模式执行
            if call_request.call_mode == CallMode.SYNC:
                result = self._call_sync(func, call_request.args, call_request.kwargs)
            elif call_request.call_mode == CallMode.ASYNC:
                result = self._call_async(func, call_request.args, call_request.kwargs)
            elif call_request.call_mode == CallMode.BATCH:
                result = self._call_batch(func, call_request.args, call_request.kwargs)
            elif call_request.call_mode == CallMode.PARALLEL:
                result = self._call_parallel(func, call_request.args, call_request.kwargs)
            else:
                result = func(*call_request.args, **call_request.kwargs)
            
            # 记录性能
            duration_ms = (time.perf_counter() - start_time) * 1000
            self._update_metrics(call_request.function_name, duration_ms)
            
            return CallResult(
                call_id=call_request.call_id,
                success=True,
                result=result,
                duration_ms=duration_ms,
                function_name=call_request.function_name
            )
            
        except Exception as e:
            duration_ms = (time.perf_counter() - start_time) * 1000
            self.logger.error(f"函数调用失败: {call_request.function_name}, {e}")
            
            return CallResult(
                call_id=call_request.call_id,
                success=False,
                error=str(e),
                duration_ms=duration_ms,
                function_name=call_request.function_name
            )
    
    def _call_sync(self, func: Callable, args: tuple, kwargs: dict) -> Any:
        """同步调用"""
        return func(*args, **kwargs)
    
    def _call_async(self, func: Callable, args: tuple, kwargs: dict) -> Future:
        """异步调用"""
        return self._thread_pool.submit(func, *args, **kwargs)
    
    def _call_batch(self, func: Callable, args: tuple, kwargs: dict) -> List[Any]:
        """批量调用"""
        # 假设第一个参数是批量数据
        if args and hasattr(args[0], '__iter__') and not isinstance(args[0], (str, bytes)):
            batch_data = args[0]
            results = []
            for item in batch_data:
                result = func(item, *args[1:], **kwargs)
                results.append(result)
            return results
        else:
            return func(*args, **kwargs)
    
    def _call_parallel(self, func: Callable, args: tuple, kwargs: dict) -> List[Any]:
        """并行调用"""
        # 假设第一个参数是可并行处理的数据
        if args and hasattr(args[0], '__iter__') and not isinstance(args[0], (str, bytes)):
            batch_data = args[0]
            futures = []
            
            for item in batch_data:
                future = self._thread_pool.submit(func, item, *args[1:], **kwargs)
                futures.append(future)
            
            # 收集结果
            results = [future.result() for future in futures]
            return results
        else:
            return func(*args, **kwargs)
    
    def _update_metrics(self, function_name: str, duration_ms: float):
        """更新性能指标"""
        if not self.enable_metrics:
            return
        
        with self._lock:
            metrics = self._call_metrics[function_name]
            count = self._call_counts[function_name]
            
            metrics['total_time'] += duration_ms
            metrics['min_time'] = min(metrics['min_time'], duration_ms)
            metrics['max_time'] = max(metrics['max_time'], duration_ms)
            
            self._call_counts[function_name] = count + 1
            metrics['avg_time'] = metrics['total_time'] / self._call_counts[function_name]
    
    def _register_core_functions(self):
        """注册核心本地函数"""
        
        def process_image(image_data: bytes, operation: str) -> bytes:
            """图像处理函数 - 直接内存操作"""
            if operation == "compress":
                # 模拟压缩 (零拷贝操作)
                return image_data[::2]
            elif operation == "resize":
                # 模拟调整大小
                return image_data[:len(image_data)//2]
            else:
                return image_data
        
        def ai_inference(features: np.ndarray, config: dict) -> dict:
            """AI推理函数 - 直接内存操作"""
            if features is not None and len(features.shape) > 0:
                # 直接数组操作，无序列化
                prediction = np.mean(features) * 0.8 + 0.1
                return {
                    "prediction": float(prediction),
                    "confidence": 0.85,
                    "model": config.get("model", "local"),
                    "features_processed": features.shape[0]
                }
            else:
                return {"prediction": 0.5, "confidence": 0.0, "model": "local"}
        
        def extract_features(image_data: bytes) -> np.ndarray:
            """特征提取函数 - 直接内存操作"""
            # 基于图像数据直接生成特征，无网络传输
            data_hash = hash(image_data) % 1000000
            features = np.array([
                data_hash / 1000000,
                len(image_data) / 10000,
                sum(image_data[:100]) / 25600,
                len(set(image_data[:1000])) / 256,
                np.std([b for b in image_data[:1000]]) / 128
            ], dtype=np.float32)
            
            return features
        
        def math_operation(operation: str, *args) -> Any:
            """数学运算函数"""
            if operation == "matrix_multiply" and len(args) >= 2:
                a, b = args[0], args[1]
                if isinstance(a, np.ndarray) and isinstance(b, np.ndarray):
                    return np.dot(a, b)
            elif operation == "vector_norm" and len(args) >= 1:
                vec = args[0]
                if isinstance(vec, np.ndarray):
                    return np.linalg.norm(vec)
            
            return None
        
        # 注册核心函数
        self.register_function("process_image", process_image, 
                              "图像处理 - 直接内存操作", 0.1, True)
        self.register_function("ai_inference", ai_inference,
                              "AI推理 - 零拷贝计算", 0.2, True)
        self.register_function("extract_features", extract_features,
                              "特征提取 - 直接数组操作", 0.1, False)
        self.register_function("math_operation", math_operation,
                              "数学运算 - 向量化操作", 0.05, False)
    
    # 便捷调用方法
    def process_image_local(self, image_data: bytes, operation: str) -> CallResult:
        """本地图像处理"""
        call = DirectCall("process_image", (image_data, operation), {})
        return self.call(call)
    
    def ai_inference_local(self, features: np.ndarray, config: dict) -> CallResult:
        """本地AI推理"""
        call = DirectCall("ai_inference", (features, config), {})
        return self.call(call)
    
    def extract_features_local(self, image_data: bytes) -> CallResult:
        """本地特征提取"""
        call = DirectCall("extract_features", (image_data,), {})
        return self.call(call)
    
    def math_operation_local(self, operation: str, *args) -> CallResult:
        """本地数学运算"""
        call = DirectCall("math_operation", (operation, *args), {})
        return self.call(call)
    
    def get_function_list(self) -> List[Dict[str, Any]]:
        """获取已注册函数列表"""
        with self._lock:
            functions = []
            for name, metadata in self._function_metadata.items():
                functions.append({
                    'name': name,
                    'description': metadata['description'],
                    'signature': metadata['signature'],
                    'expected_latency_ms': metadata['expected_latency_ms'],
                    'memory_intensive': metadata['memory_intensive'],
                    'call_count': self._call_counts.get(name, 0),
                    'avg_time_ms': self._call_metrics.get(name, {}).get('avg_time', 0.0)
                })
            return functions
    
    def get_metrics(self) -> Dict[str, Dict[str, Any]]:
        """获取性能指标"""
        with self._lock:
            return {
                name: {
                    'call_count': self._call_counts[name],
                    'total_time_ms': metrics['total_time'],
                    'avg_time_ms': metrics['avg_time'],
                    'min_time_ms': metrics['min_time'] if metrics['min_time'] != float('inf') else 0,
                    'max_time_ms': metrics['max_time']
                }
                for name, metrics in self._call_metrics.items()
            }
    
    def close(self):
        """关闭分派器"""
        self._thread_pool.shutdown(wait=True)
        
        if self.enable_metrics and self.debug:
            metrics = self.get_metrics()
            self.logger.info(f"直接调用分派器性能统计: {metrics}")
        
        self.logger.info("直接调用分派器已关闭")


# 全局分派器实例
_global_dispatcher: Optional[CallDispatcher] = None


def get_global_dispatcher() -> CallDispatcher:
    """获取全局分派器"""
    global _global_dispatcher
    if _global_dispatcher is None:
        _global_dispatcher = CallDispatcher()
    return _global_dispatcher


if __name__ == "__main__":
    # 测试代码
    print("=== 直接调用分派器测试 ===")
    
    dispatcher = CallDispatcher(debug=True)
    
    # 测试图像处理
    test_image = b"test_image_data" * 1000
    result = dispatcher.process_image_local(test_image, "compress")
    print(f"✅ 图像处理: 成功={result.success}, 耗时={result.duration_ms:.3f}ms")
    
    # 测试AI推理
    features = np.random.rand(100).astype(np.float32)
    config = {"model": "test_local"}
    result = dispatcher.ai_inference_local(features, config)
    print(f"✅ AI推理: 成功={result.success}, 预测={result.result}")
    
    # 测试特征提取
    result = dispatcher.extract_features_local(test_image)
    print(f"✅ 特征提取: 成功={result.success}, 特征数量={len(result.result) if result.result is not None else 0}")
    
    # 获取函数列表
    functions = dispatcher.get_function_list()
    print(f"📋 注册函数: {len(functions)}个")
    
    # 性能指标
    metrics = dispatcher.get_metrics()
    print(f"📊 性能指标: {metrics}")
    
    dispatcher.close()
    print("🎯 直接调用分派器测试完成！")
