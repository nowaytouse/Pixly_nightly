# 📝 PIXLY日志系统文档

## 设计原则

遵循项目质量宣言（PROJECT_QUALITY_MANIFESTO.md）：

1. **零硬编码日志** - 所有日志使用LOG常量
2. **分级控制** - DEBUG/INFO/WARN/ERROR四个级别
3. **环境感知** - 开发/生产环境自动切换
4. **结构化日志** - 使用参数而不是字符串拼接
5. **可维护性** - 集中管理，易于查找和修改

## 架构

```
config.js (配置)
    ↓
logger.js (日志引擎)
    ↓
log-constants.js (日志消息)
    ↓
*.js (业务代码)
```

## 日志级别

### DEBUG (0)
- **用途**: 开发调试信息
- **环境**: 仅在development模式显示
- **示例**: CLI参数、文件路径、中间状态

```javascript
logger.debug('PIXLY Format', 'CLI execution started', { 
    path: state.rustCorePath,
    args: args.join(' ')
});
```

### INFO (1)
- **用途**: 正常操作信息
- **环境**: 所有环境显示
- **示例**: 初始化完成、文件加载、转换开始

```javascript
logger.info('PIXLY Format', LOG.INIT_COMPLETE);
logger.info('PIXLY Format', LOG.FILE_LOADING_COMPLETE, { count: 5 });
```

### WARN (2)
- **用途**: 警告信息
- **环境**: 所有环境显示
- **示例**: Rust核心未找到、工具缺失

```javascript
logger.warn('PIXLY Format', LOG.RUST_CORE_NOT_FOUND);
```

### ERROR (3)
- **用途**: 错误信息
- **环境**: 所有环境显示
- **示例**: 转换失败、文件读取错误

```javascript
logger.error('PIXLY Format', LOG.CONVERSION_ERROR, {}, error);
```

## 配置

### 开发模式

编辑 `plugin/format/js/config.js`:

```javascript
const config = {
    development: true,   // 启用开发模式
    logLevel: 'DEBUG',   // 显示所有日志
    // ...
};
```

### 生产模式

```javascript
const config = {
    development: false,  // 生产模式
    logLevel: 'INFO',    // 只显示INFO及以上
    // ...
};
```

### 运行时修改

```javascript
// 在浏览器控制台中
logger.setLevel(0);  // 启用DEBUG
logger.setLevel(1);  // 只显示INFO及以上
logger.setLevel(2);  // 只显示WARN及以上
logger.setLevel(3);  // 只显示ERROR
```

## LOG常量

所有日志消息定义在 `log-constants.js`:

```javascript
const LOG = {
    // 初始化
    INIT_COMPLETE: 'Initialization complete',
    
    // 文件加载
    FILE_LOADING_START: 'Starting file selection',
    FILE_LOADING_COMPLETE: 'Loaded {count} files',
    
    // 转换
    CONVERSION_START: 'Starting conversion',
    CONVERSION_COMPLETE: 'Conversion complete',
    
    // 错误
    RUST_CORE_NOT_FOUND: 'Rust core not found',
    CONVERSION_ERROR: 'Conversion failed',
};
```

## 使用规范

### ✅ 正确用法

```javascript
// 使用LOG常量
logger.info('PIXLY Format', LOG.FILE_LOADING_COMPLETE, { count: files.length });

// 使用DEBUG级别进行调试
logger.debug('PIXLY Format', 'Processing file', { name: file.name, size: file.size });

// 错误日志包含error对象
logger.error('PIXLY Format', LOG.CONVERSION_ERROR, { file: file.name }, error);
```

### ❌ 错误用法

```javascript
// ❌ 硬编码日志消息
console.log('[PIXLY Format] 文件加载完成');

// ❌ 字符串拼接
console.log(`[PIXLY Format] Loaded ${count} files`);

// ❌ 直接使用console.log进行调试
console.log('Debug info:', data);

// ❌ 中文日志
logger.info('PIXLY Format', '转换完成');
```

## 添加新日志

### 步骤1: 在log-constants.js中添加常量

```javascript
const LOG = {
    // ... 现有常量 ...
    
    // 新功能
    NEW_FEATURE_START: 'New feature started',
    NEW_FEATURE_COMPLETE: 'New feature completed: {result}',
};
```

### 步骤2: 在代码中使用

```javascript
logger.info('PIXLY Format', LOG.NEW_FEATURE_START);
// ... 执行功能 ...
logger.info('PIXLY Format', LOG.NEW_FEATURE_COMPLETE, { result: 'success' });
```

## 调试技巧

### 查看所有日志

```javascript
// 在浏览器控制台
logger.setLevel(0);  // 显示DEBUG日志
```

### 过滤特定模块

```javascript
// 浏览器控制台过滤
// 输入: PIXLY Format
// 只显示Format插件的日志
```

### 查找日志来源

所有日志都使用LOG常量，可以轻松追踪：

```bash
# 查找某个日志的使用位置
grep -r "LOG.FILE_LOADING_COMPLETE" plugin/format/js/
```

## 性能考虑

### 条件日志

logger会自动检查日志级别，不满足条件的日志不会执行：

```javascript
// 这个调用在生产环境中不会执行任何操作
logger.debug('PIXLY Format', 'Heavy computation', { 
    data: expensiveOperation()  // 不会被调用
});
```

### 延迟求值

对于昂贵的操作，使用函数：

```javascript
logger.debug('PIXLY Format', 'Data dump', { 
    data: () => JSON.stringify(largeObject)  // 只在需要时执行
});
```

## 迁移指南

### 从硬编码日志迁移

**之前**:
```javascript
console.log('[PIXLY Format] 🔍 开始加载文件...');
console.log(`[PIXLY Format] ✅ 已加载 ${count} 个文件`);
```

**之后**:
```javascript
logger.info('PIXLY Format', LOG.FILE_LOADING_START);
logger.info('PIXLY Format', LOG.FILE_LOADING_COMPLETE, { count });
```

### 从调试console.log迁移

**之前**:
```javascript
console.log('CLI path:', path);
console.log('Args:', args);
```

**之后**:
```javascript
logger.debug('PIXLY Format', 'CLI execution', { path, args: args.join(' ') });
```

## 测试

### 单元测试

```javascript
// 测试日志级别
logger.setLevel(LogLevel.ERROR);
logger.info('Test', 'Should not appear');  // 不会输出
logger.error('Test', 'Should appear');     // 会输出
```

### 集成测试

```javascript
// 测试LOG常量存在
assert(LOG.INIT_COMPLETE !== undefined);
assert(LOG.FILE_LOADING_COMPLETE !== undefined);
```

## 总结

✅ 所有日志使用LOG常量  
✅ 分级控制（DEBUG/INFO/WARN/ERROR）  
✅ 环境感知（dev/prod）  
✅ 结构化日志  
✅ 零硬编码  
✅ 易于维护  

**遵循质量宣言，拒绝硬编码！**

---

**文档版本**: 1.0.0  
**最后更新**: 2024-11-17  
**符合**: PROJECT_QUALITY_MANIFESTO.md v3.0.0
