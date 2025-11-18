<template>
  <div class="params-group">
    <div class="param-row">
      <label class="param-label">Effort</label>
      <select v-model="localParams.effort" class="param-select">
        <option value="3">3 - 快速</option>
        <option value="5">5 - 平衡</option>
        <option value="7" selected>7 - 推荐</option>
        <option value="9">9 - 极致</option>
      </select>
    </div>
    
    <div class="param-row">
      <label class="param-label">Distance</label>
      <input 
        type="number" 
        v-model.number="localParams.distance"
        min="0" 
        max="15" 
        step="0.1"
        class="param-input"
      />
    </div>
    
    <div class="param-row">
      <label class="param-label">
        <input type="checkbox" v-model="localParams.lossless" />
        <span>无损模式</span>
      </label>
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
  effort: 7,
  distance: 1.0,
  lossless: false,
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
  gap: 12px;
}

.param-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.param-label {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.param-select,
.param-input {
  flex: 1;
  padding: 6px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  color: var(--text-primary);
}

.param-select:focus,
.param-input:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
