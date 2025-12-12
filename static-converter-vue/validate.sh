#!/opt/homebrew/bin/bash
# Validation script for JPEG to JXL Eagle Plugin

set -e

PLUGIN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 JPEG to JXL Plugin Validation${NC}"
echo "=================================="
echo ""

ERRORS=0
WARNINGS=0

# Check manifest.json
echo -e "${BLUE}[1/6]${NC} Checking manifest.json..."
if [ -f "$PLUGIN_DIR/manifest.json" ]; then
    echo -e "${GREEN}  ✓${NC} manifest.json exists"
    
    # Validate JSON syntax
    if command -v python3 &> /dev/null; then
        if python3 -c "import json; json.load(open('$PLUGIN_DIR/manifest.json'))" 2>/dev/null; then
            echo -e "${GREEN}  ✓${NC} Valid JSON format"
        else
            echo -e "${RED}  ✗${NC} Invalid JSON format"
            ((ERRORS++))
        fi
    fi
else
    echo -e "${RED}  ✗${NC} manifest.json not found"
    ((ERRORS++))
fi
echo ""

# Check logo
echo -e "${BLUE}[2/6]${NC} Checking logo.png..."
if [ -f "$PLUGIN_DIR/logo.png" ]; then
    SIZE=$(stat -f%z "$PLUGIN_DIR/logo.png" 2>/dev/null || stat -c%s "$PLUGIN_DIR/logo.png" 2>/dev/null)
    echo -e "${GREEN}  ✓${NC} logo.png exists (${SIZE} bytes)"
else
    echo -e "${RED}  ✗${NC} logo.png not found"
    ((ERRORS++))
fi
echo ""

# Check index.html
echo -e "${BLUE}[3/6]${NC} Checking index.html..."
if [ -f "$PLUGIN_DIR/index.html" ]; then
    echo -e "${GREEN}  ✓${NC} index.html exists"
    
    # Check for required JavaScript functions
    if grep -q "getPlatform" "$PLUGIN_DIR/index.html"; then
        echo -e "${GREEN}  ✓${NC} Cross-platform detection found"
    else
        echo -e "${YELLOW}  ⚠${NC} Cross-platform detection missing"
        ((WARNINGS++))
    fi
else
    echo -e "${RED}  ✗${NC} index.html not found"
    ((ERRORS++))
fi
echo ""

# Check binaries
echo -e "${BLUE}[4/6]${NC} Checking platform binaries..."

# macOS
if [ -f "$PLUGIN_DIR/bin/darwin/cjxl" ]; then
    if [ -x "$PLUGIN_DIR/bin/darwin/cjxl" ]; then
        echo -e "${GREEN}  ✓${NC} macOS: cjxl found and executable"
    else
        echo -e "${YELLOW}  ⚠${NC} macOS: cjxl found but not executable"
        echo -e "      Run: chmod +x $PLUGIN_DIR/bin/darwin/cjxl"
        ((WARNINGS++))
    fi
else
    echo -e "${YELLOW}  ⚠${NC} macOS: cjxl not found"
    ((WARNINGS++))
fi

# Windows
if [ -f "$PLUGIN_DIR/bin/win32/cjxl.exe" ]; then
    echo -e "${GREEN}  ✓${NC} Windows: cjxl.exe found"
else
    echo -e "${YELLOW}  ⚠${NC} Windows: cjxl.exe not found"
    ((WARNINGS++))
fi

# Linux
if [ -f "$PLUGIN_DIR/bin/linux/cjxl" ]; then
    if [ -x "$PLUGIN_DIR/bin/linux/cjxl" ]; then
        echo -e "${GREEN}  ✓${NC} Linux: cjxl found and executable"
    else
        echo -e "${YELLOW}  ⚠${NC} Linux: cjxl found but not executable"
        echo -e "      Run: chmod +x $PLUGIN_DIR/bin/linux/cjxl"
        ((WARNINGS++))
    fi
else
    echo -e "${YELLOW}  ⚠${NC} Linux: cjxl not found (run ./setup.sh to download)"
    ((WARNINGS++))
fi
echo ""

# Check directory structure
echo -e "${BLUE}[5/6]${NC} Checking directory structure..."
REQUIRED_DIRS=("bin" "bin/darwin" "bin/win32" "bin/linux")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$PLUGIN_DIR/$dir" ]; then
        echo -e "${GREEN}  ✓${NC} $dir exists"
    else
        echo -e "${RED}  ✗${NC} $dir missing"
        ((ERRORS++))
    fi
done
echo ""

# Test binary (if on the current platform)
echo -e "${BLUE}[6/6]${NC} Testing binary on current platform..."
CURRENT_PLATFORM=""
case "$OSTYPE" in
    darwin*)  CURRENT_PLATFORM="darwin" ;;
    linux*)   CURRENT_PLATFORM="linux" ;;
    msys*|cygwin*) CURRENT_PLATFORM="win32" ;;
esac

if [ -n "$CURRENT_PLATFORM" ]; then
    BINARY_PATH="$PLUGIN_DIR/bin/$CURRENT_PLATFORM/cjxl"
    if [ "$CURRENT_PLATFORM" = "win32" ]; then
        BINARY_PATH="$BINARY_PATH.exe"
    fi
    
    if [ -f "$BINARY_PATH" ] && [ -x "$BINARY_PATH" ]; then
        if "$BINARY_PATH" --version &>/dev/null || "$BINARY_PATH" --help &>/dev/null; then
            echo -e "${GREEN}  ✓${NC} Binary test successful"
        else
            echo -e "${YELLOW}  ⚠${NC} Binary exists but may not work correctly"
            ((WARNINGS++))
        fi
    else
        echo -e "${YELLOW}  ⚠${NC} Cannot test: binary not found or not executable"
    fi
else
    echo -e "${YELLOW}  ⚠${NC} Unknown platform, cannot test binary"
fi
echo ""

# Summary
echo "=================================="
echo -e "${BLUE}Summary:${NC}"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed!${NC}"
    echo ""
    echo "The plugin is ready to be imported into Eagle."
    echo ""
    echo "Next steps:"
    echo "1. Open Eagle"
    echo "2. Go to Plugins → Developer → Import Local Plugin"
    echo "3. Select this directory: $PLUGIN_DIR"
    echo ""
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  ${WARNINGS} warning(s) found${NC}"
    echo ""
    echo "The plugin should work but may not support all platforms."
    echo "Run ./setup.sh to download missing binaries."
    echo ""
    exit 0
else
    echo -e "${RED}❌ ${ERRORS} error(s) and ${WARNINGS} warning(s) found${NC}"
    echo ""
    echo "Please fix the errors before importing the plugin into Eagle."
    echo ""
    exit 1
fi
