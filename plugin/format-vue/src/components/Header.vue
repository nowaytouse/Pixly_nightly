<template>
  <header class="header">
    <div class="header-left">
      <img src="/logo.png" alt="PIXLY" class="logo" />
      <div class="title-group">
        <h1 class="title">PIXLY Format</h1>
        <p class="subtitle">专业格式转换</p>
      </div>
    </div>
    
    <div class="header-right">
      <button class="icon-btn" @click="$emit('toggle-theme')" title="切换主题">
        <span>🌙</span>
      </button>
    </div>
  </header>
</template>

<script setup>
defineEmits(['toggle-theme'])
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo {
  width: 36px;
  height: 36px;
  border-radius: 8px;
}

.title-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.title {
  font-size: 16px;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.subtitle {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1;
}

.header-right {
  display: flex;
  gap: 8px;
}

.icon-btn {
  width: 32px;
  height: 32px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  transition: all 0.3s;
}

.icon-btn:hover {
  background: var(--primary-color);
  border-color: var(--primary-color);
  transform: scale(1.05);
}
</style>
