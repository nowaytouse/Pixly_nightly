# Phase 40.7.3 测试指南

## 🎯 测试目标

验证以下两个关键问题已修复：
1. **动态图片转换后不会动**
2. **转换过程沉默，没有反馈**

---

## 📋 测试前准备

### 1. 启动AI服务
```bash
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/cmd/ai-service
go run main.go --port 50052
```

### 2. 确认Rust CLI已编译
```bash
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/pixly-rust
cargo build --release
ls -lh target/release/pixly-rust
```

### 3. 准备测试文件
- 动态GIF文件（至少1个）
- 静态JPEG文件（至少1个）
- 静态PNG文件（至少1个）

---

## ✅ 测试1：动画GIF → WebP

### 步骤
1. 在Eagle中选择一个**动态GIF**文件
2. 右键 → Pixly Plugin
3. 确保选择**Smart Mode**（智能模式）
4. 选择**WebP**格式
5. 点击"开始转换"
6. **打开Eagle的Console**（菜单→查看→开发者工具→Console）

### 预期结果

#### Console日志应显示：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [1/1] 正在转换: animation.gif
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | 🔄 Converting: /path/to/animation.gif -> /path/to/animation.webp (webp)
[PIXLY Rust CLI] | 🤖 Querying AI service for optimal parameters...
[PIXLY Rust CLI] | ✅ AI parameters received:
[PIXLY Rust CLI] |    Quality: 90, Speed: 7, Lossless: false
[PIXLY Rust CLI] | 📋 Registering conversion strategies...
[PIXLY Rust CLI] |   ✓ Animated GIF (gif2webp/ffmpeg) - Available: true
[PIXLY Rust CLI] |   ✓ CLI JXL (cjxl) - Available: true
[PIXLY Rust CLI] | 
[PIXLY Rust CLI] | 🎬 Detected GIF input with keep_animated=true     ⬅️ 关键！
[PIXLY Rust CLI] |    → Forcing Animated GIF Strategy                ⬅️ 关键！
[PIXLY Rust CLI] | ✅ Strategy: Animated GIF (gif2webp)              ⬅️ 关键！
[PIXLY Rust CLI] | Converting GIF to WebP with animation...
[PIXLY Rust CLI] | ✅ GIF animation conversion successful
[PIXLY] ✅ 成功: animation.gif → webp

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 🎉 转换完成：成功 1，失败 0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### Toast通知应显示：
1. `✅ animation.gif` （转换完成时）
2. `�� 全部成功！转换了 1 个文件` （最终汇总）

#### 验证动画：
```bash
# 在终端验证
ffplay /path/to/animation.webp
# 应该显示动画，帧率约25fps
```

### ❌ 如果失败

**症状1**：日志显示 `Strategy: Native WebP (webp)`
- **原因**：策略选择逻辑没有强制使用`AnimatedGifStrategy`
- **检查**：`pixly-rust/src/converter/strategy.rs` 是否有Phase 40.7的强制检查

**症状2**：没有任何Console日志
- **原因**：`rust-cli-executor.js`还在使用`exec()`而不是`execWithProgress()`
- **检查**：`plugin/js/plugin-modules/rust-cli-executor.js` 第272行

**症状3**：`ffplay`显示单帧静态图
- **原因**：虽然日志正确，但`gif2webp`执行失败
- **检查**：`gif2webp --version` 是否安装

---

## ✅ 测试2：动画GIF → AVIF

### 步骤
同上，但选择**AVIF**格式

### 预期结果

#### Console日志应显示：
```
[PIXLY Rust CLI] | 🎬 Detected GIF input with keep_animated=true
[PIXLY Rust CLI] |    → Forcing Animated GIF Strategy
[PIXLY Rust CLI] | ✅ Strategy: Animated GIF (ffmpeg)              ⬅️ 使用ffmpeg
[PIXLY Rust CLI] | Converting GIF to AVIF with animation via FFmpeg...
[PIXLY Rust CLI] | ✅ GIF animation conversion successful
```

#### 验证动画：
```bash
ffplay /path/to/animation.avif
# 应该显示动画，但帧率低（1-2 fps，AVIF限制）
```

---

## ✅ 测试3：JPEG → JXL（无损）

### 步骤
1. 在Eagle中选择一个**JPEG**文件
2. 右键 → Pixly Plugin
3. 确保选择**Smart Mode**（智能模式）
4. 选择**JXL**格式
5. 点击"开始转换"

### 预期结果

#### Console日志应显示：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [1/1] 正在转换: photo.jpg
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | 🤖 Querying AI service for optimal parameters...
[PIXLY Rust CLI] | ✅ AI parameters received:
[PIXLY Rust CLI] |    Quality: 100, Speed: 7, Lossless: true       ⬅️ AI预测无损
[PIXLY Rust CLI] |    Final config: Q=100, S=7, Lossless=true, Metadata=true
[PIXLY Rust CLI] | ✅ Strategy: CLI JXL (cjxl)
[PIXLY Rust CLI] | ✅ Conversion successful
[PIXLY] ✅ 成功: photo.jpg → jxl

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 🎉 转换完成：成功 1，失败 0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### Toast通知应显示：
1. `✅ photo.jpg`
2. `🎉 全部成功！转换了 1 个文件`

---

## ✅ 测试4：批量转换（混合文件）

### 步骤
1. 在Eagle中选择**多个文件**（包括GIF、JPEG、PNG）
2. 右键 → Pixly Plugin
3. 选择**Smart Mode** + **WebP**格式
4. 点击"开始转换"

### 预期结果

#### Console日志应显示：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [1/3] 正在转换: animation.gif
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | 🎬 Detected GIF input with keep_animated=true
[PIXLY Rust CLI] | ✅ Strategy: Animated GIF (gif2webp)
[PIXLY] ✅ 成功: animation.gif → webp

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [2/3] 正在转换: photo.jpg
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | ✅ Strategy: CLI WebP (cwebp)
[PIXLY] ✅ 成功: photo.jpg → webp

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [3/3] 正在转换: image.png
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | ✅ Strategy: CLI WebP (cwebp)
[PIXLY] ✅ 成功: image.png → webp

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 🎉 转换完成：成功 3，失败 0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### Toast通知应显示：
1. `✅ animation.gif`
2. `✅ photo.jpg`
3. `✅ image.png`
4. `🎉 全部成功！转换了 3 个文件`

---

## ✅ 测试5：失败场景（AI服务未启动）

### 步骤
1. **停止AI服务**
2. 在Eagle中选择一个**PNG**文件
3. 右键 → Pixly Plugin
4. 选择**Smart Mode** + **JXL**格式
5. 点击"开始转换"

### 预期结果

#### Console日志应显示：
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 📸 [1/1] 正在转换: image.png
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[PIXLY Rust CLI] | 🤖 Querying AI service for optimal parameters...
[PIXLY Rust CLI] | ❌ AI prediction failed: ❌ AI service required but not available!
[PIXLY Rust CLI] | Start Go AI service:
[PIXLY Rust CLI] | cd cmd/ai-service && go run main.go --port 50052
[PIXLY Rust CLI] | 🔥 AI service is REQUIRED. No fallback available.
[PIXLY] ❌ 失败: image.png: AI service required but not available!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[PIXLY] 🎉 转换完成：成功 0，失败 1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### Toast通知应显示：
1. `❌ image.png: AI service required but not available!`
2. `❌ 全部失败！1 个文件转换失败`

---

## 📊 测试检查表

| 测试项 | 预期 | 实际 | 状态 |
|--------|------|------|------|
| GIF→WebP 动画保留 | ✅ | ? | ⬜ |
| GIF→AVIF 动画保留 | ✅ | ? | ⬜ |
| Console日志详细 | ✅ | ? | ⬜ |
| Toast通知显示 | ✅ | ? | ⬜ |
| 转换完成汇总 | ✅ | ? | ⬜ |
| 失败场景提示 | ✅ | ? | ⬜ |
| JPEG无损转码 | ✅ | ? | ⬜ |

---

## 🐛 常见问题排查

### 问题1：看不到Console日志
**解决**：
1. 检查Eagle的开发者工具是否打开
2. 检查Console是否被过滤（取消勾选"Hide network"等）
3. 刷新Eagle插件（关闭并重新打开）

### 问题2：Toast没有显示
**解决**：
1. 检查`window.PIXLY.Toast`是否存在：
   ```javascript
   // 在Eagle Console中执行
   console.log(window.PIXLY?.Toast);  // 应该显示对象
   ```
2. 检查`plugin/js/plugin-modules/toast.js`是否加载

### 问题3：GIF转换后还是静态
**解决**：
1. 检查Console日志是否显示"🎬 Detected GIF input"
2. 检查策略是否为"Animated GIF"而不是"Native WebP"
3. 验证`gif2webp`和`ffmpeg`是否安装：
   ```bash
   which gif2webp
   which ffmpeg
   ```

---

**测试完成后，请将结果反馈给开发者，包括：**
- Console完整日志（截图）
- Toast通知截图
- 转换后的文件（用`ffplay`验证动画）
- 任何异常或错误信息

