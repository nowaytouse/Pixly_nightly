<template>
  <div class="panel">
    <div class="panel-title">
      <span>🎯</span>
      <span>{{ t('quality.title') }}</span>
    </div>
    
    <div class="quality-control">
      <div class="quality-header">
        <span>{{ t('quality.label') }}</span>
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
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

const props = defineProps({
  modelValue: Number
})

defineEmits(['update:modelValue'])

const qualityHint = computed(() => {
  const q = props.modelValue
  if (q >= 95) return t('quality.extreme')
  if (q >= 85) return t('quality.high')
  if (q >= 70) return t('quality.balanced')
  if (q >= 50) return t('quality.compressed')
  return t('quality.maximum')
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
  color: var(--color-primary);
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
  background: var(--color-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.slider::-moz-range-thumb:hover {
  transform: scale(1.2);
}

.quality-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
}
</style>
