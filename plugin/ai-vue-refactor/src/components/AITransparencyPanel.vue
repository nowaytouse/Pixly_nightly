<template>
  <div class="ai-transparency-panel">
    <div class="panel-header" @click="toggleExpanded">
      <div class="header-left">
        <span class="icon">🔍</span>
        <h3>{{ $t('aiTransparency.title') }}</h3>
        <span class="badge" :class="{ active: hasDecisionData }">
          {{ hasDecisionData ? $t('aiTransparency.analyzing') : $t('aiTransparency.ready') }}
        </span>
      </div>
      <span class="toggle-icon" :class="{ expanded: isExpanded }">▼</span>
    </div>

    <transition name="slide">
      <div v-if="isExpanded" class="panel-content">
        
        <!-- 文件分析阶段 -->
        <div class="section" v-if="fileAnalysis">
          <div class="section-header">
            <span class="step-number">1</span>
            <h4>{{ $t('aiTransparency.fileAnalysis') }}</h4>
          </div>
          <div class="info-grid">
            <div class="info-item">
              <span class="label">{{ $t('aiTransparency.fileSize') }}</span>
              <span class="value">{{ formatFileSize(fileAnalysis.size) }}</span>
            </div>
            <div class="info-item">
              <span class="label">{{ $t('aiTransparency.dimensions') }}</span>
              <span class="value">{{ fileAnalysis.width }} × {{ fileAnalysis.height }}</span>
            </div>
            <div class="info-item">
              <span class="label">{{ $t('aiTransparency.format') }}</span>
              <span class="value">{{ fileAnalysis.format }}</span>
            </div>
            <div class="info-item">
              <span class="label">{{ $t('aiTransparency.colorDepth') }}</span>
              <span class="value">{{ fileAnalysis.colorDepth }} bit</span>
            </div>
          </div>
        </div>

        <!-- 特征提取阶段 -->
        <div class="section" v-if="featureExtraction">
          <div class="section-header">
            <span class="step-number">2</span>
            <h4>{{ $t('aiTransparency.featureExtraction') }}</h4>
          </div>
          <div class="feature-list">
            <div class="feature-item" v-for="(value, key) in featureExtraction" :key="key">
              <span class="feature-name">{{ $t(`aiTransparency.features.${key}`) }}</span>
              <div class="feature-bar">
                <div class="feature-fill" :style="{ width: `${value * 100}%` }"></div>
              </div>
              <span class="feature-value">{{ (value * 100).toFixed(1) }}%</span>
            </div>
          </div>
        </div>

        <!-- AI决策阶段 -->
        <div class="section" v-if="aiDecision">
          <div class="section-header">
            <span class="step-number">3</span>
            <h4>{{ $t('aiTransparency.aiDecision') }}</h4>
          </div>
          <div class="decision-card">
            <div class="decision-item">
              <span class="decision-label">{{ $t('aiTransparency.recommendedFormat') }}</span>
              <span class="decision-value highlight">{{ aiDecision.format }}</span>
            </div>
            <div class="decision-item">
              <span class="decision-label">{{ $t('aiTransparency.quality') }}</span>
              <span class="decision-value">{{ aiDecision.quality }}</span>
            </div>
            <div class="decision-item">
              <span class="decision-label">{{ $t('aiTransparency.speed') }}</span>
              <span class="decision-value">{{ aiDecision.speed }}</span>
            </div>
            <div class="decision-item">
              <span class="decision-label">{{ $t('aiTransparency.confidence') }}</span>
              <span class="decision-value">{{ (aiDecision.confidence * 100).toFixed(1) }}%</span>
            </div>
          </div>
          
          <!-- 决策理由 -->
          <div class="reasoning" v-if="aiDecision.reasoning">
            <h5>{{ $t('aiTransparency.reasoning') }}</h5>
            <ul>
              <li v-for="(reason, index) in aiDecision.reasoning" :key="index">
                {{ reason }}
              </li>
            </ul>
          </div>
        </div>

        <!-- 处理流程阶段 -->
        <div class="section" v-if="processingSteps.length > 0">
          <div class="section-header">
            <span class="step-number">4</span>
            <h4>{{ $t('aiTransparency.processingSteps') }}</h4>
          </div>
          <div class="timeline">
            <div 
              class="timeline-item" 
              v-for="(step, index) in processingSteps" 
              :key="index"
              :class="{ active: step.status === 'active', completed: step.status === 'completed' }"
            >
              <div class="timeline-marker">
                <span v-if="step.status === 'completed'">✓</span>
                <span v-else-if="step.status === 'active'" class="spinner">⟳</span>
                <span v-else>{{ index + 1 }}</span>
              </div>
              <div class="timeline-content">
                <div class="step-name">{{ step.name }}</div>
                <div class="step-detail" v-if="step.detail">{{ step.detail }}</div>
                <div class="step-time" v-if="step.duration">{{ step.duration }}ms</div>
              </div>
            </div>
          </div>
        </div>

        <!-- 性能统计 -->
        <div class="section" v-if="performanceStats">
          <div class="section-header">
            <span class="step-number">5</span>
            <h4>{{ $t('aiTransparency.performance') }}</h4>
          </div>
          <div class="stats-grid">
            <div class="stat-card">
              <div class="stat-value">{{ performanceStats.totalTime }}ms</div>
              <div class="stat-label">{{ $t('aiTransparency.totalTime') }}</div>
            </div>
            <div class="stat-card">
              <div class="stat-value">{{ formatFileSize(performanceStats.originalSize) }}</div>
              <div class="stat-label">{{ $t('aiTransparency.originalSize') }}</div>
            </div>
            <div class="stat-card">
              <div class="stat-value">{{ formatFileSize(performanceStats.compressedSize) }}</div>
              <div class="stat-label">{{ $t('aiTransparency.compressedSize') }}</div>
            </div>
            <div class="stat-card">
              <div class="stat-value">{{ performanceStats.compressionRatio }}%</div>
              <div class="stat-label">{{ $t('aiTransparency.compressionRatio') }}</div>
            </div>
          </div>
        </div>

      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, watch, inject, getCurrentInstance } from 'vue'

const t = inject('t')

// Expose t as $t for template usage
const instance = getCurrentInstance()
if (instance) {
  instance.appContext.config.globalProperties.$t = t
}

const props = defineProps({
  decisionData: {
    type: Object,
    default: null
  }
})

const isExpanded = ref(true)
const fileAnalysis = ref(null)
const featureExtraction = ref(null)
const aiDecision = ref(null)
const processingSteps = ref([])
const performanceStats = ref(null)

const hasDecisionData = computed(() => {
  return !!(fileAnalysis.value || aiDecision.value || processingSteps.value.length > 0)
})

const toggleExpanded = () => {
  isExpanded.value = !isExpanded.value
}

const formatFileSize = (bytes) => {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return (bytes / Math.pow(k, i)).toFixed(2) + ' ' + sizes[i]
}

// 监听外部数据更新
watch(() => props.decisionData, (newData) => {
  if (newData) {
    fileAnalysis.value = newData.fileAnalysis
    featureExtraction.value = newData.featureExtraction
    aiDecision.value = newData.aiDecision
    processingSteps.value = newData.processingSteps || []
    performanceStats.value = newData.performanceStats
  }
}, { deep: true, immediate: true })

// 暴露方法供父组件调用
defineExpose({
  updateFileAnalysis: (data) => { fileAnalysis.value = data },
  updateFeatureExtraction: (data) => { featureExtraction.value = data },
  updateAIDecision: (data) => { aiDecision.value = data },
  addProcessingStep: (step) => { processingSteps.value.push(step) },
  updateProcessingStep: (index, updates) => {
    if (processingSteps.value[index]) {
      Object.assign(processingSteps.value[index], updates)
    }
  },
  updatePerformanceStats: (data) => { performanceStats.value = data },
  reset: () => {
    fileAnalysis.value = null
    featureExtraction.value = null
    aiDecision.value = null
    processingSteps.value = []
    performanceStats.value = null
  }
})
</script>

<style scoped>
.ai-transparency-panel {
  background: var(--color-bg-secondary);
  border-radius: 8px;
  overflow: hidden;
  margin-top: 16px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  cursor: pointer;
  user-select: none;
  transition: background 0.2s;
}

.panel-header:hover {
  background: var(--color-bg-hover);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.icon {
  font-size: 20px;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.badge {
  padding: 4px 12px;
  background: var(--color-border);
  color: var(--text-secondary);
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.3s;
}

.badge.active {
  background: var(--color-primary);
  color: white;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

.toggle-icon {
  transition: transform 0.3s;
  color: var(--text-secondary);
}

.toggle-icon.expanded {
  transform: rotate(180deg);
}

.panel-content {
  padding: 0 16px 16px;
}

.section {
  margin-bottom: 24px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.step-number {
  width: 28px;
  height: 28px;
  background: var(--color-primary);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
}

.section-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px;
  background: var(--color-bg);
  border-radius: 6px;
}

.info-item .label {
  font-size: 12px;
  color: var(--text-secondary);
}

.info-item .value {
  font-size: 14px;
  font-weight: 600;
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.feature-item {
  display: grid;
  grid-template-columns: 120px 1fr 60px;
  align-items: center;
  gap: 12px;
}

.feature-name {
  font-size: 13px;
  color: var(--text-secondary);
}

.feature-bar {
  height: 8px;
  background: var(--color-border);
  border-radius: 4px;
  overflow: hidden;
}

.feature-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--color-primary), var(--color-success));
  transition: width 0.5s ease;
}

.feature-value {
  font-size: 13px;
  font-weight: 600;
  text-align: right;
}

.decision-card {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
  margin-bottom: 16px;
}

.decision-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px;
  background: var(--color-bg);
  border-radius: 6px;
}

.decision-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.decision-value {
  font-size: 16px;
  font-weight: 600;
}

.decision-value.highlight {
  color: var(--color-primary);
}

.reasoning {
  padding: 12px;
  background: var(--color-bg);
  border-radius: 6px;
  border-left: 3px solid var(--color-primary);
}

.reasoning h5 {
  margin: 0 0 8px 0;
  font-size: 13px;
  font-weight: 600;
}

.reasoning ul {
  margin: 0;
  padding-left: 20px;
}

.reasoning li {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.timeline {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.timeline-item {
  display: flex;
  gap: 12px;
  position: relative;
}

.timeline-item:not(:last-child)::before {
  content: '';
  position: absolute;
  left: 14px;
  top: 32px;
  width: 2px;
  height: calc(100% + 16px);
  background: var(--color-border);
}

.timeline-marker {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--color-border);
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 14px;
  flex-shrink: 0;
  transition: all 0.3s;
}

.timeline-item.active .timeline-marker {
  background: var(--color-primary);
  color: white;
}

.timeline-item.completed .timeline-marker {
  background: var(--color-success);
  color: white;
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.timeline-content {
  flex: 1;
  padding: 4px 0;
}

.step-name {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.step-detail {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.step-time {
  font-size: 11px;
  color: var(--text-tertiary);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 12px;
}

.stat-card {
  padding: 16px;
  background: var(--color-bg);
  border-radius: 6px;
  text-align: center;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-primary);
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
  max-height: 2000px;
}

.slide-enter-from,
.slide-leave-to {
  max-height: 0;
  opacity: 0;
}
</style>
