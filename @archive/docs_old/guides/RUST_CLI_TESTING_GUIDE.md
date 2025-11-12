# 🧪 Rust CLI独立测试指南

> **核心原则**: Rust执行端具备完全独立执行能力，无需依赖UI插件  
> **测试方式**: CLI命令行直接测试所有验证链路

## 🎯 测试目标

1. ✅ 验证Rust参数回显机制
2. ✅ 验证AI返回值验证
3. ✅ 验证输入/输出文件验证
4. ✅ 验证错误响亮报错
5. ✅ 验证参数完整性链路

## 📋 前置准备

### 1. 编译Rust CLI

```bash
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/rust

# 编译release版本
cargo build --release

# 或编译debug版本（带更多日志）
cargo build

# 验证编译成功
./target/release/pixly-rust --version
# 或
./target/debug/pixly-rust --version
```

### 2. 准备测试图像

```bash
# 创建测试目录
mkdir -p ~/pixly-test
cd ~/pixly-test

# 准备测试图像（复制一张现有图片）
cp ~/Pictures/test.jpg ./input.jpg

# 或下载测试图像
curl -o input.jpg https://via.placeholder.com/1920x1080.jpg
```

## 🧪 测试场景

### 场景1: 基础转换测试（验证参数回显）

#### 测试命令

```bash
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/rust

# AVIF转换
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85 \
  --speed 6

# JXL转换  
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.jxl \
  --format jxl \
  --quality 90 \
  --effort 7

# WebP转换
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.webp \
  --format webp \
  --quality 85 \
  --method 4
```

#### 期望输出

```
✅ 转换成功
📊 参数回显:
  - quality: 85
  - speed: 6
  - lossless: false
  - format: avif
  - strategy_type: CLI AVIF (avifenc)
  - params_source: user  ← 关键：用户参数未被修改
```

### 场景2: AI参数预测测试（验证AI验证）

```bash
# 使用analyze命令触发AI预测
./target/release/pixly-rust analyze \
  --input ~/pixly-test/input.jpg \
  --format jxl \
  --dry-run

# 期望：
# - AI预测参数
# - 置信度检查 (>= 0.5)
# - 参数范围验证
# - params_source: ai
```

### 场景3: 无效参数测试（验证响亮报错）

#### 测试3.1: 无效质量参数

```bash
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 150  # 超出范围

# 期望：
# ❌ Error: Quality参数超出范围: 150 (应为1-100)
# （立即终止，响亮报错）
```

#### 测试3.2: 无效速度参数

```bash
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85 \
  --speed 15  # 超出范围

# 期望：
# ❌ Error: Speed参数超出范围: 15 (应为0-10)
```

#### 测试3.3: 不存在的输入文件

```bash
./target/release/pixly-rust convert \
  --input ~/pixly-test/nonexistent.jpg \
  --output ~/pixly-test/output.avif \
  --format avif

# 期望：
# ❌ Error: 输入文件不存在: ~/pixly-test/nonexistent.jpg
```

#### 测试3.4: 不兼容的参数组合

```bash
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.jpg \
  --format jpeg \
  --lossless  # JPEG不支持无损

# 期望：
# ❌ Error: JPEG格式不支持无损模式
```

### 场景4: 文件类型检测测试

```bash
# 使用detect命令
./target/release/pixly-rust detect \
  --input ~/pixly-test/input.jpg

# 期望输出：
# ✅ 文件类型: image/jpeg
# ✅ 安全检查: 通过
# ✅ 格式匹配: 是
```

#### 测试伪装文件检测

```bash
# 创建伪装的PNG（实际是JPG）
cp ~/pixly-test/input.jpg ~/pixly-test/fake.png

./target/release/pixly-rust detect \
  --input ~/pixly-test/fake.png

# 期望：
# ⚠️ 警告: 文件扩展名不匹配
# 实际类型: image/jpeg
# 扩展名: .png
```

### 场景5: 批量转换测试

```bash
# 准备多张图片
cp ~/pixly-test/input.jpg ~/pixly-test/image1.jpg
cp ~/pixly-test/input.jpg ~/pixly-test/image2.jpg
cp ~/pixly-test/input.jpg ~/pixly-test/image3.jpg

# Eagle库批量优化
./target/release/pixly-rust eagle optimize \
  --library-path ~/Pictures/Eagle/Library.library \
  --quality 85 \
  --max-workers 4

# 期望：
# - 批量验证所有输入
# - 每个文件的参数回显
# - 统计报告
```

## 📊 验证检查点

### 必须验证的检查点

#### 1. 参数回显 ✅

```bash
# 执行转换后，输出应包含：
{
  "actualParams": {
    "quality": 85,
    "speed": 6,
    "lossless": false,
    "format": "avif",
    "preserve_metadata": true,
    "keep_animated": false,
    "strategy_type": "CLI AVIF (avifenc)",
    "params_source": "user"  ← 关键
  }
}
```

#### 2. AI参数验证 ✅

当AI返回置信度过低时：

```bash
# 日志应显示：
❌ [AI Validation] Confidence too low: 45.0% (threshold: 50%)
Error: AI置信度过低，拒绝使用
```

#### 3. 输入文件验证 ✅

```bash
# 应验证：
✅ 文件存在性
✅ 文件可读性
✅ Magika AI检测
✅ 安全性检查
```

#### 4. 输出文件验证 ✅

```bash
# 转换后应验证：
✅ 输出文件已生成
✅ 输出文件非空
✅ 输出格式正确
✅ 文件大小合理
```

#### 5. 错误响亮报错 ✅

```bash
# 所有错误都应：
❌ 立即终止流程
❌ 打印清晰错误消息
❌ 返回非零退出码
❌ 不使用默认值覆盖
```

## 🔧 调试模式

### 启用详细日志

```bash
# 设置日志级别
export RUST_LOG=debug

# 或在命令中指定
RUST_LOG=debug ./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85
```

### 查看参数流动

```bash
# 详细模式
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85 \
  --verbose  # 如果支持

# 应显示：
# 📥 接收参数: quality=85, speed=6
# 🤖 AI预测（如有）: confidence=0.85
# ✅ 参数验证通过
# 🔧 实际使用: quality=85, speed=6
# 📤 参数回显: params_source=user
```

## 🧪 自动化测试脚本

创建测试脚本：

```bash
#!/bin/bash
# test-rust-validation.sh

set -e

RUST_BIN="./target/release/pixly-rust"
TEST_DIR="$HOME/pixly-test"
TEST_IMAGE="$TEST_DIR/input.jpg"

echo "🧪 Rust验证链路测试"
echo "━━━━━━━━━━━━━━━━━━━━━━━━"

# 测试1: 正常转换
echo "📝 测试1: 正常转换（参数回显）"
$RUST_BIN convert \
  --input "$TEST_IMAGE" \
  --output "$TEST_DIR/test1.avif" \
  --format avif \
  --quality 85 \
  --speed 6
echo "✅ 测试1通过"
echo ""

# 测试2: 无效质量参数
echo "📝 测试2: 无效质量参数（响亮报错）"
if $RUST_BIN convert \
  --input "$TEST_IMAGE" \
  --output "$TEST_DIR/test2.avif" \
  --format avif \
  --quality 150 2>&1 | grep -q "超出范围"; then
  echo "✅ 测试2通过：正确拒绝无效参数"
else
  echo "❌ 测试2失败：未能拒绝无效参数"
  exit 1
fi
echo ""

# 测试3: 文件类型检测
echo "📝 测试3: 文件类型检测"
$RUST_BIN detect --input "$TEST_IMAGE"
echo "✅ 测试3通过"
echo ""

# 测试4: 不存在的文件
echo "📝 测试4: 不存在的文件（响亮报错）"
if $RUST_BIN convert \
  --input "$TEST_DIR/nonexistent.jpg" \
  --output "$TEST_DIR/test4.avif" \
  --format avif 2>&1 | grep -q "不存在"; then
  echo "✅ 测试4通过：正确报告文件不存在"
else
  echo "❌ 测试4失败：未能报告文件不存在"
  exit 1
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 所有测试通过！"
```

运行测试：

```bash
chmod +x test-rust-validation.sh
./test-rust-validation.sh
```

## 📈 性能测试

```bash
# 测试验证开销
time ./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85

# 期望：
# - 验证开销 < 10ms
# - 转换时间正常
# - 总体性能无明显影响
```

## ✅ 验证清单

完成以下测试后，验证链路通过：

- [ ] 基础转换成功，参数正确回显
- [ ] 参数来源标识正确（user/ai/hybrid）
- [ ] 无效参数被拒绝并响亮报错
- [ ] 文件类型检测正常工作
- [ ] AI置信度低时正确拒绝
- [ ] 输入文件验证生效
- [ ] 输出文件验证生效
- [ ] 参数组合冲突被检测
- [ ] 批量转换验证正常
- [ ] 所有错误消息清晰明确
- [ ] 无沉默失败场景
- [ ] 性能开销可接受

## 🐛 常见问题

### 问题1: 参数回显未显示

**症状**: 转换成功但未看到actualParams

**解决**:
```bash
# 检查输出格式
$RUST_BIN convert ... --output-format json

# 或检查日志
RUST_LOG=debug $RUST_BIN convert ...
```

### 问题2: AI验证失败但未报错

**症状**: AI返回无效参数但未被拒绝

**解决**: 检查validate_ai_response是否被调用
```bash
RUST_LOG=debug $RUST_BIN analyze ...
# 应看到: [AI Validation] checking response...
```

### 问题3: 验证开销过大

**症状**: 转换速度明显变慢

**解决**:
```bash
# 测量验证耗时
time $RUST_BIN convert ... --dry-run
# 验证逻辑应 < 10ms
```

## 📝 测试报告模板

```markdown
## Rust CLI验证测试报告

**测试日期**: 2025-11-10
**测试环境**: macOS / Rust 1.xx

### 测试结果

| 测试场景 | 状态 | 说明 |
|---------|------|------|
| 基础转换 | ✅ | 参数回显正常 |
| AI验证 | ✅ | 低置信度被拒绝 |
| 无效参数 | ✅ | 响亮报错 |
| 文件检测 | ✅ | Magika正常工作 |
| 批量转换 | ✅ | 验证链路完整 |

### 发现的问题

- 无

### 建议

- 验证链路工作正常
- 可投入生产使用
```

---

**核心价值**: Rust执行端完全独立，CLI测试验证所有链路！ 🚀
