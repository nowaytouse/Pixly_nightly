#!/bin/bash

# 🔥 PIXLY AI Service Startup Script
# Purpose: Automatically start GO AI service for video processing

set -e

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 项目路径
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GO_DIR="$PROJECT_ROOT/core/go"
PID_FILE="$PROJECT_ROOT/.pixly-ai.pid"
LOG_FILE="$PROJECT_ROOT/pixly-ai.log"

echo -e "${GREEN}🚀 PIXLY AI Service Manager${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 检查是否已经在运行
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if ps -p $OLD_PID > /dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  AI service already running (PID: $OLD_PID)${NC}"
        echo -e "${GREEN}✅ Service URL: http://localhost:50052${NC}"
        exit 0
    else
        echo -e "${YELLOW}🧹 Cleaning up stale PID file${NC}"
        rm "$PID_FILE"
    fi
fi

# 检查GO目录
if [ ! -d "$GO_DIR" ]; then
    echo -e "${RED}❌ GO directory not found: $GO_DIR${NC}"
    exit 1
fi

# 启动服务
echo -e "${GREEN}🔄 Starting AI service...${NC}"
cd "$GO_DIR"

# 后台启动并保存PID
nohup go run cmd/pixly-ai/main.go > "$LOG_FILE" 2>&1 &
SERVICE_PID=$!

# 保存PID
echo $SERVICE_PID > "$PID_FILE"

# 等待服务启动
echo -e "${YELLOW}⏳ Waiting for service to start...${NC}"
sleep 3

# 检查服务健康状态
if curl -s http://localhost:50052/api/v1/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ AI service started successfully!${NC}"
    echo -e "${GREEN}📊 PID: $SERVICE_PID${NC}"
    echo -e "${GREEN}🌐 URL: http://localhost:50052${NC}"
    echo -e "${GREEN}📝 Log: $LOG_FILE${NC}"
    echo ""
    echo -e "${YELLOW}💡 To stop the service, run:${NC}"
    echo -e "   kill $SERVICE_PID"
    echo -e "   or: ./stop-ai-service.sh"
else
    echo -e "${RED}❌ Failed to start AI service${NC}"
    echo -e "${YELLOW}📝 Check log file: $LOG_FILE${NC}"
    rm "$PID_FILE"
    exit 1
fi
