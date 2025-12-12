#!/opt/homebrew/bin/bash
# 🎬 完整视频功能测试脚本
# 遵循 PROJECT_QUALITY_MANIFESTO.md - 真实性原则
# 测试所有声称的视频功能

set -e

echo "🎬 Pixly Video功能完整测试"
echo "======================================"
echo ""

# 检查CLI是否存在
if [ ! -f "./target/release/pixly-converter" ]; then
    echo "❌ pixly-converter not found. Please run: cargo build --release"
    exit 1
fi

CLI="./target/release/pixly-converter"

# 创建测试目录
TEST_DIR="test_output/video_tests"
mkdir -p "$TEST_DIR"

# 准备测试文件
if [ ! -f "test_output/test_anim.gif" ]; then
    echo "❌ test_anim.gif not found. Please create test files first."
    exit 1
fi

TEST_INPUT="test_output/test_anim.gif"

echo "📋 测试计划:"
echo "  1. 基础H.265转换"
echo "  2. H.264转换"
echo "  3. AI智能模式"
echo "  4. Two-Pass编码"
echo "  5. 不同容器格式"
echo "  6. GPU加速"
echo ""

# 测试计数器
TOTAL=0
PASSED=0
FAILED=0

# 测试函数
test_video() {
    local name="$1"
    local output="$2"
    shift 2
    local args="$@"
    
    TOTAL=$((TOTAL + 1))
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🧪 测试 $TOTAL: $name"
    echo "   命令: video $TEST_INPUT $output $args"
    echo ""
    
    # 删除旧文件
    rm -f "$output"
    
    # 执行转换
    if $CLI video "$TEST_INPUT" "$output" $args 2>&1 | tee "$TEST_DIR/test_${TOTAL}.log"; then
        # 检查输出文件
        if [ -f "$output" ] && [ -s "$output" ]; then
            local size=$(du -h "$output" | cut -f1)
            echo ""
            echo "✅ 测试通过: $name"
            echo "   输出文件: $output ($size)"
            PASSED=$((PASSED + 1))
        else
            echo ""
            echo "❌ 测试失败: $name - 输出文件不存在或为空"
            FAILED=$((FAILED + 1))
        fi
    else
        echo ""
        echo "❌ 测试失败: $name - 命令执行失败"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

# 1. 基础H.265转换
test_video "基础H.265转换" \
    "$TEST_DIR/test_h265.mp4" \
    --codec h265 --crf 23 --preset medium

# 2. H.264转换
test_video "H.264转换" \
    "$TEST_DIR/test_h264.mp4" \
    --codec h264 --crf 23 --preset fast

# 3. AI智能模式 - Quality
test_video "AI智能模式 (Quality)" \
    "$TEST_DIR/test_ai_quality.mp4" \
    --ai --optimize-mode quality

# 4. AI智能模式 - Size
test_video "AI智能模式 (Size)" \
    "$TEST_DIR/test_ai_size.mp4" \
    --ai --optimize-mode size

# 5. AI智能模式 - Balanced
test_video "AI智能模式 (Balanced)" \
    "$TEST_DIR/test_ai_balanced.mp4" \
    --ai --optimize-mode balanced

# 6. Two-Pass编码
test_video "Two-Pass编码" \
    "$TEST_DIR/test_twopass.mp4" \
    --codec h265 --crf 20 --two-pass

# 7. WebM容器
test_video "WebM容器 (VP9)" \
    "$TEST_DIR/test_vp9.webm" \
    --codec vp9 --container webm --crf 30

# 8. MOV容器
test_video "MOV容器" \
    "$TEST_DIR/test_h265.mov" \
    --codec h265 --container mov --crf 23

# 9. GPU加速 (默认开启)
test_video "GPU加速" \
    "$TEST_DIR/test_gpu.mp4" \
    --codec h265 --gpu --crf 23

# 10. 高质量编码
test_video "高质量编码 (CRF 18)" \
    "$TEST_DIR/test_hq.mp4" \
    --codec h265 --crf 18 --preset slow

# 总结
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 测试总结"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "总测试数: $TOTAL"
echo "✅ 通过: $PASSED"
echo "❌ 失败: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "🎉 所有测试通过！"
    echo ""
    echo "📁 输出文件位置: $TEST_DIR/"
    ls -lh "$TEST_DIR"/*.mp4 "$TEST_DIR"/*.webm "$TEST_DIR"/*.mov 2>/dev/null || true
    exit 0
else
    echo "⚠️  有 $FAILED 个测试失败"
    echo "📋 查看日志: $TEST_DIR/test_*.log"
    exit 1
fi
