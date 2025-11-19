# 🚨 Critical Issues - JS Modules Violate Architecture

## 发现的严重违规

### 1. format-recommender.js - 重复实现AI功能
**问题**: JS本地推荐格式，没有调用Go AI
**日志**: `AI recommended format: jxl for gif`
**真相**: 这不是AI，是JS硬编码规则
**违反**: 真实性原则，Go职责

### 2. 智能模式失效
**问题**: 所有格式都推荐JXL
**原因**: format-recommender.js硬编码逻辑
**应该**: 调用Go AI Service (端口50052)

### 3. XMP被删除
**问题**: 转换后XMP文件丢失
**需要**: 检查metadata.rs XMP处理

### 4. JS重复造轮子
- deduplicator.js (应由Rust)
- file-handler.js (应由Rust)
- format-selector.js (简单UI)
- i18n-helpers.js (重复)
- log-manager.js (重复)

## 立即行动

### Priority 0: 修复智能模式
```javascript
// 错误做法 (当前)
const recommendation = window.recommendFormat(imageInfo);  // JS本地
recommendedFormat = recommendation.format || 'jxl';

// 正确做法
const ai_response = await fetch('http://localhost:50052/api/v1/predict', {
    method: 'POST',
    body: JSON.stringify(imageInfo)
});
const params = await ai_response.json();
recommendedFormat = params.format;  // Go AI决定
```

### Priority 1: 废弃重复模块
- format-recommender.js → DEPRECATED
- deduplicator.js → DEPRECATED  
- format-selector.js → DEPRECATED

### Priority 2: 修复XMP删除

### Priority 3: 添加HEIC动态图警告

