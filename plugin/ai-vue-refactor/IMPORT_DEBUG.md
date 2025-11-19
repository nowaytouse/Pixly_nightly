# Eagle插件导入调试指南

## 当前状态检查

### 1. 文件完整性
```bash
✅ manifest.json - 存在且JSON有效
✅ logo.png - 存在 (256x256 PNG)
✅ dist/index.html - 存在 (446字节)
✅ dist/assets/ - 存在 (JS和CSS文件)
```

### 2. manifest.json配置
```json
{
  "id": "com.pixly.ai.vue",
  "version": "3.0.0",
  "name": "PIXLY AI Vue",
  "main": {
    "url": "dist/index.html",
    "frameless": true,
    "titleBarStyle": "hidden"
  }
}
```

### 3. 与format-vue对比
- ✅ 相同的目录结构
- ✅ 相同的manifest格式
- ✅ 相同的frameless配置

## 可能的问题

### 问题1: Eagle缓存
**症状**: 修改后仍然无法导入
**解决**: 
1. 完全退出Eagle
2. 删除Eagle缓存: `~/Library/Application Support/Eagle/`
3. 重新启动Eagle
4. 重新导入插件

### 问题2: 插件ID冲突
**症状**: 提示"插件已存在"
**解决**:
1. 在Eagle中完全删除旧的PIXLY AI插件
2. 确认插件列表中没有`com.pixly.ai.vue`
3. 重新导入

### 问题3: 文件权限
**症状**: 无法读取文件
**解决**:
```bash
chmod -R 755 plugin/ai-vue-refactor/
chmod 644 plugin/ai-vue-refactor/manifest.json
chmod 644 plugin/ai-vue-refactor/logo.png
```

### 问题4: 构建产物损坏
**症状**: 加载后白屏或报错
**解决**:
```bash
cd plugin/ai-vue-refactor
rm -rf dist node_modules
npm install
npm run build
```

## 导入步骤

1. **清理环境**
   ```bash
   # 删除旧的开发版index.html（已完成）
   rm plugin/ai-vue-refactor/index.html
   ```

2. **验证构建**
   ```bash
   cd plugin/ai-vue-refactor
   npm run build
   ls -la dist/
   ```

3. **在Eagle中导入**
   - 打开Eagle
   - 插件 → 开发者 → 导入插件
   - 选择 `plugin/ai-vue-refactor` 目录
   - 确认导入

4. **检查日志**
   - 如果失败，查看Eagle控制台（开发者工具）
   - 查看错误信息

## 紧急修复

如果仍然无法导入，尝试完全复制format-vue的配置：

```bash
# 备份当前manifest
cp plugin/ai-vue-refactor/manifest.json plugin/ai-vue-refactor/manifest.json.bak

# 使用format-vue的manifest作为模板
cp plugin/format-vue/manifest.json plugin/ai-vue-refactor/manifest.json

# 修改ID和名称
# 手动编辑manifest.json，只改id和name字段
```

## 下一步

如果以上都不行，需要：
1. 查看Eagle的实际错误日志
2. 对比两个插件的二进制差异
3. 检查Eagle版本兼容性
