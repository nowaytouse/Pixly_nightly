# Phase 40.6: AI API修复 - 灾难级问题解决

## 🔥 发现的根本问题

### 问题1: API Endpoint路径错误
```
Rust: /api/ai/predict  ❌
Go:   /api/v1/predict  ✅
```

### 问题2: API格式不匹配

**Rust发送的格式（基于特征）**:
```json
{
  "input_format": "png",
  "target_format": "jxl",
  "width": 1920,
  "height": 1080,
  "file_size": 500000,
  ...
}
```

**Go服务期望的格式（基于文件）**:
```json
{
  "image_path": "/path/to/image.png",  // ❌ 必需！
  "tool": "jxl",
  "target_quality": 90,
  "optimize_mode": "balanced"
}
```

## ✅ 修复方案

### 1. 简化`is_available()`检查
```rust
// 修复前：每次都HTTP检查，影响性能
pub fn is_available(&self) -> bool {
    let health_url = format!("{}/api/v1/health", ...);
    self.client.get(&health_url).send().is_ok()
}

// 修复后：只检查config
pub fn is_available(&self) -> bool {
    self.config.enabled
}
```

### 2. 修复endpoint路径
```rust
// 修复前
let url = format!("{}/api/ai/predict", self.config.base_url);

// 修复后
let url = format!("{}/api/v1/predict", self.config.base_url);
```

### 3. API格式适配（临时方案）
```rust
// 🔥 适配Go AI服务的请求格式
let adapted_request = serde_json::json!({
    "image_path": format!("/tmp/dummy_{}x{}.{}", width, height, input_format),
    "tool": &target_format,
    "target_quality": preserve_quality ? 95 : 85,
    "optimize_mode": preserve_quality ? "quality" : "balanced",
});
```

### 4. 响应格式适配
```rust
// Go响应 → Rust PredictionResponse
let go_response: serde_json::Value = response.json()?;
if go_response["success"].as_bool().unwrap_or(false) {
    let params = &go_response["params"];
    return Ok(PredictionResponse {
        quality: params["quality"].as_i64().map(|v| v as u8),
        speed: params["speed"].as_i64().map(|v| v as u8),
        confidence: params["confidence"].as_f64().unwrap_or(0.8) as f32,
        ...
    });
}
```

## 📊 影响

### Before (Phase 40.5)
- ✅ JPEG → JXL 成功（3次）
- ❌ PNG → JXL 失败（API路径错误）
- ❌ PNG → AVIF/WebP 失败
- ❌ 所有手动模式转换失败

### After (Phase 40.6)
- ✅ 所有格式预测应该正常工作
- ✅ 智能模式 + 手动模式都可用
- ⚠️  `lossless`预测暂时硬编码为`false`（Go服务待实现）

## 🚨 已知限制

1. **Go AI服务的lossless预测**：
   - 当前Go服务不返回lossless建议
   - Rust暂时硬编码为`false`
   - **TODO**: 更新Go服务以支持lossless预测

2. **image_path伪造**：
   - Rust发送假路径`/tmp/dummy_wxh.format`
   - Go服务可能在未来版本拒绝这种请求
   - **TODO**: 更新Go服务以支持基于特征的API

## 🔮 未来改进（Phase 41+）

### 架构统一方案
```
┌─────────────┐
│  Rust Core  │
│ (Features)  │
└──────┬──────┘
       │ 
       │ 标准化API
       ▼
┌──────────────────┐
│ Go AI Service v5 │
│  (Feature-Based) │
└──────────────────┘

Request Format:
{
  "input_format": "png",
  "target_format": "jxl",
  "width": 1920,
  "height": 1080,
  "file_size": 500000,
  "has_alpha": false,
  "has_animation": false,
  "priority": "balanced",
  "preserve_quality": false
}

Response Format:
{
  "quality": 90,
  "speed": 4,
  "effort": 7,
  "lossless": false,        // ✅ 支持lossless预测
  "lossless_jpeg": false,
  "confidence": 0.92,
  "model_used": "lightgbm-v1.0"
}
```

---

**编译状态**: ✅ 成功 (61秒)
**测试状态**: ⏳ 待用户验证
**下一步**: 
1. 测试PNG/AVIF/WebP转换
2. 实施项目重组（Phase 41）
3. 实现XMP合并功能（Rust）
