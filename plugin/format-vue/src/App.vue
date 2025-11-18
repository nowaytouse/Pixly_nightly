<template>
  <div class="pixly-app" :class="{ 'dark-theme': isDark }">
    <Header @toggle-theme="toggleTheme" />
    
    <div class="main-container">
      <div class="left-panel">
        <FormatSelector v-model="selectedFormat" />
        <QualityPanel v-model="quality" />
        <AdvancedParams :format="selectedFormat" v-model="advancedParams" />
      </div>
      
      <div class="right-panel">
        <FileList :files="files" @remove="removeFile" />
        <ConvertButton 
          :disabled="files.length === 0" 
          @click="startConversion" 
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import Header from './components/Header.vue'
import FormatSelector from './components/FormatSelector.vue'
import QualityPanel from './components/QualityPanel.vue'
import AdvancedParams from './components/AdvancedParams.vue'
import FileList from './components/FileList.vue'
import ConvertButton from './components/ConvertButton.vue'

const isDark = ref(true)
const selectedFormat = ref('jxl')
const quality = ref(90)
const advancedParams = ref({})
const files = ref([])

const toggleTheme = () => {
  isDark.value = !isDark.value
}

const removeFile = (index) => {
  files.value.splice(index, 1)
}

const startConversion = async () => {
  console.log('Starting conversion...', {
    format: selectedFormat.value,
    quality: quality.value,
    files: files.value.length
  })
}

onMounted(async () => {
  // 从Eagle加载文件
  if (window.eagle) {
    const items = await window.eagle.item.getSelected()
    files.value = items.map(item => ({
      id: item.id,
      name: item.name,
      path: item.filePath,
      size: item.size,
      ext: item.ext
    }))
  }
})
</script>

<style scoped>
.pixly-app {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.main-container {
  flex: 1;
  display: grid;
  grid-template-columns: 420px 1fr;
  gap: 16px;
  padding: 16px;
  overflow: hidden;
}

.left-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
}

.right-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>
