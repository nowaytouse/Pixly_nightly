#!/bin/bash

# 测试Eagle原地替换功能
# 使用实际的Eagle .info目录

TEST_DIR="/Users/nyamiiko/Downloads/11.library/images/MI31JYH9TLXLM.info"
INPUT_FILE="$TEST_DIR/posse_shy.png"

echo "🧪 Testing Eagle in-place replacement"
echo "======================================"
echo ""
echo "Test directory: $TEST_DIR"
echo "Input file: $INPUT_FILE"
echo ""

# 检查文件是否存在
if [ ! -f "$INPUT_FILE" ]; then
    echo "❌ Input file not found!"
    exit 1
fi

echo "📋 Before conversion:"
ls -lh "$TEST_DIR"
echo ""
echo "📄 metadata.json before:"
cat "$TEST_DIR/metadata.json" | jq '.ext, .size, .name'
echo ""

# 执行转换
echo "🔄 Converting..."
./plugin/format-vue/bin/pixly-converter convert "$INPUT_FILE" --format jxl --quality 90

echo ""
echo "📋 After conversion:"
ls -lh "$TEST_DIR"
echo ""
echo "📄 metadata.json after:"
cat "$TEST_DIR/metadata.json" | jq '.ext, .size, .name'
echo ""

# 检查结果
if [ -f "$TEST_DIR/posse_shy.jxl" ]; then
    echo "✅ JXL file created"
else
    echo "❌ JXL file NOT created"
fi

if [ -f "$INPUT_FILE" ]; then
    echo "❌ Original PNG file still exists (should be deleted!)"
else
    echo "✅ Original PNG file deleted"
fi

EXT=$(cat "$TEST_DIR/metadata.json" | jq -r '.ext')
if [ "$EXT" = "jxl" ]; then
    echo "✅ metadata.json ext updated to jxl"
else
    echo "❌ metadata.json ext NOT updated (still: $EXT)"
fi
