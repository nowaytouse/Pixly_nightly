#!/usr/bin/env python3
"""
Pixly SSIM Calculator
Version: 4.2.0

计算两张图像的结构相似性指数（SSIM）
"""

import sys
from PIL import Image
import numpy as np
from skimage.metrics import structural_similarity as ssim

def calculate_ssim(image1_path, image2_path):
    """
    计算两张图像的SSIM
    
    Args:
        image1_path: 原始图像路径
        image2_path: 转换后图像路径
    
    Returns:
        float: SSIM值 (0-1)
    """
    try:
        # 加载图像
        img1 = Image.open(image1_path).convert('RGB')
        img2 = Image.open(image2_path).convert('RGB')
        
        # 确保尺寸一致
        if img1.size != img2.size:
            # 缩放到相同尺寸
            img2 = img2.resize(img1.size, Image.LANCZOS)
        
        # 转换为numpy数组
        arr1 = np.array(img1)
        arr2 = np.array(img2)
        
        # 计算SSIM（multi-channel模式，返回RGB的平均SSIM）
        ssim_value = ssim(arr1, arr2, channel_axis=2, data_range=255)
        
        return float(ssim_value)
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 0.0

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: calculate_ssim.py <original> <converted>", file=sys.stderr)
        sys.exit(1)
    
    original_path = sys.argv[1]
    converted_path = sys.argv[2]
    
    ssim_value = calculate_ssim(original_path, converted_path)
    
    # 输出SSIM值（stdout，供JS读取）
    print(f"{ssim_value:.6f}")
