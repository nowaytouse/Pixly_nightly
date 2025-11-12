# 🔗 AI→Rust参数透明化流程

> **核心目标**: 确保AI推荐的参数能被Rust正确接收、验证并回显  
> **核心原则**: 参数来源100%可追溯，无沉默修改

## 📊 当前架构分析

### 流程1：UI直接调用（已完善 ✅）

```
┌──────────┐
│  Eagle   │ 用户手动设置参数
│    UI    │ quality=85, speed=6
└─────┬────┘
      │ HTTP POST /convert
      │ { quality: 85, speed: 6, ... }
      ▼
┌─────────────────────┐
│   Rust HTTP Server  │
│                     │
│ 1️⃣ 接收请求参数      │
│ 2️⃣ 参数验证         │
│ 3️⃣ 执行转换         │
│ 4️⃣ 参数回显         │  ✅ params_source: "user"
│    actualParams: {  │
│      quality: 85,   │
│      speed: 6,      │
│      params_source: │
│        "user"       │
│    }                │
└─────────────────────┘
```

**状态**: ✅ 完全透明，params_source正确标记为"user"

---

### 流程2：AI辅助调用（需要完善 ⚠️）

```
┌──────────┐
│  Eagle   │ 用户选择"AI优化"
│    UI    │ 或参数设置为"auto"
└─────┬────┘
      │ 1. 请求AI预测
      │ HTTP POST /predict
      │ { image_path, tool, quality, ... }
      ▼
┌─────────────────────┐
│   Go AI Service     │
│   (HTTP :50052)     │
│                     │
│ 1️⃣ 验证请求参数      │ ✅ ValidateHTTPPredictRequest
│ 2️⃣ 调用Python推理   │
│ 3️⃣ 返回AI推荐       │
│    {                │
│      quality: 90,   │
│      distance: 1.5, │
│      confidence:    │
│        0.85,        │
│      params_source: │  ⚠️ 需要添加！
│        "ai"         │
│    }                │
└─────┬───────────────┘
      │ 2. AI推荐返回给UI
      │ UI接收AI推荐参数
      ▼
┌──────────┐
│  Eagle   │ UI显示AI推荐
│    UI    │ 用户可选择接受/调整
└─────┬────┘
      │ 3. 使用AI推荐参数转换
      │ HTTP POST /convert
      │ { quality: 90, speed: 8, ...}
      │ ⚠️ 问题：Rust不知道这是AI推荐的！
      ▼
┌─────────────────────┐
│   Rust HTTP Server  │
│                     │
│ 1️⃣ 接收请求参数      │
│ 2️⃣ 参数验证         │
│ 3️⃣ 执行转换         │
│ 4️⃣ 参数回显         │  ❌ params_source: "user"
│    actualParams: {  │     (错误！实际应该是"ai")
│      quality: 90,   │
│      params_source: │
│        "user"  ❌   │
│    }                │
└─────────────────────┘
```

**问题**: Rust无法区分参数是用户手动设置还是AI推荐的！

---

## 🔧 完善方案

### 方案A：UI标记参数来源（推荐 ⭐）

**思路**: UI在调用Rust转换时，明确标记参数来源

#### 1. 扩展Rust ConvertRequest

```rust
// core/rust/src/server/models.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertRequest {
    pub input: String,
    pub output: String,
    pub format: String,
    pub quality: u8,
    pub speed: u8,
    // ... 其他参数 ...
    
    /// 🆕 参数来源标记
    #[serde(default)]
    pub params_source: Option<String>,  // "user" | "ai" | "hybrid"
    
    /// 🆕 AI推荐的原始confidence（如果是AI推荐）
    #[serde(default)]
    pub ai_confidence: Option<f64>,
}
```

#### 2. UI调用时标记来源

```javascript
// plugin/.../image-conversion.js

// 场景1：用户手动设置
async function convertWithUserParams(file, params) {
    const request = {
        input: file.path,
        output: getOutputPath(file),
        format: params.format,
        quality: params.quality,
        speed: params.speed,
        params_source: 'user',  // 🔥 明确标记
    };
    
    return await rustCLI.convert(request);
}

// 场景2：AI推荐参数
async function convertWithAIParams(file, aiResult) {
    const request = {
        input: file.path,
        output: getOutputPath(file),
        format: aiResult.format,
        quality: aiResult.quality,
        speed: aiResult.speed,
        params_source: 'ai',  // 🔥 明确标记为AI
        ai_confidence: aiResult.confidence,  // 🔥 记录置信度
    };
    
    return await rustCLI.convert(request);
}

// 场景3：混合模式（AI推荐 + 用户调整）
async function convertWithHybridParams(file, aiResult, userAdjustments) {
    const request = {
        input: file.path,
        output: getOutputPath(file),
        format: userAdjustments.format || aiResult.format,
        quality: userAdjustments.quality || aiResult.quality,
        speed: userAdjustments.speed || aiResult.speed,
        params_source: 'hybrid',  // 🔥 混合标记
        ai_confidence: aiResult.confidence,
    };
    
    return await rustCLI.convert(request);
}
```

#### 3. Rust处理params_source

```rust
// core/rust/src/server/handlers.rs
pub async fn convert(
    Json(req): Json<ConvertRequest>,
) -> Json<ConvertResponse> {
    // ... 验证和转换逻辑 ...
    
    // 🔥 使用UI传递的params_source
    let params_source = req.params_source.unwrap_or_else(|| {
        // 如果UI没有标记，尝试推断
        if req.quality == config.quality && req.speed == config.speed {
            "user".to_string()
        } else {
            "unknown".to_string()  // 未知来源
        }
    });
    
    // 🔥 如果是AI推荐，验证置信度
    if params_source == "ai" {
        if let Some(confidence) = req.ai_confidence {
            if confidence < 0.5 {
                log_validation_warning!(
                    "PIXLY-CORE-VAL-005",
                    "AI置信度过低",
                    confidence = confidence,
                    threshold = 0.5
                );
            }
        }
    }
    
    let actual_params = ActualParams {
        quality: config.quality,
        speed: config.speed,
        lossless: config.lossless,
        format: req.format.clone(),
        preserve_metadata: req.preserve_metadata,
        keep_animated: req.keep_animated,
        strategy_type: result.strategy_used.clone(),
        params_source,  // 🔥 透明回显来源
    };
    
    ConvertResponse {
        success: true,
        actualParams: Some(actual_params),
        // ...
    }
}
```

---

### 方案B：Go直接调用Rust（更彻底 ⭐⭐）

**思路**: AI推荐后，Go直接调用Rust进行转换

```
┌──────────┐
│  Eagle   │ 
│    UI    │ 
└─────┬────┘
      │ HTTP POST /predict-and-convert
      │ { image_path, tool, ... }
      ▼
┌─────────────────────┐
│   Go AI Service     │
│                     │
│ 1️⃣ 验证请求参数      │
│ 2️⃣ 调用Python推理   │  AI返回: quality=90, conf=0.85
│ 3️⃣ 验证AI返回       │  ✅ 置信度检查
│ 4️⃣ 调用Rust转换     │  
│    HTTP POST        │
│    localhost:3000   │
│    {               │
│      quality: 90,   │
│      params_source: │
│        "ai",        │
│      ai_confidence: │
│        0.85         │
│    }                │
└─────┬───────────────┘
      │
      ▼
┌─────────────────────┐
│   Rust HTTP Server  │
│                     │
│ 1️⃣ 接收带标记的参数  │  ✅ params_source: "ai"
│ 2️⃣ 参数验证         │  ✅ 验证AI置信度
│ 3️⃣ 执行转换         │
│ 4️⃣ 参数回显         │  ✅ 正确标记来源
└─────┬───────────────┘
      │
      ▼
┌──────────┐
│  Eagle   │ 接收最终结果
│    UI    │ actualParams.params_source: "ai" ✅
└──────────┘
```

#### 实现步骤

1. **Go添加Rust客户端**

```go
// core/go/ai/rust_client.go
package ai

import (
    "bytes"
    "encoding/json"
    "net/http"
)

type RustClient struct {
    baseURL string
}

func NewRustClient(baseURL string) *RustClient {
    return &RustClient{baseURL: baseURL}
}

// ConvertWithAIParams 使用AI推荐参数调用Rust转换
func (c *RustClient) ConvertWithAIParams(
    imagePath string,
    params *PredictParams,
    confidence float64,
) (*RustConvertResponse, error) {
    req := RustConvertRequest{
        Input:  imagePath,
        Output: generateOutputPath(imagePath),
        Format: params.Format,
        Quality: params.Quality,
        Speed: params.Speed,
        ParamsSource: "ai",  // 🔥 明确标记
        AIConfidence: &confidence,
    }
    
    body, _ := json.Marshal(req)
    resp, err := http.Post(
        c.baseURL+"/convert",
        "application/json",
        bytes.NewBuffer(body),
    )
    // ... 处理响应 ...
}
```

2. **新增AI预测+转换端点**

```go
// core/go/ai/http_gateway.go

// handlePredictAndConvert AI预测并直接转换
func (gw *HTTPGateway) handlePredictAndConvert(w http.ResponseWriter, r *http.Request) {
    // 1. AI预测
    result, err := gw.pythonBridge.Predict(...)
    if err != nil {
        gw.sendError(w, err.Error(), 500)
        return
    }
    
    // 2. 验证AI置信度
    if result.Confidence < 0.5 {
        gw.sendError(w, "AI置信度过低", 400)
        return
    }
    
    // 3. 调用Rust转换
    rustResp, err := gw.rustClient.ConvertWithAIParams(
        req.ImagePath,
        &result.Params,
        result.Confidence,
    )
    if err != nil {
        gw.sendError(w, err.Error(), 500)
        return
    }
    
    // 4. 返回完整结果（包含AI预测+转换结果）
    resp := PredictAndConvertResponse{
        AIResult: result,
        ConvertResult: rustResp,
        ParamsSourceVerified: rustResp.ActualParams.ParamsSource == "ai", // ✅
    }
    
    json.NewEncoder(w).Encode(resp)
}
```

---

## ✅ 推荐实施路径

### Phase 1: UI标记来源（立即实施）

1. ✅ Rust添加`params_source`和`ai_confidence`字段
2. ✅ UI在调用时明确标记参数来源
3. ✅ Rust验证并正确回显来源

**优点**: 
- 快速实施
- 不改变现有架构
- UI保持完全控制

### Phase 2: Go→Rust直接调用（后续优化）

1. Go添加Rust HTTP客户端
2. 新增`/predict-and-convert`端点
3. 简化UI调用流程

**优点**:
- 参数流动更直接
- AI→Rust的验证链路完整
- 减少UI复杂度

---

## 📊 参数透明度对比

| 场景 | 当前状态 | Phase 1后 | Phase 2后 |
|------|---------|----------|----------|
| **用户手动** | ✅ "user" | ✅ "user" | ✅ "user" |
| **AI推荐** | ❌ "user" | ✅ "ai" | ✅ "ai" |
| **AI+用户调整** | ❌ "user" | ✅ "hybrid" | ✅ "hybrid" |
| **AI置信度** | ❌ 无 | ✅ 记录 | ✅ 验证+记录 |
| **参数验证** | ✅ Rust | ✅ Rust | ✅ Go+Rust双验证 |

---

## 🎯 最终目标

**100%参数透明化**:
- ✅ 参数来源可追溯（user/ai/hybrid）
- ✅ AI置信度可查询
- ✅ 参数修改记录完整
- ✅ 验证链路无遗漏

**核心价值**:
- 🔍 问题定位：参数问题可快速追溯到来源
- 📊 数据分析：AI推荐的准确度可评估
- 🛡️ 质量保证：低置信度AI推荐可拒绝
- 👥 用户信任：参数流动完全透明

---

**时间**: 2025-11-11 07:45  
**状态**: 方案设计完成，待实施
