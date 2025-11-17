#!/bin/bash
# 批量为 ui-handlers.js 的纯 console 调用添加 pixlyLog
# Phase 1: Lines 829-1100

TARGET_FILE="core/plugin/js/plugin-modules/ui-handlers.js"
BACKUP_FILE="${TARGET_FILE}.batch1.backup"

echo "=== ui-handlers.js 批量迁移 Phase 1 ==="
echo "目标: Lines 829-1100 的纯 console 调用"
echo ""

# 备份
cp "$TARGET_FILE" "$BACKUP_FILE"
echo "✅ 已备份到: $BACKUP_FILE"

# 由于需要精确修改，建议手动处理
# 列出需要修改的行
echo ""
echo "需要添加 pixlyLog 的行:"
grep -n "^\s*console\.\(log\|warn\|error\)" "$TARGET_FILE" | \
  grep -v "log\?\." | \
  awk -F: '$1 >= 829 && $1 <= 1100 {print}'

echo ""
echo "⚠️ 由于每个 console 调用的上下文不同"
echo "建议使用 multi_edit 工具逐个处理"
echo ""
echo "模式："
echo "原: console.log('[TAG] message', data)"
echo "新: if (log) log.info('TAG', 'message', data);"
echo "    console.log('[TAG] message', data)  // 保留作为 fallback"
