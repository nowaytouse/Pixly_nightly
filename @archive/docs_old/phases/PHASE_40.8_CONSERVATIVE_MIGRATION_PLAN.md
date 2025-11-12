# Phase 40.8: 保守迁移计划

**Date**: 2025-11-06  
**Principle**: 保守迁移，严禁脚本简单化处理

## 🛑 紧急停止

已停止自动化脚本执行，改为手动逐个分析。

## 📊 当前状态分析

### ✅ 已执行的操作
1. **pkg模块已复制到core/go/** (但原文件未删除)
   - `pkg/ai` → `core/go/ai`
   - `pkg/predictor` → `core/go/predictor`
   - `pkg/quality` → `core/go/quality`
   - `pkg/knowledge` → `core/go/knowledge`
   - `pkg/ui` → `core/go/services/ui`
   - `pkg/monitor` → `core/go/services/monitor`
   - `pkg/progressui` → `core/go/services/progressui`

2. **部分模块已移到deprecated/pkg_backup/**
   - `pkg/converter`
   - `pkg/metadata`
   - `pkg/xmp`
   - `pkg/deduplicator`
   - `pkg/scanner`
   - `pkg/utils`
   - `pkg/config`
   - `pkg/cache`
   - `pkg/checkpoint`
   - `pkg/concurrency`
   - `pkg/errorhandling`
   - `pkg/validation`
   - ... (more)

### ❌ 问题：过于激进

发现cmd/中的GO代码**正在使用**很多被移动的模块：

| 模块 | 使用位置 | 状态 | 影响 |
|------|---------|------|------|
| `pkg/converter` | pixly-cli, rust-service | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/metadata` | pixly-cli | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/concurrency` | cmd/commands | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/checkpoint` | cmd/commands | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/quality` | cmd/commands | 复制到core/go | ✅ 但需更新导入 |
| `pkg/validation` | cmd/commands | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/errorhandling` | cmd/commands | 已移到deprecated | ❌ 会破坏构建 |
| `pkg/config` | cmd/commands | 已移到deprecated | ❌ 会破坏构建 |

## 🔄 恢复步骤

### Step 1: 恢复被移动的GO活跃模块

```bash
# 恢复cmd正在使用的模块
cd /Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly

# 从deprecated恢复
mv deprecated/pkg_backup/converter pkg/converter
mv deprecated/pkg_backup/metadata pkg/metadata
mv deprecated/pkg_backup/concurrency pkg/concurrency
mv deprecated/pkg_backup/checkpoint pkg/checkpoint
mv deprecated/pkg_backup/validation pkg/validation
mv deprecated/pkg_backup/errorhandling pkg/errorhandling
mv deprecated/pkg_backup/config pkg/config
mv deprecated/pkg_backup/system pkg/system  # 如果存在
```

### Step 2: 保留deprecated中真正废弃的模块

这些模块**确实**可以废弃（Rust已实现且GO不再使用）：
- `xmp` - Rust的metadata.rs已实现
- `deduplicator` - 未在cmd中使用
- `scanner` - 未在cmd中使用
- `utils` - 通用工具，各核心自行实现
- `cache` - 未在cmd中使用
- `i18n` - 插件层处理
- `internal` - 空或废弃
- `core` - 空或废弃
- `security` - 未在cmd中使用
- `protection` - 未在cmd中使用
- `pipeline` - 未在cmd中使用
- `tools` - 未在cmd中使用  
- `optimizer` - 未在cmd中使用

## 📋 正确的迁移策略

### Phase 1: 分析依赖（当前阶段）

- [x] 找出cmd中所有pkg导入
- [ ] 分析每个模块的功能
- [ ] 确定哪些被Rust替代
- [ ] 确定哪些GO服务还需要
- [ ] 确定哪些真正废弃

### Phase 2: 逐个模块迁移

#### 模块分类

**A类：GO AI核心（必须保留）**
- `pkg/ai` - AI核心
- `pkg/predictor` - 参数预测
- `pkg/quality` - 质量评估
- `pkg/knowledge` - 知识库

**B类：GO服务支持（需要保留）**
- `pkg/converter` - GO转换器接口（cmd/pixly-cli使用）
- `pkg/metadata` - 元数据处理（cmd/pixly-cli使用）
- `pkg/concurrency` - 并发控制（cmd/commands使用）
- `pkg/checkpoint` - 断点续传（cmd/commands使用）
- `pkg/validation` - 验证逻辑（cmd/commands使用）
- `pkg/errorhandling` - 错误恢复（cmd/commands使用）
- `pkg/config` - 配置管理（cmd/commands使用）
- `pkg/system` - 系统工具（cmd/pixly-cli使用）
- `pkg/progressui` - 进度UI（cmd/commands使用）
- `pkg/monitor` - 监控（cmd/commands可能使用）
- `pkg/ui` - UI服务（cmd可能使用）

**C类：Rust已替代（可废弃）**
- `pkg/xmp` → Rust metadata.rs
- `pkg/deduplicator` → 可移到Rust
- `pkg/scanner` → 可移到Rust

**D类：通用工具（可废弃）**
- `pkg/utils` - 各核心自行实现
- `pkg/cache` - 不再需要
- `pkg/i18n` - 插件层处理
- `pkg/internal` - 废弃
- `pkg/core` - 废弃
- `pkg/security` - 不再需要
- `pkg/protection` - 不再需要
- `pkg/pipeline` - 不再需要
- `pkg/tools` - 不再需要
- `pkg/optimizer` - 不再需要

### Phase 3: 手动迁移计划

#### Step 3.1: A类模块（AI核心）
```
pkg/ai          → core/go/ai         (保留原路径，创建symlink)
pkg/predictor   → core/go/predictor  (保留原路径，创建symlink)
pkg/quality     → core/go/quality    (保留原路径，创建symlink)
pkg/knowledge   → core/go/knowledge  (保留原路径，创建symlink)
```

#### Step 3.2: B类模块（GO服务）
```
观察cmd使用情况，决定：
选项1: 保留在pkg/（向后兼容）
选项2: 移到core/go/services/，更新cmd导入
选项3: 逐步废弃，功能迁移到Rust
```

#### Step 3.3: C类模块（Rust替代）
```
pkg/xmp → 已废弃 (Rust实现)
pkg/deduplicator → 待实现到Rust
pkg/scanner → 待实现到Rust
```

#### Step 3.4: D类模块（真正废弃）
```
移到deprecated/pkg_废弃/
```

## ⚠️ 重要原则

1. **不破坏现有功能** - cmd必须能正常编译运行
2. **逐步迁移** - 一次一个模块，测试后再继续
3. **保留向后兼容** - 使用symlink保持旧路径可用
4. **文档先行** - 每个改动都记录原因
5. **可回滚** - 保留备份，随时可恢复

## 🎯 下一步行动

**立即**:
1. 恢复cmd正在使用的模块（从deprecated恢复到pkg）
2. 测试cmd/ai-service能否编译
3. 测试cmd/pixly-cli能否编译

**然后**:
4. 仔细分析每个B类模块的使用情况
5. 决定是保留还是迁移到Rust
6. 制定详细的逐个模块迁移计划

**最后**:
7. 手动执行迁移（一次一个）
8. 每步都测试
9. 更新文档

---

**教训**: 不要用脚本批量处理复杂的依赖关系。保守 > 激进。
