# 🚨 缺失功能审计报告

**日期**: 2025-11-18  
**审计范围**: AI Vue Refactor Plugin  
**严重性**: 🔴 高 - 违反真实性原则

## ❌ UI 声称但后端未实现的功能

### 1. AI 文件验证（Magika）
- **UI**: `enableFileValidation` checkbox
- **后端**: ❌ 无对应 CLI 参数
- **模块**: `src/magika_detector.rs` 存在但未集成到 CLI
- **修复**: 添加 `--validate-files` 参数

### 2. SSIM 质量验证
- **UI**: `enableSSIM` checkbox
- **后端**: ❌ 无对应 CLI 参数
- **模块**: 需要检查是否有 SSIM 实现
- **修复**: 添加 `--check-quality` 参数

### 3. GPU 硬件加速
- **UI**: `enableGPU` checkbox（默认开启）
- **后端**: ❌ 无对应 CLI 参数
- **模块**: 视频处理有 GPU 支持，但图像处理未暴露
- **修复**: 添加 `--gpu` / `--no-gpu` 参数

### 4. 智能预处理
- **UI**: `enablePreprocess` checkbox
- **后端**: ❌ 无对应 CLI 参数
- **模块**: `src/preprocessing.rs` 存在但未集成到 CLI
- **修复**: 添加 `--preprocess` 参数

### 5. 格式自动修正
- **UI**: `enableFormatCorrection` checkbox
- **后端**: ❌ 无对应 CLI 参数
- **模块**: 未知是否存在
- **修复**: 添加 `--format-correction` 参数或移除 UI

### 6. 优化模式
- **UI**: `optimizeMode` select（balanced/quality/size）
- **后端**: ❌ 无对应 CLI 参数
- **修复**: 添加 `--optimize-mode` 参数

## ✅ 已正确实现的功能

### 1. XMP 合并
- **UI**: 自动提示
- **后端**: ✅ `--merge-xmp` / `--xmp-path`
- **状态**: 正常

### 2. 文件名规范化
- **UI**: 自动提示
- **后端**: ✅ `--normalize-filenames`
- **状态**: 正常

### 3. AI 参数预测
- **UI**: `enableAIPrediction` checkbox
- **后端**: ✅ `--ai`
- **状态**: 正常

### 4. 文件属性保留
- **UI**: 元数据保留提示
- **后端**: ✅ `src/file_attributes.rs` 已实现
- **状态**: 正常

## 🔧 修复优先级

### 🔴 高优先级（必须修复）
1. **优化模式** - 核心功能，UI 已暴露
2. **AI 文件验证** - 安全功能，模块已存在
3. **SSIM 质量验证** - 质量保证功能

### 🟡 中优先级（建议修复）
4. **智能预处理** - 模块已存在，需要集成
5. **GPU 硬件加速** - 性能优化功能

### 🟢 低优先级（可选）
6. **格式自动修正** - 实验性功能，可以先移除 UI

## 📋 修复计划

### Phase 1: 添加缺失的 CLI 参数
```rust
// pixly_converter_cli.rs - Convert 命令

/// Optimize mode (balanced, quality, size)
#[arg(long, default_value = "balanced")]
optimize_mode: String,

/// Enable Magika AI file validation
#[arg(long, default_value = "false")]
validate_files: bool,

/// Enable SSIM quality check
#[arg(long, default_value = "false")]
check_quality: bool,

/// Enable GPU acceleration
#[arg(long, default_value = "true")]
gpu: bool,

/// Enable intelligent preprocessing
#[arg(long, default_value = "false")]
preprocess: bool,
```

### Phase 2: 集成到转换流程
- 将 `magika_detector` 集成到文件验证
- 将 `preprocessing` 集成到转换前处理
- 添加 SSIM 质量检查（转换后）
- 添加 GPU 加速选项传递

### Phase 3: 更新 useRustCLI.js
- 确保所有参数正确传递
- 添加参数验证
- 更新文档

## ⚠️ 违反的原则

根据 `PROJECT_QUALITY_MANIFESTO.md`:

1. **❌ 真实性原则** - UI 声称的功能后端未实现
2. **❌ 反对摆设代码** - UI 控件没有真实功能
3. **❌ 响亮失败原则** - 功能缺失但未报错

## 📝 建议

### 短期（立即）
1. 在 UI 中禁用未实现的功能
2. 添加"开发中"标签
3. 更新帮助文档说明实际功能

### 中期（本周）
1. 添加所有缺失的 CLI 参数
2. 集成已存在的模块
3. 完整测试所有功能

### 长期（持续）
1. 建立 UI-Backend 功能对照表
2. 自动化测试 UI 功能的真实性
3. 代码审查时检查功能完整性
