# Eagle 缓存问题解决方案

## 问题
Eagle 持续加载旧的 JavaScript 文件，即使文件已经修复。

## 根本原因
Eagle 的 Chromium DevTools 缓存了旧版本的 JS 文件。

## ✅ 解决方案（按顺序尝试）

### 方法 1：DevTools 禁用缓存（最有效）⭐️⭐️⭐️

1. **打开 Eagle DevTools**：
   - 方法 A：在 Eagle 中按 `Cmd + Option + I`
   - 方法 B：右键点击插件界面 → "检查元素"

2. **禁用缓存**：
   - 在 DevTools 顶部点击 **"Network"** 标签
   - 勾选 **"Disable cache"** 复选框
   
3. **保持 DevTools 打开并刷新**：
   - 按 `Cmd + R` 刷新插件
   - **或者**关闭并重新打开插件（保持 DevTools 打开）

4. **验证**：
   - 检查 Console 标签是否还有 `SyntaxError`
   - 如果错误消失，问题解决！

### 方法 2：硬刷新（DevTools 打开时）⭐️⭐️

1. 打开 DevTools (`Cmd + Option + I`)
2. **右键点击** 浏览器的刷新按钮
3. 选择 **"清空缓存并硬性重新加载"**（Empty Cache and Hard Reload）

### 方法 3：Application Storage 清除⭐️

1. 打开 DevTools (`Cmd + Option + I`)
2. 点击 **"Application"** 标签
3. 在左侧找到 **"Storage"**
4. 点击 **"Clear site data"**
5. 确认并刷新插件

### 方法 4：修改文件强制刷新

如果上述方法都不行，添加一个无意义的空格：

```bash
# 在每个文件末尾添加一个空格强制时间戳变化
echo " " >> core/plugin/js/plugin-modules/template-loader.js
echo " " >> core/plugin/js/plugin-modules/file-validator.js
echo " " >> core/plugin/js/plugin-modules/ai-client.js

# 提交
git add -A
git commit -m "force: add whitespace to force Eagle refresh"
```

## 验证成功的标志

刷新后应该看到：

```
✅ [7/33] template-loader.js loaded successfully   ← 无 SyntaxError
✅ [15/33] file-validator.js loaded successfully   ← 无 SyntaxError
✅ [26/33] ai-client.js loaded successfully        ← 无 SyntaxError
```

**不再看到：**
```
❌ Uncaught SyntaxError: Identifier 'log' has already been declared
❌ Template loader not found
```

## 文件验证

所有文件已确认修复：
- ✅ template-loader.js - 只有1个 `const log` 声明（第9行）
- ✅ file-validator.js - 只有1个 `const log` 声明（第23行）
- ✅ ai-client.js - 只有1个 `const log` 声明（第14行）

## 注意事项

⚠️ **不要清理 Eagle 的全局缓存或设置**
- 可能导致 macOS 文件选择器延迟问题
- 只需要清除当前页面/插件的缓存

## 如果仍然不行

检查是否有多个 Eagle 实例在运行：
```bash
ps aux | grep Eagle
```

如果有多个，全部退出再重新打开。
