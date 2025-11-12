#!/bin/bash
# ========================================
# 🔄 单次同步插件到Eagle
# ========================================
# 快速同步，不监听变化
# 使用: ./sync-once.sh
# ========================================

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

SOURCE_DIR="/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/plugin"
TARGET_DIR="$HOME/Library/Application Support/Eagle/plugins/pixly"

echo -e "${YELLOW}🔄 Syncing to Eagle...${NC}"

# 删除符号链接（如果存在）
if [ -L "$TARGET_DIR" ]; then
    rm "$TARGET_DIR"
fi

# 同步
rsync -a --delete \
    --exclude '.DS_Store' \
    --exclude '.git' \
    --exclude 'node_modules' \
    "$SOURCE_DIR/" "$TARGET_DIR/"

echo -e "${GREEN}✅ Sync completed!${NC}"
echo -e "${YELLOW}💡 Restart Eagle to load changes${NC}"
