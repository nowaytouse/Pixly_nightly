"""
统一处理器 - Rust-Python融合的统一入口

提供单一API接口，智能调度Rust和Python处理器：
- 自动选择最优处理路径
- 负载均衡和故障转移
- 性能监控和优化建议
- 透明的融合处理

用户只需调用统一接口，底层自动优化执行路径
"""

import time
import asyncio
import threading
from typing import Dict, List, Optional, Any, Union, Callable, Tuple
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from concurrent.futures import ThreadPoolExecutor, Future
import numpy as np
import logging
import queue

from .rust_python_bridge import RustPythonBridge, get_global_bridge, FusionTask, ProcessorType


class TaskPriority(Enum):
    """任务优先级"""
    CRITICAL = 1    # 关键任务
    HIGH = 2        # 高优先级
    NORMAL = 3      # 普通优先级
    LOW = 4         # 低优先级
    BACKGROUND = 5  # 后台任务


@dataclass
class ProcessingRequest:
    """处理请求"""
    request_id: str
    operation_type: str  # 'image_process', 'ai_inference', 'batch_process'
    data: Any
    params: Dict[str, Any] = field(default_factory=dict)
    priority: TaskPriority = TaskPriority.NORMAL
    
    # 性能要求
    max_latency_ms: Optional[float] = None
    require_accuracy: bool = True
    
    # 回调
    callback: Optional[Callable] = None
    
    # 创建时间
    created_at: datetime = field(default_factory=datetime.now)


@dataclass
class ProcessingResult:
    """处理结果"""
    request_id: str
    success: bool
    result: Any
    processing_time_ms: float
    processor_used: str
    
    # 性能指标
    throughput_ops_per_sec: float = 0.0
    memory_usage_mb: float = 0.0
    cpu_utilization_percent: float = 0.0
    
    # 错误信息
    error: Optional[str] = None
    
    # 完成时间
    completed_at: datetime = field(default_factory=datetime.now)


class LoadBalancer:
    """负载均衡器"""
    
    def __init__(self):
        self.rust_load = 0.0
        self.python_load = 0.0
        self.rust_queue_size = 0
        self.python_queue_size = 0
        self._lock = threading.RLock()
    
    def get_optimal_processor(self, 
                            task_type: str,
                            data_size: int,
                            priority: TaskPriority) -> ProcessorType:
        """获取最优处理器"""
        
        with self._lock:
            # 基于当前负载选择
            if self.rust_load < self.python_load * 0.7:  # Rust负载低30%
                if task_type in ['image_compress', 'image_resize', 'math_compute']:
                    return ProcessorType.RUST_NATIVE
            
            if self.python_load < self.rust_load * 0.7:  # Python负载低30%
                if task_type in ['ai_inference', 'image_analyze', 'classify']:
                    return ProcessorType.PYTHON_AI
            
            # 基于任务特性选择
            if data_size > 1000000:  # 大数据倾向Rust
                return ProcessorType.RUST_NATIVE
            elif task_type.startswith('ai_'):  # AI任务倾向Python
                return ProcessorType.PYTHON_AI
            
            # 默认混合处理
            return ProcessorType.HYBRID
    
    def update_load(self, processor: str, delta: float):
        """更新负载"""
        with self._lock:
            if processor.startswith('rust'):
                self.rust_load = max(0.0, self.rust_load + delta)
            else:
                self.python_load = max(0.0, self.python_load + delta)
    
    def get_stats(self) -> Dict[str, float]:
        """获取负载统计"""
        with self._lock:
            return {
                'rust_load': self.rust_load,
                'python_load': self.python_load,
                'rust_queue_size': self.rust_queue_size,
                'python_queue_size': self.python_queue_size,
                'load_balance_ratio': self.rust_load / (self.python_load + 0.001)
            }


class UnifiedProcessor:
    """
    统一处理器 - Rust-Python融合的统一入口
    
    提供简单易用的API，自动优化执行路径
    """
    
    def __init__(self, 
                 max_workers: int = 8,
                 enable_async: bool = True,
                 debug: bool = False):
        """
        初始化统一处理器
        
        Args:
            max_workers: 最大工作线程数
            enable_async: 启用异步处理
            debug: 调试模式
        """
        self.max_workers = max_workers
        self.enable_async = enable_async
        self.debug = debug
        
        # 核心组件
        self.bridge = get_global_bridge()
        self.load_balancer = LoadBalancer()
        
        # 线程池
        self.executor = ThreadPoolExecutor(max_workers=max_workers)
        
        # 任务队列
        self._priority_queues = {
            TaskPriority.CRITICAL: queue.PriorityQueue(),
            TaskPriority.HIGH: queue.PriorityQueue(), 
            TaskPriority.NORMAL: queue.PriorityQueue(),
            TaskPriority.LOW: queue.PriorityQueue(),
            TaskPriority.BACKGROUND: queue.PriorityQueue()
        }
        
        # 结果存储
        self._results: Dict[str, ProcessingResult] = {}
        self._pending_requests: Dict[str, Future] = {}
        
        # 统计信息
        self._stats = {
            'total_requests': 0,
            'completed_requests': 0,
            'failed_requests': 0,
            'total_processing_time_ms': 0.0,
            'avg_processing_time_ms': 0.0,
            'throughput_ops_per_sec': 0.0
        }
        
        # 线程安全
        self._lock = threading.RLock()
        
        # 日志
        self.logger = logging.getLogger(__name__)
        
        # 启动后台处理
        self._running = True
        self._worker_thread = threading.Thread(target=self._process_queue, daemon=True)
        self._worker_thread.start()
        
        if debug:
            self.logger.debug("统一处理器初始化完成")
    
    def process_image(self,
                     image_data: Union[bytes, np.ndarray, str],
                     operation: str,
                     priority: TaskPriority = TaskPriority.NORMAL,
                     **params) -> str:
        """
        处理图像 - 统一接口
        
        Args:
            image_data: 图像数据 (字节、数组或路径)
            operation: 操作类型 (compress, resize, analyze, enhance, classify)
            priority: 任务优先级
            **params: 额外参数
            
        Returns:
            request_id: 请求ID，用于获取结果
        """
        
        # 处理图像数据输入
        if isinstance(image_data, str):
            # 文件路径
            with open(image_data, 'rb') as f:
                image_data = f.read()
        
        # 创建处理请求
        request_id = f"image_{operation}_{int(time.time() * 1000000)}"
        
        request = ProcessingRequest(
            request_id=request_id,
            operation_type=f"image_{operation}",
            data=image_data,
            params=params,
            priority=priority
        )
        
        # 提交到队列
        self._submit_request(request)
        
        return request_id
    
    def ai_inference(self,
                    features: Union[np.ndarray, List, Dict],
                    model_config: Dict[str, Any],
                    priority: TaskPriority = TaskPriority.HIGH,
                    **params) -> str:
        """
        AI推理 - 统一接口
        
        Args:
            features: 特征数据
            model_config: 模型配置
            priority: 任务优先级
            **params: 额外参数
            
        Returns:
            request_id: 请求ID
        """
        
        # 标准化特征数据
        if isinstance(features, list):
            features = np.array(features, dtype=np.float32)
        elif isinstance(features, dict):
            # 从字典构建特征向量
            features = np.array(list(features.values()), dtype=np.float32)
        
        request_id = f"ai_inference_{int(time.time() * 1000000)}"
        
        request = ProcessingRequest(
            request_id=request_id,
            operation_type="ai_inference",
            data={'features': features, 'model_config': model_config},
            params=params,
            priority=priority
        )
        
        self._submit_request(request)
        
        return request_id
    
    def batch_process(self,
                     items: List[Dict[str, Any]],
                     operation: str,
                     priority: TaskPriority = TaskPriority.NORMAL,
                     **params) -> str:
        """
        批量处理 - 统一接口
        
        Args:
            items: 待处理项目列表
            operation: 操作类型
            priority: 任务优先级
            **params: 额外参数
            
        Returns:
            request_id: 请求ID
        """
        
        request_id = f"batch_{operation}_{int(time.time() * 1000000)}"
        
        request = ProcessingRequest(
            request_id=request_id,
            operation_type=f"batch_{operation}",
            data=items,
            params=params,
            priority=priority
        )
        
        self._submit_request(request)
        
        return request_id
    
    def _submit_request(self, request: ProcessingRequest):
        """提交请求到队列"""
        
        with self._lock:
            self._stats['total_requests'] += 1
            
            # 添加到优先级队列
            priority_queue = self._priority_queues[request.priority]
            priority_queue.put((request.priority.value, time.time(), request))
            
            # 更新队列大小统计
            if request.operation_type.startswith('ai_'):
                self.load_balancer.python_queue_size += 1
            else:
                self.load_balancer.rust_queue_size += 1
        
        if self.debug:
            self.logger.debug(f"请求已提交: {request.request_id}, 优先级: {request.priority.name}")
    
    def _process_queue(self):
        """后台队列处理"""
        
        while self._running:
            try:
                # 按优先级处理
                for priority in TaskPriority:
                    priority_queue = self._priority_queues[priority]
                    
                    if not priority_queue.empty():
                        try:
                            _, timestamp, request = priority_queue.get_nowait()
                            
                            # 提交到线程池处理
                            future = self.executor.submit(self._execute_request, request)
                            self._pending_requests[request.request_id] = future
                            
                        except queue.Empty:
                            continue
                        
                        break  # 处理一个后跳出，保持优先级顺序
                
                # 检查已完成的任务
                self._check_completed_tasks()
                
                # 短暂休眠
                time.sleep(0.001)
                
            except Exception as e:
                self.logger.error(f"队列处理错误: {e}")
    
    def _execute_request(self, request: ProcessingRequest) -> ProcessingResult:
        """执行请求"""
        
        start_time = time.perf_counter()
        
        try:
            # 选择最优处理器
            processor_type = self.load_balancer.get_optimal_processor(
                request.operation_type,
                self._estimate_data_size(request.data),
                request.priority
            )
            
            # 更新负载
            processor_name = processor_type.value
            self.load_balancer.update_load(processor_name, 1.0)
            
            # 执行处理
            if request.operation_type.startswith('image_'):
                # 图像处理
                operation = request.operation_type.replace('image_', '')
                fusion_task = self.bridge.process_image_fusion(
                    request.data,
                    operation,
                    **request.params
                )
                
                result_data = fusion_task.result
                processor_used = fusion_task.processor_used or processor_name
                
            elif request.operation_type == 'ai_inference':
                # AI推理
                fusion_task = self.bridge.ai_inference_fusion(
                    request.data['features'],
                    request.data['model_config']
                )
                
                result_data = fusion_task.result
                processor_used = fusion_task.processor_used or processor_name
                
            elif request.operation_type.startswith('batch_'):
                # 批量处理
                result_data = self._process_batch(request)
                processor_used = "batch_processor"
                
            else:
                raise ValueError(f"不支持的操作类型: {request.operation_type}")
            
            processing_time = (time.perf_counter() - start_time) * 1000
            
            # 计算性能指标
            throughput = 1000.0 / processing_time if processing_time > 0 else 0
            
            result = ProcessingResult(
                request_id=request.request_id,
                success=True,
                result=result_data,
                processing_time_ms=processing_time,
                processor_used=processor_used,
                throughput_ops_per_sec=throughput
            )
            
            # 更新统计
            with self._lock:
                self._stats['completed_requests'] += 1
                self._stats['total_processing_time_ms'] += processing_time
                
                if self._stats['completed_requests'] > 0:
                    self._stats['avg_processing_time_ms'] = (
                        self._stats['total_processing_time_ms'] / self._stats['completed_requests']
                    )
                
                # 更新吞吐量 (最近10个请求的平均)
                recent_avg = self._stats['avg_processing_time_ms']
                if recent_avg > 0:
                    self._stats['throughput_ops_per_sec'] = 1000.0 / recent_avg
            
            # 降低负载
            self.load_balancer.update_load(processor_name, -0.8)
            
            if self.debug:
                self.logger.debug(f"请求完成: {request.request_id}, 耗时: {processing_time:.3f}ms")
            
        except Exception as e:
            processing_time = (time.perf_counter() - start_time) * 1000
            
            result = ProcessingResult(
                request_id=request.request_id,
                success=False,
                result=None,
                processing_time_ms=processing_time,
                processor_used="error",
                error=str(e)
            )
            
            # 更新失败统计
            with self._lock:
                self._stats['failed_requests'] += 1
            
            # 降低负载
            self.load_balancer.update_load(processor_type.value, -1.0)
            
            self.logger.error(f"请求失败: {request.request_id}, 错误: {e}")
        
        return result
    
    def _process_batch(self, request: ProcessingRequest) -> Dict[str, Any]:
        """处理批量任务"""
        
        items = request.data
        operation = request.operation_type.replace('batch_', '')
        
        results = []
        total_items = len(items)
        
        # 并行处理批量项目
        batch_futures = []
        
        for i, item in enumerate(items):
            if operation == 'image_process':
                future = self.executor.submit(
                    self.bridge.process_image_fusion,
                    item['image_data'],
                    item['operation'],
                    **item.get('params', {})
                )
            elif operation == 'ai_inference':
                future = self.executor.submit(
                    self.bridge.ai_inference_fusion,
                    item['features'],
                    item['model_config']
                )
            else:
                # 简单处理
                future = self.executor.submit(lambda x: {'processed': True, 'item': x}, item)
            
            batch_futures.append((i, future))
        
        # 收集结果
        for i, future in batch_futures:
            try:
                result = future.result(timeout=30)  # 30秒超时
                results.append({
                    'index': i,
                    'success': True,
                    'result': result.result if hasattr(result, 'result') else result
                })
            except Exception as e:
                results.append({
                    'index': i,
                    'success': False,
                    'error': str(e)
                })
        
        return {
            'total_items': total_items,
            'processed_items': len(results),
            'success_items': sum(1 for r in results if r['success']),
            'failed_items': sum(1 for r in results if not r['success']),
            'results': results
        }
    
    def _estimate_data_size(self, data: Any) -> int:
        """估算数据大小"""
        
        if isinstance(data, bytes):
            return len(data)
        elif isinstance(data, np.ndarray):
            return data.nbytes
        elif isinstance(data, str):
            return len(data.encode('utf-8'))
        elif isinstance(data, (list, tuple)):
            return len(data) * 100  # 估算
        elif isinstance(data, dict):
            return sum(self._estimate_data_size(v) for v in data.values())
        else:
            return 1000  # 默认估算
    
    def _check_completed_tasks(self):
        """检查已完成的任务"""
        
        completed_ids = []
        
        for request_id, future in self._pending_requests.items():
            if future.done():
                try:
                    result = future.result()
                    self._results[request_id] = result
                except Exception as e:
                    # 创建错误结果
                    error_result = ProcessingResult(
                        request_id=request_id,
                        success=False,
                        result=None,
                        processing_time_ms=0.0,
                        processor_used="error",
                        error=str(e)
                    )
                    self._results[request_id] = error_result
                
                completed_ids.append(request_id)
        
        # 清理已完成的任务
        for request_id in completed_ids:
            del self._pending_requests[request_id]
    
    def get_result(self, request_id: str, timeout: Optional[float] = None) -> Optional[ProcessingResult]:
        """
        获取处理结果
        
        Args:
            request_id: 请求ID
            timeout: 超时时间(秒)
            
        Returns:
            ProcessingResult: 处理结果
        """
        
        start_time = time.time()
        
        while True:
            # 检查是否已完成
            if request_id in self._results:
                return self._results[request_id]
            
            # 检查超时
            if timeout and (time.time() - start_time) > timeout:
                return None
            
            # 检查是否还在处理中
            if request_id not in self._pending_requests:
                return None
            
            time.sleep(0.01)  # 10ms轮询
    
    def wait_for_completion(self, request_ids: List[str], timeout: Optional[float] = None) -> Dict[str, ProcessingResult]:
        """等待多个请求完成"""
        
        results = {}
        start_time = time.time()
        
        while len(results) < len(request_ids):
            for request_id in request_ids:
                if request_id not in results:
                    result = self.get_result(request_id, timeout=0.1)
                    if result:
                        results[request_id] = result
            
            # 检查超时
            if timeout and (time.time() - start_time) > timeout:
                break
            
            time.sleep(0.01)
        
        return results
    
    def get_status(self) -> Dict[str, Any]:
        """获取处理器状态"""
        
        with self._lock:
            load_stats = self.load_balancer.get_stats()
            
            return {
                'running': self._running,
                'max_workers': self.max_workers,
                'pending_requests': len(self._pending_requests),
                'completed_results': len(self._results),
                'statistics': dict(self._stats),
                'load_balancer': load_stats,
                'queue_sizes': {
                    priority.name: self._priority_queues[priority].qsize()
                    for priority in TaskPriority
                }
            }
    
    def clear_results(self, older_than_minutes: int = 60):
        """清理旧结果"""
        
        cutoff_time = datetime.now().timestamp() - (older_than_minutes * 60)
        
        cleared_count = 0
        for request_id, result in list(self._results.items()):
            if result.completed_at.timestamp() < cutoff_time:
                del self._results[request_id]
                cleared_count += 1
        
        if self.debug:
            self.logger.debug(f"清理了 {cleared_count} 个旧结果")
    
    def shutdown(self):
        """关闭处理器"""
        
        self._running = False
        
        # 等待工作线程结束
        if self._worker_thread.is_alive():
            self._worker_thread.join(timeout=5)
        
        # 关闭线程池
        self.executor.shutdown(wait=True)
        
        # 关闭桥接器
        self.bridge.shutdown()
        
        self.logger.info("统一处理器已关闭")


# 全局实例
_global_processor: Optional[UnifiedProcessor] = None


def get_unified_processor() -> UnifiedProcessor:
    """获取全局统一处理器"""
    global _global_processor
    if _global_processor is None:
        _global_processor = UnifiedProcessor()
    return _global_processor


if __name__ == "__main__":
    # 测试代码
    print("=== 统一处理器测试 ===")
    
    processor = UnifiedProcessor(debug=True)
    
    # 测试图像处理
    print("🖼️ 测试图像处理...")
    test_image = b"fake_image_data" * 1000
    
    compress_id = processor.process_image(test_image, 'compress', quality=80)
    analyze_id = processor.process_image(test_image, 'analyze')
    
    # 等待结果
    compress_result = processor.get_result(compress_id, timeout=5)
    analyze_result = processor.get_result(analyze_id, timeout=5)
    
    print(f"  压缩结果: {compress_result.success if compress_result else False}")
    print(f"  分析结果: {analyze_result.success if analyze_result else False}")
    
    # 测试AI推理
    print("🧠 测试AI推理...")
    test_features = np.random.rand(100).astype(np.float32)
    inference_id = processor.ai_inference(test_features, {'model': 'test_model'})
    
    inference_result = processor.get_result(inference_id, timeout=5)
    print(f"  推理结果: {inference_result.success if inference_result else False}")
    
    # 测试批量处理
    print("📦 测试批量处理...")
    batch_items = [
        {'image_data': b"test1" * 100, 'operation': 'compress'},
        {'image_data': b"test2" * 100, 'operation': 'compress'},
        {'image_data': b"test3" * 100, 'operation': 'compress'}
    ]
    
    batch_id = processor.batch_process(batch_items, 'image_process')
    batch_result = processor.get_result(batch_id, timeout=10)
    
    print(f"  批量处理结果: {batch_result.success if batch_result else False}")
    if batch_result and batch_result.success:
        batch_data = batch_result.result
        print(f"  处理项目: {batch_data['processed_items']}/{batch_data['total_items']}")
    
    # 状态信息
    status = processor.get_status()
    print(f"📊 处理器状态:")
    print(f"  待处理请求: {status['pending_requests']}")
    print(f"  已完成结果: {status['completed_results']}")
    print(f"  平均耗时: {status['statistics']['avg_processing_time_ms']:.3f}ms")
    print(f"  吞吐量: {status['statistics']['throughput_ops_per_sec']:.1f} ops/sec")
    
    processor.shutdown()
    print("🎯 统一处理器测试完成！")
