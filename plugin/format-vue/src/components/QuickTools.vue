<template>
  <div class="panel">
    <div class="panel-header" @click="expanded = !expanded">
      <div class="panel-title">
        <span>🛠️</span>
        <span>{{ t('tools.title') }}</span>
      </div>
      <span class="expand-icon">{{ expanded ? t('ui.collapseIcon') : t('ui.expandIcon') }}</span>
    </div>
    
    <div v-if="expanded" class="panel-content">
      <!-- AI 文件验证 -->
      <label class="tool-item" :title="t('tools.fileValidationHint')">
        <input type="checkbox" v-model="localTools.fileValidation" />
        <div class="tool-info">
          <div class="tool-name">
            {{ t('tools.fileValidation') }}
            <span class="badge-experimental">{{ t('common.experimental') }}</span>
          </div>
          <div class="tool-desc">{{ t('tools.fileValidationDesc') }}</div>
        </div>
      </label>

      <!-- 格式修正 -->
      <label class="tool-item" :title="t('tools.formatCorrectionHint')">
        <input type="checkbox" v-model="localTools.formatCorrection" />
        <div class="tool-info">
          <div class="tool-name">
            {{ t('tools.formatCorrection') }}
            <span class="badge-experimental">{{ t('common.experimental') }}</span>
          </div>
          <div class="tool-desc">{{ t('tools.formatCorrectionDesc') }}</div>
        </div>
      </label>

      <!-- 自动合并 XMP -->
      <label class="tool-item" :title="t('tools.autoMergeXmpHint')">
        <input type="checkbox" v-model="localTools.autoMergeXmp" />
        <div class="tool-info">
          <div class="tool-name">{{ t('tools.autoMergeXmp') }}</div>
          <div class="tool-desc">{{ t('tools.autoMergeXmpHint') }}</div>
        </div>
      </label>

      <!-- 规范化文件名 -->
      <label class="tool-item" :title="t('tools.normalizeFilenamesHint')">
        <input type="checkbox" v-model="localTools.normalizeFilenames" />
        <div class="tool-info">
          <div class="tool-name">{{ t('tools.normalizeFilenames') }}</div>
          <div class="tool-desc">{{ t('tools.normalizeFilenamesHint') }}</div>
        </div>
      </label>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

const props = defineProps({
  modelValue: {
    type: Object,
    default: () => ({
      fileValidation: true,
      formatCorrection: true,
      autoMergeXmp: true,
      normalizeFilenames: false
    })
  }
})

const emit = defineEmits(['update:modelValue'])

const expanded = ref(true)
const localTools = ref({ ...props.modelValue })

watch(localTools, (newVal) => {
  emit('update:modelValue', newVal)
}, { deep: true })

watch(() => props.modelValue, (newVal) => {
  localTools.value = { ...newVal }
}, { deep: true })
</script>

<style scoped>
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  user-select: none;
  padding: 12px;
  border-radius: 8px;
  transition: background var(--transition-fast);
}

.panel-header:hover {
  background: rgba(255, 255, 255, 0.03);
}

.expand-icon {
  font-size: 12px;
  color: var(--text-secondary);
  transition: transform var(--transition-fast);
}

.panel-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px 12px 12px;
}

.tool-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px;
  border-radius: 6px;
  cursor: pointer;
  transition: all var(--transition-fast);
  background: var(--bg-primary);
}

.tool-item:hover {
  background: rgba(255, 255, 255, 0.05);
}

.tool-item input[type="checkbox"] {
  margin-top: 2px;
  width: 16px;
  height: 16px;
  cursor: pointer;
  flex-shrink: 0;
}

.tool-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.tool-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.tool-desc {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.badge-experimental {
  display: inline-block;
  padding: 2px 6px;
  background: rgba(255, 165, 0, 0.2);
  color: #ffa500;
  font-size: 9px;
  font-weight: 700;
  border-radius: 3px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
