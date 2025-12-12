#!/opt/homebrew/bin/bash
# 🧪 Python ML集成测试脚本

set -e

echo "🧪 Python ML Integration Test"
echo "=============================="
echo ""

# 1. 测试Python ML Bridge
echo "📋 Test 1: Python ML Bridge"
python3 scripts/ml_bridge.py --test
echo ""

# 2. 测试模型列表
echo "📋 Test 2: Model List"
python3 scripts/ml_bridge.py --list-models
echo ""

# 3. 测试Python ML预测（模拟请求）
echo "📋 Test 3: Python ML Prediction"
cat > /tmp/test_ml_request.json << 'EOF'
{
  "features": [1920.0, 1080.0, 2073600.0, 2.5, 1.78, 0.0, 0.0, 0.65, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  "target_format": "avif",
  "quality_mode": "balanced"
}
EOF

python3 scripts/ml_bridge.py --predict "$(cat /tmp/test_ml_request.json)"
echo ""

# 4. 测试视频预测
echo "📋 Test 4: Video ML Prediction"
cat > /tmp/test_video_request.json << 'EOF'
{
  "features": [1920.0, 1080.0, 2073600.0, 10.5, 1.78, 300.0, 30.0, 10.0, 1.0, 0.7, 1.0, 0.0, 1.0, 15.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
  "target_format": "video",
  "quality_mode": "balanced"
}
EOF

python3 scripts/ml_bridge.py --predict "$(cat /tmp/test_video_request.json)"
echo ""

# 5. 检查Rust CLI是否存在
echo "📋 Test 5: Rust CLI Binary"
if [ -f "./target/release/pixly-converter" ]; then
    echo "✅ Rust CLI binary exists"
    ./target/release/pixly-converter --version || echo "⚠️ Version command failed"
else
    echo "❌ Rust CLI binary not found"
    echo "   Run: cargo build --release"
fi
echo ""

echo "=============================="
echo "✅ All Python ML tests passed!"
echo ""
echo "Next steps:"
echo "1. Create test image: convert -size 1920x1080 xc:blue test.png"
echo "2. Test conversion: ./target/release/pixly-converter convert test.png test.avif"
echo "3. Check logs for: '🐍 Calling Python ML Bridge...'"
