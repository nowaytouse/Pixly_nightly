# PIXLY Logger 使用示例

## 📝 基本用法

### 1. 使用快捷方法（推荐）

```javascript
// 调试日志（仅记录到Eagle日志）
Logger.debug('ModuleName', 'Debug information');

// 信息日志（仅记录到Eagle日志）
Logger.info('ModuleName', 'Operation completed');

// 警告日志（记录到Eagle + 显示在UI）
Logger.warn('ModuleName', 'Warning message');

// 错误日志（记录到Eagle + 显示在UI）
Logger.error('ModuleName', 'Error occurred');

// 成功日志（仅记录到Eagle日志）
Logger.success('ModuleName', 'Success message');
```

### 2. 使用通用方法

```javascript
Logger.log('ModuleName', 'Message', 'info');
Logger.log('ModuleName', 'Warning', 'warning');
Logger.log('ModuleName', 'Error', 'error');
```

### 3. 兼容旧API

```javascript
// 仍然可用，但推荐使用新API
window.addLog('Message', 'info');
```

---

## 🎯 实际应用示例

### 示例1：文件转换模块

```javascript
// image-conversion.js
async function startConversion() {
    Logger.info('Conversion', '开始转换流程');
    
    try {
        // 转换逻辑...
        Logger.success('Conversion', `成功转换 ${count} 个文件`);
    } catch (error) {
        Logger.error('Conversion', `转换失败: ${error.message}`);
    }
}
```

### 示例2：AI预测模块

```javascript
// ai-client.js
async function predict(request) {
    Logger.debug('AI Client', `发送预测请求: ${JSON.stringify(request)}`);
    
    try {
        const response = await fetch(url, options);
        
        if (!response.ok) {
            Logger.warn('AI Client', `AI服务返回错误: ${response.status}`);
            return null;
        }
        
        Logger.info('AI Client', 'AI预测成功');
        return response.data;
    } catch (error) {
        Logger.error('AI Client', `AI预测失败: ${error.message}`);
        throw error;
    }
}
```

### 示例3：文件处理模块

```javascript
// file-handler.js
async function selectFiles() {
    Logger.debug('File Handler', '开始选择文件');
    
    const files = await eagle.item.getSelected();
    
    if (files.length === 0) {
        Logger.warn('File Handler', '未选择任何文件');
        return;
    }
    
    Logger.info('File Handler', `已选择 ${files.length} 个文件`);
}
```

---

## 🔍 日志级别说明

| 级别 | Eagle日志 | UI显示 | 使用场景 |
|------|-----------|--------|----------|
| `debug` | ✅ | ❌ | 开发调试信息 |
| `info` | ✅ | ❌ | 一般信息 |
| `success` | ✅ | ❌ | 成功操作 |
| `warning` | ✅ | ✅ | 警告信息 |
| `error` | ✅ | ✅ | 错误信息 |

---

## 💡 最佳实践

### 1. 使用有意义的模块名

```javascript
// ✅ 好的做法
Logger.info('File Handler', 'Files selected');
Logger.error('AI Client', 'Prediction failed');
Logger.warn('Rust CLI', 'Command timeout');

// ❌ 不好的做法
Logger.info('PIXLY', 'Something happened');
Logger.error('Error', 'Failed');
```

### 2. 提供详细的错误信息

```javascript
// ✅ 好的做法
try {
    await convertImage(file);
} catch (error) {
    Logger.error('Conversion', `转换失败: ${file.name} - ${error.message}`);
}

// ❌ 不好的做法
try {
    await convertImage(file);
} catch (error) {
    Logger.error('Conversion', 'Failed');
}
```

### 3. 合理使用日志级别

```javascript
// ✅ 好的做法
Logger.debug('Module', '详细的调试信息');  // 开发时使用
Logger.info('Module', '正常操作信息');     // 记录流程
Logger.warn('Module', '可恢复的问题');     // 用户应该知道
Logger.error('Module', '严重错误');        // 必须显示给用户

// ❌ 不好的做法
Logger.error('Module', '文件已选择');  // 这不是错误
Logger.debug('Module', '转换失败');    // 错误应该用error级别
```

### 4. 避免日志刷屏

```javascript
// ✅ 好的做法
Logger.info('Progress', `转换进度: ${progress}%`);  // 自动去重

// ❌ 不好的做法
for (let i = 0; i < 1000; i++) {
    console.log(`Processing ${i}`);  // 会刷屏
}
```

---

## 🛠️ 高级功能

### 1. 日志去重

Logger自动去重5秒内的相同日志：

```javascript
// 这些日志只会记录一次
Logger.info('Module', 'Same message');
Logger.info('Module', 'Same message');  // 被抑制
Logger.info('Module', 'Same message');  // 被抑制

// 5秒后才会再次记录
setTimeout(() => {
    Logger.info('Module', 'Same message');  // 会记录
}, 6000);
```

### 2. 清空UI日志

```javascript
// 清空UI显示的日志（不影响Eagle日志）
Logger.clearLog();
```

### 3. 导出日志

```javascript
// 导出UI日志为文本
const logText = Logger.exportLog();
console.log(logText);
```

---

## 📊 查看Eagle日志

1. 打开Eagle软件
2. 菜单栏 → 帮助 → 显示日志
3. 搜索 `[PIXLY]` 或模块名

---

## 🔄 从旧API迁移

### 旧代码
```javascript
console.log('[PIXLY] Starting conversion');
window.addLog('转换开始', 'info');
```

### 新代码
```javascript
Logger.info('Conversion', 'Starting conversion');
```

### 迁移步骤
1. 找到所有 `console.log('[PIXLY]')`
2. 替换为 `Logger.debug()` 或 `Logger.info()`
3. 找到所有 `window.addLog()`
4. 替换为对应的 `Logger` 方法
5. 测试验证

---

## ⚠️ 注意事项

1. **不要记录敏感信息**
   ```javascript
   // ❌ 不要这样做
   Logger.info('Auth', `API Key: ${apiKey}`);
   
   // ✅ 应该这样做
   Logger.info('Auth', 'API Key loaded');
   ```

2. **Eagle日志会持久化**
   - Eagle日志会保存到文件
   - 用户可以查看历史日志
   - 不要记录临时调试信息到info/warn/error

3. **UI日志仅显示重要信息**
   - 仅warning和error显示在UI
   - 不要滥用error级别
   - UI日志会自动滚动和限制数量

---

最后更新：2025-11-09
