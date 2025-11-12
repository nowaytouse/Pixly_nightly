# ⚠️ DEPRECATED: GO转换服务 (Port 8080)

## 状态
**此服务已被标记为DEPRECATED** - 2025-11-05

## 原因
根据架构对齐决策，GO应仅实现AI预测功能，转换功能应由Rust CLI直接实现。

## 当前状态
- ✅ 功能完整可用
- ✅ 有完整的TUI说明书
- ⚠️ 不推荐新项目使用
- ⚠️ 将在未来版本中移除

## 替代方案

### **推荐: 使用Rust CLI直接转换**
```bash
# 直接调用Rust CLI
pixly-rust convert input.png output.avif -q 85 -s 4

# 或在JavaScript插件中使用
window.rustCLI.convertImage(inputPath, outputPath, format, config)
```

**优势**:
- ✅ 更高性能（无HTTP开销）
- ✅ 更简单的架构
- ✅ 更好的错误处理
- ✅ 符合预期架构

### **替代: 使用AI服务 (仅预测)**
```bash
# GO AI服务 (Port 50052) - 推荐继续使用
curl -X POST http://localhost:50052/api/ai/predict \
  -H 'Content-Type: application/json' \
  -d '{"format": "avif", "quality": 85}'
```

## 迁移指南

### 从GO转换服务 → Rust CLI

**旧方式** (Deprecated):
```javascript
// 通过HTTP调用GO服务
const result = await window.rustConverter.convertImage(
    inputPath, outputPath, format, config
);
```

**新方式** (推荐):
```javascript
// 直接调用Rust CLI
const result = await window.rustCLI.convertImage(
    inputPath, outputPath, format, config
);
```

**区别**:
- 旧方式: JS → HTTP → GO → CLI工具
- 新方式: JS → Rust CLI

## 何时移除

### **Phase 1** (当前)
- ✅ 标记为DEPRECATED
- ✅ 添加警告信息
- ✅ 提供迁移指南

### **Phase 2** (未来)
- 更新所有调用方从HTTP改为CLI
- 验证功能完整性
- 性能测试

### **Phase 3** (移除)
- 删除GO转换服务代码
- 仅保留GO AI服务

**预计移除时间**: TBD（待所有客户端迁移完成）

## 保留原因

### **为什么暂时保留**
1. **向后兼容**: 现有插件仍在使用
2. **过渡期**: 给用户迁移时间
3. **Fallback**: 可作为备用方案

### **何时可以删除**
1. ✅ 所有插件迁移到Rust CLI
2. ✅ 测试验证完成
3. ✅ 没有外部依赖

## 技术细节

### **当前架构** (Deprecated)
```
JavaScript Plugin
    ↓ (HTTP POST)
GO Conversion Service (8080)
    ↓ (exec)
CLI Tools (cjxl, avifenc, cwebp...)
```

### **目标架构** (推荐)
```
JavaScript Plugin
    ↓ (spawn)
Rust CLI (pixly-rust)
    ↓ (direct)
Image Converter Library
```

### **GO AI服务** (继续使用)
```
JavaScript Plugin
    ↓ (HTTP POST)
GO AI Service (50052)
    ↓ (lightgbm)
AI Models
```

## 联系方式

如有疑问，请查阅:
- 架构文档: `ARCHITECTURE_ANALYSIS_AND_TODO.md`
- Rust CLI: `pixly-rust/src/main.rs`
- JS执行器: `plugin/js/plugin-modules/28-rust-cli-executor.js`

---

**最后更新**: 2025-11-05  
**状态**: DEPRECATED  
**建议**: 迁移到Rust CLI
