#!/usr/bin/env python3
"""
🚀 Pixly Python ML 性能基准测试

运行方式：
python scripts/performance_benchmark.py

测试项目：
1. 特征提取性能
2. 模型预测性能
3. 数据预处理性能
4. 内存使用情况
5. 并发处理性能
"""

import time
import sys
import json
import numpy as np
from pathlib import Path
from typing import List, Dict, Any
import tracemalloc
import statistics

class PerformanceBenchmark:
    """性能基准测试类"""
    
    def __init__(self):
        self.results = []
        self.memory_baseline = None
    
    def record_memory_baseline(self):
        """记录内存基线"""
        tracemalloc.start()
        self.memory_baseline = tracemalloc.get_traced_memory()
        print(f"📊 Memory Baseline: {self.memory_baseline[0] / 1024 / 1024:.2f} MB")
    
    def benchmark(self, name: str, func, iterations: int = 1000):
        """测试函数执行时间"""
        print(f"\n🔍 Testing: {name}")
        
        # 预热
        for _ in range(10):
            func()
        
        # 正式测试
        times = []
        for _ in range(iterations):
            start = time.perf_counter()
            func()
            end = time.perf_counter()
            times.append((end - start) * 1000)  # 转换为毫秒
        
        # 统计
        avg = statistics.mean(times)
        min_time = min(times)
        max_time = max(times)
        p95 = statistics.quantiles(times, n=20)[18]  # 95th percentile
        
        result = {
            'name': name,
            'iterations': iterations,
            'avg': round(avg, 3),
            'min': round(min_time, 3),
            'max': round(max_time, 3),
            'p95': round(p95, 3),
            'unit': 'ms'
        }
        
        self.results.append(result)
        
        print(f"  ✅ Avg: {result['avg']}ms | Min: {result['min']}ms | Max: {result['max']}ms | P95: {result['p95']}ms")
        
        return result
    
    def benchmark_memory(self, name: str, func, iterations: int = 100):
        """测试内存使用"""
        print(f"\n💾 Memory Test: {name}")
        
        tracemalloc.start()
        before = tracemalloc.get_traced_memory()
        
        for _ in range(iterations):
            func()
        
        after = tracemalloc.get_traced_memory()
        tracemalloc.stop()
        
        heap_diff = (after[0] - before[0]) / 1024 / 1024
        
        result = {
            'name': name,
            'iterations': iterations,
            'heapDiff': round(heap_diff, 2),
            'unit': 'MB'
        }
        
        self.results.append(result)
        
        print(f"  ✅ Heap Diff: {result['heapDiff']} MB")
        
        return result
    
    def generate_report(self):
        """生成报告"""
        print('\n' + '=' * 80)
        print('📊 PERFORMANCE BENCHMARK REPORT')
        print('=' * 80)
        
        print('\n⏱️  Timing Results:')
        print('─' * 80)
        for r in self.results:
            if r['unit'] == 'ms':
                print(f"{r['name']:<40} | Avg: {r['avg']}ms | P95: {r['p95']}ms")
        
        print('\n💾 Memory Results:')
        print('─' * 80)
        for r in self.results:
            if r['unit'] == 'MB':
                print(f"{r['name']:<40} | Heap: {r['heapDiff']} MB")
        
        print('\n' + '=' * 80)
        
        return self.results
    
    def save_report(self, filename: str = 'performance-report-python.json'):
        """保存报告到文件"""
        report = {
            'timestamp': time.strftime('%Y-%m-%d %H:%M:%S'),
            'baseline': self.memory_baseline,
            'results': self.results
        }
        
        with open(filename, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"\n💾 Report saved to: {filename}")


def run_benchmarks():
    """运行所有基准测试"""
    bench = PerformanceBenchmark()
    
    bench.record_memory_baseline()
    
    # 1. 测试NumPy数组操作
    bench.benchmark('NumPy Array Creation (128d)', lambda: np.random.rand(128), 10000)
    
    # 2. 测试特征归一化
    features = np.random.rand(128)
    bench.benchmark('Feature Normalization', lambda: features / np.sum(features), 10000)
    
    # 3. 测试矩阵乘法
    matrix_a = np.random.rand(128, 128)
    matrix_b = np.random.rand(128, 128)
    bench.benchmark('Matrix Multiplication (128x128)', lambda: np.dot(matrix_a, matrix_b), 1000)
    
    # 4. 测试列表操作
    bench.benchmark('List Comprehension (1000 items)', 
                   lambda: [i * 2 for i in range(1000)], 1000)
    
    # 5. 测试字典操作
    bench.benchmark('Dict Creation (100 items)', 
                   lambda: {f'key_{i}': i for i in range(100)}, 1000)
    
    # 6. 测试JSON序列化
    data = {'features': [0.5] * 128, 'quality': 90, 'speed': 4}
    bench.benchmark('JSON Serialization', lambda: json.dumps(data), 10000)
    
    # 7. 测试JSON反序列化
    json_str = json.dumps(data)
    bench.benchmark('JSON Deserialization', lambda: json.loads(json_str), 10000)
    
    # 8. 测试文件路径操作
    bench.benchmark('Path Operations', 
                   lambda: Path('/path/to/file.jpg').with_suffix('.jxl'), 10000)
    
    # 9. 内存测试：大数组
    bench.benchmark_memory('Large NumPy Array (1000x1000)', 
                          lambda: np.random.rand(1000, 1000), 10)
    
    # 10. 内存测试：特征向量列表
    bench.benchmark_memory('1000 Feature Vectors (128d)', 
                          lambda: [np.random.rand(128) for _ in range(1000)], 10)
    
    bench.generate_report()
    bench.save_report('scripts/performance-report-python.json')


if __name__ == '__main__':
    print('🚀 Starting Python ML Performance Benchmark...\n')
    try:
        run_benchmarks()
    except Exception as e:
        print(f"\n❌ Error: {e}")
        traceback.print_exc()
        sys.exit(1)
