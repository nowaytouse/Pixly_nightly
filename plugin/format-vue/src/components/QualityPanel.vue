<template>
  <div class="panel">
    <div class="panel-title">
      <span>🎯</span>
      <span>质量设置</span>
    </div>
    
    <div class="quality-control">
      <div class="quality-header">
        <span>质量</span>
        <span class="quality-value">{{ modelValue }}</span>
      </div>
      <input 
        type="range" 
        :value="modelValue"
        @input="$emit('update:modelValue', Number($event.target.value))"
        min="1" 
        max="100" 
        class="slider"
      />
      <div class="quality-hint">{{ qualityHint }}</div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  modelValue: Number
})

defineEmits(['update:modelValue'])

const qualityHint = computed(() => {
  const q = props.modelValue
  if (q >= 95) return '🔥 极致质量'
  if (q >= 85) return '✨ 高质量'
  if (q >= 70) return '⚖️ 平衡'
  if (q >= 50) return '📦 压缩优先'
  return '🗜️ 极限压缩'
})
</script>

<style scoped>
.quality-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.quality-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  color: var(--text-primary);
}

.quality-value {
  font-weight: 700;
  color: var(--primary-color);
  font-size: 16px;
}

.slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-tertiary);
  outline: none;
  -webkit-appearance: none;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--primary-color);
  cursor: pointer;
  transition: all 0.3s;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.quality-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
}
</style>
