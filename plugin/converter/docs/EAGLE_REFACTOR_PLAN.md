# Eagle Plugin Refactoring Plan
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

## 🎯 目标
1. 修复转换功能
2. 改善UI/UX交互
3. 移除紫色主题
4. 进一步解耦模块
5. 优化性能

## Phase 1: 配色方案重构 (优先级: 🔴 Critical)

### 新配色方案 - 专业蓝灰系
```css
暗色模式:
  --primary: #3b82f6 (蓝色)
  --primary-light: #60a5fa
  --primary-dark: #2563eb
  --accent: #06b6d4 (青色)
  --bg-base: #0f172a (深灰蓝)
  --bg-elevated: #1e293b
  --bg-card: #334155

亮色模式:
  --primary: #2563eb
  --primary-light: #3b82f6
  --bg-base: #f8fafc
  --bg-card: #ffffff
  --border: #e2e8f0
```

### 移除紫色
- 替换所有 #667eea -> #3b82f6
- 替换所有 #764ba2 -> #2563eb
- 替换所有 #8b9bff -> #60a5fa
- 移除渐变中的紫色

## Phase 2: CSS架构清理 (优先级: 🟡 High)

### HTML内联样式迁移
- 将index.html中的<style>块移至styles.css
- 移除!important滥用
- 统一主题变量使用

### 模块化CSS
- 创建 css/components/ 目录
- 拆分: buttons.css, forms.css, cards.css, ai-panel.css

## Phase 3: UI模块拆分 (优先级: 🟡 High)

### ui-handlers.js拆分方案 (3056行 -> 多个文件)
```
ui-handlers.js (核心协调 ~500行)
  ├── ui-format-panel.js (格式选择面板 ~400行)
  ├── ui-quality-panel.js (质量控制面板 ~400行)
  ├── ui-ai-panel.js (AI选项面板 ~400行)
  ├── ui-advanced-panel.js (高级选项面板 ~400行)
  ├── ui-conversion-list.js (转换列表管理 ~500行)
  └── ui-progress.js (进度显示 ~400行)
```

## Phase 4: 转换功能修复 (优先级: 🔴 Critical)

### 问题诊断
- 检查 rust-cli-executor.js 调用链
- 验证 image-conversion.js 参数传递
- 测试 Eagle API 集成

### 修复步骤
1. 重构convertImage流程
2. 添加详细错误日志
3. 实现重试机制
4. 统一错误处理

## Phase 5: EventBus完全集成 (优先级: 🟢 Medium)

### 事件驱动重构
- 格式选择 -> event: 'format:changed'
- 质量调整 -> event: 'quality:changed'
- 转换开始 -> event: 'conversion:start'
- 转换完成 -> event: 'conversion:complete'
- 错误发生 -> event: 'conversion:error'

## 实施顺序
1. ✅ Phase 1: 配色方案重构 (1小时)
2. ✅ Phase 4: 转换功能修复 (2小时)
3. ✅ Phase 2: CSS架构清理 (1小时)
4. ✅ Phase 3: UI模块拆分 (2小时)
5. ✅ Phase 5: EventBus集成 (1小时)

## 质量目标
- 转换成功率 > 95%
- UI响应时间 < 100ms
- 代码覆盖率 > 70%
- 用户满意度 > 4.5/5

