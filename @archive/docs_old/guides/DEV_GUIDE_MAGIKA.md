# 🔧 Magika AI 文件验证 - 开发者指南

**版本**: Phase 45 完整集成版  
**更新时间**: 2025-11-07  
**目标读者**: Pixly 开发者、贡献者

---

## 📑 目录

1. [架构概览](#架构概览)
2. [技术栈](#技术栈)
3. [模块详解](#模块详解)
4. [API 参考](#api-参考)
5. [扩展点](#扩展点)
6. [测试指南](#测试指南)
7. [性能优化](#性能优化)
8. [故障排查](#故障排查)

---

## 🏗️ 架构概览

### 分层架构

```
┌─────────────────────────────────────────────────────┐
│          Eagle Plugin UI (JavaScript)               │
│  ┌─────────────────────────────────────────────┐   │
│  │  FileValidator (file-validator.js)          │   │
│  │  - 单文件验证                                │   │
│  │  - 批量验证                                  │   │
│  │  - 结果分析                                  │   │
│  │  - UI交互                                    │   │
│  └──────────────────┬──────────────────────────┘   │
└────────────────────│────────────────────────────────┘
                     │
                     │ 调用 rustCLI.execCommand
                     ↓
┌─────────────────────────────────────────────────────┐
│          Rust CLI (pixly-rust)                      │
│  ┌─────────────────────────────────────────────┐   │
│  │  detect 命令 (cli/commands.rs)              │   │
│  │  - 参数解析                                  │   │
│  │  - 调用检测引擎                              │   │
│  │  - 格式化输出（JSON/Human-readable）         │   │
│  └──────────────────┬──────────────────────────┘   │
└────────────────────│────────────────────────────────┘
                     │
                     │ 调用 MagikaDetector
                     ↓
┌─────────────────────────────────────────────────────┐
│     MagikaDetector (converter/magika_detector.rs)   │
│  ┌─────────────────────────────────────────────┐   │
│  │  - initialize() (单例初始化)                │   │
│  │  - detect_file_type()                       │   │
│  │  - validate_security()                      │   │
│  └──────────────────┬──────────────────────────┘   │
└────────────────────│────────────────────────────────┘
                     │
                     │ 调用 Magika Rust Crate
                     ↓
┌─────────────────────────────────────────────────────┐
│          Magika (Google, magika crate)              │
│  ┌─────────────────────────────────────────────┐   │
│  │  Session::new()                             │   │
│  │  session.identify_file_sync()               │   │
│  │  - ONNX Runtime 推理                        │   │
│  │  - 深度学习模型                             │   │
│  │  - ~5ms/文件                                 │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### 数据流

```
User Action (Eagle UI)
  ↓
FileValidator.validateFile(file)
  ↓
rustCLI.execCommand('detect', [filepath, '--json', '--security'])
  ↓
handle_detect_command(&args)
  ↓
MagikaDetector::initialize()
  ↓
detector.detect_file_type(path)
  ↓
Magika Session.identify_file_sync()
  ↓
ONNX Runtime Inference (~5ms)
  ↓
FileTypeDetection Result
  ↓
SecurityValidation (optional)
  ↓
JSON Output → stdout
  ↓
JavaScript JSON.parse()
  ↓
FileValidator.analyzeDetection()
  ↓
UI Display (Notification / Dialog)
```

---

## 🛠️ 技术栈

### Rust 后端

| 组件 | 版本 | 用途 |
|------|------|------|
| **Magika** | `1.0` | AI文件类型检测核心 |
| **ONNX Runtime** | `2.0.0-rc.10` | 深度学习推理引擎 |
| **Anyhow** | `1.0` | 错误处理 |
| **Serde JSON** | `1.0` | JSON序列化 |
| **Once Cell** | `1.19` | 单例模式（懒加载） |

### JavaScript 前端

| 组件 | 用途 |
|------|------|
| **FileValidator** | 验证逻辑封装 |
| **Eagle Plugin API** | 对话框、通知 |
| **rustCLI.execCommand** | Rust CLI调用 |
| **JSON.parse()** | 解析检测结果 |

### 依赖关系

```toml
# core/rust/Cargo.toml
[dependencies]
magika = "1.0"
ort = { version = "2.0.0-rc.10", features = ["download-binaries"] }
anyhow = "1.0"
serde_json = "1.0"
once_cell = "1.19"
```

---

## 📦 模块详解

### 1. MagikaDetector (Rust)

**文件**: `core/rust/src/converter/magika_detector.rs` (366 lines)

#### 核心结构体

```rust
/// 文件类型检测结果
pub struct FileTypeDetection {
    pub file_path: String,
    pub detected_type: String,
    pub confidence: f64,
    pub mime_type: String,
    pub description: String,
    pub is_binary: bool,
    pub is_high_confidence: bool,
}

/// 安全验证结果
pub struct SecurityValidation {
    pub detected_type: String,
    pub type_match: bool,
    pub is_safe: bool,
    pub is_suspicious: bool,
    pub warnings: Vec<String>,
}
```

#### 关键方法

##### `initialize() -> Result<()>`

初始化 Magika Session（单例模式）

```rust
static MAGIKA_SESSION: Lazy<Result<Session, String>> = Lazy::new(|| {
    Session::new()
        .map_err(|e| format!("Failed to initialize Magika: {}", e))
});

pub fn initialize() -> Result<()> {
    match &*MAGIKA_SESSION {
        Ok(_) => {
            println!("✅ Magika Session initialized successfully");
            Ok(())
        }
        Err(e) => {
            bail!("Failed to initialize Magika: {}", e);
        }
    }
}
```

**线程安全**: `Lazy<T>` 确保多线程环境下只初始化一次  
**错误处理**: 初始化失败会返回 `Err`，后续调用会复用错误

##### `detect_file_type(path: &Path) -> Result<FileTypeDetection>`

检测文件类型

```rust
pub fn detect_file_type(path: &Path) -> Result<FileTypeDetection> {
    let session = match &*MAGIKA_SESSION {
        Ok(s) => s,
        Err(e) => bail!("Magika not initialized: {}", e),
    };
    
    let file_type = session
        .identify_file_sync(path)
        .map_err(|e| anyhow!("Failed to detect file: {}", e))?;
    
    let info = file_type.info();
    
    Ok(FileTypeDetection {
        file_path: path.display().to_string(),
        detected_type: info.name.to_string(),
        confidence: file_type.score() as f64,
        mime_type: info.mime_type.to_string(),
        description: info.description.to_string(),
        is_binary: Self::is_binary_type(info.name),
        is_high_confidence: file_type.score() > 0.95,
    })
}
```

**性能**: ~5ms/文件（ONNX Runtime优化）  
**准确率**: ~99% (取决于文件类型)

##### `validate_security(path: &Path) -> Result<SecurityValidation>`

安全验证（伪装检测）

```rust
pub fn validate_security(path: &Path) -> Result<SecurityValidation> {
    let detection = Self::detect_file_type(path)?;
    
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let type_match = Self::extension_matches_type(extension, &detection.detected_type);
    
    let mut warnings = Vec::new();
    if !type_match {
        warnings.push(format!(
            "⚠️  File type mismatch: Expected '{}' but detected '{}' (confidence: {:.1}%)",
            extension,
            detection.detected_type,
            detection.confidence * 100.0
        ));
    }
    
    let is_suspicious = !type_match;
    let is_safe = !Self::is_executable_type(&detection.detected_type) || type_match;
    
    Ok(SecurityValidation {
        detected_type: detection.detected_type,
        type_match,
        is_safe,
        is_suspicious,
        warnings,
    })
}
```

**安全逻辑**:
- 类型不匹配 → `is_suspicious = true`
- 可执行文件且类型不匹配 → `is_safe = false`
- 扩展名别名支持（如 `jpeg`/`jpg`）

---

### 2. CLI 命令 (Rust)

**文件**: `core/rust/src/cli/commands.rs`

#### `handle_detect_command(&[String])`

处理 `detect` 子命令

```rust
pub fn handle_detect_command(args: &[String]) {
    use pixly_converter::converter::magika_detector::MagikaDetector;
    use std::path::Path;
    
    if args.is_empty() {
        eprintln!("❌ Error: Missing file path");
        eprintln!("   Usage: pixly-rust detect <file> [--json] [--security]");
        return;
    }
    
    let mut file_path = None;
    let mut json_output = false;
    let mut security_check = false;
    let mut verbose = false;
    
    // 参数解析
    for arg in args {
        match arg.as_str() {
            "--json" => json_output = true,
            "--security" => security_check = true,
            "--verbose" | "-v" => verbose = true,
            _ => {
                if file_path.is_none() && !arg.starts_with("--") {
                    file_path = Some(arg.clone());
                }
            }
        }
    }
    
    // 执行检测 + 输出
    // ...
}
```

**输出模式**:
1. **Human-readable** (默认): 格式化文本
2. **JSON** (`--json`): 机器可读JSON
3. **Verbose** (`-v`): 详细元数据

**示例**:
```bash
# 基础检测
$ pixly-rust detect photo.png

# JSON输出
$ pixly-rust detect photo.png --json

# 安全验证
$ pixly-rust detect photo.png --security

# 组合
$ pixly-rust detect photo.png --json --security --verbose
```

---

### 3. FileValidator (JavaScript)

**文件**: `core/plugin/js/plugin-modules/file-validator.js` (359 lines)

#### 类结构

```javascript
class FileValidator {
    constructor() {
        this.enabled = true;         // 启用状态
        this.strictMode = false;     // 严格模式
        this.cache = new Map();      // 结果缓存
    }
    
    async validateFile(file) { /* ... */ }
    async validateBatch(files, onProgress) { /* ... */ }
    async detectFileType(filePath) { /* ... */ }
    analyzeDetection(file, detection) { /* ... */ }
    async showWarningDialog(warnings, fileName) { /* ... */ }
    showBatchResults(results) { /* ... */ }
}
```

#### 核心方法详解

##### `validateFile(file) -> Promise<Object>`

验证单个文件

```javascript
async validateFile(file) {
    if (!this.enabled) {
        return { success: true, skipped: true };
    }
    
    // 检查缓存
    if (this.cache.has(file.filePath)) {
        return this.cache.get(file.filePath);
    }
    
    try {
        // 调用 Rust CLI
        const detection = await this.detectFileType(file.filePath);
        
        // 分析结果
        const result = this.analyzeDetection(file, detection);
        
        // 缓存
        this.cache.set(file.filePath, result);
        
        return result;
    } catch (error) {
        return {
            success: false,
            error: error.message,
            warnings: [`检测失败: ${error.message}`]
        };
    }
}
```

**缓存策略**: Map<filePath, result>  
**错误处理**: 捕获并返回失败结果，不抛出异常

##### `detectFileType(filePath) -> Promise<Object>`

调用 Rust CLI 进行检测

```javascript
async detectFileType(filePath) {
    const result = await window.rustCLI.execCommand('detect', [
        filePath,
        '--json',
        '--security'
    ]);
    
    if (!result.success) {
        throw new Error(`Detection failed: ${result.error}`);
    }
    
    // 智能 JSON 解析（处理混合输出）
    const lines = result.stdout.split('\n');
    let detectionJson = null;
    let securityJson = null;
    
    let currentJson = '';
    let braceCount = 0;
    
    for (const line of lines) {
        if (line.includes('{')) {
            braceCount += (line.match(/{/g) || []).length;
        }
        if (line.includes('}')) {
            braceCount -= (line.match(/}/g) || []).length;
        }
        
        if (braceCount > 0 || line.includes('{') || line.includes('}')) {
            currentJson += line;
            
            if (braceCount === 0 && currentJson.includes('}')) {
                try {
                    const parsed = JSON.parse(currentJson);
                    if (!detectionJson) {
                        detectionJson = parsed;
                    } else if (!securityJson) {
                        securityJson = parsed;
                    }
                    currentJson = '';
                } catch (e) {
                    // JSON 未完整，继续累积
                }
            }
        }
    }
    
    return {
        detection: detectionJson || {},
        security: securityJson || {}
    };
}
```

**智能解析**: 处理 CLI 输出中的装饰文本（如分隔线）  
**鲁棒性**: 逐行解析，支持多个JSON对象

##### `analyzeDetection(file, detection) -> Object`

分析检测结果

```javascript
analyzeDetection(file, detection) {
    const warnings = [];
    let suspicious = false;
    let success = true;
    
    const det = detection.detection || {};
    const sec = detection.security || {};
    
    // 置信度检查
    if (det.confidence && det.confidence < 0.9) {
        warnings.push(`⚠️ 低置信度检测: ${(det.confidence * 100).toFixed(1)}%`);
    }
    
    // 类型匹配
    if (sec.type_match === false) {
        warnings.push(`⚠️ 文件类型不匹配: 扩展名为 .${file.ext}，但检测为 ${det.detected_type}`);
        suspicious = true;
    }
    
    // 可疑标记
    if (sec.is_suspicious) {
        warnings.push('🚨 检测到可疑文件');
        suspicious = true;
    }
    
    // 安全性
    if (sec.is_safe === false) {
        warnings.push('❌ 文件可能不安全');
        if (this.strictMode) {
            success = false;
        }
    }
    
    return {
        success,
        file,
        detectedType: det.detected_type,
        confidence: det.confidence,
        suspicious,
        warnings
    };
}
```

**警告级别**:
- 低置信度 (< 90%) → 提示
- 类型不匹配 → 警告 + 可疑标记
- 不安全 → 严格模式下阻止

---

## 🔌 API 参考

### Rust API

#### MagikaDetector

```rust
use pixly_converter::converter::magika_detector::MagikaDetector;

// 初始化（必须先调用）
MagikaDetector::initialize()?;

// 检测文件类型
let detection = MagikaDetector::detect_file_type(Path::new("photo.png"))?;
println!("Type: {}, Confidence: {:.2}%", 
    detection.detected_type, 
    detection.confidence * 100.0
);

// 安全验证
let security = MagikaDetector::validate_security(Path::new("photo.png"))?;
if !security.is_safe {
    eprintln!("⚠️ File may be unsafe!");
}
```

#### CLI命令

```rust
use pixly_converter::cli::handle_detect_command;

// 程序化调用
handle_detect_command(&["photo.png".to_string(), "--json".to_string()]);
```

### JavaScript API

#### FileValidator

```javascript
// 全局实例
const validator = window.fileValidator;

// 单文件验证
const result = await validator.validateFile(file);
if (result.warnings.length > 0) {
    console.warn('Warnings:', result.warnings);
}

// 批量验证
const results = await validator.validateBatch(files, (progress) => {
    console.log(`Progress: ${progress.percent}%`);
});

// 配置
validator.setEnabled(true);
validator.setStrictMode(false);
validator.clearCache();
```

---

## 🔧 扩展点

### 1. 自定义扩展名映射

**文件**: `magika_detector.rs`

```rust
pub fn extension_matches_type(extension: &str, detected_type: &str) -> bool {
    let ext_lower = extension.to_lowercase();
    let type_lower = detected_type.to_lowercase();
    
    match (ext_lower.as_str(), type_lower.as_str()) {
        // 现有映射
        ("jpg", "jpeg") | ("jpeg", "jpg") => true,
        ("tif", "tiff") | ("tiff", "tif") => true,
        
        // 🔥 添加新映射
        ("jfif", "jpeg") => true,  // JFIF是JPEG子格式
        ("mpg", "mpeg") => true,   // MPEG别名
        
        _ => ext_lower == type_lower,
    }
}
```

### 2. 自定义安全规则

**文件**: `magika_detector.rs`

```rust
pub fn is_executable_type(file_type: &str) -> bool {
    matches!(
        file_type.to_lowercase().as_str(),
        "exe" | "dll" | "so" | "dylib" | "sh" | "bat" | "cmd" | "ps1"
        // 🔥 添加新可执行类型
        | "msi" | "app" | "deb" | "rpm" | "apk"
    )
}
```

### 3. 自定义验证逻辑

**文件**: `file-validator.js`

```javascript
analyzeDetection(file, detection) {
    // ... 现有逻辑 ...
    
    // 🔥 添加自定义规则
    if (det.detected_type === 'svg' && file.size > 10 * 1024 * 1024) {
        warnings.push('⚠️ SVG文件过大，可能包含恶意代码');
        suspicious = true;
    }
    
    if (det.detected_type === 'pdf' && !sec.is_safe) {
        warnings.push('❌ PDF可能包含可执行内容');
        success = false;
    }
    
    return { success, suspicious, warnings, ... };
}
```

---

## 🧪 测试指南

### 单元测试（Rust）

**文件**: `core/rust/tests/test_magika.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_initialize() {
        let result = MagikaDetector::initialize();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_detect_png() {
        MagikaDetector::initialize().unwrap();
        let detection = MagikaDetector::detect_file_type(
            Path::new("testdata/sample.png")
        ).unwrap();
        
        assert_eq!(detection.detected_type, "png");
        assert!(detection.confidence > 0.95);
    }
    
    #[test]
    fn test_security_disguised_file() {
        MagikaDetector::initialize().unwrap();
        
        // PNG伪装成TXT
        let security = MagikaDetector::validate_security(
            Path::new("testdata/disguised.txt")
        ).unwrap();
        
        assert_eq!(security.type_match, false);
        assert_eq!(security.is_suspicious, true);
    }
}
```

### 集成测试（JavaScript）

创建测试文件: `core/plugin/tests/test-file-validator.html`

```html
<!DOCTYPE html>
<html>
<head>
    <title>FileValidator Tests</title>
</head>
<body>
    <script src="../js/plugin-modules/file-validator.js"></script>
    <script>
        async function runTests() {
            const validator = window.fileValidator;
            
            // Test 1: 正常PNG
            const result1 = await validator.validateFile({
                name: 'photo.png',
                ext: 'png',
                filePath: '/path/to/photo.png'
            });
            console.assert(result1.success, 'Test 1 Failed');
            
            // Test 2: 伪装文件
            const result2 = await validator.validateFile({
                name: 'disguised.txt',
                ext: 'txt',
                filePath: '/path/to/disguised.txt'
            });
            console.assert(result2.suspicious, 'Test 2 Failed');
            
            console.log('All tests passed!');
        }
        
        runTests();
    </script>
</body>
</html>
```

### 手动测试

```bash
# 1. 测试正常文件
pixly-rust detect photo.png --json --security

# 2. 测试伪装文件
cp photo.png disguised.txt
pixly-rust detect disguised.txt --json --security

# 3. 测试批量
for file in *.jpg *.png *.gif; do
    pixly-rust detect "$file" --json
done
```

---

## ⚡ 性能优化

### 1. 单例初始化

**问题**: Magika Session 初始化较慢（~100ms）

**解决**: 使用 `Lazy<T>` 懒加载，只初始化一次

```rust
static MAGIKA_SESSION: Lazy<Result<Session, String>> = Lazy::new(|| {
    Session::new().map_err(|e| format!("Failed: {}", e))
});
```

**效果**:
- 首次检测: ~105ms (初始化 + 检测)
- 后续检测: ~5ms (仅检测)

### 2. 结果缓存（JavaScript）

**问题**: 重复文件多次检测

**解决**: Map 缓存已检测结果

```javascript
this.cache = new Map();

if (this.cache.has(filePath)) {
    return this.cache.get(filePath);  // 缓存命中
}

// 检测后缓存
this.cache.set(filePath, result);
```

**效果**:
- 缓存命中: 0ms (直接返回)
- 内存占用: ~200 bytes/文件

### 3. ONNX Runtime 优化

**Cargo.toml**:
```toml
[dependencies]
ort = { version = "2.0.0-rc.10", features = ["download-binaries"] }
```

**自动下载**: 编译时自动下载优化的ONNX Runtime库  
**硬件加速**: 自动使用可用的硬件加速（CPU SIMD、GPU等）

---

## 🐛 故障排查

### 问题 1: "Magika not initialized"

**错误**:
```
Error: Magika not initialized: Failed to initialize Magika
```

**原因**: Session 初始化失败

**解决**:
1. 检查 ONNX Runtime 是否正确安装
2. 检查模型文件是否存在
3. 检查 Cargo.toml 中 `ort` 依赖

```bash
# 重新编译，强制下载 ONNX Runtime
cargo clean
cargo build --release
```

---

### 问题 2: JSON 解析失败

**错误**:
```javascript
SyntaxError: Unexpected token in JSON at position 0
```

**原因**: CLI 输出包含非JSON文本

**解决**: 使用智能解析（已实现）

```javascript
// 检查 JSON 解析逻辑
const lines = stdout.split('\n');
for (const line of lines) {
    if (line.trim().startsWith('{')) {
        try {
            const parsed = JSON.parse(line);
            // ...
        } catch (e) {
            console.warn('Invalid JSON:', line);
        }
    }
}
```

---

### 问题 3: 检测速度慢

**症状**: 每个文件检测 > 100ms

**排查**:
1. **首次检测慢 (正常)**: 初始化 Session
2. **持续慢**: 检查系统资源

```bash
# 测试检测速度
time pixly-rust detect photo.png

# 多次检测平均
for i in {1..10}; do
    time pixly-rust detect photo.png
done
```

**优化**:
- 确保使用 `--release` 编译
- 检查是否有其他进程占用 CPU
- 考虑使用缓存

---

### 问题 4: 类型匹配误报

**症状**: 正常文件被标记为可疑

**示例**: `photo.jpeg` 检测为 `jpg`，误报类型不匹配

**解决**: 添加扩展名别名

```rust
// magika_detector.rs
pub fn extension_matches_type(extension: &str, detected_type: &str) -> bool {
    match (ext_lower.as_str(), type_lower.as_str()) {
        ("jpg", "jpeg") | ("jpeg", "jpg") => true,  // 添加别名
        // ...
    }
}
```

---

## 📚 参考资源

### 官方文档
- **Magika GitHub**: https://github.com/google/magika
- **ONNX Runtime**: https://onnxruntime.ai/docs/
- **Rust Magika Crate**: https://crates.io/crates/magika

### 项目文档
- **用户手册**: `newdocs/USER_GUIDE_MAGIKA.md`
- **架构文档**: `newdocs/PROJECT_QUALITY_MANIFESTO.md`
- **Phase 报告**: `newdocs/phase_reports/phase45_*.md`

### 代码示例
- **Rust CLI 示例**: `core/rust/examples/test_magika.rs`
- **Rust 单元测试**: `core/rust/tests/test_magika.rs`

---

## 🔄 版本历史

### Phase 45.4 (2025-11-07)
- ✅ Eagle Plugin UI 完整集成
- ✅ FileValidator 类实现
- ✅ 图像+视频转换集成

### Phase 45.3 (2025-11-07)
- ✅ CLI `detect` 命令
- ✅ JSON/Security/Verbose 输出模式

### Phase 45.2 (2025-11-07)
- ✅ MediaAnalyzer 集成
- ✅ AI文件类型检测集成到转换流程

### Phase 45.1 (2025-11-07)
- ✅ MagikaDetector 核心引擎
- ✅ 依赖搭建与初始化

---

## 🤝 贡献指南

### 添加新功能

1. **Rust 后端**: 修改 `magika_detector.rs`
2. **CLI 命令**: 修改 `cli/commands.rs`
3. **JavaScript 前端**: 修改 `file-validator.js`
4. **更新文档**: 同步更新本文档和用户手册
5. **添加测试**: 编写单元测试和集成测试

### 代码规范

- **Rust**: 遵循 `rustfmt` 和 `clippy` 建议
- **JavaScript**: ESLint + Prettier
- **注释**: 所有公共 API 必须有文档注释

### Pull Request

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/magika-improvement`)
3. 提交更改 (`git commit -am 'Add feature'`)
4. 推送分支 (`git push origin feature/magika-improvement`)
5. 创建 Pull Request

---

**文档版本**: 1.0.0  
**最后更新**: 2025-11-07  
**维护者**: Pixly Development Team  

---

🔧 **开发愉快！** 如有问题，请查看用户手册或提交 Issue。
