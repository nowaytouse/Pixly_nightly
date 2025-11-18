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

    <!-- Tiles -->
    <div class="tiles-group">
      <label class="tiles-label">Tiles (行×列)</label>
      <div class="tiles-inputs">
        <input 
          type="number" 
          v-model.number="localParams.tilesRows"
          min="1" 
          max="8"
          class="tiles-input"
        />
        <span>×</span>
        <input 
          type="number" 
          v-model.number="localParams.tilesCols"
          min="1" 
          max="8"
          class="tiles-input"
        />
      </div>
      <div class="tiles-hint">分块并行编码，提升大图速度</div>
    </div>

    <!-- 色度子采样 -->
    <div class="select-group">
      <label class="select-label">色度子采样</label>
      <select v-model="localParams.chromaSubsampling" class="param-select">
        <option value="auto">自动</option>
        <option value="420">4:2:0（标准）</option>
        <option value="422">4:2:2（平衡）</option>
        <option value="444">4:4:4（最佳）</option>
      </select>
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
  tilesRows: 1,
  tilesCols: 1,
  chromaSubsampling: 'auto',
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

.tiles-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tiles-label {
  font-size: 13px;
  color: var(--text-primary);
}

.tiles-inputs {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tiles-input {
  width: 60px;
  padding: 6px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  text-align: center;
}

.tiles-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.tiles-inputs span {
  color: var(--text-secondary);
  font-size: 14px;
}

.tiles-hint {
  font-size: 11px;
  color: var(--text-tertiary);
}

.select-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.select-label {
  font-size: 13px;
  color: var(--text-primary);
}

.param-select {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
}

.param-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(0, 114, 239, 0.1);
}
</style>
