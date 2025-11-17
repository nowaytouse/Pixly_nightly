# 📦 PIXLY插件依赖说明

## 核心依赖

### pixly-converter CLI ✅
- **位置**: `plugin/format/bin/pixly-converter`
- **状态**: 已包含在插件中
- **作用**: 核心转换引擎

## 外部工具（可选）

插件会根据需要调用以下外部工具。如果工具不存在，会显示清晰的安装指引。

### JXL格式支持

**工具**: `cjxl`, `djxl`

**安装方法**:
```bash
# macOS
brew install jpeg-xl

# Ubuntu/Debian
sudo apt install libjxl-tools

# Fedora/RHEL
sudo yum install libjxl-tools

# Windows
# 从 https://github.com/libjxl/libjxl/releases 下载
```

### AVIF格式支持

**工具**: `avifenc`, `avifdec`

**安装方法**:
```bash
# macOS
brew install libavif

# Ubuntu/Debian
sudo apt install libavif-bin

# Windows
# 从 https://github.com/AOMediaCodec/libavif/releases 下载
```

### WebP格式支持

**工具**: `cwebp`, `dwebp`

**安装方法**:
```bash
# macOS
brew install webp

# Ubuntu/Debian
sudo apt install webp

# Windows
# 从 https://developers.google.com/speed/webp/download 下载
```

### 视频/HEIC支持

**工具**: `ffmpeg`, `ffprobe`

**安装方法**:
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt install ffmpeg

# Windows
# 从 https://ffmpeg.org/download.html 下载
```

### 元数据支持（可选）

**工具**: `exiftool`

**安装方法**:
```bash
# macOS
brew install exiftool

# Ubuntu/Debian
sudo apt install libimage-exiftool-perl

# Windows
# 从 https://exiftool.org 下载
```

## 快速安装（macOS）

```bash
# 安装所有工具
brew install jpeg-xl libavif webp ffmpeg exiftool
```

## 快速安装（Ubuntu/Debian）

```bash
# 安装所有工具
sudo apt install libjxl-tools libavif-bin webp ffmpeg libimage-exiftool-perl
```

## 工具检测

插件会自动检测已安装的工具：
- ✅ 工具已安装 → 正常使用
- ⚠️ 工具未安装 → 显示安装指引

## 无需全部安装

你只需要安装你要使用的格式对应的工具：

- 只用JXL → 只装 `jpeg-xl`
- 只用WebP → 只装 `webp`
- 只用视频 → 只装 `ffmpeg`

## 验证安装

```bash
# 检查工具是否安装
which cjxl
which ffmpeg
which cwebp
which avifenc

# 测试转换
plugin/format/bin/pixly-converter convert test.png --format jxl --quality 90
```

## 常见问题

### Q: 为什么不把工具打包到插件中？

A: 因为：
1. 工具依赖动态库，简单复制不work
2. 不同操作系统需要不同的二进制
3. 工具体积大（ffmpeg ~100MB）
4. 用户可能已经安装了这些工具

### Q: 转换失败怎么办？

A: 检查错误信息：
- "cjxl not found" → 安装 jpeg-xl
- "ffmpeg not found" → 安装 ffmpeg
- "cwebp not found" → 安装 webp

### Q: 可以使用系统已安装的工具吗？

A: 可以！插件会自动使用系统PATH中的工具。

### Q: Windows上怎么办？

A: 
1. 下载工具的Windows版本
2. 添加到系统PATH
3. 或者放在插件bin目录中

---

**推荐**: 安装所有工具以获得完整功能！
