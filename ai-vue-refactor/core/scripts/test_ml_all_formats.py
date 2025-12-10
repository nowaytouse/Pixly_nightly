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
    """Test all format support"""
    print("🧪 Testing ML System Format Support", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    
    router = ModelRouter()
    features = create_test_features()
    
    # Test all image formats
    formats = ["webp", "avif", "jxl", "png", "jpeg", "gif", "bmp", "tiff"]
    
    print("\n📷 Image Format Prediction Testing:", file=sys.stderr)
    print("-" * 60, file=sys.stderr)
    
    results = {}
    for fmt in formats:
        try:
            # Use LightGBM model for prediction
            prediction = router.predict("lightgbm", features, fmt, "balanced")
            
            results[fmt] = {
                "quality": prediction.quality,
                "effort": prediction.effort,
                "confidence": prediction.confidence,
                "status": "✅"
            }
            
            print(f"  {fmt:8s}: ✅ Q={prediction.quality:2d} E={prediction.effort:2d} C={prediction.confidence:.2f}", file=sys.stderr)
            
        except Exception as e:
            results[fmt] = {
                "error": str(e),
                "status": "❌"
            }
            print(f"  {fmt:8s}: ❌ {e}", file=sys.stderr)
    
    # Test same-format optimization
    print("\n🔄 Same Format Optimization Prediction Testing:", file=sys.stderr)
    print("-" * 60, file=sys.stderr)
    
    same_format_tests = ["jpeg", "png", "webp"]
    for fmt in same_format_tests:
        try:
            prediction = router.predict("lightgbm", features, fmt, "balanced")
            print(f"  {fmt}→{fmt}: ✅ Q={prediction.quality:2d} (Optimization)", file=sys.stderr)
        except Exception as e:
            print(f"  {fmt}→{fmt}: ❌ {e}", file=sys.stderr)
    
    # Statistics
    print("\n" + "=" * 60, file=sys.stderr)
    success_count = sum(1 for r in results.values() if r.get("status") == "✅")
    total_count = len(results)
    
    print(f"📊 Testing Result: {success_count}/{total_count} formats supported", file=sys.stderr)
    
    if success_count == total_count:
        print("✅ All format ML predictions working correctly!", file=sys.stderr)
        return 0
    else:
        print(f"⚠️  {total_count - success_count} formats need fixing", file=sys.stderr)
        return 1

if __name__ == "__main__":
    sys.exit(test_format_support())
