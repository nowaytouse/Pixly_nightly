# 清除语言缓存

## 问题
简体中文界面显示日语，因为localStorage中保存了错误的语言偏好（ja_JP）。

## 解决方案

### 方法1：在Eagle插件中运行（推荐）

1. 打开PIXLY插件
2. 按 `Cmd+Option+I` (macOS) 或 `Ctrl+Shift+I` (Windows) 打开开发者工具
3. 在Console中粘贴以下代码：

```javascript
// 清除语言偏好
localStorage.removeItem('pixly_language');
console.log('✅ 语言偏好已清除');

// 设置为简体中文
localStorage.setItem('pixly_language', 'zh_CN');
console.log('✅ 已设置为简体中文');

// 重新加载插件
location.reload();
```

### 方法2：使用语言选择器

1. 在PIXLY插件界面找到语言选择器（EN / ZH-CN / ZH-TW / JP）
2. 点击 **ZH-CN**
3. 这会自动保存简体中文偏好

## 为什么会这样？

从控制台日志：
```
i18n.fixed.js:33 [i18n] 💾 User preference found: ja_JP
```

说明localStorage中存储了`ja_JP`，可能是：
1. 之前测试时切换过语言到日语
2. 某次操作意外保存了日语设置

## 验证

清除后重新加载插件，应该看到：
```
[i18n] 📥 Loading translation file: zh_CN
[i18n] ✅ Translation loaded: zh_CN
```

而不是：
```
[i18n] 💾 User preference found: ja_JP
[i18n] 📥 Loading translation file: ja_JP
```
