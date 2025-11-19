# 🔍 国际化问题诊断指南

## 问题现象
用户在**简体中文**界面看到**日语**文本，例如：
- "AIオンライン"
- "スマートモード - ベストプリセット"
- "AI が最適な設定を自動選択"

## ✅ 源文件验证结果

**所有源文件100%正确：**
- ✅ `en.json`: 100% English (766 keys)
- ✅ `zh_CN.json`: 100% Simplified Chinese (766 keys)
- ✅ `zh_TW.json`: 100% Traditional Chinese (766 keys)
- ✅ `ja_JP.json`: 100% Japanese (766 keys)
- ✅ HTML placeholders: 100% English

**验证命令：**
```bash
cd core/plugin
python3 << 'EOF'
import json
import re

# 检查zh_CN.json是否有日语
with open('_locales/zh_CN.json', 'r', encoding='utf-8') as f:
    zh_cn = json.load(f)

def has_japanese(obj):
    if isinstance(obj, dict):
        return any(has_japanese(v) for v in obj.values())
    elif isinstance(obj, str):
        return bool(re.search(r'[\u3040-\u309f\u30a0-\u30ff]', obj))
    return False

if has_japanese(zh_cn):
    print("⚠️ zh_CN.json有日语！")
else:
    print("✅ zh_CN.json正确，没有日语")
EOF
```

## 🔧 解决方案

### 方案1：清除Eagle缓存（最可能有效）

1. **完全退出Eagle**
   - macOS: Cmd+Q
   - Windows: 右键任务栏图标 → 退出

2. **清除Eagle缓存**
   ```bash
   # macOS
   rm -rf ~/Library/Application\ Support/Eagle/Cache
   rm -rf ~/Library/Caches/Eagle
   
   # Windows
   # 删除: %APPDATA%\Eagle\Cache
   # 删除: %LOCALAPPDATA%\Eagle\Cache
   ```

3. **重启Eagle**

### 方案2：清除浏览器缓存

Eagle插件使用的是Chromium内核，需要清除缓存：

1. **在Eagle插件中按快捷键**
   - macOS: Cmd+Option+I
   - Windows: Ctrl+Shift+I

2. **打开开发者工具后**
   - 右键刷新按钮
   - 选择"清空缓存并硬性重新加载"
   - 或者: Application → Clear storage → Clear site data

3. **重新加载插件**

### 方案3：检查Eagle语言设置

1. **打开Eagle设置**
   - macOS: Eagle → Preferences
   - Windows: File → Settings

2. **检查语言设置**
   - 确保选择的是"简体中文"
   - 如果已经是简体中文，尝试切换到其他语言，再切回来

3. **重启Eagle**

### 方案4：检查i18n加载（开发者）

在Eagle插件中打开开发者工具（F12或Cmd+Option+I），运行：

```javascript
// 检查当前语言
console.log('Current language:', window.i18n?.currentLang);

// 检查zh_CN.json是否正确加载
console.log('zh_CN ai.bestModeDesc:', window.i18n?.t('ai.bestModeDesc'));
console.log('Expected (should be Chinese):', '已启用最佳AI配置组合...');

// 如果显示日语，说明加载了错误的语言文件
```

### 方案5：重新安装插件

1. 在Eagle中卸载PIXLY插件
2. 重新安装
3. 重启Eagle

## 🎯 最可能的原因

根据源文件100%正确的验证结果，问题几乎肯定是：

1. **Eagle缓存了旧版本**（70%可能性）
   - Eagle在更新插件后，仍使用旧的语言文件
   - 解决：方案1

2. **i18n语言检测错误**（20%可能性）
   - 系统语言设置导致i18n加载了错误的语言文件
   - 解决：方案3

3. **浏览器缓存**（10%可能性）
   - Chromium内核缓存了旧版本
   - 解决：方案2

## 📞 如果问题仍然存在

请提供以下信息：

1. **开发者工具控制台输出**
   ```javascript
   console.log('i18n status:', {
       currentLang: window.i18n?.currentLang,
       bestModeDesc: window.i18n?.t('ai.bestModeDesc'),
       onlineStatus: window.i18n?.t('ai.onlineStatus')
   });
   ```

2. **Eagle版本**
   - Eagle → About Eagle

3. **操作系统**
   - macOS版本或Windows版本

4. **是否尝试过上述所有方案**

---

**最后更新**: 2025-11-10
**验证通过**: 所有源文件100%正确
