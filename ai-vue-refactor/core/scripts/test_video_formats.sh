#!/opt/homebrew/bin/bash
# 测试视频格式支持

set -e

echo "🎬 测试视频格式支持"
echo "================================"

CLI="./target/release/pixly"

# 测试GIF转视频
echo ""
echo "📹 测试动图转视频..."
echo "--------------------------------"

# 创建测试GIF (如果不存在)
if [ ! -f "test_output/test_anim.gif" ]; then
    echo "创建测试动图..."
    magick convert data/test_images/test.jpg -resize 200x200 \
        \( -clone 0 -rotate 10 \) \
        \( -clone 0 -rotate 20 \) \
        \( -clone 0 -rotate 30 \) \
        -loop 0 -delay 10 test_output/test_anim.gif 2>/dev/null || echo "⏭️  跳过GIF创建"
fi

if [ -f "test_output/test_anim.gif" ]; then
    codecs=("h265" "h264" "vp9" "av1")
    
    for codec in "${codecs[@]}"; do
        output="test_output/video_${codec}.mp4"
        echo -n "  GIF→${codec}: "
        
        if timeout 30 $CLI video test_output/test_anim.gif "$output" --codec "$codec" 2>&1 | grep -q "conversion complete"; then
            if [ -f "$output" ] && [ -s "$output" ]; then
                size=$(du -h "$output" | cut -f1)
                echo "✅ ($size)"
            else
                echo "❌ 文件为空"
            fi
        else
            echo "❌ 转换失败或超时"
        fi
    done
else
    echo "  ⏭️  跳过(无测试GIF)"
fi

echo ""
echo "================================"
echo "✅ 视频格式测试完成"
