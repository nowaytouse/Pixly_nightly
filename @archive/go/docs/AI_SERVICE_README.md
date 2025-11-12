# Pixly AI Service - Go机器学习服务

## 参考文档

### Eagle API
- [Eagle Plugin API](https://developer.eagle.cool/plugin-api)
- Eagle插件通过HTTP调用本服务的ML预测接口

### 参考项目
位于 `@reference/` 文件夹:
- **squoosh-dev/**: Google图像压缩算法参考
- **rimage-main/**: Rust图像优化策略参考
- **xl-converter-unstable/**: JPEG XL编码参考

## 架构原则

### 纯机器学习
- ✅ 使用LightGBM进行参数预测
- ✅ 使用PPO强化学习优化
- ❌ **严禁硬编码规则**
- ❌ **严禁fallback策略**

### HTTP API服务
- 端口: 50052
- 健康检查: `/health`
- 图像预测: `/api/v1/predict`
- 视频预测: `/api/v1/predict/video`
- 观测记录: `/api/v1/observations`

## 功能模块

### 1. 参数预测
- 图像: quality, speed, lossless
- 视频: CRF, preset, tune

### 2. 强化学习
- 收集转换观测数据
- 计算奖励值
- PPO模型训练

### 3. 模型管理
- LightGBM模型加载
- 模型版本控制
- 训练队列管理

## 集成方式

### Rust CLI集成
```rust
// ai_client.rs
let client = AIClient::with_default();
let params = client.predict_image_params(image_path, ...)?;
```

### Eagle插件集成
```javascript
// observation-recorder.js
await fetch('http://localhost:50052/api/v1/observations', {
    method: 'POST',
    body: JSON.stringify(observation)
});
```

## TUI说明书

运行以下命令查看完整TUI说明书：
```bash
cd core/go
go run cmd/pixly-ai/main.go --help
```

详见 `cmd/pixly-ai/tui_manual.go`

