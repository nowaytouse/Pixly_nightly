#!/usr/bin/env python3
"""
🔍 PPO训练环境检查脚本

检查PPO训练所需的所有依赖
"""

import sys

def check_pytorch():
    """检查PyTorch"""
    try:
        import torch
        print(f"✅ PyTorch: {torch.__version__}")
        print(f"   CUDA available: {torch.cuda.is_available()}")
        if torch.cuda.is_available():
            print(f"   CUDA version: {torch.version.cuda}")
        return True
    except ImportError:
        print("❌ PyTorch not installed")
        print("   Install: pip install torch")
        return False

def check_numpy():
    """检查NumPy"""
    try:
        import numpy as np
        print(f"✅ NumPy: {np.__version__}")
        return True
    except ImportError:
        print("❌ NumPy not installed")
        print("   Install: pip install numpy")
        return False

def check_rust_cli():
    """检查Rust CLI"""
    import subprocess
    try:
        result = subprocess.run(
            ['cargo', 'run', '--release', '--bin', 'pixly-converter', '--', '--version'],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            print("✅ Rust CLI available")
            return True
        else:
            print("❌ Rust CLI not working")
            return False
    except Exception as e:
        print(f"❌ Rust CLI check failed: {e}")
        return False

def check_imagemagick():
    """检查ImageMagick"""
    import subprocess
    try:
        result = subprocess.run(
            ['magick', '--version'],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            version_line = result.stdout.split('\n')[0]
            print(f"✅ ImageMagick: {version_line}")
            return True
        else:
            print("❌ ImageMagick not working")
            return False
    except FileNotFoundError:
        print("❌ ImageMagick not installed")
        print("   Install: brew install imagemagick")
        return False
    except Exception as e:
        print(f"❌ ImageMagick check failed: {e}")
        return False

def main():
    print("="*60)
    print("🔍 PPO Training Requirements Check")
    print("="*60)
    
    checks = {
        'PyTorch': check_pytorch(),
        'NumPy': check_numpy(),
        'Rust CLI': check_rust_cli(),
        'ImageMagick': check_imagemagick(),
    }
    
    print("\n" + "="*60)
    print("📊 Summary")
    print("="*60)
    
    passed = sum(checks.values())
    total = len(checks)
    
    for name, status in checks.items():
        status_str = "✅" if status else "❌"
        print(f"{status_str} {name}")
    
    print(f"\nPassed: {passed}/{total}")
    
    if passed == total:
        print("\n✅ All requirements met! Ready to train PPO.")
        return 0
    else:
        print("\n❌ Some requirements missing. Please install them first.")
        print("\n📝 Installation Guide:")
        if not checks['PyTorch']:
            print("   PyTorch: pip install torch")
        if not checks['NumPy']:
            print("   NumPy: pip install numpy")
        if not checks['ImageMagick']:
            print("   ImageMagick: brew install imagemagick")
        if not checks['Rust CLI']:
            print("   Rust CLI: cargo build --release --bin pixly-converter")
        return 1

if __name__ == '__main__':
    sys.exit(main())
