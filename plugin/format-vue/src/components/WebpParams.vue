<template>
  <div class="params-group">
    <!-- Method滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Method</span>
        <span class="slider-value">{{ localParams.method }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.method"
        min="0" 
        max="6" 
        step="1"
        class="slider"
      />
      <div class="slider-hint">0-2: 快速 | 3-4: 平衡 | 5-6: 最佳</div>
    </div>
    
    <!-- 无损模式 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.lossless" />
        <span>无损模式</span>
      </label>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  modelValue: Object
})

const emit = defineEmits(['update:modelValue'])

const localParams = ref({
  method: 4,
  lossless: false,
  ...props.modelValue
})

watch(localParams, (newVal) => {
  emit('update:modelValue', newVal)
}, { deep: true })
</script>

<style scoped>
.params-group {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.slider-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.slider-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  color: var(--text-primary);
}

.slider-value {
  font-weight: 600;
  color: var(--color-primary);
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
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.2);
}

.slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  border: none;
  transition: all 0.2s;
}

.slider-hint {
  font-size: 11px;
  color: var(--text-tertiary);
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: var(--color-primary);
}
</style>
