#!/opt/homebrew/bin/bash
# ai-vue-refactor: 设置开发环境 - 创建符号链接到共享二进制

set -e
cd "$(dirname "$0")"

echo "🔗 Setting up ai-vue-refactor development environment..."

# 创建 bin 目录
mkdir -p bin

# 创建符号链接到共享二进制
if [ -f "../shared/bin/pixly-eagle-core" ]; then
    ln -sf ../../shared/bin/pixly-eagle-core bin/pixly-eagle-core
    echo "✅ Symlinked: ai-vue-refactor/bin/pixly-eagle-core -> ../shared/bin/pixly-eagle-core"
else
    echo "❌ Error: Shared binary not found at ../shared/bin/pixly-eagle-core"
    echo "   Please run: cd ../shared && bash build.sh"
    exit 1
fi

echo ""
echo "Development environment ready!"
echo "Now run: npm install && npm run dev"
