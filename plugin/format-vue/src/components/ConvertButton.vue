<template>
  <button 
    class="convert-btn"
    :class="{ converting: isConverting }"
    :disabled="disabled || isConverting"
    @click="handleClick"
  >
    <span class="btn-icon">{{ isConverting ? '⏳' : '🚀' }}</span>
    <span class="btn-text">{{ isConverting ? t('convert.converting') : t('convert.start') }}</span>
  </button>
</template>

<script setup>
import { ref } from 'vue'
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()

defineProps({
  disabled: Boolean
})

const emit = defineEmits(['click'])

const isConverting = ref(false)

const handleClick = async () => {
  isConverting.value = true
  try {
    await emit('click')
  } finally {
    isConverting.value = false
  }
}
</script>

<style scoped>
.convert-btn {
  width: 100%;
  padding: 14px;
  background: linear-gradient(135deg, var(--color-primary), hsl(var(--hue-primary), 80%, 50%));
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  transition: all var(--duration-fast) var(--ease-out);
  box-shadow: 0 4px 12px hsla(var(--hue-primary), 90%, 60%, 0.3);
  position: relative;
  overflow: hidden;
  letter-spacing: 0.02em;
}

.convert-btn::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(rgba(255,255,255,0.1), transparent);
  opacity: 0;
  transition: opacity var(--duration-fast);
}

.convert-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px hsla(var(--hue-primary), 90%, 60%, 0.4);
  filter: brightness(1.1);
}

.convert-btn:hover:not(:disabled)::after {
  opacity: 1;
}

.convert-btn:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: 0 2px 8px hsla(var(--hue-primary), 90%, 60%, 0.3);
}

.convert-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  filter: grayscale(0.5);
  box-shadow: none;
}

.convert-btn.converting {
  background: linear-gradient(135deg, var(--color-success), hsl(var(--hue-success), 80%, 50%));
  box-shadow: 0 4px 12px hsla(var(--hue-success), 90%, 60%, 0.3);
}

.btn-icon {
  font-size: 18px;
  transition: transform var(--duration-fast) var(--ease-elastic);
}

.convert-btn:hover .btn-icon {
  transform: scale(1.2) rotate(-10deg);
}

.convert-btn.converting .btn-icon {
  animation: spin 2s linear infinite;
  transform: none;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
