<template>
  <div class="panel">
    <div class="panel-title">
      <span>📦</span>
      <span>{{ t('format.title') }}</span>
    </div>
    
    <select 
      :value="modelValue"
      @change="$emit('update:modelValue', $event.target.value)"
      class="format-select"
    >
      <option value="jxl">✨ {{ t('format.jxl') }}</option>
      <option value="avif">🎬 {{ t('format.avif') }}</option>
      <option value="webp">🌐 {{ t('format.webp') }}</option>
      <option value="heic">🍎 {{ t('format.heic') }}</option>
    </select>
  </div>
</template>

<script setup>
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

defineProps({
  modelValue: String
})

defineEmits(['update:modelValue'])
</script>

<style scoped>
.format-select {
  width: 100%;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.format-select:hover {
  border-color: var(--color-primary);
}

.format-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(0, 114, 239, 0.1);
}
</style>
