<template>
  <div id="app" class="pixly-ai-optimizer">
    <!-- 头部 -->
    <header class="app-header">
      <div class="header-left">
        <h1 class="app-title">
          <span class="ai-gradient">PIXLY AI</span> 全媒体智能优化
        </h1>
      </div>
      <div class="header-right">
        <el-button
          v-if="mediaFiles.length > 0 && !isAnalyzing"
          type="primary"
          @click="startAnalysis"
          :disabled="isAnalyzing"
        >
          {{ t('header.analyze') }}
        </el-button>
        <el-button
          v-if="isAnalyzing"
          type="info"
          loading
        >
          {{ t('header.analyzing') }}
        </el-button>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="app-main">
      <!-- 空状态 -->
      <div v-if="mediaFiles.length === 0" class="empty-state">
        <div class="empty-icon">
          <svg width="120" height="120" viewBox="0 0 120 120" fill="none">
            <circle cx="60" cy="60" r="50" stroke="currentColor" stroke-width="2" opacity="0.2"/>
            <path d="M40 60L55 75L80 45" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <h2 class="empty-title">{{ t('main.empty.title') }}</h2>
        <p class="empty-content">{{ t('main.empty.content') }}</p>
      </div>

      <!-- 媒体文件列表 -->
      <div v-else class="media-list">
        <div
          v-for="file in mediaFiles"
          :key="file.id"
          class="media-item"
          :class="{ 'has-recommendation': file.recommendation }"
        >
          <!-- 文件信息 -->
          <div class="media-info">
            <div class="media-preview">
              <img v-if="file.type === 'image'" :src="file.thumbnail" alt="">
              <div v-else class="media-icon" :class="file.type">
                {{ getMediaIcon(file.type) }}
              </div>
            </div>
            <div class="media-details">
              <div class="media-name">{{ file.name }}</div>
              <div class="media-meta">
                <span class="media-badge" :class="file.type">
                  {{ t(`main.mediaTypes.${file.type}`) }}
                </span>
                <span v-if="file.isAnimated" class="media-badge animated">
                  {{ t('main.mediaTypes.animated') }}
                </span>
                <span v-if="file.hasAlpha" class="media-badge transparent">
                  {{ t('main.mediaTypes.transparent') }}
                </span>
                <span class="media-size">{{ formatFileSize(file.size) }}</span>
              </div>
            </div>
          </div>

          <!-- AI推荐 -->
          <div v-if="file.recommendation" class="media-recommendation">
            <div class="recommend-header">
              <h3 class="ai-gradient">{{ t('main.recommendations.title') }}</h3>
              <div class="confidence-badge">
                <span>{{ t('main.recommendations.confidence') }}</span>
                <strong>{{ (file.recommendation.confidence * 100).toFixed(0) }}%</strong>
              </div>
            </div>

            <div class="recommend-content">
              <!-- 推荐格式 -->
              <div class="recommend-item">
                <label>{{ t('main.recommendations.format') }}</label>
                <div class="recommend-value format-badge">
                  {{ file.recommendation.format.toUpperCase() }}
                </div>
              </div>

              <!-- 推荐参数 -->
              <div class="recommend-item">
                <label>{{ t('main.recommendations.params') }}</label>
                <div class="recommend-params">
                  <span v-for="(value, key) in file.recommendation.params" :key="key" class="param-tag">
                    {{ key }}: {{ value }}
                  </span>
                </div>
              </div>

              <!-- 预估效果 -->
              <div class="recommend-item">
                <label>{{ t('main.recommendations.estimated') }}</label>
                <div class="recommend-estimated">
                  <div class="estimated-size">
                    <span class="label">文件大小:</span>
                    <span class="value improved">{{ file.recommendation.estimatedSize }}</span>
                    <span class="reduction">(-{{ file.recommendation.sizeReduction }}%)</span>
                  </div>
                  <div class="estimated-quality">
                    <span class="label">质量评分:</span>
                    <span class="value">{{ file.recommendation.qualityScore }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- 置信度进度条 -->
            <div class="confidence-bar">
              <div
                class="confidence-bar-fill"
                :style="{ width: `${file.recommendation.confidence * 100}%` }"
              ></div>
            </div>
          </div>

          <!-- 分析中状态 -->
          <div v-else-if="isAnalyzing" class="media-analyzing">
            <div class="comet-border">
              <div class="analyzing-content">
                <el-progress
                  type="circle"
                  :percentage="50"
                  :indeterminate="true"
                  :width="60"
                />
                <p>AI分析中...</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 底部操作栏 -->
    <footer v-if="hasRecommendations" class="app-footer">
      <div class="footer-stats">
        <span>已分析 <strong class="ai-gradient">{{ analyzedCount }}</strong> 个文件</span>
        <span>预计节省 <strong class="improved">{{ totalSavings }}</strong></span>
      </div>
      <div class="footer-actions">
        <el-button @click="clearAll">清空</el-button>
        <el-button type="primary" @click="applyRecommendations">
          {{ t('header.apply') }}
        </el-button>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { ElButton, ElProgress, ElMessage } from 'element-plus'
import { useI18n } from './composables/useI18n'
import { useEagleAPI } from './composables/useEagleAPI'
import { useRustCLI } from './composables/useRustCLI'

const { t } = useI18n()
const { getSelectedItems } = useEagleAPI()
const { analyzeMedia } = useRustCLI()

const mediaFiles = ref([])
const isAnalyzing = ref(false)

const hasRecommendations = computed(() => {
  return mediaFiles.value.some(f => f.recommendation)
})

const analyzedCount = computed(() => {
  return mediaFiles.value.filter(f => f.recommendation).length
})

const totalSavings = computed(() => {
  const total = mediaFiles.value.reduce((sum, f) => {
    return sum + (f.recommendation?.sizeReduction || 0)
  }, 0)
  return `${(total / mediaFiles.value.length).toFixed(1)}%`
})

// 加载Eagle选中的文件
async function loadMediaFiles() {
  try {
    const items = await getSelectedItems()
    mediaFiles.value = items.map(item => ({
      id: item.id,
      name: item.name,
      path: item.filePath,
      size: item.size,
      type: detectMediaType(item.ext),
      isAnimated: false, // 待AI分析
      hasAlpha: false,   // 待AI分析
      thumbnail: item.thumbnail,
      recommendation: null
    }))
  } catch (error) {
    console.error('Failed to load media files:', error)
    ElMessage.error('加载文件失败')
  }
}

// 检测媒体类型
function detectMediaType(ext) {
  const imageExts = ['jpg', 'jpeg', 'png', 'webp', 'avif', 'jxl', 'gif', 'apng']
  const videoExts = ['mp4', 'mov', 'webm', 'mkv', 'avi']
  const audioExts = ['mp3', 'aac', 'flac', 'opus', 'wav', 'ogg']
  
  if (imageExts.includes(ext.toLowerCase())) return 'image'
  if (videoExts.includes(ext.toLowerCase())) return 'video'
  if (audioExts.includes(ext.toLowerCase())) return 'audio'
  return 'unknown'
}

// 开始AI分析
async function startAnalysis() {
  isAnalyzing.value = true
  
  try {
    for (const file of mediaFiles.value) {
      const result = await analyzeMedia(file.path)
      file.recommendation = result.recommendation
      file.isAnimated = result.features.isAnimated
      file.hasAlpha = result.features.hasAlpha
    }
    
    ElMessage.success(`分析完成！已为 ${analyzedCount.value} 个文件生成优化建议`)
  } catch (error) {
    console.error('Analysis failed:', error)
    ElMessage.error('AI分析失败，请重试')
  } finally {
    isAnalyzing.value = false
  }
}

// 应用推荐
async function applyRecommendations() {
  // TODO: 打开format-vue插件并传递参数
  ElMessage.info('即将打开格式转换插件...')
}

// 清空列表
function clearAll() {
  mediaFiles.value = []
}

// 获取媒体图标
function getMediaIcon(type) {
  const icons = {
    image: '🖼️',
    video: '🎬',
    audio: '🎵'
  }
  return icons[type] || '📄'
}

// 格式化文件大小
function formatFileSize(bytes) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

// 插件启动时加载文件
onMounted(() => {
  eagle.onPluginRun(async () => {
    await loadMediaFiles()
  })
})
</script>

<style scoped>
@import './styles/variables.css';

.pixly-ai-optimizer {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--el-bg-color);
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--el-border-color);
}

.app-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--el-text-color-secondary);
}

.empty-icon {
  margin-bottom: 24px;
  opacity: 0.5;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--el-text-color-primary);
}

.media-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.media-item {
  background: var(--el-bg-color);
  border: 1px solid var(--el-border-color);
  border-radius: 12px;
  padding: 20px;
  transition: all 0.3s ease;
}

.media-item:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.media-info {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
}

.media-preview {
  width: 80px;
  height: 80px;
  border-radius: 8px;
  overflow: hidden;
  background: var(--el-fill-color-light);
}

.media-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.media-icon {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
}

.media-details {
  flex: 1;
}

.media-name {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 8px;
  color: var(--el-text-color-primary);
}

.media-meta {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
}

.media-size {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.media-recommendation {
  border-top: 1px solid var(--el-border-color-lighter);
  padding-top: 16px;
}

.recommend-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.recommend-header h3 {
  font-size: 16px;
  margin: 0;
}

.confidence-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.confidence-badge strong {
  color: var(--color-ai-gradient-1);
  font-size: 16px;
}

.recommend-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 12px;
}

.recommend-item label {
  display: block;
  font-size: 13px;
  color: var(--el-text-color-secondary);
  margin-bottom: 4px;
}

.format-badge {
  display: inline-block;
  padding: 6px 16px;
  background: linear-gradient(135deg, var(--color-ai-gradient-1), var(--color-ai-gradient-2));
  color: white;
  border-radius: 6px;
  font-weight: 600;
  font-size: 14px;
}

.recommend-params {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.param-tag {
  padding: 4px 12px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  font-size: 13px;
  color: var(--el-text-color-regular);
}

.recommend-estimated {
  display: flex;
  gap: 24px;
}

.estimated-size,
.estimated-quality {
  display: flex;
  align-items: center;
  gap: 8px;
}

.estimated-size .value.improved {
  color: var(--color-status-success);
  font-weight: 600;
}

.reduction {
  color: var(--color-status-success);
  font-size: 12px;
}

.media-analyzing {
  border-top: 1px solid var(--el-border-color-lighter);
  padding-top: 16px;
}

.analyzing-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 24px;
}

.app-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-top: 1px solid var(--el-border-color);
  background: var(--el-bg-color);
}

.footer-stats {
  display: flex;
  gap: 24px;
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.footer-stats strong {
  font-weight: 600;
}

.footer-stats .improved {
  color: var(--color-status-success);
}

.footer-actions {
  display: flex;
  gap: 12px;
}
</style>
