#!/bin/bash

echo "🌐 启动本地测试服务器..."
echo ""
echo "访问地址："
echo "  http://localhost:8000/simple-test.html"
echo ""
echo "按 Ctrl+C 停止服务器"
echo ""

cd "$(dirname "$0")"
python3 -m http.server 8000
