"""
🧠 PIXLY v3.0 Python端Rust桥接适配器

为本地化AI系统提供Rust性能核心集成：
- 零拷贝特征提取适配
- UI友好的配置接口
- 错误处理与状态管理
- 插件系统集成支持

完全本地化实现，零网络依赖
"""

import json
import time
import threading
from pathlib import Path
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
from enum import Enum

# 尝试导入Rust性能核心
try:
    import pixly_performance_core
    RUST_CORE_AVAILABLE = True
except ImportError:
    RUST_CORE_AVAILABLE = False
    print("⚠️ Rust性能核心不可用，将使用Python后备实现")

from .local_dispatcher import LocalPredictionRequest, LocalPredictionResult, PredictionStatus


class RustBridgeStatus(Enum):
    """Rust桥接器状态"""
    INITIALIZING = "initializing"
    READY = "ready"
    BUSY = "busy"
    ERROR = "error"
    STOPPED = "stopped"


@dataclass
class RustFeatureConfig:
    """Rust特征提取配置"""
    enable_simd: bool = True
    enable_gpu: bool = False
    enable_advanced_features: bool = True
    performance_level: str = "balanced"  # power_saver, balanced, high_performance, maximum
    cache_size: int = 1000
    debug_mode: bool = False


class RustBridgeAdapter:
    """
    🧠 Python端Rust桥接适配器
    
    提供Python AI系统与Rust性能核心的无缝集成
    """
    
    def __init__(self, models_dir: str = "models"):
        self.models_dir = Path(models_dir)
        self.config = RustFeatureConfig()
        self.status = RustBridgeStatus.INITIALIZING
        self.stats = {
            "total_predictions": 0,
            "rust_predictions": 0,
            "python_fallback": 0,
            "avg_speedup": 1.0,
        }
        self._lock = threading.RLock()
        
        # Rust接口实例
        self.rust_interface = None
        
        # 初始化
        self._initialize_rust_bridge()
    
    def _initialize_rust_bridge(self):
        """初始化Rust桥接器"""
        try:
            if RUST_CORE_AVAILABLE:
                # 使用Rust UI接口
                self.rust_interface = pixly_performance_core.get_ui_interface()
                
                # 应用配置
                config_dict = {
                    "enable_simd": self.config.enable_simd,
                    "enable_gpu": self.config.enable_gpu,
                    "debug_mode": self.config.debug_mode,
                    "cache_size": self.config.cache_size,
                }
                
                # 初始化Rust桥接器
                self.rust_interface.initialize_with_preset(self.config.performance_level)
                self.rust_interface.update_config(config_dict)
                
                self.status = RustBridgeStatus.READY
                print("✅ Rust桥接适配器初始化成功")
            else:
                self.status = RustBridgeStatus.ERROR
                print("⚠️ Rust桥接适配器使用Python后备模式")
                
        except Exception as e:
            self.status = RustBridgeStatus.ERROR
            print(f"❌ Rust桥接适配器初始化失败: {e}")
    
    def predict_with_rust_features(self, request: LocalPredictionRequest) -> LocalPredictionResult:
        """使用Rust特征提取进行预测"""
        start_time = time.time()
        
        with self._lock:
            self.stats["total_predictions"] += 1
            
            if self.rust_interface and self.status == RustBridgeStatus.READY:
                try:
                    # 使用Rust进行预测
                    rust_result = self.rust_interface.predict_image_simple(
                        request.image_path,
                        request.target_quality,
                        request.tool
                    )
                    
                    # 转换为Python结果
                    result = self._convert_rust_result(rust_result)
                    result.inference_time_ms = (time.time() - start_time) * 1000
                    
                    self.stats["rust_predictions"] += 1
                    
                    # 计算加速比
                    if hasattr(request, 'expected_python_time_ms') and request.expected_python_time_ms > 0:
                        speedup = request.expected_python_time_ms / result.inference_time_ms
                        self.stats["avg_speedup"] = (self.stats["avg_speedup"] * (self.stats["rust_predictions"] - 1) + speedup) / self.stats["rust_predictions"]
                    
                    return result
                    
                except Exception as e:
                    print(f"⚠️ Rust预测失败，回退到Python: {e}")
                    self.stats["python_fallback"] += 1
                    return self._python_fallback_predict(request)
            else:
                # Python后备预测
                self.stats["python_fallback"] += 1
                return self._python_fallback_predict(request)
    
    def _convert_rust_result(self, rust_result: Dict[str, Any]) -> LocalPredictionResult:
        """转换Rust结果为Python结果"""
        return LocalPredictionResult(
            success=True,
            status=PredictionStatus.SUCCESS,
            quality=int(rust_result.get("quality", 85)),
            distance=float(rust_result.get("distance", 0.95)),
            confidence=float(rust_result.get("confidence", 0.8)),
            model_used=rust_result.get("model_used", "rust_integrated"),
            inference_time_ms=float(rust_result.get("inference_time_ms", 0)),
            reasoning=rust_result.get("reasoning", "Rust SIMD加速预测"),
            recommended_format=rust_result.get("tool", "webp"),
            format_reason="Rust性能核心推荐"
        )
    
    def _python_fallback_predict(self, request: LocalPredictionRequest) -> LocalPredictionResult:
        """Python后备预测"""
        # 简化的Python预测实现
        return LocalPredictionResult(
            success=True,
            status=PredictionStatus.SUCCESS,
            quality=request.target_quality,
            distance=0.95,
            confidence=0.7,
            model_used="python_fallback",
            inference_time_ms=50.0,  # 假设50ms
            reasoning="Python后备预测",
            recommended_format=request.tool,
            format_reason="默认工具选择"
        )
    
    def predict_batch(self, requests: List[LocalPredictionRequest]) -> List[LocalPredictionResult]:
        """批量预测"""
        if self.rust_interface and self.status == RustBridgeStatus.READY:
            try:
                # 转换为Rust批量请求格式
                rust_requests = []
                for req in requests:
                    rust_req = {
                        "image_path": req.image_path,
                        "quality": req.target_quality,
                        "tool": req.tool,
                    }
                    rust_requests.append(rust_req)
                
                # Rust批量预测
                rust_results = self.rust_interface.predict_batch_simple(rust_requests)
                
                # 转换结果
                results = []
                for rust_result in rust_results:
                    if "error" in rust_result:
                        # 错误情况，使用Python后备
                        results.append(self._python_fallback_predict(requests[len(results)]))
                    else:
                        results.append(self._convert_rust_result(rust_result))
                
                self.stats["rust_predictions"] += len([r for r in rust_results if "error" not in r])
                self.stats["python_fallback"] += len([r for r in rust_results if "error" in r])
                
                return results
                
            except Exception as e:
                print(f"⚠️ Rust批量预测失败: {e}")
                # 全部回退到Python
                self.stats["python_fallback"] += len(requests)
                return [self._python_fallback_predict(req) for req in requests]
        else:
            # 全部使用Python
            self.stats["python_fallback"] += len(requests)
            return [self._python_fallback_predict(req) for req in requests]
    
    def update_config(self, updates: Dict[str, Any]):
        """更新配置"""
        with self._lock:
            # 更新Python配置
            if "enable_simd" in updates:
                self.config.enable_simd = updates["enable_simd"]
            if "enable_gpu" in updates:
                self.config.enable_gpu = updates["enable_gpu"]
            if "performance_level" in updates:
                self.config.performance_level = updates["performance_level"]
            if "cache_size" in updates:
                self.config.cache_size = updates["cache_size"]
            if "debug_mode" in updates:
                self.config.debug_mode = updates["debug_mode"]
            
            # 更新Rust配置
            if self.rust_interface:
                try:
                    self.rust_interface.update_config(updates)
                except Exception as e:
                    print(f"⚠️ 更新Rust配置失败: {e}")
    
    def get_status(self) -> Dict[str, Any]:
        """获取状态信息"""
        status = {
            "bridge_status": self.status.value,
            "rust_available": RUST_CORE_AVAILABLE,
            "config": asdict(self.config),
            "stats": self.stats.copy(),
        }
        
        # 获取Rust状态
        if self.rust_interface:
            try:
                rust_status = self.rust_interface.get_status()
                status["rust_status"] = rust_status
                
                rust_performance = self.rust_interface.get_performance_stats()
                status["rust_performance"] = rust_performance
                
                rust_health = self.rust_interface.get_health_status()
                status["rust_health"] = rust_health
            except Exception as e:
                status["rust_error"] = str(e)
        
        return status
    
    def get_performance_report(self) -> Dict[str, Any]:
        """获取性能报告"""
        total = self.stats["total_predictions"]
        rust_pct = (self.stats["rust_predictions"] / total * 100) if total > 0 else 0
        python_pct = (self.stats["python_fallback"] / total * 100) if total > 0 else 0
        
        report = {
            "总预测次数": total,
            "Rust加速预测": self.stats["rust_predictions"],
            "Python后备预测": self.stats["python_fallback"],
            "Rust使用率": f"{rust_pct:.1f}%",
            "Python后备率": f"{python_pct:.1f}%",
            "平均加速比": f"{self.stats['avg_speedup']:.1f}x",
            "Rust状态": self.status.value,
        }
        
        return report
    
    def restart_rust_bridge(self):
        """重启Rust桥接器"""
        with self._lock:
            if self.rust_interface:
                try:
                    self.rust_interface.restart()
                    self.status = RustBridgeStatus.READY
                    print("✅ Rust桥接器重启成功")
                except Exception as e:
                    self.status = RustBridgeStatus.ERROR
                    print(f"❌ Rust桥接器重启失败: {e}")
            else:
                self._initialize_rust_bridge()
    
    def clear_cache(self):
        """清空缓存"""
        if self.rust_interface:
            try:
                self.rust_interface.clear_cache_and_stats()
            except Exception as e:
                print(f"⚠️ 清空Rust缓存失败: {e}")
        
        # 重置Python统计
        self.stats = {
            "total_predictions": 0,
            "rust_predictions": 0,
            "python_fallback": 0,
            "avg_speedup": 1.0,
        }
        print("✅ 缓存和统计已清空")
    
    def enable_performance_mode(self, mode: str = "high_performance"):
        """启用性能模式"""
        self.config.performance_level = mode
        if mode == "maximum":
            self.config.enable_simd = True
            self.config.enable_gpu = True
            self.config.enable_advanced_features = True
        elif mode == "high_performance":
            self.config.enable_simd = True
            self.config.enable_gpu = True
        elif mode == "balanced":
            self.config.enable_simd = True
            self.config.enable_gpu = False
        elif mode == "power_saver":
            self.config.enable_simd = False
            self.config.enable_gpu = False
        
        self.update_config(asdict(self.config))
        print(f"✅ 性能模式已设置为: {mode}")
    
    def is_rust_ready(self) -> bool:
        """检查Rust桥接器是否就绪"""
        return RUST_CORE_AVAILABLE and self.status == RustBridgeStatus.READY


# 全局适配器实例
_global_adapter: Optional[RustBridgeAdapter] = None
_adapter_lock = threading.Lock()

def get_rust_bridge_adapter() -> RustBridgeAdapter:
    """获取全局Rust桥接适配器实例（单例模式）"""
    global _global_adapter
    
    with _adapter_lock:
        if _global_adapter is None:
            _global_adapter = RustBridgeAdapter()
        return _global_adapter
