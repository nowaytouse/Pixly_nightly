<template>
  <div class="params-group">
    <!-- 编码器选择 -->
    <div class="select-group">
      <label class="select-label">{{ t('params.encoder') }}</label>
      <select v-model="localParams.encoder" class="param-select">
        <option value="x265">{{ t('params.encoderX265') }}</option>
        <option value="libheif">{{ t('params.encoderLibheif') }}</option>
      </select>
    </div>

    <!-- 色度子采样 -->
    <div class="select-group">
      <label class="select-label">{{ t('params.chromaSubsampling') }}</label>
      <select v-model="localParams.chromaSubsampling" class="param-select">
        <option value="auto">{{ t('params.chromaAuto') }}</option>
        <option value="420">{{ t('params.chroma420') }}</option>
        <option value="444">{{ t('params.chroma444') }}</option>
      </select>
    </div>
    
    <!-- 无损模式 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.lossless" />
        <span>{{ t('params.lossless') }}</span>
      </label>
    </div>

    <!-- 嵌入缩略图 -->
    <div class="checkbox-group">
      <label class="checkbox-label">
        <input type="checkbox" v-model="localParams.embedThumbnail" />
        <span>{{ t('params.embedThumbnail') }}</span>
      </label>
      <div class="checkbox-hint">{{ t('params.embedThumbnailHint') }}</div>
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
