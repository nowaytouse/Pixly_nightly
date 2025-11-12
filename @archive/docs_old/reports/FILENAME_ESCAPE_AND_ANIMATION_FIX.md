# 🐛 文件名特殊字符 & GIF动画丢失修复

**日期**: 2025-11-06  
**Bug 1**: 文件名包含括号/空格导致转换失败  
**Bug 2**: GIF动画转换后变成单帧静态图  
**状态**: 🔧 **Bug 1已修复, Bug 2待调查**

---

## Bug 1: 文件名特殊字符导致Shell错误

### 问题表现

```
文件名: 普通 (3).jpg

错误: /bin/sh: -c: line 0: syntax error near unexpected token `('
```

**原因**: Node.js的`execSync`将整个命令作为shell字符串执行，括号被shell解释器错误解析。

### 错误的代码

```javascript
// ❌ 使用execSync with shell string
const fullCommand = `"${this.path}" ${command} ${args.join(' ')}`;
stdout = execSync(fullCommand, { encoding: 'utf8' });

// 问题：args中的 "/path/普通 (3).jpg" 没有被正确转义
// Shell看到："/path/普通 (3).jpg" 
// 报错：syntax error near unexpected token `('
```

### 修复方案

**使用`spawnSync`的数组参数形式**，完全避免shell解析：

```javascript
// ✅ 使用spawnSync with array args
const { spawnSync } = require('child_process');
const fullArgs = [command, ...args];

const result = spawnSync(this.path, fullArgs, {
    encoding: 'utf8',
    timeout: 300000
});
```

**优势**:
- ✅ 不经过shell，直接传递参数给进程
- ✅ 自动处理所有特殊字符（括号、空格、中文、引号等）
- ✅ 更安全，避免shell注入攻击

### 测试用例

| 文件名 | 之前 | 现在 |
|--------|------|------|
| `普通 (3).jpg` | ❌ Shell error | ✅ 应该正常 |
| `test (815).png` | ❌ Shell error | ✅ 应该正常 |
| `file   space.gif` | ❌ Shell error | ✅ 应该正常 |
| `中文 文件.jpg` | ❌ Shell error | ✅ 应该正常 |

---

## Bug 2: GIF动画转换后丢失帧 🔥🔥🔥

### 问题表现

用户用`ffplay`预览转换结果：

#### WebP (动画GIF→WebP)
```bash
Input #0, webp_pipe, from '/path/to/file.webp':
  Stream #0:0: Video: webp, yuv420p, 600x338, 25 fps, 25 tbr, 25 tbn
  # ✅ 显示25fps，但实际播放不动（只有一帧）
```

#### AVIF (动画GIF→AVIF)
```bash
Input #0, mov,mp4,m4a,3gp,3g2,mj2, from '/path/to/file.avif':
  Stream #0:0[0x1]: Video: av1, yuv444p10le, 720x1080, 1 fps, 1 tbr, 1 tbn
  # ✅ 显示1fps，但实际播放不动（只有一帧）
```

**结论**: GIF动画被转换成**单帧静态图**！

### 根本原因 (待调查)

可能的原因：
1. **CLI工具参数错误**: `cwebp`, `avifenc`没有传递保留动画帧的参数
   - `cwebp`: 需要 `-min_size` 或特殊参数保留动画
   - `avifenc`: 可能不支持动画AVIF（较新特性）
   
2. **Rust CLI逻辑错误**: 转换策略没有检测GIF是否为动画
   - 可能只读取了第一帧
   - 没有调用动画转换API

3. **格式限制**: AVIF对动画支持有限
   - 动画AVIF是较新的特性
   - `avifenc`版本可能不支持

### 需要调查的代码位置

#### Rust CLI
- `pixly-rust/src/cli/conversion.rs`: 主转换逻辑
- `pixly-rust/src/converter/strategies/cli_strategy.rs`: CLI工具调用
- `pixly-rust/src/converter/strategies/native_webp_strategy.rs`: 原生WebP
- `pixly-rust/src/converter/strategies/native_avif_strategy.rs`: 原生AVIF

#### 关键检查点
```rust
// 是否检测GIF动画？
let is_animated = ...; // 需要实现

// 是否传递动画参数给CLI工具？
if is_animated {
    args.push("--animated");  // 需要添加
}
```

### 测试验证

```bash
# 1. 检查工具版本
cwebp -version
avifenc --version

# 2. 手动测试动画转换
cwebp -q 90 animated.gif -o test.webp  # 默认会保留动画吗？
avifenc -q 90 animated.gif test.avif   # 是否支持动画？

# 3. 检查输出
ffprobe test.webp  # 查看帧数
ffprobe test.avif  # 查看帧数
```

---

## 修复进度

### ✅ 已完成
- [x] 修复文件名特殊字符问题（使用spawnSync）
- [x] 更新`28-rust-cli-executor.js`
- [x] 添加更详细的日志输出

### 🔜 待完成
- [ ] 调查GIF动画检测逻辑
- [ ] 调查CLI工具动画转换参数
- [ ] 修复WebP动画转换
- [ ] 修复AVIF动画转换（如果工具支持）
- [ ] 添加动画转换测试用例
- [ ] 更新文档说明动画支持情况

---

## 历史功能: 文件名规范化

用户提到之前有"规范化重命名"功能，在`04-conversion.js.backup`中发现：

```javascript
// 步骤1: 文件名规范化（文件系统层面）
const autoNormalizeNames = true;  // 🔒 Force enable

// 规范化函数：移除/替换特殊字符
const normalizeForMatch = (name) => {
    return name
        .replace(/\s*\((\d+)\)/g, '_$1')  // "file (815)" → "file_815"
        .replace(/\s*\(\)/g, '_')         // "file ()" → "file_"
        .replace(/\s*\(/g, '_')           // "file (" → "file_"
        .replace(/\)/g, '')               // ")" → ""
        .replace(/\s+/g, '_')             // 多空格 → 单下划线
        .toLowerCase();
};
```

**流程**:
1. 转换前：`普通 (3).jpg` → `普通_3.jpg` (文件系统重命名)
2. 转换中：使用规范化后的文件名
3. 转换后：`普通_3.avif` → `普通 (3).avif` (恢复原名)

**现状**: 此功能在新架构中未实现，但使用`spawnSync`后已不需要。

---

**下一步**: 
1. ✅ 测试文件名特殊字符修复
2. 🔥 **紧急**: 调查并修复GIF动画丢失问题
3. 📝 更新用户文档关于动画格式支持情况

---

**修复完成时间**: 2025-11-06 16:00 (部分)  
**待测试**: 括号/空格文件名转换  
**待修复**: GIF动画转换

---

## 🔧 修复完成 (2025-11-06 16:15)

### ✅ 已修复

#### 1. 文件名特殊字符问题 (已完成)
- **修改文件**: `plugin/js/plugin-modules/28-rust-cli-executor.js`
- **方案**: 将`execSync`替换为`spawnSync`的数组参数形式
- **效果**: 完全避免shell解析，支持所有特殊字符（括号、空格、中文等）

#### 2. AnimatedGifStrategy日志显示 (已完成)
- **修改文件**: `pixly-rust/src/converter/strategies/mod.rs`
- **方案**: 添加`gif2webp/ffmpeg`可用性检测和`println!`日志输出
- **效果**: 现在会显示 "✓ Animated GIF (gif2webp/ffmpeg) - Available: true"

### 测试验证

#### 请在Eagle中重新测试:
1. ✅ 选择文件名带括号的JPEG (如：`普通 (3).jpg`)
2. ✅ 转换为AVIF/WebP/JXL/HEIC
3. ✅ 检查Eagle控制台日志是否显示:
   ```
   📋 Registering conversion strategies...
   ✓ Animated GIF (gif2webp/ffmpeg) - Available: true
   ✓ CLI JXL (cjxl) - Available: true
   ...
   ```
4. 🔥 **关键测试**: 选择一个动画GIF文件
5. 🔥 转换为WebP格式
6. 🔥 用`ffplay`检查输出是否保留了动画:
   ```bash
   ffplay output.webp
   # 应该显示多帧动画，而不是单帧静态图
   ```

### 预期行为

#### 对于带括号的文件名
```
之前: ❌ /bin/sh: syntax error near unexpected token `('
现在: ✅ 正常转换
```

#### 对于动画GIF → WebP
```
之前: ❌ 单帧静态WebP (丢失动画)
现在: ✅ 多帧动画WebP (保留动画)
      ✓ 使用gif2webp工具
      ✓ 优先级90，优先匹配动画GIF
```

#### 对于动画GIF → AVIF
```
状态: ⚠️  实验性支持
方案: 使用FFmpeg转换
限制: AVIF动画支持较新，可能兼容性问题
```

### 技术细节

#### AnimatedGifStrategy优先级
```rust
priority() -> 90  // 高于普通CLI策略(80)和原生编码器(100)
```

#### 策略注册顺序
```
1. AnimatedGifStrategy (90) - 专门处理动画GIF
2. NativeAvifStrategy (100) - 处理普通图像
3. NativeWebPStrategy (100)
4. NativePngStrategy (100)
5. NativeJpegStrategy (100)
6. CliStrategy::Avifenc (80)
7. CliStrategy::Cjxl (80)
8. CliStrategy::Cwebp (80)
9. SameFormatOptimizer (80)
```

#### 策略匹配流程
```
输入: animated.gif → webp

1. AnimatedGifStrategy.can_convert(gif → webp)?
   → is_animated_gif(animated.gif)?
   → ✅ Yes (多帧)
   → ✅ Use gif2webp tool
   
2. 如果不是动画GIF:
   → 降级到 CliStrategy::Cwebp
   → 使用普通cwebp转换
```

---

**🎯 下一步行动**: 
1. 在Eagle中测试文件名特殊字符修复
2. 测试动画GIF转换功能
3. 如有问题，提供完整日志反馈
