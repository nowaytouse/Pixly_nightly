# Go孤儿代码废弃说明

## 废弃日期
2025-11-11

## 废弃原因

根据PROJECT_QUALITY_MANIFESTO.md的"禁止孤儿代码"原则，这些代码已被识别为孤儿代码并移至此处。

## 孤儿代码定义

- **定义但未调用**：代码存在但没有被项目的任何活跃部分使用
- **引用不存在的包**：依赖不存在的模块，导致编译失败
- **与现有架构不符**：不符合当前双内核架构（Go AI + Rust执行）

## 废弃的模块

### 1. predictor/ 目录

**问题**:
- 引用不存在的`pixly/knowledge`包
- 与AI服务核心（ai/）无集成
- 自身内部循环引用
- 编译失败

**文件**:
- predictor/custom_predictor.go
- predictor/predictor_v31.go
- predictor/time_estimator.go
- predictor/enhanced/ai_predictor.go
- predictor/ml/adaptive_predictor.go
- predictor/ml/alpha_predictor.go
- predictor/ml/chroma_predictor.go
- predictor/ml/format_recommender.go

**现状**:
- AI预测功能已由`ai/`模块实现
- 使用Python Bridge（ai/python_bridge.go）调用predict_params.py
- LightGBM模型正常工作

### 2. pkg/concurrency/ 目录

**问题**:
- 引用不存在的`pixly/pkg/core/types`包
- 无任何其他模块引用
- 编译失败

**文件**:
- pkg/concurrency/smart_concurrency.go
- pkg/concurrency/smart_concurrency_core.go

**现状**:
- Go服务是AI预测服务，不需要并发管理
- Rust端负责实际执行，有自己的并发策略

## 迁移路径

如果未来需要这些功能：

### predictor功能
1. 使用现有的`ai/`模块
2. 通过`ai/python_bridge.go`调用Python AI
3. 参考`ai/http_gateway.go`的API设计

### 并发管理
1. Go服务是HTTP服务器，由框架管理并发
2. 如需批量处理，实现在`ai/batch.go`中
3. Rust端使用rayon进行并行处理

## 相关文档

- `docs/architecture/PROJECT_QUALITY_MANIFESTO.md` - 质量原则
- `docs/todolist/MASTER_TODO_LIST.md` - 任务清单Q-001
- `THREE_TIER_INTEGRATION_STATUS.md` - 三端架构说明

## 清理决策

**决策**: 废弃而非删除
**理由**:
1. 保留历史记录，便于审查
2. 如有遗漏的引用可以找回
3. 可作为反面教材学习

**审查**: 如3个月内无需求，可永久删除

---

**此废弃符合质量宣言原则：禁止孤儿代码** ✅
