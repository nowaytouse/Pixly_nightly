#!/usr/bin/env python3
"""
并行处理性能测试
验证Rust批量处理的性能提升
"""

import subprocess
import time
from pathlib import Path
import json
import sys

def test_parallel_performance():
    """测试并行 vs 串行性能对比"""
    
    print("="*70)
    print("🚀 并行处理性能测试")
    print("="*70)
    
    root_dir = Path(__file__).parent.parent
    pixly_bin = root_dir / 'core' / 'rust' / 'target' / 'release' / 'pixly-rust'
    test_files = root_dir / '@reference' / 'data'
    output_dir = root_dir / 'test_data_output' / 'parallel_test'
    
    # 检查二进制文件
    if not pixly_bin.exists():
        print("❌ pixly-rust 未找到，请先编译：")
        print("   cd core/rust && cargo build --release")
        return False
    
    # 准备输出目录
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # 选择测试文件
    image_files = []
    for ext in ['*.png', '*.jpg', '*.webp']:
        image_files.extend(list(test_files.glob(ext)))
    
    # 限制到10个文件用于测试
    test_images = image_files[:10]
    
    if len(test_images) < 5:
        print(f"❌ 测试文件不足（需要至少5个，找到{len(test_images)}个）")
        return False
        
    print(f"📂 测试文件: {len(test_images)}个")
    
    results = {}
    
    # 测试不同的线程数配置
    thread_configs = [
        (1, "串行处理"),
        (2, "2线程并行"),
        (4, "4线程并行"),
        (0, "自动线程数")
    ]
    
    for threads, desc in thread_configs:
        print(f"\n🔄 测试: {desc} (threads={threads})")
        
        # 清理输出目录
        for f in output_dir.glob("*"):
            if f.is_file():
                f.unlink()
        
        # 构建命令
        cmd = [
            str(pixly_bin),
            'batch',
            str(test_files),
            str(output_dir),
            'avif',
            '--quality', '85',
            '--threads', str(threads)
        ]
        
        # 执行并计时
        start_time = time.time()
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=120  # 2分钟超时
            )
            
            elapsed = time.time() - start_time
            
            if result.returncode == 0:
                # 统计成功转换的文件数
                output_files = list(output_dir.glob("*.avif"))
                
                print(f"   ✅ 完成: {len(output_files)}个文件")
                print(f"   ⏱️ 耗时: {elapsed:.2f}秒")
                print(f"   📈 速度: {len(output_files)/elapsed:.2f} 文件/秒")
                
                results[threads] = {
                    'desc': desc,
                    'files': len(output_files),
                    'time': elapsed,
                    'speed': len(output_files) / elapsed if elapsed > 0 else 0,
                    'success': True
                }
                
            else:
                print(f"   ❌ 失败: {result.stderr[:100]}")
                results[threads] = {
                    'desc': desc,
                    'success': False,
                    'error': result.stderr
                }
                
        except subprocess.TimeoutExpired:
            print(f"   ❌ 超时 (>2分钟)")
            results[threads] = {
                'desc': desc,
                'success': False,
                'error': 'Timeout'
            }
        except Exception as e:
            print(f"   ❌ 错误: {str(e)}")
            results[threads] = {
                'desc': desc,
                'success': False,
                'error': str(e)
            }
    
    # 性能对比分析
    print("\n" + "="*70)
    print("📊 性能对比分析")
    print("="*70)
    
    successful_results = {k: v for k, v in results.items() if v.get('success')}
    
    if len(successful_results) < 2:
        print("❌ 成功测试不足，无法进行对比")
        return False
    
    # 找到基准（1线程串行）
    baseline = successful_results.get(1)
    
    print(f"{'配置':<12} {'文件数':<8} {'耗时(s)':<10} {'速度':<12} {'提升':<10}")
    print("-" * 60)
    
    for threads, data in successful_results.items():
        speed_str = f"{data['speed']:.2f} 文件/秒"
        
        if baseline and threads != 1:
            speedup = data['speed'] / baseline['speed']
            improvement = f"{speedup:.2f}x"
        else:
            improvement = "基准"
            
        print(f"{data['desc']:<12} {data['files']:<8} {data['time']:<10.2f} {speed_str:<12} {improvement:<10}")
    
    # 计算最佳性能提升
    if baseline:
        best_speed = max(d['speed'] for d in successful_results.values())
        max_speedup = best_speed / baseline['speed']
        
        print(f"\n🎯 性能提升总结:")
        print(f"   最大加速比: {max_speedup:.2f}x")
        
        if max_speedup >= 2.0:
            print("   ✅ 优秀: 并行处理效果显著")
        elif max_speedup >= 1.5:
            print("   ✅ 良好: 并行处理有明显提升")
        else:
            print("   ⚠️  一般: 并行提升有限，可能受I/O或其他因素限制")
    
    # 保存结果
    report_path = output_dir / 'performance_report.json'
    with open(report_path, 'w') as f:
        json.dump({
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
            'test_files': len(test_images),
            'results': results
        }, f, indent=2)
    
    print(f"\n📄 详细报告: {report_path}")
    print("="*70)
    
    return True

def main():
    if test_parallel_performance():
        print("✅ 并行性能测试完成")
        return 0
    else:
        print("❌ 并行性能测试失败")
        return 1

if __name__ == '__main__':
    sys.exit(main())
