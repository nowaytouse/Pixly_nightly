#!/bin/bash
# Comprehensive Code Quality Verification
# Based on PROJECT_QUALITY_MANIFESTO.md requirements

echo "🔍 Pixly Code Quality Verification"
echo "===================================="
echo ""

ERRORS=0
WARNINGS=0

# 1. Check for Chinese output in Python scripts
echo "📋 [1/6] Checking Python scripts for Chinese output..."
CHINESE_PRINTS=$(grep -rn "print.*[\u4e00-\u9fff]" scripts/*.py tools/**/*.py 2>/dev/null | \
    grep -v "batch_fix_chinese_output.py" | \
    grep -v "fix_chinese_output.py" | \
    grep -v "translate_chinese" | \
    grep -v "#.*print" | \
    wc -l | tr -d ' ')

if [ "$CHINESE_PRINTS" -gt 0 ]; then
    echo "   ❌ Found $CHINESE_PRINTS print statements with Chinese output"
    echo "   Files:"
    grep -rn "print.*[\u4e00-\u9fff]" scripts/*.py tools/**/*.py 2>/dev/null | \
        grep -v "batch_fix_chinese_output.py" | \
        grep -v "fix_chinese_output.py" | \
        grep -v "translate_chinese" | \
        grep -v "#.*print" | \
        cut -d: -f1 | sort -u | sed 's/^/      - /'
    ERRORS=$((ERRORS + 1))
else
    echo "   ✅ No Chinese output in Python scripts"
fi

# 2. Check for Chinese output in Rust code
echo ""
echo "📋 [2/6] Checking Rust code for Chinese output..."
RUST_CHINESE=$(grep -rn "println!.*[\u4e00-\u9fff]" src/*.rs 2>/dev/null | wc -l | tr -d ' ')

if [ "$RUST_CHINESE" -gt 0 ]; then
    echo "   ❌ Found $RUST_CHINESE println! with Chinese output"
    ERRORS=$((ERRORS + 1))
else
    echo "   ✅ No Chinese output in Rust code"
fi

# 3. Check for hardcoded Chinese in Vue components
echo ""
echo "📋 [3/6] Checking Vue components for hardcoded Chinese..."
VUE_CHINESE=$(grep -rn "[\u4e00-\u9fff]" plugin/*/src/**/*.vue 2>/dev/null | \
    grep -v "i18n" | \
    grep -v "zh_CN" | \
    wc -l | tr -d ' ')

if [ "$VUE_CHINESE" -gt 0 ]; then
    echo "   ⚠️  Found $VUE_CHINESE potential hardcoded Chinese strings"
    WARNINGS=$((WARNINGS + 1))
else
    echo "   ✅ No hardcoded Chinese in Vue components"
fi

# 4. Check for direct console.log usage (should use logger)
echo ""
echo "📋 [4/6] Checking for direct console usage in plugins..."
CONSOLE_USAGE=$(grep -rn "console\.\(log\|warn\|error\|info\)" plugin/*/src/**/*.{js,vue} 2>/dev/null | \
    grep -v "logger.js" | \
    grep -v "// console" | \
    wc -l | tr -d ' ')

if [ "$CONSOLE_USAGE" -gt 0 ]; then
    echo "   ⚠️  Found $CONSOLE_USAGE direct console usage (should use logger)"
    echo "   Files:"
    grep -rn "console\.\(log\|warn\|error\|info\)" plugin/*/src/**/*.{js,vue} 2>/dev/null | \
        grep -v "logger.js" | \
        grep -v "// console" | \
        cut -d: -f1 | sort -u | sed 's/^/      - /'
    WARNINGS=$((WARNINGS + 1))
else
    echo "   ✅ All logging uses logger system"
fi

# 5. Check for logger system existence
echo ""
echo "📋 [5/6] Verifying logger systems..."
LOGGER_COUNT=0

if [ -f "plugin/format-vue/src/utils/logger.js" ]; then
    echo "   ✅ format-vue has logger system"
    LOGGER_COUNT=$((LOGGER_COUNT + 1))
else
    echo "   ❌ format-vue missing logger system"
    ERRORS=$((ERRORS + 1))
fi

if [ -f "plugin/ai-vue-refactor/src/utils/logger.js" ]; then
    echo "   ✅ ai-vue-refactor has logger system"
    LOGGER_COUNT=$((LOGGER_COUNT + 1))
else
    echo "   ❌ ai-vue-refactor missing logger system"
    ERRORS=$((ERRORS + 1))
fi

# 6. Check for i18n files
echo ""
echo "📋 [6/6] Verifying i18n files..."
I18N_COUNT=0

for plugin in format-vue ai-vue-refactor; do
    if [ -f "plugin/$plugin/src/i18n/en.json" ] && [ -f "plugin/$plugin/src/i18n/zh_CN.json" ]; then
        echo "   ✅ $plugin has complete i18n"
        I18N_COUNT=$((I18N_COUNT + 1))
    else
        echo "   ❌ $plugin missing i18n files"
        ERRORS=$((ERRORS + 1))
    fi
done

# Summary
echo ""
echo "===================================="
echo "📊 Verification Summary"
echo "===================================="
echo "Errors:   $ERRORS"
echo "Warnings: $WARNINGS"
echo ""

if [ "$ERRORS" -eq 0 ] && [ "$WARNINGS" -eq 0 ]; then
    echo "✅ All checks passed! Code quality is excellent."
    exit 0
elif [ "$ERRORS" -eq 0 ]; then
    echo "⚠️  All critical checks passed, but there are $WARNINGS warnings."
    exit 0
else
    echo "❌ Found $ERRORS critical issues. Please fix them."
    exit 1
fi
