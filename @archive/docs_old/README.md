# 📚 PIXLY 项目文档

本目录包含了PIXLY项目的所有技术文档，按类别组织。

## 📁 目录结构

### 🏗️ architecture/ - 架构设计
项目架构、设计决策和技术规范文档。

- **核心架构文档**
- **三端架构设计** (Go AI + Rust Core + JS UI)
- **系统集成方案**
- **跨平台日志规范**

### 📖 guides/ - 使用指南
开发、测试和使用相关的指南文档。

- **开发指南**
- **测试指南**
- **快速开始指南**
- **AI集成指南**
- **日志迁移指南**

### 🔄 phases/ - 开发阶段
按阶段记录的开发过程和功能实现。

- **Phase 36-40**: 主要功能开发
- **Phase 45**: Bug修复和优化
- **各阶段完成报告**

### 📊 reports/ - 报告和分析
质量审计、性能分析、Bug报告等。

- **代码质量审计报告**
- **性能测试报告**
- **Bug修复报告**
- **功能状态报告**
- **迁移完成报告**

### 📝 sessions/ - 会话记录
开发会话的详细记录和总结。

- **会话笔记**
- **会话总结**
- **下次会话计划**

## 🔍 快速索引

### 新手入门
1. [README](../README.md) - 项目概述
2. [快速开始指南](guides/QUICK_START.md)
3. [架构概览](architecture/ARCHITECTURE_CURRENT_STATE.md)

### 开发者
1. [开发指南](guides/DEV_GUIDE_MAGIKA.md)
2. [测试指南](guides/TESTING_GUIDE.md)
3. [代码质量标准](architecture/PROJECT_QUALITY_MANIFESTO.md)

### 维护者
1. [最新会话记录](sessions/)
2. [待办事项](sessions/TODO_NEXT_SESSION.md)
3. [架构分析](architecture/)

## 📌 重要文档

### 必读文档
- 📘 [项目质量宣言](architecture/PROJECT_QUALITY_MANIFESTO.md) - **必读**
- 🏗️ [当前架构状态](architecture/ARCHITECTURE_CURRENT_STATE.md)
- 🚀 [快速开始](guides/QUICK_START.md)

### 核心原则
1. **质量 > 速度**: 永远不牺牲质量换速度
2. **正面解决 > 绕过**: 禁止fallback、禁止模拟数据
3. **响亮报错 > 静默降级**: AI不可用应立即报错
4. **真实调用 > 演示代码**: 所有功能必须真实工作

## 🔄 文档维护

- **添加新文档**: 放入对应的子目录
- **命名规范**: 使用大写加下划线，如 `FEATURE_NAME.md`
- **保持更新**: 重大变更后及时更新相关文档
- **索引维护**: 新增重要文档后更新此README

---

**提示**: 查找特定主题时，可以使用 `grep -r "关键词" docs/` 搜索。
