<template>
  <div class="video-panel">
    <!-- 容器格式 -->
    <div class="panel">
      <div class="panel-title">
        <span>📦</span>
        <span>{{ t('video.container.label') }}</span>
      </div>
      <select v-model="localParams.container" class="param-select">
        <option value="mp4">{{ t('video.container.mp4') }}</option>
        <option value="mov">{{ t('video.container.mov') }}</option>
        <option value="webm">{{ t('video.container.webm') }}</option>
        <option value="mkv">{{ t('video.container.mkv') }}</option>
      </select>
    </div>

    <!-- 编码器 -->
    <div class="panel">
      <div class="panel-title">
        <span>🎬</span>
        <span>{{ t('video.codec.label') }}</span>
      </div>
      <select v-model="localParams.codec" class="param-select">
        <option value="h266">{{ t('video.codec.h266') }}</option>
        <option value="h265">{{ t('video.codec.h265') }}</option>
        <option value="av1">{{ t('video.codec.av1') }}</option>
        <option value="h264">{{ t('video.codec.h264') }}</option>
        <option value="vp9">{{ t('video.codec.vp9') }}</option>
      </select>
    </div>

    <!-- CRF质量 -->
    <div class="panel">
      <div class="panel-title">
        <span>🎯</span>
        <span>{{ t('video.crf') }}</span>
      </div>
      <div class="slider-group">
        <div class="slider-header">
          <span>{{ t('video.crfLabel') }}</span>
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
      </div>
    </div>

    <!-- 编码参数 -->
    <div class="panel">
      <div class="panel-title">
        <span>⚙️</span>
        <span>{{ t('ui.advancedParams') }}</span>
      </div>
      
      <!-- 编码速度 -->
      <div class="slider-group">
        <div class="slider-header">
          <span>{{ t('video.speed') }}</span>
          <span class="slider-value">{{ localParams.speed }}</span>
        </div>
        <input 
          type="range" 
          v-model.number="localParams.speed"
          min="0" 
          max="9" 
          step="1"
          class="slider"
        />
      </div>

      <!-- GOP大小 -->
      <div class="slider-group">
        <div class="slider-header">
          <span>{{ t('video.gopSize') }}</span>
          <span class="slider-value">{{ localParams.gopSize }}</span>
        </div>
        <input 
          type="range" 
          v-model.number="localParams.gopSize"
          min="1" 
          max="600" 
          step="1"
          class="slider"
        />
      </div>

      <!-- B帧数量 -->
      <div class="slider-group">
        <div class="slider-header">
          <span>{{ t('video.bframes') }}</span>
          <span class="slider-value">{{ localParams.bframes }}</span>
        </div>
        <input 
          type="range" 
          v-model.number="localParams.bframes"
          min="0" 
          max="16" 
          step="1"
          class="slider"
        />
      </div>

      <!-- 参考帧 -->
      <div class="slider-group">
        <div class="slider-header">
          <span>{{ t('video.refs') }}</span>
          <span class="slider-value">{{ localParams.refs }}</span>
        </div>
        <input 
          type="range" 
          v-model.number="localParams.refs"
          min="1" 
          max="16" 
          step="1"
          class="slider"
        />
      </div>
    </div>

    <!-- 高级选项 -->
    <div class="panel">
      <div class="panel-title">
        <span>🔧</span>
        <span>{{ t('ui.advancedParams') }}</span>
      </div>

      <!-- 像素格式 -->
      <div class="param-group">
        <label class="param-label">{{ t('video.pixelFormat.label') }}</label>
        <select v-model="localParams.pixelFormat" class="param-select">
          <option value="auto">{{ t('video.pixelFormat.auto') }}</option>
          <option value="yuv420p">{{ t('video.pixelFormat.yuv420p') }}</option>
          <option value="yuv422p">{{ t('video.pixelFormat.yuv422p') }}</option>
          <option value="yuv444p">{{ t('video.pixelFormat.yuv444p') }}</option>
        </select>
      </div>

      <!-- 硬件加速 -->
      <div class="param-group">
        <label class="param-label">{{ t('video.hwAccel.label') }}</label>
        <select v-model="localParams.hwAccel" class="param-select">
          <option value="auto">{{ t('video.hwAccel.auto') }}</option>
          <option value="none">{{ t('video.hwAccel.none') }}</option>
          <option value="nvenc">{{ t('video.hwAccel.nvenc') }}</option>
          <option value="qsv">{{ t('video.hwAccel.qsv') }}</option>
          <option value="videotoolbox">{{ t('video.hwAccel.videotoolbox') }}</option>
        </select>
      </div>

      <!-- Two-pass编码 -->
      <div class="checkbox-group">
        <label class="checkbox-label">
          <input type="checkbox" v-model="localParams.twoPass" />
          <span>{{ t('video.twoPass') }}</span>
        </label>
      </div>
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
  container: 'mp4',
  codec: 'h266', // 🔥 默认使用最新的H.266
  crf: 23,
  speed: 5,
  gopSize: 250,
  bframes: 3,
  refs: 3,
  pixelFormat: 'auto',
  hwAccel: 'auto',
  twoPass: false,
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

.slider-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
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

.param-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.param-label {
  font-size: 13px;
  color: var(--text-primary);
  font-weight: 500;
}

.param-select {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 12px;
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
