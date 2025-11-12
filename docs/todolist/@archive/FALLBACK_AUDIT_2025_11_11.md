# Fallback代码审计报告

## 审计日期
2025-11-11

## 审计目标

根据PROJECT_QUALITY_MANIFESTO.md的核心原则：
> **禁止Fallback代码（让AI成为摆设）**  
> **响亮报错 > 静默降级**

## 审计范围

- Rust代码：core/rust/src/**/*.rs
- Go代码：core/go/**/*.go
- 已废弃代码：@deprecated/（不处理）

## 审计结果

### ✅ 符合规范的代码（24处）

#### 1. 注释/文档中的说明（18处）
这些是对"不使用fallback"的说明，符合质量原则：

```rust
// ✅ 响亮报错，不fallback
error!("❌ AI service FAILED: {}", e);
```

**文件**:
- `cli/help.rs`: 文档说明"CLI fallback for JXL"（架构说明）
- `cli/conversion.rs:193`: "No fallback available"（正确拒绝）
- `converter/ai_client.rs`: 多处"响亮报错，不fallback"注释
- `converter/batch_processor.rs`: "失败就失败，不要fallback"原则声明

#### 2. 合理的策略选择（4处）
这些不是fallback，而是架构设计的策略选择：

| 位置 | 代码 | 说明 | 判定 |
|------|------|------|------|
| `converter/dimension_validator.rs:63` | `should_use_cli_fallback()` | 根据尺寸选择CLI工具 | ✅ 策略 |
| `ai/model_router.go:153` | `return versions[0]` | 选择最新版本模型 | ✅ 默认值 |
| `ai/model_manager.go:171` | `return ModelBaseline` | 选择baseline模型 | ✅ 默认值 |
| `converter/gif_animation_strategy.rs:33` | gif2webp→FFmpeg | 工具链选择 | ✅ 策略 |

#### 3. 已废弃代码（2处）
在@deprecated目录中，不需要处理。

### ⚠️ 需要修复的代码（3处）

#### 问题1: cli/dependency_checker.rs:222

**代码**:
```rust
// fallback: 直接返回命令名
return Some(PathBuf::from("ffmpeg"));
```

**问题**: 找不到ffmpeg时静默返回命令名，隐藏了问题

**修复**: 应该返回None并让调用者报错

**优先级**: 🟡 中（已有其他检查）

#### 问题2: converter/ai_client.rs:316

**代码**:
```rust
let image_path = if let Some(ref path) = request.image_path {
    path.clone()
} else {
    // fallback: 仅当路径未提供时使用dummy
    format!("/tmp/dummy_{}x{}.{}", request.width, request.height, request.input_format)
};
```

**问题**: 路径未提供时创建假路径，掩盖了必需参数缺失

**修复**: 应该直接返回错误

**优先级**: 🔴 高（核心API）

#### 问题3: converter/ai_client.rs:620

**代码**:
```rust
// Fallback: return empty map
Ok(std::collections::HashMap::new())
```

**问题**: 解析失败时返回空map，掩盖了解析错误

**修复**: 应该返回Err

**优先级**: 🟡 中（有日志warning）

### ❌ 需要移除的配置项（1处）

#### ai/types.go:78

**代码**:
```go
FallbackOnError   bool  `json:"fallback_on_error"`
```

**问题**: 配置项允许fallback行为，违反质量原则

**修复**: 移除此字段

**优先级**: 🔴 高（影响架构）

## 修复计划

### Phase 1: 高优先级（✅ 已完成）

1. **ai/types.go**: 移除FallbackOnError字段 🔴 ✅
2. **converter/ai_client.rs:316**: 路径检查改为必需 🔴 ✅

### Phase 2: 中优先级（✅ 已完成）

3. **converter/ai_client.rs:620**: 解析失败返回错误 🟡 ✅
4. **cli/dependency_checker.rs:222**: 返回None并报错 🟡 ✅

### Phase 3: 文档更新

5. 更新API文档说明必需参数
6. 更新错误处理指南

## 遵循原则

### ✅ 我们做到了

1. **响亮报错**: AI服务失败时立即exit(1)
2. **拒绝降级**: convert.rs中明确"No fallback available"
3. **架构清晰**: CLI工具是策略选择，不是fallback
4. **文档完整**: 代码注释明确说明反fallback原则

### ❌ 需要改进

1. 仍有3处静默fallback（dummy path、empty map、command name）
2. 配置项暴露fallback选项
3. 部分错误处理过于宽容

## 质量影响评估

**当前状态**: 85%符合质量宣言

**修复后**: 100%符合质量宣言

**风险**: 
- ✅ 修复会使错误更明显（好事）
- ✅ 用户必须正确配置（符合原则）
- ✅ 系统行为更可预测

---

**审计结论**: 大部分代码符合质量原则，仅4处需要修复 ✅

**符合质量宣言**: Phase 1修复后可达100% ✅
