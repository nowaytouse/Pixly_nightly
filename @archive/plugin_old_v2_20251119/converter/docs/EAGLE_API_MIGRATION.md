# Eagle官方API迁移指南

## 📋 概述

本文档记录了从自定义实现迁移到Eagle官方API的过程。

---

## ✅ 已完成：日志系统迁移

### 旧实现
```javascript
console.log('[PIXLY] message');
window.addLog('message', 'info');
```

### 新实现（Eagle官方API）
```javascript
// 使用Eagle官方日志API
eagle.log.debug(obj);   // 调试日志
eagle.log.info(obj);    // 信息日志
eagle.log.warn(obj);    // 警告日志
eagle.log.error(obj);   // 错误日志
```

### PIXLY封装（推荐使用）
```javascript
// 快捷方法
Logger.debug('ModuleName', 'Debug message');
Logger.info('ModuleName', 'Info message');
Logger.warn('ModuleName', 'Warning message');
Logger.error('ModuleName', 'Error message');
Logger.success('ModuleName', 'Success message');

// 或使用通用方法
Logger.log('ModuleName', 'Message', 'info');

// 兼容旧API
window.addLog('Message', 'info');
```

### 特性
- ✅ 自动记录到Eagle日志系统
- ✅ 仅warning和error显示在UI中
- ✅ 5秒去重窗口防止日志刷屏
- ✅ 保留旧API兼容性

---

## 📝 待迁移：国际化系统

### 当前实现
- 自定义i18n系统：`i18n.fixed.js`
- 手动加载JSON文件
- 自定义翻译逻辑

### Eagle官方实现
Eagle插件内建`i18next`模块，提供完整的国际化支持。

#### 步骤1：文件结构
```
_locales/
  ├── en.json
  ├── zh_CN.json
  ├── zh_TW.json
  └── ja_JP.json
```

#### 步骤2：manifest.json配置
```json
{
  "name": "{{manifest.app.name}}",
  "fallbackLanguage": "zh_CN",
  "languages": ["en", "zh_TW", "zh_CN", "ja_JP"]
}
```

#### 步骤3：JSON文件格式
```json
{
  "manifest": {
    "app": {
      "name": "PIXLY - 图像转换"
    }
  },
  "ui": {
    "convert": "转换",
    "cancel": "取消",
    "settings": "设置"
  }
}
```

#### 步骤4：代码中使用
```javascript
// 获取翻译
let text = i18next.t('ui.convert');

// 当前语言
let locale = eagle.app.locale;

// HTML中使用
<button data-i18n="ui.convert">Convert</button>
```

### 支持的语言
- `en` - English
- `ja_JP` - 日本語
- `es_ES` - Español
- `de_DE` - Deutsch
- `zh_TW` - 繁體中文
- `zh_CN` - 简体中文
- `ko_KR` - 한국어
- `ru_RU` - Русский

---

## 🔄 迁移计划

### Phase 1: 日志系统 ✅ 已完成
- [x] 重构logger.js使用Eagle API
- [x] 保留UI日志功能
- [x] 保持向后兼容
- [x] 测试验证

### Phase 2: 国际化系统 ✅ 已完成
- [x] 分析现有i18n.fixed.js
- [x] 迁移到Eagle i18next
- [x] 更新manifest.json
- [x] 重写i18n.fixed.js
- [x] 保持API向后兼容
- [x] 创建迁移文档
- [x] 备份旧系统

### Phase 3: 其他API优化
- [ ] 通知系统（已使用eagle.notification）
- [ ] 文件操作（已使用eagle.item）
- [ ] 对话框系统（已使用eagle.dialog）

---

## 📚 参考文档

- [Eagle日志API](https://developer.eagle.cool/plugin-api/zh-cn/api/log)
- [Eagle国际化教程](https://developer.eagle.cool/plugin-api/zh-cn/tutorial/i18n)
- [i18next官方文档](https://www.i18next.com/overview/getting-started)
- [Eagle插件示例](https://github.com/eagle-app/eagle-plugin-examples/tree/main/i18n)

---

## ⚠️ 注意事项

1. **日志系统**
   - Eagle日志会记录到Eagle软件日志中
   - 可在Eagle菜单 → 帮助 → 显示日志 中查看
   - 不要记录敏感信息

2. **国际化系统**
   - 必须使用Eagle支持的语言代码
   - JSON文件必须放在`_locales`目录
   - 语言切换需要重启插件生效

3. **向后兼容**
   - 保留旧API以确保现有代码正常工作
   - 逐步迁移，不要一次性全部修改
   - 充分测试每个迁移步骤

---

最后更新：2025-11-09
