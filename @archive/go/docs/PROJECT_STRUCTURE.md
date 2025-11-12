# GO项目结构说明

## 📁 当前目录结构

```
go/
├── cmd/                    # 🚀 命令行入口
│   └── pixly-ai/           # AI服务主程序
│
├── ai/                     # 🤖 AI核心模块
│   ├── http_gateway.go     # HTTP网关（主要入口）
│   ├── python_bridge.go    # Python桥接
│   ├── model_manager.go    # 模型管理
│   ├── training_queue.go   # 训练队列
│   ├── pb/                 # Protobuf定义
│   ├── features/           # 特征工程
│   ├── models/             # 模型实现
│   ├── storage/            # 数据存储
│   └── quality/            # ⚠️ 与根quality/重复
│
├── knowledge/              # 📚 知识库
│   ├── database.go         # 数据库操作
│   ├── types.go            # 数据类型
│   ├── query.go            # 查询接口
│   └── seeds/              # 种子数据（140+条）
│       ├── default_knowledge.go
│       └── real_knowledge.go
│
├── predictor/              # 🔮 预测器（部分功能与ai/重叠）
│   ├── ml/                 # ML模块
│   │   └── format_recommender.go  # 格式推荐器（活跃）
│   ├── predictor.go
│   ├── predictor_v31.go    # ⚠️ 旧版本
│   └── batch_decision_*.go # ⚠️ 可能已整合到ai/
│
├── quality/                # 📊 质量评估
│   ├── metrics.go          # 质量指标
│   ├── assessment.go       # 质量评估
│   └── analyzer.go         # 分析器
│
├── deprecated/             # 🗑️ 过时代码（新建）
│   └── README.md           # 使用说明
│
└── bin/                    # 📦 编译输出
    └── pixly-ai            # 可执行文件
```

## ⚠️ 当前问题

### 1. 模块重复
- `ai/quality/` ↔️ `quality/` （质量评估重复）
- `ai/` ↔️ `predictor/` （部分功能重叠）

### 2. 旧版本代码未清理
- `predictor/predictor_v31.go` - 版本号暗示有更新版本
- `predictor/batch_decision_*` - 批处理逻辑可能已集成

### 3. 缺少文档
- 缺少各模块职责说明
- 缺少架构图

## ✅ 推荐的简化方案

### 方案A: 最小化改动（推荐）
**优点**: 不破坏现有功能，快速清理

1. 创建`deprecated/`文件夹 ✅
2. 移动明确过时的代码：
   - `predictor/predictor_v31.go` → `deprecated/`
   - `predictor/batch_decision_*` → `deprecated/`（如果ai/已实现）
3. 添加README标记重复模块
4. 保持运行中的代码不变

### 方案B: 完全重构（需要更多时间）
**优点**: 结构清晰，长期维护性好

```
go/
├── cmd/              # 命令入口
├── core/             # 核心功能（合并ai + predictor）
│   ├── ml/           # 机器学习
│   ├── knowledge/    # 知识库
│   └── quality/      # 质量（合并两个quality）
├── internal/         # 内部工具
└── deprecated/       # 过时代码
```

**缺点**: 需要大量重构和测试

## 🎯 当前行动

已执行：
- ✅ 创建`deprecated/`目录
- ✅ 添加README说明
- ✅ 此文档记录现状

待执行（可选）：
- 📋 评估哪些代码确实已过时
- 📋 移动过时代码到deprecated/
- 📋 更新import路径
- 📋 测试编译

## 📝 维护建议

1. **新代码规范**:
   - 模块职责清晰，避免功能重叠
   - 及时删除或标记过时代码
   
2. **定期清理**:
   - 每季度review `deprecated/`
   - 确认可以安全删除的代码

3. **文档更新**:
   - 修改结构时更新此文档
   - 记录重大架构决策

---
📅 创建时间: 2025-11-08
📊 总文件数: ~80个GO文件
🎯 状态: 运行正常，结构待优化
