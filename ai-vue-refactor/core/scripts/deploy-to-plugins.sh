#!/bin/bash

# 自动部署 pixly-converter 到 Eagle 插件
# 在编译完 Rust 项目后运行此脚本

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."

echo "🔨 部署 pixly-converter 到 Eagle 插件..."

# 检查 release 二进制文件是否存在
if [ ! -f "$PROJECT_ROOT/target/release/pixly-converter" ]; then
    echo "❌ 未找到 target/release/pixly-converter"
    echo "请先编译: cargo build --release"
    exit 1
fi

# 复制到 ai-vue-refactor 插件
echo "📦 部署到 ai-vue-refactor..."
mkdir -p "$PROJECT_ROOT/plugin/ai-vue-refactor/bin"
cp "$PROJECT_ROOT/target/release/pixly-converter" "$PROJECT_ROOT/plugin/ai-vue-refactor/bin/"
chmod +x "$PROJECT_ROOT/plugin/ai-vue-refactor/bin/pixly-converter"

# 验证
echo ""
echo "✅ 部署完成!"
echo ""
echo "已部署的二进制文件:"
ls -lh "$PROJECT_ROOT/plugin/ai-vue-refactor/bin/pixly-converter"

echo ""
echo "版本信息:"
"$PROJECT_ROOT/plugin/ai-vue-refactor/bin/pixly-converter" --version

echo ""
echo "🎉 现在可以在 Eagle 中使用 AI 插件了!"
