# 🤖 PIXLY Go AI 服务

[![版本](https://img.shields.io/badge/%E7%89%88%E6%9C%AC-1.0.0-blue.svg)](https://golang.org/)
[![许可证](https://img.shields.io/badge/%E8%AE%B8%E5%8F%AF%E8%AF%81-MIT-green.svg)](../../LICENSE)

基于AI的智能决策服务，处理质量预测、格式推荐和参数优化。

---

## 📋 概述

Go服务是PIXLY的**AI决策核心**。它负责：
- 运行LightGBM机器学习模型进行质量预测
- 提供PPO强化学习优化
- 基于图像分析推荐最优格式
- 使用SSIM/VMAF验证转换质量
- 为插件提供RESTful HTTP API

**架构**: Go + LightGBM + PPO + SSIM/VMAF 验证

---

## ✨ 功能特性

### AI 能力
- 🧠 **LightGBM 质量预测** - 基于机器学习的质量参数选择
- 🎯 **PPO 强化学习** - 持续优化决策准确度
- 📊 **SSIM 验证** - 结构相似度验证
- 🎬 **VMAF 评估** - 视频质量评估
- 🔧 **格式推荐** - 智能格式选择

### 处理模式
- **智能模式**: AI驱动的预测与验证
- **平衡模式**: ML + SSIM验证（推荐）
- **质量模式**: 深度AI + PPO + 多指标验证

---

## 🚀 快速开始

### 前置要求
```bash
# 安装 Go 1.21+
brew install go

# 安装依赖
go mod download
```

### 运行服务
```bash
# 开发模式
go run main.go

# 生产构建
go build -o pixly-ai-service
./pixly-ai-service
```

### 默认配置
- **端口**: `3001`
- **API地址**: `http://localhost:3001`
- **健康检查**: `GET /health`

---

## 📡 API 端点

### 健康检查
```bash
GET /health
```

### 质量预测
```bash
POST /predict-quality
Content-Type: application/json

{
  "imagePath": "/path/to/image.jpg",
  "targetFormat": "jxl",
  "optimizeMode": "balanced"
}
```

**响应**:
```json
{
  "quality": 85,
  "effort": 7,
  "confidence": 0.92,
  "recommendation": {
    "format": "jxl",
    "reason": "此图像类型的最佳压缩方案"
  }
}
```

### 格式推荐
```bash
POST /recommend-format
Content-Type: application/json

{
  "imagePath": "/path/to/image.jpg",
  "constraints": {
    "maxSize": 1048576,
    "preserveTransparency": true
  }
}
```

### SSIM 验证
```bash
POST /validate-ssim
Content-Type: application/json

{
  "originalPath": "/path/to/original.jpg",
  "convertedPath": "/path/to/converted.jxl",
  "threshold": 0.95
}
```

---

## 🏗️ 架构设计

### 组件

```
┌─────────────────────────────────────────┐
│      Go AI 服务 (端口 3001)              │
├─────────────────────────────────────────┤
│  ┌──────────────────────────────────┐   │
│  │   LightGBM 模型                  │   │
│  │   - 质量预测                     │   │
│  │   - 特征提取                     │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   PPO 优化器                     │   │
│  │   - 强化学习                     │   │
│  │   - 决策优化                     │   │
│  └──────────────────────────────────┘   │
│  ┌──────────────────────────────────┐   │
│  │   SSIM/VMAF 验证器               │   │
│  │   - 质量验证                     │   │
│  │   - 指标计算                     │   │
│  └──────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

### 数据流
1. **插件** 发送预测请求
2. **Go服务** 分析图像特征
3. **LightGBM** 预测最优质量
4. **PPO** 优化决策
5. **验证器** 验证结果质量
6. 返回响应给插件

---

## 📁 项目结构

```
core/go/
├── main.go                 # 服务入口
├── handlers/               # HTTP 请求处理器
│   ├── predict.go         # 质量预测
│   ├── recommend.go       # 格式推荐
│   └── validate.go        # 质量验证
├── ml/                     # 机器学习
│   ├── lightgbm.go        # LightGBM 集成
│   ├── ppo.go             # PPO 优化器
│   └── features.go        # 特征提取
├── models/                 # 训练好的ML模型
│   ├── quality_predictor.txt
│   └── ppo_weights.bin
├── validators/             # 质量验证
│   ├── ssim.go            # SSIM 计算器
│   └── vmaf.go            # VMAF 评估器
└── utils/                  # 工具函数
    ├── logger.go
    └── config.go
```

---

## 🔧 配置

### 环境变量
```bash
# 服务配置
AI_SERVICE_PORT=3001
AI_LOG_LEVEL=info

# 模型路径
LIGHTGBM_MODEL_PATH=./models/quality_predictor.txt
PPO_WEIGHTS_PATH=./models/ppo_weights.bin

# 验证阈值
SSIM_THRESHOLD=0.95
VMAF_THRESHOLD=85
```

### 配置文件 (`config.yaml`)
```yaml
service:
  port: 3001
  timeout: 30s

models:
  lightgbm: "./models/quality_predictor.txt"
  ppo: "./models/ppo_weights.bin"

validation:
  ssim_threshold: 0.95
  vmaf_threshold: 85
  enable_auto_retry: true
```

---

## 🧪 测试

### 运行测试
```bash
# 单元测试
go test ./...

# 带覆盖率
go test -cover ./...

# 指定包
go test ./ml/
```

### 集成测试
```bash
# 启动服务
go run main.go &

# 运行集成测试
go test -tags=integration ./tests/

# 停止服务
pkill pixly-ai-service
```

---

## 📊 机器学习模型

### LightGBM 质量预测器
- **输入特征**: 图像尺寸、复杂度、格式、色彩空间
- **输出**: 质量值(0-100)、努力程度(1-9)
- **准确率**: 验证集上约92%

### PPO 优化器
- **训练**: 基于转换结果的强化学习
- **奖励**: 文件大小减少 + SSIM维持
- **更新**: 持续从用户转换中学习

---

## 🐛 调试

### 启用调试日志
```bash
export AI_LOG_LEVEL=debug
go run main.go
```

### 测试端点
```bash
# 健康检查
curl http://localhost:3001/health

# 预测质量
curl -X POST http://localhost:3001/predict-quality \
  -H "Content-Type: application/json" \
  -d '{"imagePath": "test.jpg", "targetFormat": "jxl"}'
```

---

## 📝 开发指南

### 添加新功能
1. 在 `handlers/` 中创建处理器
2. 实现业务逻辑
3. 添加测试
4. 更新API文档
5. 在 `main.go` 中注册路由

### 模型更新
1. 训练新模型
2. 导出为兼容格式
3. 放置在 `models/` 目录
4. 更新配置路径
5. 重启服务

---

## 🤝 集成

### 与Rust服务
```go
// 调用Rust服务进行转换
resp, err := http.Post("http://localhost:3000/convert", ...)
```

### 与插件(JS)
```javascript
// 从插件调用Go服务
const response = await fetch('http://localhost:3001/predict-quality', {
  method: 'POST',
  body: JSON.stringify({
    imagePath: '/path/to/image.jpg',
    targetFormat: 'jxl'
  })
});
```

---

## 📚 资源

- [LightGBM 文档](https://lightgbm.readthedocs.io/)
- [Go 文档](https://golang.org/doc/)
- [PPO 算法](https://arxiv.org/abs/1707.06347)
- [SSIM 指数](https://en.wikipedia.org/wiki/Structural_similarity)

---

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](../../LICENSE)

---

## 🔗 相关服务

- **[Rust 转换服务](../rust/)** - 处理实际转换
- **[插件 (JS)](../plugin/)** - Eagle中的用户界面

---

**版本**: 1.0.0  
**最后更新**: 2025-11-09  
**维护者**: PIXLY 团队
