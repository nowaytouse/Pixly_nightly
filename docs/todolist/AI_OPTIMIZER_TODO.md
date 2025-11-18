# AI Optimizer Plugin - TODO List

## 高优先级 🔴

### AI-001: 集成现有的Python ML + Rust推理系统
**状态**: ✅ 已完成  
**优先级**: 🔴 高  
**完成时间**: 2025-11-18  

**实现内容**:
- ✅ 创建`cli_analyze.rs`模块
- ✅ 集成`format_recommender.rs` AI推荐器
- ✅ 使用真实的ML系统（不是硬编码规则）
- ✅ 响亮的错误处理（AI失败时明确报错）
- ✅ JSON输出支持（供JS解析）

**真实架构** (Python + Rust):
- ✅ Python训练: `scripts/ml_bridge.py` + LightGBM模型
- ✅ Rust推理: `src/ml_bridge.rs` + 128维特征提取
- ✅ 特征提取器: `src/feature_extractor_128d.rs`
- ✅ 统一AI接口: `src/unified_ai_interface.rs`
- ✅ 格式推荐器: `src/format_recommender.rs`
- ❌ **没有GO服务**（过时架构，已废弃）

**需要实现**:
1. 在`cli_analyze.rs`中调用现有的ML系统
   ```rust
   use crate::unified_ai_interface::UnifiedAIPredictor;
   use crate::feature_extractor_128d::FeatureExtractor128D;
   use crate::format_recommender::AIFormatRecommender;
   
   // 提取128维特征
   let extractor = FeatureExtractor128D::new();
   let features = extractor.extract(file_path)?;
   
   // AI预测
   let predictor = UnifiedAIPredictor::new();
   let prediction = predictor.predict(&features, target_format)?;
   
   // 或使用格式推荐器
   let recommender = AIFormatRecommender::new();
   let recommendation = recommender.recommend(&features)?;
   ```

2. 删除硬编码规则
   - 移除临时的match语句
   - 使用真实的ML预测

3. 错误处理
   - Python进程不可用时响亮报错
   - 模型文件缺失时明确提示
   - 无fallback到硬编码规则

**参考**:
- `PROJECT_QUALITY_MANIFESTO.md` - 反对作弊代码
- `src/cli_analyze.rs:108` - 当前临时实现

---

### AI-002: 完善analyze命令的CLI集成
**状态**: ✅ 已完成  
**优先级**: 🔴 高  
**完成时间**: 2025-11-18  

**实现内容**:
1. ✅ 在`pixly_converter_cli.rs`中添加`Analyze`命令
2. ✅ 参数支持:
   - `--ai`: 使用AI推荐（默认true）
   - `--json`: JSON格式输出
   - `--format <format>`: 指定目标格式
3. ✅ 帮助文档完整
4. ✅ 测试通过（logo.png测试成功）

**测试结果**:
```bash
./target/release/pixly-converter analyze ./plugin/format-vue/logo.png --ai --json
# ✅ 输出正确的JSON格式
# ✅ AI推荐: AVIF (confidence: 75%)
# ✅ 预估大小: 7.7 KB (减少50%)
```

---

## 中优先级 🟡

### AI-003: AI插件UI完善
**状态**: ✅ 核心功能完成  
**优先级**: 🟡 中  
**完成时间**: 2025-11-18  

**已完成**:
- ✅ 基础Vue3 + Element Plus架构
- ✅ Eagle API集成
- ✅ Rust CLI调用封装（真实AI调用）
- ✅ 删除fallback hell违规代码
- ✅ JSON输出解析（支持混合stderr/stdout）
- ✅ 响亮的错误处理

**待完善**（低优先级）:
- [ ] 主题适配（参考官方AI插件）
- [ ] Comet动画效果
- [ ] 状态图标系统
- [ ] 对比视图组件
- [ ] 批量处理进度优化

---

### AI-004: 媒体特征提取增强
**状态**: ✅ 已完成（图片部分）  
**优先级**: 🟡 中  
**完成时间**: 2025-11-18  

**已完成**:
1. ✅ 图片复杂度计算
   - ✅ 纹理分析（Sobel边缘检测）
   - ✅ 边缘检测（4方向分布）
   - ✅ 色彩复杂度（HSV统计）
   - ✅ 128维特征全部真实计算

**待实现**:
2. 视频场景分析
   - 场景变化检测
   - 运动复杂度

3. 音频动态范围分析

---

## 低优先级 🟢

### AI-005: 与format-vue插件集成
**状态**: ❌ 未开始  
**优先级**: 🟢 低  
**预计时间**: 2小时  

**需要实现**:
1. 参数传递机制
2. 插件间通信
3. 一键应用推荐

---

### AI-006: 批量分析优化
**状态**: ❌ 未开始  
**优先级**: 🟢 低  
**预计时间**: 2小时  

**需要实现**:
1. 并行分析
2. 进度回调
3. 结果缓存

---

## 质量标准

遵循 `PROJECT_QUALITY_MANIFESTO.md`:
- ✅ 真实的AI调用（不模拟）
- ✅ 响亮的错误处理
- ✅ 完整的功能实现
- ✅ 明确标注临时实现
- ✅ 记录TODO任务

---

**最后更新**: 2025-11-18  
**负责人**: Kiro AI Assistant
