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
import { useI18n } from './composables/useI18n'
import { logger, LOG_KEYS } from './utils/logger'

const selectedFormat = ref('jxl')
const quality = ref(90)
const advancedParams = ref({})
const files = ref([])

const { t } = useI18n()
const { loadSelectedFiles, refreshLibrary, showNotification } = useEagleAPI()
const { convertImages, isConverting, progress, currentFile } = useRustCLI()

// 日志：应用初始化
logger.info(LOG_KEYS.APP_INIT, 'PIXLY Format Vue initialized')

const removeFile = (index) => {
  files.value.splice(index, 1)
}

const startConversion = async () => {
  if (files.value.length === 0) {
    showNotification(t('notification.selectFiles'), 'warning')
    logger.warn(LOG_KEYS.CONVERT_START, 'No files selected')
    return
  }

  logger.info(LOG_KEYS.CONVERT_START, 'Starting conversion', {
    format: selectedFormat.value,
    quality: quality.value,
    fileCount: files.value.length
  })

  try {
    const options = {
      format: selectedFormat.value,
      quality: quality.value,
      ...advancedParams.value
    }

    const result = await convertImages(files.value, options)

    if (result.success) {
      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Conversion completed', {
        fileCount: files.value.length
      })
      showNotification(t('notification.convertSuccess', { count: files.value.length }), 'success')
      await refreshLibrary()
      await loadFiles()
    } else {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Conversion failed', { error: result.error })
      showNotification(t('notification.convertFailed', { error: result.error }), 'error')
    }
  } catch (error) {
    logger.error(LOG_KEYS.CONVERT_ERROR, 'Conversion exception', { error: error.message })
    showNotification(t('notification.convertError', { error: error.message }), 'error')
  }
}

const loadFiles = async () => {
  logger.info(LOG_KEYS.FILE_LOAD, 'Loading files from Eagle')
  try {
    const items = await loadSelectedFiles()
    files.value = items
    logger.info(LOG_KEYS.FILE_LOAD_SUCCESS, 'Files loaded', { count: items.length })
  } catch (error) {
    logger.error(LOG_KEYS.FILE_LOAD_ERROR, 'Failed to load files', { error: error.message })
    showNotification(t('notification.loadFailed'), 'error')
  }
}

onMounted(async () => {
  logger.info(LOG_KEYS.APP_MOUNT, 'App mounted')
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
