<template>
  <div class="pixly-app">
    <!-- 🔥 无边框窗口标题栏 -->
    <div class="titlebar">
      <div class="titlebar-drag">
        <span class="titlebar-title">{{ t('app.title') }}</span>
      </div>
      <div class="titlebar-controls">
        <button class="titlebar-btn" @click="minimizeWindow" :title="t('ui.minimize')">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect x="2" y="5" width="8" height="2" fill="currentColor"/>
          </svg>
        </button>
        <button class="titlebar-btn" @click="maximizeWindow" :title="t('ui.maximize')">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5"/>
          </svg>
        </button>
        <button class="titlebar-btn titlebar-close" @click="closeWindow" :title="t('ui.close')">
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M2 2 L10 10 M10 2 L2 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- 类型切换 -->
    <div class="type-tabs">
      <button 
        class="type-tab"
        :class="{ active: conversionType === 'image' }"
        @click="conversionType = 'image'"
      >
        {{ t('tabs.image') }}
      </button>
      <button 
        class="type-tab"
        :class="{ active: conversionType === 'video' }"
        @click="conversionType = 'video'"
      >
        {{ t('tabs.video') }}
      </button>
    </div>
    
    <div class="main-container">
      <div class="left-panel">
        <!-- 图像面板 -->
        <template v-if="conversionType === 'image'">
          <FormatSelector v-model="selectedFormat" />
          <QualityPanel v-model:modelValue="quality" v-model:lossless="lossless" />
          <AdvancedParams :format="selectedFormat" v-model="advancedParams" :lossless="lossless" />
        </template>
        
        <!-- 视频面板 -->
        <template v-else>
          <VideoPanel v-model="videoParams" />
        </template>
      </div>
      
      <div class="right-panel">
        <FileList :files="files" @remove="removeFile" />
      </div>
    </div>
    
    <footer class="footer">
      <div class="footer-left">
        <span class="file-count">{{ files.length }} {{ t('files.count') }}</span>
        <button 
          class="btn-refresh"
          :disabled="isConverting"
          @click="loadFiles"
          :title="t('ui.refreshFiles')"
        >
          🔄 {{ t('ui.refresh') }}
        </button>
      </div>
      <div class="footer-right">
        <button 
          class="btn-convert"
          :disabled="files.length === 0 || isConverting"
          @click="startConversion"
        >
          {{ isConverting ? t('convert.converting') : t('convert.start') }}
        </button>
      </div>
    </footer>
    
    <ProgressBar 
      :show="isConverting"
      :progress="progress"
      :current-file="currentFile"
    />
    
    <ErrorToast
      :show="toast.show"
      :type="toast.type"
      :title="toast.title"
      :message="toast.message"
      @close="toast.show = false"
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import FormatSelector from './components/FormatSelector.vue'
import QualityPanel from './components/QualityPanel.vue'
import AdvancedParams from './components/AdvancedParams.vue'
import VideoPanel from './components/VideoPanel.vue'
import FileList from './components/FileList.vue'
import ProgressBar from './components/ProgressBar.vue'
import ErrorToast from './components/ErrorToast.vue'
import { useEagleAPI } from './composables/useEagleAPI'
import { useRustCLI } from './composables/useRustCLI'
import { useI18n } from './composables/useI18n'
import { logger, LOG_KEYS } from './utils/logger'

const conversionType = ref('image') // 'image' or 'video'
const selectedFormat = ref('jxl')
const quality = ref(90)
const lossless = ref(false)
const advancedParams = ref({})
const videoParams = ref({})
const files = ref([])

// Toast状态
const toast = ref({
  show: false,
  type: 'error',
  title: '',
  message: ''
})

const { t } = useI18n()
const { loadSelectedFiles, refreshLibrary, showNotification, setEagleReady } = useEagleAPI()
const { convertImages, convertVideos, isConverting, progress, currentFile } = useRustCLI()

// 显示Toast
const showToast = (type, title, message) => {
  toast.value = { show: true, type, title, message }
  setTimeout(() => {
    toast.value.show = false
  }, 5000)
}

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
    let result
    
    if (conversionType.value === 'image') {
      const options = {
        format: selectedFormat.value,
        quality: quality.value,
        lossless: lossless.value,
        ...advancedParams.value
      }
      result = await convertImages(files.value, options)
    } else {
      const options = {
        ...videoParams.value
      }
      result = await convertVideos(files.value, options)
    }

    if (result.success) {
      // 🔥 显示详细的转换结果
      const summary = result.summary || {}
      const successMsg = `✅ 转换完成！\n成功: ${summary.success || 0}/${summary.total || 0}` +
                        (summary.failed > 0 ? `\n失败: ${summary.failed}` : '') +
                        (summary.xmpMerged > 0 ? `\nXMP已合并: ${summary.xmpMerged}` : '')
      
      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Conversion completed', {
        total: summary.total,
        success: summary.success,
        failed: summary.failed,
        xmpMerged: summary.xmpMerged
      })
      
      // 🔥 显示 Toast 和 Eagle 通知
      showToast('success', '转换完成', successMsg)
      showNotification(successMsg, 'success')
      
      // 🔥 删除已合并的 XMP 资源（从 Eagle 数据库）
      if (summary.xmpMerged > 0 && result.results) {
        const xmpIdsToDelete = []
        result.results.forEach(r => {
          if (r.success && r.hasXmp && r.xmpId) {
            xmpIdsToDelete.push(r.xmpId)
          }
        })
        
        if (xmpIdsToDelete.length > 0) {
          logger.info(LOG_KEYS.EAGLE_API_CALL, 'Deleting XMP resources from Eagle', {
            count: xmpIdsToDelete.length,
            ids: xmpIdsToDelete
          })
          
          // 调用 Eagle API 删除 XMP 资源
          for (const xmpId of xmpIdsToDelete) {
            try {
              if (window.eagle && window.eagle.item && window.eagle.item.removeItems) {
                await window.eagle.item.removeItems([xmpId])
                logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'XMP resource deleted from Eagle', { id: xmpId })
              }
            } catch (e) {
              logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Failed to delete XMP from Eagle', { 
                id: xmpId, 
                error: e.message 
              })
            }
          }
        }
      }
      
      await refreshLibrary()
      await loadFiles()
    } else {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Conversion failed', { error: result.error })
      showToast('error', t('notification.convertFailed'), result.error)
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

// 🔥 监听Eagle事件 - 参考官方插件实现
// 官方插件使用 eagle.onPluginRun 自动加载选中的文件
if (window.eagle) {
  logger.info(LOG_KEYS.APP_INIT, 'Registering Eagle event listeners')
  
  // 监听插件创建事件
  if (window.eagle.onPluginCreate) {
    window.eagle.onPluginCreate(async (plugin) => {
      logger.info(LOG_KEYS.APP_INIT, 'Eagle plugin-create event', { 
        id: plugin?.id,
        name: plugin?.name 
      })
      // 设置 Eagle 就绪状态
      setEagleReady()
    })
  }

  // 🔥 监听插件运行事件 - 官方插件在这里加载文件
  if (window.eagle.onPluginRun) {
    window.eagle.onPluginRun(async (plugin) => {
      logger.info(LOG_KEYS.FILE_LOAD, 'Eagle plugin-run event, loading selected files')
      
      // 自动加载选中的文件
      await loadFiles()
    })
  }

  // 🔥 监听插件显示事件（用户切换回插件）
  // 这是最重要的事件：用户在Eagle中重新选择文件后切换回插件
  if (window.eagle.onPluginShow) {
    window.eagle.onPluginShow(async () => {
      logger.info(LOG_KEYS.FILE_LOAD, 'Eagle plugin-show event, reloading files')
      
      // 🔥 修复：不要在转换中跳过，而是等待转换完成后加载
      if (isConverting.value) {
        logger.info(LOG_KEYS.FILE_LOAD, 'Conversion in progress, will reload after completion')
        // 等待转换完成
        const checkInterval = setInterval(async () => {
          if (!isConverting.value) {
            clearInterval(checkInterval)
            logger.info(LOG_KEYS.FILE_LOAD, 'Conversion completed, reloading files')
            await loadFiles()
          }
        }, 500)
        return
      }
      
      // 立即加载文件
      await loadFiles()
    })
  }
  
  // 🔥 监听选择变化事件（如果Eagle支持）
  if (window.eagle.onSelectionChanged) {
    window.eagle.onSelectionChanged(async () => {
      logger.info(LOG_KEYS.FILE_LOAD, 'Eagle selection changed, reloading files')
      
      if (!isConverting.value) {
        await loadFiles()
      }
    })
  }
} else {
  logger.error(LOG_KEYS.APP_INIT, 'Eagle API not available!')
}

// 🔥 窗口控制函数
const minimizeWindow = () => {
  try {
    logger.info(LOG_KEYS.APP_INIT, 'Attempting to minimize window')
    
    // 检查Eagle API
    if (!window.eagle) {
      logger.error(LOG_KEYS.APP_INIT, 'Eagle API not available')
      return
    }
    
    logger.debug(LOG_KEYS.APP_INIT, 'Eagle API available', {
      hasWindow: !!window.eagle.window,
      windowMethods: window.eagle.window ? Object.keys(window.eagle.window) : []
    })
    
    // 尝试不同的API方法
    if (window.eagle.window && typeof window.eagle.window.minimize === 'function') {
      window.eagle.window.minimize()
      logger.info(LOG_KEYS.APP_INIT, 'Window minimized')
    } else if (window.eagle.app && typeof window.eagle.app.minimize === 'function') {
      window.eagle.app.minimize()
      logger.info(LOG_KEYS.APP_INIT, 'Window minimized (via app)')
    } else {
      logger.error(LOG_KEYS.APP_INIT, 'Minimize method not found')
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_INIT, 'Failed to minimize window', { error: error.message })
  }
}

// 🔥 跟踪窗口最大化状态
const isMaximized = ref(false)

const maximizeWindow = () => {
  try {
    logger.info(LOG_KEYS.APP_INIT, 'Attempting to toggle maximize window', { currentState: isMaximized.value })
    
    if (!window.eagle || !window.eagle.window) {
      logger.error(LOG_KEYS.APP_INIT, 'Eagle API not available')
      return
    }
    
    // 🔥 根据当前状态切换
    if (isMaximized.value) {
      // 当前是最大化，需要还原
      if (typeof window.eagle.window.unmaximize === 'function') {
        window.eagle.window.unmaximize()
        isMaximized.value = false
        logger.info(LOG_KEYS.APP_INIT, 'Window unmaximized')
      } else if (typeof window.eagle.window.restore === 'function') {
        window.eagle.window.restore()
        isMaximized.value = false
        logger.info(LOG_KEYS.APP_INIT, 'Window restored')
      } else {
        logger.error(LOG_KEYS.APP_INIT, 'Unmaximize method not found')
      }
    } else {
      // 当前是正常大小，需要最大化
      if (typeof window.eagle.window.maximize === 'function') {
        window.eagle.window.maximize()
        isMaximized.value = true
        logger.info(LOG_KEYS.APP_INIT, 'Window maximized')
      } else {
        logger.error(LOG_KEYS.APP_INIT, 'Maximize method not found')
      }
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_INIT, 'Failed to toggle maximize window', { error: error.message })
  }
}

const closeWindow = () => {
  try {
    logger.info(LOG_KEYS.APP_INIT, 'Attempting to close window')
    
    if (!window.eagle) {
      logger.error(LOG_KEYS.APP_INIT, 'Eagle API not available')
      return
    }
    
    // 尝试不同的API方法
    if (window.eagle.window && typeof window.eagle.window.close === 'function') {
      window.eagle.window.close()
      logger.info(LOG_KEYS.APP_INIT, 'Window closed')
    } else if (window.eagle.app && typeof window.eagle.app.close === 'function') {
      window.eagle.app.close()
      logger.info(LOG_KEYS.APP_INIT, 'Window closed (via app)')
    } else if (window.close && typeof window.close === 'function') {
      window.close()
      logger.info(LOG_KEYS.APP_INIT, 'Window closed (via window.close)')
    } else {
      logger.error(LOG_KEYS.APP_INIT, 'Close method not found')
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_INIT, 'Failed to close window', { error: error.message })
  }
}

onMounted(async () => {
  logger.info(LOG_KEYS.APP_MOUNT, 'App mounted')
  logger.info(LOG_KEYS.APP_MOUNT, 'PIXLY Format Vue application mounted successfully')
  logger.info(LOG_KEYS.APP_MOUNT, 'Waiting for Eagle plugin-create event...')
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
  /* 性能优化 */
  transform: translateZ(0);
  backface-visibility: hidden;
}

/* 🔥 无边框窗口标题栏 */
.titlebar {
  height: 32px;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  user-select: none;
}

.titlebar-drag {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 12px;
  -webkit-app-region: drag;
  cursor: move;
}

.titlebar-title {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.titlebar-controls {
  display: flex;
  height: 100%;
  -webkit-app-region: no-drag;
}

.titlebar-btn {
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast) var(--ease-out);
}

.titlebar-btn:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-primary);
}

.titlebar-btn:active {
  background: rgba(255, 255, 255, 0.1);
}

.titlebar-close:hover {
  background: #e81123;
  color: white;
}

.titlebar-close:active {
  background: #c50f1f;
}

.main-container {
  flex: 1;
  display: grid;
  grid-template-columns: 360px 1fr;
  gap: 16px;
  padding: 16px;
  overflow: hidden;
  /* 性能优化 */
  will-change: auto;
}

.left-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow-y: auto;
  /* 流畅滚动 */
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
}

.left-panel > * {
  animation: slideIn var(--transition-base) var(--ease-out);
  animation-fill-mode: both;
}

.left-panel > *:nth-child(1) { animation-delay: 0ms; }
.left-panel > *:nth-child(2) { animation-delay: 50ms; }
.left-panel > *:nth-child(3) { animation-delay: 100ms; }

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.right-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow: hidden;
  animation: fadeIn var(--transition-base) var(--ease-out);
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
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
  transition: all var(--transition-fast) var(--ease-out);
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.file-count {
  font-size: 13px;
  color: var(--text-secondary);
  transition: color var(--transition-fast) var(--ease-out);
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
  transition: all var(--transition-base) var(--ease-out);
  box-shadow: 0 2px 4px rgba(0, 114, 239, 0.3);
  /* 性能优化 */
  transform: translateZ(0);
  backface-visibility: hidden;
}

.btn-convert:hover:not(:disabled) {
  background: var(--color-primary-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 114, 239, 0.4);
}

.btn-convert:active:not(:disabled) {
  background: var(--color-primary-active);
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(0, 114, 239, 0.3);
}

.btn-convert:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  filter: grayscale(0.3);
}

.btn-refresh {
  padding: 6px 16px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all var(--transition-base) var(--ease-out);
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-refresh:hover:not(:disabled) {
  background: var(--bg-secondary);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.btn-refresh:active:not(:disabled) {
  transform: scale(0.95);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.type-tabs {
  display: flex;
  gap: 8px;
  padding: 12px 16px 0;
  background: var(--bg-secondary);
  position: relative;
}

.type-tab {
  flex: 1;
  padding: 8px 16px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-base) var(--ease-out);
  position: relative;
  /* 性能优化 */
  transform: translateZ(0);
}

.type-tab::before {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  width: 0;
  height: 2px;
  background: var(--color-primary);
  transition: all var(--transition-base) var(--ease-out);
  transform: translateX(-50%);
}

.type-tab:hover {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.03);
}

.type-tab.active {
  color: var(--color-primary);
}

.type-tab.active::before {
  width: 100%;
}
</style>
