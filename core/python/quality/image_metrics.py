"""
质量度量系统 - 图像质量评估

基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/quality/metrics.go 重新实现

核心功能:
- SSIM结构相似性精确计算（8x8窗口滑动）
- PSNR峰值信噪比测量（dB精度）
- MSE均方误差计算（RGB三通道）
- 局部SSIM质量热图分析
- 图像质量对比验证

EX-017实现: 从Go废弃代码价值提取
遵循四高原则：高规范化、高兼容性、高扩展性、高稳定性
"""

import numpy as np
from PIL import Image
import math
from typing import Optional, Tuple, Union
from dataclasses import dataclass
from pathlib import Path
import logging


@dataclass
class QualityMetrics:
    """质量评估指标"""
    ssim: float = 0.0       # 结构相似性指数 (0-1)
    psnr: float = 0.0       # 峰值信噪比 (dB)
    mse: float = 0.0        # 均方误差


class ImageQualityCalculator:
    """
    图像质量评估计算器
    
    高规范化、高兼容性、高扩展性、高稳定性实现
    """
    
    def __init__(self, debug: bool = False):
        """
        初始化质量评估计算器
        
        Args:
            debug: 调试模式
        """
        self.debug = debug
        self.logger = logging.getLogger(__name__)
        
        if debug:
            self.logger.setLevel(logging.DEBUG)
    
    def compare(self, 
                original_path: Union[str, Path], 
                compressed_path: Union[str, Path]) -> Optional[QualityMetrics]:
        """
        比较两张图像的质量
        
        Args:
            original_path: 原始图像路径
            compressed_path: 压缩后图像路径
            
        Returns:
            质量指标，如果失败则返回None
        """
        try:
            # 加载图像
            orig_img = self._load_image(original_path)
            comp_img = self._load_image(compressed_path)
            
            if orig_img is None or comp_img is None:
                return None
            
            # 检查尺寸一致性
            if orig_img.shape != comp_img.shape:
                self.logger.error(f"图像尺寸不一致: {orig_img.shape} vs {comp_img.shape}")
                return None
            
            # 计算各项指标
            mse = self._calculate_mse(orig_img, comp_img)
            psnr = self._calculate_psnr(mse)
            ssim = self._calculate_ssim(orig_img, comp_img)
            
            metrics = QualityMetrics(ssim=ssim, psnr=psnr, mse=mse)
            
            if self.debug:
                self.logger.debug(f"质量指标: SSIM={ssim:.4f}, PSNR={psnr:.2f}dB, MSE={mse:.2f}")
            
            return metrics
            
        except Exception as e:
            self.logger.error(f"质量对比失败: {e}")
            return None
    
    def _load_image(self, image_path: Union[str, Path]) -> Optional[np.ndarray]:
        """
        加载图像文件
        
        Args:
            image_path: 图像路径
            
        Returns:
            图像数组，失败返回None
        """
        try:
            with Image.open(image_path) as img:
                # 转换为RGB（去除Alpha通道的影响）
                if img.mode in ['RGBA', 'LA']:
                    img = img.convert('RGB')
                elif img.mode in ['L']:
                    img = img.convert('RGB')
                elif img.mode in ['P']:
                    img = img.convert('RGB')
                
                # 转换为numpy数组
                img_array = np.array(img, dtype=np.float64)
                
                if self.debug:
                    self.logger.debug(f"图像加载成功: {image_path}, 形状: {img_array.shape}")
                
                return img_array
                
        except Exception as e:
            self.logger.error(f"图像加载失败 {image_path}: {e}")
            return None
    
    def _calculate_mse(self, img1: np.ndarray, img2: np.ndarray) -> float:
        """
        计算均方误差 (Mean Squared Error)
        
        Args:
            img1: 第一张图像
            img2: 第二张图像
            
        Returns:
            MSE值
        """
        # 确保图像在0-255范围内
        if img1.max() <= 1.0:
            img1 = img1 * 255.0
        if img2.max() <= 1.0:
            img2 = img2 * 255.0
        
        # 计算每个像素的差异平方
        diff = img1 - img2
        squared_diff = diff ** 2
        
        # 计算所有像素的平均值
        mse = np.mean(squared_diff)
        
        return float(mse)
    
    def _calculate_psnr(self, mse: float) -> float:
        """
        计算峰值信噪比 (Peak Signal-to-Noise Ratio)
        PSNR = 10 * log10(MAX^2 / MSE)
        MAX = 255 (8-bit图像)
        
        Args:
            mse: 均方误差
            
        Returns:
            PSNR值(dB)
        """
        if mse == 0:
            return 100.0  # 完全相同
        
        max_pixel_value = 255.0
        psnr = 10.0 * math.log10((max_pixel_value * max_pixel_value) / mse)
        
        return float(psnr)
    
    def _calculate_ssim(self, img1: np.ndarray, img2: np.ndarray) -> float:
        """
        计算结构相似性指数 (Structural Similarity Index)
        使用8x8窗口的局部SSIM平均值
        
        Args:
            img1: 第一张图像
            img2: 第二张图像
            
        Returns:
            SSIM值(0-1)
        """
        # 确保图像在0-255范围内
        if img1.max() <= 1.0:
            img1 = img1 * 255.0
        if img2.max() <= 1.0:
            img2 = img2 * 255.0
        
        height, width = img1.shape[:2]
        window_size = 8
        
        ssim_sum = 0.0
        window_count = 0
        
        # 滑动窗口计算局部SSIM
        for y in range(0, height - window_size + 1, window_size):
            for x in range(0, width - window_size + 1, window_size):
                # 提取窗口
                window1 = img1[y:y+window_size, x:x+window_size]
                window2 = img2[y:y+window_size, x:x+window_size]
                
                # 计算局部SSIM
                local_ssim = self._calculate_local_ssim(window1, window2)
                ssim_sum += local_ssim
                window_count += 1
        
        if window_count == 0:
            return 0.0
        
        return float(ssim_sum / window_count)
    
    def _calculate_local_ssim(self, window1: np.ndarray, window2: np.ndarray) -> float:
        """
        计算局部SSIM
        SSIM(x,y) = (2*μx*μy + C1)(2*σxy + C2) / ((μx^2 + μy^2 + C1)(σx^2 + σy^2 + C2))
        
        Args:
            window1: 第一个窗口
            window2: 第二个窗口
            
        Returns:
            局部SSIM值
        """
        # SSIM常数
        C1 = (0.01 * 255) ** 2  # 6.5025
        C2 = (0.03 * 255) ** 2  # 58.5225
        
        # 如果是彩色图像，计算每个通道的平均值
        if len(window1.shape) == 3:
            # 转换为灰度图像
            window1 = np.mean(window1, axis=2)
            window2 = np.mean(window2, axis=2)
        
        # 计算均值
        mu1 = np.mean(window1)
        mu2 = np.mean(window2)
        
        # 计算方差和协方差
        sigma1_sq = np.var(window1)
        sigma2_sq = np.var(window2)
        sigma12 = np.mean((window1 - mu1) * (window2 - mu2))
        
        # 计算SSIM
        numerator = (2 * mu1 * mu2 + C1) * (2 * sigma12 + C2)
        denominator = (mu1 * mu1 + mu2 * mu2 + C1) * (sigma1_sq + sigma2_sq + C2)
        
        if denominator == 0:
            return 1.0
        
        ssim = numerator / denominator
        return float(ssim)
    
    def calculate_quality_map(self, 
                            original_path: Union[str, Path], 
                            compressed_path: Union[str, Path],
                            window_size: int = 8) -> Optional[np.ndarray]:
        """
        计算质量热图
        
        Args:
            original_path: 原始图像路径
            compressed_path: 压缩图像路径
            window_size: 窗口大小
            
        Returns:
            质量热图数组
        """
        try:
            orig_img = self._load_image(original_path)
            comp_img = self._load_image(compressed_path)
            
            if orig_img is None or comp_img is None:
                return None
            
            if orig_img.shape != comp_img.shape:
                return None
            
            height, width = orig_img.shape[:2]
            quality_map = np.zeros((height // window_size, width // window_size))
            
            for i, y in enumerate(range(0, height - window_size + 1, window_size)):
                for j, x in enumerate(range(0, width - window_size + 1, window_size)):
                    window1 = orig_img[y:y+window_size, x:x+window_size]
                    window2 = comp_img[y:y+window_size, x:x+window_size]
                    
                    local_ssim = self._calculate_local_ssim(window1, window2)
                    quality_map[i, j] = local_ssim
            
            return quality_map
            
        except Exception as e:
            self.logger.error(f"质量热图计算失败: {e}")
            return None
    
    def assess_quality_level(self, metrics: QualityMetrics) -> str:
        """
        评估质量等级
        
        Args:
            metrics: 质量指标
            
        Returns:
            质量等级描述
        """
        if metrics.ssim >= 0.98 and metrics.psnr >= 45:
            return "🏆 优秀 (几乎无损)"
        elif metrics.ssim >= 0.95 and metrics.psnr >= 40:
            return "✅ 良好 (高质量)"
        elif metrics.ssim >= 0.90 and metrics.psnr >= 35:
            return "🟡 中等 (可接受)"
        elif metrics.ssim >= 0.80 and metrics.psnr >= 30:
            return "⚠️ 偏低 (明显压缩)"
        else:
            return "❌ 差劲 (严重劣化)"
    
    def compare_with_reference(self, 
                             test_image: Union[str, Path],
                             reference_images: list,
                             threshold_ssim: float = 0.95) -> dict:
        """
        与参考图像对比
        
        Args:
            test_image: 测试图像
            reference_images: 参考图像列表
            threshold_ssim: SSIM阈值
            
        Returns:
            对比结果
        """
        results = {
            'test_image': str(test_image),
            'comparisons': [],
            'best_match': None,
            'passed_threshold': False
        }
        
        best_ssim = 0.0
        best_match_idx = -1
        
        for i, ref_image in enumerate(reference_images):
            metrics = self.compare(test_image, ref_image)
            if metrics:
                comparison = {
                    'reference': str(ref_image),
                    'ssim': metrics.ssim,
                    'psnr': metrics.psnr,
                    'mse': metrics.mse,
                    'quality_level': self.assess_quality_level(metrics)
                }
                results['comparisons'].append(comparison)
                
                if metrics.ssim > best_ssim:
                    best_ssim = metrics.ssim
                    best_match_idx = i
        
        # 设置最佳匹配
        if best_match_idx >= 0:
            results['best_match'] = results['comparisons'][best_match_idx]
            results['passed_threshold'] = best_ssim >= threshold_ssim
        
        return results


# 便捷函数
def calculate_image_quality(original_path: Union[str, Path], 
                          compressed_path: Union[str, Path], 
                          debug: bool = False) -> Optional[QualityMetrics]:
    """
    计算图像质量指标的便捷函数
    
    Args:
        original_path: 原始图像路径
        compressed_path: 压缩图像路径
        debug: 调试模式
        
    Returns:
        质量指标
    """
    calculator = ImageQualityCalculator(debug=debug)
    return calculator.compare(original_path, compressed_path)


def assess_image_quality(metrics: QualityMetrics) -> str:
    """
    评估图像质量等级的便捷函数
    
    Args:
        metrics: 质量指标
        
    Returns:
        质量等级描述
    """
    calculator = ImageQualityCalculator()
    return calculator.assess_quality_level(metrics)


if __name__ == "__main__":
    # 测试代码
    print("=== 质量度量系统测试 ===")
    
    calculator = ImageQualityCalculator(debug=True)
    
    # 模拟测试（需要实际图像文件）
    print("📊 质量度量系统初始化成功")
    print("   功能:")
    print("   ✅ SSIM结构相似性计算")
    print("   ✅ PSNR峰值信噪比测量") 
    print("   ✅ MSE均方误差计算")
    print("   ✅ 局部质量热图分析")
    print("   ✅ 质量等级评估")
    
    # 测试质量等级评估
    test_metrics = QualityMetrics(ssim=0.96, psnr=42.5, mse=3.2)
    quality_level = calculator.assess_quality_level(test_metrics)
    print(f"   📈 测试评估: SSIM={test_metrics.ssim:.3f}, PSNR={test_metrics.psnr:.1f}dB → {quality_level}")
    
    print("🎯 质量度量系统就绪！")
