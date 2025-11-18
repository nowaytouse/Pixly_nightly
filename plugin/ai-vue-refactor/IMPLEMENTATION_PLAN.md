# PIXLY AI Vue 重构 - 实施计划

## ✅ 已完成

1. **项目结构**
   - package.json
   - vite.config.js
   - manifest.json（无边框 + vibrancy）
   - CSS 变量（官方配色）

## 🔄 需要完成

### 1. 默认暗色模式
```javascript
// src/main.js
document.documentElement.setAttribute('data-theme', 'dark')
```

### 2. 完整功能列表（参考 plugin/old/converter）

**AI 功能**:
- ✅ 智能质量预测
- ✅ 格式推荐
- ✅ 自动优化
- ⏳ SSIM 验证
- ⏳ GPU 加速
- ⏳ 智能预处理

**转换选项**:
- ⏳ 输出格式（下拉菜单）
- ⏳ 质量设置（滑块）
- ⏳ 速度设置（下拉）
- ⏳ 无损模式
- ⏳ 批量处理

**高级功能**:
- ⏳ XMP 合并
- ⏳ 文件名规范化
- ⏳ Eagle 元数据更新
- ⏳ 进度显示
- ⏳ 日志查看

### 3. UI 改进

**下拉菜单替代卡片**:
```vue
<!-- 不要这样 -->
<div class="preset-grid">
  <div class="preset-card">平衡</div>
  <div class="preset-card">质量</div>
  <div class="preset-card">体积</div>
</div>

<!-- 改为这样 -->
<select v-model="optimizeTarget">
  <option value="balanced">⚖️ 平衡 - 质量与体积兼顾</option>
  <option value="quality">💎 质量优先 - 最佳画质</option>
  <option value="size">📦 体积优先 - 最小文件</option>
</select>
```

**紧凑布局**:
- 使用 `<select>` 替代卡片
- 使用 `<details>` 折叠高级选项
- 减少 padding/margin

### 4. 官方配色应用

```css
/* 主按钮 */
.primary-btn {
  background: linear-gradient(90deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
}

/* 背景 */
body {
  background: var(--color-bg-secondary);
  color: var(--color-text-primary);
}

/* 面板 */
.panel {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border-primary);
}
```

## 📋 下一步行动

1. 复制 `plugin/old/converter/js/` 的逻辑到 Vue Composables
2. 复制 `plugin/old/converter/css/` 的样式（使用官方配色变量）
3. 创建下拉菜单组件
4. 集成 Rust CLI 调用
5. 测试构建和 Eagle 导入

## 🎯 目标

- **样式**: 100% 官方配色
- **布局**: 紧凑，无浪费空间
- **功能**: 完整（参考 old 版本）
- **默认**: 暗色模式 + 无边框
