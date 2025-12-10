# 快速开始指南 / Quick Start Guide

## 中文版

### 1. 安装插件

#### 自动验证和设置
```bash
cd /path/to/jpeg2jxl-vue
./validate.sh    # 验证插件完整性
./setup.sh       # 下载缺失的二进制文件（如需要）
```

#### 导入到 Eagle
1. 打开 Eagle 应用
2. 点击菜单：**插件 → 开发者 → 导入本地插件**
3. 选择 `jpeg2jxl-vue` 文件夹
4. 等待导入完成

### 2. 使用插件

1. 在 Eagle 中选择一个或多个 JPEG 图片
2. 打开插件：**插件 → JPEG to JXL Converter**
3. 点击 **"Convert to JXL"** 按钮
4. 等待转换完成

✅ 转换后的 `.jxl` 文件将保存在原始文件的同一目录中

### 3. 查看结果

- JXL 文件通常比原始 JPEG 小 20-50%
- 完全无损，保持原始图片的所有质量
- 可以使用支持 JXL 的查看器或浏览器查看

### 4. 故障排除

**如果遇到问题**：

```bash
# 运行验证脚本检查
./validate.sh

# 如果 Linux 平台缺少二进制文件
./setup.sh

# 查看详细文档
cat README.md
```

**常见问题**：
- ❌ "cjxl binary not found" → 运行 `./setup.sh`
- ❌ 插件无法导入 → 检查 `manifest.json` 和 `logo.png` 是否存在
- ❌ 转换失败 → 在 Eagle 中启用开发者工具查看控制台

---

## English Version

### 1. Install Plugin

#### Automated Validation and Setup
```bash
cd /path/to/jpeg2jxl-vue
./validate.sh    # Validate plugin integrity
./setup.sh       # Download missing binaries (if needed)
```

#### Import to Eagle
1. Open Eagle app
2. Go to: **Plugins → Developer → Import Local Plugin**
3. Select the `jpeg2jxl-vue` folder
4. Wait for import to complete

### 2. Use Plugin

1. Select one or more JPEG images in Eagle
2. Open plugin: **Plugins → JPEG to JXL Converter**
3. Click **"Convert to JXL"** button
4. Wait for conversion to complete

✅ Converted `.jxl` files will be saved in the same directory as the source files

### 3. View Results

- JXL files are typically 20-50% smaller than original JPEGs
- Completely lossless, preserving all original image quality
- View using JXL-supported viewers or browsers

### 4. Troubleshooting

**If you encounter issues**:

```bash
# Run validation script
./validate.sh

# If Linux binary is missing
./setup.sh

# View detailed documentation
cat README.md
```

**Common Issues**:
- ❌ "cjxl binary not found" → Run `./setup.sh`
- ❌ Plugin cannot be imported → Check if `manifest.json` and `logo.png` exist
- ❌ Conversion fails → Enable dev tools in Eagle to view console

---

## 支持的平台 / Supported Platforms

| Platform | Status | Binary Path |
|----------|--------|-------------|
| 🍎 macOS | ✅ Ready | `bin/darwin/cjxl` |
| 🪟 Windows | ✅ Ready | `bin/win32/cjxl.exe` |
| 🐧 Linux | ⚠️ Manual Setup | `bin/linux/cjxl` |

## 文件结构 / File Structure

```
jpeg2jxl-vue/
├── manifest.json       # Eagle 插件配置
├── index.html          # 插件界面和逻辑
├── logo.png           # 插件图标
├── package.json       # 项目元数据
├── README.md          # 详细文档
├── CHANGELOG.md       # 版本历史
├── QUICKSTART.md      # 本文件
├── setup.sh          # 自动安装脚本
├── validate.sh       # 验证脚本
└── bin/              # 二进制文件目录
    ├── darwin/       # macOS
    ├── win32/        # Windows
    └── linux/        # Linux
```

## 获取帮助 / Get Help

- 📖 详细文档: `README.md`
- 🔍 验证插件: `./validate.sh`
- ⚙️ 安装设置: `./setup.sh`
- 🐛 报告问题: 在项目仓库中提交 Issue
