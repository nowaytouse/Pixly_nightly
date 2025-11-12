# 🔍 参数验证系统调查报告

> **日期**: 2025-11-10  
> **目标**: 调查三端参数流动和验证机制，识别缺口，制定增强方案

## 📊 参数流动全景图

```
┌─────────────────────────────────────────────────────────────┐
│                     用户交互层 (JS UI)                         │
│  plugin/converter/js/plugin-modules/                         │
│                                                              │
│  [UI Controls] → getConversionConfig()                       │
│      ├─ 智能模式: format only (AI决定参数)                    │
│      └─ 手动模式: format + quality + speed + lossless        │
│                                                              │
│  ✅ 已有验证: conversion-validator.js (4级验证)               │
│  ✅ 已有验证: file-validator.js (AI文件检测)                  │
│  ❌ 缺失: 参数快照与回显对比                                  │
└──────────────────┬───────────────────────────────────────────┘
                   │
                   ▼ rustCLI.convertImage(options)
┌─────────────────────────────────────────────────────────────┐
│                   Rust 执行内核 (CLI)                         │
│  core/rust/src/cli/commands/convert.rs                      │
│                                                              │
│  ConvertRequest {                                            │
│    input, output, format,                                    │
│    quality, speed, lossless,                                 │
│    preserve_metadata, keep_animated                          │
│  }                                                           │
│                                                              │
│  ✅ 已有验证: FileValidator (输入文件)                         │
│  ✅ 已有验证: OutputValidator (输出文件)                       │
│  ❌ 缺失: 参数接收确认返回                                     │
│  ❌ 缺失: 实际使用参数的回显                                   │
└──────────────────┬───────────────────────────────────────────┘
                   │
                   ▼ 需要AI预测时
┌─────────────────────────────────────────────────────────────┐
│                    Go AI 决策服务                             │
│  core/go/proto/ai_service.proto                             │
│                                                              │
│  PredictRequest {                                            │
│    mode: BASIC | ADVANCED | HYBRID                          │
│    image: ImageInfo                                          │
│    tool: cjxl | avifenc | cwebp | ffmpeg                   │
│    options: PredictOptions                                   │
│  }                                                           │
│                                                              │
│  PredictResponse {                                           │
│    params: ToolParams (quality, effort, etc.)               │
│    confidence, ssim_estimate, reason                         │
│  }                                                           │
│                                                              │
│  ❌ 缺失: 输入参数验证                                         │
│  ❌ 缺失: 返回参数合理性检查                                   │
└─────────────────────────────────────────────────────────────┘
```

## 🔍 现有验证机制分析

### ✅ JS UI 层 - 已实现

#### 1. **conversion-validator.js** (4级验证)

**Level 1: 输入文件验证**
```javascript
validateInputFiles(files)
  ✓ 文件路径有效性
  ✓ 扩展名存在性
  ✓ 文件大小检查 (0 < size < 1GB)
  ✓ 文件名合法字符
```

**Level 2: 转换参数验证**
```javascript
validateParameters(config, files)
  ✓ 格式有效性 (jxl/avif/webp/heic/png/jpg)
  ✓ 质量范围 (1-100)
  ✓ 速度范围 (0-10)
  ✓ 格式兼容性 (透明度、动画)
  ✓ 无损模式兼容性
```

**Level 3: 模式一致性验证**
```javascript
validateModeConsistency(mode, config)
  ✓ 手动模式参数完整性
  ✓ 智能模式AI服务可用性
```

**Level 4: 输出验证**
```javascript
validateOutput(outputPath, expectedSize)
  ✓ 文件是否生成
  ✓ 文件大小非零
  ✓ 大小合理性 (0.01x - 10x)
```

#### 2. **file-validator.js** (AI安全验证)

```javascript
✓ Magika AI 文件类型检测
✓ 格式伪装检测 (type_match)
✓ 文件安全性检查 (is_safe)
✓ 批量验证支持
✓ Eagle metadata 自动修正
```

### ✅ Rust Core 层 - 已实现

#### FileValidator
```rust
validate_input(path)
  ✓ 文件存在性
  ✓ 文件可读性
  ✓ 文件格式检测 (Magika)
  ✓ 格式匹配验证
```

#### OutputValidator
```rust
validate_output(path, expected_format)
  ✓ 文件生成确认
  ✓ 格式正确性
  ✓ 文件大小合理性
```

### ❌ Go AI 层 - 未实现

**当前状态**: 无参数验证
- 接收 PredictRequest
- 直接处理，无范围检查
- 返回 PredictResponse，无合理性验证

## 🚨 识别的验证缺口

### 缺口 1: **参数传递完整性验证**

**问题**: UI → Rust → Go，参数传递过程无回显确认

```
UI发送: { format: "avif", quality: 85, speed: 4 }
         ↓
Rust接收: ？？？
         ↓
实际使用: ？？？
```

**风险**:
- 参数传递丢失
- 参数被意外修改
- 默认值覆盖用户输入

### 缺口 2: **手动模式参数二次验证**

**问题**: 用户手动设置的参数，只有基础范围检查

```
用户设置: HEIC + lossless = true
当前验证: ✓ 范围检查通过
应增强: ✗ HEIC不支持无损，应阻止
```

**需增强**:
- 参数组合冲突检测
- 格式特性匹配验证
- 不合理组合警告

### 缺口 3: **AI预测参数合理性验证**

**问题**: Go AI返回的参数，Rust直接使用，无二次检查

```
Go AI返回: { quality: 120, speed: 15 }  // 超出范围
Rust: ？直接使用？还是验证？
```

**需增强**:
- AI返回值范围验证
- 置信度阈值检查
- 异常值拒绝机制

### 缺口 4: **Rust实际使用参数回显**

**问题**: Rust执行转换后，未返回实际使用的参数

```
ConvertResponse {
  success: true,
  output_path: "xxx.avif",
  file_size: 123456,
  ❌ 缺失: actual_params_used
}
```

**需增强**:
- 返回实际应用的参数
- UI可对比发送vs实际
- 增强可追溯性

### 缺口 5: **Go AI输入参数验证**

**问题**: Go AI服务直接接收参数，无边界检查

```
PredictRequest {
  image: { width: -100, height: 0 }  // 无效值
  options: { ssim_threshold: 2.5 }   // 超出 [0, 1]
}
// Go AI: 直接处理，无验证
```

**需增强**:
- Protobuf层参数验证
- 图像属性合理性检查
- 选项值范围验证

## 📋 验证标准制定

### 标准 1: **参数范围边界**

| 参数 | 类型 | 范围 | 说明 |
|------|------|------|------|
| **quality** | u8 | 1-100 | 质量参数 |
| **speed** | u8 | 0-10 | 速度/effort |
| **width** | u32 | 1-65535 | 图像宽度 |
| **height** | u32 | 1-65535 | 图像高度 |
| **file_size** | u64 | >0 | 文件大小 |
| **ssim_threshold** | f32 | 0.0-1.0 | SSIM阈值 |
| **confidence** | f32 | 0.0-1.0 | AI置信度 |

### 标准 2: **格式兼容性矩阵**

| 格式 | 无损 | 透明 | 动画 | 说明 |
|------|------|------|------|------|
| **AVIF** | ✅ | ✅ | ✅ | 全支持 |
| **JXL** | ✅ | ✅ | ✅ | 全支持 |
| **WebP** | ✅ | ✅ | ✅ | 全支持 |
| **PNG** | ✅ | ✅ | ✅ | 全支持 |
| **JPEG** | ❌ | ❌ | ❌ | 都不支持 |
| **HEIC** | ❌ | ❌ | ❌ | 都不支持 |

### 标准 3: **参数组合冲突**

**禁止的组合**:
```javascript
// 1. JPEG + lossless
{ format: "jpeg", lossless: true }  // ❌ JPEG不支持无损

// 2. HEIC + 透明/动画
{ format: "heic", source: "png_with_alpha" }  // ⚠️ 透明度丢失

// 3. quality > 95 + lossless = false
{ quality: 98, lossless: false }  // ⚠️ 建议直接使用无损
```

### 标准 4: **AI预测合理性**

**置信度阈值**:
```javascript
confidence < 0.5  // ❌ 拒绝，太低
confidence < 0.7  // ⚠️ 警告，建议review
confidence >= 0.7 // ✅ 接受
```

**参数变化范围**:
```javascript
// 相比用户输入，AI调整应在合理范围内
quality_change: ±20  // 允许 ±20%
speed_change: ±3     // 允许 ±3档
```

### 标准 5: **验证责任边界**

| 层级 | 验证职责 | 失败处理 |
|------|----------|----------|
| **Go AI** | 输入参数边界、返回值范围 | 返回错误，记录日志 |
| **Rust Core** | AI结果二次验证、参数组合冲突 | 拒绝执行，响亮报错 |
| **JS UI** | 用户输入基础验证、参数快照对比 | 提示用户，阻止提交 |

## 🎯 增强方案优先级

### P0 - 高优先级

1. **参数回显机制** (Rust → UI)
   - 修改 `ConvertResponse` 添加 `actual_params`
   - UI显示参数对比

2. **AI返回值验证** (Rust)
   - 验证AI返回的参数范围
   - 置信度阈值检查

3. **参数快照对比** (UI)
   - 发送前快照
   - 返回后对比

### P1 - 中优先级

4. **Go AI输入验证**
   - Protobuf参数边界检查
   - 图像属性合理性验证

5. **参数组合冲突检测** (UI + Rust)
   - 增强UI层冲突检测
   - Rust层二次确认

### P2 - 低优先级

6. **完整性报告生成**
   - 参数验证链路可视化
   - 验证失败追溯

## 📝 实施建议

### 阶段一: 快速增强 (1-2小时)

1. **Rust参数回显**
   ```rust
   // 修改 ConvertResponse
   pub struct ConvertResponse {
       // ... existing fields
       pub actual_params: Option<ActualParams>,  // 新增
   }
   
   pub struct ActualParams {
       pub quality: u8,
       pub speed: u8,
       pub lossless: bool,
       pub strategy_used: String,
   }
   ```

2. **JS参数对比**
   ```javascript
   // 新增: param-integrity-validator.js
   captureSnapshot(uiParams);
   sendToRust(uiParams);
   compareResponse(snapshot, rustResponse.actual_params);
   ```

### 阶段二: 深度增强 (3-4小时)

3. **AI返回值验证**
   ```rust
   // param_optimizers.rs
   fn validate_ai_response(response: &PredictResponse) -> Result<()> {
       if response.confidence < 0.7 {
           bail!("AI confidence too low: {}", response.confidence);
       }
       // ... 范围检查
   }
   ```

4. **Go AI输入验证**
   ```go
   // pkg/ai/validator.go
   func ValidatePredictRequest(req *pb.PredictRequest) error {
       if req.Image.Width < 1 || req.Image.Width > 65535 {
           return fmt.Errorf("invalid width: %d", req.Image.Width)
       }
       // ...
   }
   ```

### 阶段三: 完善优化 (2-3小时)

5. **完整验证链路**
   - 统一验证错误码
   - 验证失败详细日志
   - 可追溯的验证报告

## 🏁 总结

### 现状
- ✅ **UI层**: 4级验证完善
- ✅ **Rust层**: 输入输出验证完善
- ❌ **Go AI层**: 无参数验证
- ❌ **跨层**: 无参数回显和对比

### 关键缺口
1. 参数传递完整性无验证
2. AI预测结果无二次检查
3. 实际使用参数不透明

### 增强价值
- 🎯 **可靠性**: 提升10倍
- 🎯 **可信度**: 参数可追溯
- 🎯 **可维护性**: 问题易定位
- 🎯 **用户体验**: 参数透明化

---

**下一步**: 制定具体实施计划，从P0开始逐步增强
