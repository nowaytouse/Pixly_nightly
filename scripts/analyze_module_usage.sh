#!/bin/bash
# 模块使用情况分析脚本

echo "# 📊 Pixly模块使用情况报告"
echo ""
echo "**生成时间**: $(date '+%Y-%m-%d %H:%M:%S')"
echo "**总模块数**: $(ls src/*.rs | wc -l | tr -d ' ')"
echo ""
echo "---"
echo ""
echo "## 模块引用统计"
echo ""
echo "| 模块名 | 引用次数 | 行数 | 状态 |"
echo "|--------|---------|------|------|"

for file in src/*.rs; do
    mod=$(basename "$file" .rs)
    if [ "$mod" = "lib" ]; then
        continue
    fi
    
    # 统计引用次数
    refs=$(grep -r "use.*::$mod\|use crate::$mod" src/ pixly_*.rs 2>/dev/null | wc -l | tr -d ' ')
    lines=$(wc -l < "$file" | tr -d ' ')
    
    # 判断状态
    if [ "$refs" -eq 0 ]; then
        status="⚠️ 未使用"
    elif [ "$refs" -lt 3 ]; then
        status="🟡 少量使用"
    else
        status="✅ 活跃"
    fi
    
    echo "| $mod | $refs | $lines | $status |"
done | sort -t'|' -k3 -n -r

echo ""
echo "---"
echo ""
echo "## 统计摘要"
echo ""

total=$(ls src/*.rs | grep -v lib.rs | wc -l | tr -d ' ')
unused=$(for file in src/*.rs; do
    mod=$(basename "$file" .rs)
    [ "$mod" = "lib" ] && continue
    refs=$(grep -r "use.*::$mod\|use crate::$mod" src/ pixly_*.rs 2>/dev/null | wc -l)
    [ "$refs" -eq 0 ] && echo "$mod"
done | wc -l | tr -d ' ')

echo "- **总模块数**: $total"
echo "- **未使用模块**: $unused"
echo "- **使用率**: $(echo "scale=1; ($total - $unused) * 100 / $total" | bc)%"
