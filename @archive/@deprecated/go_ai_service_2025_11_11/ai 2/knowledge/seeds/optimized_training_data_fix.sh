#!/bin/bash
# 将所有记录改为指针

FILE="pkg/knowledge/seeds/optimized_training_data.go"

sed -i.bak2 '
s/return \[\]knowledge\.ConversionRecord{/records := []knowledge.ConversionRecord{/
' "$FILE"

# 在最后一个}前添加指针转换代码
cat >> "$FILE" << 'EOT'

	// 转换为指针切片
	result := make([]*knowledge.ConversionRecord, len(records))
	for i := range records {
		result[i] = &records[i]
	}
	return result
}
EOT

# 删除原来的最后两行（旧的return语句）
head -n -2 "$FILE" > "$FILE.tmp" && mv "$FILE.tmp" "$FILE"

echo "✅ 已修复指针类型"
