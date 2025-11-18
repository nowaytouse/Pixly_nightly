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

    <!-- 编码参数 -->
    <div class="panel">
      <div class="panel-title">
        <span>⚙️</span>
        <span>编码参数</span>
      </div>
      
      <!-- Speed -->
      <div class="slider-group">
        <div class="slider-header">
          <span>编码速度</span>
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
        <div class="slider-hint">0=最快 | 5=平衡 | 9=最慢最优</div>
      </div>

      <!-- GOP Size -->
      <div class="slider-group">
        <div class="slider-header">
          <span>GOP Size</span>
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
        <div class="slider-hint">关键帧间隔 (250=10秒@25fps)</div>
      </div>

      <!-- B-frames -->
      <div class="slider-group">
        <div class="slider-header">
          <span>B帧数量</span>
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
        <div class="slider-hint">0=禁用 | 3=推荐 | 16=最大</div>
      </div>

      <!-- Reference Frames -->
      <div class="slider-group">
        <div class="slider-header">
          <span>参考帧</span>
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
        <div class="slider-hint">1=最快 | 3=推荐 | 16=最佳质量</div>
      </div>
    </div>

    <!-- 高级选项 -->
    <div class="panel">
      <div class="panel-title">
        <span>🔧</span>
        <span>高级选项</span>
      </div>

      <!-- 像素格式 -->
      <div class="param-group">
        <label class="param-label">像素格式</label>
        <select v-model="localParams.pixelFormat" class="param-select">
          <option value="auto">自动</option>
          <option value="yuv420p">YUV 4:2:0 8-bit</option>
          <option value="yuv422p">YUV 4:2:2 8-bit</option>
          <option value="yuv444p">YUV 4:4:4 8-bit</option>
          <option value="yuv420p10le">YUV 4:2:0 10-bit</option>
          <option value="yuv422p10le">YUV 4:2:2 10-bit</option>
        </select>
      </div>

      <!-- 硬件加速 -->
      <div class="param-group">
        <label class="param-label">硬件加速</label>
        <select v-model="localParams.hwAccel" class="param-select">
          <option value="auto">自动检测</option>
          <option value="none">禁用</option>
          <option value="nvenc">NVIDIA (NVENC)</option>
          <option value="qsv">Intel (QSV)</option>
          <option value="videotoolbox">Apple (VideoToolbox)</option>
          <option value="amf">AMD (AMF)</option>
        </select>
      </div>

      <!-- Two-pass -->
      <div class="checkbox-group">
        <label class="checkbox-label">
          <input type="checkbox" v-model="localParams.twoPass" />
          <span>Two-pass 编码</span>
        </label>
        <div class="checkbox-hint">两次编码，更好的码率分配（速度慢2倍）</div>
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

.param-group {
  margin-bottom: 12px;
}

.param-label {
  display: block;
  font-size: 13px;
  color: var(--text-primary);
  margin-bottom: 6px;
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
</style>
