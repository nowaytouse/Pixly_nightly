# 🔄 僵尸代码复活 - Phase 3: 功能验证与优化

**日期**: 2025-11-19  
**Phase**: 3  
**目标**: 验证所有已注册模块是否真正被使用

---

## 📊 Phase 2 回顾

✅ **已完成**:
- 集成audio_processor.rs
- 集成format_selector.rs到analyze命令
- 集成缓存压缩功能
- 删除2,029行僵尸代码
- 零编译警告

---

## 🎯 Phase 3 目标

### 策略: 深度使用验证

根据PROJECT_QUALITY_MANIFESTO.md的教训：
> **⚠️ 草率归类**: 将未被CLI使用的代码简单标记为"僵尸"
> **⚠️ 价值未充分评估**: 可能丢弃了有价值的算法和功能

**Phase 3不会简单删除代码，而是**:
1. 验证每个模块是否被真正调用
2. 对于未使用的模块，分析其价值
3. 高价值模块 → 集成到CLI/API
4. 低价值模块 → 深度分析后再决定

---

## 📋 验证方法

### 方法1: 编译器分析
```bash
# 检查dead_code警告
cargo build --release 2>&1 | grep "never used"
```

### 方法2: 代码引用搜索
```bash
# 对每个模块，搜索其使用情况
for mod in $(grep '^pub mod' src/lib.rs | awk '{print $3}' | sed 's/;//'); do
    echo "=== $mod ==="
    grep -r "use.*::$mod" src/ --include="*.rs" | wc -l
done
```

### 方法3: CLI命令覆盖
```bash
# 检查哪些模块被CLI使用
grep -r "use crate::" pixly_*.rs | sort -u
```

---

## 🔍 Phase 3 任务

### Task 3.1: 模块使用情况审计 ✅ 高优先级

**目标**: 生成完整的模块使用报告

**输出**: `docs/MODULE_USAGE_REPORT.md`

**内容**:
- 每个模块的引用次数
- 被哪些模块/CLI使用
- 是否有公开API
- 价值评估

---

### Task 3.2: 未使用模块价值分析 🟡 中优先级

**目标**: 对引用次数=0的模块进行深度分析

**分析维度**:
1. 代码质量（算法复杂度、实现完整性）
2. 功能独特性（是否有替代品）
3. 未来价值（roadmap中是否需要）
4. 集成难度（依赖复杂度）

---

### Task 3.3: 高价值模块集成计划 🟡 中优先级

**目标**: 为高价值但未使用的模块制定集成计划

**示例**:
- `gpu_accelerator.rs` → 未来GPU加速功能
- `video_strategy.rs` → 可能合并到video_processor

---

### Task 3.4: 低价值模块清理 🟢 低优先级

**目标**: 安全删除确认无价值的模块

**原则**:
- 必须有充分的价值分析
- 必须确认无依赖
- 必须备份到@archive

---

## ⏱️ 时间估算

- Task 3.1: 1小时
- Task 3.2: 2小时
- Task 3.3: 1小时
- Task 3.4: 30分钟

**总计**: ~4.5小时

---

## 🚀 开始Phase 3

**下一步**: Task 3.1 - 模块使用情况审计
