# PIXLY 自动环境配置脚本

## 📋 概述

这些脚本用于自动安装和配置PIXLY所需的外部依赖（ExifTool、FFmpeg等），支持多种操作系统和使用场景。

## 🚀 快速开始

### macOS / Linux

#### 方式1：在线一键安装（推荐）
```bash
bash <(curl -fsSL https://pixly.app/install.sh)
```

#### 方式2：下载脚本后执行
```bash
# 下载脚本
curl -O https://pixly.app/scripts/install.sh
chmod +x install.sh

# 运行安装
./install.sh
```

#### 方式3：Eagle插件专用配置
```bash
# 在Pixly项目目录下运行
cd /path/to/Pixly_Nightly
./scripts/setup-eagle-plugin.sh
```

### Windows

#### 方式1：在线一键安装（推荐）
```powershell
# 以管理员身份运行PowerShell
powershell -c "irm https://pixly.app/install.ps1 | iex"
```

#### 方式2：下载脚本后执行
```powershell
# 下载脚本
Invoke-WebRequest -Uri https://pixly.app/scripts/install.ps1 -OutFile install.ps1

# 以管理员身份运行PowerShell，然后执行
.\install.ps1
```

---

## 📁 脚本说明

### 1. `install.sh` - macOS/Linux通用安装脚本

**功能：**
- 自动检测操作系统和发行版
- 安装/更新 Homebrew（macOS）或包管理器（Linux）
- 安装 ExifTool 和 FFmpeg
- 验证安装并显示版本信息

**支持的系统：**
- macOS (Intel / Apple Silicon)
- Ubuntu / Debian
- Fedora / RHEL / CentOS
- Arch Linux / Manjaro

**使用示例：**
```bash
./install.sh
```

---

### 2. `install.ps1` - Windows PowerShell安装脚本

**功能：**
- 自动下载并安装 ExifTool
- 自动下载并安装 FFmpeg
- 配置系统PATH环境变量
- 验证安装

**要求：**
- Windows 10/11
- PowerShell 5.0+
- **管理员权限**

**使用示例：**
```powershell
# 右键PowerShell → 以管理员身份运行
.\install.ps1
```

---

### 3. `setup-eagle-plugin.sh` - Eagle插件专用配置

**功能：**
- 检测Eagle应用安装
- 智能检查依赖状态
- 提供一键安装选项
- 测试Rust CLI环境
- 生成详细使用指南

**适用场景：**
- Eagle插件用户首次配置
- 验证插件环境
- 排查依赖问题

**使用示例：**
```bash
cd /path/to/Pixly_Nightly
./scripts/setup-eagle-plugin.sh
```

---

## 🔧 手动安装（备选方案）

### macOS

```bash
# 安装Homebrew（如果未安装）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装依赖
brew install exiftool ffmpeg
```

### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install libimage-exiftool-perl ffmpeg
```

### Linux (Fedora/RHEL)

```bash
sudo dnf install perl-Image-ExifTool ffmpeg
```

### Linux (Arch)

```bash
sudo pacman -S perl-image-exiftool ffmpeg
```

### Windows

1. **ExifTool:**
   - 下载: https://exiftool.org/
   - 解压到 `C:\exiftool\exiftool.exe`
   - 添加到PATH环境变量

2. **FFmpeg:**
   - 下载: https://ffmpeg.org/download.html#build-windows
   - 解压到 `C:\ffmpeg\bin\ffmpeg.exe`
   - 添加到PATH环境变量

---

## ✅ 验证安装

### 命令行验证

```bash
# 检查ExifTool
exiftool -ver

# 检查FFmpeg
ffmpeg -version

# 使用Rust CLI检查（在项目目录下）
./core/rust/target/release/pixly-rust --check-deps
```

### Eagle插件验证

1. 重启Eagle应用
2. 启动PIXLY插件
3. 查看依赖状态面板（应显示绿色✅标记）

---

## 🐛 常见问题

### Q1: 脚本执行权限被拒绝

**A:** 添加执行权限
```bash
chmod +x install.sh
```

### Q2: macOS提示"无法验证开发者"

**A:** 在系统偏好设置 > 安全性与隐私中允许运行

### Q3: Windows PowerShell执行策略限制

**A:** 以管理员身份运行PowerShell，然后执行：
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Q4: Linux需要sudo密码

**A:** 某些包管理器命令需要管理员权限，脚本会在需要时提示输入密码

### Q5: Homebrew安装缓慢（中国大陆）

**A:** 可以使用镜像源：
```bash
# 使用清华大学镜像
export HOMEBREW_BREW_GIT_REMOTE="https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git"
```

### Q6: FFmpeg下载失败

**A:** 手动从官网下载后放置到指定位置：
- macOS: `/opt/homebrew/bin/ffmpeg`
- Windows: `C:\ffmpeg\bin\ffmpeg.exe`
- Linux: `/usr/bin/ffmpeg`

---

## 📊 依赖说明

| 依赖 | 必需性 | 用途 | 大小 |
|------|--------|------|------|
| **ExifTool** | ✅ 必需 | 元数据处理（EXIF/XMP/ICC） | ~10MB |
| **FFmpeg** | ⚠️ 推荐 | 视频转换、GIF分析 | ~100MB |
| **FFprobe** | ⚠️ 推荐 | 视频信息获取（随FFmpeg安装） | 包含在FFmpeg中 |

---

## 🔗 相关链接

- **PIXLY文档:** https://pixly.app/docs
- **故障排查:** https://pixly.app/docs/troubleshooting
- **GitHub仓库:** https://github.com/nowaytouse/Pixly_nightly
- **ExifTool官网:** https://exiftool.org/
- **FFmpeg官网:** https://ffmpeg.org/

---

## 📝 更新日志

### v1.0.0 (2024-11-09)
- ✨ 初始发布
- ✅ 支持 macOS / Linux / Windows
- ✅ 智能依赖检测
- ✅ 自动PATH配置
- ✅ Eagle插件专用脚本

---

## 💬 获取帮助

遇到问题？

1. 查看 [故障排查文档](https://pixly.app/docs/troubleshooting)
2. 运行 `pixly-rust --check-deps` 查看详细状态
3. 提交 [GitHub Issue](https://github.com/nowaytouse/Pixly_nightly/issues)

---

## 📄 许可证

这些脚本遵循 MIT 许可证，可自由使用和修改。
