<template>
  <div class="app">
    <!-- Header -->
    <header class="header">
      <div class="header-left">
        <div class="logo">🤖</div>
        <div class="title-group">
          <h1>PIXLY AI</h1>
          <p>智能多媒体处理</p>
        </div>
      </div>
      <div class="header-right">
        <button class="icon-btn" @click="refreshFiles" title="刷新">🔄</button>
        <button class="icon-btn" @click="showHelp = true" title="使用帮助">❓</button>
        <button class="icon-btn" @click="toggleTheme" title="切换主题">
          {{ isDark ? '☀️' : '🌙' }}
        </button>
      </div>
    </header>

    <!-- Main -->
    <main class="main">
      <!-- Empty State -->
      <div v-if="files.length === 0" class="empty">
        <div class="empty-icon">📁</div>
        <h3>未选择文件</h3>
        <p>在 Eagle 中选择图片或视频</p>
        <button class="btn-primary" @click="refreshFiles">加载文件</button>
      </div>

      <!-- Content -->
      <div v-else class="content">
        <!-- Left: Controls -->
        <div class="panel controls-panel">
          <h3 class="panel-title">🤖 AI 智能选项</h3>

          <!-- 优化目标 -->
          <div class="form-group">
            <label>优化目标</label>
            <select v-model="optimizeMode" class="select">
              <option value="balanced">⚖️ 平衡 - 质量与体积兼顾</option>
              <option value="quality">💎 质量优先 - 最佳画质</option>
              <option value="size">📦 体积优先 - 最小文件</option>
            </select>
          </div>

          <!-- 🖼️ 图像输出格式 (仅图像模式) -->
          <div v-if="isImageMode" class="form-group">
            <label>输出格式</label>
            <select v-model="outputFormat" class="select">
              <option value="auto">🔮 自动选择（AI推荐）</option>
              <option value="disabled">🚫 禁用（原格式优化）</option>
              <option value="avif">AVIF</option>
              <option value="jxl">JXL</option>
              <option value="webp">WebP</option>
              <option value="heic">HEIC</option>
              <option value="png">PNG</option>
            </select>
          </div>

          <!-- 🎬 视频输出格式 (仅视频模式) -->
          <div v-if="isVideoMode" class="form-group">
            <label>视频编码器</label>
            <select v-model="videoCodec" class="select">
              <option value="h265">H.265/HEVC (推荐)</option>
              <option value="h264">H.264/AVC</option>
              <option value="av1">AV1 (最高效)</option>
              <option value="vp9">VP9</option>
            </select>
          </div>

          <div v-if="isVideoMode" class="form-group">
            <label>容器格式</label>
            <select v-model="videoContainer" class="select">
              <option value="mp4">MP4 (推荐)</option>
              <option value="mov">MOV</option>
              <option value="webm">WebM</option>
              <option value="mkv">MKV</option>
            </select>
          </div>

          <!-- 🖼️ 图像 AI 功能 (仅图像模式) -->
          <details v-if="isImageMode" class="details" open>
            <summary>🧠 图像 AI 功能</summary>
            <div class="checkbox-group">
              <label class="checkbox">
                <input type="checkbox" v-model="enableAIPrediction">
                <span>🎯 智能参数预测</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableFileValidation">
                <span>🔒 AI 文件验证 <span class="badge">实验性</span></span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableSSIM">
                <span>📊 SSIM 质量验证</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableGPU">
                <span>⚡ GPU 硬件加速</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enablePreprocess">
                <span>🔗 智能预处理</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableFormatCorrection">
                <span>🔧 格式自动修正 <span class="badge">实验性</span></span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>🎬 动图转视频推荐</span>
              </label>
            </div>
          </details>

          <!-- 📦 混合模式提示 -->
          <div v-if="isMixedMode" class="mixed-mode-notice">
            <div class="notice-header">
              <span class="notice-icon">📦</span>
              <strong>混合模式</strong>
            </div>
            <div class="notice-body">
              <p>检测到图像和视频混合选择，将自动分组处理：</p>
              <div class="file-groups">
                <div class="group-item">
                  <span class="group-icon">🖼️</span>
                  <span>图像: {{ selectedFiles.filter(f => /\.(jpg|jpeg|png|gif|webp|avif|jxl|heic|heif|bmp|tiff|tif)$/i.test(f.name)).length }} 个</span>
                </div>
                <div class="group-item">
                  <span class="group-icon">🎬</span>
                  <span>视频: {{ selectedFiles.filter(f => /\.(mp4|mov|avi|mkv|webm|flv|wmv|m4v|mpg|mpeg)$/i.test(f.name)).length }} 个</span>
                </div>
              </div>
              <p class="notice-tip">💡 图像和视频将使用各自的AI功能和参数</p>
            </div>
          </div>

          <!-- 🎬 视频 AI 功能 (仅视频模式) -->
          <details v-if="isVideoMode" class="details" open>
            <summary>🎬 视频 AI 功能</summary>
            <div class="checkbox-group">
              <label class="checkbox">
                <input type="checkbox" v-model="enableVideoForAnimation">
                <span>🎬 动图转视频推荐</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableSceneDetection">
                <span>🎞️ 场景检测</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableVMAF">
                <span>📊 VMAF 质量验证</span>
              </label>
              <label class="checkbox">
                <input type="checkbox" v-model="enableTwoPass">
                <span>🔄 Two-Pass 编码</span>
              </label>
            </div>
          </details>

          <!-- 自动处理提示 -->
          <div class="auto-hints">
            <div class="hint-item">🔒 8层验证机制</div>
            <div class="hint-item">💡 XMP自动合并</div>
            <div class="hint-item">📝 文件名自动规范化</div>
          </div>

          <!-- 元数据保留提示 -->
          <div class="metadata-notice">
            <strong>📦 元数据完整保留</strong>
            <div class="metadata-items">
              <span>✓ EXIF</span>
              <span>✓ XMP</span>
              <span>✓ ICC</span>
              <span>✓ 时间戳</span>
              <span>✓ 扩展属性</span>
            </div>
          </div>

          <!-- 转换按钮 -->
          <button 
            class="btn-convert" 
            @click="startConvert"
            :disabled="selectedCount === 0 || processing"
          >
            {{ processing ? '⚙️ 处理中...' : '✨ 开始 AI 处理' }}
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
            <h3 class="panel-title">📁 文件列表</h3>
            <div class="header-right-badges">
              <!-- 🔥 文件类型指示器 -->
              <span v-if="isVideoMode" class="type-badge video">🎬 视频模式</span>
              <span v-else-if="isImageMode" class="type-badge image">🖼️ 图像模式</span>
              <span v-else-if="isMixedMode" class="type-badge mixed">📦 混合模式</span>
              <span class="file-count">{{ selectedCount }}/{{ files.length }}</span>
            </div>
          </div>

          <!-- 🔥 批量操作栏 -->
          <div class="batch-actions">
            <button class="batch-btn" @click="selectAll" title="全选">
              ✓ 全选
            </button>
            <button class="batch-btn" @click="selectNone" title="取消全选">
              ✗ 取消
            </button>
            <button class="batch-btn" @click="selectInvert" title="反选">
              ⇄ 反选
            </button>
            <button class="batch-btn" @click="selectImages" title="选择所有图像">
              🖼️ 图像
            </button>
            <button class="batch-btn" @click="selectVideos" title="选择所有视频">
              🎬 视频
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
              <img :src="file.thumbnail" class="file-thumb" alt="">
              <div class="file-info">
                <div class="file-name">{{ file.name }}</div>
                <div class="file-meta">
                  {{ formatSize(file.size) }} · {{ file.width }}×{{ file.height }}
                </div>
              </div>
            </label>
          </div>
        </div>
      </div>
    </main>

    <!-- Help Modal -->
    <div v-if="showHelp" class="modal-overlay" @click="showHelp = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h2>💡 使用帮助</h2>
          <button class="modal-close" @click="showHelp = false">✕</button>
        </div>
        <div class="modal-body">
          <section>
            <h3>🎯 插件愿景</h3>
            <p><strong>让 AI 成为你的转换专家</strong> - 告别繁琐的参数调试，只需选择你的目标（极致压缩/视觉无损/平衡），AI 会根据每张图片的特征（纹理复杂度、色彩分布、透明度等）自动优化所有参数。</p>
            <p><strong>拥抱现代格式</strong> - 默认推荐 AVIF、JXL、WebP 等新一代格式，相同质量下体积减少 30-50%，让你的资源库更轻盈。同时支持 HDR、动画、透明度等高级特性。</p>
            <p><strong>智能格式升级</strong> - 检测到 JPEG/PNG 等传统格式时，AI 会评估升级收益（压缩率提升、特性增强），在合适的时机推荐现代格式，让每一次转换都物有所值。</p>
          </section>
          
          <section>
            <h3>📦 元数据完整保留</h3>
            <p>所有元数据自动保留，无需任何配置：</p>
            <ul>
              <li><strong>EXIF 相机信息</strong> - 拍摄参数、GPS 位置、设备型号</li>
              <li><strong>XMP 编辑历史</strong> - Photoshop/Lightroom 编辑记录</li>
              <li><strong>ICC 色彩配置</strong> - 色彩空间和配置文件</li>
              <li><strong>XMP Sidecar 合并</strong> - 使用 exiftool 自动合并外部 XMP 文件</li>
              <li><strong>Eagle 资源信息</strong> - 标签、评分、备注同步更新</li>
              <li><strong>文件时间戳</strong> - 创建/修改/访问时间完整保留</li>
              <li><strong>扩展属性</strong> - macOS xattr、Windows ADS 自动保留</li>
            </ul>
          </section>

          <section>
            <h3>🧠 图像 AI 功能</h3>
            <ul>
              <li><strong>智能参数预测</strong> - AI 分析图像特征，自动选择最优参数</li>
              <li><strong>AI 文件验证</strong> - 使用 Google Magika 检测文件类型，防止伪装文件</li>
              <li><strong>SSIM 质量验证</strong> - 转换后自动验证画质损失</li>
              <li><strong>GPU 硬件加速</strong> - 自动检测并使用 GPU 加速（速度提升 5-20 倍）</li>
              <li><strong>智能预处理</strong> - 自动优化图像（去噪、锐化、色彩校正）</li>
              <li><strong>格式自动修正</strong> - 检测文件扩展名与实际格式是否匹配，自动识别伪装文件（如 .jpg 实际是 .png）</li>
            </ul>
          </section>

          <section>
            <h3>🎬 视频 AI 功能</h3>
            <ul>
              <li><strong>动图转视频推荐</strong> - 检测大型动图（GIF/APNG/WebP），智能推荐转为视频格式（MP4/WebM），体积减少 60-80%</li>
              <li><strong>场景检测</strong> - 智能检测场景变化，优化关键帧分布</li>
              <li><strong>VMAF 质量验证</strong> - 使用 Netflix VMAF 算法验证视频质量</li>
              <li><strong>Two-Pass 编码</strong> - 两次编码优化码率分配，相同质量下体积减少 10-20%</li>
            </ul>
          </section>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRustCLI } from './composables/useRustCLI'
import { useEagleAPI } from './composables/useEagleAPI'
import { logger, LOG_KEYS } from './utils/logger'

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

const isVideoMode = computed(() => {
  if (selectedFiles.value.length === 0) return false
  const videoExts = /\.(mp4|mov|avi|mkv|webm|flv|wmv|m4v|mpg|mpeg)$/i
  return selectedFiles.value.every(f => videoExts.test(f.name))
})

const isImageMode = computed(() => {
  if (selectedFiles.value.length === 0) return false
  const imageExts = /\.(jpg|jpeg|png|gif|webp|avif|jxl|heic|heif|bmp|tiff|tif)$/i
  return selectedFiles.value.every(f => imageExts.test(f.name))
})

const isMixedMode = computed(() => {
  return selectedFiles.value.length > 0 && !isVideoMode.value && !isImageMode.value
})



const toggleTheme = () => {
  isDark.value = !isDark.value
  document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
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
  const imageExts = /\.(jpg|jpeg|png|gif|webp|avif|jxl|heic|heif|bmp|tiff|tif)$/i
  files.value.forEach(f => {
    f.selected = imageExts.test(f.name)
  })
}

const selectVideos = () => {
  const videoExts = /\.(mp4|mov|avi|mkv|webm|flv|wmv|m4v|mpg|mpeg)$/i
  files.value.forEach(f => {
    f.selected = videoExts.test(f.name)
  })
}



const formatSize = (bytes) => {
  return (bytes / 1024 / 1024).toFixed(2) + ' MB'
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
      
      const imageExts = /\.(jpg|jpeg|png|gif|webp|avif|jxl|heic|heif|bmp|tiff|tif)$/i
      const videoExts = /\.(mp4|mov|avi|mkv|webm|flv|wmv|m4v|mpg|mpeg)$/i
      
      const images = selected.filter(f => imageExts.test(f.name))
      const videos = selected.filter(f => videoExts.test(f.name))
      
      logger.info(LOG_KEYS.CONVERT_START, 'File grouping', { images: images.length, videos: videos.length })
      
      let processed = 0
      const total = selected.length
      
      // 先处理图像
      if (images.length > 0) {
        progressText.value = `处理图像 (${images.length} 个)...`
        
        for (let i = 0; i < images.length; i++) {
          const file = images[i]
          try {
            processed++
            progress.value = Math.round((processed / total) * 100)
            progressText.value = `[图像 ${i + 1}/${images.length}] ${file.name}`
            
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
        progressText.value = `处理视频 (${videos.length} 个)...`
        
        for (let i = 0; i < videos.length; i++) {
          const file = videos[i]
          try {
            processed++
            progress.value = Math.round((processed / total) * 100)
            progressText.value = `[视频 ${i + 1}/${videos.length}] ${file.name}`
            
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
          progressText.value = `处理视频 (${i + 1}/${selected.length}): ${file.name}`
          
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
          progressText.value = `处理图像 (${info.current}/${info.total}): ${info.file}`
        }
      )
    }

    const successCount = results.filter(r => r.success).length
    progressText.value = `完成！成功: ${successCount}/${results.length}`
    
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

.main {
  padding: 24px;
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

.file-thumb {
  width: 50px;
  height: 50px;
  border-radius: 6px;
  object-fit: cover;
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
