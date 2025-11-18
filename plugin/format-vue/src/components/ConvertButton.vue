<template>
  <button 
    class="convert-btn"
    :class="{ converting: isConverting }"
    :disabled="disabled || isConverting"
    @click="handleClick"
  >
    <span class="btn-icon">{{ isConverting ? '⏳' : '🚀' }}</span>
    <span class="btn-text">{{ isConverting ? '转换中...' : '开始转换' }}</span>
  </button>
</template>

<script setup>
import { ref } from 'vue'

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
  padding: 12px;
  background: var(--color-primary);
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: all 0.2s;
  box-shadow: var(--shadow-sm);
}

.convert-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
  box-shadow: var(--shadow-md);
}

.convert-btn:active:not(:disabled) {
  transform: scale(0.98);
}

.convert-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.convert-btn.converting {
  background: var(--success-color);
}

.btn-icon {
  font-size: 16px;
}
</style>
