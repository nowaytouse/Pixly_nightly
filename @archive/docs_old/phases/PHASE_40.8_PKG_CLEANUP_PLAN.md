# Phase 40.8: pkg文件夹清理计划

## ✅ 迁移完成状态 (2025-11-06 15:45)

### 已完成的迁移

#### 1. AI核心迁移 ✅
- **源路径**: `pkg/{ai,predictor,quality,knowledge}`
- **目标路径**: `core/go/{ai,predictor,quality,knowledge}`
- **向后兼容**: 已创建symlinks (`pkg/* -> core/go/*`)
- **验证结果**: ✅ AI服务编译通过

#### 2. Rust核心迁移 ✅
- **源路径**: `pixly-rust/`
- **目标路径**: `core/rust/`
- **向后兼容**: 已创建symlink (`pixly-rust -> core/rust`)
- **验证结果**: ✅ Rust CLI编译通过

#### 3. 旧cmd服务废弃 ✅
- **已废弃**: `cmd/{pixly,pixly-cli,rust-service,commands}`
- **目标路径**: `deprecated/cmd_old/`
- **保留服务**: `cmd/ai-service` (唯一活跃的Go服务)

### 当前项目结构

```
Pixly_Nightly/
├── core/                    # 🆕 统一的核心代码目录
│   ├── go/                  # Go AI核心
│   │   ├── ai/             # AI服务核心
│   │   ├── predictor/      # 参数预测
│   │   ├── quality/        # 质量评估
│   │   └── knowledge/      # 知识库
│   ├── rust/                # Rust转换核心 (原pixly-rust)
│   │   ├── src/
│   │   └── Cargo.toml
│   └── plugin/              # 插件核心
├── cmd/
│   └── ai-service/          # ✅ 唯一活跃的Go服务
├── pkg/                     # 🔗 向后兼容symlinks
│   ├── ai -> ../core/go/ai
│   ├── predictor -> ../core/go/predictor
│   ├── quality -> ../core/go/quality
│   └── knowledge -> ../core/go/knowledge
├── pixly-rust -> core/rust  # 🔗 向后兼容symlink
├── plugin/                  # JavaScript插件
└── deprecated/              # 废弃代码
    └── cmd_old/            # 旧的Go CLI实现
```

### 迁移质量验证

- [x] AI模块嵌套结构修正 (`core/go/ai/ai/` → `core/go/ai/`)
- [x] Rust模块嵌套结构修正 (`core/rust/pixly-rust/` → `core/rust/`)
- [x] Symlinks正确性验证
- [x] AI服务编译测试通过
- [x] Rust CLI编译测试通过

---

## 原始分析内容

**Date**: 2025-11-06  
**Goal**: 废弃pkg中过时的GO实现，只保留AI核心

## 🎯 核心原则

**架构分工**:
- **Rust**: 文件处理、转换、元数据、验证
- **GO**: AI预测、参数优化、知识库
- **Plugin**: UI交互、Eagle集成

**清理目标**: pkg中文件处理相关的GO代码都是**旧实现**，Rust已经实现，需要废弃。

## 📊 cmd服务分析

### cmd/ai-service/ ✅ 保留
- **用途**: AI参数预测服务 (gRPC + HTTP)
- **依赖**: `pkg/ai`
- **状态**: 核心服务，必须保留

### cmd/pixly-cli/ ❌ 废弃
- **用途**: GO实现的CLI工具
- **依赖**: `pkg/converter`, `pkg/metadata`, `pkg/system`
- **问题**: 与pixly-rust重复
- **决策**: **废弃整个目录**，使用pixly-rust CLI

### cmd/rust-service/ ❌ 废弃
- **用途**: GO包装器调用pkg/converter
- **依赖**: `pkg/converter`
- **问题**: 没必要的中间层
- **决策**: **废弃整个目录**，直接用Rust

### cmd/pixly/ ❓ 待分析
- **用途**: 不明确
- **依赖**: TBD
- **决策**: 需要检查功能

### cmd/commands/ ❌ 废弃
- **用途**: CLI命令实现
- **依赖**: `pkg/converter`, `pkg/concurrency`, `pkg/checkpoint`, etc.
- **问题**: 依赖大量旧实现
- **决策**: **废弃整个目录**（如果是pixly-cli的一部分）

## 📋 pkg模块清理方案

### A类：AI核心（保留）

```
pkg/ai/          → 保留 ✅ cmd/ai-service使用
pkg/predictor/   → 保留 ✅ AI预测核心
pkg/quality/     → 保留 ✅ 质量评估
pkg/knowledge/   → 保留 ✅ 知识库
```

### B类：文件处理（废弃，Rust已实现）

```
pkg/converter/   → 废弃 ❌ Rust: src/converter/strategy.rs
pkg/metadata/    → 废弃 ❌ Rust: src/converter/metadata.rs
pkg/validation/  → 废弃 ❌ Rust: 可在strategy中验证
```

**Rust实现检查**:
- ✅ `pixly-rust/src/converter/strategy.rs` - 转换策略
- ✅ `pixly-rust/src/converter/strategies/` - 各种策略实现
- ✅ `pixly-rust/src/converter/metadata.rs` - 元数据处理
- ✅ `pixly-rust/src/converter/batch.rs` - 批量转换

### C类：辅助功能（废弃，各核心自行实现）

```
pkg/concurrency/    → 废弃 ❌ Rust有自己的并发
pkg/checkpoint/     → 废弃 ❌ 可在Rust实现（如需要）
pkg/errorhandling/  → 废弃 ❌ Rust: anyhow::Result
pkg/config/         → 废弃 ❌ 各自配置
pkg/system/         → 废弃 ❌ 各自实现
pkg/progressui/     → 废弃 ❌ Plugin层处理
pkg/monitor/        → 废弃 ❌ 不需要
pkg/ui/             → 废弃 ❌ Plugin层处理
```

## 🗑️ 清理步骤

### Step 1: 废弃cmd服务 (除ai-service外)

```bash
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly

# 移到deprecated
mkdir -p deprecated/cmd_old
mv cmd/pixly-cli deprecated/cmd_old/
mv cmd/rust-service deprecated/cmd_old/
mv cmd/commands deprecated/cmd_old/

# 检查cmd/pixly是什么
# (待确认后再移动)
```

### Step 2: 废弃pkg文件处理模块

```bash
# 移到deprecated/pkg_file_processing
mkdir -p deprecated/pkg_file_processing

mv pkg/converter deprecated/pkg_file_processing/
mv pkg/metadata deprecated/pkg_file_processing/
mv pkg/validation deprecated/pkg_file_processing/
```

### Step 3: 废弃pkg辅助模块

```bash
# 移到deprecated/pkg_utilities
mkdir -p deprecated/pkg_utilities

mv pkg/concurrency deprecated/pkg_utilities/
mv pkg/checkpoint deprecated/pkg_utilities/
mv pkg/errorhandling deprecated/pkg_utilities/
mv pkg/config deprecated/pkg_utilities/
mv pkg/system deprecated/pkg_utilities/
mv pkg/progressui deprecated/pkg_utilities/
mv pkg/monitor deprecated/pkg_utilities/
mv pkg/ui deprecated/pkg_utilities/
```

### Step 4: 保留pkg AI核心

```bash
# 最终pkg/只剩下：
pkg/
├── ai/
├── predictor/
├── quality/
└── knowledge/
```

### Step 5: 清理core/go副本

```bash
# 删除core/go（只是备份，已不需要）
rm -rf core/go
```

## ✅ 清理后的结构

```
Pixly_Nightly/
├── pixly-rust/           🦀 Rust转换核心
│   ├── src/
│   │   ├── converter/    ✅ 文件转换
│   │   ├── cli/          ✅ CLI接口
│   │   └── ...
│   └── Cargo.toml
│
├── cmd/
│   └── ai-service/       🤖 GO AI服务（唯一保留）
│       └── main.go
│
├── pkg/                  🤖 GO AI核心（仅AI相关）
│   ├── ai/
│   ├── predictor/
│   ├── quality/
│   └── knowledge/
│
├── plugin/               🔌 Eagle插件
│   ├── js/
│   └── index.html
│
└── deprecated/           🗑️ 废弃代码
    ├── cmd_old/
    │   ├── pixly-cli/
    │   ├── rust-service/
    │   └── commands/
    ├── pkg_file_processing/
    │   ├── converter/
    │   ├── metadata/
    │   └── validation/
    └── pkg_utilities/
        ├── concurrency/
        ├── checkpoint/
        └── ...
```

## 📝 清理原因记录

| 模块 | 废弃原因 | Rust替代 |
|------|---------|---------|
| pkg/converter | Rust已完整实现 | src/converter/strategy.rs |
| pkg/metadata | Rust已完整实现 | src/converter/metadata.rs |
| pkg/validation | Rust策略中验证 | strategy.rs中的错误处理 |
| pkg/concurrency | Rust自带并发 | rayon, tokio |
| pkg/checkpoint | 不需要 | 可选实现 |
| pkg/errorhandling | Rust自带 | anyhow::Result |
| cmd/pixly-cli | 重复 | pixly-rust CLI |
| cmd/rust-service | 无用中间层 | 直接用pixly-rust |

## ⚠️ 注意事项

1. **先移到deprecated，不要直接删除**
2. **保留go.mod依赖完整性**（ai-service还需要）
3. **文档记录每个废弃的原因**
4. **检查是否有其他代码引用**

## 🎯 验证步骤

### 验证1: AI服务能否编译
```bash
cd cmd/ai-service
go build
```

### 验证2: Rust CLI功能完整
```bash
pixly-rust --help
pixly-rust convert --help
pixly-rust file-info --help
```

### 验证3: Plugin能否正常工作
- 测试图像转换
- 测试AI参数预测
- 测试Eagle集成

## 📚 相关文档

- [PHASE_40.8_CURRENT_STATUS.md](./PHASE_40.8_CURRENT_STATUS.md)
- [PHASE_40.8_CONSERVATIVE_MIGRATION_PLAN.md](./PHASE_40.8_CONSERVATIVE_MIGRATION_PLAN.md)

---

**目标**: pkg文件夹最终只保留4个AI相关模块，其他全部废弃。
