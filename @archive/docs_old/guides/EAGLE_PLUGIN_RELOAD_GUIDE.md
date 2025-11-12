# 🦅 Eagle插件重新加载指南

**问题**: 插件显示 "No available strategy for format: jxl"  
**原因**: Eagle可能在使用缓存的旧binary  
**状态**: Binary已更新 (2025-11-06 10:45)，但Eagle未加载新版本

---

## ✅ 验证状态

### 1. Binary确认正常 ✅
```bash
$ /path/to/pixly-rust/target/release/pixly-rust convert test.gif test.jxl

输出:
📋 Registering conversion strategies...  ← 新日志
  ✓ CLI JXL (cjxl) - Available: true    ← 新日志
✅ Conversion successful!
```

### 2. Node.js环境确认正常 ✅
```javascript
const { execSync } = require('child_process');
execSync('pixly-rust convert test.gif test.jxl');
// 输出包含所有新日志
```

### 3. Eagle插件异常 ❌
```
❌ Conversion failed: No available strategy for format: jxl
(没有看到"Registering conversion strategies"日志)
```

---

## 🔄 如何让Eagle加载新Binary

### 方案1: 完全重启Eagle（推荐）

#### macOS
```bash
# 1. 完全退出Eagle
# 方法A: 使用菜单 Eagle → Quit Eagle
# 方法B: 使用快捷键 Cmd+Q
# 方法C: 使用终端强制退出
killall Eagle

# 2. 等待3秒确保进程完全退出
sleep 3

# 3. 重新打开Eagle
open -a Eagle
```

#### 为什么需要完全重启？
- Eagle是长期运行的应用
- 插件在首次加载时可能会缓存binary路径/状态
- Node.js child_process可能缓存了旧的进程
- 只有完全重启才能清除所有缓存

### 方案2: 重新加载插件（可能不够）

如果Eagle支持插件热重载：
1. 打开Eagle插件管理
2. 禁用Pixly插件
3. 等待2秒
4. 启用Pixly插件

**⚠️ 警告**: 这可能不足以清除所有缓存！

### 方案3: 清除Eagle缓存（高级）

```bash
# 1. 完全退出Eagle
killall Eagle

# 2. 清除Eagle的缓存（谨慎操作！）
# 这会清除Eagle的所有缓存，包括其他数据
# rm -rf ~/Library/Caches/Eagle/

# 3. 重启Eagle
open -a Eagle
```

---

## 📊 验证是否加载了新Binary

### 在Eagle中执行转换后，检查控制台日志

#### 如果看到这些，说明成功 ✅
```
📋 Registering conversion strategies...
   Current PATH: /opt/homebrew/bin:...
  ✓ CLI JXL (cjxl) - Available: true
✅ Conversion successful!
```

#### 如果仍然看到这些，说明未加载 ❌
```
❌ Conversion failed: No available strategy for format: jxl
(没有"Registering"日志)
```

---

## 🔍 深入调查（如果重启后仍失败）

### 1. 检查Eagle实际使用的binary路径

在Eagle控制台中查找这一行：
```
[PIXLY Rust CLI] 📁 Path: /Users/.../pixly-rust/target/release/pixly-rust
```

确认这个路径是否正确。

### 2. 手动测试Eagle使用的路径

```bash
# 使用Eagle日志中显示的完整路径
"/Users/.../pixly-rust/target/release/pixly-rust" --version

# 应该显示
Pixly Rust Converter v0.3.0
```

### 3. 检查binary修改时间

```bash
ls -lh "/Users/.../pixly-rust/target/release/pixly-rust"

# 应该显示
-rwxr-xr-x  ... 5.6M Nov  6 10:45 pixly-rust
                        ^^^^^^^^^ 这个时间应该是最新的
```

### 4. 检查Eagle的Node.js环境

在Eagle控制台执行：
```javascript
console.log('PATH:', process.env.PATH);
console.log('Node version:', process.version);
```

确认PATH是否包含 `/opt/homebrew/bin`（cjxl所在位置）

---

## 🎯 预期结果

### 重启Eagle后应该看到：

#### 控制台日志
```
[PIXLY Rust CLI] ✅ Available - Version: 0.3.0
[PIXLY Rust CLI] 🔄 Converting: test.gif → test.jxl
[PIXLY Rust CLI] 📋 Registering conversion strategies...  ← 新日志！
[PIXLY Rust CLI]   ✓ CLI JXL (cjxl) - Available: true    ← 新日志！
[PIXLY Rust CLI] ✅ Conversion successful!
```

#### 转换结果
- ✅ GIF → JXL 成功
- ✅ 文件大小合理（~1-2% of original）
- ✅ 元数据保留
- ✅ 没有错误消息

---

## ⚠️ 如果问题依然存在

### 请提供以下信息：

1. **Eagle版本**
   ```
   Eagle → About Eagle → 版本号
   ```

2. **完整的控制台日志**
   - 从加载插件开始
   - 到转换失败为止
   - 所有日志行

3. **Binary验证结果**
   ```bash
   # 执行并提供输出
   /path/to/pixly-rust/target/release/pixly-rust convert test.gif test.jxl 2>&1
   ```

4. **Eagle进程信息**
   ```bash
   # Eagle是否有多个实例？
   ps aux | grep Eagle
   ```

---

## 🚀 后续计划

如果重启Eagle后仍然失败，可能需要：

1. **添加binary版本检查**
   - 在插件启动时验证binary版本
   - 如果版本不匹配，显示警告

2. **添加binary重新加载机制**
   - 插件检测binary修改时间
   - 自动重新加载binary

3. **改进错误消息**
   - 明确指出binary版本不匹配
   - 提供重新加载说明

---

**创建时间**: 2025-11-06 11:00  
**Binary版本**: 0.3.0  
**Binary修改时间**: 2025-11-06 10:45

**🎯 下一步：请完全重启Eagle，然后重新测试转换。**
