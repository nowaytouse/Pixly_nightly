#!/bin/bash

# 🔥 PIXLY AI Service Stop Script

set -e

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PID_FILE="$PROJECT_ROOT/.pixly-ai.pid"

echo -e "${GREEN}🛑 PIXLY AI Service Stopper${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 检查PID文件
if [ ! -f "$PID_FILE" ]; then
    echo -e "${YELLOW}⚠️  No PID file found. Service may not be running.${NC}"
    
    # 尝试查找进程
    PIXLY_PIDS=$(ps aux | grep "[p]ixly-ai\|go run.*main.go" | awk '{print $2}')
    if [ -n "$PIXLY_PIDS" ]; then
        echo -e "${YELLOW}🔍 Found potential AI service processes:${NC}"
        echo "$PIXLY_PIDS"
        echo ""
        echo -e "${YELLOW}Kill these processes? (y/n)${NC}"
        read -r answer
        if [ "$answer" = "y" ]; then
            echo "$PIXLY_PIDS" | xargs kill
            echo -e "${GREEN}✅ Processes killed${NC}"
        fi
    else
        echo -e "${GREEN}✅ No AI service process found${NC}"
    fi
    exit 0
fi

# 读取PID
SERVICE_PID=$(cat "$PID_FILE")

# 检查进程是否存在
if ps -p $SERVICE_PID > /dev/null 2>&1; then
    echo -e "${YELLOW}🔄 Stopping AI service (PID: $SERVICE_PID)...${NC}"
    kill $SERVICE_PID
    
    # 等待进程结束
    for i in {1..5}; do
        if ! ps -p $SERVICE_PID > /dev/null 2>&1; then
            break
        fi
        sleep 1
    done
    
    # 强制杀死（如果还在运行）
    if ps -p $SERVICE_PID > /dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Process still running, forcing kill...${NC}"
        kill -9 $SERVICE_PID
    fi
    
    echo -e "${GREEN}✅ AI service stopped${NC}"
else
    echo -e "${YELLOW}⚠️  Process $SERVICE_PID not found (already stopped?)${NC}"
fi

# 清理PID文件
rm "$PID_FILE"
echo -e "${GREEN}🧹 Cleaned up PID file${NC}"
