#!/usr/bin/env python3
"""
Phase 4测试：格式选择器验证

快速测试格式选择器的推荐是否合理
"""

import subprocess
import json
from pathlib import Path

def test_format_recommendation(input_file: str, user_format: str = None):
    """测试格式推荐"""
    print(f"\n📸 测试: {input_file}")
    print(f"   用户指定: {user_format or '自动'}")
    
    # 调用Rust CLI的analyze命令
    cmd = ['./target/release/pixly-converter', 'analyze', input_file, '--json']
    if user_format:
        cmd.extend(['--format', user_format])
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            # 解析JSON输出
            data = json.loads(result.stdout)
            print(f"   ✅ 推荐格式: {data.get('recommended_format', 'N/A')}")
            print(f"   📊 置信度: {data.get('confidence', 0):.2%}")
            print(f"   💡 原因: {data.get('reason', 'N/A')}")
            return data
        else:
            print(f"   ❌ 分析失败: {result.stderr}")
            return None
    except Exception as e:
        print(f"   ❌ 错误: {e}")
        return None

def main():
    print("=" * 60)
    print("🧪 Phase 4: 格式选择器测试")
    print("=" * 60)
    
    # 测试用例
    test_cases = [
        ("data/training_samples/表情包 (24).jpeg", None),  # JPEG自动
        ("data/training_samples/表情包 (24).jpeg", "webp"),  # JPEG→WebP（风险）
        ("data/training_samples/表情包 (24).jpeg", "jxl"),  # JPEG→JXL（推荐）
    ]
    
    results = []
    for input_file, user_format in test_cases:
        if Path(input_file).exists():
            result = test_format_recommendation(input_file, user_format)
            results.append(result)
        else:
            print(f"\n⚠️  文件不存在: {input_file}")
    
    print("\n" + "=" * 60)
    print("📊 测试总结")
    print("=" * 60)
    print(f"   测试数量: {len(test_cases)}")
    print(f"   成功: {sum(1 for r in results if r is not None)}")
    print(f"   失败: {sum(1 for r in results if r is None)}")

if __name__ == '__main__':
    main()
