<template>
  <div class="panel">
    <div class="panel-title" @click="expanded = !expanded" style="cursor: pointer;">
      <span>⚙️</span>
      <span>高级参数</span>
      <span class="expand-icon">{{ expanded ? '▼' : '▶' }}</span>
    </div>
    
    <div v-if="expanded" class="params-content">
      <JxlParams v-if="format === 'jxl'" v-model="localParams" />
      <div v-else class="hint">{{ format.toUpperCase() }} 参数待实现</div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import JxlParams from './JxlParams.vue'

const props = defineProps({
  format: String,
  modelValue: Object
})

const emit = defineEmits(['update:modelValue'])

const expanded = ref(false)
const localParams = ref(props.modelValue || {})

watch(localParams, (newVal) => {
  emit('update:modelValue', newVal)
}, { deep: true })
</script>

<style scoped>
.expand-icon {
  margin-left: auto;
  font-size: 12px;
  color: var(--text-tertiary);
}

.params-content {
  padding-top: 12px;
}

.hint {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
  padding: 20px;
}
</style>
