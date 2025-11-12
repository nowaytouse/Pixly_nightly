# 🔧 Phase 46.9 执行计划 - 架构修正

> **目标**: 代码实现符合正确的架构定位  
> **状态**: ⏳ 规划完成，准备执行  
> **时间**: 2025-11-11 08:35

---

## 🎯 核心问题总结

### 问题1: Rust CLI逻辑需要修正
**当前行为**:
- 即使用户指定了`--quality`和`--speed`，仍会调用AI预测
- 没有明确的"用户手动参数"vs"AI推荐参数"区分

**目标行为**:
- 用户明确指定参数 → 不调用AI，直接执行
- 用户未指定参数 → 调用AI获取推荐
- 提供`--use-defaults`选项 → 使用默认值，不调用AI

---

## 📝 Phase 46统一任务总结

### ✅ Phase 46.8 已完成（100%）
1. ✅ 三端错误码/日志/常量统一
2. ✅ Rust参数透明化
3. ✅ Go代码迁移（7文件50处）
4. ✅ 架构角色文档明确
5. ✅ 测试验证通过

### ⏳ Phase 46.9 待执行（架构修正）
1. ⏳ 修正CLI逻辑 - 支持用户手动参数
2. ⏳ 重命名误导性文件/函数
3. ⏳ 更新代码注释
4. ⏳ 验证Rust独立运行能力

---

## 🔧 Phase 46.9 详细修正项

### 修正1: CLI参数处理逻辑 (P0)

**文件**: `core/rust/src/cli/commands/convert.rs`

**当前代码**:
```rust
// 智能参数优化
let (quality, speed) = if !options.no_auto {
    optimize_parameters(input, output, options.quality, options.speed, ...)
} else {
    (options.quality, options.speed)
};

// AI预测
let prediction_data = if !options.no_auto {
    perform_ai_prediction(input, output, quality, speed, ...)
} else {
    None
};
```

**问题**: 即使用户指定参数，仍会调用AI

**修正后代码**:
```rust
// 确定参数来源
let params_source = determine_params_source(&options);

let (quality, speed, prediction_data) = match params_source {
    ParamsSource::UserSpecified => {
        // 用户明确指定，不调用AI
        info!("Using user-specified parameters");
        (options.quality, options.speed, None)
    },
    ParamsSource::Defaults => {
        // 使用默认值，不调用AI
        info!("Using default parameters");
        (85, 4, None) // 默认quality=85, speed=4
    },
    ParamsSource::AIRequired => {
        // 需要AI推荐
        info!("Requesting AI parameter recommendation");
        let (q, s) = optimize_parameters(...)?;
        let pred = perform_ai_prediction(...)?;
        (q, s, Some(pred))
    },
};

// 添加参数来源标记
let mut final_options = options;
final_options.params_source = params_source.to_string();
```

**辅助函数**:
```rust
enum ParamsSource {
    UserSpecified,  // 用户明确指定
    Defaults,       // 使用默认值
    AIRequired,     // 需要AI推荐
}

fn determine_params_source(options: &ConvertOptions) -> ParamsSource {
    // 如果用户明确指定了quality和speed
    if options.quality_explicit && options.speed_explicit {
        return ParamsSource::UserSpecified;
    }
    
    // 如果用户使用--use-defaults
    if options.use_defaults {
        return ParamsSource::Defaults;
    }
    
    // 如果用户使用--no-auto
    if options.no_auto {
        // 使用CLI提供的默认值或用户部分指定的值
        return ParamsSource::Defaults;
    }
    
    // 其他情况需要AI
    ParamsSource::AIRequired
}
```

---

### 修正2: 添加--use-defaults选项 (P0)

**文件**: `core/rust/src/cli/commands/convert.rs`

**添加到ConvertOptions**:
```rust
struct ConvertOptions {
    quality: u8,
    speed: u8,
    quality_explicit: bool,  // 🆕 用户是否明确指定quality
    speed_explicit: bool,    // 🆕 用户是否明确指定speed
    use_defaults: bool,      // 🆕 使用默认值，不调用AI
    no_auto: bool,
    // ... 其他字段
}
```

**解析逻辑**:
```rust
fn parse_options(args: &[String]) -> ConvertOptions {
    let mut options = ConvertOptions::default();
    
    for i in 2..args.len() {
        match args[i].as_str() {
            "--quality" | "-q" => {
                options.quality = parse_u8(&args[i + 1]);
                options.quality_explicit = true; // 🆕 标记为明确指定
            },
            "--speed" | "-s" => {
                options.speed = parse_u8(&args[i + 1]);
                options.speed_explicit = true; // 🆕 标记为明确指定
            },
            "--use-defaults" => {
                options.use_defaults = true; // 🆕 新选项
            },
            // ... 其他选项
        }
    }
    
    options
}
```

---

### 修正3: 文件和函数重命名 (P1)

#### 文件重命名
```bash
# 1. 重命名param_optimizers.rs
cd core/rust/src/converter
mv param_optimizers.rs ai_parameter_provider.rs

# 2. 更新mod.rs
# 修改: pub mod param_optimizers;
# 为:   pub mod ai_parameter_provider;
```

#### 函数重命名
```rust
// ai_parameter_provider.rs

// 旧名称 → 新名称
optimize_avif() → get_ai_params_for_avif()
optimize_webp() → get_ai_params_for_webp()
optimize_jxl()  → get_ai_params_for_jxl()
optimize_png()  → get_ai_params_for_png()
optimize_jpeg() → get_ai_params_for_jpeg()
```

---

### 修正4: 注释和文档更新 (P1)

#### 更新ai_parameter_provider.rs顶部注释
```rust
/**
 * AI参数提供器
 * 
 * 架构定位：
 * - Rust核心是唯一的转换器执行层和文件处理器
 * - Go AI是最完全的AI增强核心
 * - 此文件仅负责调用Go AI服务获取参数推荐
 * 
 * 职责：
 * - ✅ 调用Go AI服务
 * - ✅ 验证AI返回的参数
 * - ✅ AI不可用时响亮报错
 * 
 * 不负责：
 * - ❌ Rust自己不做参数优化
 * - ❌ 不做智能决策
 * - ❌ 不做fallback到规则
 * 
 * 使用场景：
 * - 仅在用户未明确指定参数时调用
 * - 用户明确指定参数时跳过AI
 */
```

#### 更新函数注释
```rust
/// 从Go AI服务获取AVIF参数推荐
/// 
/// # 参数
/// * `chars` - 图像特征（仅用于传递给AI）
/// * `prefer_quality` - 是否倾向质量（仅用于传递给AI）
/// 
/// # 返回
/// * `Ok(OptimizedParams)` - AI推荐的参数
/// * `Err` - AI服务不可用或返回无效参数
/// 
/// # 注意
/// - 此函数仅调用Go AI服务，Rust不做参数优化
/// - AI不可用时会响亮报错（符合质量宣言）
/// - 用户明确指定参数时不应调用此函数
pub fn get_ai_params_for_avif(
    chars: &ImageCharacteristics,
    prefer_quality: bool
) -> Result<OptimizedParams> {
    // ...
}
```

---

## 📊 执行顺序和依赖

### 阶段1: CLI逻辑修正（必须先完成）
1. ⏳ 修改ConvertOptions结构
2. ⏳ 添加params_source判断逻辑
3. ⏳ 添加--use-defaults选项
4. ⏳ 测试三种模式

### 阶段2: 重命名和注释（可并行）
5. ⏳ 重命名文件和函数
6. ⏳ 更新所有注释
7. ⏳ 更新导入路径

### 阶段3: 验证和文档（最后）
8. ⏳ 编译测试
9. ⏳ 运行时测试
10. ⏳ 更新README

---

## 🧪 测试计划

### 测试1: 用户手动参数
```bash
pixly-rust convert input.png output.avif --quality 85 --speed 4
# 期望: 不调用AI，直接使用用户参数
# 验证: 日志中应显示"Using user-specified parameters"
```

### 测试2: 使用默认值
```bash
pixly-rust convert input.png output.avif --use-defaults
# 期望: 不调用AI，使用默认值quality=85, speed=4
# 验证: 日志中应显示"Using default parameters"
```

### 测试3: AI推荐（Go AI运行）
```bash
pixly-rust convert input.png output.avif
# 期望: 调用Go AI服务获取推荐
# 验证: 日志中应显示"Requesting AI parameter recommendation"
```

### 测试4: AI推荐（Go AI未运行）
```bash
# 停止Go AI服务
pixly-rust convert input.png output.avif
# 期望: 响亮报错"AI service required but not available"
# 验证: 转换失败并给出明确错误信息
```

---

## ✅ 成功标准

修正完成后应满足：

1. ✅ **Rust独立运行**
   - 用户指定参数时不调用AI
   - 提供--use-defaults选项
   - 三种模式都能正常工作

2. ✅ **命名清晰**
   - 文件名反映真实职责
   - 函数名不暗示Rust做优化

3. ✅ **注释准确**
   - 明确说明架构角色
   - 说明调用Go AI的目的

4. ✅ **行为符合预期**
   - 用户手动参数优先
   - AI是可选的增强
   - 响亮报错（AI不可用时）

---

## 📝 下一步

**Phase 46.8完成**: ✅ 100%（三端统一）  
**Phase 46.9规划**: ✅ 100%（架构修正计划）  
**Phase 46.9执行**: ⏳ **待开始**

**预计工作量**: 1.5-2小时  
**优先级**: P0-P1（高优先级）  
**建议**: 逐步执行，每个修正后测试验证

---

**任务状态**: ⏳ **规划完成，等待执行**  
**准备度**: ✅ **100%** （计划详细，可立即开始）
