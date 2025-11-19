<template>
  <div class="panel">
    <div class="panel-title">
      <span>🎯</span>
      <span>{{ t('quality.title') }}</span>
      <label class="lossless-toggle">
        <input 
          type="checkbox" 
          :checked="lossless"
          @change="$emit('update:lossless', $event.target.checked)"
        />
        <span>{{ t('quality.lossless') }}</span>
      </label>
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
        :disabled="lossless"
      />
      <div class="quality-hint">
        {{ qualityHint }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

const props = defineProps({
  modelValue: Number,
  lossless: Boolean
})

defineEmits(['update:modelValue', 'update:lossless'])

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
  transition: color var(--transition-fast) var(--ease-out);
}

.quality-value {
  font-weight: 700;
  color: var(--color-primary);
  font-size: 16px;
  transition: all var(--transition-base) var(--ease-bounce);
  transform: translateZ(0);
}

.quality-value:hover {
  transform: scale(1.05);
}

.slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--bg-tertiary);
  outline: none;
  -webkit-appearance: none;
  transition: background var(--transition-fast) var(--ease-out);
}

.slider:hover:not(:disabled) {
  background: #353535;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  transition: all var(--transition-base) var(--ease-bounce);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.slider::-webkit-slider-thumb:hover {
  transform: scale(1.25);
  box-shadow: 0 4px 8px rgba(0, 114, 239, 0.5);
}

.slider::-webkit-slider-thumb:active {
  transform: scale(1.15);
}

.slider::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--color-primary);
  cursor: pointer;
  border: none;
  transition: all var(--transition-base) var(--ease-bounce);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.slider::-moz-range-thumb:hover {
  transform: scale(1.25);
  box-shadow: 0 4px 8px rgba(0, 114, 239, 0.5);
}

.slider::-moz-range-thumb:active {
  transform: scale(1.15);
}

.quality-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
  transition: color var(--transition-fast) var(--ease-out);
  animation: fadeIn var(--transition-base) var(--ease-out);
}

.lossless-toggle {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all var(--transition-fast) var(--ease-out);
}

.lossless-toggle:hover {
  color: var(--text-primary);
  background: var(--bg-tertiary);
}

.lossless-toggle input[type="checkbox"] {
  width: 14px;
  height: 14px;
  cursor: pointer;
}

.slider:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  filter: grayscale(0.5);
}

/* AI模式样式 */
.ai-mode-toggle {
  padding: 12px;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(168, 85, 247, 0.1));
  border-radius: 8px;
  margin-bottom: 12px;
  border: 1px solid rgba(99, 102, 241, 0.2);
  transition: all var(--transition-base) var(--ease-out);
}

.ai-mode-toggle:hover {
  border-color: rgba(99, 102, 241, 0.4);
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.15), rgba(168, 85, 247, 0.15));
}

.ai-toggle-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  cursor: pointer;
  user-select: none;
}

.ai-icon {
  font-size: 16px;
  animation: pulse 2s ease-in-out infinite;
}

.ai-hint {
  display: block;
  margin-top: 8px;
  font-size: 11px;
  color: var(--text-tertiary);
  font-style: italic;
}

.quality-control.ai-disabled {
  opacity: 0.6;
  pointer-events: none;
}

/* 动画效果 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-5px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}
</style>
