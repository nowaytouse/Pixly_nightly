# 无边框窗口设置指南

**版本**: 3.0.1  
**日期**: 2025-11-18

---

## ✅ 已完成的配置

### 1. manifest.json 配置

```json
{
  "main": {
    "frameless": true,
    "frame": false,
    "titleBarStyle": "hidden"
  }
}
```

### 2. 自定义标题栏

已在 `App.vue` 中添加：
- 拖拽区域（`-webkit-app-region: drag`）
- 窗口控制按钮（最小化、最大化、关闭）
- 标题显示

### 3. 全局样式

已在 `global.css` 中添加：
- 移除 body 默认边距
- 设置 overflow: hidden
- 确保全屏显示

---

## 🔄 使无边框生效的步骤

### 方法1: 重启Eagle（推荐）

1. 完全退出Eagle应用
2. 重新启动Eagle
3. 打开PIXLY Format Vue插件

### 方法2: 重新安装插件

1. 在Eagle中删除插件
2. 重新安装插件
3. 打开插件

### 方法3: 清除Eagle缓存

```bash
# 关闭Eagle
# 删除插件缓存
rm -rf ~/Library/Application\ Support/Eagle/Plugins/com.pixly.format.vue

# 重新启动Eagle
```

---

## 🧪 验证无边框是否生效

打开插件后，应该看到：

✅ **正确的无边框窗口**：
- 没有macOS的红绿灯按钮
- 没有系统标题栏
- 顶部是自定义的PIXLY标题栏（32px高）
- 可以通过标题栏拖拽窗口
- 右上角有自定义的最小化/最大化/关闭按钮

❌ **仍有边框（配置未生效）**：
- 仍然看到macOS红绿灯按钮
- 仍然有系统标题栏
- 需要重启Eagle

---

## 🎨 自定义标题栏功能

### 拖拽窗口
- 点击标题栏任意位置可拖拽窗口
- 使用 `-webkit-app-region: drag` 实现

### 窗口控制按钮
- **最小化**：点击 `-` 按钮
- **最大化/还原**：点击 `□` 按钮
- **关闭**：点击 `×` 按钮（红色悬停效果）

### 标题显示
- 显示 "PIXLY Format Vue"
- 使用国际化文本 `t('app.title')`

---

## 🔧 技术实现

### 标题栏HTML结构

```vue
<div class="titlebar">
  <div class="titlebar-drag">
    <span class="titlebar-title">{{ t('app.title') }}</span>
  </div>
  <div class="titlebar-controls">
    <button class="titlebar-btn" @click="minimizeWindow">-</button>
    <button class="titlebar-btn" @click="maximizeWindow">□</button>
    <button class="titlebar-btn titlebar-close" @click="closeWindow">×</button>
  </div>
</div>
```

### 窗口控制API

```javascript
// 最小化
window.eagle.window.minimize()

// 最大化/还原
window.eagle.window.toggleMaximize()

// 关闭
window.eagle.window.close()
```

---

## 📊 配置对比

### 有边框（默认）
```json
{
  "main": {
    "url": "dist/index.html",
    "width": 1200,
    "height": 800
  }
}
```

### 无边框（当前）
```json
{
  "main": {
    "url": "dist/index.html",
    "width": 1200,
    "height": 800,
    "frameless": true,
    "frame": false,
    "titleBarStyle": "hidden"
  }
}
```

---

## ⚠️ 常见问题

### Q: 配置了frameless但仍有边框？

**A**: Eagle需要重启才能应用新的manifest配置。

**解决方案**：
1. 完全退出Eagle（Cmd+Q）
2. 重新启动Eagle
3. 打开插件

### Q: 标题栏无法拖拽？

**A**: 检查CSS是否正确应用。

**验证**：
```css
.titlebar-drag {
  -webkit-app-region: drag;
}
```

### Q: 窗口控制按钮不工作？

**A**: 检查Eagle API是否可用。

**验证**：
```javascript
console.log(window.eagle.window) // 应该有 minimize, toggleMaximize, close 方法
```

### Q: 标题栏被内容覆盖？

**A**: 检查z-index和布局。

**修复**：
```css
.titlebar {
  position: relative;
  z-index: 1000;
}
```

---

## 🚀 下一步

如果无边框仍未生效：

1. **检查Eagle版本**：确保使用最新版本的Eagle
2. **检查插件ID**：确保插件ID唯一
3. **查看Eagle日志**：检查是否有错误信息
4. **联系Eagle支持**：如果问题持续存在

---

## 📝 版本历史

- **3.0.1** (2025-11-18): 添加无边框窗口支持
- **3.0.0** (2025-11-18): Vue重构版本

---

**🔥 重要提示：修改manifest.json后必须重启Eagle才能生效！**
