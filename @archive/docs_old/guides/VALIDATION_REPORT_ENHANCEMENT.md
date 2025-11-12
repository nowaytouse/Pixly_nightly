# 📊 验证报告可视化增强方案

> **目标**: Eagle UI中展示完整验证链路、参数对比、错误追踪  
> **原则**: 透明化、用户友好、可操作

## 🎯 核心功能

### 1. 验证链路可视化

显示参数从UI→Rust→AI的完整流动路径：

```
┌─────────────────────────────────────────────┐
│          📸 UI参数快照                       │
│  quality: 85, speed: 6, lossless: false    │
└──────────────┬──────────────────────────────┘
               │ ✅ 完整性验证通过
               ▼
┌─────────────────────────────────────────────┐
│          🦀 Rust接收参数                     │
│  quality: 85, speed: 6, lossless: false    │
└──────────────┬──────────────────────────────┘
               │ ✅ 参数验证通过
               ▼
┌─────────────────────────────────────────────┐
│          🤖 AI预测参数 (如需要)              │
│  confidence: 0.85, params_source: user      │
└──────────────┬──────────────────────────────┘
               │ ✅ AI验证通过
               ▼
┌─────────────────────────────────────────────┐
│          ⚙️  实际执行参数                    │
│  quality: 85, speed: 6, lossless: false    │
│  strategy: CLI AVIF (avifenc)               │
└──────────────┬──────────────────────────────┘
               │ ✅ 转换成功
               ▼
┌─────────────────────────────────────────────┐
│          📤 参数回显验证                     │
│  actualParams与snapshot一致: ✅             │
└─────────────────────────────────────────────┘
```

### 2. 参数对比表格

| 参数 | UI设置 | Rust接收 | 实际使用 | 状态 |
|------|--------|---------|---------|------|
| quality | 85 | 85 | 85 | ✅ 一致 |
| speed | 6 | 6 | 6 | ✅ 一致 |
| lossless | false | false | false | ✅ 一致 |
| format | avif | avif | avif | ✅ 一致 |

### 3. 验证报告卡片

```
┌─────────────────────────────────────────────────┐
│  ✅ 验证完成 - test.jpg                          │
├─────────────────────────────────────────────────┤
│  📊 验证统计:                                     │
│    • 参数验证: 通过 (4/4)                        │
│    • 完整性验证: 通过                            │
│    • AI置信度: 85% ✅                            │
│    • 执行时间: 125ms                             │
│                                                  │
│  📋 参数来源: user                               │
│  🔧 使用策略: CLI AVIF (avifenc)                │
│                                                  │
│  [查看详情] [导出报告]                           │
└─────────────────────────────────────────────────┘
```

### 4. 错误详情展开

```
┌─────────────────────────────────────────────────┐
│  ❌ 验证失败 - broken.jpg                        │
├─────────────────────────────────────────────────┤
│  错误码: PIXLY-CORE-VAL-001                      │
│  错误消息: Quality参数超出范围: 150 (应为1-100)  │
│                                                  │
│  📍 错误位置: Rust Core → 参数验证器              │
│  ⏰ 发生时间: 2025-11-10 19:30:45               │
│  🔍 追踪ID: req-abc123                          │
│                                                  │
│  💡 建议:                                        │
│    • 请将quality值调整到1-100之间                │
│    • 当前值: 150                                 │
│    • 推荐值: 85                                  │
│                                                  │
│  [重新尝试] [查看文档] [复制错误信息]            │
└─────────────────────────────────────────────────┘
```

## 🎨 UI设计

### Eagle通知样式

#### 成功通知（绿色）

```javascript
eagle.notification.show({
    title: '✅ 转换完成',
    message: '参数验证通过，转换成功',
    type: 'success',
    duration: 3000,
    actions: [
        {
            text: '查看报告',
            callback: () => showValidationReport()
        }
    ]
});
```

#### 警告通知（黄色）

```javascript
eagle.notification.show({
    title: '⚠️ AI置信度较低',
    message: '置信度: 55%，建议手动检查参数',
    type: 'warning',
    duration: 5000,
    actions: [
        {
            text: '查看详情',
            callback: () => showAIConfidenceDetails()
        },
        {
            text: '手动调整',
            callback: () => openManualMode()
        }
    ]
});
```

#### 错误通知（红色）

```javascript
eagle.notification.show({
    title: '❌ 验证失败',
    message: 'PIXLY-CORE-VAL-001: Quality参数超出范围',
    type: 'error',
    duration: 0, // 不自动关闭
    actions: [
        {
            text: '查看错误',
            callback: () => showErrorDetails()
        },
        {
            text: '修正参数',
            callback: () => fixParameters()
        }
    ]
});
```

### 验证报告模态框

```html
<div class="validation-report-modal">
  <div class="modal-header">
    <h2>🛡️ 参数验证报告</h2>
    <span class="close-btn">×</span>
  </div>
  
  <div class="modal-body">
    <!-- 验证链路 -->
    <section class="validation-chain">
      <h3>验证链路</h3>
      <div class="chain-step success">
        <span class="step-icon">📸</span>
        <span class="step-name">UI参数快照</span>
        <span class="step-status">✅</span>
      </div>
      <div class="chain-connector"></div>
      <div class="chain-step success">
        <span class="step-icon">🦀</span>
        <span class="step-name">Rust参数验证</span>
        <span class="step-status">✅</span>
      </div>
      <div class="chain-connector"></div>
      <div class="chain-step success">
        <span class="step-icon">🤖</span>
        <span class="step-name">AI参数预测</span>
        <span class="step-status">✅</span>
      </div>
      <div class="chain-connector"></div>
      <div class="chain-step success">
        <span class="step-icon">📤</span>
        <span class="step-name">参数回显验证</span>
        <span class="step-status">✅</span>
      </div>
    </section>

    <!-- 参数对比 -->
    <section class="parameter-comparison">
      <h3>参数对比</h3>
      <table class="comparison-table">
        <thead>
          <tr>
            <th>参数</th>
            <th>UI设置</th>
            <th>实际使用</th>
            <th>状态</th>
          </tr>
        </thead>
        <tbody>
          <tr class="match">
            <td>quality</td>
            <td>85</td>
            <td>85</td>
            <td><span class="status-icon">✅</span></td>
          </tr>
          <tr class="match">
            <td>speed</td>
            <td>6</td>
            <td>6</td>
            <td><span class="status-icon">✅</span></td>
          </tr>
        </tbody>
      </table>
    </section>

    <!-- 验证统计 -->
    <section class="validation-stats">
      <h3>验证统计</h3>
      <div class="stats-grid">
        <div class="stat-item">
          <span class="stat-label">参数验证</span>
          <span class="stat-value success">通过 (4/4)</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">完整性验证</span>
          <span class="stat-value success">通过</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">AI置信度</span>
          <span class="stat-value success">85%</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">执行时间</span>
          <span class="stat-value">125ms</span>
        </div>
      </div>
    </section>
  </div>

  <div class="modal-footer">
    <button class="btn-secondary" onclick="exportReport()">导出报告</button>
    <button class="btn-primary" onclick="closeModal()">关闭</button>
  </div>
</div>
```

### CSS样式

```css
/* 验证报告模态框 */
.validation-report-modal {
  width: 800px;
  max-height: 90vh;
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

.modal-header {
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-body {
  padding: 20px;
  max-height: calc(90vh - 150px);
  overflow-y: auto;
}

/* 验证链路 */
.validation-chain {
  margin-bottom: 30px;
}

.chain-step {
  display: flex;
  align-items: center;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 8px;
  margin-bottom: 8px;
}

.chain-step.success {
  background: #d4edda;
  border-left: 4px solid #28a745;
}

.chain-step.error {
  background: #f8d7da;
  border-left: 4px solid #dc3545;
}

.chain-step.warning {
  background: #fff3cd;
  border-left: 4px solid #ffc107;
}

.step-icon {
  font-size: 24px;
  margin-right: 12px;
}

.step-name {
  flex: 1;
  font-weight: 500;
}

.step-status {
  font-size: 20px;
}

.chain-connector {
  width: 2px;
  height: 20px;
  background: #dee2e6;
  margin-left: 30px;
}

/* 参数对比表格 */
.comparison-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 12px;
}

.comparison-table th {
  background: #f8f9fa;
  padding: 12px;
  text-align: left;
  font-weight: 600;
  border-bottom: 2px solid #dee2e6;
}

.comparison-table td {
  padding: 10px 12px;
  border-bottom: 1px solid #dee2e6;
}

.comparison-table tr.match {
  background: #f0fff4;
}

.comparison-table tr.mismatch {
  background: #fff5f5;
}

.status-icon {
  font-size: 16px;
}

/* 验证统计 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
  margin-top: 12px;
}

.stat-item {
  background: #f8f9fa;
  padding: 16px;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
}

.stat-label {
  font-size: 14px;
  color: #6c757d;
  margin-bottom: 8px;
}

.stat-value {
  font-size: 20px;
  font-weight: 600;
}

.stat-value.success {
  color: #28a745;
}

.stat-value.warning {
  color: #ffc107;
}

.stat-value.error {
  color: #dc3545;
}

/* 按钮 */
.modal-footer {
  padding: 20px;
  background: #f8f9fa;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn-primary,
.btn-secondary {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #667eea;
  color: white;
}

.btn-primary:hover {
  background: #5568d3;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background: #5a6268;
}
```

## 💻 JavaScript实现

### 验证报告生成器

```javascript
// plugin/converter/js/plugin-modules/validation-report-ui.js

class ValidationReportUI {
    constructor() {
        this.modal = null;
    }

    /**
     * 显示验证报告
     */
    show(validationResult) {
        this.modal = this.createModal(validationResult);
        document.body.appendChild(this.modal);
        this.animateIn();
    }

    /**
     * 创建模态框
     */
    createModal(result) {
        const modal = document.createElement('div');
        modal.className = 'validation-report-overlay';
        modal.innerHTML = `
            <div class="validation-report-modal">
                ${this.renderHeader()}
                ${this.renderBody(result)}
                ${this.renderFooter()}
            </div>
        `;

        // 绑定事件
        modal.querySelector('.close-btn').addEventListener('click', () => this.close());
        modal.querySelector('.btn-export').addEventListener('click', () => this.exportReport(result));
        modal.querySelector('.btn-close').addEventListener('click', () => this.close());

        return modal;
    }

    /**
     * 渲染头部
     */
    renderHeader() {
        return `
            <div class="modal-header">
                <h2>🛡️ 参数验证报告</h2>
                <span class="close-btn">×</span>
            </div>
        `;
    }

    /**
     * 渲染主体
     */
    renderBody(result) {
        return `
            <div class="modal-body">
                ${this.renderValidationChain(result.chain)}
                ${this.renderParameterComparison(result.comparison)}
                ${this.renderStatistics(result.stats)}
                ${result.errors ? this.renderErrors(result.errors) : ''}
            </div>
        `;
    }

    /**
     * 渲染验证链路
     */
    renderValidationChain(chain) {
        const steps = [
            { id: 'snapshot', name: 'UI参数快照', icon: '📸' },
            { id: 'rustValidation', name: 'Rust参数验证', icon: '🦀' },
            { id: 'aiPrediction', name: 'AI参数预测', icon: '🤖' },
            { id: 'paramEcho', name: '参数回显验证', icon: '📤' },
        ];

        const stepsHTML = steps.map((step, index) => {
            const status = chain[step.id] || {};
            const statusClass = status.passed ? 'success' : (status.warning ? 'warning' : 'error');
            const statusIcon = status.passed ? '✅' : (status.warning ? '⚠️' : '❌');

            return `
                <div class="chain-step ${statusClass}">
                    <span class="step-icon">${step.icon}</span>
                    <span class="step-name">${step.name}</span>
                    <span class="step-status">${statusIcon}</span>
                </div>
                ${index < steps.length - 1 ? '<div class="chain-connector"></div>' : ''}
            `;
        }).join('');

        return `
            <section class="validation-chain">
                <h3>验证链路</h3>
                ${stepsHTML}
            </section>
        `;
    }

    /**
     * 渲染参数对比
     */
    renderParameterComparison(comparison) {
        const rows = Object.entries(comparison).map(([param, data]) => {
            const match = data.ui === data.actual;
            const statusIcon = match ? '✅' : '❌';
            const rowClass = match ? 'match' : 'mismatch';

            return `
                <tr class="${rowClass}">
                    <td>${param}</td>
                    <td>${data.ui}</td>
                    <td>${data.actual}</td>
                    <td><span class="status-icon">${statusIcon}</span></td>
                </tr>
            `;
        }).join('');

        return `
            <section class="parameter-comparison">
                <h3>参数对比</h3>
                <table class="comparison-table">
                    <thead>
                        <tr>
                            <th>参数</th>
                            <th>UI设置</th>
                            <th>实际使用</th>
                            <th>状态</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${rows}
                    </tbody>
                </table>
            </section>
        `;
    }

    /**
     * 渲染统计信息
     */
    renderStatistics(stats) {
        const items = [
            {
                label: '参数验证',
                value: stats.paramValidation.passed ? '通过' : '失败',
                class: stats.paramValidation.passed ? 'success' : 'error',
            },
            {
                label: '完整性验证',
                value: stats.integrityCheck.passed ? '通过' : '失败',
                class: stats.integrityCheck.passed ? 'success' : 'error',
            },
            {
                label: 'AI置信度',
                value: stats.aiConfidence ? `${Math.round(stats.aiConfidence * 100)}%` : 'N/A',
                class: stats.aiConfidence >= 0.7 ? 'success' : (stats.aiConfidence >= 0.5 ? 'warning' : 'error'),
            },
            {
                label: '执行时间',
                value: `${stats.executionTime}ms`,
                class: '',
            },
        ];

        const itemsHTML = items.map(item => `
            <div class="stat-item">
                <span class="stat-label">${item.label}</span>
                <span class="stat-value ${item.class}">${item.value}</span>
            </div>
        `).join('');

        return `
            <section class="validation-stats">
                <h3>验证统计</h3>
                <div class="stats-grid">
                    ${itemsHTML}
                </div>
            </section>
        `;
    }

    /**
     * 渲染错误信息
     */
    renderErrors(errors) {
        const errorsHTML = errors.map(error => `
            <div class="error-item">
                <div class="error-header">
                    <span class="error-code">${error.code}</span>
                    <span class="error-severity">${error.severity}</span>
                </div>
                <div class="error-message">${error.message}</div>
                <div class="error-suggestion">
                    💡 建议: ${error.suggestion}
                </div>
            </div>
        `).join('');

        return `
            <section class="validation-errors">
                <h3>❌ 错误详情</h3>
                ${errorsHTML}
            </section>
        `;
    }

    /**
     * 渲染页脚
     */
    renderFooter() {
        return `
            <div class="modal-footer">
                <button class="btn-secondary btn-export">导出报告</button>
                <button class="btn-primary btn-close">关闭</button>
            </div>
        `;
    }

    /**
     * 导出报告
     */
    exportReport(result) {
        const report = {
            timestamp: new Date().toISOString(),
            ...result,
        };

        const blob = new Blob([JSON.stringify(report, null, 2)], {
            type: 'application/json',
        });

        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `validation-report-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    }

    /**
     * 动画显示
     */
    animateIn() {
        requestAnimationFrame(() => {
            this.modal.classList.add('show');
        });
    }

    /**
     * 关闭模态框
     */
    close() {
        this.modal.classList.remove('show');
        setTimeout(() => {
            this.modal.remove();
        }, 300);
    }
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { ValidationReportUI };
}
```

## 📱 集成到Eagle

### 触发验证报告

```javascript
// 转换完成后自动显示
async function convertImage(file, options) {
    try {
        // 1. 捕获参数快照
        const snapshot = paramIntegrityValidator.captureSnapshot(options);

        // 2. 执行转换
        const result = await window.rustCLI.convertImage(file, options);

        // 3. 验证参数完整性
        const validation = paramIntegrityValidator.validateIntegrity(
            snapshot,
            result.actualParams
        );

        // 4. 显示验证报告（如有需要）
        if (validation.hasMismatches || validation.hasWarnings) {
            const reportUI = new ValidationReportUI();
            reportUI.show({
                chain: validation.chain,
                comparison: validation.comparison,
                stats: validation.stats,
                errors: validation.errors,
            });
        }

        // 5. 显示简要通知
        eagle.notification.show({
            title: validation.isValid ? '✅ 转换完成' : '⚠️ 转换完成（有警告）',
            message: `参数验证: ${validation.isValid ? '通过' : '存在差异'}`,
            type: validation.isValid ? 'success' : 'warning',
        });

    } catch (error) {
        // 错误处理
        eagle.notification.show({
            title: '❌ 转换失败',
            message: error.message,
            type: 'error',
        });
    }
}
```

---

**可视化验证报告，让参数流动一目了然！** 📊
