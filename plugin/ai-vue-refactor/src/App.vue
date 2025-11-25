<template>
  <div class="app" :data-theme="isDark ? 'dark' : 'light'">
    <LiquidFilter />
    <!-- Header -->
    <header class="header liquid-glass">
      <div class="header-left">
        <div class="logo-container">
          <div class="logo-icon">🤖</div>
          <div class="logo-glow"></div>
        </div>
        <div class="title-group">
          <h1>{{ t('app.title') }}</h1>
          <p>{{ t('app.subtitle') }}</p>
        </div>
      </div>
      <div class="header-right">
        <div class="action-group">
          <button class="icon-btn" @click="refreshFiles" :title="t('header.refresh')">
            <span class="icon">🔄</span>
          </button>
          <button class="icon-btn" @click="showHelp = true" :title="t('header.help')">
            <span class="icon">❓</span>
          </button>
          <button class="icon-btn" @click="toggleLanguage" :title="t('header.language')">
            <span class="text-icon">{{ locale === 'zh_CN' ? 'CN' : 'EN' }}</span>
          </button>
          <button class="icon-btn" @click="toggleTheme" :title="t('header.theme')">
            <span class="icon">{{ isDark ? '☀️' : '🌙' }}</span>
          </button>
        </div>
        
        <!-- Window Controls -->
        <div class="window-controls">
          <button class="window-btn minimize" @click="minimizeWindow" title="Minimize">
            <svg width="10" height="10" viewBox="0 0 12 12"><rect x="2" y="5" width="8" height="2" fill="currentColor"/></svg>
          </button>
          <button class="window-btn maximize" @click="maximizeWindow" title="Maximize">
            <svg width="10" height="10" viewBox="0 0 12 12"><rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>
          </button>
          <button class="window-btn close" @click="closeWindow" title="Close">
            <svg width="10" height="10" viewBox="0 0 12 12"><path d="M2 2 L10 10 M10 2 L2 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Main -->
    <main class="main">
      <!-- Loading State -->
      <div v-if="isLoading" class="loading-container fade-in">
        <div class="loading-content">
          <SkeletonLoader :count="8" />
        </div>
      </div>

      <!-- Empty State -->
      <div v-else-if="files.length === 0" class="empty fade-in">
        <div class="empty-content liquid-glass">
          <div class="empty-icon">📁</div>
          <h3>{{ t('empty.title') }}</h3>
          <p>{{ t('empty.subtitle') }}</p>
          <button class="btn btn-primary btn-lg" @click="refreshFiles">
            {{ t('empty.loadButton') }}
          </button>
        </div>
      </div>

      <!-- Content -->
      <div v-else class="content fade-in">
        <!-- Left: Controls -->
        <div class="panel controls-panel">
          <h3 class="panel-title">
            <span class="icon">⚙️</span> {{ t('controls.title') }}
          </h3>

          <!-- 优化目标 -->
          <div class="form-group">
            <label>{{ t('controls.optimizeMode') }}</label>
            <div class="select-wrapper">
              <select v-model="optimizeMode" class="select">
                <option value="balanced">{{ t('controls.balanced') }}</option>
                <option value="quality">{{ t('controls.quality') }}</option>
                <option value="size">{{ t('controls.size') }}</option>
              </select>
            </div>
          </div>

          <!-- 🖼️ 图像输出格式 (仅图像模式) -->
          <div v-if="isImageMode" class="form-group slide-in">
            <label>{{ t('image.format') }}</label>
            <div class="select-wrapper">
              <select v-model="outputFormat" class="select">
                <option value="auto">{{ t('image.formatAuto') }}</option>
                <option value="disabled">🚫 Disabled (Original Format)</option>
                <option value="avif">AVIF</option>
                <option value="jxl">JXL</option>
                <option value="webp">WebP</option>
                <option value="heic">HEIC</option>
                <option value="png">PNG</option>
              </select>
            </div>
          </div>

          <!-- 🎬 视频输出格式 (仅视频模式) -->
          <template v-if="isVideoMode">
            <div class="form-group slide-in">
              <label>{{ t('video.codec') }}</label>
              <div class="select-wrapper">
                <select v-model="videoCodec" class="select">
                  <option value="h265">H.265/HEVC (Recommended)</option>
                  <option value="h264">H.264/AVC</option>
                  <option value="av1">AV1 (Most Efficient)</option>
                  <option value="vp9">VP9</option>
                </select>
              </div>
            </div>

            <div class="form-group slide-in">
              <label>{{ t('video.container') }}</label>
              <div class="select-wrapper">
                <select v-model="videoContainer" class="select">
                  <option value="mp4">MP4 (Recommended)</option>
                  <option value="mov">MOV</option>
                  <option value="webm">WebM</option>
                  <option value="mkv">MKV</option>
                </select>
              </div>
            </div>
          </template>

          <!-- 🖼️ 图像 AI 功能 (仅图像模式) -->
          <details v-if="isImageMode" class="details glass-panel" open>
            <summary>{{ t('image.title') }}</summary>
            <div class="checkbox-group">
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableAIPrediction">
                <span>{{ t('image.aiPrediction') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableFileValidation">
                <span>{{ t('image.fileValidation') }} <span class="badge badge-primary">Exp</span></span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableSSIM">
                <span>{{ t('image.ssimValidation') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableGPU">
                <span>{{ t('image.gpuAccel') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enablePreprocess">
                <span>{{ t('image.preprocess') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableFormatCorrection">
                <span>{{ t('image.formatCorrection') }} <span class="badge badge-primary">Exp</span></span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>{{ t('image.animationToVideo') }}</span>
              </label>
            </div>
          </details>

          <!-- 📦 混合模式提示 -->
          <div v-if="isMixedMode" class="mixed-mode-notice glass slide-in">
            <div class="notice-header">
              <span class="notice-icon">📦</span>
              <strong>{{ t('mixedMode.title') }}</strong>
            </div>
            <div class="notice-body">
              <p>{{ t('mixedMode.description') }}</p>
              <div class="file-groups">
                <div class="group-item">
                  <span class="group-icon">🖼️</span>
                  <span>{{ t('mixedMode.images', { count: selectedFiles.filter(f => {
                    const ext = (f.ext || '').toLowerCase()
                    return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff', 'tif', 'apng'].includes(ext)
                  }).length }) }}</span>
                </div>
                <div class="group-item">
                  <span class="group-icon">🎬</span>
                  <span>{{ t('mixedMode.videos', { count: selectedFiles.filter(f => {
                    const ext = (f.ext || '').toLowerCase()
                    return ['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', 'mpg', 'mpeg'].includes(ext)
                  }).length }) }}</span>
                </div>
              </div>
              <p class="notice-tip">{{ t('mixedMode.hint') }}</p>
            </div>
          </div>

          <!-- 🎬 视频 AI 功能 (仅视频模式) -->
          <details v-if="isVideoMode" class="details glass-panel" open>
            <summary>{{ t('video.title') }}</summary>
            <div class="checkbox-group">
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableVideoCodecRecommendation" checked>
                <span>🤖 {{ t('video.codecRecommendation') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>🎬 {{ t('video.animationToVideo') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableSceneDetection">
                <span>🎞️ {{ t('video.sceneDetection') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableVMAF">
                <span>📊 {{ t('video.vmafValidation') }}</span>
              </label>
              <label class="checkbox-wrapper">
                <input type="checkbox" v-model="enableTwoPass">
                <span>🔄 {{ t('video.twoPass') }}</span>
              </label>
            </div>
          </details>

          <!-- 🔍 AI日志窗口 / 功能说明 切换区域 -->
          <div class="log-window-container glass">
            <!-- 切换标签 -->
            <div class="log-tabs">
              <button 
                class="log-tab" 
                :class="{ active: showAILog }"
                @click="showAILog = true"
              >
                {{ t('log.aiLog') }}
              </button>
              <button 
                class="log-tab" 
                :class="{ active: !showAILog }"
                @click="showAILog = false"
              >
                {{ t('log.features') }}
              </button>
            </div>

            <!-- AI日志窗口 -->
            <div v-if="showAILog" class="log-window">
              <div class="log-content" ref="logContent" @wheel.stop>
                <div v-for="(log, index) in aiLogs" :key="index" class="log-entry" :class="log.type">
                  <span class="log-time">{{ log.time }}</span>
                  <span class="log-icon">{{ log.icon }}</span>
                  <span class="log-message">{{ log.message }}</span>
                </div>
                <div v-if="aiLogs.length === 0" class="log-empty">
                  <span class="pulse-dot"></span>
                  Waiting for tasks...
                </div>
              </div>
            </div>

            <!-- 功能说明面板 -->
            <div v-else class="features-panel fade-in">
              <!-- 自动处理提示 -->
              <div class="auto-hints">
                <div class="hint-item">
                  <span class="hint-icon">✨</span>
                  {{ t('features.validation') }}
                </div>
                <div class="hint-item">
                  <span class="hint-icon">🔄</span>
                  {{ t('features.xmpMerge') }}
                </div>
                <div class="hint-item">
                  <span class="hint-icon">📝</span>
                  {{ t('features.filenameNorm') }}
                </div>
              </div>

              <!-- 元数据保留提示 -->
              <div class="metadata-notice">
                <strong>{{ t('features.metadataTitle') }}</strong>
                <div class="metadata-items">
                  <span class="meta-tag">Exif</span>
                  <span class="meta-tag">XMP</span>
                  <span class="meta-tag">ICC</span>
                  <span class="meta-tag">Time</span>
                </div>
              </div>
            </div>
          </div>

          <!-- 转换按钮 -->
          <button 
            class="btn btn-primary btn-block btn-lg" 
            @click="startConvert"
            :disabled="selectedCount === 0 || processing"
          >
            <span v-if="processing" class="spinner"></span>
            {{ processing ? t('button.processing') : t('button.startConvert') }}
          </button>

          <!-- Progress Bar -->
          <div v-if="processing || progress > 0" class="progress-container fade-in">
            <SmartProgressBar 
              :progress="progress" 
              :status="processing ? 'processing' : (progress >= 100 ? 'success' : 'idle')"
              :statusText="progressText"
              :smartSmoothing="true"
            />
          </div>
        </div>

        <!-- Right: File List -->
        <div class="panel files-panel">
          <div class="panel-header">
            <h3 class="panel-title">📁 {{ t('fileList.title') }}</h3>
            <div class="header-right-badges">
              <!-- 🔥 文件类型指示器 -->
              <span v-if="isVideoMode" class="type-badge video">🎬 {{ t('mode.video') }}</span>
              <span v-else-if="isImageMode" class="type-badge image">🖼️ {{ t('mode.image') }}</span>
              <span v-else-if="isMixedMode" class="type-badge mixed">📦 {{ t('mode.mixed') }}</span>
              <span class="file-count">{{ selectedCount }}/{{ files.length }}</span>
            </div>
          </div>

          <!-- 🔥 批量操作栏 -->
          <div class="batch-actions">
            <button class="batch-btn" @click="selectAll" :title="t('fileList.selectAll')">
              ✓ All
            </button>
            <button class="batch-btn" @click="selectNone" :title="t('fileList.selectNone')">
              ✗ None
            </button>
            <button class="batch-btn" @click="selectInvert" :title="t('fileList.selectInvert')">
              ⇄ Invert
            </button>
            <div class="divider"></div>
            <button class="batch-btn" @click="selectImages" :title="t('fileList.selectImages')">
              🖼️ Img
            </button>
            <button class="batch-btn" @click="selectVideos" :title="t('fileList.selectVideos')">
              🎬 Vid
            </button>
          </div>

          <div class="file-list">
            <label 
              v-for="file in files" 
              :key="file.id"
              class="file-item"
              :class="{ selected: file.selected }"
            >
              <div class="checkbox-container">
                <input type="checkbox" v-model="file.selected">
              </div>
              <!-- 🔥 缩略图容器：emoji背景 + 图片覆盖 -->
              <div class="file-thumb-container">
                <!-- Emoji占位符 - 始终显示 -->
                <div class="file-thumb-placeholder">
                  <span class="file-emoji">{{ getFileEmoji(file) }}</span>
                </div>
                <!-- 缩略图 - 如果有则覆盖在上面 -->
                <img 
                  v-if="file.thumbnail" 
                  :src="file.thumbnail" 
                  class="file-thumb file-thumb-overlay" 
                  alt=""
                  @error="handleImageError"
                >
              </div>
              <div class="file-info">
                <div class="file-name">{{ file.name }}</div>
                <div class="file-meta">
                  <span class="meta-pill">{{ (file.ext || '').toUpperCase() }}</span>
                  <span class="meta-text">{{ formatSize(file.size) }} · {{ file.width }}×{{ file.height }}</span>
                </div>
              </div>
            </label>
          </div>
        </div>
      </div>
    </main>

    <!-- Help Modal -->
    <div v-if="showHelp" class="modal-overlay fade-in" @click="showHelp = false">
      <div class="modal glass" @click.stop>
        <div class="modal-header">
          <h2>💡 {{ t('help.title') }}</h2>
          <button class="modal-close" @click="showHelp = false">✕</button>
        </div>
        <div class="modal-body">
          <section>
            <h3>🎯 {{ t('help.vision') }}</h3>
            <p><strong>{{ t('help.visionDesc') }}</strong></p>
            <p><strong>{{ t('help.modernFormats') }}</strong></p>
            <p><strong>{{ t('help.smartUpgrade') }}</strong></p>
          </section>
          
          <section>
            <h3>📦 {{ t('help.metadataTitle') }}</h3>
            <div class="feature-grid">
              <div class="feature-item">Exif</div>
              <div class="feature-item">XMP</div>
              <div class="feature-item">ICC Profile</div>
              <div class="feature-item">Eagle Tags</div>
            </div>
          </section>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, inject } from 'vue'
import { useRustCLI } from './composables/useRustCLI'
import { useEagleAPI } from './composables/useEagleAPI'
import { logger, LOG_KEYS } from './utils/logger'
import SmartProgressBar from './components/SmartProgressBar.vue'
import SkeletonLoader from './components/SkeletonLoader.vue'
import LiquidFilter from './components/LiquidFilter.vue'

// Use global i18n instance from main.js
const i18n = inject('i18n')
const { t, setLocale, locale } = i18n

const rustCLI = useRustCLI()
const eagleAPI = useEagleAPI()

const isDark = ref(true)
const showHelp = ref(false)
const files = ref([])
const isLoading = ref(true) // 🔥 加载状态
const optimizeMode = ref('balanced')

// 🖼️ 图像相关
const outputFormat = ref('auto')
const enableAIPrediction = ref(true)
const enableFileValidation = ref(true)
const enableSSIM = ref(false)
const enableGPU = ref(true)
const enablePreprocess = ref(true)
const enableFormatCorrection = ref(false)

// 🎬 视频相关
const videoCodec = ref('h265')
const videoContainer = ref('mp4')
const enableVideoCodecRecommendation = ref(true)
const enableVideoForAnimation = ref(true)
const enableSceneDetection = ref(false)
const enableVMAF = ref(false)
const enableTwoPass = ref(false)

const processing = ref(false)
const progress = ref(0)
const progressText = ref('')

const selectedCount = computed(() => files.value.filter(f => f.selected).length)

// 🔥 检测文件类型
const selectedFiles = computed(() => files.value.filter(f => f.selected))

// 🔍 AI日志系统
const showAILog = ref(true) // 默认显示AI日志
const aiLogs = ref([])
const logContent = ref(null)

// 🔍 日志系统性能优化
let scrollTimer = null  // 防抖滚动定时器

// 添加日志
const addLog = (message, type = 'info', icon = '📝') => {
  const now = new Date()
  const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`
  
  aiLogs.value.push({
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
    if (logContent.value) {
      logContent.value.scrollTop = logContent.value.scrollHeight
    }
    scrollTimer = null
  }, 50)  // 增加到50ms，减少滚动频率
}

// 清空日志
const clearLogs = () => {
  aiLogs.value = []
  // 🔥 清理定时器
  if (scrollTimer) {
    clearTimeout(scrollTimer)
    scrollTimer = null
  }
}

const isVideoMode = computed(() => {
  if (selectedFiles.value.length === 0) return false
  const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', 'mpg', 'mpeg']
  return selectedFiles.value.every(f => {
    const ext = (f.ext || '').toLowerCase()
    return videoExts.includes(ext)
  })
})

const isImageMode = computed(() => {
  if (selectedFiles.value.length === 0) return false
  const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff', 'tif', 'apng']
  return selectedFiles.value.every(f => {
    const ext = (f.ext || '').toLowerCase()
    return imageExts.includes(ext)
  })
})

const isMixedMode = computed(() => {
  return selectedFiles.value.length > 0 && !isVideoMode.value && !isImageMode.value
})



const toggleTheme = () => {
  isDark.value = !isDark.value
  document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
}

const toggleLanguage = () => {
  const newLocale = locale.value === 'zh_CN' ? 'en' : 'zh_CN'
  setLocale(newLocale)
  logger.info(LOG_KEYS.UI_CLICK, 'Language switched', { locale: newLocale })
}

const refreshFiles = async () => {
  isLoading.value = true
  try {
    const items = await eagleAPI.getSelectedItems()
    // 模拟网络延迟以展示骨架屏效果 (仅开发时)
    // await new Promise(resolve => setTimeout(resolve, 800))
    files.value = items.map(item => ({ ...item, selected: false }))
  } catch (err) {
    logger.error(LOG_KEYS.FILE_LOAD_ERROR, 'Failed to load files', { error: err.message })
  } finally {
    isLoading.value = false
  }
}

// 🔥 批量选择功能
const selectAll = () => {
  files.value.forEach(f => f.selected = true)
}

const selectNone = () => {
  files.value.forEach(f => f.selected = false)
}

const selectInvert = () => {
  files.value.forEach(f => f.selected = !f.selected)
}

const selectImages = () => {
  const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff', 'tif', 'apng']
  files.value.forEach(f => {
    const ext = (f.ext || '').toLowerCase()
    f.selected = imageExts.includes(ext)
  })
}

const selectVideos = () => {
  const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', 'mpg', 'mpeg']
  files.value.forEach(f => {
    const ext = (f.ext || '').toLowerCase()
    f.selected = videoExts.includes(ext)
  })
}



const formatSize = (bytes) => {
  return (bytes / 1024 / 1024).toFixed(2) + ' MB'
}

// 🔥 根据文件类型返回emoji
const getFileEmoji = (file) => {
  const ext = (file.ext || '').toLowerCase()
  
  // 图像格式
  const imageFormats = {
    'jxl': '🎨',
    'avif': '🖼️',
    'webp': '🌐',
    'heic': '🍎',
    'heif': '🍎',
    'png': '🖼️',
    'jpg': '📷',
    'jpeg': '📷',
    'gif': '🎞️',
    'bmp': '🖼️',
    'tiff': '🖼️',
    'tif': '🖼️',
    'apng': '🎞️',
    'svg': '🎨'
  }
  
  // 视频格式
  const videoFormats = {
    'mp4': '🎬',
    'mov': '🎥',
    'avi': '📹',
    'mkv': '🎞️',
    'webm': '🌐',
    'm4v': '📱',
    'flv': '📺',
    'wmv': '🎬',
    'mpg': '📹',
    'mpeg': '📹'
  }
  
  if (imageFormats[ext]) return imageFormats[ext]
  if (videoFormats[ext]) return videoFormats[ext]
  
  return '📄'
}

// 🔥 处理图片加载失败
const handleImageError = (event) => {
  event.target.style.display = 'none'
}

const startConvert = async () => {
  const selected = files.value.filter(f => f.selected)
  if (selected.length === 0) return

  processing.value = true
  progress.value = 0
  
  // 🔍 Clear logs and switch to log view
  clearLogs()
  showAILog.value = true
  
  // 🔍 Add initial logs
  addLog(`开始处理 ${selected.length} 个文件`, 'info', '🚀')
  addLog(`优化模式: ${optimizeMode.value}`, 'info', '⚙️')
  
  if (selected.length > 0) {
    const firstFile = selected[0]
    addLog(`文件: ${firstFile.name} (${formatSize(firstFile.size)})`, 'info', '📄')
    addLog(`尺寸: ${firstFile.width}×${firstFile.height}`, 'info', '📐')
  }

  try {
    let results = []
    
    // 🔥 混合模式 - 自动分组处理
    if (isMixedMode.value) {
      logger.info(LOG_KEYS.CONVERT_START, 'Mixed mode - auto grouping', {})
      
      const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'avif', 'jxl', 'heic', 'heif', 'bmp', 'tiff', 'tif', 'apng']
      const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'flv', 'wmv', 'm4v', 'mpg', 'mpeg']
      
      const images = selected.filter(f => {
        const ext = (f.ext || '').toLowerCase()
        return imageExts.includes(ext)
      })
      const videos = selected.filter(f => {
        const ext = (f.ext || '').toLowerCase()
        return videoExts.includes(ext)
      })
      
      logger.info(LOG_KEYS.CONVERT_START, 'File grouping', { images: images.length, videos: videos.length })
      
      let processed = 0
      const total = selected.length
      
      // 先处理图像
      if (images.length > 0) {
        progressText.value = t('progress.processingImages', { count: images.length })
        
        for (let i = 0; i < images.length; i++) {
          const file = images[i]
          try {
            processed++
            progress.value = Math.round((processed / total) * 100)
            progressText.value = t('progress.imageProgress', { current: i + 1, total: images.length, filename: file.name })
            
            // 🔍 Log processing start
            addLog(`[${i + 1}/${images.length}] 处理: ${file.name}`, 'info', '⚙️')
            
            const result = await rustCLI.convert({
              inputPath: file.path,
              outputPath: file.path.replace(/\.[^.]+$/, `.${outputFormat.value === 'auto' || outputFormat.value === 'disabled' ? 'avif' : outputFormat.value}`),
              format: outputFormat.value === 'auto' ? null : (outputFormat.value === 'disabled' ? null : outputFormat.value),
              disableFormatChange: outputFormat.value === 'disabled',
              useAI: enableAIPrediction.value,
              optimizeMode: optimizeMode.value,
              enableFileValidation: enableFileValidation.value,
              enableSSIM: enableSSIM.value,
              enableGPU: enableGPU.value,
              enablePreprocess: enablePreprocess.value,
              enableFormatCorrection: enableFormatCorrection.value,
              enableVideoForAnimation: enableVideoForAnimation.value
            })
            
            results.push({ file: file.name, success: true, result })
          } catch (err) {
            results.push({ file: file.name, success: false, error: err.message })
          }
        }
      }
      
      // 再处理视频
      if (videos.length > 0) {
        progressText.value = t('progress.processingVideos', { count: videos.length })
        
        for (let i = 0; i < videos.length; i++) {
          const file = videos[i]
          try {
            processed++
            progress.value = Math.round((processed / total) * 100)
            progressText.value = t('progress.videoProgress', { current: i + 1, total: videos.length, filename: file.name })
            
            const result = await rustCLI.convertVideo({
              inputPath: file.path,
              outputPath: file.path.replace(/\.[^.]+$/, `.${videoContainer.value}`),
              codec: videoCodec.value,
              container: videoContainer.value,
              useAI: enableAIPrediction.value,
              optimizeMode: optimizeMode.value,
              enableGPU: enableGPU.value,
              enableVideoForAnimation: enableVideoForAnimation.value,
              enableSceneDetection: enableSceneDetection.value,
              enableVMAF: enableVMAF.value,
              enableTwoPass: enableTwoPass.value
            })
            
            results.push({ file: file.name, success: true, result })
          } catch (err) {
            results.push({ file: file.name, success: false, error: err.message })
          }
        }
      }
    }
    // 🎬 纯视频模式
    else if (isVideoMode.value) {
      logger.info(LOG_KEYS.CONVERT_START, 'Video conversion mode', {})
      
      for (let i = 0; i < selected.length; i++) {
        const file = selected[i]
        
        try {
          progress.value = Math.round(((i + 1) / selected.length) * 100)
          progressText.value = t('progress.videoProgress', { current: i + 1, total: selected.length, filename: file.name })
          
          const result = await rustCLI.convertVideo({
            inputPath: file.path,
            outputPath: file.path.replace(/\.[^.]+$/, `.${videoContainer.value}`),
            codec: videoCodec.value,
            container: videoContainer.value,
            useAI: enableAIPrediction.value,
            optimizeMode: optimizeMode.value,
            enableGPU: enableGPU.value,
            enableVideoForAnimation: enableVideoForAnimation.value,
            enableSceneDetection: enableSceneDetection.value,
            enableVMAF: enableVMAF.value,
            enableTwoPass: enableTwoPass.value
          })
          
          results.push({ file: file.name, success: true, result })
        } catch (err) {
          results.push({ file: file.name, success: false, error: err.message })
        }
      }
    }
    // 🖼️ 纯图像模式
    else if (isImageMode.value) {
      logger.info(LOG_KEYS.CONVERT_START, 'Image conversion mode', {})
      results = await rustCLI.batchConvert(
        selected,
        {
          format: outputFormat.value === 'auto' ? null : (outputFormat.value === 'disabled' ? null : outputFormat.value),
          disableFormatChange: outputFormat.value === 'disabled',
          useAI: enableAIPrediction.value,
          optimizeMode: optimizeMode.value,
          enableFileValidation: enableFileValidation.value,
          enableSSIM: enableSSIM.value,
          enableGPU: enableGPU.value,
          enablePreprocess: enablePreprocess.value,
          enableFormatCorrection: enableFormatCorrection.value,
          enableVideoForAnimation: enableVideoForAnimation.value
        },
        (info) => {
          progress.value = info.percentage
          progressText.value = t('progress.imageProgress', { current: info.current, total: info.total, filename: info.file })
        }
      )
    }

    const successCount = results.filter(r => r.success).length
    const failedCount = results.length - successCount
    progressText.value = t('progress.success', { success: successCount, total: results.length })
    
    // 🔍 Add completion logs with real data
    addLog('─────────────────────────', 'info', '')
    
    if (successCount === results.length) {
      addLog(`✅ 全部完成！成功: ${successCount}/${results.length}`, 'success', '✅')
    } else if (successCount > 0) {
      addLog(`⚠️ 部分完成！成功: ${successCount}, 失败: ${failedCount}`, 'warning', '⚠️')
    } else {
      addLog(`❌ 全部失败！失败: ${failedCount}/${results.length}`, 'error', '❌')
    }
    
    // 显示每个文件的结果
    results.forEach(r => {
      if (r.success) {
        addLog(`  ✓ ${r.file}`, 'success', '')
        // 如果result包含实际数据，显示它
        if (r.result && typeof r.result === 'object') {
          if (r.result.outputSize) {
            const ratio = Math.round((1 - r.result.outputSize / r.result.inputSize) * 100)
            addLog(`    压缩: ${formatSize(r.result.inputSize)} → ${formatSize(r.result.outputSize)} (${ratio}%)`, 'info', '📊')
          }
        }
      } else {
        addLog(`  ✗ ${r.file}: ${r.error}`, 'error', '')
      }
    })
    
    // AI决策信息
    addLog('─────────────────────────', 'info', '')
    const targetFormat = outputFormat.value === 'auto' ? 'AVIF' : outputFormat.value.toUpperCase()
    addLog(t('log.targetFormat', { format: targetFormat }), 'info', '🎯')
    addLog(t('log.optimizeMode', { mode: optimizeMode.value }), 'info', '⚙️')
    addLog(t('log.aiPrediction', { status: enableAIPrediction.value ? '✅ ' + t('log.enabled') : '❌ ' + t('log.disabled') }), 'info', '🤖')
    
    setTimeout(() => {
      processing.value = false
      refreshFiles()
    }, 2000)
  } catch (err) {
    logger.error(LOG_KEYS.CONVERT_ERROR, 'Conversion failed', { error: err.message })
    progressText.value = 'Conversion failed: ' + err.message
    processing.value = false
  }
}

// 🔥 Window control methods (Eagle API - FIXED)
// Ref: format-vue working implementation
const isMaximized = ref(false)

const minimizeWindow = () => {
  try {
    logger.info(LOG_KEYS.UI_CLICK, 'Minimizing window')
    if (window.eagle && window.eagle.window && typeof window.eagle.window.minimize === 'function') {
      window.eagle.window.minimize()
      logger.info(LOG_KEYS.UI_CLICK, 'Window minimized')
    } else if (window.eagle && window.eagle.app && typeof window.eagle.app.minimize === 'function') {
      window.eagle.app.minimize()
      logger.info(LOG_KEYS.UI_CLICK, 'Window minimized (via app)')
    } else {
      logger.error(LOG_KEYS.APP_ERROR, 'Minimize method not found')
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_ERROR, 'Failed to minimize window', { error: error.message })
  }
}

const maximizeWindow = () => {
  try {
    logger.info(LOG_KEYS.UI_CLICK, 'Toggling maximize', { currentState: isMaximized.value })
    
    if (!window.eagle || !window.eagle.window) {
      logger.error(LOG_KEYS.APP_ERROR, 'Eagle window API not available')
      return
    }
    
    // Toggle between maximize and restore
    if (isMaximized.value) {
      // Currently maximized, restore to normal
      if (typeof window.eagle.window.unmaximize === 'function') {
        window.eagle.window.unmaximize()
        isMaximized.value = false
        logger.info(LOG_KEYS.UI_CLICK, 'Window unmaximized')
      } else if (typeof window.eagle.window.restore === 'function') {
        window.eagle.window.restore()
        isMaximized.value = false
        logger.info(LOG_KEYS.UI_CLICK, 'Window restored')
      } else {
        logger.error(LOG_KEYS.APP_ERROR, 'Unmaximize method not found')
      }
    } else {
      // Currently normal, maximize
      if (typeof window.eagle.window.maximize === 'function') {
        window.eagle.window.maximize()
        isMaximized.value = true
        logger.info(LOG_KEYS.UI_CLICK, 'Window maximized')
      } else {
        logger.error(LOG_KEYS.APP_ERROR, 'Maximize method not found')
      }
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_ERROR, 'Failed to toggle maximize', { error: error.message })
  }
}

const closeWindow = () => {
  try {
    logger.info(LOG_KEYS.UI_CLICK, 'Closing window')
    
    if (!window.eagle) {
      logger.error(LOG_KEYS.APP_ERROR, 'Eagle API not available')
      return
    }
    
    // Try different API methods
    if (window.eagle.window && typeof window.eagle.window.close === 'function') {
      window.eagle.window.close()
      logger.info(LOG_KEYS.UI_CLICK, 'Window closed')
    } else if (window.eagle.app && typeof window.eagle.app.close === 'function') {
      window.eagle.app.close()
      logger.info(LOG_KEYS.UI_CLICK, 'Window closed (via app)')
    } else if (window.close && typeof window.close === 'function') {
      window.close()
      logger.info(LOG_KEYS.UI_CLICK, 'Window closed (via window.close)')
    } else {
      logger.error(LOG_KEYS.APP_ERROR, 'Close method not found')
    }
  } catch (error) {
    logger.error(LOG_KEYS.APP_ERROR, 'Failed to close window', { error: error.message })
  }
}

onMounted(() => {
  eagleAPI.detect()
  rustCLI.init()
  refreshFiles()
})
</script>

<style scoped>
@import './styles/variables.css';
@import './styles/liquid.css';

.app {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #0f172a;
  color: var(--color-text-primary);
  overflow: hidden;
  font-family: 'Inter', sans-serif;
  position: relative;
}

/* 🔥 动态极光背景 (让玻璃显形的关键) */
.app::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: conic-gradient(
    from 0deg at 50% 50%,
    #0f172a 0deg,
    #1e1b4b 60deg, /* Indigo 950 */
    #312e81 120deg, /* Indigo 900 */
    #4c1d95 180deg, /* Violet 900 */
    #312e81 240deg,
    #1e1b4b 300deg,
    #0f172a 360deg
  );
  animation: bg-spin 120s linear infinite;
  z-index: 0;
  opacity: 0.8;
  pointer-events: none;
}

@keyframes bg-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Header Styles */
.header {
  height: 60px;
  padding: 0 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 10; /* 提升层级 */
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  position: relative;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.logo-container {
  position: relative;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-primary), var(--color-ai-gradient-2));
  border-radius: 10px;
  box-shadow: 0 4px 12px var(--color-ai-gradient-1-20);
}

.logo-icon {
  font-size: 20px;
  z-index: 2;
}

.logo-glow {
  position: absolute;
  inset: -2px;
  background: inherit;
  filter: blur(8px);
  opacity: 0.5;
  z-index: 1;
}

.title-group h1 {
  font-size: 16px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(to right, var(--color-text-primary), var(--color-primary));
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.title-group p {
  font-size: 11px;
  color: var(--color-text-secondary);
  margin: 2px 0 0;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.action-group {
  display: flex;
  gap: 8px;
  padding-right: 16px;
  border-right: 1px solid var(--color-border-primary);
}

.icon-btn {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  color: var(--color-text-secondary);
}

.icon-btn:hover {
  background: var(--color-bg-hover);
  color: var(--color-primary);
}

.text-icon {
  font-size: 12px;
  font-weight: 700;
}

.window-controls {
  display: flex;
  gap: 8px;
}

.window-btn {
  width: 24px;
  height: 24px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.window-btn:hover {
  background: var(--color-bg-hover);
  color: var(--color-text-primary);
}

.window-btn.close:hover {
  background: var(--color-negative);
  color: white;
}

/* Main Content */
.main {
  flex: 1;
  overflow: hidden;
  position: relative;
  z-index: 1; /* 确保在背景之上 */
}

.content {
  display: grid;
  grid-template-columns: 360px 1fr;
  height: 100%;
}

.panel {
  padding: 20px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.controls-panel {
  background: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border-primary);
}

.files-panel {
  background: var(--color-bg-primary);
}

.panel-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
}

/* Form Elements */
.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.select-wrapper {
  position: relative;
}

.select {
  width: 100%;
  padding: 10px 12px;
  border-radius: 8px;
  border: 1px solid var(--color-border-secondary);
  background: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-size: 13px;
  appearance: none;
  cursor: pointer;
  transition: all 0.2s;
}

.select:hover {
  border-color: var(--color-primary);
}

.select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-dim);
}

/* Checkbox Group */
.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-wrapper {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.checkbox-wrapper:hover {
  background: var(--color-bg-hover);
}

.checkbox-wrapper input[type="checkbox"] {
  appearance: none;
  width: 18px;
  height: 18px;
  border: 2px solid var(--color-border-secondary);
  border-radius: 5px;
  background: var(--color-bg-primary);
  cursor: pointer;
  position: relative;
  transition: all 0.2s;
}

.checkbox-wrapper input[type="checkbox"]:checked {
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.checkbox-wrapper input[type="checkbox"]:checked::after {
  content: '';
  position: absolute;
  left: 5px;
  top: 2px;
  width: 4px;
  height: 8px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.checkbox-wrapper span {
  font-size: 13px;
  color: var(--color-text-primary);
}

/* Details Panel */
.details {
  border: 1px solid var(--color-border-primary);
  border-radius: 12px;
  overflow: hidden;
  background-image: var(--gradient-dark);
  color: var(--color-text-primary);
}

/* Modern Glassmorphism Utilities */
.glass {
  /* Legacy glass support, now using liquid-glass */
  background: var(--glass-bg);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid var(--glass-border);
  box-shadow: var(--glass-shadow);
}

.glass-panel {
  /* Enhanced with liquid properties */
  background: var(--color-bg-card);
  backdrop-filter: blur(20px);
  border: 1px solid var(--glass-border);
  border-radius: 16px;
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
  position: relative;
  overflow: hidden;
}

.glass-panel::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
  opacity: 0.5;
}

.glass-panel:hover {
  border-color: var(--color-border-glow);
  box-shadow: var(--glow-primary);
  transform: translateY(-2px) scale(1.01);
}

/* Header Styles */
.header {
  height: 64px;
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 100;
  position: relative;
}

.logo-container {
  position: relative;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-primary);
  border-radius: 12px;
  margin-right: 16px;
  box-shadow: var(--glow-primary);
}

.logo-icon {
  font-size: 24px;
  z-index: 2;
}

.logo-glow {
  position: absolute;
  inset: -4px;
  background: var(--gradient-primary);
  filter: blur(12px);
  opacity: 0.6;
  z-index: 1;
  animation: pulse-glow 3s infinite;
}

@keyframes pulse-glow {
  0%, 100% { opacity: 0.6; transform: scale(1); }
  50% { opacity: 0.8; transform: scale(1.1); }
}

.title-group h1 {
  font-size: 18px;
  font-weight: 700;
  background: linear-gradient(to right, #fff, #a5b4fc);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  margin: 0;
}

.title-group p {
  font-size: 11px;
  color: var(--color-text-secondary);
  margin: 2px 0 0 0;
  letter-spacing: 0.5px;
}

/* Button Styles - Radical Liquid Update */
.btn {
  position: relative;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(10px);
}

.btn-primary {
  background: rgba(99, 102, 241, 0.2);
  color: white;
  box-shadow: 
    0 4px 15px rgba(99, 102, 241, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.4);
}

.btn-primary::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.6), rgba(139, 92, 246, 0.6));
  z-index: -1;
  opacity: 0.8;
  transition: opacity 0.3s;
}

.btn-primary:hover {
  transform: translateY(-2px) scale(1.02);
  box-shadow: 
    0 8px 25px rgba(99, 102, 241, 0.5),
    inset 0 0 20px rgba(255, 255, 255, 0.4);
  border-color: rgba(255, 255, 255, 0.8);
}

.btn-primary:hover::before {
  opacity: 1;
  animation: liquid-pulse 2s infinite alternate;
}

.icon-btn {
  width: 36px;
  height: 36px;
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(255, 255, 255, 0.05);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s;
  color: var(--color-text-secondary);
  backdrop-filter: blur(10px);
}

.icon-btn:hover {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border-color: rgba(255, 255, 255, 0.5);
  transform: scale(1.1);
  box-shadow: 0 0 15px rgba(255, 255, 255, 0.2);
}

/* Window Controls */
.window-controls {
  display: flex;
  gap: 8px;
  margin-left: 16px;
  padding-left: 16px;
  border-left: 1px solid var(--glass-border);
}

.window-btn {
  width: 28px;
  height: 28px;
  border-radius: 8px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
}

.window-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.window-btn.close:hover {
  background: var(--color-negative);
  color: white;
}

/* Main Layout */
.main {
  flex: 1;
  display: flex;
  overflow: hidden;
  position: relative;
}

.content {
  flex: 1;
  display: flex;
  padding: 20px;
  gap: 20px;
  height: 100%;
}

.panel {
  background: var(--color-bg-card);
  border: 1px solid var(--glass-border);
  border-radius: 16px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  backdrop-filter: blur(10px);
}

.controls-panel {
  width: 320px;
  padding: 20px;
  gap: 20px;
}

.preview-panel {
  flex: 1;
  padding: 20px;
}

.panel-title {
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 16px 0;
  display: flex;
  align-items: center;
  gap: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Form Elements - Liquid Style */
.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  font-size: 12px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.8);
  margin-bottom: 8px;
  text-shadow: 0 0 10px rgba(0,0,0,0.5);
}

.select-wrapper {
  position: relative;
}

.select {
  width: 100%;
  padding: 12px 16px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  color: white;
  font-size: 13px;
  appearance: none;
  cursor: pointer;
  transition: all 0.3s;
  backdrop-filter: blur(10px);
  box-shadow: inset 0 2px 5px rgba(0,0,0,0.2);
}

.select:hover {
  border-color: rgba(255, 255, 255, 0.3);
  background: rgba(0, 0, 0, 0.4);
  box-shadow: 0 0 15px rgba(255, 255, 255, 0.1);
}

.select:focus {
  outline: none;
  border-color: #8b5cf6;
  box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.3), 0 0 20px rgba(139, 92, 246, 0.2);
  background: rgba(0, 0, 0, 0.5);
}

.details {
  border: 1px solid var(--glass-border);
  border-radius: 8px;
  overflow: hidden;
  background: rgba(0, 0, 0, 0.1);
}

.details summary {
  padding: 12px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  background: rgba(255, 255, 255, 0.02);
  user-select: none;
  color: var(--color-text-primary);
  transition: background 0.2s;
}

.details summary:hover {
  background: rgba(255, 255, 255, 0.05);
}

.details .checkbox-group {
  padding: 12px;
  background: rgba(0, 0, 0, 0.2);
}

/* Log Window */
.log-window-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  overflow: hidden;
  min-height: 200px;
}

.log-tabs {
  display: flex;
  border-bottom: 1px solid var(--glass-border);
  background: rgba(0,0,0,0.02);
}

.log-tab {
  flex: 1;
  padding: 8px;
  border: none;
  background: transparent;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.log-tab.active {
  color: var(--color-primary);
  background: var(--glass-bg);
  box-shadow: inset 0 -2px 0 var(--color-primary);
}

.log-content {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  background: rgba(0,0,0,0.02);
}

.log-entry {
  display: flex;
  gap: 8px;
  margin-bottom: 6px;
  line-height: 1.4;
  padding: 4px 8px;
  border-radius: 4px;
}

.log-entry:hover {
  background: rgba(0,0,0,0.05);
}

.log-time {
  color: var(--color-text-secondary);
  opacity: 0.7;
}

.log-entry.error { color: var(--color-negative); }
.log-entry.success { color: var(--color-positive); }
.log-entry.warning { color: var(--color-warning); }

/* File List */
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.header-right-badges {
  display: flex;
  align-items: center;
  gap: 8px;
}

.type-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 12px;
  background: var(--color-bg-hover);
  color: var(--color-text-secondary);
  font-weight: 600;
}

.batch-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  overflow-x: auto;
  padding-bottom: 4px;
}

.batch-btn {
  padding: 4px 10px;
  border: 1px solid var(--color-border-secondary);
  border-radius: 6px;
  background: transparent;
  font-size: 11px;
  color: var(--color-text-secondary);
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.2s;
}

.batch-btn:hover {
  border-color: var(--color-primary);
  color: var(--color-primary);
  background: var(--color-primary-dim);
}

.divider {
  width: 1px;
  background: var(--color-border-primary);
  margin: 0 4px;
}

.file-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

/* File List Items */
.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  border: 1px solid var(--glass-border);
  background: rgba(255, 255, 255, 0.02);
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.file-item:hover {
  border-color: var(--color-primary);
  background: rgba(255, 255, 255, 0.05);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.file-item.selected {
  border-color: var(--color-primary);
  background: var(--color-primary-dim);
  box-shadow: 0 0 0 1px var(--color-primary);
}

.file-item::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(45deg, transparent, rgba(255,255,255,0.03), transparent);
  transform: translateX(-100%);
  transition: transform 0.5s;
}

.file-item:hover::before {
  transform: translateX(100%);
}

.checkbox-container {
  display: flex;
  align-items: center;
}

.file-thumb-container {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
  background: var(--color-bg-hover);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
}

.file-thumb-overlay {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-meta {
  display: flex;
  align-items: center;
  gap: 6px;
}

.meta-pill {
  font-size: 10px;
  padding: 1px 4px;
  border-radius: 4px;
  background: var(--color-bg-active);
  color: var(--color-text-secondary);
  font-weight: 600;
}

.meta-text {
  font-size: 11px;
  color: var(--color-text-secondary);
}

/* Empty State */
.empty {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at center, var(--color-bg-hover), var(--color-bg-secondary));
}

.empty-content {
  text-align: center;
  padding: 40px;
  border-radius: 24px;
  max-width: 400px;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 20px;
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

/* Progress Bar */
.progress-container {
  margin-top: 12px;
}

.progress-bar-bg {
  height: 6px;
  background: var(--color-bg-active);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary), var(--color-ai-gradient-2));
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 11px;
  color: var(--color-text-secondary);
  text-align: center;
  margin-top: 4px;
}

/* Animations */
.fade-in { animation: fadeIn 0.3s ease-out; }
.slide-in { animation: slideIn 0.3s ease-out; }

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

/* Utils */
.glass {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
}

.glass-panel {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(10px);
}

.badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 600;
}

.badge-primary {
  background: var(--color-primary-dim);
  color: var(--color-primary);
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.modal {
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  background: var(--color-bg-primary);
  border-radius: 16px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--color-border-primary);
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
}

.modal-close {
  background: transparent;
  border: none;
  font-size: 20px;
  cursor: pointer;
  color: var(--color-text-secondary);
}

.modal-body {
  padding: 20px;
}

.modal-body section {
  margin-bottom: 24px;
}

.modal-body h3 {
  font-size: 16px;
  margin-bottom: 12px;
  color: var(--color-primary);
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 10px;
}

.feature-item {
  padding: 10px;
  background: var(--color-bg-secondary);
  border-radius: 8px;
  text-align: center;
  font-size: 12px;
  font-weight: 500;
}

.pulse-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-primary);
  margin-right: 8px;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(var(--color-primary), 0.7); }
  70% { transform: scale(1); box-shadow: 0 0 0 10px rgba(var(--color-primary), 0); }
  100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(var(--color-primary), 0); }
}

.spinner {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255,255,255,0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s ease-in-out infinite;
  margin-right: 8px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
.loading-container {
  padding: 20px;
  height: 100%;
  overflow-y: auto;
  display: flex;
  justify-content: center;
}

.loading-content {
  width: 100%;
  max-width: 1200px;
}
</style>
