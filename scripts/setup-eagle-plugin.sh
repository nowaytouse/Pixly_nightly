#!/bin/bash
# PIXLY Eagle插件环境配置
# 专为Eagle插件用户设计的一键配置脚本

set -e

echo "🦅 PIXLY Eagle插件环境配置"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 检测Eagle
detect_eagle() {
    echo "🔍 检测Eagle应用..."
    
    local eagle_found=false
    local eagle_path=""
    
    # macOS常见位置
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if [[ -d "/Applications/Eagle.app" ]]; then
            eagle_found=true
            eagle_path="/Applications/Eagle.app"
        fi
    fi
    
    if $eagle_found; then
        echo -e "${GREEN}✅ 找到Eagle: $eagle_path${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  未找到Eagle应用${NC}"
        echo "   请确保Eagle已安装到 /Applications/Eagle.app"
        return 1
    fi
}

# 检查依赖状态
check_dependencies() {
    echo ""
    echo "📋 检查依赖状态..."
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    local missing_deps=()
    
    # 检查ExifTool
    if command -v exiftool &> /dev/null; then
        echo -e "${GREEN}✅ ExifTool: $(exiftool -ver)${NC}"
    else
        echo -e "${RED}❌ ExifTool: 未安装${NC}"
        missing_deps+=("exiftool")
    fi
    
    # 检查FFmpeg
    if command -v ffmpeg &> /dev/null; then
        echo -e "${GREEN}✅ FFmpeg: $(ffmpeg -version | head -n1 | cut -d' ' -f3)${NC}"
    else
        echo -e "${YELLOW}⚠️  FFmpeg: 未安装 (推荐安装以支持GIF/视频功能)${NC}"
        missing_deps+=("ffmpeg")
    fi
    
    echo ""
    
    if [ ${#missing_deps[@]} -eq 0 ]; then
        echo -e "${GREEN}🎉 所有依赖已就绪！${NC}"
        return 0
    else
        return 1
    fi
}

# 自动安装依赖
auto_install() {
    echo ""
    echo -e "${CYAN}🤖 自动安装依赖${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # 检测操作系统
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        echo "📦 检测到 macOS，使用 Homebrew 安装..."
        echo ""
        
        # 检查Homebrew
        if ! command -v brew &> /dev/null; then
            echo -e "${BLUE}正在安装 Homebrew...${NC}"
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            
            # 配置环境变量
            if [[ -f "/opt/homebrew/bin/brew" ]]; then
                eval "$(/opt/homebrew/bin/brew shellenv)"
            fi
        fi
        
        # 安装ExifTool
        if ! command -v exiftool &> /dev/null; then
            echo -e "${BLUE}📥 安装 ExifTool...${NC}"
            brew install exiftool
        fi
        
        # 安装FFmpeg
        if ! command -v ffmpeg &> /dev/null; then
            echo -e "${BLUE}📥 安装 FFmpeg...${NC}"
            brew install ffmpeg
        fi
        
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        if [[ -f /etc/os-release ]]; then
            . /etc/os-release
            DISTRO=$ID
        fi
        
        echo "📦 检测到 Linux ($DISTRO)，使用包管理器安装..."
        echo ""
        
        case "$DISTRO" in
            ubuntu|debian)
                sudo apt update
                if ! command -v exiftool &> /dev/null; then
                    sudo apt install -y libimage-exiftool-perl
                fi
                if ! command -v ffmpeg &> /dev/null; then
                    sudo apt install -y ffmpeg
                fi
                ;;
            fedora|rhel|centos)
                if ! command -v exiftool &> /dev/null; then
                    sudo dnf install -y perl-Image-ExifTool
                fi
                if ! command -v ffmpeg &> /dev/null; then
                    sudo dnf install -y ffmpeg
                fi
                ;;
            arch|manjaro)
                if ! command -v exiftool &> /dev/null; then
                    sudo pacman -S --noconfirm perl-image-exiftool
                fi
                if ! command -v ffmpeg &> /dev/null; then
                    sudo pacman -S --noconfirm ffmpeg
                fi
                ;;
        esac
    fi
    
    echo ""
    echo -e "${GREEN}✅ 依赖安装完成${NC}"
}

# 生成配置指南
generate_guide() {
    echo ""
    echo -e "${CYAN}📖 Eagle插件使用指南${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "接下来的步骤："
    echo ""
    echo "1️⃣  重启Eagle应用"
    echo "   关闭并重新打开Eagle，让环境变量生效"
    echo ""
    echo "2️⃣  启用PIXLY插件"
    echo "   Eagle > 扩展 > PIXLY > 启用"
    echo ""
    echo "3️⃣  验证依赖"
    echo "   插件会自动检测依赖状态"
    echo "   如果看到绿色✅标记，说明一切正常"
    echo ""
    echo "4️⃣  开始使用"
    echo "   选择图片/GIF > 启动PIXLY > 选择转换模式"
    echo ""
    echo -e "${YELLOW}💡 提示：${NC}"
    echo "   - 首次使用建议选择'智能模式'"
    echo "   - GIF/视频信息面板需要FFmpeg支持"
    echo "   - 遇到问题查看: https://pixly.app/docs"
    echo ""
}

# 测试Eagle环境
test_eagle_env() {
    echo ""
    echo -e "${CYAN}🧪 测试Eagle环境${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # 检查Rust CLI
    local rust_cli_paths=(
        "./core/rust/target/release/pixly-rust"
        "../rust/target/release/pixly-rust"
        "./pixly-rust"
    )
    
    local rust_cli_found=false
    for path in "${rust_cli_paths[@]}"; do
        if [[ -f "$path" ]]; then
            echo -e "${GREEN}✅ 找到 Rust CLI: $path${NC}"
            
            # 测试执行
            if "$path" --version &> /dev/null; then
                local version=$("$path" --version 2>/dev/null)
                echo -e "${GREEN}   版本: $version${NC}"
            fi
            
            # 测试依赖检测
            echo ""
            echo "运行依赖检测..."
            "$path" --check-deps
            
            rust_cli_found=true
            break
        fi
    done
    
    if ! $rust_cli_found; then
        echo -e "${YELLOW}⚠️  未找到 Rust CLI 二进制文件${NC}"
        echo "   请确保已编译 Rust 内核："
        echo "   cd core/rust && cargo build --release"
    fi
}

# 主流程
main() {
    # 检测Eagle
    detect_eagle
    
    # 检查依赖
    if ! check_dependencies; then
        echo ""
        echo -e "${YELLOW}检测到缺失依赖${NC}"
        echo ""
        read -p "是否自动安装？ (y/n) " -n 1 -r
        echo ""
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            auto_install
            echo ""
            check_dependencies
        else
            echo ""
            echo -e "${YELLOW}请手动安装依赖后重新运行此脚本${NC}"
            echo ""
            echo "macOS:"
            echo "  brew install exiftool ffmpeg"
            echo ""
            echo "Linux (Ubuntu/Debian):"
            echo "  sudo apt install libimage-exiftool-perl ffmpeg"
            echo ""
            exit 0
        fi
    fi
    
    # 测试Eagle环境
    test_eagle_env
    
    # 生成配置指南
    generate_guide
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${GREEN}🎉 配置完成！${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# 运行主流程
main
