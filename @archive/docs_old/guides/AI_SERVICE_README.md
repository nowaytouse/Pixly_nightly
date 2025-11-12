# 🤖 PIXLY AI Service 管理指南

## 📖 概述

PIXLY 使用 **GO AI Service** 提供智能视频处理功能，包括：
- 🧠 视频质量参数AI预测
- 🎬 场景检测和智能分析
- 📊 VMAF质量验证
- ⚡ Transformer精细处理

---

## 🚀 快速启动

### 方法1：使用启动脚本（推荐）

```bash
# 启动AI服务
./start-ai-service.sh

# 停止AI服务
./stop-ai-service.sh
```

### 方法2：手动启动

```bash
# 后台启动
cd core/go
go run cmd/pixly-ai/main.go &

# 前台启动（用于调试）
cd core/go
go run cmd/pixly-ai/main.go
```

---

## 📊 服务状态检查

### 检查服务是否运行

```bash
# 方法1：使用curl
curl http://localhost:50052/api/v1/health

# 预期输出：
# {"status":"ok","version":"1.0.0","port":"50052"}

# 方法2：检查进程
ps aux | grep pixly-ai
```

### 查看服务日志

```bash
# 查看实时日志
tail -f pixly-ai.log

# 查看最近100行
tail -100 pixly-ai.log
```

---

## ⚠️ 常见问题

### 1. **服务启动失败 - 端口占用**

**错误：** `bind: address already in use`

**解决：**
```bash
# 查找占用50052端口的进程
lsof -i :50052

# 杀死进程
kill -9 <PID>

# 重新启动
./start-ai-service.sh
```

### 2. **插件显示"AI异常无法启动"**

**原因：**
- AI服务未启动
- 端口50052被占用
- GO环境未配置

**解决步骤：**
1. 启动AI服务
   ```bash
   ./start-ai-service.sh
   ```

2. 刷新Eagle插件
   ```
   在Eagle中关闭并重新打开PIXLY插件
   ```

3. 检查控制台
   - 应该看到：`✅ AI service is online (Go service port 50052)`
   - 不应该看到：`❌ AI service not available`

### 3. **视频AI选项面板显示异常**

**症状：**
- "🧠 视频AI智能选项" 区域显示不正确
- 显示 "🤖 检测中..." 或 "❌ AI离线"

**修复：**
1. 确保AI服务运行
   ```bash
   curl http://localhost:50052/api/v1/health
   ```

2. 刷新插件
   - 关闭Eagle插件
   - 重新打开
   - 等待3-5秒让AI服务检测完成

3. 检查浏览器控制台
   - 不应该有 `ERR_CONNECTION_REFUSED` 错误

---

## 🔧 高级用法

### 自动启动（开机自启）

#### macOS (LaunchAgent)

1. 创建plist文件：
   ```bash
   nano ~/Library/LaunchAgents/com.pixly.ai.plist
   ```

2. 添加内容：
   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
             "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
   <plist version="1.0">
   <dict>
       <key>Label</key>
       <string>com.pixly.ai</string>
       <key>ProgramArguments</key>
       <array>
           <string>/PATH/TO/start-ai-service.sh</string>
       </array>
       <key>RunAtLoad</key>
       <true/>
       <key>KeepAlive</key>
       <true/>
   </dict>
   </plist>
   ```

3. 加载服务：
   ```bash
   launchctl load ~/Library/LaunchAgents/com.pixly.ai.plist
   ```

---

## 📚 API文档

### Health Check

```bash
GET http://localhost:50052/api/v1/health
```

**Response:**
```json
{
  "status": "ok",
  "version": "1.0.0",
  "port": "50052"
}
```

### Video Prediction

```bash
POST http://localhost:50052/api/v1/predict/video
Content-Type: application/json

{
  "file_path": "/path/to/video.mp4",
  "preset": "balanced"
}
```

---

## 🐛 调试

### 启用详细日志

```bash
# 设置环境变量
export PIXLY_AI_DEBUG=true

# 启动服务
cd core/go
go run cmd/pixly-ai/main.go
```

### 测试AI预测

```bash
# 使用curl测试
curl -X POST http://localhost:50052/api/v1/predict/video \
  -H "Content-Type: application/json" \
  -d '{
    "file_path": "/path/to/test.mp4",
    "preset": "balanced"
  }'
```

---

## 📝 服务状态说明

| 状态 | 说明 | 插件显示 |
|------|------|---------|
| **运行中** | 服务正常，端口50052可访问 | ✅ AI service is online |
| **离线** | 服务未启动或崩溃 | ❌ AI离线 |
| **检测中** | 插件正在连接服务 | 🤖 检测中... |

---

## 🎯 最佳实践

1. **开发时**
   - 使用前台启动查看实时日志
   - 保持一个终端专门运行AI服务

2. **日常使用**
   - 使用 `start-ai-service.sh` 后台启动
   - 定期检查日志文件大小
   - 不需要时及时停止服务

3. **调试时**
   - 启用 `PIXLY_AI_DEBUG=true`
   - 查看详细的请求/响应日志
   - 使用curl测试API端点

---

## 🆘 获取帮助

如果遇到无法解决的问题：

1. 收集信息：
   ```bash
   # 服务状态
   curl http://localhost:50052/api/v1/health
   
   # 进程信息
   ps aux | grep pixly-ai
   
   # 最近日志
   tail -100 pixly-ai.log
   ```

2. 检查Issues或创建新Issue
3. 提供上述信息以便快速定位问题

---

**最后更新：** 2025-11-08  
**版本：** 1.0.0
