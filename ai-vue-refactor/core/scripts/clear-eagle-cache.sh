#!/bin/bash

echo "🔧 PIXLY - Eagle 缓存清理脚本"
echo "================================"
echo ""

# 检查 Eagle 是否在运行
if pgrep -x "Eagle" > /dev/null; then
    echo "⚠️  检测到 Eagle 正在运行"
    echo "请先退出 Eagle (Cmd+Q)，然后重新运行此脚本"
    echo ""
    read -p "按回车键退出..."
    exit 1
fi

echo "✅ Eagle 已关闭"
echo ""

# 清除缓存
CACHE_DIR="$HOME/Library/Application Support/Eagle/Caches"

if [ -d "$CACHE_DIR" ]; then
    echo "🗑️  正在清除缓存..."
    rm -rf "$CACHE_DIR"/*
    echo "✅ 缓存已清除"
else
    echo "ℹ️  未找到缓存目录"
fi

echo ""
echo "================================"
echo "✅ 清理完成！"
echo ""
echo "现在可以："
echo "1. 打开 Eagle"
echo "2. 重新加载 PIXLY 插件"
echo ""
echo "错误应该已经消失！"
echo "================================"
