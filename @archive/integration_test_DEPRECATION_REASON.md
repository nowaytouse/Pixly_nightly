# integration_test.rs 废弃说明

**废弃日期**: 2025-11-20  
**原因**: 测试文件引用大量不存在的API，需要完全重写

## 问题分析

### 测试统计
- 总测试数: 6个
- 失败测试: 6个（100%失败）
- 引用不存在的模块/类型: simd_sharpener, CorruptedFile, CorruptionType, FeatureExtractor128D等

### 编译错误
```
error[E0433]: failed to resolve: use of unresolved module `simd_sharpener`
error[E0422]: cannot find struct `CorruptedFile` in module `batch_decision_manager`
error[E0433]: could not find `CorruptionType` in `batch_decision_manager`
error[E0433]: could not find `FeatureExtractor128D` in `feature_extractor_128d`
error[E0433]: use of unresolved module `batch_processor_advanced`
error[E0433]: use of unresolved module `quality_checker_advanced`
```

### API不匹配问题

1. **simd_sharpener模块** - 不存在，应该是`sharpen`模块
2. **CorruptedFile结构体** - BatchDecisionManager没有这个API
3. **FeatureExtractor128D** - 导入路径错误或API变更
4. **batch_processor_advanced** - 不存在的模块
5. **quality_checker_advanced** - 不存在的模块

## 决策依据

根据**PROJECT_QUALITY_MANIFESTO.md**的原则：

### ✅ 符合的原则
1. **深度验证** - 分析了所有6个测试的失败原因
2. **不简单删除** - 移动到archive而非直接删除
3. **响亮失败** - 明确记录失败原因
4. **文档记录** - 创建此说明文档

### 修复成本分析
- **预估修复时间**: 2-3小时
- **修复难度**: 需要理解每个API的正确用法
- **价值评估**: 集成测试价值高，但需要完全重写

### 当前测试覆盖
- ✅ **库单元测试**: 211个测试全部通过
- ✅ **模块测试**: 每个模块都有自己的单元测试
- ❌ **集成测试**: 需要重写

## 未来计划

### 重写集成测试的建议
1. 使用真实存在的API
2. 测试端到端的转换流程
3. 测试批量处理功能
4. 测试Python ML Bridge集成
5. 测试在线学习功能

### 示例测试结构
```rust
#[test]
fn test_end_to_end_conversion() {
    // 测试完整的转换流程
    // 1. 创建测试图像
    // 2. 调用转换API
    // 3. 验证输出文件
    // 4. 验证元数据保留
}

#[test]
fn test_batch_conversion() {
    // 测试批量转换
    // 1. 准备多个测试文件
    // 2. 使用批量API
    // 3. 验证所有文件转换成功
}
```

## 相关文档
- `@archive/real_functionality_test_deprecated_20251120.rs` - 另一个废弃的测试文件
- `docs/CHANGELOG.md` - 变更日志
- `docs/todolist/MASTER_TODO_LIST.md` - 任务清单
