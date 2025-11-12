# 🔧 插件UI「AI离线」问题 - 解决方案

## 📋 问题说明

### 现象
- ✅ GO AI服务已正常运行（端口50052）
- ✅ 命令行测试正常工作
- ❌ **Eagle插件UI仍显示「AI离线」**

### 原因
插件使用**5分钟缓存机制**避免频繁检测：
- 首次打开Eagle时，GO服务未启动 → 检测失败 → 缓存「离线」状态
- 后台手动启动GO服务后 → 缓存未刷新 → **UI仍显示离线**

---

## ✅ 解决方法（3种）

### 方法1：一键修复（推荐）⭐

1. **找到插件UI上的🔧修复按钮**
   - 位置：AI智能模式旁边
   - 显示：「❌ AI离线 [🔧 修复]」

2. **点击🔧修复按钮**
   - 插件会：
     - 清除缓存
     - 重新检测服务
     - 刷新UI状态

3. **看到提示**
   ```
   ✅ AI核心已在线
   4.2.0 运行正常
   ```

4. **UI自动更新**
   ```
   ❌ AI离线 → ✅ AI智能模式 ✅
   ```

---

### 方法2：重启Eagle

1. 完全退出Eagle应用
2. 确保GO服务已运行：
   ```bash
   lsof -i :50052
   ```
3. 重新启动Eagle
4. 打开插件 → ✅ AI智能模式

---

### 方法3：等待5分钟

- 缓存会在5分钟后自动失效
- 下次操作时自动重新检测
- **不推荐**：等待时间较长

---

## 🔍 验证GO服务状态

### 检查服务是否运行

```bash
# 方法1：检查端口
lsof -i :50052

# 方法2：HTTP测试
curl http://localhost:50052/api/v1/health

# 方法3：版本检测
curl http://localhost:50052/api/v1/version
```

### 预期输出

```json
{
  "version": "4.2.0",
  "name": "Pixly AI Service",
  "ai_enabled": true,
  "features": [
    "SWT小波变换Quality分析",
    "LightGBM参数预测",
    "多Tool支持(JXL/AVIF/WebP)",
    "智能优化Mode"
  ]
}
```

---

## 🚀 启动GO服务

### 如果服务未运行

```bash
# 进入GO目录
cd core/go

# 方法1：使用编译版本（推荐）
./pixly-ai --port 50052

# 方法2：使用go run
go run cmd/pixly-ai/main.go --port 50052
```

### 后台运行

```bash
# macOS/Linux
nohup ./pixly-ai --port 50052 > /tmp/pixly-ai.log 2>&1 &

# 查看日志
tail -f /tmp/pixly-ai.log
```

---

## 📊 完整工作流程

### 理想流程

```mermaid
graph LR
    A[启动GO服务] --> B[打开Eagle]
    B --> C[插件检测服务]
    C --> D[✅ AI智能模式]
```

### 实际可能流程

```mermaid
graph LR
    A[打开Eagle] --> B[插件检测服务]
    B --> C[❌ 服务未运行]
    C --> D[缓存\"离线\"状态]
    D --> E[后台启动GO服务]
    E --> F[UI仍显示离线]
    F --> G[点击🔧修复]
    G --> H[✅ AI智能模式]
```

---

## 💡 常见问题

### Q1: 为什么不自动刷新？
**A:** 性能优化
- 5分钟缓存避免频繁检测
- 减少后台轮询资源消耗
- 用户控制刷新时机

### Q2: 点击修复按钮无反应？
**A:** 检查：
1. GO服务是否真的在运行（`lsof -i :50052`）
2. 查看Eagle插件控制台（开发者工具）
3. 查看GO服务日志（`/tmp/pixly-ai.log`）

### Q3: 修复后仍显示离线？
**A:** 可能原因：
1. GO服务崩溃（检查进程）
2. 端口被占用（检查50052端口）
3. 防火墙阻止（macOS检查安全设置）

---

## 🎯 最佳实践

### 推荐工作流程

1. **启动顺序**
   ```bash
   # 1. 先启动GO服务
   cd core/go && ./pixly-ai --port 50052 &
   
   # 2. 再打开Eagle
   open -a Eagle
   ```

2. **服务管理**
   - 使用系统服务管理器（launchd/systemd）
   - 设置开机自启动
   - 监控服务健康状态

3. **问题排查**
   - 先检查服务状态
   - 再尝试UI修复
   - 最后考虑重启

---

## 🔧 技术细节

### 缓存机制

```javascript
// 插件缓存逻辑
let _goCoreCache = {
    result: { available: boolean, version: string },
    timestamp: number  // 5分钟有效期
};

// fixAICore清除缓存
_goCoreCache = null;
_goCoreDetecting = null;
```

### 检测流程

1. **TCP连接测试**
   - 尝试连接localhost:50052
   - 超时1秒

2. **HTTP版本检测**
   - GET /api/v1/version
   - 超时3秒

3. **UI状态更新**
   - 成功 → ✅ AI智能模式
   - 失败 → ❌ AI离线

---

## 📞 获取帮助

如果以上方法都无法解决：

1. **查看日志**
   ```bash
   # GO服务日志
   tail -100 /tmp/pixly-ai.log
   
   # Eagle控制台
   # Eagle → 右键插件 → 检查元素 → Console
   ```

2. **收集信息**
   - GO服务状态
   - Eagle插件版本
   - 错误日志
   - 系统环境

3. **提交Issue**
   - 附带完整日志
   - 描述复现步骤
   - 系统信息

---

**🎉 99%的情况下，点击🔧修复按钮即可解决！**
