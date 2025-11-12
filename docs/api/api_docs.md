# Pixly AI Service API

AI驱动的图像和视频参数预测服务

## 服务器信息

- **Development Server**: `http://localhost:8080`
- **Production Server**: `https://api.pixly.ai`

## API端点

### Prediction

#### POST /api/v1/predict

基于图像特征预测最佳压缩参数

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `image_path` | string | 是 | 图像文件路径 | /path/to/image.jpg |
| `tool` | string | 否 | 目标压缩工具 | jxl |
| `target_quality` | integer | 否 | 目标质量 | 85 |
| `optimize_mode` | string | 否 | 优化模式 | balanced |

**响应:**

- **200**: 预测成功
  ```json
{
  "success": true,
  "params": {
    "quality": 85,
    "effort": 7,
    "distance": 1.0
  },
  "confidence": 0.85,
  "model_used": "lightgbm",
  "inference_time_ms": 120.5
}
  ```
- **400**: 请求参数错误
- **500**: 服务器内部错误

---

#### POST /api/v1/predict/video

基于视频特征预测最佳压缩参数

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `video_path` | string | 是 | 视频文件路径 | /path/to/video.mp4 |
| `target_codec` | string | 否 | 目标编解码器 | h265 |
| `target_crf` | integer | 否 | 目标CRF值 | 23 |
| `preset` | string | 否 | 编码预设 | medium |

**响应:**

- **200**: 预测成功
  ```json
{
  "success": true,
  "params": {
    "crf": 23,
    "preset": "medium",
    "codec": "h265"
  },
  "confidence": 0.82,
  "estimated_size_mb": 125.6,
  "estimated_time_seconds": 45.2
}
  ```

---

### Ai

#### POST /api/v1/predict

基于图像特征预测最佳压缩参数

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `image_path` | string | 是 | 图像文件路径 | /path/to/image.jpg |
| `tool` | string | 否 | 目标压缩工具 | jxl |
| `target_quality` | integer | 否 | 目标质量 | 85 |
| `optimize_mode` | string | 否 | 优化模式 | balanced |

**响应:**

- **200**: 预测成功
  ```json
{
  "success": true,
  "params": {
    "quality": 85,
    "effort": 7,
    "distance": 1.0
  },
  "confidence": 0.85,
  "model_used": "lightgbm",
  "inference_time_ms": 120.5
}
  ```
- **400**: 请求参数错误
- **500**: 服务器内部错误

---

### System

#### GET /api/v1/health

获取服务健康状态

**响应:**

- **200**: 服务健康
  ```json
{
  "status": "healthy",
  "version": "3.0.0",
  "ready": true,
  "timestamp": "2025-11-12T17:20:00Z"
}
  ```

---

#### GET /api/v1/capabilities

获取AI服务支持的功能和格式列表

**响应:**

- **200**: 服务能力
  ```json
{
  "prediction": {
    "image": true,
    "video": true,
    "supported_formats": [
      "jpg",
      "png",
      "webp",
      "avif",
      "jxl",
      "heic"
    ]
  },
  "models": {
    "lightgbm": true,
    "ppo": true,
    "ab_testing": true
  },
  "features": {
    "online_learning": true,
    "feedback": true,
    "batch_processing": true
  }
}
  ```

---

### Models

#### GET /api/v1/models

获取所有可用的AI模型版本

**响应:**

- **200**: 模型列表
  ```json
{
  "models": {
    "lightgbm": [
      {
        "version": "v1.0.0",
        "status": "active"
      }
    ],
    "ppo": [
      {
        "version": "v1.0.0",
        "status": "active"
      }
    ]
  }
}
  ```

---

#### POST /api/v1/models/register

注册新的AI模型版本到系统

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `name` | string | 是 | 模型名称 | lightgbm |
| `version` | string | 是 | 模型版本 | v2.0.0 |
| `model_path` | string | 是 | 模型文件路径 | /models/lightgbm_v2.pth |
| `description` | string | 否 | 模型描述 | 改进的LightGBM模型 |

**响应:**

- **201**: 注册成功
  ```json
{
  "success": true,
  "model_id": "lightgbm_v2.0.0",
  "message": "模型注册成功"
}
  ```
- **409**: 模型已存在
- **400**: 注册参数错误

---

### Training

#### POST /api/v1/training/start

启动自动化模型训练队列

**响应:**

- **200**: 训练启动成功
  ```json
{
  "success": true,
  "message": "训练队列已启动"
}
  ```

---

#### GET /api/v1/training/status

获取当前模型训练队列的状态

**响应:**

- **200**: 训练状态
  ```json
{
  "is_running": true,
  "current_epoch": 15,
  "total_epochs": 100,
  "progress": 0.15,
  "eta_minutes": 42.5,
  "current_loss": 0.0234
}
  ```

---

### Video

#### POST /api/v1/predict/video

基于视频特征预测最佳压缩参数

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `video_path` | string | 是 | 视频文件路径 | /path/to/video.mp4 |
| `target_codec` | string | 否 | 目标编解码器 | h265 |
| `target_crf` | integer | 否 | 目标CRF值 | 23 |
| `preset` | string | 否 | 编码预设 | medium |

**响应:**

- **200**: 预测成功
  ```json
{
  "success": true,
  "params": {
    "crf": 23,
    "preset": "medium",
    "codec": "h265"
  },
  "confidence": 0.82,
  "estimated_size_mb": 125.6,
  "estimated_time_seconds": 45.2
}
  ```

---

### Feedback

#### POST /api/v1/feedback/record

记录用户对AI预测结果的反馈

**参数:**

| 名称 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `prediction_id` | string | 是 | 预测ID | pred_123456 |
| `user_rating` | integer | 是 | 用户评分 | 4 |
| `actual_quality` | integer | 否 | 实际质量 | 87 |
| `comments` | string | 否 | 用户评论 | 预测很准确 |

**响应:**

- **200**: 反馈记录成功
  ```json
{
  "success": true,
  "feedback_id": "fb_789012",
  "message": "感谢您的反馈"
}
  ```

---

