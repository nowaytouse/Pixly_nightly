#!/usr/bin/env python3
"""
🔍 简化的SSIM计算工具

使用PIL和numpy计算简化的SSIM（MSE-based approximation）
"""

import sys
from pathlib import Path

def calculate_ssim_simple(image1_path, image2_path):
    """
    使用PIL计算简化的SSIM
    
    实际上是基于MSE的质量估算：
    SSIM ≈ 1 - normalized_MSE
    """
    try:
        from PIL import Image
        import numpy as np
        
        # 读取图像
        img1 = Image.open(image1_path).convert('RGB')
        img2 = Image.open(image2_path).convert('RGB')
        
        # 调整到相同尺寸
        if img1.size != img2.size:
            # 调整到较小的尺寸
            min_size = tuple(min(s1, s2) for s1, s2 in zip(img1.size, img2.size))
            img1 = img1.resize(min_size, Image.Resampling.LANCZOS)
            img2 = img2.resize(min_size, Image.Resampling.LANCZOS)
        
        # 转换为numpy数组
        arr1 = np.array(img1, dtype=np.float32)
        arr2 = np.array(img2, dtype=np.float32)
        
        # 计算MSE
        mse = np.mean((arr1 - arr2) ** 2)
        
        # 归一化MSE（假设像素范围0-255）
        max_mse = 255.0 ** 2
        normalized_mse = mse / max_mse
        
        # 转换为SSIM近似值
        # SSIM ≈ 1 - sqrt(normalized_MSE)
        ssim_approx = 1.0 - np.sqrt(normalized_mse)
        
        # 确保在0-1范围内
        ssim_approx = max(0.0, min(1.0, ssim_approx))
        
        return ssim_approx
        
    except Exception as e:
        print(f"Error calculating SSIM: {e}", file=sys.stderr)
        return None

def main():
    if len(sys.argv) != 3:
        print("Usage: calculate_ssim.py <image1> <image2>")
        sys.exit(1)
    
    image1 = sys.argv[1]
    image2 = sys.argv[2]
    
    ssim_value = calculate_ssim_simple(image1, image2)
    
    if ssim_value is not None:
        print(f"{ssim_value:.6f}")
        sys.exit(0)
    else:
        sys.exit(1)

if __name__ == '__main__':
    main()
