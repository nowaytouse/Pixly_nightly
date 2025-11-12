# Pixly UI 改进需求文档

> **文档版本**: 1.0.0  
> **创建日期**: 2025-01-08  
> **状态**: 待实现

---

## 📋 概述

本文档记录了 Pixly 插件 UI 层面需要改进和增强的功能需求，基于测试报告反馈和用户体验优化建议。

---

## 🎨 UI 功能改进清单

### 1. 主题切换功能缺失 ⚠️

**问题描述**：
- 无主题切换功能，或在模块化重构后被误删除
- 用户无法自定义界面主题（亮色/暗色模式）

**影响**：
- 用户体验下降，无法适应不同使用场景
- 在暗色 Eagle 环境中可能产生视觉不适

**优先级**：中

**实现建议**：
```javascript
// 在 theme.js 中实现主题切换
window.PIXLY = window.PIXLY || {};
window.PIXLY.theme = {
  current: 'light', // 'light' | 'dark' | 'auto'
  
  toggle() {
    this.current = this.current === 'light' ? 'dark' : 'light';
    this.apply();
  },
  
  apply() {
    document.documentElement.setAttribute('data-theme', this.current);
    localStorage.setItem('pixly-theme', this.current);
  }
};
```

**UI 位置**：
- 在设置面板或顶部工具栏添加主题切换按钮
- 图标：🌙 (暗色) / ☀️ (亮色)

---

### 2. 下拉菜单显示不持久 ⚠️

**问题描述**：
- 开始转换后的下拉菜单显示不持久
- 菜单快速出现又快速消失
- 用户无法及时看到或操作菜单内容

**影响**：
- 用户体验混乱
- 可能错过重要的操作选项
- 影响转换过程中的控制

**优先级**：高

**实现建议**：
```javascript
// 在 ui-handlers.js 中调整菜单显示逻辑
function showConversionMenu() {
  const menu = document.getElementById('conversionMenu');
  if (menu) {
    menu.style.display = 'block';
    menu.style.opacity = '1';
    menu.style.transition = 'opacity 0.3s ease-in-out';
    
    // 不自动隐藏，等待用户手动关闭或转换完成
    // setTimeout(() => menu.style.display = 'none', 3000); // ❌ 删除自动隐藏
  }
}
```

**修复位置**：
- `ui-handlers.js` - 菜单显示/隐藏逻辑
- 确保菜单在转换过程中持续显示
- 只在用户明确操作或转换完成时关闭

---

### 3. 完善鼠标悬浮提示功能并完善 i18n 🌐

**问题描述**：
- 各控件缺少鼠标悬浮提示（tooltip）
- 现有提示文本未完全国际化
- 用户不清楚某些控件的功能和用途

**影响**：
- 学习曲线陡峭
- 用户需要猜测控件功能
- 国际化不完整

**优先级**：中

**实现建议**：

#### 3.1 添加 tooltip 系统
```javascript
// 在 utils.js 中添加 tooltip 工具函数
function initTooltips() {
  const tooltipElements = document.querySelectorAll('[data-tooltip]');
  
  tooltipElements.forEach(el => {
    el.addEventListener('mouseenter', (e) => {
      const text = e.target.getAttribute('data-tooltip');
      const i18nKey = e.target.getAttribute('data-tooltip-i18n');
      const tooltipText = i18nKey ? window.i18n.t(i18nKey) : text;
      
      showTooltip(e.target, tooltipText);
    });
    
    el.addEventListener('mouseleave', () => {
      hideTooltip();
    });
  });
}
```

#### 3.2 i18n 键值扩展
```json
// i18n.fixed.js 中添加 tooltip 翻译
{
  "tooltips": {
    "quality": "图片质量：数值越高质量越好，文件越大",
    "speed": "编码速度：数值越高速度越快，但压缩率略低",
    "lossless": "无损模式：保持 100% 画质，但文件较大",
    "keepAnimated": "保留动画：GIF/WebP 动画将被保留",
    "mergeXmp": "自动合并 XMP 元数据到目标文件",
    "normalizeFilename": "规范化文件名：移除特殊字符"
  }
}
```

#### 3.3 需要添加 tooltip 的控件
- ✅ 质量滑块（quality slider）
- ✅ 速度滑块（speed slider）
- ✅ 无损模式复选框（lossless checkbox）
- ✅ 保留动画复选框（keep animated）
- ✅ XMP 合并开关（merge XMP）
- ✅ 文件名规范化开关（normalize）
- ✅ 格式选择按钮（format buttons）
- ✅ AI 高级选项（AI advanced options）

---

### 4. 宽屏适配优化 🖥️

**问题描述**：
- 宽屏情况下左右两边过于空旷
- 内部控件未适配宽屏预览窗口
- 布局未充分利用可用空间

**影响**：
- 宽屏显示器使用体验差
- 大量空白浪费屏幕空间
- 控件显得拥挤或过小

**优先级**：中

**实现建议**：

#### 4.1 响应式布局
```css
/* 在 style.css 中添加媒体查询 */
@media (min-width: 1920px) {
  .pixly-container {
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 40px;
  }
  
  .control-panel {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 30px;
  }
  
  .preview-area {
    min-height: 600px;
  }
}

@media (min-width: 2560px) {
  .pixly-container {
    max-width: 2000px;
  }
  
  .control-panel {
    grid-template-columns: repeat(3, 1fr);
  }
}
```

#### 4.2 控件自适应
- 文件选择面板：宽屏下显示更多列
- 预览区域：自动扩展以利用空间
- 控制面板：多列布局而非单列堆叠

---

### 5. 智能模式下简化功能描述 🎯

**问题描述**：
- 智能模式下功能描述过于复杂和专业
- 对普通用户造成认知负担
- 手动模式和智能模式应有不同的描述风格

**影响**：
- 用户困惑，不知道如何选择
- 智能模式的"智能"优势未体现
- 专业术语吓跑新手用户

**优先级**：高

**实现建议**：

#### 5.1 描述分级系统
```javascript
// 根据模式切换描述详细程度
const descriptions = {
  smart: {
    quality: "让 AI 选择最佳质量",
    speed: "自动优化处理速度",
    format: "推荐格式：JXL（最佳压缩）"
  },
  manual: {
    quality: "图片质量 (0-100)：数值越高质量越好，文件越大。推荐 85-95",
    speed: "编码速度 (1-10)：数值越高速度越快，但压缩率略低。推荐 4-7",
    format: "目标格式选择：JXL (新一代格式，最佳压缩), AVIF (现代格式，广泛支持), WebP (兼容性好)"
  }
};

function updateDescriptionsByMode(mode) {
  const descSet = descriptions[mode];
  document.querySelectorAll('[data-desc]').forEach(el => {
    const key = el.getAttribute('data-desc');
    if (descSet[key]) {
      el.textContent = descSet[key];
    }
  });
}
```

#### 5.2 UI 简化规则
**智能模式**：
- ✅ 隐藏高级参数调节
- ✅ 使用简单直白的文案
- ✅ 提供"一键优化"体验
- ✅ 减少技术术语

**手动模式**：
- ✅ 显示所有参数控制
- ✅ 提供详细的参数说明
- ✅ 显示专业术语和建议值
- ✅ 提供参数预览和对比

---

### 6. 增强输出可靠性（多级验证） 🛡️

**问题描述**：
- 缺少输入输出验证机制
- 手动转换时未验证参数匹配性
- 可能出现异常或错误情况

**影响**：
- 转换失败率高
- 错误信息不明确
- 用户数据安全风险

**优先级**：高

**实现建议**：

#### 6.1 多级验证架构
```javascript
// 验证层级
const ValidationLevels = {
  INPUT: 'input',      // 输入验证
  PARAM: 'parameter',  // 参数验证
  FORMAT: 'format',    // 格式兼容性
  OUTPUT: 'output'     // 输出验证
};

class ConversionValidator {
  // Level 1: 输入文件验证
  validateInput(files) {
    const errors = [];
    
    files.forEach(file => {
      // 检查文件存在性
      if (!file.filePath || !fs.existsSync(file.filePath)) {
        errors.push(`文件不存在: ${file.name}`);
      }
      
      // 检查文件格式
      if (!this.isSupportedFormat(file.ext)) {
        errors.push(`不支持的格式: ${file.ext}`);
      }
      
      // 检查文件大小
      if (file.size === 0) {
        errors.push(`文件为空: ${file.name}`);
      }
      
      // 检查文件权限
      if (!this.hasReadPermission(file.filePath)) {
        errors.push(`无读取权限: ${file.name}`);
      }
    });
    
    return { valid: errors.length === 0, errors };
  }
  
  // Level 2: 参数验证
  validateParameters(params, targetFormat) {
    const errors = [];
    
    // 质量范围检查
    if (params.quality < 1 || params.quality > 100) {
      errors.push(`质量参数超出范围: ${params.quality}`);
    }
    
    // 速度范围检查
    if (params.speed < 1 || params.speed > 10) {
      errors.push(`速度参数超出范围: ${params.speed}`);
    }
    
    // 格式特定验证
    if (targetFormat === 'jxl' && params.distance !== undefined) {
      if (params.distance < 0 || params.distance > 15) {
        errors.push(`JXL distance 超出范围: ${params.distance}`);
      }
    }
    
    // 无损模式验证
    if (params.lossless && params.quality !== 100) {
      console.warn(`无损模式下质量将自动调整为 100`);
    }
    
    return { valid: errors.length === 0, errors };
  }
  
  // Level 3: 格式兼容性验证
  validateFormatCompatibility(inputFormat, outputFormat, fileProperties) {
    const errors = [];
    const warnings = [];
    
    // 动画兼容性
    if (fileProperties.isAnimated && outputFormat === 'heic') {
      warnings.push('HEIC 不支持动画，动画效果将丢失');
    }
    
    // 透明度兼容性
    if (fileProperties.hasAlpha && outputFormat === 'heic') {
      warnings.push('HEIC 不完全支持透明度，背景可能变色');
    }
    
    // 色彩空间兼容性
    if (fileProperties.colorSpace === 'CMYK' && ['avif', 'jxl'].includes(outputFormat)) {
      errors.push(`${outputFormat.toUpperCase()} 不支持 CMYK 色彩空间`);
    }
    
    return { valid: errors.length === 0, errors, warnings };
  }
  
  // Level 4: 输出验证
  async validateOutput(outputPath, expectedProperties) {
    const errors = [];
    
    // 文件存在性
    if (!fs.existsSync(outputPath)) {
      errors.push('输出文件未生成');
      return { valid: false, errors };
    }
    
    // 文件大小合理性
    const outputSize = fs.statSync(outputPath).size;
    if (outputSize === 0) {
      errors.push('输出文件为空');
    }
    
    // 格式验证
    try {
      const actualFormat = await this.detectFileFormat(outputPath);
      if (actualFormat !== expectedProperties.format) {
        errors.push(`输出格式不匹配: 期望 ${expectedProperties.format}, 实际 ${actualFormat}`);
      }
    } catch (e) {
      errors.push(`无法验证输出格式: ${e.message}`);
    }
    
    // 元数据验证（如果需要）
    if (expectedProperties.preserveMetadata) {
      const hasMetadata = await this.checkMetadata(outputPath);
      if (!hasMetadata) {
        console.warn('输出文件缺少元数据');
      }
    }
    
    return { valid: errors.length === 0, errors };
  }
}
```

#### 6.2 使用流程
```javascript
async function safeConversion(files, config) {
  const validator = new ConversionValidator();
  
  // Level 1: 输入验证
  const inputValidation = validator.validateInput(files);
  if (!inputValidation.valid) {
    showErrors('输入验证失败', inputValidation.errors);
    return;
  }
  
  // Level 2: 参数验证
  const paramValidation = validator.validateParameters(config, config.format);
  if (!paramValidation.valid) {
    showErrors('参数验证失败', paramValidation.errors);
    return;
  }
  
  // Level 3: 格式兼容性（显示警告但允许继续）
  const compatValidation = validator.validateFormatCompatibility(
    files[0].ext,
    config.format,
    files[0].properties
  );
  
  if (compatValidation.warnings.length > 0) {
    const userConfirm = await showWarnings(compatValidation.warnings);
    if (!userConfirm) return;
  }
  
  // 执行转换
  const result = await performConversion(files, config);
  
  // Level 4: 输出验证
  for (const outputFile of result.outputs) {
    const outputValidation = await validator.validateOutput(
      outputFile.path,
      { format: config.format, preserveMetadata: config.preserveMetadata }
    );
    
    if (!outputValidation.valid) {
      showErrors(`输出验证失败: ${outputFile.name}`, outputValidation.errors);
    }
  }
  
  return result;
}
```

---

## 🔍 测试报告中发现的额外问题

### 7. Eagle 库未自动刷新 🔄

**问题描述**：
- 转换完成后 Eagle 未自动刷新
- 需要用户手动刷新才能看到转换结果

**影响**：
- 用户体验不佳
- 造成转换失败的错觉

**优先级**：高

**实现建议**：
```javascript
// 在转换完成后调用 Eagle API 刷新
async function refreshEagleLibrary() {
  try {
    await eagle.library.refresh();
    console.log('[PIXLY] ✅ Eagle library refreshed');
  } catch (e) {
    console.warn('[PIXLY] ⚠️ Failed to refresh Eagle:', e);
  }
}
```

---

### 8. 转换过程中文件选择未锁定 🔒

**问题描述**：
- 转换过程中仍可选择文件
- 导致文件列表意外更新
- 影响转换进度和结果

**影响**：
- 转换过程混乱
- 可能导致转换中断
- 用户体验差

**优先级**：高

**实现建议**：
```javascript
// 转换开始时锁定选择
function lockFileSelection() {
  window.PIXLY_SELECTION_LOCKED = true;
  
  // 禁用文件选择相关 UI
  document.getElementById('fileSection')?.classList.add('locked');
  
  // 阻止 Eagle 文件选择事件
  console.log('[PIXLY] 🔒 File selection locked during conversion');
}

// 转换完成后解锁
function unlockFileSelection() {
  window.PIXLY_SELECTION_LOCKED = false;
  document.getElementById('fileSection')?.classList.remove('locked');
  console.log('[PIXLY] 🔓 File selection unlocked');
}
```

---

### 9. XMP 文件选中时缺少提示 💡

**问题描述**：
- XMP 文件被选中时无提示信息
- 用户不知道系统支持 XMP 处理
- 缺少功能引导

**影响**：
- 功能被忽视
- 用户体验不完整

**优先级**：中

**实现建议**：
```javascript
// 检测 XMP 文件并显示提示
function updateXmpNotice() {
  const hasXmpFiles = window.selectedFiles.some(f => f.ext === 'xmp');
  const notice = document.getElementById('xmpFunctionNotice');
  
  if (hasXmpFiles && notice) {
    notice.style.display = 'block';
    notice.innerHTML = `
      <span class="icon">📄</span>
      <span>检测到 XMP 元数据文件，系统将自动合并到目标图片</span>
    `;
  } else if (notice) {
    notice.style.display = 'none';
  }
}
```

---

## 📊 实现优先级

### 高优先级（立即实施）
1. ✅ 下拉菜单显示不持久
2. ✅ 智能模式简化描述
3. ✅ 多级验证系统
4. ✅ Eagle 自动刷新
5. ✅ 文件选择锁定

### 中优先级（近期实施）
1. ⏳ 完善 tooltip 和 i18n
2. ⏳ 宽屏适配
3. ⏳ 主题切换功能
4. ⏳ XMP 选中提示

---

## 📝 实施检查清单

- [ ] 恢复或重新实现主题切换功能
- [ ] 修复下拉菜单显示持久性问题
- [ ] 为所有控件添加 tooltip
- [ ] 完善 i18n 翻译覆盖
- [ ] 实现响应式宽屏布局
- [ ] 简化智能模式描述文案
- [ ] 实现四级验证系统
- [ ] 转换完成后自动刷新 Eagle
- [ ] 转换时锁定文件选择
- [ ] XMP 文件选中提示

---

## 🎯 预期效果

实施这些改进后，Pixly 将提供：
- ✨ 更直观的用户界面
- 🛡️ 更可靠的转换流程
- 🌐 更完整的国际化支持
- 🖥️ 更好的宽屏体验
- 🎯  更清晰的功能引导

---

**文档维护者**: Pixly 开发团队  
**最后更新**: 2025-01-08
