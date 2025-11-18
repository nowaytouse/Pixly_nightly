<template>
  <div class="panel file-panel">
    <div class="panel-title">
      <span>📁</span>
      <span>文件列表 ({{ files.length }})</span>
    </div>
    
    <div v-if="files.length === 0" class="empty-state">
      <div class="empty-icon">📂</div>
      <p>在 Eagle 中选择文件</p>
    </div>
    
    <div v-else class="file-list">
      <div 
        v-for="(file, index) in files" 
        :key="file.id"
        class="file-item"
      >
        <span class="file-icon">📄</span>
        <div class="file-info">
          <div class="file-name">{{ file.name }}</div>
          <div class="file-meta">{{ formatSize(file.size) }} · {{ file.ext }}</div>
        </div>
        <button class="remove-btn" @click="$emit('remove', index)">
          ✕
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  files: Array
})

defineEmits(['remove'])

const formatSize = (bytes) => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
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
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

.file-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: var(--bg-tertiary);
  border-radius: 6px;
  transition: all 0.3s;
}

.file-item:hover {
  background: var(--bg-primary);
}

.file-icon {
  font-size: 24px;
  flex-shrink: 0;
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
}

.file-meta {
  font-size: 11px;
  color: var(--text-tertiary);
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
