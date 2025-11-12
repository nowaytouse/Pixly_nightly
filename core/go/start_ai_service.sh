#!/bin/bash

# Pixly AI Service 启动脚本
# 用途：一键启动AI参数预测服务

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🚀 Pixly AI Service Launcher"
echo "============================="
echo ""

# 检查Go是否安装
if ! command -v go &> /dev/null; then
    echo "❌ Go is not installed"
    echo "💡 Please install Go: https://golang.org/dl/"
    exit 1
fi

echo "✅ Go version: $(go version | cut -d' ' -f3)"
echo ""

# 检查main.go是否存在
if [ ! -f "cmd/pixly-ai/main.go" ]; then
    echo "❌ main.go not found at cmd/pixly-ai/main.go"
    exit 1
fi

echo "✅ AI Service main.go found"
echo ""

# 检查端口50052是否被占用
if lsof -Pi :50052 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "⚠️  Port 50052 is already in use"
    echo "💡 AI Service might already be running"
    echo ""
    read -p "Kill existing process and restart? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        lsof -ti:50052 | xargs kill -9 2>/dev/null || true
        echo "✅ Existing process killed"
        sleep 1
    else
        echo "❌ Aborted"
        exit 0
    fi
fi

echo "🔄 Starting AI Service..."
echo ""

# 启动服务
go run cmd/pixly-ai/main.go
