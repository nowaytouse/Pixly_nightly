<template>
  <div class="panel file-panel">
    <div class="panel-title">
      <span>📁</span>
      <span>{{ t('ui.fileList') }} ({{ files.length }})</span>
    </div>
    
    <div v-if="files.length === 0" class="empty-state">
      <div class="empty-icon">📂</div>
      <p>{{ t('ui.selectInEagle') }}</p>
      <p class="empty-hint">{{ t('ui.clickRefresh') }}</p>
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
          @error="(e) => { e.target.style.display = 'none'; e.target.nextElementSibling.style.display = 'flex' }"
        />
        <div class="file-thumbnail-placeholder" :style="{ display: file.thumbnail ? 'none' : 'flex' }">
          <span class="file-emoji">{{ getFileEmoji(file) }}</span>
          <span class="file-ext-badge">{{ (file.ext || '').toUpperCase() }}</span>
        </div>
        <div class="file-info">
          <div class="file-name">{{ file.name }}</div>
          <div class="file-meta">{{ formatFileSize(file.size) }} · {{ file.ext || 'unknown' }}</div>
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

// 根据文件类型返回对应的 emoji
const getFileEmoji = (file) => {
  const ext = (file.ext || '').toLowerCase()
  
  // XMP元数据文件（特殊处理）
  if (ext === 'xmp') {
    return '📎'
  }
  
  // 图像格式
  const imageFormats = {
    'jxl': '🎨',
    'avif': '🖼️',
    'webp': '🌐',
    'heic': '🍎',
    'heif': '🍎',
    'png': '🖼️',
    'jpg': '📷',
    'jpeg': '📷',
    'gif': '🎞️',
    'bmp': '🖼️',
    'tiff': '🖼️',
    'svg': '🎨'
  }
  
  // 视频格式
  const videoFormats = {
    'mp4': '🎬',
    'mov': '🎥',
    'avi': '📹',
    'mkv': '🎞️',
    'webm': '🌐',
    'm4v': '📱',
    'flv': '📺',
    'wmv': '🎬'
  }
  
  // 音频格式
  const audioFormats = {
    'mp3': '🎵',
    'wav': '🎶',
    'flac': '🎼',
    'aac': '🎧',
    'm4a': '🎵',
    'ogg': '🎶'
  }
  
  // 返回对应的 emoji
  if (imageFormats[ext]) return imageFormats[ext]
  if (videoFormats[ext]) return videoFormats[ext]
  if (audioFormats[ext]) return audioFormats[ext]
  
  // 默认文件图标
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
  gap: 16px;
  color: var(--text-tertiary);
  animation: fadeIn var(--transition-slow) var(--ease-out);
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
  animation: float 3s ease-in-out infinite;
}

.empty-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  margin-top: 8px;
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

.file-thumbnail-placeholder {
  width: 48px;
  height: 48px;
  border-radius: 6px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--bg-secondary) 0%, var(--bg-tertiary) 100%);
  position: relative;
  overflow: hidden;
  transition: transform var(--transition-base) var(--ease-out);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.file-item:hover .file-thumbnail-placeholder {
  transform: scale(1.05);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.file-emoji {
  font-size: 32px;
  line-height: 1;
  transition: transform var(--transition-base) var(--ease-bounce);
}

.file-item:hover .file-emoji {
  transform: scale(1.1);
}

.file-ext-badge {
  position: absolute;
  bottom: 2px;
  right: 2px;
  background: var(--color-primary);
  color: white;
  font-size: 8px;
  font-weight: 700;
  padding: 1px 3px;
  border-radius: 2px;
  line-height: 1;
  text-transform: uppercase;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
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
