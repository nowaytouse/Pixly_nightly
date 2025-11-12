# 🏗️ Pixly架构角色明确定义

> **核心原则**: 明确各层职责，Rust执行，Go AI决策，JS UI展示  
> **更新时间**: 2025-11-11 08:30

---

## 🎯 三层架构角色定位

### 1. Rust核心层 - 必选服务 ✅

**定位**: 唯一的、最全面的转换器执行层和文件处理器

**职责**:
- ✅ **文件处理**: 读取、写入、格式转换、元数据处理
- ✅ **转换执行**: 调用原生编码器（AVIF/WebP/JXL等）
- ✅ **性能优化**: 多线程、内存管理、批量处理
- ✅ **质量保证**: 错误处理、进度报告、结果验证
- ✅ **CLI接口**: 命令行工具，可独立运行
- ✅ **HTTP服务器**: 可选的HTTP API（feature flag）

**不负责**:
- ❌ **AI决策**: 不做智能参数推荐
- ❌ **参数优化**: 不做复杂的参数选择逻辑
- ❌ **工具选择**: 不做"应该用哪个工具"的决策

**参数来源**:
```rust
// Rust接受参数，但不做AI决策
pub struct ConvertConfig {
    pub quality: u8,        // 来自: 用户手动 OR Go AI推荐
    pub speed: u8,          // 来自: 用户手动 OR Go AI推荐
    pub format: String,     // 来自: 用户手动 OR Go AI推荐
    
    // 默认参数策略（非AI）
    // 仅在用户未指定且无AI推荐时使用
    pub use_defaults: bool,
}
```

**独立运行能力**:
```bash
# 场景1: 用户手动指定所有参数
pixly-rust convert input.png output.avif --quality 85 --speed 4

# 场景2: 使用默认参数策略（非AI）
pixly-rust convert input.png output.avif

# 场景3: 批量处理（默认参数）
pixly-rust batch /path/to/images --format avif
```

---

### 2. Go AI层 - 可选服务 🔄

**定位**: 最完全的AI增强核心，提供最全面的AI服务功能

**职责**:
- ✅ **智能参数推荐**: 基于图像特征的AI参数优化
  - Quality推荐（1-100）
  - Speed推荐（0-10）
  - Effort推荐
  - Distance推荐
  - 原生编码器精细参数

- ✅ **工具选择推荐**: 
  - 格式选择（AVIF/WebP/JXL/PNG等）
  - 编码器选择（原生/工具）
  - 依赖工具参数优化

- ✅ **高级AI服务**:
  - 图像特征分析（SWT小波变换）
  - LightGBM/PPO模型推理
  - A/B测试和模型管理
  - 在线学习和反馈收集

- ✅ **完整AI Pipeline**:
  - 预测 → 推荐 → 验证 → 反馈 → 训练

**服务模式**:
```go
// Go AI提供完整的参数推荐
type PredictResponse struct {
    // 基础参数推荐
    Quality    int     `json:"quality"`     // 85
    Speed      int     `json:"speed"`       // 4
    Lossless   bool    `json:"lossless"`    // false
    
    // 高级参数推荐
    Distance   float64 `json:"distance"`    // 0.5
    Effort     int     `json:"effort"`      // 7
    
    // 格式和工具推荐
    RecommendedFormat string `json:"recommended_format"` // "avif"
    FormatReason      string `json:"format_reason"`      // "透明度+高压缩"
    
    // AI置信度
    Confidence float64 `json:"confidence"`  // 0.85
    
    // 特征分析
    Features map[string]interface{} `json:"features"`
}
```

**与Rust的关系**:
```
用户 → Go AI (参数推荐) → Rust (执行转换)
          ↓
    AI推荐参数 + 置信度
          ↓
    Rust验证并执行
```

**可选性**:
```
场景A: 使用AI (推荐)
  用户 → Go AI → Rust → 输出
  
场景B: 不使用AI (手动)
  用户 → Rust → 输出
  (用户手动指定参数)
  
场景C: Go AI不可用
  用户 → Rust (使用默认策略) → 输出
  (Rust响亮报错: "AI服务不可用，使用默认参数")
```

---

### 3. JS UI层 - 可选服务 🔄

**定位**: 可选的图形用户界面（Photoshop插件）

**职责**:
- ✅ **UI展示**: 参数输入、预览、进度显示
- ✅ **参数标记**: 标记参数来源（user/ai/hybrid）
- ✅ **AI集成**: 调用Go AI获取推荐
- ✅ **Rust调用**: 执行转换任务
- ✅ **结果展示**: 显示转换结果和AI置信度

**不负责**:
- ❌ **文件处理**: 不直接操作文件
- ❌ **AI决策**: 不做参数推荐
- ❌ **转换逻辑**: 不做格式转换

**调用链路**:
```javascript
// 完整链路（使用AI）
用户操作UI 
  → JS调用Go AI获取推荐
  → 用户确认/修改参数
  → JS调用Rust CLI执行转换
  → 显示结果

// 简化链路（不使用AI）
用户操作UI
  → 用户手动输入参数
  → JS调用Rust CLI执行转换
  → 显示结果
```

---

## 📊 职责对比矩阵

| 功能 | Rust | Go AI | JS UI |
|------|------|-------|-------|
| **文件读写** | ✅ 负责 | ❌ 不涉及 | ❌ 不涉及 |
| **格式转换** | ✅ 负责 | ❌ 不涉及 | ❌ 不涉及 |
| **原生编码器调用** | ✅ 负责 | ❌ 不涉及 | ❌ 不涉及 |
| **AI参数推荐** | ❌ 不负责 | ✅ 负责 | ❌ 不涉及 |
| **工具选择推荐** | ❌ 不负责 | ✅ 负责 | ❌ 不涉及 |
| **图像特征分析(AI)** | ❌ 不负责 | ✅ 负责 | ❌ 不涉及 |
| **用户界面** | ❌ 不涉及 | ❌ 不涉及 | ✅ 负责 |
| **参数来源标记** | ✅ 接收 | ❌ 不涉及 | ✅ 负责 |
| **CLI运行** | ✅ 可独立 | ✅ 可独立 | ❌ 需要宿主 |
| **是否必选** | ✅ 必选 | 🔄 可选 | 🔄 可选 |

---

## 🔄 数据流向

### 完整AI增强流程
```
1. 用户操作 → JS UI
2. JS UI → Go AI: "请推荐参数"
3. Go AI分析图像 → 特征提取 → AI推理
4. Go AI → JS UI: {quality:85, speed:4, format:"avif", confidence:0.85}
5. JS UI显示推荐，用户确认/修改
6. JS UI → Rust CLI: convert input.png output.avif --quality 85 --params-source ai
7. Rust执行转换 → 文件处理 → 编码器调用
8. Rust → JS UI: {success:true, actual_params:{quality:85, params_source:"ai"}}
9. JS UI显示结果
```

### 手动处理流程（无AI）
```
1. 用户操作 → JS UI (或直接CLI)
2. 用户手动输入参数
3. JS UI → Rust CLI: convert input.png output.avif --quality 85 --params-source user
4. Rust执行转换
5. Rust → JS UI: {success:true, actual_params:{quality:85, params_source:"user"}}
6. JS UI显示结果
```

### 纯CLI流程（无AI无UI）
```
1. 用户 → Rust CLI: pixly-rust convert input.png output.avif --quality 85
2. Rust执行转换（使用用户指定参数）
3. Rust → 终端: 转换成功
```

---

## 🎯 关键设计原则

### 1. Rust是唯一的执行层 ✅

**为什么**:
- 性能: Rust提供最佳性能
- 安全: 内存安全，无数据竞争
- 完整: 支持所有格式和编码器
- 可靠: 响亮报错，不静默失败

**如何保证**:
- Go AI不直接操作文件
- JS UI不直接操作文件
- 所有文件处理都通过Rust

---

### 2. Go AI是完整的AI服务 ✅

**为什么**:
- 专业: 专注AI决策，不混杂执行逻辑
- 完整: 提供最全面的AI功能
- 可选: 用户可以不用AI
- 独立: AI崩溃不影响Rust执行

**AI能力范围**:
```
基础推荐:
  - Quality (1-100)
  - Speed (0-10)
  - Lossless (true/false)

高级推荐:
  - Distance
  - Effort
  - 原生编码器精细参数

智能推荐:
  - 格式选择（AVIF vs WebP vs JXL）
  - 工具选择（原生 vs cjxl vs magick）
  - 参数组合优化

特征分析:
  - SWT小波变换
  - 图像复杂度分析
  - 透明度检测
  - 动画检测
```

---

### 3. JS UI只做展示 ✅

**为什么**:
- 职责单一: 只负责UI交互
- 不做决策: 不替代Go AI
- 不做执行: 不替代Rust
- 可选性: 用户可以不用UI

---

## 🔒 边界清晰性

### Rust边界
```rust
// ✅ Rust应该做的
pub fn convert_image(config: ConvertConfig) -> Result<()> {
    // 1. 验证参数（基础验证，非AI）
    validate_params(&config)?;
    
    // 2. 读取文件
    let image = read_image(&config.input)?;
    
    // 3. 调用编码器
    let encoded = encode_image(image, &config)?;
    
    // 4. 写入文件
    write_image(&config.output, encoded)?;
    
    // 5. 报告结果
    Ok(())
}

// ❌ Rust不应该做的
pub fn recommend_params(image_path: &str) -> RecommendedParams {
    // ❌ 不做AI推荐
    // ❌ 不做复杂参数优化
    // ❌ 不做工具选择建议
}
```

### Go AI边界
```go
// ✅ Go AI应该做的
func PredictParams(req PredictRequest) (*PredictResponse, error) {
    // 1. 图像特征提取
    features := extractFeatures(req.ImagePath)
    
    // 2. AI模型推理
    params := model.Predict(features)
    
    // 3. 返回完整推荐
    return &PredictResponse{
        Quality:    params.Quality,
        Speed:      params.Speed,
        Format:     params.RecommendedFormat,
        Confidence: params.Confidence,
        Features:   features,
    }, nil
}

// ❌ Go AI不应该做的
func ConvertImage(input, output string, params Params) error {
    // ❌ 不直接操作文件
    // ❌ 不调用编码器
    // ❌ 不替代Rust
}
```

### JS UI边界
```javascript
// ✅ JS UI应该做的
async function handleConvert() {
    // 1. 获取AI推荐（可选）
    const aiParams = await callGoAI(imagePath);
    
    // 2. 显示推荐，等待用户确认
    const userParams = await showParamsDialog(aiParams);
    
    // 3. 标记参数来源
    userParams.paramsSource = aiParams ? "ai" : "user";
    
    // 4. 调用Rust执行
    const result = await callRustCLI(userParams);
    
    // 5. 显示结果
    showResult(result);
}

// ❌ JS UI不应该做的
function convertImageDirectly(input, output) {
    // ❌ 不直接读写文件
    // ❌ 不做参数推荐
    // ❌ 不调用编码器
}
```

---

## 📝 总结

### ✅ 正确的架构理解

1. **Rust = 唯一执行层（必选）**
   - 文件处理 + 格式转换 + 编码器调用
   - 可独立CLI运行（使用默认策略或用户参数）
   - 不做AI决策，只执行

2. **Go AI = 完整AI服务（可选）**
   - 最全面的AI增强功能
   - 智能参数推荐 + 工具选择
   - 特征分析 + 模型推理
   - 可选但强大

3. **JS UI = 展示层（可选）**
   - 用户交互界面
   - 参数来源标记
   - 结果展示
   - 仅UI职责

---

### 🎯 关键要点

| 层级 | 定位 | 必选性 | 职责 |
|------|------|--------|------|
| **Rust** | 唯一执行层 | ✅ 必选 | 文件处理 + 转换执行 |
| **Go AI** | 完整AI服务 | 🔄 可选 | AI决策 + 参数推荐 |
| **JS UI** | 展示界面 | 🔄 可选 | UI交互 + 参数标记 |

---

**架构原则**: 职责清晰 + 边界明确 + 可选性强  
**核心价值**: Rust独立可用 + Go AI增强能力 + JS UI提升体验  
**设计目标**: 最小依赖 + 最大灵活性 + 最佳性能
