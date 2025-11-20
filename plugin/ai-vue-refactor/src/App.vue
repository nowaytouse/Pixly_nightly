<template>
  <div class="app">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <div class="logo">🤖</div>
        <div class="title-group">
          <h1>{{ t('app.title') }}</h1>
          <p>{{ t('app.subtitle') }}</p>
        </div>
      </div>
      <div class="header-right">
        <button class="icon-btn" @click="refreshFiles" :title="t('header.refresh')">🔄</button>
        <button class="icon-btn" @click="showHelp = true" :title="t('header.help')">❓</button>
        <button class="icon-btn" @click="toggleLanguage" :title="t('header.language')">
          {{ locale === 'zh_CN' ? '🇨🇳' : '🇺🇸' }}
        </button>
        <button class="icon-btn" @click="toggleTheme" :title="t('header.theme')">
          {{ isDark ? '☀️' : '🌙' }}
        </button>
        <!-- Window Controls -->
        <div class="window-controls">
          <button class="window-btn minimize" @click="minimizeWindow" title="Minimize">−</button>
          <button class="window-btn maximize" @click="maximizeWindow" title="Maximize">□</button>
          <button class="window-btn close" @click="closeWindow" title="Close">✕</button>
        </div>
      </div>
    </header>

    <!-- Main -->
    <main class="main">
      <!-- Empty State -->
      <div v-if="files.length === 0" class="empty">
        <div class="empty-icon">📁</div>
        <h3>{{ t('empty.title') }}</h3>
        <p>{{ t('empty.subtitle') }}</p>
        <button class="btn-primary" @click="refreshFiles">{{ t('empty.loadButton') }}</button>
      </div>

      <!-- Content -->
      <div v-else class="content">
        <!-- Left: Controls -->
        <div class="panel controls-panel">
          <h3 class="panel-title">{{ t('controls.title') }}</h3>

          <!-- 优化目标 -->
          <div class="form-group">
            <label>{{ t('controls.optimizeMode') }}</label>
            <select v-model="optimizeMode" class="select">
              <option value="balanced">{{ t('controls.balanced') }}</option>
              <option value="quality">{{ t('controls.quality') }}</option>
              <option value="size">{{ t('controls.size') }}</option>
            </select>
          </div>

          <!-- 🖼️ 图像输出格式 (仅图像模式) -->
          <div v-if="isImageMode" class="form-group">
            <label>{{ t('image.format') }}</label>
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

          <!-- 🎬 视频输出格式 (仅视频模式) -->
          <div v-if="isVideoMode" class="form-group">
            <label>{{ t('video.codec') }}</label>
            <select v-model="videoCodec" class="select">
              <option value="h265">H.265/HEVC (Recommended)</option>
              <option value="h264">H.264/AVC</option>
              <option value="av1">AV1 (Most Efficient)</option>
              <option value="vp9">VP9</option>
            </select>
          </div>

          <div v-if="isVideoMode" class="form-group">
            <label>{{ t('video.container') }}</label>
            <select v-model="videoContainer" class="select">
              <option value="mp4">MP4 (Recommended)</option>
              <option value="mov">MOV</option>
              <option value="webm">WebM</option>
              <option value="mkv">MKV</option>
            </select>
          </div>

          <!-- 🖼️ 图像 AI 功能 (仅图像模式) -->
          <details v-if="isImageMode" class="details" open>
            <summary>{{ t('image.title') }}</summary>
            <div class="checkbox-group">
              <label class="checkbox">
                <input type="checkbox" v-model="enableAIPrediction">
                <span>{{ t('image.aiPrediction') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableFileValidation">
                <span>{{ t('image.fileValidation') }} <span class="badge">Experimental</span></span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableSSIM">
                <span>{{ t('image.ssimValidation') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableGPU">
                <span>{{ t('image.gpuAccel') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enablePreprocess">
                <span>{{ t('image.preprocess') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableFormatCorrection">
                <span>{{ t('image.formatCorrection') }} <span class="badge">Experimental</span></span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>🎬 Animation to Video</span>
              </label>
            </div>
          </details>

          <!-- 📦 混合模式提示 -->
          <div v-if="isMixedMode" class="mixed-mode-notice">
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
          <details v-if="isVideoMode" class="details" open>
            <summary>{{ t('video.title') }}</summary>
            <div class="checkbox-group">
              <label class="checkbox">
                <input type="checkbox" v-model="enableVideoCodecRecommendation" checked>
                <span>🤖 {{ t('video.codecRecommendation') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>🎬 {{ t('video.animationToVideo') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableSceneDetection">
                <span>🎞️ {{ t('video.sceneDetection') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableVMAF">
                <span>📊 {{ t('video.vmafValidation') }}</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableTwoPass">
                <span>🔄 {{ t('video.twoPass') }}</span>
              </label>
            </div>
          </details>

          <!-- 自动处理提示 -->
          <div class="auto-hints">
            <div class="hint-item">{{ t('features.validation') }}</div>
            <div class="hint-item">{{ t('features.xmpMerge') }}</div>
            <div class="hint-item">{{ t('features.filenameNorm') }}</div>
          </div>

          <!-- 元数据保留提示 -->
          <div class="metadata-notice">
            <strong>{{ t('features.metadataTitle') }}</strong>
            <div class="metadata-items">
              <span>✓ {{ t('features.exif') }}</span>
              <span>✓ {{ t('features.xmp') }}</span>
              <span>✓ {{ t('features.icc') }}</span>
              <span>✓ {{ t('features.timestamps') }}</span>
              <span>✓ {{ t('features.extendedAttr') }}</span>
            </div>
          </div>

          <!-- 转换按钮 -->
          <button 
            class="btn-convert" 
            @click="startConvert"
            :disabled="selectedCount === 0 || processing"
          >
            {{ processing ? '⚙️ ' + t('button.processing') : '✨ ' + t('button.startConvert') }}
          </button>

          <!-- 进度 -->
          <div v-if="processing" class="progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: progress + '%' }"></div>
            </div>
            <div class="progress-text">{{ progressText }}</div>
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
              ✓ {{ t('fileList.selectAll') }}
            </button>
            <button class="batch-btn" @click="selectNone" :title="t('fileList.selectNone')">
              ✗ {{ t('fileList.selectNone') }}
            </button>
            <button class="batch-btn" @click="selectInvert" :title="t('fileList.selectInvert')">
              ⇄ {{ t('fileList.selectInvert') }}
            </button>
            <button class="batch-btn" @click="selectImages" :title="t('fileList.selectImages')">
              🖼️ {{ t('fileList.selectImages') }}
            </button>
            <button class="batch-btn" @click="selectVideos" :title="t('fileList.selectVideos')">
              🎬 {{ t('fileList.selectVideos') }}
            </button>
          </div>

          <div class="file-list">
            <label 
              v-for="file in files" 
              :key="file.id"
              class="file-item"
              :class="{ selected: file.selected }"
            >
              <input type="checkbox" v-model="file.selected">
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
                  {{ formatSize(file.size) }} · {{ file.width }}×{{ file.height }}
                </div>
              </div>
            </label>
          </div>
        </div>

        <!-- 🔍 AI决策透明面板 - 展示处理过程的透明化 -->
        <AITransparencyPanel 
          ref="transparencyPanel"
          :decision-data="aiDecisionData"
        />
      </div>
    </main>

    <!-- Help Modal -->
    <div v-if="showHelp" class="modal-overlay" @click="showHelp = false">
      <div class="modal" @click.stop>
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
            <p>{{ t('help.metadataDesc') }}</p>
            <ul>
              <li><strong>{{ t('help.metadataExif') }}</strong></li>
              <li><strong>{{ t('help.metadataXmp') }}</strong></li>
              <li><strong>{{ t('help.metadataIcc') }}</strong></li>
              <li><strong>{{ t('help.metadataXmpMerge') }}</strong></li>
              <li><strong>{{ t('help.metadataEagle') }}</strong></li>
              <li><strong>{{ t('help.metadataTimestamp') }}</strong></li>
              <li><strong>{{ t('help.metadataExtended') }}</strong></li>
            </ul>
          </section>

          <section>
            <h3>{{ t('help.imageFeaturesTitle') }}</h3>
            <ul>
              <li><strong>{{ t('help.imageFeature1') }}</strong></li>
              <li><strong>{{ t('help.imageFeature2') }}</strong></li>
              <li><strong>{{ t('help.imageFeature3') }}</strong></li>
              <li><strong>{{ t('help.imageFeature4') }}</strong></li>
              <li><strong>{{ t('help.imageFeature5') }}</strong></li>
              <li><strong>{{ t('help.imageFeature6') }}</strong></li>
            </ul>
          </section>

          <section>
            <h3>{{ t('help.videoFeaturesTitle') }}</h3>
            <ul>
              <li><strong>{{ t('help.videoFeature1') }}</strong></li>
              <li><strong>{{ t('help.videoFeature2') }}</strong></li>
              <li><strong>{{ t('help.videoFeature3') }}</strong></li>
              <li><strong>{{ t('help.videoFeature4') }}</strong></li>
            </ul>
          </section>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, provide } from 'vue'
import { useRustCLI } from './composables/useRustCLI'
import { useEagleAPI } from './composables/useEagleAPI'
import { logger, LOG_KEYS } from './utils/logger'
import { useI18n } from './composables/useI18n'
import AITransparencyPanel from './components/AITransparencyPanel.vue'

const { t, setLocale, locale } = useI18n()

// Provide t function to child components
provide('t', t)

const rustCLI = useRustCLI()
const eagleAPI = useEagleAPI()

const isDark = ref(true)
const showHelp = ref(false)
const files = ref([])
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

// 🔍 AI决策透明面板
const transparencyPanel = ref(null)
const aiDecisionData = ref(null)

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
  try {
    const items = await eagleAPI.getSelectedItems()
    files.value = items.map(item => ({ ...item, selected: false }))
  } catch (err) {
    logger.error(LOG_KEYS.FILE_LOAD_ERROR, 'Failed to load files', { error: err.message })
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
    progressText.value = t('progress.success', { success: successCount, total: results.length })
    
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
    console.log('[PIXLY AI] Minimizing window...')
    if (window.eagle && window.eagle.window && typeof window.eagle.window.minimize === 'function') {
      window.eagle.window.minimize()
      console.log('[PIXLY AI] ✅ Window minimized')
    } else if (window.eagle && window.eagle.app && typeof window.eagle.app.minimize === 'function') {
      window.eagle.app.minimize()
      console.log('[PIXLY AI] ✅ Window minimized (via app)')
    } else {
      console.error('[PIXLY AI] ❌ Minimize method not found')
    }
  } catch (error) {
    console.error('[PIXLY AI] ❌ Failed to minimize:', error)
  }
}

const maximizeWindow = () => {
  try {
    console.log('[PIXLY AI] Toggling maximize...', { currentState: isMaximized.value })
    
    if (!window.eagle || !window.eagle.window) {
      console.error('[PIXLY AI] ❌ Eagle window API not available')
      return
    }
    
    // Toggle between maximize and restore
    if (isMaximized.value) {
      // Currently maximized, restore to normal
      if (typeof window.eagle.window.unmaximize === 'function') {
        window.eagle.window.unmaximize()
        isMaximized.value = false
        console.log('[PIXLY AI] ✅ Window unmaximized')
      } else if (typeof window.eagle.window.restore === 'function') {
        window.eagle.window.restore()
        isMaximized.value = false
        console.log('[PIXLY AI] ✅ Window restored')
      } else {
        console.error('[PIXLY AI] ❌ Unmaximize method not found')
      }
    } else {
      // Currently normal, maximize
      if (typeof window.eagle.window.maximize === 'function') {
        window.eagle.window.maximize()
        isMaximized.value = true
        console.log('[PIXLY AI] ✅ Window maximized')
      } else {
        console.error('[PIXLY AI] ❌ Maximize method not found')
      }
    }
  } catch (error) {
    console.error('[PIXLY AI] ❌ Failed to toggle maximize:', error)
  }
}

const closeWindow = () => {
  try {
    console.log('[PIXLY AI] Closing window...')
    
    if (!window.eagle) {
      console.error('[PIXLY AI] ❌ Eagle API not available')
      return
    }
    
    // Try different API methods
    if (window.eagle.window && typeof window.eagle.window.close === 'function') {
      window.eagle.window.close()
      console.log('[PIXLY AI] ✅ Window closed')
    } else if (window.eagle.app && typeof window.eagle.app.close === 'function') {
      window.eagle.app.close()
      console.log('[PIXLY AI] ✅ Window closed (via app)')
    } else if (window.close && typeof window.close === 'function') {
      window.close()
      console.log('[PIXLY AI] ✅ Window closed (via window.close)')
    } else {
      console.error('[PIXLY AI] ❌ Close method not found')
    }
  } catch (error) {
    console.error('[PIXLY AI] ❌ Failed to close:', error)
  }
}

onMounted(() => {
  eagleAPI.detect()
  rustCLI.init()
  refreshFiles()
})
</script>

<style scoped>
.app {
  min-height: 100vh;
  background: var(--color-bg-secondary);
  color: var(--color-text-primary);
}

.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: var(--color-bg-primary);
  border-bottom: 1px solid var(--color-border-primary);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: linear-gradient(135deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
}

.title-group h1 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.title-group p {
  margin: 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.header-right {
  display: flex;
  gap: 8px;
}

.icon-btn {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: none;
  background: var(--color-bg-hover);
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: var(--color-bg-active);
}

/* Window Controls */
.window-controls {
  display: flex;
  gap: 0;
  margin-left: 12px;
  padding-left: 12px;
  border-left: 1px solid var(--color-border-primary);
}

.window-btn {
  width: 46px;
  height: 36px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 16px;
  color: var(--color-text-secondary);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.window-btn:hover {
  background: var(--color-bg-hover);
}

.window-btn.close:hover {
  background: #e81123;
  color: white;
}

.main {
  padding: 24px;
  padding-top: 96px; /* 为固定的header留出空间 (header高度约72px + padding) */
  max-width: 1400px;
  margin: 0 auto;
}

.empty {
  text-align: center;
  padding: 60px 20px;
  background: var(--color-bg-primary);
  border-radius: 12px;
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
}

.empty p {
  margin: 0 0 24px 0;
  color: var(--color-text-secondary);
}

.btn-primary {
  padding: 12px 24px;
  border-radius: 8px;
  border: none;
  background: linear-gradient(90deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
  color: white;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.content {
  display: grid;
  grid-template-columns: 360px 1fr;
  gap: 20px;
}

/* 文件类型指示器 */
.header-right-badges {
  display: flex;
  align-items: center;
  gap: 8px;
}

.type-badge {
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
}

.type-badge.video {
  background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
  color: white;
}

.type-badge.image {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
}

.type-badge.mixed {
  background: linear-gradient(135deg, #f59e0b 0%, #ef4444 100%);
  color: white;
}

.panel {
  background: var(--color-bg-primary);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--color-border-primary);
}

.panel-title {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.file-count {
  font-size: 11px;
  padding: 4px 10px;
  border-radius: 6px;
  background: var(--color-bg-hover);
  color: var(--color-text-secondary);
  font-weight: 600;
  white-space: nowrap;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 6px;
}

.select {
  width: 100%;
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid var(--color-border-primary);
  background: var(--color-bg-secondary);
  color: var(--color-text-primary);
  font-size: 13px;
}

.slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: var(--color-bg-secondary);
  outline: none;
}

.slider::-webkit-slider-thumb {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
  cursor: pointer;
}

.details {
  margin-bottom: 16px;
  border: 1px solid var(--color-border-primary);
  border-radius: 8px;
  padding: 12px;
}

.details summary {
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  user-select: none;
}

.checkbox-group {
  margin-top: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  cursor: pointer;
}

.btn-convert {
  width: 100%;
  padding: 14px;
  border-radius: 8px;
  border: none;
  background: linear-gradient(90deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
  color: white;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-convert:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-convert:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.progress {
  margin-top: 16px;
}

.progress-bar {
  height: 6px;
  background: var(--color-bg-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-ai-gradient-1) 0%, var(--color-ai-gradient-2) 100%);
  transition: width 0.3s;
}

.progress-text {
  font-size: 12px;
  color: var(--color-text-secondary);
  text-align: center;
}

/* 批量操作栏 */
.batch-actions {
  display: flex;
  gap: 6px;
  padding: 12px;
  background: var(--color-bg-secondary);
  border-radius: 8px;
  margin-bottom: 12px;
  flex-wrap: wrap;
}

.batch-btn {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid var(--color-border-primary);
  background: var(--color-bg-primary);
  color: var(--color-text-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.batch-btn:hover {
  background: var(--color-bg-hover);
  border-color: var(--color-ai-gradient-1);
}

.batch-btn:active {
  transform: scale(0.95);
}

/* 混合模式提示 */
.mixed-mode-notice {
  margin-bottom: 16px;
  padding: 16px;
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.1) 0%, rgba(239, 68, 68, 0.1) 100%);
  border: 1px solid rgba(245, 158, 11, 0.3);
  border-radius: 8px;
}

.notice-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.notice-icon {
  font-size: 20px;
}

.notice-header strong {
  font-size: 14px;
  color: var(--color-text-primary);
}

.notice-body p {
  margin: 0 0 12px 0;
  font-size: 12px;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.file-groups {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.group-item {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--color-bg-primary);
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
}

.group-icon {
  font-size: 16px;
}

.notice-tip {
  margin: 0;
  padding: 8px;
  background: var(--color-bg-primary);
  border-radius: 6px;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-style: italic;
}

.btn-convert-to-video:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn-convert-to-video:active {
  transform: translateY(0);
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 600px;
  overflow-y: auto;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  border-radius: 8px;
  border: 1px solid transparent;
  cursor: pointer;
  transition: all 0.2s;
}

.file-item:hover {
  background: var(--color-bg-hover);
}

.file-item.selected {
  border-color: var(--color-ai-gradient-1);
  background: var(--color-ai-gradient-1-20);
}

/* 🔥 缩略图容器 */
.file-thumb-container {
  position: relative;
  width: 50px;
  height: 50px;
  flex-shrink: 0;
}

/* Emoji占位符 - 始终显示 */
.file-thumb-placeholder {
  width: 50px;
  height: 50px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--color-bg-secondary) 0%, var(--color-bg-hover) 100%);
  position: relative;
  z-index: 1;
}

.file-emoji {
  font-size: 28px;
  line-height: 1;
}

/* 缩略图覆盖层 */
.file-thumb {
  width: 50px;
  height: 50px;
  border-radius: 6px;
  object-fit: cover;
}

.file-thumb-overlay {
  position: absolute;
  top: 0;
  left: 0;
  z-index: 2;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-meta {
  font-size: 11px;
  color: var(--color-text-secondary);
  margin-top: 2px;
}

/* 自动处理提示 */
.auto-hints {
  margin-top: 16px;
  padding: 12px;
  background: var(--color-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--color-border-primary);
}

.hint-item {
  font-size: 11px;
  color: var(--color-text-secondary);
  padding: 4px 0;
  display: flex;
  align-items: center;
  gap: 6px;
}

.hint-item:not(:has(.checkbox))::before {
  content: '•';
  color: var(--color-ai-gradient-1);
}

.hint-item .checkbox {
  margin: 0;
  font-size: 11px;
}

/* 实验性标签 */
.badge {
  font-size: 9px;
  font-weight: 700;
  padding: 2px 6px;
  border-radius: 3px;
  background: linear-gradient(135deg, #f59e0b 0%, #ef4444 100%);
  color: white;
  margin-left: 4px;
  letter-spacing: 0.3px;
}

/* 信息提示 */
.info-text {
  font-size: 12px;
  color: var(--color-text-secondary);
  padding: 8px;
  background: var(--color-bg-secondary);
  border-radius: 6px;
  margin-top: 12px;
  text-align: center;
}

/* 功能说明 */
.feature-description {
  margin-bottom: 16px;
  padding: 16px;
  background: var(--color-bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--color-border-primary);
}

.feature-description h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
}

.feature-description p {
  margin: 0 0 12px 0;
  font-size: 12px;
  line-height: 1.6;
  color: var(--color-text-secondary);
}

.feature-description p:last-of-type {
  margin-bottom: 16px;
}

.metadata-notice {
  padding: 12px;
  background: var(--color-bg-primary);
  border-radius: 6px;
  border: 1px solid var(--color-border-primary);
}

.metadata-notice strong {
  display: block;
  font-size: 12px;
  margin-bottom: 8px;
  color: var(--color-text-primary);
}

.metadata-items {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-bottom: 8px;
}

.metadata-items span {
  font-size: 10px;
  padding: 3px 6px;
  background: var(--color-ai-gradient-1-20);
  border-radius: 3px;
  color: var(--color-text-primary);
}

.metadata-desc {
  font-size: 10px;
  color: var(--color-text-secondary);
  margin: 0;
  font-style: italic;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal {
  background: var(--color-bg-primary);
  border-radius: 12px;
  max-width: 600px;
  width: 100%;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--color-border-primary);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border-primary);
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.modal-close {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: none;
  background: var(--color-bg-hover);
  cursor: pointer;
  font-size: 18px;
  color: var(--color-text-secondary);
  transition: all 0.2s;
}

.modal-close:hover {
  background: var(--color-bg-active);
  color: var(--color-text-primary);
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
}

.modal-body section {
  margin-bottom: 24px;
}

.modal-body section:last-child {
  margin-bottom: 0;
}

.modal-body h3 {
  margin: 0 0 12px 0;
  font-size: 15px;
  font-weight: 600;
}

.modal-body p {
  margin: 0 0 12px 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--color-text-secondary);
}

.modal-body ul {
  margin: 0;
  padding-left: 20px;
  font-size: 13px;
  line-height: 1.8;
  color: var(--color-text-secondary);
}

.modal-body li {
  margin-bottom: 8px;
}

.modal-body strong {
  color: var(--color-text-primary);
}


</style>
