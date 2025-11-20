# ML模块废弃说明

**废弃日期**: 2025-11-20  
**决策依据**: 深度价值分析 + 质量宣言合规性审查

## 📦 废弃的模块

1. `src/ml_predictor.rs` (346行)
2. `src/automl.rs` (400+行)

## 🔍 废弃原因

### ml_predictor.rs - 伪装的ML实现

**声称的功能**:
- "真实AI预测"
- "基于LightGBM模型"
- 12维特征工程

**实际情况**:
```rust
// 代码中的注释承认了真相:
// 简化的ML预测 (真实实现需要LightGBM库)
// 这里使用基于特征的启发式规则模拟ML输出
```

**问题**:
1. ❌ **伪装成AI** - 声称LightGBM但实际是if-else规则
2. ❌ **从未被使用** - CLI从不调用，只在测试中
3. ❌ **硬编码配置** - `load_webp_model()` 只返回硬编码的配置，不读取模型文件
4. ❌ **违反真实性原则** - 质量宣言明确禁止这种伪装

**代码证据**:
```rust
fn predict_quality(&self, _normalized: &[f64], features: &ImageFeatures) -> u32 {
    // 基于复杂度和大小的质量预测
    let base_quality = 80;
    let complexity_bonus = (features.complexity * 15.0) as u32;
    let size_penalty = if features.is_large_image() { 5 } else { 0 };
    
    (base_quality + complexity_bonus - size_penalty).clamp(60, 95)
}
```
这是简单的规则引擎，不是ML推理！

**使用情况分析**:
```bash
# CLI调用: 0次
grep -rn "MLPredictor::new" pixly_*.rs | grep -v test
# 结果: 无

# 只在测试中使用
grep -rn "MLPredictor::new" tests/
# 结果: 3次测试调用
```

### automl.rs - 纯粹的孤儿代码

**声称的功能**:
- 自动特征选择
- 自动模型选择
- 自动超参数优化
- 集成学习

**实际情况**:
1. ❌ **完全未使用** - 零业务逻辑引用
2. ❌ **只有框架** - 大量空函数和TODO
3. ❌ **与架构不符** - 项目已有Python ML Bridge

**使用情况分析**:
```bash
# 业务逻辑调用: 0次
grep -rn "AutoML::new" --include="*.rs" . | grep -v test | grep -v "@archive"
# 结果: 无
```

## ✅ 实际工作的ML系统

**当前架构**:
```
Rust CLI (pixly_converter_cli.rs)
  ↓
python_ml_caller::call_python_ml()
  ↓
Python ML Bridge (scripts/ml_bridge.py)
  ↓
真实的LightGBM模型 (models/lightgbm_*.txt)
  ↓
返回预测结果
```

**关键文件**:
- ✅ `src/python_ml_caller.rs` - 真正的ML调用
- ✅ `src/feature_extractor_128d.rs` - 128维特征提取
- ✅ `scripts/ml_bridge.py` - Python ML桥接
- ✅ `models/lightgbm_quality_128d.txt` - 真实训练的模型

**证据**:
```rust
// pixly_converter_cli.rs:532
use pixly_kernel::python_ml_caller::{call_python_ml, MLPredictRequest};
// ...
match call_python_ml(&ml_request) {
    Ok(ml_response) => {
        println!("✅ Python ML video prediction received:");
        // 真正使用ML预测结果
    }
}
```

## 🚨 违反的质量宣言原则

### 1. 真实性原则 🔴
> "代码真正做它声称要做的事"

- ml_predictor.rs 声称"真实AI预测"但实际是规则引擎
- 伪装成ML但没有真正的模型推理
- 从未被实际业务逻辑调用

### 2. 反对摆设代码 🔴
> "功能真正地工作（不模拟、不作弊）"

- automl.rs 完全未使用，纯粹摆设
- ml_predictor.rs 只在测试中使用，实际业务不调用

### 3. 孤儿代码 🔴
> "定义了但从未调用"

- 两个模块都是孤儿代码
- 在lib.rs中导出但从不使用

## 💎 提取的价值

虽然代码被废弃，但知识被保留：

**已创建文档**: `docs/ML_FEATURE_ENGINEERING_REFERENCE.md`

**保留内容**:
1. ✅ 12维特征定义和含义
2. ✅ Scaler参数（mean/std）
3. ✅ 特征提取逻辑
4. ✅ 为什么升级到128维的说明

## 📋 清理检查清单

- [x] 深度价值分析完成
- [x] 知识提取到文档
- [x] 创建废弃说明
- [x] 验证无业务逻辑依赖
- [x] 确认测试覆盖充足
- [ ] 移动文件到@archive
- [ ] 更新src/lib.rs
- [ ] 运行完整测试
- [ ] 更新CHANGELOG.md
- [ ] Git提交

## 🎓 教训总结

1. **不要被注释欺骗** - "真实AI预测"实际是规则引擎
2. **深度验证很重要** - 表面看起来有价值，实际是伪装
3. **价值提取优先** - 即使要删除，也要先提取知识
4. **文档化决策** - 记录为什么删除，避免重复劳动
5. **遵循质量宣言** - 真实性 > 表面功能

## 🔗 相关资源

- 价值提取文档: `docs/ML_FEATURE_ENGINEERING_REFERENCE.md`
- 当前ML系统: `src/python_ml_caller.rs`
- 128维特征: `src/feature_extractor_128d.rs`
- Python ML Bridge: `scripts/ml_bridge.py`
- 质量宣言: `PROJECT_QUALITY_MANIFESTO.md`

---

**结论**: 这两个模块违反了项目的核心质量原则，且已有更好的替代方案。废弃是正确且负责任的决定。
