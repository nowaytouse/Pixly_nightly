<template>
  <div class="panel">
    <div class="panel-title" @click="expanded = !expanded" style="cursor: pointer;">
      <span>⚙️</span>
      <span>高级参数</span>
      <span class="expand-icon">{{ expanded ? '▼' : '▶' }}</span>
    </div>
    
    <div v-if="expanded" class="params-content">
      <p class="hint">根据格式显示不同的高级参数</p>
      <!-- 后续添加具体参数 -->
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

defineProps({
  format: String,
  modelValue: Object
})

defineEmits(['update:modelValue'])

const expanded = ref(false)
</script>

<style scoped>
.expand-icon {
  margin-left: auto;
  font-size: 12px;
  color: var(--text-tertiary);
}

.params-content {
  padding-top: 8px;
}

.hint {
  font-size: 12px;
  color: var(--text-tertiary);
  text-align: center;
  padding: 20px;
}
</style>
