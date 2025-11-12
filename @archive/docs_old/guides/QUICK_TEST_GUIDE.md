# 🧪 快速测试指南

## 🔴 最重要的测试

### 1. 视频面板显示测试

**步骤**:
1. 刷新Eagle插件（重新加载）
2. 点击「🎬 视频处理」标签
3. **确认面板不是空白！**

**预期结果**:
✅ 应该看到：
- 🤖 智能转换 / ⚙️ 手动转换 标签
- 🧠 视频AI智能选项
- 输入类型选择（动图/视频）
- 编码器选择（H.264/H.265/AV1等）

❌ 如果看到空白，说明修复失败

---

### 2. 手动模式可用测试

**步骤**:
1. 切换到「图像转换」或「视频处理」
2. 点击「⚙️ 手动转换」标签
3. **格式选择区域应该立即可用（不灰色）**

**预期结果**:
✅ 控制台日志：
```
[PIXLY Manual] ✅ Rust CLI available, enabling controls immediately...
```

✅ 界面：所有格式按钮可点击，不灰色

---

### 3. i18n翻译测试

**步骤**:
1. 查看「🛠️ 快捷工具」区域

**预期结果**:
✅ 显示：「AI文件验证」
❌ 错误：`tools.fileValidation`

---

### 4. JPEG按钮调试测试

**步骤**:
1. 打开F12控制台
2. 选择一个JPEG文件
3. 查看控制台日志

**预期日志**:
```
[PIXLY] 🔍 Checking selectedFiles: 1 files
[PIXLY] 📄 File 1: 格局技巧.JPG | ext: .jpg  ← 查看这一行！
[PIXLY] ✅ Found JPEG file: 格局技巧.JPG ext: .jpg
[PIXLY] ✨ Showing JPEG→JXL button (found JPEG files)
```

**如果按钮不显示**:
- 检查 `ext:` 后面的值
- 如果不是 `.jpg` / `.jpeg` / `.jpe` / `.jfif` / `.jfi`，说明文件不是JPEG
- 或者Eagle没有正确识别文件类型

---

## 📸 预期界面截图说明

### 视频面板（修复前 vs 修复后）

**修复前** ❌:
```
┌─────────────────────────────────┐
│ 🎬 视频处理                      │
├─────────────────────────────────┤
│                                 │  ← 空白！
│                                 │
│                                 │
└─────────────────────────────────┘
```

**修复后** ✅:
```
┌─────────────────────────────────┐
│ 🎬 视频处理                      │
├─────────────────────────────────┤
│ ┌─────────┬─────────┐          │
│ │🤖智能转换│⚙️手动转换│          │  ← 可见！
│ └─────────┴─────────┘          │
│ 🧠 视频AI智能选项                │
│ ┌─────────────────────┐        │
│ │ ⚡ 快捷预设          │        │
│ └─────────────────────┘        │
└─────────────────────────────────┘
```

---

## 🐛 如果问题依旧

### 视频面板仍然空白
1. 打开F12控制台
2. 查找错误信息
3. 检查是否有以下日志：
   ```
   [PIXLY Template] ✅ Loaded: video-panel (xxxxx chars)
   [PIXLY Template] 🎯 Injected video-panel into #video-panel-container
   ```
4. 在控制台执行：
   ```javascript
   console.log(document.getElementById('video-panel-container').innerHTML.length);
   ```
   如果返回0，说明模板没有注入

### 手动模式仍被禁用
1. 在控制台执行：
   ```javascript
   console.log('Rust CLI:', window.rustCLI);
   console.log('Available:', window.rustCLI?.available);
   ```
2. 如果 `available: false`，说明Rust CLI未启动
3. 如果 `available: true`，但控件仍禁用，请提供完整的控制台日志

### JPEG按钮仍不显示
1. 在控制台执行：
   ```javascript
   console.table(window.selectedFiles.map(f => ({
       name: f.name,
       ext: f.ext,
       isImage: f.isImage
   })));
   ```
2. 截图发送给我

---

**测试完成后请反馈**:
1. ✅ 哪些问题已修复
2. ❌ 哪些问题仍存在
3. 📋 相关的控制台日志截图

