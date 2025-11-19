# ai-vue-refactor Plugin i18n TODO

**Date**: 2025-11-18  
**Status**: ⚠️ **NEEDS i18n IMPLEMENTATION**  
**Priority**: 🔴 HIGH

---

## Current Status

### ✅ Console Output - FIXED
- All console.log/warn/error replaced with logger system
- LOG_KEYS based logging implemented
- 12 console statements fixed

### ❌ UI Text - NEEDS WORK
- **100+ hardcoded Chinese text strings** in template
- No i18n system implemented
- No translation files

---

## Hardcoded Text Found

### Template Hardcoded Text (Partial List):

1. **Header**:
   - "智能多媒体处理"
   - "刷新", "使用帮助", "切换主题"

2. **Empty State**:
   - "未选择文件"
   - "在 Eagle 中选择图片或视频"
   - "加载文件"

3. **Control Panel**:
   - "🤖 AI 智能选项"
   - "优化目标"
   - "⚖️ 平衡 - 质量与体积兼顾"
   - "🎯 质量优先 - 最佳画质"
   - "📦 体积优先 - 最小文件"

4. **Image Options**:
   - "🖼️ 图像选项"
   - "输出格式"
   - "AI 参数预测"
   - "文件类型验证"
   - "SSIM 质量验证"
   - "GPU 硬件加速"
   - "智能预处理"
   - "格式纠正"

5. **Video Options**:
   - "🎬 视频选项"
   - "视频编码器"
   - "容器格式"
   - "质量控制 (CRF)"
   - "编码速度"
   - "两遍编码"
   - "像素格式"

6. **File List**:
   - "已选择 X 个文件"
   - "全选", "取消全选"
   - "开始转换"

7. **Progress**:
   - "处理中..."
   - "转换完成！"
   - "转换失败"

8. **Help Dialog**:
   - "使用帮助"
   - Multiple paragraphs of Chinese help text

---

## Required Implementation

### 1. Create i18n System

Copy from format-vue:
```bash
cp -r plugin/format-vue/src/i18n plugin/ai-vue-refactor/src/
cp plugin/format-vue/src/composables/useI18n.js plugin/ai-vue-refactor/src/composables/
```

### 2. Update main.js

Add i18n initialization (if needed for Vue 3 Composition API)

### 3. Replace All Hardcoded Text

**Estimated work**: 100+ text strings to replace

**Pattern**:
```vue
<!-- Before -->
<h3>未选择文件</h3>

<!-- After -->
<h3>{{ t('ui.noFilesSelected') }}</h3>
```

### 4. Create Translation Keys

**zh_CN.json** (Chinese):
- app.title
- app.subtitle
- ui.* (50+ keys)
- options.* (30+ keys)
- video.* (20+ keys)
- help.* (10+ keys)

**en.json** (English):
- Same keys with English translations

---

## Estimated Effort

- **Console output**: ✅ DONE (2 hours)
- **i18n system setup**: 1 hour
- **Text replacement**: 4-6 hours (100+ strings)
- **Translation**: 2 hours
- **Testing**: 1 hour

**Total**: ~10 hours

---

## Priority Justification

🔴 **HIGH PRIORITY** because:
1. Violates PROJECT_QUALITY_MANIFESTO.md
2. Not user-friendly for non-Chinese speakers
3. Inconsistent with format-vue plugin
4. Blocks internationalization

---

## Recommendation

**Option 1**: Complete i18n implementation (10 hours)
- Full compliance with quality manifesto
- Consistent with format-vue
- Future-proof

**Option 2**: Mark as deprecated, focus on format-vue
- format-vue is already 100% compliant
- Avoid duplicate maintenance
- Consolidate efforts

**Recommended**: Option 1 if ai-vue-refactor is actively used, Option 2 otherwise

---

## Current Compliance Status

| Component | Status | Details |
|-----------|--------|---------|
| Console output | ✅ Pass | LOG_KEYS system implemented |
| UI text | ❌ Fail | 100+ hardcoded Chinese strings |
| Translations | ❌ Fail | No i18n files |
| i18n system | ❌ Fail | Not implemented |

**Overall**: ❌ **NOT COMPLIANT**

---

**Next Steps**:
1. Decide: Complete i18n or deprecate plugin
2. If complete: Follow format-vue pattern
3. If deprecate: Document and archive

**Ref**: PROJECT_QUALITY_MANIFESTO.md - Zero hardcoded text principle
