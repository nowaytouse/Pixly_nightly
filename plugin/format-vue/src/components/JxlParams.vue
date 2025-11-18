<template>
  <div class="params-group">
    <!-- Effort滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>{{ t('advanced.jxl.effort') }}</span>
        <span class="slider-value">{{ localParams.effort }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.effort"
        min="1" 
        max="9" 
        step="1"
        class="slider"
      />
      <div class="slider-hint">{{ t('advanced.jxl.effortHint') }}</div>
    </div>
    
    <!-- Distance滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>{{ t('advanced.jxl.distance') }}</span>
        <span class="slider-value">{{ localParams.distance.toFixed(1) }}</span>
      </div>
      <input 
        type="range" 
        v-model.number="localParams.distance"
        min="0" 
        max="15" 
        step="0.1"
        class="slider"
      />
      <div class="slider-hint">{{ t('advanced.jxl.distanceHint') }}</div>
    </div>
    
    <!-- JPEG无损转码 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.jpegLossless" />
        <span>{{ t('advanced.jxl.jpegLossless') }}</span>
      </label>
      <div class="checkbox-hint">{{ t('advanced.jxl.jpegLosslessHint') }}</div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

const props = defineProps({
  modelValue: Object
})

const emit = defineEmits(['update:modelValue'])

const localParams = ref({
  effort: 7,
  distance: 1.0,
  jpegLossless: false,
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
  gap: 8px;
}

.slider-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  color: var(--text-primary);
}

.slider-value {
  font-weight: 700;
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
}

.slider::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  border: none;
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
}

.checkbox-hint {
  font-size: 11px;
  color: var(--text-tertiary);
  padding-left: 24px;
}
</style>
