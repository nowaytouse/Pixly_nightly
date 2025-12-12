#!/opt/homebrew/bin/bash
# 测试不同RUST_LOG级别的日志输出
# 
# 用法: ./test_logging.sh

echo "======================================"
echo "Pixly日志系统测试"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试文件
TEST_FILE="test_input.txt"
echo "创建测试文件..."
echo "test content" > "$TEST_FILE"

echo ""
echo "${BLUE}1. 测试 RUST_LOG=error (仅错误)${NC}"
echo "--------------------------------------"
RUST_LOG=error cargo run --release -- convert "$TEST_FILE" --format webp 2>&1 | head -20
echo ""

echo "${BLUE}2. 测试 RUST_LOG=warn (警告+错误)${NC}"
echo "--------------------------------------"
RUST_LOG=warn cargo run --release -- convert "$TEST_FILE" --format webp 2>&1 | head -20
echo ""

echo "${BLUE}3. 测试 RUST_LOG=info (信息+警告+错误)${NC}"
echo "--------------------------------------"
RUST_LOG=info cargo run --release -- convert "$TEST_FILE" --format webp 2>&1 | head -20
echo ""

echo "${BLUE}4. 测试 RUST_LOG=debug (全部日志)${NC}"
echo "--------------------------------------"
RUST_LOG=debug cargo run --release -- convert "$TEST_FILE" --format webp 2>&1 | head -30
echo ""

echo "${BLUE}5. 测试模块特定日志 (pixly_kernel::ai=debug)${NC}"
echo "--------------------------------------"
RUST_LOG=pixly_kernel::ai=debug,info cargo run --release -- convert "$TEST_FILE" --format webp --ai 2>&1 | head -30
echo ""

# 清理
rm -f "$TEST_FILE"
echo "${GREEN}✅ 测试完成！${NC}"
echo ""
echo "总结:"
echo "- error: 仅显示关键错误"
echo "- warn:  显示警告和错误"
echo "- info:  显示常规信息、警告、错误(推荐)"
echo "- debug: 显示所有调试信息(详细模式)"
