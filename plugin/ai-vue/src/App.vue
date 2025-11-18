<template>
  <div class="pixly-ai-app">
    <el-container>
      <!-- Header -->
      <el-header height="60px">
        <div class="header-content">
          <div class="header-left">
            <img src="/logo.png" class="logo" alt="PIXLY AI">
            <div class="title-group">
              <h1>PIXLY AI</h1>
              <p>智能多媒体处理 · 一键优化</p>
            </div>
          </div>
          <div class="header-right">
            <el-button circle @click="refreshFiles">
              <span>🔄</span>
            </el-button>
          </div>
        </div>
      </el-header>

      <!-- Main Content -->
      <el-main>
        <!-- Empty State -->
        <el-empty v-if="files.length === 0" description="从 Eagle 选择文件开始">
          <template #image>
            <div class="empty-icon">🤖</div>
          </template>
        </el-empty>

        <!-- File List & AI Options -->
        <div v-else class="main-grid">
          <!-- Left: AI Options -->
          <div class="ai-panel">
            <el-card>
              <template #header>
                <span>🤖 AI 智能选项</span>
              </template>
              
              <!-- Preset -->
              <div class="option-group">
                <label>优化目标</label>
                <el-radio-group v-model="aiConfig.preset" class="preset-group">
                  <el-radio-button value="balanced">⚖️ 平衡</el-radio-button>
                  <el-radio-button value="quality">💎 质量</el-radio-button>
                  <el-radio-button value="size">📦 体积</el-radio-button>
                </el-radio-group>
              </div>

              <!-- Features -->
              <div class="option-group">
                <label>AI 功能</label>
                <el-checkbox v-model="aiConfig.smartQuality">🎯 智能质量预测</el-checkbox>
                <el-checkbox v-model="aiConfig.formatRecommend">📋 格式推荐</el-checkbox>
                <el-checkbox v-model="aiConfig.autoOptimize">⚡ 自动参数优化</el-checkbox>
                <el-checkbox v-model="aiConfig.batchOptimize">🔄 批量优化</el-checkbox>
              </div>
            </el-card>
          </div>

          <!-- Right: File List -->
          <div class="file-panel">
            <el-card>
              <template #header>
                <span>文件列表 ({{ files.length }})</span>
              </template>
              
              <div class="file-list">
                <div v-for="(file, index) in files" :key="file.id" class="file-item">
                  <img :src="file.thumbnail" class="file-thumb" :alt="file.name">
                  <div class="file-info">
                    <div class="file-name">{{ file.name }}.{{ file.ext }}</div>
                    <div class="file-meta">{{ formatSize(file.size) }} · {{ file.ext.toUpperCase() }}</div>
                  </div>
                  <el-button circle size="small" @click="removeFile(index)">✕</el-button>
                </div>
              </div>

              <el-button type="primary" size="large" :loading="isConverting" @click="startConversion" class="convert-btn">
                🚀 开始 AI 转换
              </el-button>
            </el-card>
          </div>
        </div>
      </el-main>

      <!-- Progress -->
      <div v-if="isConverting" class="progress-bar">
        <el-progress :percentage="progress" :format="() => `${currentFile}/${files.length}`" />
      </div>
    </el-container>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'

const files = ref([])
const isConverting = ref(false)
const progress = ref(0)
const currentFile = ref(0)
const aiConfig = ref({
  preset: 'balanced',
  smartQuality: true,
  formatRecommend: true,
  autoOptimize: true,
  batchOptimize: true
})

const formatSize = (bytes) => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

const refreshFiles = async () => {
  try {
    const items = await window.eagle.item.getSelected()
    files.value = items.map(item => ({
      id: item.id,
      name: item.name,
      ext: item.ext,
      path: item.filePath,
      size: item.size,
      thumbnail: item.thumbnailURL
    }))
  } catch (error) {
    ElMessage.error('加载文件失败: ' + error.message)
  }
}

const removeFile = (index) => {
  files.value.splice(index, 1)
}

const startConversion = async () => {
  isConverting.value = true
  currentFile.value = 0
  
  try {
    // TODO: 调用Rust CLI进行AI转换
    for (let i = 0; i < files.value.length; i++) {
      currentFile.value = i + 1
      progress.value = Math.round((i + 1) / files.value.length * 100)
      
      // 模拟转换
      await new Promise(resolve => setTimeout(resolve, 1000))
    }
    
    ElMessage.success(`✅ 转换完成: ${files.value.length} 个文件`)
    await refreshFiles()
  } catch (error) {
    ElMessage.error('转换失败: ' + error.message)
  } finally {
    isConverting.value = false
  }
}

onMounted(() => {
  if (window.eagle) {
    window.eagle.onPluginCreate(() => {
      console.log('[PIXLY AI] Plugin created')
    })
    
    window.eagle.onPluginRun(() => {
      refreshFiles()
    })
    
    window.eagle.onPluginShow(() => {
      refreshFiles()
    })
  }
})
</script>

<style scoped>
.pixly-ai-app {
  width: 100%;
  height: 100vh;
  background: var(--el-bg-color);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo {
  width: 40px;
  height: 40px;
}

.title-group h1 {
  margin: 0;
  font-size: 18px;
}

.title-group p {
  margin: 0;
  font-size: 12px;
  opacity: 0.6;
}

.empty-icon {
  font-size: 80px;
}

.main-grid {
  display: grid;
  grid-template-columns: 350px 1fr;
  gap: 20px;
  height: 100%;
}

.option-group {
  margin-bottom: 20px;
}

.option-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 600;
}

.preset-group {
  width: 100%;
}

.file-list {
  max-height: 500px;
  overflow-y: auto;
  margin-bottom: 16px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  margin-bottom: 8px;
  background: var(--el-fill-color-light);
}

.file-thumb {
  width: 48px;
  height: 48px;
  object-fit: cover;
  border-radius: 4px;
}

.file-info {
  flex: 1;
}

.file-name {
  font-weight: 600;
  margin-bottom: 4px;
}

.file-meta {
  font-size: 12px;
  opacity: 0.6;
}

.convert-btn {
  width: 100%;
}

.progress-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 16px;
  background: var(--el-bg-color);
  border-top: 1px solid var(--el-border-color);
}
</style>
