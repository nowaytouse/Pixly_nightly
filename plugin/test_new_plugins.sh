#!/bin/bash

# Pixly插件测试脚本

echo "🧪 测试Pixly插件"
echo "=================="
echo ""

# 检查Converter Plugin
echo "📦 检查 Converter Plugin..."
if [ -f "Converter Plugin/index.html" ] && [ -f "Converter Plugin/js/plugin.js" ]; then
    echo "✅ Converter Plugin 文件完整"
    echo "   - index.html: $(wc -l < "Converter Plugin/index.html") 行"
    echo "   - plugin.js: $(wc -l < "Converter Plugin/js/plugin.js") 行"
    echo "   - rust-cli.js: $(wc -l < "Converter Plugin/js/rust-cli.js") 行"
else
    echo "❌ Converter Plugin 文件缺失"
fi
echo ""

# 检查AI Optimizer Plugin
echo "🤖 检查 AI Optimizer Plugin..."
if [ -f "AI Optimizer Plugin/index.html" ] && [ -f "AI Optimizer Plugin/js/plugin.js" ]; then
    echo "✅ AI Optimizer Plugin 文件完整"
    echo "   - index.html: $(wc -l < "AI Optimizer Plugin/index.html") 行"
    echo "   - plugin.js: $(wc -l < "AI Optimizer Plugin/js/plugin.js") 行"
    echo "   - rust-cli.js: $(wc -l < "AI Optimizer Plugin/js/rust-cli.js") 行"
else
    echo "❌ AI Optimizer Plugin 文件缺失"
fi
echo ""

# 统计代码行数
echo "📊 代码统计..."
converter_lines=$(find "Converter Plugin" -name "*.js" -o -name "*.html" | xargs wc -l | tail -1 | awk '{print $1}')
optimizer_lines=$(find "AI Optimizer Plugin" -name "*.js" -o -name "*.html" | xargs wc -l | tail -1 | awk '{print $1}')
total_lines=$((converter_lines + optimizer_lines))

echo "   Converter Plugin: $converter_lines 行"
echo "   AI Optimizer Plugin: $optimizer_lines 行"
echo "   总计: $total_lines 行"
echo ""

# 对比旧版本
if [ -d "old/converter" ]; then
    old_lines=$(find "old" -name "*.js" -o -name "*.html" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}')
    echo "📉 代码减少: $old_lines → $total_lines 行"
    reduction=$((100 - (total_lines * 100 / old_lines)))
    echo "   减少比例: ${reduction}%"
fi
echo ""

echo "✅ 测试完成！"
