# 👁️ PIXLY 预览插件 - Eagle专用

[![版本](https://img.shields.io/badge/版本-2.1.0-green.svg)](https://eagle.cool/)
[![许可证](https://img.shields.io/badge/许可证-MIT-green.svg)](../LICENSE)

为 [Eagle App](https://eagle.cool/) 设计的 **JXL** 和 **AVIF** 图像原生预览插件。支持动画图像，集成FFplay播放器。

[English Documentation](README.md) | [主项目](../README.md)

---

## ✨ 核心功能

### 🖼️ 格式支持
- **JXL (JPEG XL)** - 新一代图像格式
- **AVIF** - 现代压缩格式
- **动画图像** - 检测并播放动画
- **缩略图生成** - Eagle资料库缩略图

### 🎬 动画支持
- **自动检测** - 自动识别动画图像
- **帧信息** - 显示帧数、时长、FPS
- **FFplay集成** - 外部播放器流畅播放
- **降级渲染** - 播放失败时显示静态预览

### 🔧 实用功能
- **图像尺寸** - 显示分辨率
- **文件大小** - 显示压缩后体积
- **格式信息** - 技术详情
- **快速预览** - 快速加载

---

## 📦 安装

### 方式1：Eagle插件商店（推荐）
1. 打开Eagle
2. 前往 **偏好设置** → **插件**
3. 搜索"PIXLY 预览"
4. 点击 **安装**

### 方式2：手动安装
1. 从 [Releases](https://github.com/yourusername/plxy-easy2jxlavif/releases) 下载
2. 解压到Eagle插件文件夹：
   - macOS: `~/Library/Application Support/Eagle/plugins/`
   - Windows: `%APPDATA%/Eagle/plugins/`
3. 重启Eagle

### 方式3：开发安装
```bash
# 克隆仓库
git clone https://github.com/yourusername/plxy-easy2jxlavif.git

# 链接插件文件夹
ln -s /path/to/plxy-easy2jxlavif/plugin_preview ~/Library/Application\ Support/Eagle/plugins/pixly-preview

# 重启Eagle
```

---

## 🚀 快速开始

### 自动预览
1. 导入JXL或AVIF图像到Eagle
2. 点击任意图像预览
3. PIXLY预览自动处理渲染

### 播放动画
1. 打开动态JXL或AVIF
2. 插件自动检测动画
3. 点击 **▶️ 使用FFplay播放** 按钮
4. 外部播放器打开并流畅播放

### 缩略图生成
- JXL和AVIF缩略图自动生成
- 缩略图显示在Eagle资料库网格中
- 快速加载并缓存

---

## 🛠️ 依赖工具

### 必需工具

#### JXL预览所需
```bash
# 安装 djxl（JPEG XL解码器）
brew install jpeg-xl  # macOS
sudo apt install libjxl-tools  # Ubuntu

# 安装 ffprobe（动画检测）
brew install ffmpeg  # macOS
sudo apt install ffmpeg  # Ubuntu
```

#### AVIF预览所需
```bash
# 安装 avifdec（AVIF解码器）
brew install libavif  # macOS
sudo apt install libavif-bin  # Ubuntu

# 安装 ffprobe（动画检测）
brew install ffmpeg  # macOS
sudo apt install ffmpeg  # Ubuntu
```

#### 动画播放所需
```bash
# 安装 ffplay（视频播放器）
brew install ffmpeg  # macOS（包含ffplay）
sudo apt install ffmpeg  # Ubuntu
```

---

## ⚙️ 配置

### 查看器设置

**JXL查看器** (`viewer/jxl.html`)：
- 通过 `ffprobe` 自动检测动画
- 使用 `djxl` 解码图像
- 显示静态预览
- 为动画提供FFplay播放选项

**AVIF查看器** (`viewer/avif.html`)：
- 通过 `avifdec --info` 自动检测动画
- 从时间信息计算动态FPS
- 显示静态预览
- 为动画提供FFplay播放选项

### 缩略图设置

**JXL缩略图** (`thumbnail/jxl.js`)：
- 生成400x400缩略图
- 缓存在Eagle的缩略图目录
- 解码失败时降级到默认图标

**AVIF缩略图** (`thumbnail/avif.js`)：
- 生成400x400缩略图
- 缓存在Eagle的缩略图目录
- 解码失败时降级到默认图标

---

## 🎯 使用场景

### 摄影资料库
```
1. 导入JXL照片到Eagle
2. 在资料库中浏览缩略图预览
3. 点击查看全分辨率预览
4. 动态JXL显示帧数和播放按钮
```

### 网页资源管理
```
1. 下载AVIF网页资源
2. 导入到Eagle进行整理
3. 直接在Eagle中预览（无需转换）
4. 根据需要分享或导出
```

### 动画收藏
```
1. 收集动态AVIF/JXL文件
2. 在Eagle中使用预览插件查看
3. 点击播放按钮观看流畅播放
4. FFplay以窗口模式打开（800x600）
```

---

## 🐛 故障排除

### 预览显示"无法加载JXL/AVIF文件"
- **检查工具**：确认 `djxl` 或 `avifdec` 已安装
- **检查PATH**：确保工具在系统PATH中
- **文件损坏**：尝试在其他查看器中打开
- **文件扩展名**：确保文件有正确的扩展名（.jxl 或 .avif）

### 动画未被检测
- **检查ffprobe**：`which ffprobe`
- **检查格式**：某些旧AVIF文件可能无法正确报告帧数
- **手动检查**：使用 `avifdec --info file.avif` 验证

### FFplay按钮未显示
- **安装ffplay**：`brew install ffmpeg`（包含ffplay）
- **检查PATH**：`which ffplay`
- **重启Eagle**：安装后重新加载插件

### FFplay全屏或在后台打开
- **当前行为**：以800x600窗口模式打开，前台显示
- **问题持续**：检查ffplay版本（`ffplay -version`）
- **替代方案**：使用系统默认视频播放器

### 缩略图未生成
- **检查权限**：Eagle需要缩略图目录的读写权限
- **检查工具**：确认解码器已安装
- **清除缓存**：删除Eagle缩略图缓存并重新加载

---

## 💡 提示与最佳实践

### 获得最佳性能
- 保持CLI工具更新到最新版本
- 使用SSD存储Eagle资料库（解码更快）
- 定期清除缩略图缓存

### 动画图像
- FFplay提供最流畅的播放
- 静态预览显示第一帧
- 状态栏显示帧数和时长

### 大型收藏
- 缩略图会被缓存（只生成一次）
- 首次加载后预览很快
- FFplay播放即时（无需预处理）

---

## 🔧 高级功能

### 自定义查看器配置

编辑查看器HTML文件自定义行为：
- `viewer/jxl.html` - JXL查看器设置
- `viewer/avif.html` - AVIF查看器设置

### 自定义缩略图尺寸

编辑缩略图JavaScript文件：
- `thumbnail/jxl.js` - JXL缩略图生成器
- `thumbnail/avif.js` - AVIF缩略图生成器

修改 `targetSize` 为所需分辨率：
```javascript
const targetSize = 400; // 默认：400x400
```

### FFplay参数

当前FFplay命令：
```bash
ffplay -x 800 -y 600 -alwaysontop <文件>
```

要自定义，编辑 `viewer/*.html` 并修改 `playWithFFplay()` 函数。

---

## 📚 相关文档

- [主项目README](../README.md)
- [转换插件](../plugin_v3/README.md)
- [AI服务](../cmd/ai-service/README.md)
- [Rust服务](../pixly-rust/README.md)

---

## 📄 许可证

MIT许可证 - 详见 [LICENSE](../LICENSE)。

---

**为Eagle制作** | [PIXLY项目](../README.md)的一部分
