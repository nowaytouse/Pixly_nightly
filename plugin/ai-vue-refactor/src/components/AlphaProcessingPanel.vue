<template>
  <div class="alpha-processing-panel">
    <div class="panel-header">
      <h3>🎨 {{ $t('alphaPanel.title') }}</h3>
      <p class="subtitle">{{ $t('alphaPanel.subtitle') }}</p>
    </div>

    <!-- 透明度检测状态 -->
    <div class="detection-section">
      <h4>🔍 {{ $t('alphaPanel.detection') }}</h4>
      <div class="status-grid">
        <div class="status-item" :class="{ active: alphaStatus.detected }">
          <span class="icon">{{ alphaStatus.detected ? '✅' : '❌' }}</span>
          <span class="label">{{ $t('alphaPanel.alphaDetected') }}</span>
          <span class="value">{{ alphaStatus.detected ? $t('common.yes') : $t('common.no') }}</span>
        </div>
        <div class="status-item">
          <span class="icon">📊</span>
          <span class="label">{{ $t('alphaPanel.alphaPixels') }}</span>
          <span class="value">{{ alphaStatus.alphaPixelCount }} ({{ alphaStatus.alphaPercentage }}%)</span>
        </div>
        <div class="status-item">
          <span class="icon">🎯</span>
          <span class="label">{{ $t('alphaPanel.alphaType') }}</span>
          <span class="value">{{ $t(`alphaPanel.types.${alphaStatus.type}`) }}</span>
        </div>
      </div>
    </div>

    <!-- 处理流程 -->
    <div class="process-section">
      <h4>⚙️ {{ $t('alphaPanel.processFlow') }}</h4>
      <div class="process-steps">
        <div 
          v-for="(step, index) in processSteps" 
          :key="index"
          class="process-step"
          :class="{ 
            active: step.status === 'active',
            completed: step.status === 'completed',
            pending: step.status === 'pending'
          }"
        >
          <div class="step-number">{{ index + 1 }}</div>
          <div class="step-content">
            <div class="step-title">{{ step.title }}</div>
            <div class="step-description">{{ step.description }}</div>
            <div v-if="step.details" class="step-details">
              <div v-for="(detail, key) in step.details" :key="key" class="detail-item">
                <span class="detail-key">{{ key }}:</span>
                <span class="detail-value">{{ detail }}</span>
              </div>
            </div>
          </div>
          <div class="step-status">
            <span v-if="step.status === 'completed'">✅</span>
            <span v-else-if="step.status === 'active'">⏳</span>
            <span v-else>⏸️</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 格式兼容性 -->
    <div class="compatibility-section">
      <h4>🎯 {{ $t('alphaPanel.formatCompatibility') }}</h4>
      <div class="format-grid">
        <div 
          v-for="format in formatCompatibility" 
          :key="format.name"
          class="format-item"
          :class="{ 
            supported: format.alphaSupport === 'full',
            partial: format.alphaSupport === 'partial',
            unsupported: format.alphaSupport === 'none'
          }"
        >
          <div class="format-name">{{ format.name.toUpperCase() }}</div>
          <div class="format-support">
            <span class="support-icon">
              {{ format.alphaSupport === 'full' ? '✅' : format.alphaSupport === 'partial' ? '⚠️' : '❌' }}
            </span>
            <span class="support-text">{{ $t(`alphaPanel.support.${format.alphaSupport}`) }}</span>
          </div>
          <div class="format-details">
            <div class="detail-row">
              <span>{{ $t('alphaPanel.quality') }}:</span>
              <span>{{ format.quality }}</span>
            </div>
            <div class="detail-row">
              <span>{{ $t('alphaPanel.compression') }}:</span>
              <span>{{ format.compression }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 高级处理选项 -->
    <div class="advanced-section">
      <h4>🔧 {{ $t('alphaPanel.advancedOptions') }}</h4>
      
      <!-- Alpha质量控制 -->
      <div class="option-group">
        <label class="option-label">
          <span>{{ $t('alphaPanel.alphaQuality') }}</span>
          <input 
            type="range" 
            v-model.number="alphaQuality" 
            min="0" 
            max="100" 
            step="1"
            @input="onAlphaQualityChange"
          />
          <span class="value-display">{{ alphaQuality }}</span>
        </label>
        <p class="option-hint">{{ $t('alphaPanel.alphaQualityHint') }}</p>
      </div>

      <!-- 预乘Alpha -->
      <div class="option-group">
        <label class="option-label checkbox">
          <input 
            type="checkbox" 
            v-model="premultiplyAlpha"
            @change="onPremultiplyChange"
          />
          <span>{{ $t('alphaPanel.premultiplyAlpha') }}</span>
        </label>
        <p class="option-hint">{{ $t('alphaPanel.premultiplyHint') }}</p>
      </div>

      <!-- Alpha通道分离 -->
      <div class="option-group">
        <label class="option-label checkbox">
          <input 
            type="checkbox" 
            v-model="separateAlpha"
            @change="onSeparateAlphaChange"
          />
          <span>{{ $t('alphaPanel.separateAlpha') }}</span>
        </label>
        <p class="option-hint">{{ $t('alphaPanel.separateAlphaHint') }}</p>
      </div>
    </div>

    <!-- 处理统计 -->
    <div class="stats-section">
      <h4>📊 {{ $t('alphaPanel.statistics') }}</h4>
      <div class="stats-grid">
        <div class="stat-item">
          <div class="stat-label">{{ $t('alphaPanel.totalPixels') }}</div>
          <div class="stat-value">{{ formatNumber(stats.totalPixels) }}</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">{{ $t('alphaPanel.opaquePixels') }}</div>
          <div class="stat-value">{{ formatNumber(stats.opaquePixels) }}</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">{{ $t('alphaPanel.transparentPixels') }}</div>
          <div class="stat-value">{{ formatNumber(stats.transparentPixels) }}</div>
        </div>
        <div class="stat-item">
          <div class="stat-label">{{ $t('alphaPanel.semiTransparent') }}</div>
          <div class="stat-value">{{ formatNumber(stats.semiTransparentPixels) }}</div>
        </div>
      </div>
    </div>

    <!-- 技术细节 -->
    <details class="technical-details">
      <summary>🔬 {{ $t('alphaPanel.technicalDetails') }}</summary>
      <div class="details-content">
        <h5>{{ $t('alphaPanel.internalProcess') }}</h5>
        <ul class="process-list">
          <li>
            <strong>1. {{ $t('alphaPanel.step1Title') }}</strong>
            <p>{{ $t('alphaPanel.step1Desc') }}</p>
            <code>has_alpha = img.color().has_alpha()</code>
          </li>
          <li>
            <strong>2. {{ $t('alphaPanel.step2Title') }}</strong>
            <p>{{ $t('alphaPanel.step2Desc') }}</p>
            <code>alpha_pixels = count(pixel[3] &lt; 255)</code>
          </li>
          <li>
            <strong>3. {{ $t('alphaPanel.step3Title') }}</strong>
            <p>{{ $t('alphaPanel.step3Desc') }}</p>
            <code>type = binary | gradient | complex</code>
          </li>
          <li>
            <strong>4. {{ $t('alphaPanel.step4Title') }}</strong>
            <p>{{ $t('alphaPanel.step4Desc') }}</p>
            <code>format_support = check_alpha_compatibility()</code>
          </li>
          <li>
            <strong>5. {{ $t('alphaPanel.step5Title') }}</strong>
            <p>{{ $t('alphaPanel.step5Desc') }}</p>
            <code>--qalpha {{ alphaQuality }} --premultiply {{ premultiplyAlpha }}</code>
          </li>
        </ul>

        <h5>{{ $t('alphaPanel.algorithmDetails') }}</h5>
        <div class="algorithm-box">
          <pre>{{ algorithmPseudocode }}</pre>
        </div>
      </div>
    </details>
  </div>
</template>

<script>
export default {
  name: 'AlphaProcessingPanel',
  
  data() {
    return {
      alphaStatus: {
        detected: false,
        alphaPixelCount: 0,
        alphaPercentage: 0,
        type: 'none' // none, binary, gradient, complex
      },
      
      processSteps: [
        {
          title: this.$t?.('alphaPanel.steps.detection') || 'Alpha Channel Detection',
          description: 'Scanning image color space for alpha channel',
          status: 'pending',
          details: null
        },
        {
          title: this.$t?.('alphaPanel.steps.analysis') || 'Pixel Analysis',
          description: 'Analyzing alpha values across all pixels',
          status: 'pending',
          details: null
        },
        {
          title: this.$t?.('alphaPanel.steps.classification') || 'Type Classification',
          description: 'Classifying transparency type',
          status: 'pending',
          details: null
        },
        {
          title: this.$t?.('alphaPanel.steps.optimization') || 'Format Optimization',
          description: 'Selecting optimal format for alpha preservation',
          status: 'pending',
          details: null
        },
        {
          title: this.$t?.('alphaPanel.steps.encoding') || 'Alpha Encoding',
          description: 'Encoding with alpha-specific parameters',
          status: 'pending',
          details: null
        }
      ],
      
      formatCompatibility: [
        {
          name: 'webp',
          alphaSupport: 'full',
          quality: 'Excellent',
          compression: 'Lossy/Lossless'
        },
        {
          name: 'avif',
          alphaSupport: 'full',
          quality: 'Excellent',
          compression: 'Lossy/Lossless'
        },
        {
          name: 'png',
          alphaSupport: 'full',
          quality: 'Perfect',
          compression: 'Lossless'
        },
        {
          name: 'jxl',
          alphaSupport: 'full',
          quality: 'Excellent',
          compression: 'Lossy/Lossless'
        },
        {
          name: 'heic',
          alphaSupport: 'partial',
          quality: 'Limited',
          compression: 'Lossy'
        },
        {
          name: 'jpeg',
          alphaSupport: 'none',
          quality: 'N/A',
          compression: 'N/A'
        }
      ],
      
      alphaQuality: 90,
      premultiplyAlpha: false,
      separateAlpha: false,
      
      stats: {
        totalPixels: 0,
        opaquePixels: 0,
        transparentPixels: 0,
        semiTransparentPixels: 0
      },
      
      algorithmPseudocode: `function processAlphaChannel(image):
  // 1. Detect alpha channel
  hasAlpha = checkColorSpace(image)
  if not hasAlpha:
    return NO_ALPHA
  
  // 2. Analyze pixel distribution
  alphaHistogram = [0] * 256
  for each pixel in image:
    alphaValue = pixel.alpha
    alphaHistogram[alphaValue]++
  
  // 3. Classify transparency type
  if alphaHistogram[0] + alphaHistogram[255] > 95%:
    type = BINARY  // Simple cutout
  else if gradientDetected(alphaHistogram):
    type = GRADIENT  // Smooth fade
  else:
    type = COMPLEX  // Mixed transparency
  
  // 4. Select optimal format
  if type == BINARY:
    recommend = [PNG, WebP]
  else if type == GRADIENT:
    recommend = [WebP, AVIF, JXL]
  else:
    recommend = [AVIF, JXL]
  
  // 5. Apply encoding parameters
  if premultiplyAlpha:
    image = premultiplyRGB(image)
  
  encode(image, {
    alphaQuality: alphaQuality,
    alphaCompression: optimal
  })
  
  return SUCCESS`
    }
  },
  
  methods: {
    formatNumber(num) {
      return num.toLocaleString()
    },
    
    onAlphaQualityChange() {
      this.$emit('alpha-quality-change', this.alphaQuality)
    },
    
    onPremultiplyChange() {
      this.$emit('premultiply-change', this.premultiplyAlpha)
    },
    
    onSeparateAlphaChange() {
      this.$emit('separate-alpha-change', this.separateAlpha)
    },
    
    updateAlphaStatus(status) {
      this.alphaStatus = { ...this.alphaStatus, ...status }
    },
    
    updateProcessStep(index, updates) {
      this.processSteps[index] = { ...this.processSteps[index], ...updates }
    },
    
    updateStats(newStats) {
      this.stats = { ...this.stats, ...newStats }
    }
  }
}
</script>

<style scoped>
.alpha-processing-panel {
  padding: 20px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
  max-width: 1200px;
  margin: 0 auto;
}

.panel-header {
  margin-bottom: 24px;
}

.panel-header h3 {
  margin: 0 0 8px 0;
  font-size: 24px;
  color: var(--text-primary, #333);
}

.subtitle {
  margin: 0;
  color: var(--text-secondary, #666);
  font-size: 14px;
}

/* Detection Section */
.detection-section,
.process-section,
.compatibility-section,
.advanced-section,
.stats-section {
  background: white;
  padding: 16px;
  border-radius: 6px;
  margin-bottom: 16px;
}

.detection-section h4,
.process-section h4,
.compatibility-section h4,
.advanced-section h4,
.stats-section h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: var(--text-primary, #333);
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  background: var(--bg-tertiary, #fafafa);
  border-radius: 4px;
  border: 2px solid transparent;
}

.status-item.active {
  border-color: var(--color-success, #4caf50);
  background: var(--bg-success-light, #e8f5e9);
}

.status-item .icon {
  font-size: 20px;
}

.status-item .label {
  flex: 1;
  font-size: 13px;
  color: var(--text-secondary, #666);
}

.status-item .value {
  font-weight: 600;
  color: var(--text-primary, #333);
}

/* Process Steps */
.process-steps {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.process-step {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 12px;
  background: var(--bg-tertiary, #fafafa);
  border-radius: 4px;
  border-left: 4px solid var(--color-border, #ddd);
}

.process-step.active {
  border-left-color: var(--color-primary, #2196f3);
  background: var(--bg-primary-light, #e3f2fd);
}

.process-step.completed {
  border-left-color: var(--color-success, #4caf50);
  opacity: 0.7;
}

.step-number {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-primary, #2196f3);
  color: white;
  border-radius: 50%;
  font-weight: 600;
  flex-shrink: 0;
}

.step-content {
  flex: 1;
}

.step-title {
  font-weight: 600;
  margin-bottom: 4px;
  color: var(--text-primary, #333);
}

.step-description {
  font-size: 13px;
  color: var(--text-secondary, #666);
  margin-bottom: 8px;
}

.step-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
  padding: 8px;
  background: white;
  border-radius: 4px;
}

.detail-item {
  display: flex;
  gap: 8px;
}

.detail-key {
  font-weight: 600;
  color: var(--text-secondary, #666);
}

.detail-value {
  color: var(--text-primary, #333);
}

.step-status {
  font-size: 20px;
  flex-shrink: 0;
}

/* Format Grid */
.format-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 12px;
}

.format-item {
  padding: 12px;
  border-radius: 4px;
  border: 2px solid var(--color-border, #ddd);
  background: white;
}

.format-item.supported {
  border-color: var(--color-success, #4caf50);
  background: var(--bg-success-light, #e8f5e9);
}

.format-item.partial {
  border-color: var(--color-warning, #ff9800);
  background: var(--bg-warning-light, #fff3e0);
}

.format-item.unsupported {
  border-color: var(--color-error, #f44336);
  background: var(--bg-error-light, #ffebee);
  opacity: 0.6;
}

.format-name {
  font-weight: 700;
  font-size: 16px;
  margin-bottom: 8px;
  color: var(--text-primary, #333);
}

.format-support {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 8px;
  font-size: 13px;
}

.format-details {
  font-size: 12px;
  color: var(--text-secondary, #666);
}

.detail-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
}

/* Advanced Options */
.option-group {
  margin-bottom: 16px;
}

.option-label {
  display: flex;
  align-items: center;
  gap: 12px;
  font-weight: 500;
  margin-bottom: 4px;
}

.option-label input[type="range"] {
  flex: 1;
}

.option-label.checkbox {
  cursor: pointer;
}

.value-display {
  min-width: 40px;
  text-align: right;
  font-weight: 600;
  color: var(--color-primary, #2196f3);
}

.option-hint {
  margin: 4px 0 0 0;
  font-size: 12px;
  color: var(--text-secondary, #666);
  font-style: italic;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
}

.stat-item {
  text-align: center;
  padding: 12px;
  background: var(--bg-tertiary, #fafafa);
  border-radius: 4px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary, #666);
  margin-bottom: 4px;
}

.stat-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-primary, #2196f3);
}

/* Technical Details */
.technical-details {
  background: white;
  padding: 16px;
  border-radius: 6px;
  border: 1px solid var(--color-border, #ddd);
}

.technical-details summary {
  cursor: pointer;
  font-weight: 600;
  font-size: 16px;
  color: var(--text-primary, #333);
  user-select: none;
}

.technical-details summary:hover {
  color: var(--color-primary, #2196f3);
}

.details-content {
  margin-top: 16px;
}

.details-content h5 {
  margin: 16px 0 8px 0;
  font-size: 14px;
  color: var(--text-primary, #333);
}

.process-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.process-list li {
  margin-bottom: 16px;
  padding: 12px;
  background: var(--bg-tertiary, #fafafa);
  border-radius: 4px;
  border-left: 3px solid var(--color-primary, #2196f3);
}

.process-list li strong {
  display: block;
  margin-bottom: 4px;
  color: var(--text-primary, #333);
}

.process-list li p {
  margin: 4px 0;
  font-size: 13px;
  color: var(--text-secondary, #666);
}

.process-list li code {
  display: block;
  margin-top: 8px;
  padding: 8px;
  background: #2d2d2d;
  color: #f8f8f2;
  border-radius: 4px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  overflow-x: auto;
}

.algorithm-box {
  background: #2d2d2d;
  color: #f8f8f2;
  padding: 16px;
  border-radius: 4px;
  overflow-x: auto;
}

.algorithm-box pre {
  margin: 0;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.6;
}
</style>
