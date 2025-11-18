<template>
  <div id="pixly-ai-app" :class="{ 'dark-mode': isDarkMode }">
    <!-- Header -->
    <header class="ai-header">
      <div class="header-left">
        <img src="/logo.png" alt="PIXLY AI" class="header-logo" />
        <div class="header-title-group">
          <h1 class="header-title">🤖 PIXLY AI</h1>
          <p class="header-subtitle">{{ $t('subtitle') }}</p>
        </div>
      </div>
      <div class="header-right">
        <el-button 
          circle 
          @click="refreshFiles"
          :icon="Refresh"
          title="刷新文件列表"
        />
        <el-select 
          v-model="currentLang" 
          @change="changeLang"
          style="width: 100px"
        >
          <el-option label="简中" value="zh_CN" />
          <el-option label="EN" value="en" />
        </el-select>
        <el-button 
          circle 
          @click="toggleTheme"
          :icon="isDarkMode ? Sunny : Moon"
          title="切换主题"
        />
      </div>
    </header>

    <!-- Main Content -->
    <main class="ai-main">
      <!-- Empty State -->
      <div v-if="files.length === 0" class="empty-state">
        <el-empty description="未选择文件">
          <template #image>
            <el-icon :size="100"><FolderOpened /></el-icon>
          </template>
          <div class="empty-steps">
            <div class="step">
              <span class="step-number">1</span>
              <span>在 Eagle 选择文件</span>
            </div>
            <div class="step">
              <span class="step-number">2</span>
              <span>返回插件</span>
            </div>
            <div class="step">
              <span class="step-number">3</span>
              <span>开始 AI 处理</span>
            </div>
          </div>
          <el-button type="primary" @click="refreshFiles" :icon="Refresh">
            刷新文件列表
          </el-button>
        </el-empty>
      </div>

      <!-- File List -->
      <div v-else class="file-list-container">
        <div class="file-stats">
          <el-tag type="info">已选择 {{ files.length }} 个文件</el-tag>
          <el-tag v-if="selectedFiles.length > 0" type="primary">
            已勾选 {{ selectedFiles.length }} 个
          </el-tag>
        </div>

        <el-table
          :data="files"
          @selection-change="handleSelectionChange"
          style="width: 100%"
          max-height="400"
        >
          <el-table-column type="selection" width="55" />
          <el-table-column label="预览" width="80">
            <template #default="{ row }">
              <img :src="row.thumbnail" class="file-thumbnail" />
            </template>
          </el-table-column>
          <el-table-column prop="name" label="文件名" />
          <el-table-column prop="ext" label="格式" width="80" />
          <el-table-column label="尺寸" width="120">
            <template #default="{ row }">
              {{ row.width }} × {{ row.height }}
            </template>
          </el-table-column>
          <el-table-column label="大小" width="100">
            <template #default="{ row }">
              {{ formatFileSize(row.size) }}
            </template>
          </el-table-column>
        </el-table>

        <!-- AI Processing Panel -->
        <div class="ai-panel">
          <el-card>
            <template #header>
              <div class="card-header">
                <span>🤖 AI 智能处理</span>
                <el-tag type="success">{{ aiStatus }}</el-tag>
              </div>
            </template>

            <el-form label-width="120px">
              <el-form-item label="处理模式">
                <el-radio-group v-model="processingMode">
                  <el-radio label="smart">智能模式</el-radio>
                  <el-radio label="manual">手动模式</el-radio>
                </el-radio-group>
              </el-form-item>

              <el-form-item label="优化目标">
                <el-select v-model="optimizeTarget" style="width: 100%">
                  <el-option label="平衡质量与大小" value="balanced" />
                  <el-option label="最高质量" value="quality" />
                  <el-option label="最小文件" value="size" />
                </el-select>
              </el-form-item>

              <el-form-item label="输出格式">
                <el-select v-model="outputFormat" style="width: 100%">
                  <el-option label="自动选择（AI推荐）" value="auto" />
                  <el-option label="AVIF" value="avif" />
                  <el-option label="JXL" value="jxl" />
                  <el-option label="WebP" value="webp" />
                  <el-option label="HEIC" value="heic" />
                </el-select>
              </el-form-item>

              <el-form-item>
                <el-button 
                  type="primary" 
                  @click="startProcessing"
                  :loading="isProcessing"
                  :disabled="selectedFiles.length === 0"
                  size="large"
                  style="width: 100%"
                >
                  <el-icon v-if="!isProcessing"><MagicStick /></el-icon>
                  {{ isProcessing ? '处理中...' : '开始 AI 处理' }}
                </el-button>
              </el-form-item>
            </el-form>

            <!-- Progress -->
            <div v-if="isProcessing" class="progress-container">
              <el-progress 
                :percentage="progress" 
                :status="progressStatus"
              />
              <p class="progress-text">{{ progressText }}</p>
            </div>
          </el-card>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { 
  Refresh, 
  Moon, 
  Sunny, 
  FolderOpened,
  MagicStick 
} from '@element-plus/icons-vue'
import { ElMessage, ElNotification } from 'element-plus'
import { useRustCLI } from './composables/useRustCLI'
import { useEagleAPI } from './composables/useEagleAPI'

// Composables
const { 
  isRustAvailable, 
  rustVersion, 
  detectRustCLI, 
  batchConvert 
} = useRustCLI()

const { 
  isEagleAvailable, 
  detectEagle, 
  getSelectedItems 
} = useEagleAPI()

// State
const isDarkMode = ref(false)
const currentLang = ref('zh_CN')
const files = ref([])
const selectedFiles = ref([])
const processingMode = ref('smart')
const optimizeTarget = ref('balanced')
const outputFormat = ref('auto')
const isProcessing = ref(false)
const progress = ref(0)
const progressStatus = ref('')
const progressText = ref('')

// Computed
const aiStatus = computed(() => {
  if (!isRustAvailable.value) return '❌ 未就绪'
  if (isProcessing.value) return '⚙️ 处理中'
  return '✅ 就绪'
})

// Methods
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  document.body.classList.toggle('dark-mode', isDarkMode.value)
}

const changeLang = (lang) => {
  console.log('Language changed to:', lang)
  // TODO: Implement i18n
}

const refreshFiles = async () => {
  try {
    // 使用 Eagle API Composable
    const items = await getSelectedItems()
    files.value = items
    
    const mode = isEagleAvailable.value ? '' : '（本地开发模式）'
    ElMessage.success(`已加载 ${files.value.length} 个文件 ${mode}`)
  } catch (error) {
    console.error('Failed to load files:', error)
    ElMessage.error('加载文件失败: ' + error.message)
  }
}

const handleSelectionChange = (selection) => {
  selectedFiles.value = selection
}

const formatFileSize = (bytes) => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

const startProcessing = async () => {
  if (selectedFiles.value.length === 0) {
    ElMessage.warning('请先选择要处理的文件')
    return
  }

  // 检查 Rust CLI
  if (!isRustAvailable.value) {
    ElMessage.error('❌ Rust 转换内核未就绪，无法处理')
    return
  }

  isProcessing.value = true
  progress.value = 0
  progressStatus.value = ''
  
  try {
    // 🔥 真实调用 Rust CLI 批量转换
    const results = await batchConvert(
      selectedFiles.value,
      {
        mode: processingMode.value,
        optimizeTarget: optimizeTarget.value,
        outputFormat: outputFormat.value === 'auto' ? 'avif' : outputFormat.value
      },
      (progressInfo) => {
        // 更新进度
        progress.value = progressInfo.percentage
        progressText.value = `正在处理 (${progressInfo.current}/${progressInfo.total}): ${progressInfo.currentFile}`
      }
    )
    
    // 统计结果
    const successCount = results.filter(r => r.success).length
    const failCount = results.length - successCount
    
    progressStatus.value = failCount === 0 ? 'success' : 'warning'
    progressText.value = `处理完成！成功: ${successCount}, 失败: ${failCount}`
    
    ElNotification({
      title: '处理完成',
      message: `成功处理 ${successCount} 个文件${failCount > 0 ? `，失败 ${failCount} 个` : ''}`,
      type: failCount === 0 ? 'success' : 'warning',
      duration: 5000
    })

    // 刷新文件列表
    await refreshFiles()
  } catch (error) {
    progressStatus.value = 'exception'
    progressText.value = '处理失败'
    console.error('❌ Processing failed:', error)
    ElMessage.error('处理失败: ' + error.message)
  } finally {
    setTimeout(() => {
      isProcessing.value = false
    }, 1000)
  }
}

// Lifecycle
onMounted(async () => {
  console.log('🚀 PIXLY AI Vue Plugin initializing...')
  
  // 检测 Eagle API
  detectEagle()
  
  // 检测 Rust CLI
  await detectRustCLI()
  
  // 加载文件
  await refreshFiles()
  
  console.log('✅ Plugin initialized')
  console.log('   Eagle API:', isEagleAvailable.value ? '✅' : '❌')
  console.log('   Rust CLI:', isRustAvailable.value ? `✅ ${rustVersion.value}` : '❌')
})
</script>

<style scoped>
#pixly-ai-app {
  min-height: 100vh;
  background: #f5f7fa;
  transition: all 0.3s;
}

.dark-mode {
  background: #1a1a1a;
  color: #fff;
}

.ai-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 30px;
  background: white;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.dark-mode .ai-header {
  background: #2a2a2a;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 15px;
}

.header-logo {
  width: 48px;
  height: 48px;
  border-radius: 8px;
}

.header-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
}

.header-subtitle {
  margin: 0;
  font-size: 14px;
  color: #666;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.ai-main {
  padding: 30px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.empty-steps {
  display: flex;
  justify-content: center;
  gap: 30px;
  margin: 30px 0;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: #409eff;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}

.file-list-container {
  max-width: 1200px;
  margin: 0 auto;
}

.file-stats {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.file-thumbnail {
  width: 60px;
  height: 60px;
  object-fit: cover;
  border-radius: 4px;
}

.ai-panel {
  margin-top: 30px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.progress-container {
  margin-top: 20px;
}

.progress-text {
  text-align: center;
  margin-top: 10px;
  color: #666;
}
</style>
