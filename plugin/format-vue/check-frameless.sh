#!/bin/bash

echo "🔍 检查无边框窗口配置"
echo "======================================"
echo ""

# 检查manifest.json
echo "📄 manifest.json 配置："
if grep -q '"frameless": true' plugin/format-vue/manifest.json; then
    echo "  ✅ frameless: true"
else
    echo "  ❌ frameless 未设置"
fi

if grep -q '"frame": false' plugin/format-vue/manifest.json; then
    echo "  ✅ frame: false"
else
    echo "  ⚠️  frame 未设置（可选）"
fi

if grep -q '"titleBarStyle"' plugin/format-vue/manifest.json; then
    echo "  ✅ titleBarStyle 已设置"
else
    echo "  ⚠️  titleBarStyle 未设置（可选）"
fi

echo ""

# 检查App.vue
echo "📄 App.vue 标题栏："
if grep -q 'class="titlebar"' plugin/format-vue/src/App.vue; then
    echo "  ✅ 标题栏组件存在"
else
    echo "  ❌ 标题栏组件缺失"
fi

if grep -q 'webkit-app-region: drag' plugin/format-vue/src/App.vue; then
    echo "  ✅ 拖拽区域已配置"
else
    echo "  ❌ 拖拽区域未配置"
fi

if grep -q 'minimizeWindow' plugin/format-vue/src/App.vue; then
    echo "  ✅ 窗口控制函数存在"
else
    echo "  ❌ 窗口控制函数缺失"
fi

echo ""

# 检查global.css
echo "📄 global.css 样式："
if grep -q 'overflow: hidden' plugin/format-vue/src/styles/global.css; then
    echo "  ✅ overflow: hidden 已设置"
else
    echo "  ❌ overflow: hidden 未设置"
fi

echo ""

# 检查构建输出
echo "📦 构建状态："
if [ -f "plugin/format-vue/dist/index.html" ]; then
    echo "  ✅ dist/index.html 存在"
    
    # 检查构建时间
    BUILD_TIME=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M:%S" plugin/format-vue/dist/index.html 2>/dev/null || stat -c "%y" plugin/format-vue/dist/index.html 2>/dev/null | cut -d'.' -f1)
    echo "  📅 构建时间: $BUILD_TIME"
else
    echo "  ❌ dist/index.html 不存在"
    echo "  💡 运行: cd plugin/format-vue && npm run build"
fi

echo ""
echo "======================================"
echo ""
echo "✅ 配置检查完成！"
echo ""
echo "🔄 使配置生效："
echo "  1. 完全退出Eagle（Cmd+Q）"
echo "  2. 重新启动Eagle"
echo "  3. 打开PIXLY Format Vue插件"
echo ""
echo "📖 详细说明: plugin/format-vue/FRAMELESS_WINDOW_SETUP.md"
