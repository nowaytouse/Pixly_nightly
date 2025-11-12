# 文档归档说明

## 目的

本目录集中存放历史文档和已经完成归档的资源，帮助保持项目根目录整洁，同时保证所有高价值信息可追溯。

## 目录概览

- `docs_old/` – Phase/Session/Report 等 153 份历史文档的原始结构副本
- `phase_46_docs/` – Phase 46 系列的重点里程碑文档
- `CLEANUP_PLAN.md` – 第二轮归档清理计划与操作记录
- `QUICK_VALIDATION_TEST.md` – 快速验证测试脚本与说明
- `@deprecated/` – 已废弃但仍具参考价值的代码与文档

### 压缩归档

- `historical_docs_phase46-47.tar.gz` – Phase 46-47 历史文档完整备份
- 完整历史归档：`~/Desktop/pixly_archive_backup_20251112.tar.gz`

## 归档策略

1. **核心文档永久保留（5 个）**
   - PROJECT_QUALITY_MANIFESTO.md
   - docs/todolist/MASTER_TODO_LIST.md
   - CHANGELOG.md
   - README.md
   - docs/ARCHITECTURE.md
2. **以下类型统一归档**
   - Phase 文档（`PHASE_*.md`）
   - Session 文档（`SESSION_*.md`）
   - 临时总结/报告（`*_SUMMARY.md`, `*_REPORT.md`）
   - 过期的指南、计划与分析文档
3. **归档时机**
   - 任务进入「已完成」状态后立即归档
   - 文档淘汰或替换时同步归档

## 历史信息导航

1. 查看 `CHANGELOG.md` 获取阶段摘要
2. 对应 Phase/Session 详情可在本目录中查找
3. 若需回溯更早内容，可解压 `historical_docs_phase46-47.tar.gz`
4. `@deprecated/` 下保留的代码可配合 `docs/DEPRECATED_ARCHIVE_ANALYSIS.md` 进行价值提取

## 🎯 已提取价值功能

### 第一轮提取（Phase 47.22）

| 功能 | 源代码 | 现行实现 | 状态 |
|------|--------|----------|------|
| EX-001 模型路由和A/B测试 | `go_ai_service/model_router.go` | `core/python/ai/model_router.py` | ✅ 已完成 |
| EX-002 训练队列管理 | `training_queue.go` | `tools/training_queue.py` | ✅ 已完成 |
| EX-003 XMP元数据处理 | `standalone_tools/merge_xmp/` | `core/python/utils/metadata/xmp_processor.py` | ✅ 已完成 |

### 第二轮提取（Phase 47.23）

| 功能 | 源代码 | 现行实现 | 状态 |
|------|--------|----------|------|
| EX-004 反馈数据库系统 | `feedback_db.go` | `core/python/ai/feedback_db.py` | ✅ 已完成 |
| EX-005 质量评估系统 | `quality/metrics.go` | `core/python/quality/metrics.py` | ✅ 已完成 |
| EX-006 预测准确性分析器 | `knowledge/analyzer.go` | `tools/accuracy_analyzer.py` | ✅ 已完成 |
| EX-007 格式知识库 | `format_knowledge.go` | `core/python/ai/format_knowledge.py` | ✅ 已完成 |
| EX-008 HTTP 参数验证器 | `http_validator.go` | `tools/request_validator.py` | ✅ 已完成 |

> 详细分析请参阅 `docs/DEPRECATED_ARCHIVE_ANALYSIS.md`

## 操作提示

- **检索**：使用 `rg "关键词" @archive` 可快速定位归档文档
- **恢复**：如需查看老版本，可从压缩包解压到临时目录并删除后续变更
- **新增归档**：遵循「更新核心文档 → 立即归档旧版」的流程，避免重复信息

---

**原则**：信息集中、更新优先、文档数量保持最简
