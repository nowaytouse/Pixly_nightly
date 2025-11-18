<template>
  <div class="panel file-panel">
    <div class="panel-title">
      <span>📁</span>
      <span>{{ t('ui.fileList') }} ({{ files.length }})</span>
    </div>
    
    <div v-if="files.length === 0" class="empty-state">
      <div class="empty-icon">📂</div>
      <p>{{ t('ui.selectInEagle') }}</p>
    </div>
    
    <div v-else class="file-list">
      <div 
        v-for="(file, index) in files" 
        :key="file.id"
        class="file-item"
      >
        <span class="file-icon">{{ getFileIcon(file) }}</span>
        <div class="file-info">
          <div class="file-name">{{ file.name }}</div>
          <div class="file-meta">{{ formatFileSize(file.size) }} · {{ file.ext }}</div>
        </div>
        <button class="remove-btn" @click="$emit('remove', index)">
          ✕
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useI18n } from '../composables/useI18n'
import { formatFileSize, getFileType } from '../utils/fileTypes'

const { t } = useI18n()

defineProps({
  files: Array
})

defineEmits(['remove'])

const getFileIcon = (file) => {
  const type = getFileType(file.name)
  if (type === 'image') return '🖼️'
  if (type === 'video') return '🎬'
  return '📄'
}
</script>

<style scoped>
.file-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-tertiary);
  animation: fadeIn var(--transition-slow) var(--ease-out);
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.file-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
  /* 流畅滚动 */
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-tertiary);
  border-radius: 6px;
  border: 1px solid transparent;
  transition: all var(--transition-base) var(--ease-out);
  animation: slideInRight var(--transition-base) var(--ease-out);
  /* 性能优化 */
  transform: translateZ(0);
  backface-visibility: hidden;
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.file-item:hover {
  background: var(--bg-primary);
  border-color: var(--border-color-hover);
  transform: translateX(-2px);
  box-shadow: var(--shadow-sm);
}

.file-icon {
  font-size: 24px;
  flex-shrink: 0;
  transition: transform var(--transition-base) var(--ease-bounce);
}

.file-item:hover .file-icon {
  transform: scale(1.1) rotate(5deg);
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color var(--transition-fast) var(--ease-out);
}

.file-item:hover .file-name {
  color: var(--color-primary);
}

.file-meta {
  font-size: 11px;
  color: var(--text-tertiary);
  transition: color var(--transition-fast) var(--ease-out);
}

.remove-btn {
  width: 24px;
  height: 24px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: var(--text-tertiary);
  transition: all 0.3s;
  flex-shrink: 0;
}

.remove-btn:hover {
  background: var(--danger-color);
  border-color: var(--danger-color);
  color: white;
}
</style>
