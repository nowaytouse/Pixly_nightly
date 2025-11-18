# 归档文件说明

本目录存放已归档的代码和文档。

---

## incomplete_plugins/

### ai-vue_20251118

**归档原因**: 半成品插件，功能与format-vue重复

**详细说明**:
- 缺少所有UI层（App.vue、组件等）
- composables与format-vue完全重复
- 无法被Eagle导入（缺少manifest.json）
- 违反PROJECT_QUALITY_MANIFESTO.md的"反对重复造轮子"原则

**归档日期**: 2025-11-18

**参考文档**: `AI_VUE_PLUGIN_INVESTIGATION.md`

**替代方案**: 使用`plugin/format-vue`（功能完整）
