#!/bin/bash
# 深度价值分析脚本 - 遵循PROJECT_QUALITY_MANIFESTO.md

echo "# 🔍 未使用模块深度价值分析"
echo ""
echo "**分析时间**: $(date '+%Y-%m-%d %H:%M:%S')"
echo "**原则**: 不草率删除，深度评估价值"
echo ""
echo "---"
echo ""

# 获取未使用的模块列表
unused_modules=$(grep "⚠️ 未使用" docs/MODULE_USAGE_REPORT.md | awk -F'|' '{print $2}' | tr -d ' ')

echo "## 分析维度"
echo ""
echo "1. **代码质量**: 函数数量、结构体数量、文档完整性"
echo "2. **功能完整性**: 是否有完整的API、测试、示例"
echo "3. **依赖复杂度**: 外部依赖数量"
echo "4. **未来价值**: 是否在roadmap中"
echo ""
echo "---"
echo ""
echo "## 详细分析"
echo ""

for mod in $unused_modules; do
    file="src/${mod}.rs"
    if [ ! -f "$file" ]; then
        continue
    fi
    
    lines=$(wc -l < "$file" | tr -d ' ')
    
    # 统计函数和结构体
    pub_fns=$(grep -c "pub fn" "$file" 2>/dev/null || echo 0)
    pub_structs=$(grep -c "pub struct" "$file" 2>/dev/null || echo 0)
    pub_enums=$(grep -c "pub enum" "$file" 2>/dev/null || echo 0)
    
    # 检查文档
    has_module_doc=$(grep -c "^// "$file" 2>/dev/null || echo 0)
    
    # 检查依赖
    external_deps=$(grep "^use " "$file" | grep -v "use crate::" | grep -v "use std::" | wc -l | tr -d ' ')
    
    # 计算价值分数 (0-5)
    score=0
    
    # 代码量 (0-2分)
    if [ "$lines" -gt 300 ]; then
        score=$((score + 2))
    elif [ "$lines" -gt 100 ]; then
        score=$((score + 1))
    fi
    
    # API完整性 (0-2分)
    total_api=$((pub_fns + pub_structs + pub_enums))
    if [ "$total_api" -gt 10 ]; then
        score=$((score + 2))
    elif [ "$total_api" -gt 3 ]; then
        score=$((score + 1))
    fi
    
    # 文档 (0-1分)
    if [ "$has_module_doc" -gt 0 ]; then
        score=$((score + 1))
    fi
    
    # 判断价值等级
    if [ "$score" -ge 4 ]; then
        value="⭐⭐⭐⭐⭐ 极高"
        action="🔥 优先集成"
    elif [ "$score" -ge 3 ]; then
        value="⭐⭐⭐⭐ 高"
        action="✅ 考虑集成"
    elif [ "$score" -ge 2 ]; then
        value="⭐⭐⭐ 中"
        action="�� 评估后决定"
    else
        value="⭐⭐ 低"
        action="🗑️ 可能删除"
    fi
    
    echo "### $mod"
    echo ""
    echo "- **行数**: $lines"
    echo "- **公开API**: $pub_fns函数 + $pub_structs结构 + $pub_enums枚举 = $total_api"
    echo "- **模块文档**: $([ "$has_module_doc" -gt 0 ] && echo "✅ 有" || echo "❌ 无")"
    echo "- **外部依赖**: $external_deps"
    echo "- **价值评分**: $score/5 - $value"
    echo "- **建议行动**: $action"
    echo ""
done

echo "---"
echo ""
echo "## 统计摘要"
echo ""

# 统计各价值等级的数量
high_value=$(grep "⭐⭐⭐⭐⭐ 极高\|⭐⭐⭐⭐ 高" docs/DEEP_VALUE_ANALYSIS.md 2>/dev/null | wc -l | tr -d ' ')
medium_value=$(grep "⭐⭐⭐ 中" docs/DEEP_VALUE_ANALYSIS.md 2>/dev/null | wc -l | tr -d ' ')
low_value=$(grep "⭐⭐ 低" docs/DEEP_VALUE_ANALYSIS.md 2>/dev/null | wc -l | tr -d ' ')

echo "- **高价值模块** (≥4分): 需要优先集成"
echo "- **中价值模块** (2-3分): 评估后决定"
echo "- **低价值模块** (<2分): 可能删除"
echo ""
echo "**下一步**: 为高价值模块制定集成计划"
