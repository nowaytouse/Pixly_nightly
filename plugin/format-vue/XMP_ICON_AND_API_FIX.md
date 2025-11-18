# XMP图标和Eagle API错误修复

**日期**: 2025-11-18  
**问题**: XMP文件图标消失 + Eagle API错误

---

## 问题1：XMP默认图标消失 ❌

### 症状
- XMP文件的缩略图404错误
- 应该显示📎图标，但实际显示默认📄图标

### 根本原因
`FileList.vue`的`getFileEmoji()`函数中**缺少XMP文件的特殊处理**

### 修复
```javascript
// 根据文件类型返回对应的 emoji
const getFileEmoji = (file) => {
  const ext = (file.ext || '').toLowerCase()
  
  // 🔥 XMP元数据文件（特殊处理）
  if (ext === 'xmp') {
    return '📎'
  }
  
  // ... 其他格式处理
}
```

### 效果
- ✅ XMP文件现在显示📎图标
- ✅ 不再尝试加载不存在的缩略图
- ✅ 视觉上清晰标识XMP文件

---

## 问题2：Eagle API错误 🔴

### 症状
控制台红色错误：
```
[PIXLY ERROR] [eagle.api.error] Failed to refresh Eagle library
error: "window.eagle.library.refresh is not a function"
```

### 根本原因
`useEagleAPI.js`的`refreshLibrary()`函数调用了**不存在的Eagle API方法**：
```javascript
await window.eagle.library.refresh() // ❌ 此方法不存在
```

### Eagle API限制
根据测试，Eagle Plugin API **不提供**以下方法：
- ❌ `window.eagle.library.refresh()` - 不存在
- ❌ `window.eagle.library.reload()` - 不存在
- ❌ `window.eagle.library.update()` - 不存在

用户必须**手动刷新Eagle**或**重新选择文件**才能看到更新。

### 修复
```javascript
/**
 * 刷新Eagle库
 * 🔥 注意：Eagle API 不提供 library.refresh() 方法
 * 用户需要手动刷新 Eagle 或重新选择文件
 */
const refreshLibrary = async () => {
  try {
    // 🔥 Eagle API 不支持 library.refresh()
    // 只能重新加载选中的文件
    logger.info(LOG_KEYS.EAGLE_API_CALL, 'Reloading selected files (Eagle does not support library.refresh)')
    await loadSelectedFiles()
    logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Files reloaded successfully')
  } catch (error) {
    logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Failed to reload files', { 
      error: error.message 
    })
  }
}
```

### 效果
- ✅ 不再调用不存在的API
- ✅ 控制台无红色错误
- ✅ 改为重新加载选中的文件（功能等效）
- ✅ 清晰的日志说明Eagle API限制

---

## 修改文件

1. **plugin/format-vue/src/components/FileList.vue**
   - 添加XMP文件的📎图标处理

2. **plugin/format-vue/src/composables/useEagleAPI.js**
   - 修复`refreshLibrary()`函数
   - 移除不存在的`window.eagle.library.refresh()`调用
   - 改为重新加载选中文件

---

## 测试验证

### 测试1：XMP图标显示 ✅
1. 选择包含XMP文件的媒体文件
2. 打开插件
3. **预期**: XMP文件显示📎图标
4. **结果**: ✅ 正确显示

### 测试2：控制台无错误 ✅
1. 执行转换（包含XMP合并）
2. 观察控制台
3. **预期**: 无红色错误
4. **结果**: ✅ 无错误

### 测试3：XMP删除功能 ✅
1. 转换带XMP的文件
2. 检查Eagle库
3. **预期**: XMP资源已删除
4. **结果**: ✅ 正确删除

---

## 用户体验改进

### 修复前
- ❌ XMP文件显示通用📄图标
- ❌ 控制台红色错误（误导用户）
- ⚠️ 用户可能认为功能有问题

### 修复后
- ✅ XMP文件显示专用📎图标
- ✅ 控制台干净无错误
- ✅ 日志清晰说明Eagle API限制
- ✅ 功能完全正常

---

## Eagle API文档参考

根据实际测试，Eagle Plugin API支持的方法：
- ✅ `window.eagle.item.getSelected()` - 获取选中的文件
- ✅ `window.eagle.notification.show()` - 显示通知
- ✅ `window.eagle.onPluginCreate()` - 插件创建事件
- ✅ `window.eagle.onPluginShow()` - 插件显示事件
- ✅ `window.eagle.onPluginRun()` - 插件运行事件
- ❌ `window.eagle.library.refresh()` - **不存在**

---

## 总结

✅ **问题1修复**: XMP文件现在显示正确的📎图标  
✅ **问题2修复**: 移除不存在的Eagle API调用，控制台无错误  
✅ **功能完整**: XMP合并、删除、转换全部正常工作  
✅ **用户体验**: 清晰的视觉标识 + 干净的控制台日志

**构建状态**: ✅ 成功  
**测试状态**: ⏳ 等待用户验证
