#!/bin/bash
# 🏥 Pixly项目健康检查脚本
# 快速验证项目状态

set -e

echo "🏥 Pixly项目健康检查"
echo "======================================"
echo ""

PASS=0
FAIL=0

# 检查函数
check() {
    local name="$1"
    local cmd="$2"
    
    echo -n "🔍 检查 $name ... "
    if eval "$cmd" > /dev/null 2>&1; then
        echo "✅"
        PASS=$((PASS + 1))
    else
        echo "❌"
        FAIL=$((FAIL + 1))
    fi
}

# 1. Rust编译
check "Rust编译" "cargo build --release"

# 2. CLI可执行
check "CLI可执行" "test -x ./target/release/pixly-converter"

# 3. Python依赖
check "Python numpy" "python3 -c 'import numpy'"
check "Python torch" "python3 -c 'import torch'"

# 4. 关键文件
check "README.md" "test -f README.md"
check "LICENSE" "test -f LICENSE"
check "Cargo.toml" "test -f Cargo.toml"

# 5. ML模型
check "经验缓冲" "test -f models/ppo/experience_buffer.json"
check "训练历史" "test -f models/ppo/batch_training_history.json"

# 6. 测试脚本
check "视频测试脚本" "test -x scripts/test_video_complete.sh"
check "在线学习测试" "test -x scripts/test_online_learning.sh"
check "ML监控脚本" "test -x scripts/ml_monitor.py"

# 7. 文档
check "格式支持文档" "test -f docs/FORMAT_SUPPORT.md"
check "工作总结" "test -f docs/WORK_SUMMARY_20251119.md"

echo ""
echo "======================================"
echo "📊 检查结果"
echo "======================================"
echo "✅ 通过: $PASS"
echo "❌ 失败: $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 项目健康状态: 优秀"
    exit 0
else
    echo "⚠️  项目健康状态: 需要关注"
    exit 1
fi
