<template>
  <div class="params-group">
    <!-- 编码器选择 -->
    <div class="select-group">
      <label class="select-label">编码器</label>
      <select v-model="localParams.encoder" class="param-select">
        <option value="x265">x265 (推荐)</option>
        <option value="libheif">libheif (标准)</option>
      </select>
    </div>

    <!-- 色度子采样 -->
    <div class="select-group">
      <label class="select-label">色度子采样</label>
      <select v-model="localParams.chromaSubsampling" class="param-select">
        <option value="auto">自动</option>
        <option value="420">4:2:0（标准）</option>
        <option value="444">4:4:4（最佳）</option>
      </select>
    </div>
    
    <!-- 无损模式 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.lossless" />
        <span>无损编码</span>
      </label>
    </div>

    <!-- 嵌入缩略图 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.embedThumbnail" />
        <span>嵌入缩略图</span>
      </label>
      <div class="checkbox-hint">在文件中嵌入预览缩略图</div>
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
  encoder: 'x265',
  chromaSubsampling: 'auto',
  lossless: false,
  embedThumbnail: false,
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
