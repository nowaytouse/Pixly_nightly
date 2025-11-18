<template>
  <div class="panel">
    <div class="panel-title">
      <span>📦</span>
      <span>输出格式</span>
    </div>
    
    <select 
      :value="modelValue"
      @change="$emit('update:modelValue', $event.target.value)"
      class="format-select"
    >
      <option 
        v-for="format in formats" 
        :key="format.value"
        :value="format.value"
      >
        {{ format.icon }} {{ format.name }} - {{ format.desc }}
      </option>
    </select>
  </div>
</template>

<script setup>
defineProps({
  modelValue: String
})

defineEmits(['update:modelValue'])

const formats = [
  { value: 'jxl', name: 'JXL', icon: '✨', desc: '次世代格式' },
  { value: 'avif', name: 'AVIF', icon: '🎬', desc: 'AV1编码' },
  { value: 'webp', name: 'WebP', icon: '🌐', desc: 'Web优化' },
  { value: 'heic', name: 'HEIC', icon: '🍎', desc: 'Apple生态' }
]
</script>

<style scoped>
.format-select {
  width: 100%;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.format-select:hover {
  border-color: var(--color-primary);
}

.format-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(0, 114, 239, 0.1);
}
</style>
