#!/usr/bin/env python3
"""
Phase 4测试：格式选择器验证

快速测试格式选择器的推荐是否合理
"""

import subprocess
import json
from pathlib import Path

def test_format_recommendation(input_file: str, user_format: str = None):
    """Test format recommendation"""
    print(f"\n📸 Testing: {input_file}")
    print(f"   User specified: {user_format or 'Auto'}")
    
    # Call Rust CLI analyze command
    cmd = ['./target/release/pixly-converter', 'analyze', input_file, '--json']
    if user_format:
        cmd.extend(['--format', user_format])
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            # Parse JSON output
            data = json.loads(result.stdout)
            print(f"   ✅ Recommended format: {data.get('recommended_format', 'N/A')}")
            print(f"   📊 Confidence: {data.get('confidence', 0):.2%}")
            print(f"   💡 Reason: {data.get('reason', 'N/A')}")
            return data
        else:
            print(f"   ❌ Analysis failed: {result.stderr}")
            return None
    except Exception as e:
        print(f"   ❌ Error: {e}")
        return None

def main():
    print("=" * 60)
    print("🧪 Phase 4: Format Selector Test")
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
            print(f"\n⚠️  File not found: {input_file}")
    
    print("\n" + "=" * 60)
    print("📊 Test Summary")
    print("=" * 60)
    print(f"   Total tests: {len(test_cases)}")
    print(f"   Success: {sum(1 for r in results if r is not None)}")
    print(f"   Failed: {sum(1 for r in results if r is None)}")

if __name__ == '__main__':
    main()
