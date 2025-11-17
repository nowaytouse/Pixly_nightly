# 🚨 紧急修复指南

## 问题：无法导入插件

### 已修复的问题
✅ 移除了 `minWidth` 和 `minHeight` 参数（Eagle可能不支持）

### 当前状态
- ✅ AI插件 manifest.json 格式正确
- ✅ Format插件 manifest.json 格式正确
- ✅ 所有JSON文件格式验证通过

---

## 立即尝试的解决方案

### 方案1：清除Eagle缓存
```bash
# macOS
rm -rf ~/Library/Application\ Support/Eagle/plugins/*
# 然后重新导入插件
```

### 方案2：使用旧版本插件
如果新插件无法导入，使用旧版本：
```bash
# 导入旧版本插件
plugin/old/converter/
```

### 方案3：检查Eagle版本
确保Eagle版本 >= 3.0，支持插件API

### 方案4：检查插件ID冲突
如果之前安装过同ID的插件，需要先卸载：
1. 打开Eagle
2. 插件 → 管理插件
3. 找到并卸载旧插件
4. 重新导入新插件

---

## 详细排查步骤

### 1. 检查Eagle日志
```bash
# macOS
tail -f ~/Library/Logs/Eagle/main.log
```

### 2. 验证插件文件完整性
```bash
# 检查必需文件
ls -la plugin/ai/manifest.json
ls -la plugin/ai/index.html
ls -la plugin/ai/logo.png
ls -la plugin/ai/js/ai-core.js
ls -la plugin/ai/js/i18n.js
```

### 3. 测试JSON格式
```bash
# 验证manifest.json
python3 -m json.tool plugin/ai/manifest.json
python3 -m json.tool plugin/format/manifest.json
```

### 4. 检查文件权限
```bash
# 确保文件可读
chmod -R 755 plugin/ai/
chmod -R 755 plugin/format/
```

---

## 当前manifest.json配置

### AI插件
```json
{
    "id": "pixly-ai-smart-converter",
    "version": "2.0.0",
    "platform": "all",
    "arch": "all",
    "name": "PIXLY AI - 智能多媒体处理",
    "logo": "logo.png",
    "keywords": ["AI", "智能转换", "自动优化", "多媒体", "机器学习"],
    "devTools": false,
    "fallbackLanguage": "zh_CN",
    "languages": ["en", "zh_TW", "zh_CN", "ja_JP"],
    "main": {
        "url": "index.html",
        "width": 1400,
        "height": 900
    }
}
```

### Format插件
```json
{
    "id": "pixly-format-professional-converter",
    "version": "2.0.0",
    "platform": "all",
    "arch": "all",
    "name": "PIXLY Format - 专业格式转换",
    "logo": "logo.png",
    "keywords": ["格式转换", "JXL", "AVIF", "WebP", "HEIC", "专业控制"],
    "devTools": false,
    "fallbackLanguage": "zh_CN",
    "languages": ["en", "zh_TW", "zh_CN", "ja_JP"],
    "main": {
        "url": "index.html",
        "width": 1600,
        "height": 1000
    }
}
```

---

## 如果仍然无法导入

### 创建最小测试插件
```bash
# 创建测试插件
mkdir -p plugin/test
cat > plugin/test/manifest.json << 'EOF'
{
    "id": "test-plugin",
    "version": "1.0.0",
    "platform": "all",
    "arch": "all",
    "name": "Test Plugin",
    "logo": "logo.png",
    "main": {
        "url": "index.html",
        "width": 800,
        "height": 600
    }
}
EOF

cat > plugin/test/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Test</title>
</head>
<body>
    <h1>Test Plugin</h1>
    <p>If you can see this, Eagle plugin system is working.</p>
</body>
</html>
EOF

# 复制logo
cp plugin/ai/logo.png plugin/test/
```

然后尝试导入 `plugin/test/`，如果能导入，说明Eagle插件系统正常。

---

## 可能的原因

### 1. Eagle版本过旧
- 需要 Eagle 3.0+
- 检查：Eagle → 关于 Eagle

### 2. 插件ID冲突
- 之前安装过相同ID的插件
- 解决：先卸载旧插件

### 3. 文件路径问题
- 确保路径中没有特殊字符
- 确保没有中文路径

### 4. 权限问题
- 确保文件可读
- 运行：`chmod -R 755 plugin/`

### 5. JSON格式问题
- 已验证：所有JSON格式正确 ✅

---

## 回滚到旧版本

如果新插件完全无法使用，使用旧版本：

```bash
# 旧版本路径
plugin/old/converter/

# 导入步骤
1. 打开Eagle
2. 插件 → 开发者 → 加载插件文件夹
3. 选择 plugin/old/converter/
```

---

## 联系支持

如果以上方法都无效，请提供：
1. Eagle版本号
2. 操作系统版本
3. 错误信息（如果有）
4. Eagle日志文件

---

**当前状态**：
- ✅ JSON格式正确
- ✅ 文件结构完整
- ✅ 移除了不支持的参数

**下一步**：
1. 重启Eagle
2. 尝试导入插件
3. 如果失败，查看Eagle日志
4. 提供错误信息以便进一步诊断
