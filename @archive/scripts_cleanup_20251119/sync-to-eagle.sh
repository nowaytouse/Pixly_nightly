#!/bin/bash
# ========================================
# 🔄 自动同步插件到Eagle
# ========================================
# 
# 问题：符号链接导致Eagle文件选择器卡死
# 解决：使用rsync实时同步，避免符号链接
#
# 使用方法：
# 1. chmod +x sync-to-eagle.sh
# 2. ./sync-to-eagle.sh
#
# 功能：
# - 监听core/plugin目录变化
# - 自动同步到Eagle插件目录
# - 保持文件时间戳
# - 排除不必要的文件
#
# ========================================

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 配置
SOURCE_DIR="/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/plugin"
TARGET_DIR="$HOME/Library/Application Support/Eagle/plugins/pixly"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}🔄 PIXLY Auto-Sync to Eagle${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${YELLOW}📁 Source:${NC} $SOURCE_DIR"
echo -e "${YELLOW}📁 Target:${NC} $TARGET_DIR"
echo ""

# 检查源目录
if [ ! -d "$SOURCE_DIR" ]; then
    echo -e "${RED}❌ Source directory not found!${NC}"
    exit 1
fi

# 检查是否安装fswatch
if ! command -v fswatch &> /dev/null; then
    echo -e "${YELLOW}⚠️  fswatch not found, installing via Homebrew...${NC}"
    brew install fswatch
fi

# 初始同步函数
sync_now() {
    echo -e "${BLUE}🔄 Syncing...${NC}"
    
    # 删除目标目录（如果是符号链接）
    if [ -L "$TARGET_DIR" ]; then
        echo -e "${YELLOW}🗑️  Removing symlink...${NC}"
        rm "$TARGET_DIR"
    fi
    
    # 使用rsync同步
    # -a: archive mode (preserve permissions, timestamps, etc.)
    # -v: verbose
    # --delete: delete files in target that don't exist in source
    # --exclude: exclude unnecessary files
    rsync -av --delete \
        --exclude '.DS_Store' \
        --exclude '.git' \
        --exclude 'node_modules' \
        --exclude '*.log' \
        --exclude '.cache' \
        "$SOURCE_DIR/" "$TARGET_DIR/"
    
    local EXIT_CODE=$?
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo -e "${GREEN}✅ Sync completed at $(date '+%H:%M:%S')${NC}"
        echo ""
    else
        echo -e "${RED}❌ Sync failed!${NC}"
        return $EXIT_CODE
    fi
}

# 首次同步
echo -e "${YELLOW}📦 Initial sync...${NC}"
sync_now

echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}👁️  Watching for changes...${NC}"
echo -e "${YELLOW}💡 Tip: Press Ctrl+C to stop${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 监听文件变化并自动同步
fswatch -o "$SOURCE_DIR" | while read f; do
    echo -e "${YELLOW}📝 Changes detected, syncing...${NC}"
    sync_now
done
