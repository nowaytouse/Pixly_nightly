# 📋 Log Migration Guide

## 🎯 目标：避免硬编码，统一使用LOG常量

### 为什么使用LOG常量？

1. **避免硬编码**：所有日志消息集中管理
2. **易于维护**：修改日志只需改一处
3. **便于搜索**：`LOG.FILE_SELECTED` vs 散落各处的字符串
4. **未来扩展**：可轻松添加日志分析、国际化等功能
5. **类型安全**：IDE可以自动补全和检查

### 迁移方式

#### ❌ 旧方式（硬编码）

```javascript
// file-handler.js
pixlyLog.info('PIXLY File', 'Selected 4 files');
pixlyLog.info('PIXLY File', `File selection completed: ${count} files in ${time}ms`);
```

#### ✅ 新方式（LOG常量 + 参数）

```javascript
// file-handler.js
pixlyLog.info('PIXLY File', LOG.FILE_SELECTED, { count: 4 });
pixlyLog.info('PIXLY File', LOG.FILE_SELECTION_COMPLETE, { count, time });
```

### 参数替换规则

日志消息模板使用 `{key}` 占位符：

```javascript
// log-constants.js
FILE_SELECTION_COMPLETE: 'File selection completed: {count} files in {time}ms'

// 使用
pixlyLog.info('PIXLY File', LOG.FILE_SELECTION_COMPLETE, { 
    count: 5, 
    time: 3.2 
});

// 输出
[PIXLY File] File selection completed: 5 files in 3.2ms
```

### 迁移优先级

#### 🔥 P0 - 高频日志（立即迁移）

1. **file-handler.js** - 文件选择日志（触发最频繁）
2. **ui-handlers.js** - UI事件日志（大量重复）
3. **theme.js** - 主题切换日志（多次重复）

#### ⚡ P1 - 中频日志（逐步迁移）

1. **ai-integration.js** - AI服务检测
2. **i18n.fixed.js** - 语言切换
3. **template-loader.js** - 模板加载

#### 📦 P2 - 低频日志（按需迁移）

1. **kernel-guard.js** - 内核检测
2. **dependency-checker.js** - 依赖检查
3. **cache-manager.js** - 缓存管理

### 迁移步骤

#### Step 1: 识别硬编码日志

```bash
# 搜索文件中的硬编码日志
grep -n "pixlyLog\\.info.*'PIXLY File'" file-handler.js
```

#### Step 2: 在LOG常量中查找或添加

```javascript
// log-constants.js
const LOG = {
    // 如果不存在，添加新常量
    FILE_SELECTED: 'Selected {count} files',
    ...
};
```

#### Step 3: 替换代码

```javascript
// 旧代码
pixlyLog.info('PIXLY File', `Selected ${count} files`);

// 新代码
pixlyLog.info('PIXLY File', LOG.FILE_SELECTED, { count });
```

#### Step 4: 测试验证

```javascript
// 控制台应该输出相同内容
✅ [PIXLY File] Selected 4 files
```

### 特殊情况处理

#### 1. 动态模块名

```javascript
// ❌ 不推荐
pixlyLog.info(`PIXLY ${moduleName}`, 'Some message');

// ✅ 推荐：使用固定模块名
pixlyLog.info('PIXLY Dynamic', LOG.SOME_MESSAGE, { module: moduleName });
```

#### 2. 复杂对象

```javascript
// 日志消息只放简单参数
pixlyLog.info('PIXLY File', LOG.FILE_INFO, { name: file.name, size: file.size });

// 复杂对象用额外参数传递
pixlyLog.debug('PIXLY File', LOG.FILE_DETAILS, { name: file.name }, file);
```

#### 3. 条件日志

```javascript
// ❌ 旧方式
if (isDev) {
    pixlyLog.debug('PIXLY', 'Debug info: ' + JSON.stringify(data));
}

// ✅ 新方式（pixlyLog已内置级别检查）
pixlyLog.debug('PIXLY', LOG.DEBUG_INFO, { data: JSON.stringify(data) });
```

### 性能考虑

1. **去重机制自动生效**：相同LOG常量1秒内只显示一次
2. **惰性求值**：参数对象在日志级别满足时才格式化
3. **O(1)查找**：Map-based缓存性能优异

### 检查清单

- [ ] 识别文件中所有硬编码日志
- [ ] 在LOG常量中找到或创建对应键
- [ ] 替换代码使用LOG常量
- [ ] 添加参数对象（如需要）
- [ ] 测试日志输出正确
- [ ] 验证去重机制工作
- [ ] 提交代码并注明迁移

### 示例：完整迁移

#### file-handler.js 迁移前后对比

**❌ 迁移前**：
```javascript
pixlyLog.info('PIXLY File', 'Starting file selection...');
pixlyLog.info('PIXLY File', `Selected ${count} files`);
pixlyLog.info('PIXLY File', `Eagle returned ${count} items`);
pixlyLog.info('PIXLY File', `Conversion type: ${type}`);
pixlyLog.info('PIXLY File', `UI updated: ${count} files displayed`);
pixlyLog.info('PIXLY File', `File selection completed: ${count} files in ${time}ms`);
```

**✅ 迁移后**：
```javascript
pixlyLog.info('PIXLY File', LOG.FILE_SELECTION_START);
pixlyLog.info('PIXLY File', LOG.FILE_SELECTED, { count });
pixlyLog.info('PIXLY File', LOG.FILE_EAGLE_RETURNED, { count });
pixlyLog.info('PIXLY File', LOG.FILE_CONVERSION_TYPE, { type });
pixlyLog.info('PIXLY File', LOG.FILE_UI_UPDATED, { count });
pixlyLog.info('PIXLY File', LOG.FILE_SELECTION_COMPLETE, { count, time });
```

**收益**：
- ✅ 代码更简洁清晰
- ✅ 日志消息统一管理
- ✅ 自动去重减少噪音
- ✅ IDE可以追踪LOG常量引用
- ✅ 修改日志不需要搜索替换

### 推荐工作流

1. **每次修改文件时顺便迁移**：不需要专门大批量迁移
2. **优先迁移高频文件**：效果最明显
3. **保持向后兼容**：旧方式仍然可用，逐步迁移即可

### 常见问题

**Q: 必须一次性迁移所有文件吗？**
A: 不需要。新旧方式兼容，可以逐步迁移。

**Q: LOG常量太多怎么办？**
A: 按模块组织，IDE可以自动补全。比如输入`LOG.FILE_`会提示所有文件相关常量。

**Q: 如何添加新的LOG常量？**
A: 在`log-constants.js`中添加即可，按模块分类。

**Q: 参数名必须和占位符一致吗？**
A: 是的。`{count}`需要`{ count: value }`。

**Q: 能用中文作为LOG常量值吗？**
A: 可以，但建议使用英语。未来如需国际化可以轻松扩展。

---

**开始迁移吧！质量第一，避免硬编码！** 🚀
