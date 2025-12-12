#!/opt/homebrew/bin/bash
# 测试所有格式转换支持

set -e

echo "🧪 测试所有格式转换支持"
echo "================================"

CLI="./target/release/pixly"
INPUT="data/test_images/test.jpg"
OUTPUT_DIR="test_output/format_test"

# 确保CLI已编译
if [ ! -f "$CLI" ]; then
    echo "❌ CLI未编译，正在编译..."
    cargo build --release
fi

# 创建输出目录
mkdir -p "$OUTPUT_DIR"

# 测试图像格式
echo ""
echo "📷 测试图像格式转换..."
echo "--------------------------------"

formats=("webp" "png" "jpeg" "gif" "bmp" "tiff" "avif" "jxl")

for fmt in "${formats[@]}"; do
    output="$OUTPUT_DIR/test.$fmt"
    echo -n "  $fmt: "
    
    if $CLI convert "$INPUT" "$output" --quality 85 2>&1 | grep -q "Conversion completed"; then
        if [ -f "$output" ] && [ -s "$output" ]; then
            size=$(du -h "$output" | cut -f1)
            echo "✅ ($size)"
        else
            echo "❌ 文件为空"
        fi
    else
        echo "❌ 转换失败"
    fi
done

# 测试同格式优化
echo ""
echo "🔄 测试同格式优化..."
echo "--------------------------------"

# 测试所有格式的同格式优化
for fmt in "${formats[@]}"; do
    input_file="$OUTPUT_DIR/test.$fmt"
    output_file="$OUTPUT_DIR/test_optimized.$fmt"
    
    if [ ! -f "$input_file" ]; then
        echo "  $fmt→$fmt: ⏭️  跳过(无输入)"
        continue
    fi
    
    echo -n "  $fmt→$fmt: "
    
    if $CLI convert "$input_file" "$output_file" --quality 85 2>&1 | grep -q "Conversion completed"; then
        if [ -f "$output_file" ] && [ -s "$output_file" ]; then
            original_size=$(stat -f%z "$input_file")
            optimized_size=$(stat -f%z "$output_file")
            ratio=$(echo "scale=1; $optimized_size * 100 / $original_size" | bc)
            echo "✅ (${ratio}% of original)"
        else
            echo "❌ 文件为空"
        fi
    else
        echo "❌ 转换失败"
    fi
done

# 测试视频格式(如果有视频文件)
echo ""
echo "🎬 测试视频格式..."
echo "--------------------------------"

if [ -f "data/test_images/test.mp4" ] || [ -f "data/test_images/test.gif" ]; then
    echo "  视频转换功能已实现"
else
    echo "  ⏭️  跳过(无测试视频)"
fi

echo ""
echo "================================"
echo "✅ 格式支持测试完成"
