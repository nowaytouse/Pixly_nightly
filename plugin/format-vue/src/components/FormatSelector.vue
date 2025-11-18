<template>
  <div class="panel">
    <div class="panel-title">
      <span>📦</span>
      <span>输出格式</span>
    </div>
    
    <div class="format-grid">
      <label 
        v-for="format in formats" 
        :key="format.value"
        class="format-card"
        :class="{ active: modelValue === format.value }"
      >
        <input 
          type="radio" 
          :value="format.value" 
          :checked="modelValue === format.value"
          @change="$emit('update:modelValue', format.value)"
        />
        <span class="format-icon">{{ format.icon }}</span>
        <span class="format-name">{{ format.name }}</span>
      </label>
    </div>
  </div>
</template>

<script setup>
defineProps({
  modelValue: String
})

defineEmits(['update:modelValue'])

const formats = [
  { value: 'jxl', name: 'JXL', icon: '✨' },
  { value: 'avif', name: 'AVIF', icon: '🎬' },
  { value: 'webp', name: 'WebP', icon: '🌐' },
  { value: 'heic', name: 'HEIC', icon: '🍎' }
]
</script>

<style scoped>
.format-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}

.format-card {
  position: relative;
  padding: 16px;
  background: var(--bg-tertiary);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  min-height: 80px;
  justify-content: center;
}

.format-card input {
  position: absolute;
  opacity: 0;
}

.format-card:hover {
  border-color: var(--primary-color);
  transform: translateY(-2px);
}

.format-card.active {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.format-icon {
  font-size: 28px;
}

.format-name {
  font-size: 14px;
  font-weight: 700;
}
</style>
