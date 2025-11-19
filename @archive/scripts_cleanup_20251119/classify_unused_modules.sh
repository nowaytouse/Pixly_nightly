#!/bin/bash
# 系统化分类未使用模块

echo "# 🔍 未使用模块分类报告"
echo ""
echo "**分类日期**: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 获取所有未使用模块
unused=$(grep "⚠️ 未使用" docs/MODULE_USAGE_REPORT.md | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')

echo "## 分类1: 旧架构遗留 (从@archive恢复但已被新架构替代)"
echo ""
for mod in $unused; do
    if grep -q "@archive\|从.*archive\|Phase.*恢复" "src/${mod}.rs" 2>/dev/null; then
        lines=$(wc -l < "src/${mod}.rs" 2>/dev/null | tr -d ' ')
        echo "- **${mod}** (${lines}行) - 🗑️ 建议删除"
    fi
done

echo ""
echo "## 分类2: CLI相关模块 (可能被pixly_*.rs使用)"
echo ""
for mod in $unused; do
    if [[ "$mod" == cli_* ]]; then
        lines=$(wc -l < "src/${mod}.rs" 2>/dev/null | tr -d ' ')
        echo "- **${mod}** (${lines}行) - ⚠️ 检查CLI使用情况"
    fi
done

echo ""
echo "## 分类3: 批处理相关 (可能重复)"
echo ""
for mod in $unused; do
    if [[ "$mod" == *batch* ]]; then
        lines=$(wc -l < "src/${mod}.rs" 2>/dev/null | tr -d ' ')
        echo "- **${mod}** (${lines}行) - 🤔 评估是否重复"
    fi
done

echo ""
echo "## 分类4: 高价值功能模块 (应该被使用)"
echo ""
for mod in format_knowledge eagle_adapter ai quality_analyzer validation_integration automl visual_quality_scorer ml_predictor; do
    if echo "$unused" | grep -q "^${mod}$"; then
        lines=$(wc -l < "src/${mod}.rs" 2>/dev/null | tr -d ' ')
        pub_fns=$(grep -c "pub fn" "src/${mod}.rs" 2>/dev/null)
        echo "- **${mod}** (${lines}行, ${pub_fns}个公开函数) - 🔥 高价值，需要集成"
    fi
done

echo ""
echo "## 分类5: 其他模块"
echo ""
echo "*(剩余模块，需要逐个评估)*"
