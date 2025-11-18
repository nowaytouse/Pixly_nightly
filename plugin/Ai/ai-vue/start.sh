#!/bin/bash

# PIXLY AI Vue Plugin - 快速启动脚本

echo "🚀 PIXLY AI Vue Plugin - 开发环境启动"
echo "========================================"
echo ""

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js 未安装！请先安装 Node.js"
    exit 1
fi

echo "✅ Node.js 版本: $(node --version)"
echo "✅ npm 版本: $(npm --version)"
echo ""

# 检查依赖
if [ ! -d "node_modules" ]; then
    echo "📦 首次运行，正在安装依赖..."
    npm install
    echo ""
fi

# 检查 Rust CLI
if [ ! -f "bin/pixly-rust" ]; then
    echo "⚠️  警告: Rust CLI 未找到 (bin/pixly-rust)"
    echo "   插件将以 Mock 模式运行"
    echo "   如需真实转换功能，请先编译 Rust CLI:"
    echo "   cd ../../ && cargo build --release"
    echo "   cp target/release/pixly-rust plugin/ai-vue-new/bin/"
    echo ""
fi

# 获取本机 IP
echo "🌐 网络地址:"
echo "   本机: http://localhost:5173"

if command -v ifconfig &> /dev/null; then
    LOCAL_IP=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n 1)
    if [ ! -z "$LOCAL_IP" ]; then
        echo "   局域网: http://$LOCAL_IP:5173"
    fi
fi

echo ""
echo "📋 快速操作:"
echo "   - 按 Ctrl+C 停止服务器"
echo "   - 浏览器会自动打开"
echo "   - 修改代码会自动热重载"
echo ""
echo "🔥 遵循 PROJECT_QUALITY_MANIFESTO.md 质量标准"
echo ""
echo "========================================"
echo "正在启动开发服务器..."
echo ""

# 启动开发服务器
npm run test:local
