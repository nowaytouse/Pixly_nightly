# 🚀 快速验证测试

> **5分钟快速验证Pixly参数验证链路**

## 🎯 目标

验证Phase 46.6-46.7实施的统一验证层是否正常工作：
- ✅ Rust参数回显
- ✅ AI返回值验证  
- ✅ 响亮报错机制
- ✅ 独立执行能力

## ⚡ 快速开始

### 方式1: 自动测试脚本（推荐）

```bash
# 进入Rust目录
cd core/rust

# 赋予执行权限
chmod +x quick-test.sh

# 运行快速测试
./quick-test.sh
```

脚本会自动：
1. 检查并编译Rust CLI
2. 查找或创建测试图像
3. 运行5个关键测试
4. 显示测试结果

### 方式2: 手动单个测试

```bash
# 编译
cd core/rust
cargo build --release

# 准备测试图像
mkdir -p ~/pixly-test
cp ~/Pictures/test.jpg ~/pixly-test/input.jpg

# 测试转换
./target/release/pixly-rust convert \
  --input ~/pixly-test/input.jpg \
  --output ~/pixly-test/output.avif \
  --format avif \
  --quality 85 \
  --speed 6

# 期望：✅ 转换成功，显示参数回显
```

## 📊 测试覆盖

| 测试项 | 验证内容 | 状态 |
|--------|---------|------|
| **正常转换** | 参数回显、actualParams | ✅ |
| **无效参数** | 响亮报错、不使用默认值 | ✅ |
| **文件不存在** | 输入验证、错误提示 | ✅ |
| **文件检测** | Magika AI、类型识别 | ✅ |
| **多格式支持** | AVIF/JXL/WebP | ✅ |

## 🔍 验证检查点

### 1. 参数回显验证

转换成功后，输出应包含：

```json
{
  "success": true,
  "actualParams": {
    "quality": 85,
    "speed": 6,
    "params_source": "user",  ← 关键：用户参数
    "strategy_type": "CLI AVIF"
  }
}
```

### 2. 响亮报错验证

无效参数时，应看到：

```
❌ Error: 质量参数超出范围: 150 (应为1-100)
```

**不应该**:
- ❌ 使用默认值（如quality=100）
- ❌ 静默忽略错误
- ❌ 继续执行转换

### 3. 文件验证

不存在的文件：

```
❌ Error: 输入文件不存在: ~/test/nonexistent.jpg
```

## 📖 详细文档

- **完整测试指南**: `docs/guides/RUST_CLI_TESTING_GUIDE.md`
- **架构设计**: `docs/architecture/UNIFIED_VALIDATION_ARCHITECTURE.md`
- **实施报告**: `docs/sessions/UNIFIED_VALIDATION_IMPLEMENTATION.md`

## 🐛 故障排查

### 编译失败

```bash
# 清理并重新编译
cargo clean
cargo build --release
```

### 测试图像缺失

```bash
# 从任意位置复制图片
cp /path/to/any-image.jpg ~/pixly-test/input.jpg
```

### 权限问题

```bash
# 确保脚本可执行
chmod +x core/rust/quick-test.sh
```

## ✅ 成功标志

当看到以下输出时，验证链路通过：

```
🎉 所有测试通过！验证链路正常工作！

✅ 参数回显机制: 正常
✅ 响亮报错机制: 正常
✅ 输入文件验证: 正常
✅ 文件类型检测: 正常
✅ 多格式支持: 正常
```

## 🚀 下一步

1. **Go AI服务测试**:
   ```bash
   cd core/go
   go run bin/ai-service/main.go --port 50052
   ```

2. **Eagle插件测试**:
   - 在Eagle中选择图片
   - 运行Pixly转换
   - 检查参数验证报告

3. **端到端测试**:
   - UI → Rust → Go AI 完整链路
   - 参数快照对比
   - 验证报告查看

---

**快速验证完成，开始享受可靠的参数验证系统！** 🎯
