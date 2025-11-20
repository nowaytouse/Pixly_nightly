# Format-Vue 日志窗口实现方案

## 🎯 目标

在format-vue插件中添加与ai-vue-refactor相同的日志窗口功能，替换QuickTools区域，支持切换显示。

## 📋 实现步骤

### 1. 在App.vue中添加日志系统

```vue
<script setup>
// 添加日志相关状态
const showLogWindow = ref(true) // 默认显示日志
const processLogs = ref([])
const logContentRef = ref(null)

// 添加日志方法
const addLog = (message, type = 'info', icon = '📝') => {
  const now = new Date()
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`
  
  processLogs.value.push({
    time,
    icon,
    message,
    type // info, success, warning, error
  })
  
  // 自动滚动到底部
  setTimeout(() => {
    if (logContentRef.value) {
      logContentRef.value.scrollTop = logContentRef.value.scrollHeight
    }
  }, 10)
}

// 清空日志
const clearLogs = () => {
  processLogs.value = []
}
</script>
```

### 2. 替换QuickTools区域为日志窗口

```vue
<template>
  <!-- 原QuickTools位置 -->
  <div class="log-window-container">
    <!-- 切换标签 -->
    <div class="log-tabs">
      <button 
        class="log-tab" 
        :class="{ active: showLogWindow }"
        @click="showLogWindow = true"
      >
        {{ t('log.processLog') }}
      </button>
      <button 
        class="log-tab" 
        :class="{ active: !showLogWindow }"
        @click="showLogWindow = false"
      >
        {{ t('log.quickTools') }}
      </button>
    </div>

    <!-- 日志窗口 -->
    <div v-if="showLogWindow" class="log-window">
      <div class="log-content" ref="logContentRef" @wheel.stop>
        <div v-for="(log, index) in processLogs" :key="index" class="log-entry" :class="log.type">
          <span class="log-time">{{ log.time }}</span>
          <span class="log-icon">{{ log.icon }}</span>
          <span class="log-message">{{ log.message }}</span>
        </div>
        <div v-if="processLogs.length === 0" class="log-empty">
          {{ t('log.waiting') }}
        </div>
      </div>
    </div>

    <!-- 快捷工具面板 -->
    <div v-else class="tools-panel">
      <QuickTools v-model="quickTools" />
    </div>
  </div>
</template>
```

### 3. 添加CSS样式（复制自ai-vue-refactor）

```css
/* 日志窗口容器 */
.log-window-container {
  margin-top: 16px;
  border-radius: 8px;
  overflow: hidden;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
}

.log-tabs {
  display: flex;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-primary);
}

.log-tab {
  flex: 1;
  padding: 10px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border-bottom: 2px solid transparent;
}

.log-tab:hover {
  background: var(--bg-hover);
}

.log-tab.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
  background: var(--bg-secondary);
}

.log-window {
  height: 200px;
  overflow: hidden;
  isolation: isolate;
  position: relative;
}

.log-content {
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
  font-size: 11px;
  line-height: 1.6;
  background: #1a1a1a;
  color: #e0e0e0;
  scroll-behavior: smooth;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
  pointer-events: auto;
  position: relative;
  z-index: 1;
}

/* 自定义滚动条 */
.log-content::-webkit-scrollbar {
  width: 8px;
}

.log-content::-webkit-scrollbar-track {
  background: #0a0a0a;
}

.log-content::-webkit-scrollbar-thumb {
  background: #333;
  border-radius: 4px;
}

.log-content::-webkit-scrollbar-thumb:hover {
  background: #444;
}

.log-entry {
  display: flex;
  gap: 8px;
  padding: 2px 0;
  align-items: flex-start;
}

.log-time {
  color: #666;
  flex-shrink: 0;
}

.log-icon {
  flex-shrink: 0;
}

.log-message {
  flex: 1;
  word-break: break-word;
}

.log-entry.success .log-message {
  color: #4caf50;
}

.log-entry.warning .log-message {
  color: #ff9800;
}

.log-entry.error .log-message {
  color: #f44336;
}

.log-empty {
  color: #666;
  text-align: center;
  padding: 60px 20px;
  font-style: italic;
}

.tools-panel {
  padding: 0;
}
```

### 4. 在转换过程中添加日志

```javascript
// 在convertImages函数开始时
clearLogs()
showLogWindow.value = true
addLog(`开始处理 ${files.value.length} 个文件`, 'info', '🚀')
addLog(`目标格式: ${selectedFormat.value}`, 'info', '🎯')

// 处理每个文件时
addLog(`[${i + 1}/${total}] 处理: ${file.name}`, 'info', '⚙️')

// 完成时
addLog('─────────────────────────', 'info', '')
if (successCount === total) {
  addLog(`✅ 全部完成！成功: ${successCount}/${total}`, 'success', '✅')
} else {
  addLog(`⚠️ 部分完成！成功: ${successCount}, 失败: ${failedCount}`, 'warning', '⚠️')
}

// 显示每个文件结果
results.forEach(r => {
  if (r.success) {
    addLog(`  ✓ ${r.file}`, 'success', '')
  } else {
    addLog(`  ✗ ${r.file}: ${r.error}`, 'error', '')
  }
})
```

### 5. 添加i18n翻译

**zh_CN.json**:
```json
{
  "log": {
    "processLog": "🔍 处理日志",
    "quickTools": "🛠️ 快捷工具",
    "waiting": "等待处理..."
  }
}
```

**en.json**:
```json
{
  "log": {
    "processLog": "🔍 Process Log",
    "quickTools": "🛠️ Quick Tools",
    "waiting": "Waiting for processing..."
  }
}
```

## 📊 参考实现

完整实现请参考：
- `plugin/ai-vue-refactor/src/App.vue` (行180-230, 行425-460, 行1350-1450)
- 日志系统状态管理
- 日志窗口HTML结构
- CSS样式定义

## ✅ 验收标准

1. 日志窗口和快捷工具可以切换
2. 日志窗口样式与ai-vue-refactor一致
3. 转换过程中实时显示日志
4. 日志可以滚动，不受文件列表影响
5. 完整的i18n支持（中英文）
6. 显示真实的转换结果数据

## 🎯 实现优先级

1. **高优先级**：基础日志窗口和切换功能
2. **中优先级**：转换过程日志集成
3. **低优先级**：样式微调和动画效果

---

**创建日期**: 2025-11-20  
**参考插件**: ai-vue-refactor  
**状态**: 📋 待实现
