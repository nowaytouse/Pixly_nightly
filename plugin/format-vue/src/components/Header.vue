<template>
  <header class="header">
    <div class="header-left">
      <img src="/logo.png" alt="PIXLY" class="logo" />
      <div class="title-group">
        <h1 class="title">PIXLY Format</h1>
        <p class="subtitle">专业格式转换</p>
      </div>
    </div>
  </header>
</template>

<script setup>
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
</style>
