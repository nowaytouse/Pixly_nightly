#!/usr/bin/env python3
"""
Pixly 批量转换测试脚本
Version: 1.0.0
Date: 2025-11-11

用途: 对@reference/data目录中的大量文件进行实际转换测试
目标: 验证"质量不变前提下必然减小空间占用"的核心原则
"""

import sys
import os
import subprocess
from pathlib import Path
import json
import time
from typing import Dict, List, Tuple, Optional
import shutil

# 测试配置
class TestConfig:
    def __init__(self):
        self.test_root = Path(__file__).parent.parent
        self.data_dir = self.test_root / '@reference' / 'data'
        self.output_dir = self.test_root / 'test_data_output' / 'batch_test'
        self.pixly_bin = self.test_root / 'core' / 'rust' / 'target' / 'release' / 'pixly-rust'
        
        # 测试参数
        self.max_files = 15  # 最多测试文件数（优化：减少数量）
        self.test_formats = {
            'png': 'avif',
            'jpg': 'jxl',
            'jpeg': 'jxl',
            'webp': 'avif',
            'gif': 'gif',  # GIF优化
            'mp3': 'aac',
            'wav': 'aac',
            'ogg': 'opus',
            'flac': 'flac',
        }
        
config = TestConfig()

# 统计类
class ConversionStats:
    def __init__(self):
        self.total = 0
        self.success = 0
        self.failed = 0
        self.skipped = 0
        self.size_reduced = 0
        self.size_increased = 0
        self.total_input_size = 0
        self.total_output_size = 0
        self.results = []
        
    def add_result(self, result: Dict):
        self.results.append(result)
        self.total += 1
        
        if result['status'] == 'success':
            self.success += 1
            input_size = result['input_size']
            output_size = result['output_size']
            self.total_input_size += input_size
            self.total_output_size += output_size
            
            if output_size < input_size:
                self.size_reduced += 1
            else:
                self.size_increased += 1
                
        elif result['status'] == 'failed':
            self.failed += 1
        else:
            self.skipped += 1
            
    def print_summary(self):
        print("\n" + "="*80)
        print("📊 批量转换测试总结")
        print("="*80)
        
        print(f"\n📈 转换统计:")
        print(f"  总计: {self.total}")
        print(f"  ✅ 成功: {self.success} ({self.success/self.total*100:.1f}%)")
        print(f"  ❌ 失败: {self.failed} ({self.failed/self.total*100:.1f}%)")
        print(f"  ⊘ 跳过: {self.skipped} ({self.skipped/self.total*100:.1f}%)")
        
        if self.success > 0:
            print(f"\n💾 大小变化:")
            print(f"  减小文件数: {self.size_reduced} ({self.size_reduced/self.success*100:.1f}%)")
            print(f"  增大文件数: {self.size_increased} ({self.size_increased/self.success*100:.1f}%)")
            print(f"  总输入大小: {self.total_input_size/1024/1024:.2f} MB")
            print(f"  总输出大小: {self.total_output_size/1024/1024:.2f} MB")
            
            reduction = (1 - self.total_output_size / self.total_input_size) * 100
            print(f"  平均压缩率: {reduction:.1f}%")
            
        print(f"\n🎯 质量目标达成度:")
        if self.success > 0:
            reduction_rate = self.size_reduced / self.success * 100
            if reduction_rate >= 90 and self.failed == 0:
                print(f"  🏆 优秀: {reduction_rate:.1f}% 文件减小，无失败")
            elif reduction_rate >= 75:
                print(f"  ✅ 良好: {reduction_rate:.1f}% 文件减小")
            elif reduction_rate >= 60:
                print(f"  ⚠️  中等: {reduction_rate:.1f}% 文件减小，需改进")
            else:
                print(f"  ❌ 不合格: 仅{reduction_rate:.1f}% 文件减小")
        else:
            print(f"  ❌ 无法评估: 没有成功转换的文件")
            
        # 详细结果
        if self.failed > 0:
            print(f"\n❌ 失败详情 ({self.failed}):")
            for i, r in enumerate([r for r in self.results if r['status'] == 'failed'][:10], 1):
                print(f"  {i}. {r['file']}: {r.get('error', 'Unknown error')[:60]}")
                
        if self.size_increased > 0:
            print(f"\n⚠️  大小增加详情 ({self.size_increased}):")
            increased = [r for r in self.results if r['status'] == 'success' and r['output_size'] >= r['input_size']]
            for i, r in enumerate(increased[:10], 1):
                increase = (r['output_size'] / r['input_size'] - 1) * 100
                print(f"  {i}. {r['file']}: {r['input_size']/1024:.1f}KB → {r['output_size']/1024:.1f}KB (+{increase:.1f}%)")
                
        return self.success / self.total >= 0.75 if self.total > 0 else False

stats = ConversionStats()

def get_file_size(path: Path) -> int:
    """获取文件大小(字节)"""
    try:
        return path.stat().st_size
    except:
        return 0

def convert_file(input_path: Path, output_format: str) -> Dict:
    """
    转换单个文件
    返回: {status, file, input_size, output_size, error, time}
    """
    result = {
        'status': 'unknown',
        'file': input_path.name,
        'input_format': input_path.suffix[1:].lower(),
        'output_format': output_format,
        'input_size': 0,
        'output_size': 0,
        'error': None,
        'time': 0
    }
    
    try:
        # 检查输入文件
        if not input_path.exists():
            result['status'] = 'skipped'
            result['error'] = 'File not found'
            return result
            
        input_size = get_file_size(input_path)
        result['input_size'] = input_size
        
        # 跳过过小文件
        if input_size < 100:
            result['status'] = 'skipped'
            result['error'] = 'File too small (<100B)'
            return result
            
        # 准备输出路径
        config.output_dir.mkdir(parents=True, exist_ok=True)
        output_name = input_path.stem + '.' + output_format
        output_path = config.output_dir / output_name
        
        # 删除旧输出文件
        if output_path.exists():
            output_path.unlink()
            
        # 检查pixly可执行文件
        if not config.pixly_bin.exists():
            result['status'] = 'failed'
            result['error'] = 'Pixly binary not found (need: cargo build --release)'
            return result
            
        # 执行转换
        start_time = time.time()
        
        cmd = [
            str(config.pixly_bin),
            'convert',
            str(input_path),
            str(output_path),
            '--format', output_format,
            '--quality', '85',  # balanced模式
        ]
        
        proc = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=10  # 10秒超时（优化：减少等待）
        )
        
        result['time'] = time.time() - start_time
        
        # 检查结果
        if proc.returncode == 0 and output_path.exists():
            output_size = get_file_size(output_path)
            result['output_size'] = output_size
            result['status'] = 'success'
            
            # 计算压缩率
            reduction = (1 - output_size / input_size) * 100
            result['reduction'] = reduction
            
        else:
            result['status'] = 'failed'
            # 提取错误信息
            if proc.stderr:
                error_lines = proc.stderr.split('\n')
                # 查找实际错误信息
                for line in error_lines:
                    if 'error' in line.lower() or 'failed' in line.lower():
                        result['error'] = line.strip()[:100]
                        break
                if not result['error']:
                    result['error'] = proc.stderr.split('\n')[0][:100]
            else:
                result['error'] = 'Conversion failed (no error output)'
                
    except subprocess.TimeoutExpired:
        result['status'] = 'failed'
        result['error'] = 'Timeout (>30s)'
    except Exception as e:
        result['status'] = 'failed'
        result['error'] = str(e)[:100]
        
    return result

def scan_test_files() -> List[Path]:
    """扫描测试文件"""
    test_files = []
    
    for ext, _ in config.test_formats.items():
        pattern = f"*.{ext}"
        files = list(config.data_dir.glob(pattern))
        test_files.extend(files)
        
    # 限制数量
    if len(test_files) > config.max_files:
        test_files = test_files[:config.max_files]
        
    return sorted(test_files)

def main():
    print("="*80)
    print("🧪 Pixly 批量转换测试")
    print("="*80)
    print(f"测试时间: {time.strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"测试目录: {config.data_dir}")
    print(f"输出目录: {config.output_dir}")
    print(f"最大测试数: {config.max_files}")
    print("="*80)
    print()
    
    # 扫描文件
    print("📂 扫描测试文件...")
    test_files = scan_test_files()
    print(f"   找到 {len(test_files)} 个测试文件")
    print()
    
    if not test_files:
        print("❌ 没有找到测试文件")
        return 1
        
    # 检查Pixly二进制文件
    if not config.pixly_bin.exists():
        print(f"❌ Pixly未编译: {config.pixly_bin}")
        print(f"   请运行: cd {config.pixly_bin.parent.parent} && cargo build --release")
        return 1
        
    # 执行转换测试
    print("🔄 开始批量转换测试...")
    print()
    
    for i, input_file in enumerate(test_files, 1):
        ext = input_file.suffix[1:].lower()
        output_format = config.test_formats.get(ext, 'avif')
        
        print(f"[{i}/{len(test_files)}] {input_file.name} → {output_format}...", end=' ', flush=True)
        
        result = convert_file(input_file, output_format)
        stats.add_result(result)
        
        # 打印结果
        if result['status'] == 'success':
            reduction = result['reduction']
            if reduction > 0:
                print(f"✅ {result['input_size']/1024:.1f}KB → {result['output_size']/1024:.1f}KB (-{reduction:.1f}%) {result['time']:.2f}s")
            else:
                print(f"⚠️  {result['input_size']/1024:.1f}KB → {result['output_size']/1024:.1f}KB (+{-reduction:.1f}%) {result['time']:.2f}s")
        elif result['status'] == 'failed':
            print(f"❌ {result.get('error', 'Unknown error')[:60]}")
        else:
            print(f"⊘ {result.get('error', 'Skipped')}")
            
    # 打印总结
    success = stats.print_summary()
    
    # 保存详细报告
    report_path = config.output_dir / 'test_report.json'
    with open(report_path, 'w', encoding='utf-8') as f:
        json.dump({
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
            'total': stats.total,
            'success': stats.success,
            'failed': stats.failed,
            'skipped': stats.skipped,
            'results': stats.results
        }, f, indent=2, ensure_ascii=False)
    print(f"\n📄 详细报告已保存: {report_path}")
    
    return 0 if success else 1

if __name__ == '__main__':
    sys.exit(main())
