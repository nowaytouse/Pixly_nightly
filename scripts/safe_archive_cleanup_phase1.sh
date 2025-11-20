#!/bin/bash
# 归档安全清理脚本 - 第一阶段
# 日期: 2025-11-16
# 目的: 安全删除已提取价值的归档文件

set -e

echo "🗑️  Pixly 归档安全清理 - 第一阶段"
echo "=================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 工作目录
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_ROOT"

echo "📍 工作目录: $WORKSPACE_ROOT"
echo ""

# 步骤1: 验证新代码存在
echo "✅ 步骤1: 验证新代码已创建"
echo "----------------------------"

NEW_FILES=(
    "src/formats.rs"
    "src/quality_predictor.rs"
    "src/preprocessing.rs"
    "docs/ARCHIVE_EXTRACTION_PHASE1_REPORT.md"
)

for file in "${NEW_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC} $file 存在"
    else
        echo -e "${RED}✗${NC} $file 不存在"
        echo -e "${RED}错误: 新代码未完全创建，中止删除${NC}"
        exit 1
    fi
done

echo ""

# 步骤2: 运行测试验证
echo "🧪 步骤2: 运行测试验证"
echo "----------------------------"

if cargo test --lib --quiet 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓${NC} 所有测试通过"
else
    echo -e "${RED}✗${NC} 测试失败"
    echo -e "${RED}错误: 测试未通过，中止删除${NC}"
    exit 1
fi

echo ""

# 步骤3: 创建备份
echo "💾 步骤3: 创建归档备份"
echo "----------------------------"

BACKUP_DIR="$HOME/Desktop"
BACKUP_FILE="$BACKUP_DIR/pixly_archive_phase1_backup_$(date +%Y%m%d_%H%M%S).tar.gz"

ARCHIVE_FILES=(
    "@archive/rust_v2_clean/src/formats.rs"
    "@archive/rust_broken/src/quality_predictor.rs"
    "@archive/rust_broken/src/preprocessing/mod.rs"
)

# 检查文件是否存在
MISSING_FILES=0
for file in "${ARCHIVE_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠${NC}  $file 不存在（可能已删除）"
        MISSING_FILES=$((MISSING_FILES + 1))
    fi
done

if [ $MISSING_FILES -eq ${#ARCHIVE_FILES[@]} ]; then
    echo -e "${YELLOW}⚠${NC}  所有归档文件已不存在，跳过备份"
else
    echo "创建备份: $BACKUP_FILE"
    tar -czf "$BACKUP_FILE" "${ARCHIVE_FILES[@]}" 2>/dev/null || true
    
    if [ -f "$BACKUP_FILE" ]; then
        BACKUP_SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
        echo -e "${GREEN}✓${NC} 备份创建成功 (大小: $BACKUP_SIZE)"
    else
        echo -e "${YELLOW}⚠${NC}  备份创建失败（文件可能已删除）"
    fi
fi

echo ""

# 步骤4: 删除归档文件
echo "🗑️  步骤4: 删除已提取的归档文件"
echo "----------------------------"

DELETED_COUNT=0
for file in "${ARCHIVE_FILES[@]}"; do
    if [ -f "$file" ]; then
        rm "$file"
        echo -e "${GREEN}✓${NC} 已删除: $file"
        DELETED_COUNT=$((DELETED_COUNT + 1))
    else
        echo -e "${YELLOW}⚠${NC}  跳过: $file (不存在)"
    fi
done

echo ""
echo "删除统计: $DELETED_COUNT / ${#ARCHIVE_FILES[@]} 个文件"

# 步骤5: 清理空目录
echo ""
echo "🧹 步骤5: 清理空目录"
echo "----------------------------"

EMPTY_DIRS=(
    "@archive/rust_broken/src/preprocessing"
)

for dir in "${EMPTY_DIRS[@]}"; do
    if [ -d "$dir" ] && [ -z "$(ls -A "$dir")" ]; then
        rmdir "$dir"
        echo -e "${GREEN}✓${NC} 已删除空目录: $dir"
    fi
done

echo ""

# 步骤6: 最终验证
echo "✅ 步骤6: 最终验证"
echo "----------------------------"

if cargo check --lib 2>&1 | grep -q "Finished"; then
    echo -e "${GREEN}✓${NC} 编译检查通过"
else
    echo -e "${RED}✗${NC} 编译检查失败"
    exit 1
fi

if cargo test --lib --quiet 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓${NC} 测试验证通过"
else
    echo -e "${RED}✗${NC} 测试验证失败"
    exit 1
fi

echo ""
echo "=================================="
echo -e "${GREEN}🎉 第一阶段清理完成！${NC}"
echo "=================================="
echo ""
echo "📊 清理统计:"
echo "  - 删除文件: $DELETED_COUNT 个"
echo "  - 新增模块: ${#NEW_FILES[@]} 个"
echo "  - 备份位置: $BACKUP_FILE"
echo ""
echo "📝 下一步:"
echo "  1. 查看提取报告: docs/ARCHIVE_EXTRACTION_PHASE1_REPORT.md"
echo "  2. 提交更改: git add -A && git commit -m '第一阶段归档提取完成'"
echo "  3. 继续第二阶段提取"
echo ""
