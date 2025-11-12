# 🚀 PIXLY Quick Start Guide

## 📖 一分钟快速开始

```bash
# 1️⃣ 检查环境（查看缺少什么工具）
./check-environment.sh

# 2️⃣ 安装依赖（一键安装所有工具）
./install-dependencies.sh

# 3️⃣ 启动AI服务
./start-ai-service.sh

# 4️⃣ 打开Eagle，使用PIXLY插件 ✨
```

---

## 🎯 三个核心脚本

### 1. 🔍 **环境检查**

```bash
./check-environment.sh
```

**作用：** 检查17个必需工具是否已安装

**检查项目：**
- ✅ GO (AI服务)
- ✅ Rust (CLI工具)  
- ✅ Node.js (插件开发)
- ✅ cjxl/avifenc/cwebp (图像编码器)
- ✅ ffmpeg (视频处理)
- ✅ exiftool (元数据)
- ✅ 更多...

**输出示例：**
```
╔══════════════════════════════════════════════════════╗
║  🔍 PIXLY Environment Check & Dependency Installer  ║
╚══════════════════════════════════════════════════════╝

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📦 Core Development Tools
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ GO (AI Service)
   → go version go1.21.0
✅ Rust (CLI Tools)
   → cargo 1.74.0
❌ Node.js (Plugin Dev)
   Install: brew install node

Total Checks:  17
✅ Passed:     16
❌ Failed:     1
```

---

### 2. 🔧 **自动安装**

```bash
./install-dependencies.sh
```

**作用：** 自动安装所有缺失的工具

**安装内容：**
1. Homebrew (如未安装)
2. GO + Rust + Node.js
3. 图像工具：jpeg-xl, libavif, webp, exiftool, libheif
4. 视频工具：ffmpeg
5. 构建 PIXLY Rust CLI

**过程：**
```
🔧 PIXLY Automatic Dependency Installer
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Homebrew found
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📦 Installing Core Development Tools
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Installing GO...
✅ GO installed
...
🎉 Installation Complete!
```

**时间：** 约5-10分钟（取决于网络和硬件）

---

### 3. 🤖 **AI服务**

```bash
# 启动
./start-ai-service.sh

# 停止
./stop-ai-service.sh
```

**作用：** 管理GO AI服务（视频智能处理必需）

**启动输出：**
```
🚀 PIXLY AI Service Manager
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔄 Starting AI service...
⏳ Waiting for service to start...
✅ AI service started successfully!
📊 PID: 12345
🌐 URL: http://localhost:50052
📝 Log: /path/to/pixly-ai.log
```

**验证：**
```bash
curl http://localhost:50052/api/v1/health
# {"status":"ok","version":"1.0.0","port":"50052"}
```

---

## 🐛 常见问题

### ❓ "check-environment.sh: command not found"

**解决：**
```bash
chmod +x *.sh
```

---

### ❓ "视频AI面板显示空白"

**原因：** CSS颜色问题（已在Phase 46.5.4修复）

**解决：**
1. 更新代码到最新版本
2. 刷新Eagle插件
3. 应该看到文字正常显示

---

### ❓ "AI service not available"

**原因：** AI服务未启动

**解决：**
```bash
./start-ai-service.sh
```

然后刷新Eagle插件

---

### ❓ "Rust CLI not found"

**解决：**
```bash
cd core/rust
cargo build --release
```

或运行：
```bash
./install-dependencies.sh
```

---

## 📊 完整工具链

```
PIXLY完整环境
├── check-environment.sh    (检查)
├── install-dependencies.sh (安装)
├── start-ai-service.sh    (启动AI)
├── stop-ai-service.sh     (停止AI)
├── AI_SERVICE_README.md   (详细文档)
└── QUICK_START.md         (本文件)
```

---

## 🎯 推荐工作流程

### **首次使用：**

```bash
# 1. 检查环境
./check-environment.sh

# 2. 安装缺失工具
./install-dependencies.sh

# 3. 启动AI服务
./start-ai-service.sh

# 4. 使用插件
# 打开Eagle → 打开PIXLY插件 → 选择文件 → 开始转换
```

### **日常使用：**

```bash
# 1. 启动AI服务（如未运行）
./start-ai-service.sh

# 2. 使用插件

# 3. 完成后停止AI服务（可选）
./stop-ai-service.sh
```

---

## 📚 更多资源

- **AI服务详细文档：** [AI_SERVICE_README.md](./AI_SERVICE_README.md)
- **项目架构：** [ARCHITECTURE.md](./ARCHITECTURE.md)
- **贡献指南：** [CONTRIBUTING.md](./CONTRIBUTING.md)

---

## 🆘 需要帮助？

如果遇到问题：

1. **查看日志：**
   ```bash
   tail -100 pixly-ai.log
   ```

2. **重新检查环境：**
   ```bash
   ./check-environment.sh
   ```

3. **查看进程：**
   ```bash
   ps aux | grep pixly
   ```

4. **创建Issue：** [GitHub Issues](https://github.com/nowaytouse/Pixly_nightly/issues)

---

**最后更新：** 2025-11-08  
**版本：** Phase 46.5.4
