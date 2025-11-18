<template>
  <div class="params-group">
    <!-- Effort滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Effort</span>
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
      <div class="slider-hint">1-3: 快速 | 4-6: 平衡 | 7-9: 极致</div>
    </div>
    
    <!-- Distance滑条 -->
    <div class="slider-group">
      <div class="slider-header">
        <span>Distance</span>
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
      <div class="slider-hint">0.0: 无损 | 1.0-3.0: 高质量 | 3.0+: 压缩</div>
    </div>
    
    <!-- JPEG无损转码 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.jpegLossless" />
        <span>JPEG 无损转码</span>
      </label>
      <div class="checkbox-hint">启用后将JPEG 100%可逆转码为JXL</div>
    </div>
    
    <!-- 无损模式 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.lossless" />
        <span>数学无损</span>
      </label>
      <div class="checkbox-hint">完全无损压缩（非JPEG）</div>
    </div>

    <!-- 色彩位深度 -->
    <div class="select-group">
      <label class="select-label">色彩位深度</label>
      <select v-model="localParams.bitDepth" class="param-select">
        <option value="auto">自动（根据源文件）</option>
        <option value="8">8-bit（标准）</option>
        <option value="10">10-bit（HDR）</option>
        <option value="12">12-bit（专业）</option>
        <option value="16">16-bit（极致）</option>
      </select>
    </div>

    <!-- 色彩空间 -->
    <div class="select-group">
      <label class="select-label">色彩空间</label>
      <select v-model="localParams.colorSpace" class="param-select">
        <option value="auto">自动（保持源色彩空间）</option>
        <option value="srgb">sRGB（标准）</option>
        <option value="p3">Display P3（广色域）</option>
        <option value="adobe">Adobe RGB（专业）</option>
        <option value="prophoto">ProPhoto RGB（极致）</option>
      </select>
    </div>

    <!-- 高级选项 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.modular" />
        <span>Modular 模式</span>
      </label>
      <div class="checkbox-hint">适合无损和高质量场景</div>
    </div>

    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.progressive" />
        <span>渐进式加载</span>
      </label>
      <div class="checkbox-hint">支持逐步显示</div>
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
  effort: 7,
  distance: 1.0,
  lossless: false,
  jpegLossless: false,
  bitDepth: 'auto',
  colorSpace: 'auto',
  modular: false,
  progressive: false,
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

.checkbox-hint {
  font-size: 11px;
  color: var(--text-tertiary);
  padding-left: 24px;
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
