# Eagle 插件重新加载指南

## 问题

Eagle 中加载的插件使用的是旧代码（搜索 `pixly-converter`），需要重新加载使用新的 `pixly-eagle-core` 共享二进制。

## 解决步骤

### 1. 已完成的准备工作 ✅

```bash
# ai-vue-refactor
cd plugin/ai-vue-refactor
npm run build  # ✅ 完成
rm bin/pixly-converter  # ✅ 清理旧文件
cp bin/pixly-eagle-core dist/bin/  # ✅ 复制新二进制

# format-vue
cd plugin/format-vue
npm run build  # ✅ 完成
rm bin/pixly-converter  # ✅ 清理旧文件
cp bin/pixly-eagle-core dist/bin/  # ✅ 复制新二进制
```

### 2. 在 Eagle 中重新加载插件

**方法 A - 重新安装** (推荐):
1. 在 Eagle 中**卸载**旧的插件
2. 重新安装 `dist/` 目录为插件
3. 打开插件

**方法 B - 刷新**:
1. 在 Eagle 插件管理器中**禁用**插件
2. **启用**插件
3. 打开插件

**方法 C - 重启 Eagle**:
1. 完全退出 Eagle
2. 重新启动 Eagle
3. 打开插件

### 3. 验证新代码已加载

打开插件后，检查浏览器控制台：

**预期日志**（新代码）:
```
[PIXLY INFO] [rust.cli.exec] Searching for pixly-eagle-core
[PIXLY INFO] [rust.cli.exec] Eagle environment detected { EAGLE_PLUGIN: 'true' }
[PIXLY INFO] [rust.cli.exec] ✅ Found pixly-eagle-core
```

**旧日志**（需要重新加载）:
```
[PIXLY INFO] [rust.cli.exec] Searching for pixly-converter  ❌ 旧版本
[PIXLY ERROR] [rust.cli.error] pixly-converter not found
```

### 4. 测试转换功能

1. 选择一张图片
2. 尝试转换（任意格式）
3. 观察是否报错

## dist/ 目录结构验证

确保 `dist/` 目录包含以下内容：

```
plugin/ai-vue-refactor/dist/
├── index.html
├── assets/
│   ├── index-xxxxx.css
│   └── index-xxxxx.js
└── bin/
    └── pixly-eagle-core  ✅ 5.1MB

plugin/format-vue/dist/
├── index.html
├── assets/
│   ├── index-xxxxx.css
│   └── index-xxxxx.js
└── bin/
    └── pixly-eagle-core  ✅ 5.1MB
```

## 下一步

1. 在 Eagle 中重新加载插件（选择方法 A/B/C）
2. 测试插件是否正常工作
3. 如果还有问题，查看控制台日志并反馈
