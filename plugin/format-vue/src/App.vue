<template>
  <div class="pixly-app">
    <!-- 🔥 无边框窗口标题栏 -->
    <div class="titlebar glass">
      <div class="titlebar-drag">
        <div class="app-logo">
          <span class="logo-icon">✨</span>
          <span class="titlebar-title">{{ t('app.title') }}</span>
        </div>
      </div>
      <div class="titlebar-actions">
        <button class="titlebar-btn" @click="toggleLanguage" :title="t('ui.language')">
          {{ locale === 'zh_CN' ? '🇨🇳' : '🇺🇸' }}
        </button>
      </div>
      <div class="titlebar-controls">
        <button class="titlebar-btn" @click="minimizeWindow" :title="t('ui.minimize')">
          <svg width="10" height="10" viewBox="0 0 12 12">
            <rect x="2" y="5" width="8" height="2" fill="currentColor"/>
          </svg>
        </button>
        <button class="titlebar-btn" @click="maximizeWindow" :title="t('ui.maximize')">
          <svg width="10" height="10" viewBox="0 0 12 12">
            <rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5"/>
          </svg>
        </button>
        <button class="titlebar-btn titlebar-close" @click="closeWindow" :title="t('ui.close')">
          <svg width="10" height="10" viewBox="0 0 12 12">
            <path d="M2 2 L10 10 M10 2 L2 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- 类型切换 -->
    <div class="type-tabs glass">
      <button 
        class="type-tab"
        :class="{ active: conversionType === 'image' }"
        @click="conversionType = 'image'"
      >
        <span class="tab-icon">🖼️</span>
        {{ t('tabs.image') }}
      </button>
      <button 
        class="type-tab"
        :class="{ active: conversionType === 'video' }"
        @click="conversionType = 'video'"
      >
        <span class="tab-icon">🎬</span>
        {{ t('tabs.video') }}
      </button>
    </div>
    
    <div class="main-container">
      <div class="left-panel">
        <!-- 图像面板 -->
        <template v-if="conversionType === 'image'">
          <div class="panel-group fade-in">
            <FormatSelector v-model="selectedFormat" />
          </div>
          <div class="panel-group fade-in" style="animation-delay: 50ms">
            <QualityPanel 
              v-model:modelValue="quality" 
              v-model:lossless="lossless"
            />
          </div>
          <div class="panel-group fade-in" style="animation-delay: 100ms">
            <AdvancedParams 
              :format="selectedFormat" 
              v-model="advancedParams" 
              :lossless="lossless"
            />
          </div>
          <div class="panel-group fade-in" style="animation-delay: 150ms">
            <QuickTools v-model="quickTools" :logs="logs" ref="quickToolsRef" />
          </div>
        </template>
        
        <!-- 视频面板 -->
        <template v-else>
          <div class="panel-group fade-in">
            <VideoPanel v-model="videoParams" />
          </div>
        </template>
      </div>
      
      <div class="right-panel fade-in">
        <FileList :files="files" @remove="removeFile" />
      </div>
    </div>
    
    <footer class="footer glass">
      <div class="footer-left">
        <div class="status-badge">
          <span class="status-dot" :class="{ active: !isConverting }"></span>
          <span class="file-count">{{ files.length }} {{ t('files.count') }}</span>
        </div>
        <button 
          class="btn-refresh"
          :disabled="isConverting"
          @click="loadFiles"
          :title="t('ui.refreshFiles')"
        >
          <span class="icon">🔄</span> {{ t('ui.refresh') }}
        </button>
      </div>
      <div class="footer-right">
        <button 
          class="btn btn-primary btn-lg"
          :disabled="files.length === 0 || isConverting"
          @click="startConversion"
        >
          <span class="icon" v-if="isConverting">⚙️</span>
          <span class="icon" v-else>✨</span>
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
import QuickTools from './components/QuickTools.vue'
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
const quickTools = ref({
  fileValidation: true,
  formatCorrection: true,
  autoMergeXmp: true,
  normalizeFilenames: false
})
const files = ref([])

// 🔍 日志系统
const logs = ref([])
const quickToolsRef = ref(null)
let scrollTimer = null  // 🔥 性能优化：防抖滚动

// 添加日志
const addLog = (message, type = 'info', icon = '📝') => {
  const now = new Date()
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`
  
  logs.value.push({
    time,
    icon,
    message,
    type // info, success, warning, error
  })
  
  // 🔥 性能优化：防抖滚动，避免创建大量定时器
  if (scrollTimer) {
    clearTimeout(scrollTimer)
  }
  scrollTimer = setTimeout(() => {
    if (quickToolsRef.value) {
      quickToolsRef.value.scrollLogToBottom()
    }
    scrollTimer = null
  }, 50)  // 增加到50ms，减少滚动频率
}

// 清空日志
const clearLogs = () => {
  logs.value = []
  // 🔥 清理定时器
  if (scrollTimer) {
    clearTimeout(scrollTimer)
    scrollTimer = null
  }
}

// Toast状态
const toast = ref({
  show: false,
  type: 'error',
  title: '',
  message: ''
})

const { t, setLocale, locale } = useI18n()
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

  // 🔍 清空日志并切换到日志标签
  clearLogs()
  if (quickToolsRef.value) {
    quickToolsRef.value.switchToLog()
  }
  
  // 🔍 添加开始日志
  if (conversionType.value === 'image') {
    addLog(t('log.startProcessingImages', { count: files.value.length }), 'info', '🚀')
    addLog(t('log.outputFormat', { format: selectedFormat.value.toUpperCase() }), 'info', '🎯')
    addLog(lossless.value 
      ? t('log.qualityLossless', { quality: quality.value })
      : t('log.qualitySetting', { quality: quality.value }), 'info', '⚙️')
  } else {
    addLog(t('log.startProcessingVideos', { count: files.value.length }), 'info', '🚀')
    addLog(t('log.videoParams', { params: JSON.stringify(videoParams.value) }), 'info', '🎯')
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
        ...advancedParams.value,
        // 快捷工具
        quickTools: quickTools.value
      }
      
      // 🔍 使用进度回调添加日志
      result = await convertImages(files.value, options, (index, total, fileName, status, error) => {
        if (status === 'processing') {
          addLog(t('log.processing', { current: index, total, file: fileName }), 'info', '⚙️')
        } else if (status === 'success') {
          addLog(t('log.success', { file: fileName }), 'success', '')
        } else if (status === 'error') {
          addLog(t('log.error', { file: fileName, error }), 'error', '')
        }
      })
    } else {
      const options = {
        ...videoParams.value
      }
      result = await convertVideos(files.value, options)
    }

    if (result.success) {
      // 🔥 显示详细的转换结果
      const summary = result.summary || {}
      const successMsg = `✅ ${t('log.conversionComplete')}\n${t('log.successCount', { success: summary.success || 0, total: summary.total || 0 })}` +
                        (summary.failed > 0 ? `\n${t('log.failedCount', { failed: summary.failed })}` : '') +
                        (summary.xmpMerged > 0 ? `\n${t('log.xmpMerged', { count: summary.xmpMerged })}` : '')
      
      // 🔍 添加完成总结日志
      addLog('─────────────────────────', 'info', '')
      if (summary.failed === 0) {
        addLog(t('log.allComplete', { success: summary.success, total: summary.total }), 'success', '✅')
      } else if (summary.success > 0) {
        addLog(t('log.partialComplete', { success: summary.success, failed: summary.failed }), 'warning', '⚠️')
      } else {
        addLog(t('log.allFailed', { failed: summary.failed, total: summary.total }), 'error', '❌')
      }
      
      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Conversion completed', {
        total: summary.total,
        success: summary.success,
        failed: summary.failed,
        xmpMerged: summary.xmpMerged
      })
      
      // 🔥 显示 Toast 和 Eagle 通知
      showToast('success', t('log.conversionComplete'), successMsg)
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

// 🌐 语言切换
const toggleLanguage = () => {
  const newLocale = locale.value === 'zh_CN' ? 'en' : 'zh_CN'
  setLocale(newLocale)
  logger.info(LOG_KEYS.UI_CLICK, 'Language switched', { locale: newLocale })
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
  background: var(--bg-app);
  color: var(--text-primary);
  overflow: hidden;
  transform: translateZ(0);
  backface-visibility: hidden;
}

/* 🔥 无边框窗口标题栏 */
.titlebar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  user-select: none;
  padding: 0 4px;
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

.app-logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  font-size: 16px;
  filter: drop-shadow(0 0 8px rgba(0, 114, 239, 0.5));
}

.titlebar-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: 0.02em;
}

.titlebar-actions,
.titlebar-controls {
  display: flex;
  height: 100%;
  -webkit-app-region: no-drag;
}

.titlebar-btn {
  width: 40px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.titlebar-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.titlebar-close:hover {
  background: var(--color-danger);
  color: white;
  box-shadow: 0 0 10px var(--color-danger);
}

.main-container {
  flex: 1;
  display: grid;
  grid-template-columns: 380px 1fr;
  gap: 20px;
  padding: 20px;
  /* 🔥 为固定的 titlebar (36px) + type-tabs (~56px) 留出空间 */
  margin-top: 92px; /* 36px titlebar + 56px tabs */
  overflow: hidden;
  /* 性能优化 */
  will-change: auto;
}

.left-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  overflow-x: hidden;
  padding-right: 4px; /* 防止滚动条遮挡内容 */
  /* 流畅滚动 */
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
  /* 🔥 确保 sticky 元素在滚动容器内工作 */
  position: relative;
}

.panel-group {
  animation: slideIn var(--duration-normal) var(--ease-out);
  animation-fill-mode: both;
}

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
  gap: 16px;
  overflow: hidden;
  background: var(--bg-panel);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  animation: fadeIn var(--duration-normal) var(--ease-out);
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
  padding: 0 20px;
  min-height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  transition: all var(--duration-fast) var(--ease-out);
  z-index: 100;
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: var(--bg-input);
  border-radius: 20px;
  border: 1px solid var(--border-color);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-success);
  box-shadow: 0 0 8px var(--color-success);
  transition: all var(--duration-normal);
}

.status-dot.active {
  background: var(--color-warning);
  box-shadow: 0 0 8px var(--color-warning);
}

.file-count {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  transition: color var(--duration-fast) var(--ease-out);
}

.btn {
  padding: 8px 16px;
  background: var(--bg-button);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.btn:hover:not(:disabled) {
  background: var(--bg-button-hover);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.btn:active:not(:disabled) {
  transform: scale(0.98);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  filter: grayscale(0.3);
}

.btn-primary {
  background: var(--gradient-primary);
  border: none;
  color: white;
  box-shadow: var(--glow-primary);
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(99, 102, 241, 0.6);
  filter: brightness(1.1);
}

.btn-primary:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: var(--glow-primary);
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
  position: fixed;
  top: 32px;
  left: 0;
  right: 0;
  z-index: 999;
  display: flex;
  gap: 8px;
  padding: 12px 16px 0;
  background: var(--bg-secondary);
  /* 确保在滚动时保持在标题栏下方 */
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.type-tab {
  flex: 1;
  padding: 12px 16px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--duration-normal) var(--ease-out);
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.type-tab:hover {
  color: var(--text-primary);
  background: rgba(255, 255, 255, 0.03);
}

.type-tab.active {
  color: var(--color-primary);
  background: linear-gradient(to bottom, transparent, rgba(99, 102, 241, 0.05));
}

.type-tab.active::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 100%;
  height: 2px;
  background: var(--gradient-primary);
  box-shadow: var(--glow-primary);
}
</style>
