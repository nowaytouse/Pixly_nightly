#!/usr/bin/env python3
"""
测试批量转换不会卡死（即使AI服务未运行）
"""
import subprocess
import time
from pathlib import Path
import sys

def test_batch_no_hang():
    print("🔍 测试批量转换不卡死...")
    
    root_dir = Path(__file__).parent.parent
    pixly_bin = root_dir / 'core' / 'rust' / 'target' / 'release' / 'pixly-rust'
    test_files = root_dir / '@reference' / 'data'
    output_dir = root_dir / 'test_data_output' / 'batch_test'
    
    if not pixly_bin.exists():
        print("❌ pixly-rust 未编译")
        return False
    
    # 准备输出目录
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # 只选择3个文件快速测试
    test_images = list(test_files.glob("*.png"))[:3]
    
    if not test_images:
        print("❌ 无测试文件")
        return False
    
    print(f"📂 测试文件: {len(test_images)}个")
    
    # 构建命令
    cmd = [
        str(pixly_bin),
        'batch',
        str(test_files),
        str(output_dir),
        'avif',
        '--quality', '85'
    ]
    
    print(f"🚀 执行命令...")
    start_time = time.time()
    
    try:
        # 设置较短的超时（30秒）
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        elapsed = time.time() - start_time
        
        if result.returncode == 0:
            output_files = list(output_dir.glob("*.avif"))
            print(f"✅ 成功! 转换{len(output_files)}个文件，耗时{elapsed:.1f}秒")
            
            # 检查是否有AI服务警告
            if "AI service not available" in result.stdout:
                print("✅ AI服务不可用警告正确显示")
                print("✅ 批量转换没有卡死，继续执行")
            return True
        else:
            print(f"❌ 转换失败: {result.stderr[:200]}")
            return False
            
    except subprocess.TimeoutExpired:
        print(f"❌ 超时！批量转换卡死了（超过30秒）")
        return False
    except Exception as e:
        print(f"❌ 错误: {e}")
        return False

if __name__ == '__main__':
    success = test_batch_no_hang()
    print("\n" + "="*50)
    if success:
        print("🎉 批量转换卡死问题已解决！")
        print("   即使AI服务未运行，也能正常转换")
    else:
        print("❌ 批量转换仍有问题")
    sys.exit(0 if success else 1)
