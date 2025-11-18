# 🚨 紧急修复指南

**问题**: 转换功能报错 "Image conversion failed"  
**原因**: pixly-converter 二进制文件未找到或不可执行

---

## 🔍 快速诊断

### 步骤1: 运行测试脚本

```bash
cd plugin/format-vue
node test-rust-cli.js
```

这会显示：
- 所有搜索路径
- 哪些路径存在文件
- 哪个路径可以执行

### 步骤2: 查看浏览器控制台

打开Eagle插件，按F12打开控制台，查找：
- `[PIXLY] Searching for pixly-converter` - 搜索日志
- `[PIXLY] Found file` - 找到文件
- `[PIXLY] Testing executable` - 测试执行
- `[PIXLY] ✅ Found pixly-converter` - 成功找到
- `[PIXLY ERROR]` - 错误信息

---

## ✅ 解决方案

### 方案1: 编译并放置二进制文件（推荐）

```bash
# 1. 编译Rust项目
cd /path/to/pixly
cargo build --release

# 2. 创建bin目录
mkdir -p plugin/format-vue/bin

# 3. 复制二进制文件
cp target/release/pixly-converter plugin/format-vue/bin/

# 4. 确保可执行
chmod +x plugin/format-vue/bin/pixly-converter

# 5. 测试
plugin/format-vue/bin/pixly-converter --version
```

### 方案2: 使用系统PATH

```bash
# 1. 编译
cd /path/to/pixly
cargo build --release

# 2. 复制到系统PATH
sudo cp target/release/pixly-converter /usr/local/bin/

# 3. 测试
pixly-converter --version
```

### 方案3: 创建符号链接

```bash
# 1. 编译
cd /path/to/pixly
cargo build --release

# 2. 创建符号链接
mkdir -p plugin/format-vue/bin
ln -s $(pwd)/target/release/pixly-converter plugin/format-vue/bin/pixly-converter

# 3. 测试
plugin/format-vue/bin/pixly-converter --version
```

---

## 🧪 验证修复

### 1. 命令行测试

```bash
# 测试版本
pixly-converter --version

# 测试转换（需要准备测试文件）
pixly-converter convert test.jpg --format jxl --quality 90
```

### 2. 插件测试

1. 重新加载Eagle插件
2. 选择一个图像文件
3. 选择JXL格式
4. 点击"开始转换"
5. 查看控制台日志

**预期日志**:
```
[PIXLY] Searching for pixly-converter
[PIXLY] Found file: /path/to/bin/pixly-converter
[PIXLY] Testing executable
[PIXLY] ✅ Found pixly-converter
[PIXLY] Rust CLI initialized
[PIXLY] Converting file
[pixly-converter] 🔄 Converting: input.jpg
[pixly-converter] ✅ Conversion complete!
```

---

## 🔧 常见问题

### Q1: "pixly-converter not found"

**原因**: 二进制文件不在搜索路径中

**解决**:
1. 确认文件存在：`ls -la plugin/format-vue/bin/pixly-converter`
2. 确认可执行：`chmod +x plugin/format-vue/bin/pixly-converter`
3. 测试执行：`plugin/format-vue/bin/pixly-converter --version`

### Q2: "Permission denied"

**原因**: 文件没有执行权限

**解决**:
```bash
chmod +x plugin/format-vue/bin/pixly-converter
```

### Q3: "No such file or directory"

**原因**: 文件路径错误或文件不存在

**解决**:
1. 检查文件是否存在
2. 使用绝对路径测试
3. 重新编译和复制

### Q4: 编译失败

**原因**: Rust环境问题

**解决**:
```bash
# 更新Rust
rustup update

# 清理并重新编译
cargo clean
cargo build --release
```

### Q5: "dyld: Library not loaded"（macOS）

**原因**: 缺少动态库

**解决**:
```bash
# 安装依赖
brew install jpeg-xl libavif ffmpeg

# 检查库路径
otool -L target/release/pixly-converter
```

---

## 📊 搜索路径优先级

插件按以下顺序搜索 `pixly-converter`:

1. `plugin/format-vue/bin/pixly-converter` ⭐ **推荐**
2. `plugin/bin/pixly-converter`
3. `bin/pixly-converter`
4. `target/release/pixly-converter`
5. `target/debug/pixly-converter`
6. 系统PATH中的 `pixly-converter`

**建议**: 使用路径1，将二进制文件放在插件目录中。

---

## 🚀 快速修复命令

```bash
# 一键修复（从项目根目录执行）
cd /path/to/pixly
cargo build --release && \
mkdir -p plugin/format-vue/bin && \
cp target/release/pixly-converter plugin/format-vue/bin/ && \
chmod +x plugin/format-vue/bin/pixly-converter && \
plugin/format-vue/bin/pixly-converter --version && \
echo "✅ 修复完成！"
```

---

## 📝 验证清单

修复后，确认以下项目：

- [ ] `pixly-converter --version` 可以执行
- [ ] 文件有执行权限（`-rwxr-xr-x`）
- [ ] 浏览器控制台显示 "✅ Found pixly-converter"
- [ ] 测试转换成功
- [ ] 输出文件正确生成

---

## 🆘 仍然无法解决？

### 收集诊断信息

```bash
# 1. 运行测试脚本
node plugin/format-vue/test-rust-cli.js > diagnostic.txt 2>&1

# 2. 检查文件
ls -la plugin/format-vue/bin/ >> diagnostic.txt

# 3. 测试执行
plugin/format-vue/bin/pixly-converter --version >> diagnostic.txt 2>&1

# 4. 查看诊断信息
cat diagnostic.txt
```

### 提供以下信息

1. 操作系统和版本
2. Rust版本：`rustc --version`
3. 项目路径
4. `diagnostic.txt` 内容
5. 浏览器控制台完整日志

---

**🔥 记住：二进制文件必须存在且可执行！这是转换功能工作的前提！**
