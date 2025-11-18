<template>
  <div class="panel file-panel">
    <div class="panel-title">
      <span>📁</span>
      <span>{{ t('ui.fileList') }} ({{ files.length }})</span>
      <button class="refresh-btn" @click="$emit('refresh')" title="刷新文件列表">
        🔄
      </button>
    </div>
    
    <div v-if="files.length === 0" class="empty-state">
      <div class="empty-icon">📂</div>
      <p>{{ t('ui.selectInEagle') }}</p>
      <button class="load-btn" @click="$emit('refresh')">
        {{ t('ui.loadFiles') || '加载文件' }}
      </button>
    </div>
    
    <div v-else class="file-list">
      <div 
        v-for="(file, index) in files" 
        :key="file.id"
        class="file-item"
      >
        <img 
          v-if="file.thumbnail" 
          :src="file.thumbnail" 
          class="file-thumbnail"
          :alt="file.name"
          @error="handleImageError"
        />
        <span v-else class="file-icon">{{ getFileIcon(file) }}</span>
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

defineEmits(['remove', 'refresh'])

const getFileIcon = (file) => {
  const type = getFileType(file.name)
  if (type === 'image') return '🖼️'
  if (type === 'video') return '🎬'
  return '📄'
}

const handleImageError = (event) => {
  // 缩略图加载失败时隐藏图片
  event.target.style.display = 'none'
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
  gap: 16px;
  color: var(--text-tertiary);
  animation: fadeIn var(--transition-slow) var(--ease-out);
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
  animation: float 3s ease-in-out infinite;
}

.load-btn {
  padding: 10px 20px;
  background: var(--color-primary);
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-base) var(--ease-out);
}

.load-btn:hover {
  background: var(--color-primary-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.refresh-btn {
  margin-left: auto;
  width: 24px;
  height: 24px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base) var(--ease-out);
}

.refresh-btn:hover {
  background: var(--bg-tertiary);
  border-color: var(--color-primary);
  color: var(--color-primary);
  transform: rotate(180deg);
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

.file-thumbnail {
  width: 48px;
  height: 48px;
  object-fit: cover;
  border-radius: 6px;
  flex-shrink: 0;
  transition: transform var(--transition-base) var(--ease-out);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.file-item:hover .file-thumbnail {
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

.file-icon {
  font-size: 24px;
  flex-shrink: 0;
  transition: transform var(--transition-base) var(--ease-bounce);
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  border-radius: 6px;
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
