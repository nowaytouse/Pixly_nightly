<template>
  <div class="video-panel">
    <!-- 容器格式 -->
    <div class="panel">
      <div class="panel-title">
        <span>📦</span>
        <span>容器格式</span>
      </div>
      <select v-model="localParams.container" class="param-select">
        <option value="mp4">MP4 - 最广泛兼容</option>
        <option value="mov">MOV - Apple优化</option>
        <option value="webm">WebM - Web优化</option>
        <option value="mkv">MKV - 通用容器</option>
      </select>
    </div>

    <!-- 编码器 -->
    <div class="panel">
      <div class="panel-title">
        <span>🎬</span>
        <span>视频编码器</span>
      </div>
      <select v-model="localParams.codec" class="param-select">
        <option value="h264">H.264 - 通用性最好</option>
        <option value="h265">H.265 - 更高压缩率</option>
        <option value="av1">AV1 - 次世代</option>
        <option value="vp9">VP9 - Web优化</option>
      </select>
    </div>

    <!-- CRF质量 -->
    <div class="panel">
      <div class="panel-title">
        <span>🎯</span>
        <span>质量控制</span>
      </div>
      <div class="slider-group">
        <div class="slider-header">
          <span>CRF</span>
          <span class="slider-value">{{ localParams.crf }}</span>
        </div>
        <input 
          type="range" 
          v-model.number="localParams.crf"
          min="0" 
          max="51" 
          step="1"
          class="slider"
        />
        <div class="slider-hint">0=无损 | 18=视觉无损 | 23=高质量 | 28=平衡</div>
      </div>
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
  container: 'mp4',
  codec: 'h265',
  crf: 23,
  ...props.modelValue
})

watch(localParams, (newVal) => {
  emit('update:modelValue', newVal)
}, { deep: true })
</script>

<style scoped>
.video-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.param-select {
  width: 100%;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
}

.param-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(0, 114, 239, 0.1);
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

.slider-hint {
  font-size: 11px;
  color: var(--text-tertiary);
}
</style>
