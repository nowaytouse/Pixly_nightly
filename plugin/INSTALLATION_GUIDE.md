# 🚀 PIXLY插件安装指南

## 快速安装

### 方法1: 使用安装脚本（推荐）

```bash
# 在项目根目录运行
./install_plugin.sh
```

这个脚本会：
1. 构建Rust CLI
2. 创建必要的目录
3. 复制CLI到插件目录
4. 验证安装

### 方法2: 手动安装

```bash
# 1. 构建Rust CLI
cargo build --release --bin pixly-converter

# 2. 创建bin目录
mkdir -p plugin/format/bin
mkdir -p plugin/ai/bin

# 3. 复制CLI
cp target/release/pixly-converter plugin/format/bin/
cp target/release/pixly-converter plugin/ai/bin/

# 4. 验证
plugin/format/bin/pixly-converter --version
```

## 在Eagle中安装插件

### Format插件

1. 打开Eagle
2. 菜单：插件 → 开发者 → 安装本地插件
3. 选择 `plugin/format` 目录
4. 重启Eagle
5. 在Eagle中选择图片，打开PIXLY Format插件

### AI插件

1. 打开Eagle
2. 菜单：插件 → 开发者 → 安装本地插件
3. 选择 `plugin/ai` 目录
4. 重启Eagle
5. 在Eagle中选择图片，打开PIXLY AI插件

## 验证安装

### 检查CLI是否存在

```bash
# Format插件
ls -lh plugin/format/bin/pixly-converter

# AI插件
ls -lh plugin/ai/bin/pixly-converter
```

### 测试CLI

```bash
# 查看版本
plugin/format/bin/pixly-converter --version

# 测试转换
plugin/format/bin/pixly-converter convert test.png --format webp --quality 90
```

## 目录结构

```
Pixly_Nightly/
├── target/
│   └── release/
│       └── pixly-converter          # 源CLI
├── plugin/
│   ├── format/
│   │   ├── bin/
│   │   │   └── pixly-converter      # Format插件CLI
│   │   ├── js/
│   │   ├── css/
│   │   ├── index.html
│   │   └── manifest.json
│   └── ai/
│       ├── bin/
│       │   └── pixly-converter      # AI插件CLI
│       ├── js/
│       ├── css/
│       ├── index.html
│       └── manifest.json
└── install_plugin.sh                # 安装脚本
```

## CLI检测路径

插件会按以下顺序查找CLI：

1. `plugin/format/bin/pixly-converter` （插件内部，优先）
2. `plugin/bin/pixly-converter`
3. `../bin/pixly-converter`
4. `../../../target/release/pixly-converter` （开发环境）
5. `../../../target/debug/pixly-converter` （开发环境）
6. `pixly-converter` （系统PATH）

## 常见问题

### Q: 插件显示"Rust核心未找到"

**A**: 运行安装脚本：
```bash
./install_plugin.sh
```

或手动复制CLI：
```bash
cp target/release/pixly-converter plugin/format/bin/
```

### Q: CLI不存在

**A**: 先构建CLI：
```bash
cargo build --release --bin pixly-converter
```

### Q: 权限错误

**A**: 确保CLI有执行权限：
```bash
chmod +x plugin/format/bin/pixly-converter
chmod +x plugin/ai/bin/pixly-converter
```

### Q: 转换失败

**A**: 检查CLI是否可以独立运行：
```bash
plugin/format/bin/pixly-converter convert test.png --format webp --quality 90
```

### Q: 需要更新CLI

**A**: 重新构建并复制：
```bash
cargo build --release --bin pixly-converter
cp target/release/pixly-converter plugin/format/bin/
cp target/release/pixly-converter plugin/ai/bin/
```

## 开发模式

如果你在开发Rust代码，每次修改后需要：

```bash
# 1. 重新构建
cargo build --release --bin pixly-converter

# 2. 更新插件
cp target/release/pixly-converter plugin/format/bin/
cp target/release/pixly-converter plugin/ai/bin/

# 3. 重新加载Eagle插件
# 在Eagle中：插件 → 开发者 → 重新加载插件
```

或者使用安装脚本：
```bash
./install_plugin.sh
```

## 系统要求

- Rust 1.70+
- Eagle 4.0+
- macOS / Windows / Linux
- 外部工具（可选）：
  - cjxl (JXL转换)
  - ffmpeg (视频/AVIF/HEIC转换)
  - cwebp (WebP转换)

## 支持

如果遇到问题：

1. 检查CLI是否存在：`ls plugin/format/bin/pixly-converter`
2. 测试CLI：`plugin/format/bin/pixly-converter --version`
3. 查看Eagle控制台日志
4. 查看插件日志（开发者工具）

---

**安装完成后，享受PIXLY的强大功能！** 🎉
