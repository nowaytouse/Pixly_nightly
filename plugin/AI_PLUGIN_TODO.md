# AI插件待修复项

## 紧急问题

### 1. Rust核心路径 ✅ 已修复
- **问题**: AI插件在找`pixly-rust`而不是`pixly-converter`
- **临时方案**: 创建了符号链接 `pixly-rust -> pixly-converter`
- **永久方案**: 更新AI插件代码使用`pixly-converter`

### 2. 日志系统 ⚠️ 部分完成
- **问题**: AI插件日志全是中文硬编码
- **已完成**:
  - ✅ 复制了log-constants.js和logger.js
  - ✅ 更新了index.html加载日志系统
  - ✅ 更新了i18n.js使用LOG常量
- **待完成**:
  - [ ] 更新ai-core.js中所有console.log使用LOG常量
  - [ ] 添加AI特定的LOG常量
  - [ ] 测试所有日志输出

## 详细任务

### ai-core.js日志迁移

需要将以下中文日志改为LOG常量：

```javascript
// 当前（中文硬编码）
console.log('[PIXLY AI] 🔍 开始检测Rust核心...');
console.log('[PIXLY AI] 尝试路径:', paths);
console.log('[PIXLY AI] ❌ 路径失败:', path, error);
console.log('[PIXLY AI] ⚠️ 未找到Rust核心');
console.log('[PIXLY AI] 💡 请确保已编译Rust核心');

// 应该改为（LOG常量）
logger.info('PIXLY AI', LOG.RUST_CORE_DETECTION_START);
logger.info('PIXLY AI', LOG.RUST_CORE_TRYING_PATHS, { count: paths.length });
logger.error('PIXLY AI', LOG.RUST_CORE_PATH_FAILED, { path }, error);
logger.warn('PIXLY AI', LOG.RUST_CORE_NOT_FOUND);
logger.info('PIXLY AI', LOG.RUST_CORE_INSTALL_HINT);
```

### 需要添加的LOG常量

```javascript
// AI特定的LOG常量
const LOG = {
    // ... 现有的常量 ...
    
    // AI功能
    AI_PRESET_CHANGED: 'AI preset changed to: {preset}',
    AI_FEATURE_TOGGLED: 'AI feature {feature} toggled: {enabled}',
    AI_PREDICTION_START: 'Starting AI prediction',
    AI_PREDICTION_COMPLETE: 'AI prediction complete',
    
    // 媒体分类
    MEDIA_CLASSIFICATION_START: 'Classifying media files',
    MEDIA_CLASSIFIED: 'Classified: {images} images, {animations} animations, {videos} videos',
    
    // 智能推荐
    SMART_QUALITY_ENABLED: 'Smart quality enabled',
    FORMAT_RECOMMEND_ENABLED: 'Format recommendation enabled',
    SSIM_VALIDATION_ENABLED: 'SSIM validation enabled',
};
```

## 快速修复方案

如果时间紧迫，可以：

1. **保持现状** - AI插件功能正常，只是日志是中文
2. **渐进式迁移** - 每次修改AI插件时顺便迁移日志
3. **批量替换** - 使用脚本批量替换（风险较高）

## 测试清单

- [ ] AI插件能找到Rust核心
- [ ] AI插件能正常转换
- [ ] 日志全部英文
- [ ] 日志使用LOG常量
- [ ] 国际化正常工作

## 优先级

1. **P0**: Rust核心路径 ✅ 已完成
2. **P1**: 基本功能测试
3. **P2**: 日志系统迁移
4. **P3**: 代码清理和优化

---

**当前状态**: AI插件可以工作，但日志需要迁移  
**建议**: 先确保功能正常，日志迁移可以后续进行
