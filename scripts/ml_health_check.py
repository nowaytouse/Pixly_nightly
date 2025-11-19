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
    print("📦 检查Python依赖...")
    
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
    print("\n🤖 检查模型文件...")
    
    models_dir = Path("models")
    if not models_dir.exists():
        print(f"  ❌ models目录不存在")
        return False
    
    # 检查LightGBM模型
    lightgbm_models = list(models_dir.glob("lightgbm_*.txt"))
    print(f"  📊 LightGBM模型: {len(lightgbm_models)}个")
    for model in lightgbm_models:
        size_kb = model.stat().st_size / 1024
        print(f"     - {model.name} ({size_kb:.1f} KB)")
    
    # 检查PPO模型
    ppo_dir = models_dir / "ppo"
    if ppo_dir.exists():
        ppo_models = list(ppo_dir.glob("*.pth"))
        print(f"  🎮 PPO模型: {len(ppo_models)}个")
        for model in ppo_models[:5]:  # 只显示前5个
            size_kb = model.stat().st_size / 1024
            print(f"     - {model.name} ({size_kb:.1f} KB)")
    else:
        print(f"  ❌ PPO目录不存在")
    
    # 检查训练数据
    training_data = models_dir / "training_data_final.json"
    if training_data.exists():
        size_mb = training_data.stat().st_size / (1024 * 1024)
        print(f"  📚 训练数据: {size_mb:.1f} MB")
    else:
        print(f"  ⚠️  训练数据不存在")
    
    return len(lightgbm_models) > 0 or (ppo_dir.exists() and len(list(ppo_dir.glob("*.pth"))) > 0)

def check_model_router():
    """检查模型路由器"""
    print("\n🔀 检查模型路由器...")
    
    try:
        router = ModelRouter()
        available = router.available_models
        
        print(f"  可用模型:")
        for model_type, is_available in available.items():
            status = "✅" if is_available else "❌"
            print(f"    {status} {model_type.value.upper()}")
        
        return any(available.values())
    except Exception as e:
        print(f"  ❌ 路由器初始化失败: {e}")
        return False

def test_prediction():
    """测试预测功能"""
    print("\n🧪 测试预测功能...")
    
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
        print(f"  ❌ 预测测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """主函数"""
    print("=" * 60)
    print("🏥 Pixly ML系统健康检查")
    print("=" * 60)
    
    results = {
        'dependencies': check_dependencies(),
        'models': check_models(),
        'router': check_model_router(),
        'prediction': test_prediction(),
    }
    
    print("\n" + "=" * 60)
    print("📊 健康检查结果:")
    print("=" * 60)
    
    for check, passed in results.items():
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"  {check.upper()}: {status}")
    
    all_passed = all(results.values())
    
    print("\n" + "=" * 60)
    if all_passed:
        print("✅ 所有检查通过！ML系统健康")
        return 0
    else:
        print("❌ 部分检查失败，请修复问题")
        return 1

if __name__ == "__main__":
    sys.exit(main())
