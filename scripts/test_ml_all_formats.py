#!/usr/bin/env python3
"""测试ML系统对所有格式的支持"""

import sys
import json
import numpy as np
from pathlib import Path

# 添加scripts目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from ml_bridge import ModelRouter, StandardFeatures

def create_test_features():
    """创建测试特征"""
    return StandardFeatures(
        basic=[1920, 1080, 2073600, 0.5, 1.0, 0.0, 0.0, 0.0] + [0.0] * 8,
        color=[0.5] * 16,
        texture=[0.5] * 16,
        shape=[0.5] * 16,
        quality=[0.5] * 16,
        metadata=[0.5] * 32,
        context=[0.5] * 16
    )

def test_format_support():
    """测试所有格式支持"""
    print("🧪 TestingML系统Format支持")
    print("=" * 60)
    
    router = ModelRouter()
    features = create_test_features()
    
    # 测试所有图像格式
    formats = ["webp", "avif", "jxl", "png", "jpeg", "gif", "bmp", "tiff"]
    
    print("\n📷 ImagesFormatPredictionTesting:")
    print("-" * 60)
    
    results = {}
    for fmt in formats:
        try:
            # 使用LightGBM模型预测
            prediction = router.predict("lightgbm", features, fmt, "balanced")
            
            results[fmt] = {
                "quality": prediction.quality,
                "effort": prediction.effort,
                "confidence": prediction.confidence,
                "status": "✅"
            }
            
            print(f"  {fmt:8s}: ✅ Q={prediction.quality:2d} E={prediction.effort:2d} C={prediction.confidence:.2f}")
            
        except Exception as e:
            results[fmt] = {
                "error": str(e),
                "status": "❌"
            }
            print(f"  {fmt:8s}: ❌ {e}")
    
    # 测试同格式优化
    print("\n🔄 同FormatOptimizationPredictionTesting:")
    print("-" * 60)
    
    same_format_tests = ["jpeg", "png", "webp"]
    for fmt in same_format_tests:
        try:
            prediction = router.predict("lightgbm", features, fmt, "balanced")
            print(f"  {fmt}→{fmt}: ✅ Q={prediction.quality:2d} (Optimization)")
        except Exception as e:
            print(f"  {fmt}→{fmt}: ❌ {e}")
    
    # 统计
    print("\n" + "=" * 60)
    success_count = sum(1 for r in results.values() if r.get("status") == "✅")
    total_count = len(results)
    
    print(f"📊 TestingResult: {success_count}/{total_count} Format支持")
    
    if success_count == total_count:
        print("✅ 所有FormatMLPrediction正常工作!")
        return 0
    else:
        print(f"⚠️  {total_count - success_count} Format需要修复")
        return 1

if __name__ == "__main__":
    sys.exit(test_format_support())
