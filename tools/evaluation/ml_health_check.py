#!/usr/bin/env python3
"""
🏥 ML系统健康检查
检查所有ML组件的状态和性能
"""

import sys
import json
from pathlib import Path
import numpy as np

# 添加scripts目录到路径
sys.path.insert(0, str(Path(__file__).parent))

from ml_bridge import ModelRouter, StandardFeatures, ModelType

def check_dependencies():
    """检查Python依赖"""
 print("📦 Checking Python dependencies...", file=sys.stderr)    
    deps = {
        'numpy': None,
        'torch': None,
        'lightgbm': None,
    }
    
    for dep in deps:
        try:
            module = __import__(dep)
            deps[dep] = getattr(module, '__version__', 'unknown')
            print(f"  ✅ {dep}: {deps[dep]}")
        except ImportError:
            deps[dep] = None
            print(f"  ❌ {dep}: NOT INSTALLED")
    
    return all(v is not None for v in deps.values())

def check_models():
    """检查模型文件"""
 print("\n🤖 modelfiles...", file=sys.stderr)    
    models_dir = Path("models")
    if not models_dir.exists():
 print(f" ❌ models directory does not exist", file=sys.stderr)        return False
    
    # 检查LightGBM模型
    lightgbm_models = list(models_dir.glob("lightgbm_*.txt"))
 print(f" 📊 LightGBM models: {len(lightgbm_models)}", file=sys.stderr)    for model in lightgbm_models:
        size_kb = model.stat().st_size / 1024
        print(f"     - {model.name} ({size_kb:.1f} KB)")
    
    # 检查PPO模型
    ppo_dir = models_dir / "ppo"
    if ppo_dir.exists():
        ppo_models = list(ppo_dir.glob("*.pth"))
        print(f"  🎮 PPO models: {len(ppo_models)}", file=sys.stderr)
        for model in ppo_models[:5]:  # Show first 5 only
            size_kb = model.stat().st_size / 1024
            print(f"     - {model.name} ({size_kb:.1f} KB)", file=sys.stderr)
    else:
        print(f"  ❌ PPO directory does not exist", file=sys.stderr)
    
    # Check training data
    training_data = models_dir / "training_data_final.json"
    if training_data.exists():
        size_mb = training_data.stat().st_size / (1024 * 1024)
 print(f" 📚 Training data: {size_mb:.1f} MB", file=sys.stderr)    else:
 print(f" ⚠️ Training data does not exist", file=sys.stderr)    
    return len(lightgbm_models) > 0 or (ppo_dir.exists() and len(list(ppo_dir.glob("*.pth"))) > 0)

def check_model_router():
    """检查模型路由器"""
 print("\n🔀 Checking model router...", file=sys.stderr)    
    try:
        router = ModelRouter()
        available = router.available_models
        
 print(f" model:", file=sys.stderr)        for model_type, is_available in available.items():
            status = "✅" if is_available else "❌"
            print(f"    {status} {model_type.value.upper()}")
        
        return any(available.values())
    except Exception as e:
 print(f" ❌ Router initialization failed: {e}", file=sys.stderr)        return False

def test_prediction():
    """测试预测功能"""
 print("\n🧪...", file=sys.stderr)    
    try:
        router = ModelRouter()
        
        # 创建测试特征
        test_features = StandardFeatures(
            basic=[0.5] * 16,
            color=[0.5] * 16,
            texture=[0.5] * 16,
            shape=[0.5] * 16,
            quality=[0.5] * 16,
            metadata=[0.5] * 32,
            context=[0.5] * 16
        )
        
        # 测试每个可用模型
        for model_type, is_available in router.available_models.items():
            if not is_available:
                continue
            
            try:
                prediction = router.predict(
                    model_type=model_type,
                    features=test_features,
                    target_format="webp",
                    quality_mode="balanced"
                )
                
                print(f"  ✅ {model_type.value.upper()}:")
                print(f"     Quality: {prediction.quality}")
                print(f"     Effort: {prediction.effort}")
                print(f"     Confidence: {prediction.confidence:.2%}")
                
            except Exception as e:
                print(f"  ❌ {model_type.value.upper()}: {e}")
        
        return True
        
    except Exception as e:
 print(f" ❌: {e}", file=sys.stderr)        import traceback
        traceback.print_exc()
        return False

def main():
    """主函数"""
    print("=" * 60)
 print("🏥 Pixly ML", file=sys.stderr)    print("=" * 60)
    
    results = {
        'dependencies': check_dependencies(),
        'models': check_models(),
        'router': check_model_router(),
        'prediction': test_prediction(),
    }
    
    print("\n" + "=" * 60)
 print("📊:", file=sys.stderr)    print("=" * 60)
    
    for check, passed in results.items():
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"  {check.upper()}: {status}")
    
    all_passed = all(results.values())
    
    print("\n" + "=" * 60)
    if all_passed:
 print("✅ All checks passed！ML", file=sys.stderr)        return 0
    else:
 print("❌ ，", file=sys.stderr)        return 1

if __name__ == "__main__":
    sys.exit(main())
