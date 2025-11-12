#!/usr/bin/env python3
"""
Pixly AI Parameter Prediction Script
Version: 4.5.0

用于Go服务调用的Python推理脚本
Phase 46.14+: 添加无损转码检测 (P-002)
Phase 46.14+: 添加视频参数AI预测 (F-002)
Phase 47.7+: 添加Magika AI文件检测 (安全增强)
"""

import json
import sys
import os
from pathlib import Path
import numpy as np
from PIL import Image
import pywt
import lightgbm as lgb
import time

# ✅ Phase 47.22: 导入ModelRouter
try:
    sys.path.insert(0, str(Path(__file__).parent.parent / 'core' / 'python'))
    from ai.model_router import ModelRouter, ModelInfo
    MODEL_ROUTER_AVAILABLE = True
except ImportError:
    MODEL_ROUTER_AVAILABLE = False

# sklearn是可选依赖（仅用于加载旧模型的scaler）
try:
    from sklearn.preprocessing import StandardScaler
    SKLEARN_AVAILABLE = True
except ImportError:
    SKLEARN_AVAILABLE = False
    # 简单的标准化器替代
    class StandardScaler:
        def __init__(self):
            self.mean_ = None
            self.scale_ = None
        
        def transform(self, X):
            if self.mean_ is None or self.scale_ is None:
                return X
            return (X - self.mean_) / self.scale_

# 🆕 Magika AI检测（可选依赖）
try:
    from magika import Magika
    MAGIKA_AVAILABLE = True
except ImportError:
    MAGIKA_AVAILABLE = False

# ✅ 导入统一日志系统 (Phase 46.14+)
try:
    from pixly_logging import log_info, log_warn, log_error, log_debug, ErrorCode
    PIXLY_LOGGING_AVAILABLE = True
except ImportError:
    PIXLY_LOGGING_AVAILABLE = False
    # 回退到标准print（开发环境）
    def log_info(component, message, **context):
        print(f"✅ [{component}] {message}", file=sys.stderr)
    def log_warn(component, message, **context):
        print(f"⚠️  [{component}] {message}", file=sys.stderr)
    def log_error(component, message, **context):
        print(f"❌ [{component}] {message}", file=sys.stderr)
    def log_debug(component, message, **context):
        print(f"🔍 [{component}] {message}", file=sys.stderr)

# ============================================================================
# 质量范围配置（与Go保持一致）
# ============================================================================

QUALITY_RANGES = {
    'size': {
        'min': 70, 'max': 85, 'default': 80,
        'distance_min': 0.5, 'distance_max': 1.5
    },
    'balanced': {
        'min': 85, 'max': 95, 'default': 90,
        'distance_min': 0.3, 'distance_max': 0.7
    },
    'quality': {
        'min': 95, 'max': 100, 'default': 100,
        'distance_min': 0.0, 'distance_max': 0.2
    }
}

def constrain_quality(quality, mode):
    """将质量值收束到范围内"""
    range_cfg = QUALITY_RANGES.get(mode, QUALITY_RANGES['balanced'])
    constrained = max(range_cfg['min'], min(range_cfg['max'], quality))
    if constrained != quality:
        print(f"⚠️  质量收束: Q={quality} → Q={constrained} (模式={mode})", file=sys.stderr)
    return constrained

def constrain_distance(distance, mode):
    """将distance值收束到范围内"""
    range_cfg = QUALITY_RANGES.get(mode, QUALITY_RANGES['balanced'])
    constrained = max(range_cfg['distance_min'], min(range_cfg['distance_max'], distance))
    if abs(constrained - distance) > 0.01:
        print(f"⚠️  Distance收束: d={distance:.2f} → d={constrained:.2f} (模式={mode})", file=sys.stderr)
    return constrained

class SWTExtractor:
    """SWT特征提取"""
    
    def __init__(self, wavelet='db1', max_level=3):
        self.wavelet = wavelet
        self.max_level = max_level
    
    def _prepare_image(self, img_array):
        """预处理图像：padding到偶数尺寸，计算最大可用层级"""
        h, w = img_array.shape
        
        # Padding到偶数尺寸
        new_h = h if h % 2 == 0 else h + 1
        new_w = w if w % 2 == 0 else w + 1
        
        if new_h != h or new_w != w:
            padded = np.zeros((new_h, new_w), dtype=img_array.dtype)
            padded[:h, :w] = img_array
            img_array = padded
        
        # 计算最大可用层级
        min_dim = min(new_h, new_w)
        max_possible_level = int(np.floor(np.log2(min_dim)))
        actual_level = min(self.max_level, max_possible_level, 3)
        
        return img_array, max(1, actual_level)
    
    def extract(self, img_array):
        """从numpy数组提取SWT特征"""
        # 预处理图像
        img_array, actual_level = self._prepare_image(img_array)
        
        try:
            coeffs = pywt.swt2(img_array, self.wavelet, level=actual_level)
        except Exception:
            coeffs = pywt.swt2(img_array, self.wavelet, level=1)
        
        features = {}
        
        # 获取实际层数
        num_levels = len(coeffs)
        
        # 边缘强度
        edge_strength = sum(np.std(cH) + np.std(cV) for _, (cH, cV, _) in coeffs) / num_levels
        features['edge_strength'] = float(edge_strength)
        
        # 纹理复杂度
        texture_complexity = sum(np.std(cD) for _, (_, _, cD) in coeffs) / num_levels
        features['texture_complexity'] = float(texture_complexity)
        
        # 噪声水平
        _, (_, _, cD) = coeffs[0]
        noise_level = np.median(np.abs(cD - np.median(cD)))
        features['noise_level'] = float(noise_level)
        
        # 细节层次
        detail_energies = [np.sum(cH**2 + cV**2 + cD**2) for _, (cH, cV, cD) in coeffs]
        features['detail_level'] = float(np.std(detail_energies))
        
        # 频率能量分布
        total_energy = sum(detail_energies)
        if total_energy > 0:
            features['high_freq_energy'] = float(detail_energies[0] / total_energy)
            features['low_freq_energy'] = float(detail_energies[-1] / total_energy)
        else:
            features['high_freq_energy'] = 0
            features['low_freq_energy'] = 0
        
        features['compression_score'] = float(
            0.5 * (1 - features['texture_complexity'] / 100) +
            0.3 * (1 - features['noise_level'] / 50) +
            0.2 * features['low_freq_energy']
        )
        
        return features

class AIPredictor:
    """图像AI参数预测器（统一策略 + 模型路由）"""
    
    def __init__(self, model_dir='models', enable_model_router=True):
        self.model_dir = Path(model_dir)
        self.models = {}  # 存储各个工具的模型
        self.scalers = {}  # 存储各个工具的标准化器
        
        # Phase 47.17: 预测结果缓存
        # 使用LRU缓存避免重复预测，最多缓存100个结果
        from functools import lru_cache
        self._prediction_cache = {}  # 格式: {cache_key: result}
        self.lgb_model = None
        self.scaler = None
        
        # ✅ Phase 47.22: ModelRouter初始化
        self.model_router = None
        if enable_model_router and MODEL_ROUTER_AVAILABLE:
            router_config_path = self.model_dir / 'model_router_config.json'
            self.model_router = ModelRouter(config_path=router_config_path if router_config_path.exists() else None)
            log_info('AIPredictor', f"ModelRouter initialized: {MODEL_ROUTER_AVAILABLE}")
            
            # 注册LightGBM模型
            if (self.model_dir / 'pixly_lightgbm_model.txt').exists():
                self.model_router.register_model(ModelInfo(
                    name="lightgbm",
                    version="v1.0.0",
                    path=str(self.model_dir / 'pixly_lightgbm_model.txt'),
                    model_type="predictor",
                    status="active",
                    priority=1
                ))
        
        # ✅ PPO强化学习模型 (Phase 47.9+)
        self.ppo_available = False
        self.ppo_actor = None
        ppo_model_path = self.model_dir / 'ppo' / 'actor_network.pth'
        try:
            import torch
            if ppo_model_path.exists():
                self.ppo_available = True
                log_info('AIPredictor', '✅ PPO强化学习可用（PyTorch + 模型已就绪）')
        except ImportError:
            log_warn('AIPredictor', '⚠️  PPO强化学习不可用：缺少PyTorch依赖')
            log_warn('AIPredictor', '   安装方法: pip install torch')
            log_warn('AIPredictor', '   将使用LightGBM基础预测（精度略低）')
        
        # 尝试加载模型
        self._load_models()
        
        # 🆕 Magika AI检测器（默认启用）
        if MAGIKA_AVAILABLE:
            try:
                self.magika = Magika()
                log_info("AIPredictor", "Magika AI detector initialized")
            except Exception as e:
                log_error('AIPredictor', f"Failed to load LightGBM: {e}")
                self.lgb_model = None
                if self.model_router:
                    self.model_router.update_metrics("lightgbm", "v1.0.0", False, 0)
        else:
            self.magika = None
            log_warn("AIPredictor", "Magika not available, AI detection disabled")
    
    def _detect_lossless_transcode(self, image_path, tool):
        """检测是否适合无损转码
        
        检测规则：
        1. JPEG → JXL: 可以无损转码（lossless_jpeg=True）
        2. PNG → AVIF/WebP: 可以无损转码（lossless=True）
        3. 已压缩格式（JPEG/WebP/AVIF）→ 其他格式: 不建议（会损失质量）
        
        Returns:
            dict: {'lossless': bool, 'lossless_jpeg': bool, 'reason': str}
        """
        try:
            img = Image.open(image_path)
            source_format = img.format.upper() if img.format else 'UNKNOWN'
            
            result = {
                'lossless': False,
                'lossless_jpeg': False,
                'reason': '',
                'source_format': source_format
            }
            
            # JPEG → JXL 无损转码
            if source_format == 'JPEG' and tool in ['jxl', 'cjxl']:
                result['lossless_jpeg'] = True
                result['reason'] = 'JPEG to JXL lossless transcode'
                log_info("lossless_detector", 
                        "JPEG无损转码检测",
                        source=source_format,
                        target=tool,
                        mode="lossless_jpeg")
                return result
            
            # PNG → AVIF/WebP/JXL 编码策略
            if source_format == 'PNG' and tool in ['avif', 'avifenc', 'webp', 'cwebp', 'jxl', 'cjxl']:
                # 🎯 智能判断PNG类型和大小
                try:
                    # 直接使用已经打开的img对象
                    file_size = os.path.getsize(image_path) if image_path else 0
                    
                    if img and file_size > 0:
                        # 获取图像特征
                        width, height = img.size
                        pixels = width * height
                        unique_colors = len(img.getcolors(maxcolors=256*256)) if img.getcolors(maxcolors=256*256) else 999999
                        
                        # 🎯 检测图形/文字类PNG（色彩少、尺寸大、文件小）
                        # 这类PNG压缩效果已很好，转换反而增大
                        is_graphic = (
                            pixels > 500000 and  # 大尺寸（>500K像素）
                            file_size < 102400 and  # 文件小（<100KB）
                            unique_colors < 5000  # 颜色少（<5000种）
                        )
                        
                        if is_graphic:
                            # 图形/文字PNG：保持原格式最优
                            result['lossless'] = False
                            result['keep_original'] = True
                            result['reason'] = f'Graphic PNG (high compression, {unique_colors} colors), keep original'
                            log_info("lossless_detector",
                                    "PNG图形检测（保持原格式）",
                                    source=source_format,
                                    pixels=pixels,
                                    file_size=f'{file_size//1024}KB',
                                    colors=unique_colors,
                                    mode="keep_original")
                            return result
                        
                        # 小PNG但非图形：使用有损压缩
                        if file_size < 102400:  # <100KB
                            result['lossless'] = False
                            result['reason'] = f'Small PNG ({file_size//1024}KB), lossy encode for better compression'
                            # 更激进的质量调整：<30KB用-30，<60KB用-25，其他用-20
                            if file_size < 30720:
                                result['quality_adjustment'] = -30
                            elif file_size < 61440:
                                result['quality_adjustment'] = -25
                            else:
                                result['quality_adjustment'] = -20
                            log_info("lossless_detector",
                                    "PNG有损编码（小文件优化）",
                                    source=source_format,
                                    target=tool,
                                    file_size=f'{file_size//1024}KB',
                                    mode="lossy")
                            return result
                except:
                    pass
                
                # 大PNG或无法分析：使用无损编码
                result['lossless'] = True
                result['reason'] = f'PNG to {tool} lossless encode'
                log_info("lossless_detector",
                        "PNG无损编码检测",
                        source=source_format,
                        target=tool,
                        mode="lossless")
                return result
            
            # 已压缩格式警告
            if source_format in ['JPEG', 'WEBP'] and tool in ['avif', 'avifenc', 'webp', 'cwebp']:
                result['reason'] = f'{source_format} to {tool} will cause quality loss (re-compression)'
                log_warn("lossless_detector",
                        "检测到重新压缩，会损失质量",
                        source=source_format,
                        target=tool,
                        recommendation="Consider using lossless mode or keeping original format")
            
            return result
            
        except Exception as e:
            log_warn("lossless_detector",
                    f"无损转码检测失败: {e}",
                    error=str(e))
            return {'lossless': False, 'lossless_jpeg': False, 'reason': f'Detection failed: {e}'}
    
    def _load_models(self):
        """加载LightGBM模型"""
        # 加载LightGBM模型（使用ModelRouter）
        model_path = self.model_dir / 'pixly_lightgbm_model.txt'
        if model_path.exists():
            try:
                self.lgb_model = lgb.Booster(model_file=str(model_path))
                log_info('AIPredictor', f"LightGBM model loaded: {model_path}")
                
                # 更新ModelRouter指标
                if self.model_router:
                    self.model_router.update_metrics("lightgbm", "v1.0.0", True, 0)
            except Exception as e:
                print(f"⚠️  无法加载 LightGBM 模型: {e}", file=sys.stderr)
    
    def predict(self, image_path, tool, target_quality=90, optimize_mode='balanced', options=None):
        """
        图像参数AI预测（统一策略）
        
        Args:
            image_path: 图片路径
            tool: 转换工具 (jxl, webp, avif等)
            target_quality: 目标质量
            optimize_mode: 优化模式 (balanced/quality) - 仅quality和balanced
            options: 可选参数 {
                - expected_format: 期望输出格式（优先级最高）
                - enable_format_recommendation: 启用格式推荐（默认True）
                - enable_preprocessing: 启用预处理建议（默认True）
                - enable_magika: 启用Magika AI检测（默认True）
                - use_cache: 启用缓存（默认True）
            }
        
        Returns:
            预测的参数字典，包含quality、speed、effort等
        """
        options = options or {}
        use_cache = options.get('use_cache', True)
        
        # 🔥 Phase 47.17: 缓存检查
        cache_key = None
        if use_cache:
            # 生成缓存键（基于文件路径、工具、质量和模式）
            cache_key = f"{image_path}:{tool}:{target_quality}:{optimize_mode}"
            
            # 检查缓存
            if cache_key in self._prediction_cache:
                self._cache_hits += 1
                log_info('ImageAI', f"Cache hit! ({self._cache_hits} hits, {self._cache_misses} misses)")
                cached_result = self._prediction_cache[cache_key].copy()
                cached_result['cached'] = True
                return cached_result
            
            self._cache_misses += 1
        expected_format = options.get('expected_format', None)
        enable_format_recommendation = options.get('enable_format_recommendation', True)
        enable_preprocessing = options.get('enable_preprocessing', True)
        enable_magika = options.get('enable_magika', True)  # 🆕 默认启用
        
        # 🆕 Magika AI文件检测（增强安全性）
        magika_result = {}
        if enable_magika and self.magika:
            try:
                detection = self.magika.identify_path(Path(image_path))
                magika_result = {
                    'detected_type': detection.output.ct_label,
                    'confidence': detection.output.score,
                    'mime_type': detection.output.mime_type,
                    'is_image': detection.output.group == 'image'
                }
                log_info('ImageAI', f"Magika: {detection.output.ct_label} ({detection.output.score:.2%})")
                
                # 安全验证
                if not magika_result['is_image']:
                    log_warn('ImageAI', f"File is not image: {magika_result['detected_type']}")
            except Exception as e:
                log_warn('ImageAI', f"Magika detection failed: {e}")
        
        # 其他选项
        recommend_video_for_animation = options.get('recommend_video_for_animation', False)
        # 🎯 PPO默认启用（如果可用），用户可以选择禁用
        enable_ppo = options.get('enable_ppo', True)
        
        # 🔥 Phase 47.16: 修复Image.open可能卡死的问题
        # 使用线程超时（兼容macOS）
        import time
        import threading
        from concurrent.futures import ThreadPoolExecutor, TimeoutError as FutureTimeoutError
        
        # 检查文件大小（超过100MB警告）
        file_size = Path(image_path).stat().st_size
        if file_size > 100 * 1024 * 1024:  # 100MB
            log_warn('ImageAI', f"Large file warning: {file_size / 1024 / 1024:.1f}MB")
            # 对于超大文件，返回默认参数
            if file_size > 500 * 1024 * 1024:  # 500MB
                log_error('ImageAI', f"File too large (>500MB), using defaults")
                return {
                    'quality': 85,
                    'effort': 7,
                    'speed': 7,
                    'recommended_format': tool,
                    'confidence': 0.5,
                    'warning': 'File too large for AI analysis'
                }
        
        # 使用线程池执行可能卡死的操作
        def load_image_with_timeout(path):
            """在线程中加载图像，避免主线程卡死"""
            start = time.time()
            img = Image.open(path)
            # 强制加载图像数据
            img.load()
            
            # 🔥 Phase 47.17: 智能缩放优化 - 大图像缩小到1024x1024进行分析
            # AI分析不需要原始分辨率，缩放可大幅提升性能
            original_size = (img.width, img.height)
            max_dimension = 1024
            
            if img.width > max_dimension or img.height > max_dimension:
                # 计算缩放比例
                scale = min(max_dimension / img.width, max_dimension / img.height)
                new_width = int(img.width * scale)
                new_height = int(img.height * scale)
                
                log_info('ImageAI', f"Resizing for analysis: {original_size} → ({new_width}x{new_height})")
                
                # 使用高质量重采样
                img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
                
                # 保存原始尺寸信息供后续使用
                img.original_width = original_size[0]
                img.original_height = original_size[1]
            else:
                img.original_width = img.width
                img.original_height = img.height
            
            elapsed = time.time() - start
            return img, elapsed
        
        # 执行带超时的图像加载（10秒超时）
        with ThreadPoolExecutor(max_workers=1) as executor:
            future = executor.submit(load_image_with_timeout, image_path)
            try:
                img, load_time = future.result(timeout=10)
                if load_time > 2:
                    log_warn('ImageAI', f"Slow image loading: {load_time:.2f}s")
            except FutureTimeoutError:
                log_error('ImageAI', f"Image loading timeout (10s): {image_path}")
                # 返回默认参数而不是抛出异常
                return {
                    'quality': 85,
                    'effort': 7,
                    'speed': 7,
                    'recommended_format': tool,
                    'confidence': 0.5,
                    'error': 'Image loading timeout'
                }
            except Exception as e:
                log_error('ImageAI', f"Image loading failed: {e}")
                raise
        
        # ✅ Phase 46.14+: 无损转码检测 (P-002)
        lossless_info = self._detect_lossless_transcode(image_path, tool)
        
        # 提取基础特征（使用原始尺寸）
        actual_width = getattr(img, 'original_width', img.width)
        actual_height = getattr(img, 'original_height', img.height)
        
        basic_features = {
            'width': actual_width,
            'height': actual_height,
            'pixels': actual_width * actual_height,
            'aspect_ratio': actual_width / actual_height,
            'has_alpha': 1 if img.mode in ('RGBA', 'LA') else 0,
        }
        
        # 🆕 智能格式推荐（总是执行，用于对比）
        recommended_format = None
        format_reason = None
        if enable_format_recommendation:
            recommended_format, format_reason = self._recommend_format(
                img, basic_features, optimize_mode, recommend_video_for_animation, image_path
            )
            print(f"🎨 AI推荐格式: {recommended_format} (原因: {format_reason})", file=sys.stderr)
        
        # 决定最终使用的格式
        if expected_format and expected_format != 'auto':
            # 用户明确指定格式，优先使用
            print(f"🎯 用户指定格式: {expected_format}", file=sys.stderr)
            if recommended_format and expected_format != recommended_format:
                print(f"⚠️  用户选择与AI推荐不同 (推荐: {recommended_format})", file=sys.stderr)
            tool = expected_format
        elif tool in ['auto', 'smart', None] and recommended_format:
            # 自动模式，使用AI推荐
            tool = recommended_format
            print(f"📌 自动采用AI推荐: {tool}", file=sys.stderr)
        
        # 提取SWT特征
        img_gray = img.convert('L')
        img_array = np.array(img_gray, dtype=np.float32)
        swt_features = self.swt.extract(img_array)
        
        # 构建特征向量
        feature_vec = [
            basic_features['width'],
            basic_features['height'],
            basic_features['pixels'],
            basic_features['aspect_ratio'],
            basic_features['has_alpha'],
            swt_features['edge_strength'],
            swt_features['texture_complexity'],
            swt_features['noise_level'],
            swt_features['detail_level'],
            swt_features['compression_score'],
            swt_features['high_freq_energy'],
            swt_features['low_freq_energy'],
        ]
        
        X = np.array([feature_vec])
        
        # 标准化
        if tool in self.scalers:
            X = self.scalers[tool].transform(X)
        
        # 预测
        if tool in self.models:
            predicted_quality = self.models[tool].predict(X)[0]
        else:
            # 回退到启发式
            predicted_quality = target_quality
        
        # 确保predicted_quality是标量值
        if isinstance(predicted_quality, np.ndarray):
            predicted_quality = float(predicted_quality.item())
        else:
            predicted_quality = float(predicted_quality)
        
        # 构建参数响应
        raw_quality = int(np.clip(predicted_quality, 1, 100))
        raw_distance = self._quality_to_distance(predicted_quality)
        
        # 计算置信度（基于模型是否存在）
        confidence_pct = 85 if tool in self.models else 75
        
        # 应用质量范围约束（如果启用）
        if options.get('enable_quality_constraint', True):
            constrained_quality = constrain_quality(raw_quality, optimize_mode)
            constrained_distance = constrain_distance(raw_distance, optimize_mode)
        else:
            constrained_quality = raw_quality
            constrained_distance = raw_distance
        
        # 🎯 JPEG→JXL无损转码特殊处理
        is_jpeg_source = hasattr(img, 'format') and img.format in ['JPEG', 'JPG']
        # 只有标准JPEG（RGB/YCbCr）才使用lossless_jpeg
        is_standard_jpeg = (
            is_jpeg_source and 
            img.mode in ['RGB', 'YCbCr']
        )
        is_jxl_lossless_jpeg = (tool in ['jxl', 'cjxl']) and is_standard_jpeg
        
        base_params = {
            'quality': constrained_quality,  # 使用约束后的质量
            'confidence': float(confidence_pct) / 100.0
        }
        
        # 🎯 应用无损检测的质量调整（针对小PNG文件）
        if lossless_info.get('quality_adjustment'):
            base_params['quality'] = max(50, constrained_quality + lossless_info['quality_adjustment'])
            log_info('AIPredictor', f'质量参数调整: {constrained_quality} → {base_params["quality"]} (小PNG优化)')
        
        # JPEG→JXL无损转码时，不设置distance参数（会自动使用--lossless_jpeg=1）
        if not is_jxl_lossless_jpeg:
            base_params['distance'] = constrained_distance
        
        # 工具特定参数
        if tool == 'jxl' or tool == 'cjxl':
            base_params['effort'] = 7 if basic_features['pixels'] < 2000000 else 5
            if is_standard_jpeg:
                # 标准JPEG源文件使用无损JPEG模式
                base_params['lossless_jpeg'] = True
            elif is_jpeg_source:
                # 特殊JPEG（灰度、CMYK等）使用常规有损模式
                base_params['lossless_jpeg'] = False
        elif tool == 'avif' or tool == 'avifenc':
            base_params['quantizer'] = int(100 - predicted_quality)
            base_params['speed'] = 6
        elif tool == 'webp' or tool == 'cwebp':
            base_params['method'] = 6
        
        # 🤖 PPO强化学习优化（实验性）
        if enable_ppo:
            if not self.ppo_available:
                # 🎯 响亮报错 > 静默降级
                log_error('AIPredictor', '❌ PPO强化学习请求但不可用！')
                log_error('AIPredictor', '   原因: 缺少PyTorch依赖')
                log_error('AIPredictor', '   安装: pip install torch')
                log_error('AIPredictor', '   影响: 将使用LightGBM基础预测（精度约降低5-10%）')
                base_params['ppo_optimized'] = False
                base_params['ppo_unavailable'] = True
            else:
                ppo_params = self._apply_ppo_optimization(
                    base_params, basic_features, tool, optimize_mode, image_path
                )
                if ppo_params:
                    log_info('AIPredictor', f'✅ PPO优化完成')
                    base_params.update(ppo_params)
                    base_params['ppo_optimized'] = True
                else:
                    log_warn('AIPredictor', '⚠️  PPO优化失败，使用LightGBM基础预测')
                    base_params['ppo_optimized'] = False
        else:
            base_params['ppo_optimized'] = False
        
        # 构建基础结果
        result = {
            'params': base_params,
            'confidence': base_params['confidence'],
            # 添加顶层字段方便访问
            'quality': base_params.get('quality'),
            'effort': base_params.get('effort'),
            'distance': base_params.get('distance'),
            'lossless_jpeg': base_params.get('lossless_jpeg', False),
        }
        
        # ✅ Phase 46.14+: 添加无损转码信息 (P-002)
        if lossless_info['lossless'] or lossless_info['lossless_jpeg']:
            result['lossless'] = lossless_info['lossless']
            result['lossless_jpeg'] = lossless_info['lossless_jpeg']
            result['transcode_reason'] = lossless_info['reason']
            result['source_format'] = lossless_info['source_format']
            log_info("predictor", 
                    f"无损转码可用: {lossless_info['reason']}",
                    lossless=lossless_info['lossless'],
                    lossless_jpeg=lossless_info['lossless_jpeg'])
        
        # 🆕 添加格式推荐信息
        if recommended_format:
            result['recommended_format'] = recommended_format
            result['format_reason'] = format_reason
        
        # 🆕 如果使用了自定义格式
        if expected_format:
            result['used_format'] = tool
            result['is_custom_format'] = (tool != recommended_format)
        
        # 🆕 可选: 返回高级参数
        if options.get('return_advanced_params', False):
            advanced_params = self._generate_advanced_params(
                tool, optimize_mode, basic_features, swt_features
            )
            result['advanced'] = advanced_params
            result['merged'] = {**base_params, **advanced_params}
        
        # 🆕 可选: 返回特征分析
        if options.get('enable_feature_analysis', False):
            result['features'] = {
                'basic': basic_features,
                'swt': swt_features,
            }
        
        # 🆕 Phase 46.14: 预处理建议 (参考Rimage)
        preprocessing_steps = self._recommend_preprocessing(
            basic_features, swt_features, target_quality, optimize_mode
        )
        if preprocessing_steps:
            result['preprocessing_steps'] = preprocessing_steps
            result['optimization_path'] = self._explain_optimization_path(preprocessing_steps)
        
        # 🔥 Phase 47.17: 将结果存入缓存（限制缓存大小）
        if use_cache and cache_key:
            # 限制缓存大小为100个条目（简单的LRU策略）
            if len(self._prediction_cache) >= 100:
                # 删除最早的缓存项
                oldest_key = next(iter(self._prediction_cache))
                del self._prediction_cache[oldest_key]
                log_info('ImageAI', f"Cache evicted oldest entry (cache full)")
            
            # 存储结果（不包含cached标记）
            cache_value = result.copy()
            if 'cached' in cache_value:
                del cache_value['cached']
            self._prediction_cache[cache_key] = cache_value
            log_info('ImageAI', f"Result cached (total: {len(self._prediction_cache)} entries)")
        
        return result
    
    def _generate_advanced_params(self, tool, optimize_mode, basic_features, swt_features):
        """生成高级参数"""
        advanced = {}
        
        if tool == 'jxl' or tool == 'cjxl':
            # JXL高级选项
            advanced['modular'] = swt_features['texture_complexity'] > 50.0
            advanced['progressive'] = basic_features['pixels'] > 2000000
            advanced['responsive'] = True
            advanced['gaborish'] = swt_features['edge_strength'] > 60.0
        
        elif tool == 'avif' or tool == 'avifenc':
            # AVIF高级选项
            advanced['tiles'] = '4x4' if basic_features['pixels'] > 4000000 else '2x2'
            if optimize_mode == 'quality':
                advanced['chroma'] = '444'
            else:  # balanced
                advanced['chroma'] = '422'
            advanced['auto_filter'] = True
        
        elif tool == 'webp' or tool == 'cwebp':
            # WebP高级选项
            if optimize_mode == 'quality':
                advanced['preprocessing'] = 4
                advanced['partitions'] = 4
            else:  # balanced
                advanced['preprocessing'] = 3
                advanced['partitions'] = 3
            advanced['segments'] = 4
        
        return advanced
    
    def _quality_to_distance(self, quality):
        """将质量参数转换为distance参数"""
        if quality >= 95:
            return 0.0
        elif quality >= 85:
            return 0.5
        else:
            return 1.0
    
    def _recommend_format(self, img, basic_features, optimize_mode, recommend_video=False, image_path=None):
        """
        智能格式推荐（v2.2 - 质量优先，移除size模式）
        
        优化模式说明：
        - quality: 最佳质量，无损优先
        - balanced: 质量与体积平衡（默认）
        
        🎯 核心原则：维持质量前提下，必然减小空间占用
        
        Args:
            img: PIL Image对象
            basic_features: 图像基础特征
            optimize_mode: 优化模式 (quality/balanced)
            recommend_video: 是否推荐动图转视频（用户可选）
        
        Returns:
            (推荐格式, 推荐原因)
        """
        width = basic_features['width']
        height = basic_features['height']
        pixels = basic_features['pixels']
        has_alpha = basic_features['has_alpha']
        
        # 检测图像类型
        is_animated = hasattr(img, 'n_frames') and img.n_frames > 1
        is_photo = self._is_photo(img)
        is_illustration = self._is_illustration(img)
        is_screenshot = self._is_screenshot(img)
        
        # 🎯 优先级规则（v2.2 优化版 - 移除size模式，质量优先）
        
        # -1. 🆕 极小文件特殊处理（< 2KB）
        # 极小文件转换overhead可能导致大小反增，建议保持原格式
        file_size = os.path.getsize(image_path) if image_path and os.path.exists(image_path) else 0
        if file_size > 0 and file_size < 2048:  # < 2KB
            source_format = img.format.lower() if hasattr(img, 'format') else 'png'
            # 几乎所有格式的极小文件都应该保持原格式
            if source_format in ['png', 'webp', 'avif', 'jxl', 'jpeg', 'jpg']:
                return source_format, f'极小文件({file_size}字节)，转换overhead可能增大，保持原格式最优'
        
        # 0. 🆕 JPEG源文件 → 推荐策略
        if hasattr(img, 'format') and img.format in ['JPEG', 'JPG']:
            # 🎯 检查JPEG是否适合转换
            is_grayscale = img.mode == 'L'
            is_cmyk = img.mode == 'CMYK'
            is_small_file = file_size > 0 and file_size < 100 * 1024  # < 100KB
            
            # 🎯 灰度小JPEG：转换收益不明显，保持原格式最优
            if is_grayscale and is_small_file:
                return 'jpeg', f'灰度JPEG小文件({file_size//1024}KB)，保持原格式最优，转换可能增大'
            
            # 🎯 CMYK JPEG：特殊色彩空间，保持原格式
            if is_cmyk:
                return 'jpeg', 'CMYK色彩空间JPEG，保持原格式确保色彩准确'
            
            # 🎯 标准RGB/YCbCr JPEG：JXL无损转码
            if img.mode in ['RGB', 'YCbCr']:
                return 'jxl', 'JPEG源文件，JXL无损转码保持完美质量且体积更小（lossless_jpeg模式）'
            
            # 🎯 灰度大文件：可以尝试JXL有损
            if is_grayscale:
                return 'jxl', f'灰度JPEG大文件，JXL有损编码可能减小体积'
            
            # 其他特殊JPEG：保持原格式
            return 'jpeg', f'特殊JPEG格式({img.mode})，保持原格式最安全'
        
        # 1. 🆕 WebP源文件特殊处理（优先检查，避免不必要的转换）
        if hasattr(img, 'format') and img.format == 'WEBP':
            # 动态WebP：保持WebP（AVIF编码器不支持WebP动画输入）
            if is_animated:
                return 'webp', '动态WebP源文件，保持原格式（转换可能失败或降质）'
            # 透明WebP：保持WebP
            if has_alpha:
                return 'webp', '透明WebP源文件，已是最优格式，保持原格式'
            # 大静态WebP且追求质量：可尝试JXL
            if pixels > 2000000 and optimize_mode == 'quality':
                return 'jxl', 'WebP大图可尝试JXL获得更好压缩'
            # 其他静态WebP：保持原格式
            return 'webp', 'WebP源文件，已是良好格式，保持原格式'
        
        # 2. 巨大动图 → 可选推荐转视频（用户控制）
        if is_animated:
            frame_count = getattr(img, 'n_frames', 1)
            
            # 仅当用户启用视频推荐时才建议转视频
            if recommend_video:
                # 超大动图：分辨率高且帧数多
                if pixels > 2000000 and frame_count > 30:  # 2MP以上 且 30帧以上
                    return 'video', '大型动图建议转换为视频格式（MP4/WebM），体积更小且兼容性更好'
                elif pixels > 5000000:  # 5MP以上的动图
                    return 'video', '超高分辨率动图建议转视频，播放性能更好'
                elif frame_count > 100:  # 帧数超多
                    return 'video', '长动图建议转视频格式，文件体积显著减小'
            
            # 普通动图或用户偏好动态图片格式：AVIF优先
            return 'avif', '动图推荐AVIF，压缩效率和质量平衡最佳'
        
        # 3. 透明度图像 → 优先WebP（通用性好）
        if has_alpha:
            if optimize_mode == 'quality' and pixels > 4000000:
                # 仅超大图且质量优先才用JXL
                return 'jxl', '超大透明图，JXL质量最优'
            else:
                return 'webp', '透明图推荐WebP，兼容性和压缩率俱佳'
        
        # 3. 照片类型 → AVIF为主
        if is_photo:
            if optimize_mode == 'quality' and pixels > 6000000:
                # 仅超高分辨率且质量优先才用JXL
                return 'jxl', '超高分辨率照片，JXL保真度最高'
            else:
                # 其他情况统一用AVIF
                return 'avif', '照片推荐AVIF，压缩率和质量最优'
        
        # 4. 插图/矢量图 → WebP为主
        if is_illustration:
            if optimize_mode == 'quality' and pixels > 3000000:
                # 仅大尺寸且质量优先才用JXL
                return 'jxl', '大型插图，JXL保留细节最佳'
            else:
                return 'webp', '插图推荐WebP，色块压缩效率高'
        
        # 5. 屏幕截图 → WebP优先
        if is_screenshot:
            if optimize_mode == 'quality' and self._has_text(img):
                # 仅质量模式且含文字才用JXL
                return 'jxl', '含文字截图，JXL文字清晰度最优'
            else:
                return 'webp', '截图推荐WebP，UI元素压缩效率高'
        
        # 6. 超大图像 → AVIF（渐进式同样支持）
        if pixels > 10000000:
            # 超超大图才考虑JXL
            if optimize_mode == 'quality':
                return 'jxl', '超大图像质量优先，JXL渐进式加载最佳'
            else:
                return 'avif', '超大图像推荐AVIF，压缩率优异'
        
        # 7. 默认策略（质量优先）
        if optimize_mode == 'quality':
            # 质量模式：无损优先
            if pixels > 4000000:
                return 'jxl', '高分辨率质量优先，JXL无损表现最佳'
            else:
                return 'avif', '质量优先，AVIF近无损效果'
        else:  # balanced（默认）
            # 平衡模式：保守策略，确保质量
            return 'avif', '平衡模式，AVIF质量与体积兼顾'
    
    def _is_photo(self, img):
        """判断是否为照片"""
        # 转换为numpy数组
        img_array = np.array(img.convert('RGB'))
        
        # 计算颜色复杂度
        unique_colors = len(np.unique(img_array.reshape(-1, img_array.shape[2]), axis=0))
        total_pixels = img_array.shape[0] * img_array.shape[1]
        color_ratio = unique_colors / total_pixels
        
        # 照片通常有更高的颜色复杂度
        return color_ratio > 0.1
    
    def _is_illustration(self, img):
        """判断是否为插图/矢量图"""
        img_array = np.array(img.convert('RGB'))
        
        # 计算颜色数量
        unique_colors = len(np.unique(img_array.reshape(-1, img_array.shape[2]), axis=0))
        
        # 插图通常颜色较少
        return unique_colors < 5000
    
    def _is_screenshot(self, img):
        """判断是否为屏幕截图"""
        # 截图通常具有标准分辨率比例
        width, height = img.size
        aspect_ratio = width / height
        
        # 常见屏幕比例: 16:9, 16:10, 4:3, 21:9等
        common_ratios = [16/9, 16/10, 4/3, 21/9, 3/2]
        
        for ratio in common_ratios:
            if abs(aspect_ratio - ratio) < 0.05:
                # 进一步检查是否有UI元素（边缘锐利）
                img_gray = np.array(img.convert('L'))
                edges = np.abs(np.diff(img_gray, axis=0)).mean()
                return edges > 20  # 截图通常有较多边缘
        
        return False
    
    def _has_text(self, img):
        """检测图像是否包含文字"""
        img_gray = np.array(img.convert('L'))
        
        # 计算边缘强度（文字区域边缘明显）
        edges_h = np.abs(np.diff(img_gray, axis=0))
        edges_v = np.abs(np.diff(img_gray, axis=1))
        
        edge_strength = (edges_h.mean() + edges_v.mean()) / 2
        
        # 文字区域通常有较强的边缘
        return edge_strength > 25
    
    def _apply_ppo_optimization(self, base_params, basic_features, tool, optimize_mode, image_path):
        """
        使用PPO强化学习优化参数
        
        Args:
            base_params: LightGBM预测的基础参数
            basic_features: 图像基础特征
            tool: 转换工具
            optimize_mode: 优化模式
            image_path: 图像路径（用于获取文件大小）
        
        Returns:
            PPO优化后的参数，或None（如果失败）
        """
        try:
            import subprocess
            import os
            
            # 获取文件大小
            input_size = os.path.getsize(image_path)
            
            # 构建PPO推理的输入状态
            ppo_state = {
                'image_width': basic_features['width'],
                'image_height': basic_features['height'],
                'has_alpha': bool(basic_features['has_alpha']),
                'is_animation': False,  # 暂不支持动图
                'input_size': input_size,
                'optimize_mode': optimize_mode,
                'tool': tool,
                # 当前参数作为初始状态
                'quality': base_params.get('quality', 85),
                'effort': base_params.get('effort', 7),
                'speed': base_params.get('speed', 6),
                'distance': base_params.get('distance', 1.0),
            }
            
            # 调用PPO推理脚本
            script_dir = os.path.dirname(os.path.abspath(__file__))
            ppo_script = os.path.join(script_dir, 'ppo_inference.py')
            
            # 使用当前Python解释器（确保在虚拟环境中）
            result = subprocess.run(
                [sys.executable, ppo_script, json.dumps(ppo_state)],
                capture_output=True,
                text=True,
                timeout=5
            )
            
            if result.returncode == 0:
                ppo_result = json.loads(result.stdout)
                if ppo_result.get('success'):
                    return ppo_result['params']
                else:
                    print(f"⚠️  PPO推理返回错误: {ppo_result.get('error')}", file=sys.stderr)
                    return None
            else:
                print(f"⚠️  PPO脚本执行失败: {result.stderr}", file=sys.stderr)
                return None
                
        except Exception as e:
            print(f"⚠️  PPO优化异常: {str(e)}", file=sys.stderr)
            return None
    
    def _recommend_preprocessing(self, basic_features, swt_features, target_quality, optimize_mode):
        """推荐预处理步骤
        
        Phase 46.14: 根据图像特征智能推荐预处理步骤
        参考Rimage的预处理管道设计
        
        Args:
            basic_features: 基础特征字典
            swt_features: SWT特征字典
            target_quality: 目标质量
            optimize_mode: 优化模式
            
        Returns:
            预处理步骤列表
        """
        recommendations = []
        
        # 规则1: 图像过大 → 建议缩小
        width = basic_features['width']
        height = basic_features['height']
        pixels = basic_features['pixels']
        
        if pixels > 4_000_000:  # 4MP以上
            # 计算合适的目标宽度
            target_width = 1920 if width > 1920 else width
            if width > target_width:
                size_reduction = ((width * height) - (target_width * height * target_width / width)) / (width * height) * 100
                recommendations.append({
                    "step": "resize",
                    "params": {
                        "size": f"{target_width}x",
                        "filter": "lanczos3"
                    },
                    "reason": f"Image resolution {width}x{height} ({pixels/1_000_000:.1f}MP) is very high. Resizing to {target_width}px width will reduce file size by ~{size_reduction:.0f}% while maintaining visual quality."
                })
        # 移除size模式的强制缩放，保持质量优先
        
        # 移除颜色量化建议，保持质量优先
        estimated_colors = int(swt_features.get('detail_level', 50) * 5000)
        
        if estimated_colors > 100_000 and target_quality < 90:
            # 根据质量目标选择颜色数
            if target_quality < 80:
                colors = 128
            elif target_quality < 85:
                colors = 192
            else:
                colors = 256
            
            recommendations.append({
                "step": "quantize",
                "params": {
                    "colors": colors
                },
                "reason": f"Image has high color count (~{estimated_colors/1000:.0f}K colors). Quantization to {colors} colors will reduce file size with minimal visual impact at Q{target_quality}."
            })
        
        # 规则3: 图像模糊且质量要求高 → 建议锐化
        edge_strength = swt_features.get('edge_strength', 50.0)
        
        if edge_strength < 40.0 and target_quality >= 85:
            # 根据模糊程度选择锐化强度
            if edge_strength < 20.0:
                amount = 0.8
                intensity = "strong"
            elif edge_strength < 30.0:
                amount = 0.5
                intensity = "moderate"
            else:
                amount = 0.3
                intensity = "mild"
            
            recommendations.append({
                "step": "sharpen",
                "params": {
                    "amount": amount
                },
                "reason": f"Image has low sharpness (edge strength: {edge_strength:.1f}). {intensity.capitalize()} sharpening (amount={amount}) will improve perceived quality."
            })
        
        # 规则4: 高质量模式且图像不大 → 不建议预处理
        if not recommendations and optimize_mode == 'quality' and pixels < 2_000_000:
            print("ℹ️  No preprocessing needed for quality mode with reasonable resolution", file=sys.stderr)
        
        return recommendations
    
    def _explain_optimization_path(self, preprocessing_steps):
        """生成优化路径说明
        
        Args:
            preprocessing_steps: 预处理步骤列表
            
        Returns:
            优化路径的文字说明
        """
        if not preprocessing_steps:
            return "Direct encoding without preprocessing"
        
        step_names = [step['step'] for step in preprocessing_steps]
        path = " → ".join(step_names).capitalize()
        return f"Recommended path: {path} → Encode"
    
    def predict_video(self, video_path, optimize_mode='balanced', options=None):
        """
        Video parameter AI prediction (unified with image strategy)
        
        Args:
            video_path: Video file path
            optimize_mode: Optimization mode (size/balanced/quality)
            options: Optional parameters {
                - expected_codec: Expected output codec (h264, h265, h266, av1, prores)
                  Same as image's expected_format for consistency
                - target_codec: Alias for expected_codec (backward compatibility)
                - enable_format_recommendation: Enable AI codec recommendation (default True)
                  If False, use source codec re-encoding (same as image)
                - enable_preprocessing: Enable preprocessing suggestions (default True)
                - aggressive_mode: Enable cutting-edge codec recommendation (H.266, AV1) (default False)
                - use_advanced_ai: Use advanced AI (default True)
                - enable_transformer: Enable Transformer (default False)
                - enable_vmaf: Enable VMAF quality assessment (default False)
            }
        
        Returns:
            {
                'encoder': str,           # Recommended encoder
                'crf': int,               # CRF value
                'preset': str,            # Encoding preset
                'two_pass': bool,         # Use two-pass encoding
                'reasoning': str,         # AI reasoning
                'confidence': float,      # Confidence score
                'source_codec': str,      # Source codec (for re-encoding)
                'recommended_codec': str, # AI recommended codec
                'preprocessing_steps': [], # Preprocessing recommendations
                'metadata': {             # Video metadata
                    'width': int,
                    'height': int,
                    'fps': float,
                    'duration': float,
                    'bitrate': int,
                    'codec': str
                }
            }
        """
        import subprocess
        
        options = options or {}
        # Unified parameter naming: expected_codec (like image's expected_format)
        expected_codec = options.get('expected_codec', None) or options.get('target_codec', None)
        enable_format_recommendation = options.get('enable_format_recommendation', True)
        enable_preprocessing = options.get('enable_preprocessing', True)
        aggressive_mode = options.get('aggressive_mode', False)  # 🆕 Cutting-edge codec mode
        
        log_info('VideoAI', f"Analyzing video: {video_path}")
        
        source_codec = None
        try:
            # 使用ffprobe提取视频元数据
            cmd = [
                'ffprobe', '-v', 'error',
                '-select_streams', 'v:0',
                '-show_entries', 'stream=width,height,r_frame_rate,bit_rate,codec_name,duration',
                '-of', 'json',
                video_path
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            if result.returncode != 0:
                raise Exception(f"ffprobe failed: {result.stderr}")
            
            probe_data = json.loads(result.stdout)
            stream = probe_data['streams'][0]
            
            width = int(stream.get('width', 1920))
            height = int(stream.get('height', 1080))
            pixels = width * height
            
            # 解析帧率
            fps_str = stream.get('r_frame_rate', '30/1')
            fps_parts = fps_str.split('/')
            fps = float(fps_parts[0]) / float(fps_parts[1]) if len(fps_parts) == 2 else 30.0
            
            duration = float(stream.get('duration', 0))
            bitrate = int(stream.get('bit_rate', 0)) // 1000  # kbps
            source_codec = stream.get('codec_name', None)  # 获取原始编码器
            
            log_info('VideoAI', f"Video metadata: {width}x{height} @{fps:.1f}fps, {duration:.1f}s, {bitrate}kbps, codec={source_codec}")
            
        except Exception as e:
            log_warn('VideoAI', f"Failed to analyze video metadata: {e}")
            # 使用默认值
            width, height, fps, duration, bitrate = 1920, 1080, 30.0, 0, 0
            pixels = width * height
        
        # ========== AI策略推荐 ==========
        
        # Encoder selection (unified strategy with image)
        # ⚠️ Strict quality manifesto: no fallback, smart codec upgrade
        recommended_codec = None
        codec_reason = None
        
        if expected_codec:
            # User explicitly specified encoder (like image's expected_format)
            encoder = expected_codec
            reasoning = f"User specified: {expected_codec}"
            log_info("VideoAI", f"User specified codec: {expected_codec}")
        elif not enable_format_recommendation:
            # Disabled format recommendation → source codec re-encoding (same as image)
            if source_codec:
                encoder = source_codec
                reasoning = f"Re-encode with source codec: {source_codec} (format recommendation disabled)"
                log_info("VideoAI", f"Source format re-encoding: {source_codec}")
            else:
                # No source codec detected, use default
                encoder = 'h265'
                reasoning = "No source codec detected, default to H.265"
                log_warn("VideoAI", "Source codec not detected, using H.265 as default")
        else:
            # AI recommendation: smart codec upgrade based on source (same strategy as image)
            # Don't blindly use modern format, upgrade based on analysis
            
            # AI recommendation with codec upgrade tracking
            # 🆕 Aggressive mode: prioritize cutting-edge codecs (H.266, AV1)
            if aggressive_mode:
                log_info("VideoAI", "Aggressive mode enabled: prioritizing H.266/AV1")
            
            # 检测原编码器并智能升级
            if source_codec in ['h264', 'avc', 'avc1']:
                # H.264 → 智能升级（根据模式和aggressive_mode）
                if aggressive_mode:
                    # 激进模式：直接升级到H.266或AV1
                    if pixels > 1920 * 1080:  # 2K+
                        encoder = 'h266'
                        recommended_codec = 'h266'
                        codec_reason = "H.264→H.266 aggressive upgrade"
                        reasoning = f"Source: H.264 → Upgrade to H.266/VVC (cutting-edge, 50% better than HEVC)"
                        log_info("VideoAI", f"Aggressive upgrade: H.264 → H.266 (VVC)")
                    else:
                        encoder = 'av1'
                        recommended_codec = 'av1'
                        codec_reason = "H.264→AV1 aggressive upgrade"
                        reasoning = f"Source: H.264 → Upgrade to AV1 (cutting-edge)"
                        log_info("VideoAI", f"Aggressive upgrade: H.264 → AV1")
                elif optimize_mode == 'size':
                    # size模式更激进：H.264 → AV1
                    encoder = 'av1'
                    recommended_codec = 'av1'
                    codec_reason = "H.264→AV1 upgrade for size"
                    reasoning = f"Source: H.264 → Upgrade to AV1 for maximum compression (size mode)"
                    log_info("VideoAI", f"Smart upgrade: H.264 → AV1 (size mode)")
                else:
                    # balanced/quality: H.264 → H.265
                    encoder = 'h265'
                    recommended_codec = 'h265'
                    codec_reason = "H.264→H.265 safe upgrade"
                    reasoning = f"Source: H.264 → Upgrade to HEVC (50% better compression, maintains compatibility)"
                    log_info("VideoAI", f"Smart upgrade: H.264 → H.265")
            elif source_codec in ['h265', 'hevc', 'hvc1']:
                # H.265 → 保持或升级到 H.266/AV1（根据模式和aggressive_mode）
                if aggressive_mode:
                    # 激进模式：升级到H.266
                    encoder = 'h266'
                    recommended_codec = 'h266'
                    codec_reason = "H.265→H.266 aggressive upgrade"
                    reasoning = f"Source: H.265 → Upgrade to H.266/VVC (cutting-edge, 50% better than HEVC)"
                    log_info("VideoAI", f"Aggressive upgrade: H.265 → H.266 (VVC)")
                elif optimize_mode == 'size':
                    encoder = 'av1'
                    recommended_codec = 'av1'
                    codec_reason = "H.265→AV1 for size"
                    reasoning = f"Source: H.265 → Upgrade to AV1 for maximum compression"
                else:
                    encoder = 'h265'
                    recommended_codec = 'h265'
                    codec_reason = "H.265 already modern"
                    reasoning = f"Source: H.265 → Keep HEVC (already modern)"
            elif source_codec in ['vp8', 'vp9']:
                # VP8/VP9 → AV1（Google系列升级）
                encoder = 'av1'
                recommended_codec = 'av1'
                codec_reason = f"{source_codec.upper()}→AV1 upgrade"
                reasoning = f"Source: {source_codec.upper()} → Upgrade to AV1 (next-gen from Google)"
            elif source_codec in ['h266', 'vvc']:
                # H.266已是最新，保持
                encoder = 'h266'
                recommended_codec = 'h266'
                codec_reason = "H.266 already cutting-edge"
                reasoning = f"Source: H.266 → Keep H.266/VVC (already cutting-edge)"
            elif source_codec in ['av1', 'av01']:
                # AV1已是最新，保持或升级到H.266
                if aggressive_mode and pixels > 1920 * 1080:
                    encoder = 'h266'
                    recommended_codec = 'h266'
                    codec_reason = "AV1→H.266 aggressive upgrade"
                    reasoning = f"Source: AV1 → Upgrade to H.266/VVC (cutting-edge, better for high-res)"
                    log_info("VideoAI", f"Aggressive upgrade: AV1 → H.266 (VVC)")
                else:
                    encoder = 'av1'
                    recommended_codec = 'av1'
                    codec_reason = "AV1 already cutting-edge"
                    reasoning = f"Source: AV1 → Keep AV1 (already cutting-edge)"
            elif source_codec in ['mpeg4', 'xvid', 'divx']:
                # 旧编码器 → H.265
                encoder = 'h265'
                recommended_codec = 'h265'
                codec_reason = f"{source_codec.upper()}→H.265 legacy upgrade"
                reasoning = f"Source: {source_codec.upper()} (legacy) → Upgrade to HEVC"
            else:
                # 未知或其他编码器 → 按分辨率和模式推荐
                if aggressive_mode and pixels > 1920 * 1080:
                    # 激进模式：2K+使用H.266
                    encoder = 'h266'
                    reasoning = "Aggressive mode: H.266/VVC for cutting-edge compression"
                elif pixels > 3840 * 2160:  # 4K+
                    encoder = 'av1'
                    reasoning = "4K+ video: AV1 for best compression"
                elif optimize_mode == 'quality':
                    encoder = 'prores'
                    reasoning = "Quality mode: ProRes for professional quality"
                elif optimize_mode == 'size':
                    encoder = 'av1'
                    reasoning = "Size mode: AV1 for maximum compression"
                else:  # balanced
                    encoder = 'h265'
                    reasoning = "Balanced mode: HEVC for modern standard"
        
        # CRF选择（基于编码器和优化模式）
        if optimize_mode == 'quality':
            if encoder == 'h264':
                crf = 18
            elif encoder == 'h265':
                crf = 20  # H.265质量更好，CRF可以稍高
            elif encoder == 'h266':
                crf = 18  # H.266/VVC质量优先
            elif encoder == 'av1':
                crf = 18  # AV1质量优先
            elif encoder == 'prores':
                crf = 0  # ProRes无损或最高质量
            else:  # vp9
                crf = 15
            crf_reason = "Highest quality"
        elif optimize_mode == 'size':
            if encoder == 'h264':
                crf = 28
            elif encoder == 'h265':
                crf = 30  # H.265压缩更好
            elif encoder == 'h266':
                crf = 35  # H.266/VVC体积优先
            elif encoder == 'av1':
                crf = 40  # AV1体积优先
            elif encoder == 'prores':
                crf = 10  # ProRes压缩模式
            else:  # vp9
                crf = 38
            crf_reason = "Optimized for size"
        else:  # balanced
            if encoder == 'h264':
                crf = 23
            elif encoder == 'h265':
                crf = 25  # H.265平衡
            elif encoder == 'h266':
                crf = 23  # H.266/VVC平衡
            elif encoder == 'av1':
                crf = 30  # AV1平衡
            elif encoder == 'prores':
                crf = 5  # ProRes平衡
            else:  # vp9
                crf = 28
            crf_reason = "Balanced quality/size"
        
        # Preset选择
        if duration > 300:  # 5分钟以上
            preset = 'fast'
            preset_reason = "Long video: fast encoding"
        elif optimize_mode == 'quality':
            preset = 'slow'
            preset_reason = "Quality mode: slow for best results"
        else:
            preset = 'medium'
            preset_reason = "Standard preset"
        
        # 两遍编码决策
        two_pass = (optimize_mode == 'quality' or pixels > 1920 * 1080) and duration > 10
        two_pass_reason = "Two-pass for quality/large videos" if two_pass else "Single-pass sufficient"
        
        # 🆕 Video preprocessing suggestions (unified with image)
        preprocessing_steps = []
        if enable_preprocessing:
            # Resolution-based preprocessing
            if pixels > 3840 * 2160 and optimize_mode == 'size':
                preprocessing_steps.append({
                    'step': 'downscale',
                    'params': {'target_resolution': '3840x2160', 'reason': '4K+ size optimization'}
                })
            # Bitrate-based preprocessing
            if bitrate > 50000:  # High bitrate videos
                preprocessing_steps.append({
                    'step': 'bitrate_optimization',
                    'params': {'strategy': 'adaptive', 'reason': 'High bitrate source'}
                })
            # FPS optimization for size mode
            if fps > 60 and optimize_mode == 'size':
                preprocessing_steps.append({
                    'step': 'fps_reduction',
                    'params': {'target_fps': 60, 'reason': 'Size mode FPS optimization'}
                })
        
        # 置信度估算
        confidence = 0.85 if expected_codec else 0.75
        
        full_reasoning = f"{reasoning} | CRF={crf} ({crf_reason}) | Preset={preset} ({preset_reason}) | {two_pass_reason}"
        
        log_info('VideoAI', f"Recommendation: {encoder} CRF{crf} {preset}, confidence={confidence:.1%}")
        if recommended_codec:
            log_info('VideoAI', f"AI recommended codec: {recommended_codec} ({codec_reason})")
        if preprocessing_steps:
            log_info('VideoAI', f"Preprocessing: {len(preprocessing_steps)} step(s) recommended")
        
        # Unified return structure (same as image)
        return {
            'encoder': encoder,
            'crf': crf,
            'preset': preset,
            'two_pass': two_pass,
            'confidence': confidence,
            'reasoning': full_reasoning,
            'source_codec': source_codec,
            'recommended_codec': recommended_codec,
            'codec_reason': codec_reason,
            'preprocessing_steps': preprocessing_steps,
            'metadata': {
                'width': width,
                'height': height,
                'fps': fps,
                'duration': duration,
                'bitrate': bitrate,
                'codec': source_codec
            }
        }

def main():
    """主函数（命令行接口）"""
    if len(sys.argv) < 3:
        print(json.dumps({
            'error': 'Usage: predict_params.py <image_path> <tool> [target_quality] [optimize_mode] [options_json]'
        }))
        sys.exit(1)
    
    image_path = sys.argv[1]
    tool = sys.argv[2]
    target_quality = int(sys.argv[3]) if len(sys.argv) > 3 else 90
    optimize_mode = sys.argv[4] if len(sys.argv) > 4 else 'balanced'
    
    # 🆕 解析options参数（JSON格式）
    options = {}
    if len(sys.argv) > 5:
        try:
            options = json.loads(sys.argv[5])
        except json.JSONDecodeError as e:
            print(json.dumps({
                'error': f'Invalid options JSON: {str(e)}'
            }))
            sys.exit(1)
    
    try:
        predictor = AIPredictor()
        result = predictor.predict(image_path, tool, target_quality, optimize_mode, options)
        print(json.dumps(result, indent=2))
    except Exception as e:
        print(json.dumps({
            'error': str(e)
        }))
        sys.exit(1)

if __name__ == '__main__':
    main()
