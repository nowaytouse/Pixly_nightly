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
  padding: 14px;
  background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%);
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 15px;
  font-weight: 700;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: all 0.3s;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

.convert-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(64, 158, 255, 0.4);
}

.convert-btn:active:not(:disabled) {
  transform: translateY(0);
}

.convert-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.convert-btn.converting {
  background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
}

.btn-icon {
  font-size: 18px;
}
</style>
