# 架构要求注释记录

## 修改的文件 (关键架构注释已添加)

### 1. pixly-rust/src/converter/ai_client.rs
**新增**: 60+ lines架构注释
**内容**:
- 双内核架构说明
- 职责分离 (Rust/Go/JS)
- 通信协议 (端口50052)
- AI预测字段 (lossless, format_options)
- 禁止事项 (4条)
- 测试要求

### 2. cmd/ai-service/main.go
**新增**: 65+ lines架构注释
**内容**:
- Go AI服务职责定位
- 禁止事项 (不做转换)
- API端点列表
- AI预测字段要求
- 双内核架构图
- 测试方法

### 3. pixly-rust/src/converter/strategies/mod.rs
**已有**: 30+ lines架构注释
**内容**:
- 双轨策略 (CLI + Native)
- 策略优先级
- 智能降级机制

## 总计
- **3个关键文件**
- **155+ lines架构注释**
- **避免重复犯错的关键要求永久记录**

## 关键禁止事项 (已在代码中强调)

1. ❌ JS插件不做转换 (04-conversion.js已删除)
2. ❌ 不硬编码 lossless=false (必须用AI预测)
3. ❌ 不硬编码 format_options=[] (必须用AI预测)
4. ❌ 端口不改为8080 (固定50052)
5. ❌ Go服务不做转换 (仅AI预测)

## 双内核架构 (已明确)

```
🦀 Rust (转换执行) ←HTTP→ �� Go (AI预测) ←SQLite→ 🐍 Python (训练)
     CLI工具                HTTP API              LightGBM
     Native编码器           多模型                增量训练
     策略系统               A/B测试               模型评估
```

