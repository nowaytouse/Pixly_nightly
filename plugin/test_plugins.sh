#!/bin/bash
# Plugin Integration Test Script
# Tests both converter and ai-optimizer plugins

echo "🧪 Pixly Plugin Integration Test"
echo "================================="

# Test directories
PLUGIN_DIR="/Users/nyamiiko/Documents/GIT/Pixly/Pixly_Nightly/plugin"
CONVERTER_DIR="$PLUGIN_DIR/converter"
AI_OPTIMIZER_DIR="$PLUGIN_DIR/ai-optimizer"

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

test_result() {
    if [ $1 -eq 0 ]; then
        echo "✅ $2"
        ((TESTS_PASSED++))
    else
        echo "❌ $2"
        ((TESTS_FAILED++))
    fi
}

echo
echo "📋 Testing Plugin Structure..."

# Test 1: Converter plugin structure
echo "🔍 Testing Converter Plugin..."
test -f "$CONVERTER_DIR/manifest.json"
test_result $? "Converter manifest.json exists"

test -f "$CONVERTER_DIR/index.html"
test_result $? "Converter index.html exists"

test -f "$CONVERTER_DIR/logo.png"
test_result $? "Converter logo.png exists"

test -d "$CONVERTER_DIR/js"
test_result $? "Converter js directory exists"

# Test 2: AI Optimizer plugin structure
echo
echo "🔍 Testing AI Optimizer Plugin..."
test -f "$AI_OPTIMIZER_DIR/manifest.json"
test_result $? "AI Optimizer manifest.json exists"

test -f "$AI_OPTIMIZER_DIR/index.html"
test_result $? "AI Optimizer index.html exists"

test -f "$AI_OPTIMIZER_DIR/logo-ai.png"
test_result $? "AI Optimizer logo-ai.png exists"

test -d "$AI_OPTIMIZER_DIR/js"
test_result $? "AI Optimizer js directory exists"

# Test 3: Manifest validation
echo
echo "🔍 Testing Manifest Files..."

# Check converter manifest JSON validity
cat "$CONVERTER_DIR/manifest.json" | python3 -m json.tool > /dev/null 2>&1
test_result $? "Converter manifest.json is valid JSON"

# Check AI optimizer manifest JSON validity
cat "$AI_OPTIMIZER_DIR/manifest.json" | python3 -m json.tool > /dev/null 2>&1
test_result $? "AI Optimizer manifest.json is valid JSON"

# Test 4: Plugin differentiation
echo
echo "🔍 Testing Plugin Differentiation..."
if grep -q "ai.optimizer" "$AI_OPTIMIZER_DIR/manifest.json" && grep -q "Pixly AI Optimizer" "$AI_OPTIMIZER_DIR/manifest.json"; then
    test_result 0 "AI Optimizer has unique identity"
else
    test_result 1 "AI Optimizer has unique identity"
fi

if grep -q "图像转换" "$CONVERTER_DIR/manifest.json"; then
    test_result 0 "Converter has correct keywords"
else
    test_result 1 "Converter has correct keywords"
fi

# Test 5: Logo files
echo
echo "🔍 Testing Logo Files..."
CONVERTER_LOGO_SIZE=$(stat -f%z "$CONVERTER_DIR/logo.png" 2>/dev/null || echo "0")
AI_LOGO_SIZE=$(stat -f%z "$AI_OPTIMIZER_DIR/logo-ai.png" 2>/dev/null || echo "0")

if [ "$CONVERTER_LOGO_SIZE" -gt 1000 ]; then
    test_result 0 "Converter logo has reasonable size ($CONVERTER_LOGO_SIZE bytes)"
else
    test_result 1 "Converter logo has reasonable size ($CONVERTER_LOGO_SIZE bytes)"
fi

if [ "$AI_LOGO_SIZE" -gt 1000 ]; then
    test_result 0 "AI Optimizer logo has reasonable size ($AI_LOGO_SIZE bytes)"
else
    test_result 1 "AI Optimizer logo has reasonable size ($AI_LOGO_SIZE bytes)"
fi

# Summary
echo
echo "📊 Test Summary:"
echo "=================="
echo "✅ Tests Passed: $TESTS_PASSED"
echo "❌ Tests Failed: $TESTS_FAILED"
echo "📊 Total Tests: $((TESTS_PASSED + TESTS_FAILED))"

if [ $TESTS_FAILED -eq 0 ]; then
    echo
    echo "🎉 All plugin integration tests passed!"
    echo "Both Converter and AI Optimizer plugins are ready for use."
    exit 0
else
    echo
    echo "⚠️  Some tests failed. Please check the plugin structure."
    exit 1
fi
