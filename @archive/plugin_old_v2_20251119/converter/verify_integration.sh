#!/bin/bash

# Pixly Plugin v4.2.0 - 集成验证脚本
# 快速检查插件关键集成点

echo "╔═══════════════════════════════════════════════════════════════════════╗"
echo "║                                                                       ║"
echo "║   🔍 Pixly Plugin v4.2.0 - 集成验证                                  ║"
echo "║                                                                       ║"
echo "╚═══════════════════════════════════════════════════════════════════════╝"
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 计数器
PASS=0
FAIL=0

# 验证函数
check() {
    local test_name=$1
    local file_path=$2
    local pattern=$3
    
    if [ -f "$file_path" ]; then
        if grep -q "$pattern" "$file_path" 2>/dev/null; then
            echo -e "${GREEN}✅${NC} $test_name"
            ((PASS++))
        else
            echo -e "${RED}❌${NC} $test_name - 未找到模式: $pattern"
            ((FAIL++))
        fi
    else
        echo -e "${RED}❌${NC} $test_name - 文件不存在: $file_path"
        ((FAIL++))
    fi
}

check_exists() {
    local test_name=$1
    local file_path=$2
    
    if [ -f "$file_path" ]; then
        echo -e "${GREEN}✅${NC} $test_name"
        ((PASS++))
    else
        echo -e "${RED}❌${NC} $test_name - 文件不存在"
        ((FAIL++))
    fi
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1️⃣  核心文件存在性检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check_exists "插件入口文件" "index.html"
check_exists "AI客户端模块" "js/plugin-modules/22-ai-client.js"
check_exists "参数构建器" "js/plugin-modules/21-params-builder.js"
check_exists "转换逻辑" "js/plugin-modules/04-conversion.js"
check_exists "插件加载器" "js/plugin-loader.js"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2️⃣  UI开关定义检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check "智能质量预测开关" "index.html" "enableSmartQuality"
check "自动参数优化开关" "index.html" "enableAutoOptimize"
check "SSIM验证开关" "index.html" "enableSSIMValidation"
check "格式智能选择开关" "index.html" "enableSmartFormat"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3️⃣  AI客户端集成检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check "AIClient类定义" "js/plugin-modules/22-ai-client.js" "class AIClient"
check "全局aiClient导出" "js/plugin-modules/22-ai-client.js" "window.aiClient"
check "健康检查方法" "js/plugin-modules/22-ai-client.js" "checkHealth"
check "参数预测方法" "js/plugin-modules/22-ai-client.js" "predictParams"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4️⃣  转换逻辑集成检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check "AI选项读取" "js/plugin-modules/04-conversion.js" "aiOptions"
check "smartQuality读取" "js/plugin-modules/04-conversion.js" "enableSmartQuality"
check "autoOptimize读取" "js/plugin-modules/04-conversion.js" "enableAutoOptimize"
check "AI日志输出" "js/plugin-modules/04-conversion.js" "AI选项"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5️⃣  参数构建器集成检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check "enhanceWithAI函数" "js/plugin-modules/21-params-builder.js" "enhanceWithAI"
check "AI客户端调用" "js/plugin-modules/21-params-builder.js" "window.aiClient"
check "全局函数导出" "js/plugin-modules/21-params-builder.js" "window.enhanceParamsWithAI"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "6️⃣  模块加载检查"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

check "22-ai-client加载" "js/plugin-loader.js" "22-ai-client.js"
check "21-params-builder加载" "js/plugin-loader.js" "21-params-builder.js"
check "04-conversion加载" "js/plugin-loader.js" "04-conversion.js"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 测试结果汇总"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "通过: ${GREEN}$PASS${NC}项"
echo -e "失败: ${RED}$FAIL${NC}项"
echo "总计: $((PASS + FAIL))项"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✅ 所有集成检查通过！${NC}"
    echo ""
    echo "下一步:"
    echo "  1. 在Eagle中加载插件"
    echo "  2. 启动AI服务: cd ../cmd/ai-service && go run main.go"
    echo "  3. 查看集成测试文档: cat INTEGRATION_TEST.md"
    exit 0
else
    echo -e "${RED}❌ 发现$FAIL个问题，请检查！${NC}"
    echo ""
    echo "故障排除:"
    echo "  1. 检查文件路径是否正确"
    echo "  2. 检查代码是否有语法错误"
    echo "  3. 查看 INTEGRATION_TEST.md 获取详细信息"
    exit 1
fi
