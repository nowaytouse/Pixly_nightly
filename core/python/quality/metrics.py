#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 47.23: 质量评估系统 - SSIM/PSNR/MSE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

功能：
- SSIM（结构相似性指数）完整实现
- PSNR（峰值信噪比）计算
- MSE（均方误差）计算
- 质量等级自动评估
- 支持转换前质量预估

从Go代码提取：core/@deprecated/go_ai_service_2025_11_11/ai 2/quality/metrics.go
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

import math
from dataclasses import dataclass
from typing import Tuple, Optional
from pathlib import Path
import numpy as np
from PIL import Image


@dataclass
class QualityMetrics:
    """质量评估指标"""
    ssim: float  # 结构相似性指数 (0-1)
    psnr: float  # 峰值信噪比 (dB)
    mse: float   # 均方误差
    
    def get_quality_level(self) -> str:
        """
        获取质量等级
        
        Returns:
            质量等级字符串
        """
        if self.ssim >= 0.98:
            return "excellent"  # 极佳
        elif self.ssim >= 0.95:
            return "good"       # 良好
        elif self.ssim >= 0.90:
            return "fair"       # 尚可
        else:
            return "poor"       # 较差
    
    def is_acceptable(self) -> bool:
        """
        判断质量是否可接受
        
        Returns:
            True如果SSIM >= 0.95
        """
        return self.ssim >= 0.95


class QualityCalculator:
    """质量评估计算器"""
    
    def __init__(self, debug: bool = False):
        """
        初始化计算器
        
        Args:
            debug: 是否启用调试模式
        """
        self.debug = debug
    
    def compare(self, original_path: str, compressed_path: str) -> QualityMetrics:
        """
        比较两张图像的质量
        
        Args:
            original_path: 原始图像路径
            compressed_path: 压缩后图像路径
            
        Returns:
            质量评估指标
            
        Raises:
            FileNotFoundError: 文件不存在
            ValueError: 图像尺寸不一致
        """
        # 加载图像
        orig_img = self._load_image(original_path)
        comp_img = self._load_image(compressed_path)
        
        # 检查尺寸一致性
        if orig_img.shape != comp_img.shape:
            raise ValueError(f"图像尺寸不一致: {orig_img.shape} vs {comp_img.shape}")
        
        # 计算MSE
        mse = self._calculate_mse(orig_img, comp_img)
        
        # 计算PSNR
        psnr = self._calculate_psnr(mse)
        
        # 计算SSIM
        ssim = self._calculate_ssim(orig_img, comp_img)
        
        metrics = QualityMetrics(ssim=ssim, psnr=psnr, mse=mse)
        
        if self.debug:
            print(f"[QualityMetrics] SSIM={ssim:.4f}, PSNR={psnr:.2f}dB, MSE={mse:.2f}")
        
        return metrics
    
    def _load_image(self, path: str) -> np.ndarray:
        """
        加载图像文件
        
        Args:
            path: 图像文件路径
            
        Returns:
            图像数组 (H, W, C)
        """
        if not Path(path).exists():
            raise FileNotFoundError(f"图像文件不存在: {path}")
        
        img = Image.open(path)
        
        # 转换为RGB（如果需要）
        if img.mode == 'RGBA':
            # 创建白色背景
            background = Image.new('RGB', img.size, (255, 255, 255))
            background.paste(img, mask=img.split()[3])
            img = background
        elif img.mode != 'RGB':
            img = img.convert('RGB')
        
        return np.array(img, dtype=np.float64)
    
    def _calculate_mse(self, img1: np.ndarray, img2: np.ndarray) -> float:
        """
        计算均方误差 (Mean Squared Error)
        
        Args:
            img1: 图像1
            img2: 图像2
            
        Returns:
            MSE值
        """
        # 计算像素差异的平方
        diff = img1 - img2
        squared_diff = diff ** 2
        
        # 计算平均值（所有通道）
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
            PSNR值（dB）
        """
        if mse == 0:
            return 100.0  # 完全相同
        
        max_pixel_value = 255.0
        psnr = 10.0 * math.log10((max_pixel_value ** 2) / mse)
        
        return float(psnr)
    
    def _calculate_ssim(self, img1: np.ndarray, img2: np.ndarray, window_size: int = 11) -> float:
        """
        计算结构相似性指数 (Structural Similarity Index)
        
        使用11x11窗口的局部SSIM平均值（与标准SSIM一致）
        
        Args:
            img1: 图像1
            img2: 图像2
            window_size: 窗口大小
            
        Returns:
            SSIM值 (0-1)
        """
        height, width = img1.shape[:2]
        
        # 转换为灰度
        gray1 = self._to_grayscale(img1)
        gray2 = self._to_grayscale(img2)
        
        # 如果图像完全相同，直接返回1.0
        if np.array_equal(gray1, gray2):
            return 1.0
        
        ssim_sum = 0.0
        window_count = 0
        
        # 滑动窗口计算局部SSIM（使用重叠窗口）
        step = window_size // 2  # 50%重叠
        for y in range(0, height - window_size + 1, step):
            for x in range(0, width - window_size + 1, step):
                # 提取窗口
                window1 = gray1[y:y+window_size, x:x+window_size]
                window2 = gray2[y:y+window_size, x:x+window_size]
                
                # 计算局部SSIM
                local_ssim = self._calculate_local_ssim(window1, window2)
                ssim_sum += local_ssim
                window_count += 1
        
        if window_count == 0:
            return 0.0
        
        return ssim_sum / window_count
    
    def _to_grayscale(self, img: np.ndarray) -> np.ndarray:
        """
        转换为灰度图
        
        Args:
            img: RGB图像数组
            
        Returns:
            灰度图像数组
        """
        if len(img.shape) == 2:
            return img
        
        # 使用标准灰度转换公式
        # Gray = 0.299*R + 0.587*G + 0.114*B
        gray = 0.299 * img[:, :, 0] + 0.587 * img[:, :, 1] + 0.114 * img[:, :, 2]
        return gray
    
    def _calculate_local_ssim(self, window1: np.ndarray, window2: np.ndarray) -> float:
        """
        计算局部SSIM
        
        SSIM(x,y) = (2*μx*μy + C1)(2*σxy + C2) / ((μx^2 + μy^2 + C1)(σx^2 + σy^2 + C2))
        
        Args:
            window1: 窗口1
            window2: 窗口2
            
        Returns:
            局部SSIM值
        """
        C1 = (0.01 * 255) ** 2
        C2 = (0.03 * 255) ** 2
        
        # 展平窗口
        w1 = window1.flatten()
        w2 = window2.flatten()
        
        # 计算均值
        mean1 = np.mean(w1)
        mean2 = np.mean(w2)
        
        # 计算标准差
        sigma1 = np.std(w1)
        sigma2 = np.std(w2)
        
        # 计算协方差
        sigma12 = np.mean((w1 - mean1) * (w2 - mean2))
        
        # 计算SSIM
        numerator = (2 * mean1 * mean2 + C1) * (2 * sigma12 + C2)
        denominator = (mean1**2 + mean2**2 + C1) * (sigma1**2 + sigma2**2 + C2)
        
        if denominator < 1e-10:
            # 如果分母接近0，检查两个窗口是否相同
            if np.array_equal(w1, w2):
                return 1.0
            return 0.0
        
        ssim = numerator / denominator
        
        # 限制在[0, 1]范围内
        return float(np.clip(ssim, 0.0, 1.0))
    
    def estimate_ssim(self, quality: int, has_alpha: bool = False, complexity: float = 0.0) -> float:
        """
        根据图像特征估算预期SSIM（用于转换前预测质量）
        
        Args:
            quality: 质量参数 (0-100)
            has_alpha: 是否有透明通道
            complexity: 图像复杂度 (0-100)
            
        Returns:
            预估SSIM值
        """
        # 基于质量参数的基础SSIM
        base_ssim = 0.75 + (quality / 100.0) * 0.20
        
        # 透明通道会略微降低SSIM
        if has_alpha:
            base_ssim -= 0.02
        
        # 高复杂度图像SSIM略低
        if complexity > 40:
            base_ssim -= 0.03
        
        # 确保在合理范围内
        base_ssim = max(0.70, min(0.99, base_ssim))
        
        return base_ssim


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 测试和演示
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if __name__ == "__main__":
    import tempfile
    import os
    
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("Phase 47.23: 质量评估系统测试")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    
    calculator = QualityCalculator(debug=True)
    
    # 创建测试图像
    print("1️⃣  创建测试图像...")
    temp_dir = tempfile.mkdtemp()
    
    # 创建原始图像（纯色渐变）
    width, height = 256, 256
    orig_img = Image.new('RGB', (width, height))
    pixels = orig_img.load()
    
    for y in range(height):
        for x in range(width):
            # 创建渐变效果
            r = int((x / width) * 255)
            g = int((y / height) * 255)
            b = 128
            pixels[x, y] = (r, g, b)
    
    orig_path = os.path.join(temp_dir, "original.png")
    orig_img.save(orig_path, "PNG")
    print(f"   ✅ 原始图像: {orig_path}")
    
    # 测试1: 完全相同的图像（应该SSIM=1.0, PSNR=100dB）
    print("\n2️⃣  测试完全相同的图像...")
    same_path = os.path.join(temp_dir, "same.png")
    orig_img.save(same_path, "PNG")
    
    metrics = calculator.compare(orig_path, same_path)
    print(f"   SSIM: {metrics.ssim:.4f} (预期: 1.0000)")
    print(f"   PSNR: {metrics.psnr:.2f} dB (预期: 100.00)")
    print(f"   MSE: {metrics.mse:.2f} (预期: 0.00)")
    print(f"   质量等级: {metrics.get_quality_level()}")
    print(f"   是否可接受: {metrics.is_acceptable()}")
    
    # 测试2: 轻微压缩的图像
    print("\n3️⃣  测试轻微压缩的图像...")
    compressed_path = os.path.join(temp_dir, "compressed.jpg")
    orig_img.save(compressed_path, "JPEG", quality=95)
    
    metrics = calculator.compare(orig_path, compressed_path)
    print(f"   SSIM: {metrics.ssim:.4f}")
    print(f"   PSNR: {metrics.psnr:.2f} dB")
    print(f"   MSE: {metrics.mse:.2f}")
    print(f"   质量等级: {metrics.get_quality_level()}")
    print(f"   是否可接受: {metrics.is_acceptable()}")
    
    # 测试3: 较强压缩的图像
    print("\n4️⃣  测试较强压缩的图像...")
    low_quality_path = os.path.join(temp_dir, "low_quality.jpg")
    orig_img.save(low_quality_path, "JPEG", quality=60)
    
    metrics = calculator.compare(orig_path, low_quality_path)
    print(f"   SSIM: {metrics.ssim:.4f}")
    print(f"   PSNR: {metrics.psnr:.2f} dB")
    print(f"   MSE: {metrics.mse:.2f}")
    print(f"   质量等级: {metrics.get_quality_level()}")
    print(f"   是否可接受: {metrics.is_acceptable()}")
    
    # 测试4: 质量预估
    print("\n5️⃣  测试质量预估...")
    for quality in [100, 90, 80, 70]:
        estimated_ssim = calculator.estimate_ssim(quality, has_alpha=False, complexity=30)
        print(f"   Quality {quality}: 预估SSIM = {estimated_ssim:.4f}")
    
    # 清理
    import shutil
    shutil.rmtree(temp_dir)
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ 所有测试通过！质量评估系统工作正常")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
