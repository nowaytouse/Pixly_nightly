# 📋 日志迁移统一计划

## 目标
将所有`console.log`迁移到统一的`pixlyLog`系统，使用`log-constants.js`中定义的常量。

---

## 当前进度

### 统计数据（2025-11-10）

| 文件 | console.log数量 | 优先级 | 状态 |
|------|----------------|--------|------|
| ui-handlers.js | 205 | 高 | 部分完成 |
| image-conversion.js | 60 | 🔥 最高 | 待迁移 |
| rust-cli-executor.js | 33 | 🔥 最高 | 待迁移 |
| file-handler.js | 31 | 高 | 待迁移 |
| conversion-guard.js | 29 | 高 | 待迁移 |
| video-conversion.js | 26 | 高 | 待迁移 |
| performance-monitor.js | 17 | 中 | 待迁移 |
| ai-integration.js | 16 | 中 | 待迁移 |
| file-validator.js | 16 | 中 | 待迁移 |
| theme.js | 15 | 中 | 待迁移 |
| 其他文件 (32个) | ~130 | 低 | 待迁移 |

**总计**: 788个console.log需要迁移

---

## 迁移优先级

### 🔥 Phase 1: 核心转换模块（最高优先级）
**影响**: 用户直接交互，高频使用，日志量大

1. **image-conversion.js** (60个)
   - 图像转换核心逻辑
   - 进度更新、错误处理
   - 用户最常用功能

2. **rust-cli-executor.js** (33个)
   - Rust CLI调用
   - 命令执行日志
   - 错误诊断关键

3. **file-handler.js** (31个)
   - 文件选择和验证
   - UI更新逻辑
   - 高频调用

### ⚡ Phase 2: 守护和视频模块（高优先级）

4. **conversion-guard.js** (29个)
   - 转换守护逻辑
   - 状态监控

5. **video-conversion.js** (26个)
   - 视频转换核心
   - 编码参数日志

### 📊 Phase 3: UI和工具模块（中优先级）

6. **ui-handlers.js** (剩余~150个)
   - 继续完成未迁移部分

7. **performance-monitor.js** (17个)
   - 性能监控日志

8. **ai-integration.js** (16个)
   - AI集成日志

9. **file-validator.js** (16个)
   - 文件验证日志

10. **theme.js** (15个)
    - 主题切换日志

### 🔧 Phase 4: 辅助模块（低优先级）

11-42. 其他32个文件 (~130个)
    - 工具类、配置类
    - 低频调用模块

---

## 迁移策略

### 1. 检查和扩展常量

**动作**: 为每个模块添加必要的LOG常量

**位置**: `/core/plugin/js/plugin-modules/log-constants.js`

**原则**:
- 语义化命名（如`IMAGE_CONV_START`, `RUST_CLI_EXEC`）
- 支持参数化（使用`{placeholder}`）
- 按模块分组

**示例**:
```javascript
// ========== Image Conversion ==========
IMAGE_CONV_START: 'startConversion() called',
IMAGE_CONV_PROGRESS: '[{current}/{total}] Converting: {file}',
IMAGE_CONV_SUCCESS: 'Conversion successful: {file}',
IMAGE_CONV_ERROR: 'Conversion failed: {file} - {error}',

// ========== Rust CLI Executor ==========
RUST_CLI_EXEC: 'Executing: {command}',
RUST_CLI_STDOUT: 'STDOUT: {output}',
RUST_CLI_STDERR: 'STDERR: {error}',
RUST_CLI_ERROR: 'Execution error: {error}',
```

### 2. 迁移模式

#### **模式A: 简单替换（无参数）**
```javascript
// Before
console.log('[PIXLY] Feature enabled');

// After
log.info?.('PIXLY', LOG.FEATURE_ENABLED) || console.log('[PIXLY] Feature enabled');
```

#### **模式B: 参数化替换**
```javascript
// Before
console.log(`[PIXLY] Converting ${file} to ${format}`);

// After
log.info?.('PIXLY', LOG.CONV_FILE, { file, format }) || 
    console.log(`[PIXLY] Converting ${file} to ${format}`);
```

#### **模式C: 条件日志**
```javascript
// Before
if (debug) {
    console.log('[Debug] Details:', data);
}

// After
log.debug?.('PIXLY', LOG.DEBUG_DETAILS, { data }) ||
    console.log('[Debug] Details:', data);
```

#### **模式D: 错误日志**
```javascript
// Before
console.error('[Error]', error);

// After
log.error?.('PIXLY', LOG.ERROR_OCCURRED, { error: error.message }) ||
    console.error('[Error]', error);
```

### 3. 保留fallback

**原则**: 所有迁移都保留`console.log` fallback，确保即使`pixlyLog`不可用也能输出

**格式**:
```javascript
log.info?.('Category', LOG.CONSTANT, params) || console.log('fallback message');
```

### 4. 测试验证

每完成一个文件的迁移：

1. ✅ **语法检查**: `node -c file.js`
2. ✅ **功能测试**: 运行对应功能
3. ✅ **日志验证**: 检查控制台输出
4. ✅ **Git提交**: 独立commit，便于回滚

---

## 迁移步骤（标准流程）

### Step 1: 分析文件
```bash
# 统计console.log数量
grep -c "console\.log" file.js

# 查看分布
grep -n "console\.log" file.js | head -20
```

### Step 2: 设计常量

根据日志内容，在`log-constants.js`添加常量：

```javascript
// ========== [Module Name] ==========
MODULE_ACTION_START: 'Action started',
MODULE_ACTION_PROGRESS: 'Progress: {percent}%',
MODULE_ACTION_COMPLETE: 'Action completed: {result}',
MODULE_ACTION_ERROR: 'Action failed: {error}',
```

### Step 3: 批量迁移

**每批处理10-20个日志**，分类处理：
- 信息日志（info）
- 调试日志（debug）
- 警告日志（warn）
- 错误日志（error）

### Step 4: 验证

```bash
# 语法检查
node -c file.js

# 同步到Eagle
./sync-once.sh

# 重启测试
killall Eagle && open -a Eagle
```

### Step 5: 提交

```bash
git add file.js log-constants.js
git commit -m "migrate: [module] console.log → pixlyLog

- Migrated X console.log statements
- Added Y new LOG constants
- Categories: info(A), debug(B), warn(C), error(D)

Tested: ✅ Syntax OK, ✅ Functionality verified"
```

---

## 常量命名规范

### 格式
```
[MODULE]_[ACTION]_[DETAIL]
```

### 示例
```javascript
// 模块名_动作_详情
IMAGE_CONV_START           // 图像转换 - 开始
IMAGE_CONV_PROGRESS        // 图像转换 - 进度
IMAGE_CONV_FILE_ANALYZED   // 图像转换 - 文件分析
IMAGE_CONV_ERROR_ANALYZE   // 图像转换 - 分析错误
IMAGE_CONV_SUCCESS         // 图像转换 - 成功
IMAGE_CONV_COMPLETE        // 图像转换 - 完成

RUST_CLI_EXEC              // Rust CLI - 执行
RUST_CLI_STDOUT            // Rust CLI - 标准输出
RUST_CLI_STDERR            // Rust CLI - 错误输出
RUST_CLI_FAILED            // Rust CLI - 执行失败

FILE_HANDLER_SELECT_START  // 文件处理 - 选择开始
FILE_HANDLER_SELECT_DONE   // 文件处理 - 选择完成
FILE_HANDLER_VALIDATE      // 文件处理 - 验证
```

### 参数命名
```javascript
// 清晰的参数名
{ file: 'test.jpg', size: '1.2MB', format: 'jxl' }

// 避免
{ f: 'test.jpg', s: '1.2MB', fmt: 'jxl' }
```

---

## 质量检查清单

每个文件迁移完成后检查：

- [ ] 所有console.log都已迁移？
- [ ] 新常量已添加到log-constants.js？
- [ ] 常量命名符合规范？
- [ ] 参数化正确（无硬编码字符串）？
- [ ] 保留了fallback？
- [ ] 语法检查通过（node -c）？
- [ ] 功能测试通过？
- [ ] Git commit信息清晰？

---

## 预期收益

### 1. 统一管理
- 所有日志消息集中在一个文件
- 易于搜索、修改、审查

### 2. 国际化友好
- 未来可轻松支持多语言日志
- 常量可映射到不同语言

### 3. 级别控制
- 支持动态日志级别（ERROR/WARN/INFO/DEBUG/TRACE）
- 生产环境可关闭DEBUG日志

### 4. 参数化
- 避免字符串拼接
- 便于日志分析和搜索

### 5. 跨平台收集
- 统一格式便于日志收集器处理
- 支持JSON结构化输出

---

## 当前任务

### 🎯 Next: Phase 1 - image-conversion.js

**目标**: 迁移60个console.log

**预计时间**: 30-45分钟

**步骤**:
1. 分析日志分布和类型
2. 设计20-30个新常量
3. 分批迁移（每批10-15个）
4. 测试验证
5. 提交

---

**开始时间**: 2025-11-10 11:45
**预计完成Phase 1**: 2025-11-10 14:00
**预计完成所有**: 2-3天
