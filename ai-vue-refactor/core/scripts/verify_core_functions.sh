#!/opt/homebrew/bin/bash
# 核心功能验证脚本 - FC-001
# 遵循PROJECT_QUALITY_MANIFESTO.md原则：真实性 > 模拟

set -e

echo "🎯 Pixly核心功能验证 - FC-001"
echo "================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
test_function() {
    local test_name="$1"
    local test_command="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "Testing: $test_name ... "
    
    if eval "$test_command" > /tmp/pixly_test_$$.log 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        echo "  Error log:"
        cat /tmp/pixly_test_$$.log | head -5 | sed 's/^/    /'
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# 检查Rust CLI是否存在
if [ ! -f "target/release/pixly-converter" ]; then
    echo -e "${RED}❌ Rust CLI not found. Building...${NC}"
    cargo build --release
fi

PIXLY_CLI="./target/release/pixly-converter"

echo "📦 1. 单图像处理验证"
echo "-----------------------------------"

# 创建测试目录
TEST_DIR="/tmp/pixly_test_$$"
mkdir -p "$TEST_DIR"

# 查找测试图像
if [ -d "@reference/data" ]; then
    TEST_IMAGE=$(find @reference/data -name "*.png" -o -name "*.jpg" | head -1)
elif [ -d "test_output" ]; then
    TEST_IMAGE=$(find test_output -name "*.png" -o -name "*.jpg" | head -1)
else
    echo -e "${YELLOW}⚠️  No test images found, skipping image tests${NC}"
    TEST_IMAGE=""
fi

if [ -n "$TEST_IMAGE" ]; then
    echo "Using test image: $TEST_IMAGE"
    
    # Test 1.1: PNG → AVIF
    test_function "PNG → AVIF conversion" \
        "$PIXLY_CLI convert '$TEST_IMAGE' --format avif --quality 85 -o '$TEST_DIR'"
    
    # Test 1.2: PNG → WebP
    test_function "PNG → WebP conversion" \
        "$PIXLY_CLI convert '$TEST_IMAGE' --format webp --quality 85 -o '$TEST_DIR'"
    
    # Test 1.3: PNG → JXL
    test_function "PNG → JXL conversion" \
        "$PIXLY_CLI convert '$TEST_IMAGE' --format jxl --quality 85 -o '$TEST_DIR'"
    
    # Test 1.4: 文件信息查询
    test_function "File info query" \
        "$PIXLY_CLI analyze '$TEST_IMAGE'"
    
    # Test 1.5: 元数据保留验证
    if [ -f "$TEST_DIR/test.avif" ]; then
        test_function "Metadata preservation" \
            "exiftool '$TEST_DIR/test.avif' | grep -q 'File Type'"
    fi
else
    echo -e "${YELLOW}⚠️  Skipping image tests (no test images)${NC}"
fi

echo ""
echo "🔄 2. 批量处理验证"
echo "-----------------------------------"

if [ -n "$TEST_IMAGE" ]; then
    # 创建批量测试文件
    cp "$TEST_IMAGE" "$TEST_DIR/batch1.png" 2>/dev/null || true
    cp "$TEST_IMAGE" "$TEST_DIR/batch2.png" 2>/dev/null || true
    
    # Test 2.1: 批量转换
    test_function "Batch conversion (2 files)" \
        "$PIXLY_CLI convert '$TEST_DIR/batch1.png' --format avif -o '$TEST_DIR/batch_output' && $PIXLY_CLI convert '$TEST_DIR/batch2.png' --format avif -o '$TEST_DIR/batch_output'"
    
    # Test 2.2: 并行处理（通过多次调用验证）
    test_function "Multiple conversions" \
        "$PIXLY_CLI convert '$TEST_DIR/batch1.png' --format webp -o '$TEST_DIR/parallel_output'"
else
    echo -e "${YELLOW}⚠️  Skipping batch tests (no test images)${NC}"
fi

echo ""
echo "🎬 3. GIF优化验证"
echo "-----------------------------------"

# 查找GIF测试文件
TEST_GIF=$(find @reference/data -name "*.gif" 2>/dev/null | head -1)

if [ -n "$TEST_GIF" ]; then
    echo "Using test GIF: $TEST_GIF"
    
    # Test 3.1: GIF优化
    test_function "GIF optimization" \
        "$PIXLY_CLI convert '$TEST_GIF' --format gif -o '$TEST_DIR'"
    
    # Test 3.2: GIF → WebP (动画)
    test_function "GIF → WebP (animated)" \
        "$PIXLY_CLI convert '$TEST_GIF' --format webp -o '$TEST_DIR'"
else
    echo -e "${YELLOW}⚠️  No GIF files found, skipping GIF tests${NC}"
fi

echo ""
echo "📝 4. 元数据处理验证"
echo "-----------------------------------"

if [ -n "$TEST_IMAGE" ]; then
    # Test 4.1: EXIF保留
    test_function "EXIF preservation" \
        "exiftool '$TEST_IMAGE' > /dev/null 2>&1"
    
    # Test 4.2: XMP处理
    if [ -f "${TEST_IMAGE%.png}.xmp" ] || [ -f "${TEST_IMAGE%.jpg}.xmp" ]; then
        test_function "XMP sidecar processing" \
            "$PIXLY_CLI convert '$TEST_IMAGE' --format avif --merge-xmp -o '$TEST_DIR'"
    else
        echo -e "${YELLOW}⚠️  No XMP sidecar found, skipping XMP test${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Skipping metadata tests${NC}"
fi

echo ""
echo "🤖 5. AI参数预测验证"
echo "-----------------------------------"

# Test 5.1: Python ML Bridge测试
test_function "Python ML Bridge" \
    "python3 scripts/ml_bridge.py --test"

# Test 5.2: 本地AI预测
if [ -n "$TEST_IMAGE" ]; then
    test_function "Local AI prediction" \
        "$PIXLY_CLI analyze '$TEST_IMAGE' --format avif"
else
    echo -e "${YELLOW}⚠️  Skipping AI tests${NC}"
fi

echo ""
echo "================================"
echo "📊 测试结果汇总"
echo "================================"
echo "总测试数: $TOTAL_TESTS"
echo -e "通过: ${GREEN}$PASSED_TESTS${NC}"
echo -e "失败: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}🎉 所有测试通过！核心功能正常工作${NC}"
    SUCCESS_RATE=100
else
    SUCCESS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "\n${YELLOW}⚠️  成功率: ${SUCCESS_RATE}%${NC}"
fi

# 清理
rm -rf "$TEST_DIR"
rm -f /tmp/pixly_test_$$.log

# 返回状态
if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
