#!/bin/bash
# PIXLY 一键环境配置脚本 (macOS/Linux)
# 自动检测并安装所需依赖

set -e

echo "🚀 PIXLY 环境配置脚本"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检测操作系统
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=Linux;;
    Darwin*)    PLATFORM=macOS;;
    *)          PLATFORM="UNKNOWN";;
esac

echo "📋 检测到系统: ${PLATFORM}"
echo ""

# 检查是否有root权限
check_sudo() {
    if [[ "$PLATFORM" == "Linux" ]]; then
        if ! sudo -n true 2>/dev/null; then
            echo -e "${YELLOW}⚠️  需要管理员权限安装系统依赖${NC}"
            echo "   请在提示时输入密码"
            echo ""
        fi
    fi
}

# macOS安装函数
install_macos() {
    echo "🍎 macOS 环境配置"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # 检查Homebrew
    if ! command -v brew &> /dev/null; then
        echo -e "${YELLOW}📦 Homebrew 未安装，正在安装...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        
        # 配置Homebrew环境变量
        if [[ -f "/opt/homebrew/bin/brew" ]]; then
            eval "$(/opt/homebrew/bin/brew shellenv)"
        fi
    else
        echo -e "${GREEN}✅ Homebrew 已安装${NC}"
    fi
    echo ""
    
    # 安装ExifTool
    if ! command -v exiftool &> /dev/null; then
        echo -e "${BLUE}📥 正在安装 ExifTool...${NC}"
        brew install exiftool
        echo -e "${GREEN}✅ ExifTool 安装完成${NC}"
    else
        echo -e "${GREEN}✅ ExifTool 已安装 ($(exiftool -ver))${NC}"
    fi
    echo ""
    
    # 安装FFmpeg
    if ! command -v ffmpeg &> /dev/null; then
        echo -e "${BLUE}📥 正在安装 FFmpeg...${NC}"
        brew install ffmpeg
        echo -e "${GREEN}✅ FFmpeg 安装完成${NC}"
    else
        echo -e "${GREEN}✅ FFmpeg 已安装 ($(ffmpeg -version | head -n1))${NC}"
    fi
    echo ""
}

# Linux安装函数
install_linux() {
    echo "🐧 Linux 环境配置"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # 检测Linux发行版
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        DISTRO=$ID
    else
        DISTRO="unknown"
    fi
    
    echo "📋 检测到发行版: ${DISTRO}"
    echo ""
    
    check_sudo
    
    case "${DISTRO}" in
        ubuntu|debian)
            echo -e "${BLUE}📦 使用 apt 安装依赖...${NC}"
            sudo apt update
            
            if ! command -v exiftool &> /dev/null; then
                echo "📥 安装 ExifTool..."
                sudo apt install -y libimage-exiftool-perl
            fi
            
            if ! command -v ffmpeg &> /dev/null; then
                echo "📥 安装 FFmpeg..."
                sudo apt install -y ffmpeg
            fi
            ;;
        fedora|rhel|centos)
            echo -e "${BLUE}📦 使用 dnf 安装依赖...${NC}"
            
            if ! command -v exiftool &> /dev/null; then
                echo "📥 安装 ExifTool..."
                sudo dnf install -y perl-Image-ExifTool
            fi
            
            if ! command -v ffmpeg &> /dev/null; then
                echo "📥 安装 FFmpeg..."
                # 启用RPM Fusion
                sudo dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
                sudo dnf install -y ffmpeg
            fi
            ;;
        arch|manjaro)
            echo -e "${BLUE}📦 使用 pacman 安装依赖...${NC}"
            
            if ! command -v exiftool &> /dev/null; then
                echo "📥 安装 ExifTool..."
                sudo pacman -S --noconfirm perl-image-exiftool
            fi
            
            if ! command -v ffmpeg &> /dev/null; then
                echo "📥 安装 FFmpeg..."
                sudo pacman -S --noconfirm ffmpeg
            fi
            ;;
        *)
            echo -e "${RED}⚠️  未识别的Linux发行版: ${DISTRO}${NC}"
            echo "   请手动安装以下依赖："
            echo "   - exiftool (libimage-exiftool-perl)"
            echo "   - ffmpeg"
            exit 1
            ;;
    esac
    
    echo ""
    echo -e "${GREEN}✅ 依赖安装完成${NC}"
    echo ""
}

# 验证安装
verify_installation() {
    echo "🔍 验证安装..."
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    local all_ok=true
    
    # 检查ExifTool
    if command -v exiftool &> /dev/null; then
        echo -e "${GREEN}✅ ExifTool: $(exiftool -ver)${NC}"
    else
        echo -e "${RED}❌ ExifTool: 未安装${NC}"
        all_ok=false
    fi
    
    # 检查FFmpeg
    if command -v ffmpeg &> /dev/null; then
        echo -e "${GREEN}✅ FFmpeg: $(ffmpeg -version | head -n1 | cut -d' ' -f3)${NC}"
    else
        echo -e "${RED}❌ FFmpeg: 未安装${NC}"
        all_ok=false
    fi
    
    # 检查FFprobe
    if command -v ffprobe &> /dev/null; then
        echo -e "${GREEN}✅ FFprobe: $(ffprobe -version | head -n1 | cut -d' ' -f3)${NC}"
    else
        echo -e "${YELLOW}⚠️  FFprobe: 未安装 (通常随FFmpeg一起安装)${NC}"
    fi
    
    echo ""
    
    if $all_ok; then
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}🎉 所有依赖安装成功！${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo "📖 下一步："
        echo "   1. 重启 Eagle 应用"
        echo "   2. 启用 PIXLY 插件"
        echo "   3. 开始使用！"
        echo ""
        echo "💡 提示："
        echo "   - 运行 'pixly-rust --check-deps' 验证依赖"
        echo "   - 查看文档: https://pixly.app/docs"
        echo ""
    else
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${RED}❌ 部分依赖安装失败${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo "请检查上方错误信息，或访问："
        echo "https://pixly.app/docs/troubleshooting"
        exit 1
    fi
}

# 主流程
main() {
    case "${PLATFORM}" in
        macOS)
            install_macos
            ;;
        Linux)
            install_linux
            ;;
        *)
            echo -e "${RED}❌ 不支持的操作系统: ${PLATFORM}${NC}"
            exit 1
            ;;
    esac
    
    verify_installation
}

# 执行主流程
main
