#!/usr/bin/env python3
"""
快速转换测试 - 仅测试3-5个文件验证基本功能
"""
import subprocess
import sys
from pathlib import Path
import time

test_root = Path(__file__).parent.parent
pixly_bin = test_root / 'core' / 'rust' / 'target' / 'release' / 'pixly-rust'
data_dir = test_root / '@reference' / 'data'
output_dir = test_root / 'test_data_output' / 'quick_test'

output_dir.mkdir(parents=True, exist_ok=True)

# 选择几个小文件快速测试
test_cases = [
    ('four-colors.png', 'avif'),
    ('peach_pi-1280x720.jpg', 'jxl'),
    ('sfx.mp3', 'aac'),
]

print("="*70)
print("🚀 快速转换测试 (3个文件)")
print("="*70)

success = 0
failed = 0

for filename, target_format in test_cases:
    input_path = data_dir / filename
    
    if not input_path.exists():
        print(f"⊘ {filename} - 文件不存在")
        continue
        
    output_path = output_dir / (input_path.stem + '.' + target_format)
    
    if output_path.exists():
        output_path.unlink()
        
    print(f"\n🔄 {filename} → {target_format}...", flush=True)
    
    start = time.time()
    
    try:
        result = subprocess.run(
            [str(pixly_bin), 'convert', str(input_path), str(output_path), 
             '--format', target_format, '--quality', '85'],
            capture_output=True,
            text=True,
            timeout=10
        )
        
        elapsed = time.time() - start
        
        if result.returncode == 0 and output_path.exists():
            input_size = input_path.stat().st_size
            output_size = output_path.stat().st_size
            reduction = (1 - output_size / input_size) * 100
            print(f"✅ 成功: {input_size/1024:.1f}KB → {output_size/1024:.1f}KB ({reduction:+.1f}%) {elapsed:.2f}s")
            success += 1
        else:
            print(f"❌ 失败: {result.stderr[:100]}")
            failed += 1
            
    except subprocess.TimeoutExpired:
        print(f"❌ 超时 (>10s)")
        failed += 1
    except Exception as e:
        print(f"❌ 错误: {str(e)[:100]}")
        failed += 1

print(f"\n{'='*70}")
print(f"📊 结果: ✅ {success}  ❌ {failed}")
print(f"{'='*70}")

sys.exit(0 if failed == 0 else 1)
