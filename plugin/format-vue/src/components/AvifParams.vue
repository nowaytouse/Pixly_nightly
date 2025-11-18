<template>
  <div class="params-group">
    <!-- Speed滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Speed</span>
        <span class="slider-value">{{ localParams.speed }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.speed"
        min="0" 
        max="10" 
        step="1"
        class="slider"
      />
      <div class="slider-hint">0-3: 极致质量 | 4-6: 平衡 | 7-10: 快速</div>
    </div>
    
    <!-- Min Quantizer -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Min Quantizer</span>
        <span class="slider-value">{{ localParams.minQuantizer }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.minQuantizer"
        min="0" 
        max="63" 
        step="1"
        class="slider"
      />
    </div>
    
    <!-- Max Quantizer -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Max Quantizer</span>
        <span class="slider-value">{{ localParams.maxQuantizer }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.maxQuantizer"
        min="0" 
        max="63" 
        step="1"
        class="slider"
      />
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
  speed: 6,
  minQuantizer: 0,
  maxQuantizer: 63,
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
</style>
