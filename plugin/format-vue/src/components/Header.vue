<template>
  <header class="header">
    <h1 class="title">{{ t('app.title') }}</h1>
  </header>
</template>

<script setup>
import { useI18n } from '../composables/useI18n'

const { t } = useI18n()
</script>

<style scoped>
.header {
  padding: 16px 20px;
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--glass-border);
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 10;
}

.title {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  background: linear-gradient(135deg, #fff 0%, var(--text-secondary) 100%);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  letter-spacing: -0.02em;
}
</style>
