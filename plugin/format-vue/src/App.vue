<template>
  <div class="pixly-app">
    <Header />
    
    <div class="main-container">
      <div class="left-panel">
        <FormatSelector v-model="selectedFormat" />
        <QualityPanel v-model="quality" />
        <AdvancedParams :format="selectedFormat" v-model="advancedParams" />
      </div>
      
      <div class="right-panel">
        <FileList :files="files" @remove="removeFile" />
      </div>
    </div>
    
    <footer class="footer">
      <div class="footer-left">
        <span class="file-count">{{ files.length }} 个文件</span>
      </div>
      <div class="footer-right">
        <button 
          class="btn-convert"
          :disabled="files.length === 0 || isConverting"
          @click="startConversion"
        >
          {{ isConverting ? '转换中...' : '开始转换' }}
        </button>
      </div>
    </footer>
    
    <ProgressBar 
      :show="isConverting"
      :progress="progress"
      :current-file="currentFile"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import Header from './components/Header.vue'
import FormatSelector from './components/FormatSelector.vue'
import QualityPanel from './components/QualityPanel.vue'
import AdvancedParams from './components/AdvancedParams.vue'
import FileList from './components/FileList.vue'
import ProgressBar from './components/ProgressBar.vue'
import { useEagleAPI } from './composables/useEagleAPI'
import { useRustCLI } from './composables/useRustCLI'

const selectedFormat = ref('jxl')
const quality = ref(90)
const advancedParams = ref({})
const files = ref([])

const { loadSelectedFiles, refreshLibrary, showNotification } = useEagleAPI()
const { convertImages, isConverting, progress, currentFile } = useRustCLI()

const removeFile = (index) => {
  files.value.splice(index, 1)
}

const startConversion = async () => {
  if (files.value.length === 0) {
    showNotification('请先选择文件', 'warning')
    return
  }

  try {
    const options = {
      format: selectedFormat.value,
      quality: quality.value,
      ...advancedParams.value
    }

    const result = await convertImages(files.value, options)

    if (result.success) {
      showNotification(`转换完成！成功转换 ${files.value.length} 个文件`, 'success')
      await refreshLibrary()
      
      // 重新加载文件列表
      await loadFiles()
    } else {
      showNotification(`转换失败：${result.error}`, 'error')
    }
  } catch (error) {
    console.error('Conversion error:', error)
    showNotification(`转换出错：${error.message}`, 'error')
  }
}

const loadFiles = async () => {
  try {
    const items = await loadSelectedFiles()
    files.value = items
  } catch (error) {
    console.error('Failed to load files:', error)
    showNotification('加载文件失败', 'error')
  }
}

onMounted(async () => {
  await loadFiles()
})
</script>

<style scoped>
.pixly-app {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.main-container {
  flex: 1;
  display: grid;
  grid-template-columns: 360px 1fr;
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
  overflow: hidden;
}

.footer {
  border-top: 1px solid var(--border-color);
  padding: 0 16px;
  min-height: 56px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--bg-primary);
  flex-shrink: 0;
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.file-count {
  font-size: 13px;
  color: var(--text-secondary);
}

.footer-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.btn-convert {
  padding: 8px 24px;
  background: var(--color-primary);
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-convert:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-convert:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
