<template>
  <transition name="toast">
    <div v-if="show" class="error-toast" :class="type">
      <div class="toast-icon">
        {{ icon }}
      </div>
      <div class="toast-content">
        <div class="toast-title">{{ title }}</div>
        <div v-if="message" class="toast-message">{{ message }}</div>
      </div>
      <button class="toast-close" @click="$emit('close')">✕</button>
    </div>
  </transition>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  show: Boolean,
  type: {
    type: String,
    default: 'error' // 'error', 'warning', 'success', 'info'
  },
  title: String,
  message: String
})

defineEmits(['close'])

const icon = computed(() => {
  switch (props.type) {
    case 'error': return '❌'
    case 'warning': return '⚠️'
    case 'success': return '✅'
    case 'info': return 'ℹ️'
    default: return '❌'
  }
})
</script>

<style scoped>
.error-toast {
  position: fixed;
  top: 20px;
  right: 20px;
  min-width: 320px;
  max-width: 480px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 16px;
  box-shadow: var(--shadow-lg);
  display: flex;
  align-items: flex-start;
  gap: 12px;
  z-index: 2000;
}

.error-toast.error {
  border-left: 4px solid var(--danger-color);
}

.error-toast.warning {
  border-left: 4px solid var(--warning-color);
}

.error-toast.success {
  border-left: 4px solid var(--success-color);
}

.error-toast.info {
  border-left: 4px solid var(--color-primary);
}

.toast-icon {
  font-size: 20px;
  flex-shrink: 0;
}

.toast-content {
  flex: 1;
  min-width: 0;
}

.toast-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.toast-message {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.toast-close {
  width: 20px;
  height: 20px;
  background: transparent;
  border: none;
  color: var(--text-tertiary);
  font-size: 14px;
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.2s;
}

.toast-close:hover {
  color: var(--text-primary);
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}
</style>
