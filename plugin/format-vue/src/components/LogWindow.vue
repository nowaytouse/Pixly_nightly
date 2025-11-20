<template>
  <div class="log-window">
    <div class="log-content" ref="logContent" @wheel.stop>
      <div v-for="(log, index) in logs" :key="index" class="log-entry" :class="log.type">
        <span class="log-time">{{ log.time }}</span>
        <span class="log-icon">{{ log.icon }}</span>
        <span class="log-message">{{ log.message }}</span>
      </div>
      <div v-if="logs.length === 0" class="log-empty">
        {{ t('log.waiting') }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

const props = defineProps({
  logs: {
    type: Array,
    default: () => []
  }
})

const logContent = ref(null)

// 暴露滚动到底部的方法
defineExpose({
  scrollToBottom: () => {
    if (logContent.value) {
      logContent.value.scrollTop = logContent.value.scrollHeight
    }
  }
})
</script>

<style scoped>
.log-window {
  height: 200px;
  overflow: hidden;
  isolation: isolate;
  position: relative;
}

.log-content {
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
  font-size: 11px;
  line-height: 1.6;
  background: #1a1a1a;
  color: #e0e0e0;
  scroll-behavior: smooth;
  overscroll-behavior: contain;
  -webkit-overflow-scrolling: touch;
  pointer-events: auto;
  position: relative;
  z-index: 1;
}

/* 自定义滚动条 */
.log-content::-webkit-scrollbar {
  width: 8px;
}

.log-content::-webkit-scrollbar-track {
  background: #0a0a0a;
}

.log-content::-webkit-scrollbar-thumb {
  background: #333;
  border-radius: 4px;
}

.log-content::-webkit-scrollbar-thumb:hover {
  background: #444;
}

.log-entry {
  display: flex;
  gap: 8px;
  padding: 2px 0;
  align-items: flex-start;
}

.log-time {
  color: #666;
  flex-shrink: 0;
}

.log-icon {
  flex-shrink: 0;
}

.log-message {
  flex: 1;
  word-break: break-word;
}

.log-entry.success .log-message {
  color: #4caf50;
}

.log-entry.warning .log-message {
  color: #ff9800;
}

.log-entry.error .log-message {
  color: #f44336;
}

.log-empty {
  color: #666;
  text-align: center;
  padding: 60px 20px;
  font-style: italic;
}
</style>
