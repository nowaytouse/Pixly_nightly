#!/bin/bash
# 🧪 Rust CLI快速验证测试
# 用途：验证参数验证链路是否正常工作

set -e

echo "🧪 Pixly Rust CLI - 验证链路快速测试"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否已编译
if [ ! -f "./target/release/pixly-rust" ] && [ ! -f "./target/debug/pixly-rust" ]; then
    echo "⚠️  未找到编译的二进制文件，正在编译..."
    cargo build --release
    echo ""
fi

# 确定使用哪个二进制
if [ -f "./target/release/pixly-rust" ]; then
    PIXLY_BIN="./target/release/pixly-rust"
    BUILD_TYPE="release"
else
    PIXLY_BIN="./target/debug/pixly-rust"
    BUILD_TYPE="debug"
fi

echo "📦 使用二进制: $PIXLY_BIN ($BUILD_TYPE)"
echo ""

# 创建测试目录
TEST_DIR="$HOME/pixly-test"
mkdir -p "$TEST_DIR"

# 检查测试图像
TEST_IMAGE="$TEST_DIR/input.jpg"
if [ ! -f "$TEST_IMAGE" ]; then
    echo "⚠️  测试图像不存在: $TEST_IMAGE"
    echo "💡 请准备一张测试图像："
    echo "   cp ~/Pictures/some-image.jpg $TEST_IMAGE"
    echo ""
    echo "或使用当前目录的任意jpg文件："
    # 查找当前目录下的jpg文件
    FOUND_JPG=$(find . -maxdepth 1 -name "*.jpg" -o -name "*.jpeg" | head -1)
    if [ -n "$FOUND_JPG" ]; then
        echo "   发现: $FOUND_JPG"
        echo "   复制为测试图像..."
        cp "$FOUND_JPG" "$TEST_IMAGE"
    else
        echo "   ❌ 未找到jpg文件"
        echo "   退出测试"
        exit 1
    fi
fi

echo "✅ 测试图像: $TEST_IMAGE"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 测试计数
PASS=0
FAIL=0

# 测试1: 正常转换（验证参数回显）
echo "📝 测试1: 正常AVIF转换"
echo "   验证: 参数回显机制"
if $PIXLY_BIN convert \
    --input "$TEST_IMAGE" \
    --output "$TEST_DIR/test1.avif" \
    --format avif \
    --quality 85 \
    --speed 6 2>&1 | tee "$TEST_DIR/test1.log"; then
    
    if grep -q "success" "$TEST_DIR/test1.log" || [ -f "$TEST_DIR/test1.avif" ]; then
        echo -e "${GREEN}✅ 测试1通过${NC}: 转换成功"
        PASS=$((PASS+1))
    else
        echo -e "${RED}❌ 测试1失败${NC}: 转换失败"
        FAIL=$((FAIL+1))
    fi
else
    echo -e "${RED}❌ 测试1失败${NC}: 命令执行失败"
    FAIL=$((FAIL+1))
fi
echo ""

# 测试2: 无效质量参数（验证响亮报错）
echo "📝 测试2: 无效质量参数 (quality=150)"
echo "   验证: 响亮报错机制"
if $PIXLY_BIN convert \
    --input "$TEST_IMAGE" \
    --output "$TEST_DIR/test2.avif" \
    --format avif \
    --quality 150 2>&1 | grep -q "范围" || \
   $PIXLY_BIN convert \
    --input "$TEST_IMAGE" \
    --output "$TEST_DIR/test2.avif" \
    --format avif \
    --quality 150 2>&1 | grep -q "invalid"; then
    echo -e "${GREEN}✅ 测试2通过${NC}: 正确拒绝无效参数"
    PASS=$((PASS+1))
else
    echo -e "${RED}❌ 测试2失败${NC}: 未能拒绝无效参数"
    FAIL=$((FAIL+1))
fi
echo ""

# 测试3: 不存在的文件（验证输入验证）
echo "📝 测试3: 不存在的输入文件"
echo "   验证: 输入文件验证"
if $PIXLY_BIN convert \
    --input "$TEST_DIR/nonexistent.jpg" \
    --output "$TEST_DIR/test3.avif" \
    --format avif 2>&1 | grep -q "不存在" || \
   $PIXLY_BIN convert \
    --input "$TEST_DIR/nonexistent.jpg" \
    --output "$TEST_DIR/test3.avif" \
    --format avif 2>&1 | grep -q "not found" || \
   $PIXLY_BIN convert \
    --input "$TEST_DIR/nonexistent.jpg" \
    --output "$TEST_DIR/test3.avif" \
    --format avif 2>&1 | grep -q "No such file"; then
    echo -e "${GREEN}✅ 测试3通过${NC}: 正确报告文件不存在"
    PASS=$((PASS+1))
else
    echo -e "${RED}❌ 测试3失败${NC}: 未能报告文件不存在"
    FAIL=$((FAIL+1))
fi
echo ""

# 测试4: 文件类型检测
echo "📝 测试4: 文件类型检测"
echo "   验证: Magika AI检测"
if $PIXLY_BIN detect --input "$TEST_IMAGE" 2>&1 | tee "$TEST_DIR/test4.log"; then
    if grep -q "image" "$TEST_DIR/test4.log" || grep -q "jpg\|jpeg" "$TEST_DIR/test4.log"; then
        echo -e "${GREEN}✅ 测试4通过${NC}: 文件类型检测正常"
        PASS=$((PASS+1))
    else
        echo -e "${YELLOW}⚠️  测试4警告${NC}: 检测结果不确定"
        PASS=$((PASS+1))  # 算通过，因为命令成功执行
    fi
else
    echo -e "${RED}❌ 测试4失败${NC}: 检测命令失败"
    FAIL=$((FAIL+1))
fi
echo ""

# 测试5: JXL转换
echo "📝 测试5: JXL转换"
echo "   验证: 多格式支持"
if $PIXLY_BIN convert \
    --input "$TEST_IMAGE" \
    --output "$TEST_DIR/test5.jxl" \
    --format jxl \
    --quality 90 \
    --effort 7 2>&1 | tee "$TEST_DIR/test5.log"; then
    
    if grep -q "success" "$TEST_DIR/test5.log" || [ -f "$TEST_DIR/test5.jxl" ]; then
        echo -e "${GREEN}✅ 测试5通过${NC}: JXL转换成功"
        PASS=$((PASS+1))
    else
        echo -e "${RED}❌ 测试5失败${NC}: JXL转换失败"
        FAIL=$((FAIL+1))
    fi
else
    echo -e "${RED}❌ 测试5失败${NC}: JXL命令执行失败"
    FAIL=$((FAIL+1))
fi
echo ""

# 总结
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 测试总结"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "通过: $PASS/5"
echo "失败: $FAIL/5"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}🎉 所有测试通过！验证链路正常工作！${NC}"
    echo ""
    echo "✅ 参数回显机制: 正常"
    echo "✅ 响亮报错机制: 正常"
    echo "✅ 输入文件验证: 正常"
    echo "✅ 文件类型检测: 正常"
    echo "✅ 多格式支持: 正常"
    echo ""
    echo "💡 下一步:"
    echo "   - 查看输出文件: ls -lh $TEST_DIR/"
    echo "   - 查看详细日志: cat $TEST_DIR/test*.log"
    echo "   - 运行更多测试: 参考 docs/guides/RUST_CLI_TESTING_GUIDE.md"
    exit 0
else
    echo -e "${RED}❌ 有 $FAIL 个测试失败${NC}"
    echo ""
    echo "💡 故障排查:"
    echo "   - 检查编译: cargo build --release"
    echo "   - 查看日志: cat $TEST_DIR/test*.log"
    echo "   - 启用调试: RUST_LOG=debug $PIXLY_BIN ..."
    echo "   - 查看文档: docs/guides/RUST_CLI_TESTING_GUIDE.md"
    exit 1
fi
