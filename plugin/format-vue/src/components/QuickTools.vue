<template>
  <div class="panel">
    <!-- 标签切换 -->
    <div class="panel-tabs">
      <button 
        class="panel-tab" 
        :class="{ active: activeTab === 'tools' }"
        @click="activeTab = 'tools'"
      >
        {{ t('log.features') }}
      </button>
      <button 
        class="panel-tab" 
        :class="{ active: activeTab === 'log' }"
        @click="activeTab = 'log'"
      >
        {{ t('log.aiLog') }}
      </button>
    </div>
    
    <!-- 快捷工具面板 -->
    <div v-if="activeTab === 'tools'" class="panel-content">
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
    
    <!-- 日志窗口 -->
    <LogWindow v-else-if="activeTab === 'log'" :logs="logs" ref="logWindow" />
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useI18n } from '../composables/useI18n'
import LogWindow from './LogWindow.vue'

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
  },
  logs: {
    type: Array,
    default: () => []
  }
})

const emit = defineEmits(['update:modelValue'])

const activeTab = ref('tools') // 'tools' or 'log'
const localTools = ref({ ...props.modelValue })
const logWindow = ref(null)

// 暴露方法供父组件调用
defineExpose({
  switchToLog: () => {
    activeTab.value = 'log'
  },
  scrollLogToBottom: () => {
    if (logWindow.value) {
      logWindow.value.scrollToBottom()
    }
  }
})

watch(localTools, (newVal) => {
  emit('update:modelValue', newVal)
}, { deep: true })

watch(() => props.modelValue, (newVal) => {
  localTools.value = { ...newVal }
}, { deep: true })
</script>

<style scoped>
/* 标签切换 */
.panel-tabs {
  display: flex;
  background: var(--bg-primary);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px 8px 0 0;
  overflow: hidden;
}

.panel-tab {
  flex: 1;
  padding: 10px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border-bottom: 2px solid transparent;
}

.panel-tab:hover {
  background: rgba(255, 255, 255, 0.05);
}

.panel-tab.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
  background: rgba(255, 255, 255, 0.03);
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
