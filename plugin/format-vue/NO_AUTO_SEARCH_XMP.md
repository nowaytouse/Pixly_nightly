# 🚫 禁止自动查找 XMP - 仅选择模式

**日期**: 2025-11-XX  
**原则**: 用户明确选择，系统不做任何自动查找  
**状态**: ✅ 已实现

---

## 核心原则

### ❌ 禁止的行为
1. **禁止自动查找 XMP 文件**
2. **禁止扫描目录**
3. **禁止猜测文件关系**
4. **禁止任何形式的"智能"匹配**

### ✅ 允许的行为
1. **用户在 Eagle 中同时选择图像和 XMP**
2. **插件基于文件名匹配配对**（用户已选择的文件）
3. **CLI 用户手动指定 `--xmp-path`**
4. **没有提供 XMP 路径 = 跳过 XMP 合并**

---

## 实现细节

### 1. Eagle 插件端 (`useEagleAPI.js`)

**工作流程**:
```javascript
// 1. 用户在 Eagle 中选择文件（可能包含图像和 XMP）
const items = await window.eagle.item.getSelected()

// 2. 分离媒体文件和 XMP 文件
const mediaFiles = []
const xmpFiles = []

items.forEach(item => {
  if (item.ext.toLowerCase() === 'xmp') {
    xmpFiles.push(item)  // XMP 文件
  } else if (isImageFile(filename) || isVideoFile(filename)) {
    mediaFiles.push(item)  // 媒体文件
  }
})

// 3. 为每个媒体文件匹配对应的 XMP（基于文件名）
const xmpMap = new Map()
xmpFiles.forEach(xmp => {
  xmpMap.set(xmp.name, xmp.filePath)  // name 不含扩展名
})

// 4. 构建文件列表（仅包含用户选择的配对）
selectedItems.value = mediaFiles.map(item => ({
  id: item.id,
  name: item.name,
  path: item.filePath,
  xmpPath: xmpMap.get(item.name) || null,  // 🔥 仅当用户选择了对应的 XMP
  // ... 其他字段
}))
```

**关键点**:
- ✅ 仅处理用户选择的文件
- ✅ 基于文件名匹配（`photo` 匹配 `photo.xmp`）
- ❌ 不检查文件系统
- ❌ 不扫描目录
- ❌ 不猜测 XMP 位置

### 2. Rust CLI 调用 (`useRustCLI.js`)

```javascript
// 构建 CLI 参数
const args = [
  'convert',
  inputPath,
  '--format', options.format,
  '--quality', options.quality.toString()
]

// 🔥 XMP 合并（仅当插件提供了 XMP 路径）
if (options.mergeXmp !== false) {
  args.push('--merge-xmp')
  
  if (file.xmpPath) {
    args.push('--xmp-path', file.xmpPath)
    logger.info('Passing XMP path to Rust CLI', { xmpPath: file.xmpPath })
  }
}
```

**关键点**:
- ✅ 仅传递插件提供的 XMP 路径
- ❌ 不做任何额外查找

### 3. Rust CLI 处理 (`pixly_converter_cli.rs`)

```rust
fn merge_xmp_sidecar(_input_path: &Path, output_path: &Path, provided_xmp_path: Option<&Path>) -> Result<()> {
    // 🔥 仅使用提供的 XMP 路径，不做任何自动查找
    let xmp_path = if let Some(provided) = provided_xmp_path {
        if provided.exists() {
            println!("   📎 Using provided XMP path: {:?}", provided);
            provided.to_path_buf()
        } else {
            println!("   ⚠️  Provided XMP path does not exist: {:?}", provided);
            return Ok(());
        }
    } else {
        // 🔥 没有提供 XMP 路径，直接返回（不查找）
        println!("   ℹ️  No XMP path provided, skipping XMP merge");
        return Ok(());
    };
    
    // ... 执行 XMP 合并 ...
}
```

**关键点**:
- ✅ 仅使用 `provided_xmp_path` 参数
- ❌ 不检查 `input_path.with_extension("xmp")`
- ❌ 不扫描 Eagle images/ 目录
- ❌ 不做任何自动查找

**已删除的函数**:
```rust
// 🔥 已删除 find_eagle_xmp_resource 函数
// 原因：不再自动查找 XMP，仅使用用户/插件提供的路径
```

---

## 使用场景

### 场景 1: Eagle 插件 - 有 XMP

**用户操作**:
1. 在 Eagle 中选择 `photo.png`
2. 同时选择 `photo.xmp`（Ctrl/Cmd + 点击）
3. 点击"开始转换"

**系统行为**:
```
1. Eagle API 返回 2 个文件
2. 插件分离：mediaFiles=[photo.png], xmpFiles=[photo.xmp]
3. 插件匹配：photo.png → photo.xmp ✅
4. 传递给 Rust: photo.png + --xmp-path photo.xmp
5. Rust 使用提供的路径合并 XMP
6. 删除 photo.xmp
```

### 场景 2: Eagle 插件 - 无 XMP

**用户操作**:
1. 在 Eagle 中选择 `image.png`（不选择 XMP）
2. 点击"开始转换"

**系统行为**:
```
1. Eagle API 返回 1 个文件
2. 插件分离：mediaFiles=[image.png], xmpFiles=[]
3. 插件匹配：image.png → null ❌
4. 传递给 Rust: image.png（无 --xmp-path）
5. Rust 跳过 XMP 合并
```

### 场景 3: CLI 环境 - 手动指定

**用户操作**:
```bash
pixly-converter convert photo.png --format jxl --quality 90 --merge-xmp --xmp-path photo.xmp
```

**系统行为**:
```
1. Rust CLI 接收 --xmp-path 参数
2. 使用提供的路径合并 XMP
3. 删除 photo.xmp
```

### 场景 4: CLI 环境 - 不指定

**用户操作**:
```bash
pixly-converter convert photo.png --format jxl --quality 90
```

**系统行为**:
```
1. Rust CLI 未接收 --xmp-path 参数
2. 跳过 XMP 合并
3. 仅转换图像
```

---

## 架构优势

### ✅ 用户完全控制
- 用户决定是否合并 XMP
- 用户通过选择文件明确意图
- 系统不做任何假设

### ✅ 性能最优
- 无文件系统扫描
- 无目录遍历
- 无猜测匹配

### ✅ 行为可预测
- 选择了 XMP = 合并
- 未选择 XMP = 不合并
- 简单明确

### ✅ 符合质量宣言
- 真实性原则：不假装"智能"
- 响亮失败原则：没有 XMP 就明确跳过
- 用户透明原则：行为完全可预测

---

## 日志示例

### Eagle 插件日志（有 XMP）
```
[PIXLY INFO] [eagle.api.call] Separated files
  total: 2
  media: 1
  xmp: 1
  ignored: 0

[PIXLY INFO] [eagle.api.call] XMP files available
  count: 1
  names: ["photo"]

[PIXLY INFO] [eagle.api.call] Matched XMP for media file
  media: "photo"
  xmp: "/Users/.../photo.xmp"

[PIXLY INFO] [rust.cli.exec] Passing XMP path to Rust CLI
  xmpPath: "/Users/.../photo.xmp"
```

### Rust CLI 日志（有 XMP）
```
📎 Using provided XMP path: "/Users/.../photo.xmp"
🔄 Merging XMP metadata to output file...
✅ XMP merged into output file: "photo.jxl"
✅ XMP data verified in output file
🗑️  Original XMP sidecar deleted
```

### Rust CLI 日志（无 XMP）
```
ℹ️  No XMP path provided, skipping XMP merge
```

---

## 修改文件清单

1. ✅ `plugin/format-vue/src/composables/useEagleAPI.js`
   - 分离媒体文件和 XMP 文件
   - 基于文件名匹配配对
   - 不做文件系统查找

2. ✅ `plugin/format-vue/src/composables/useRustCLI.js`
   - 传递 `file.xmpPath` 给 Rust CLI
   - 仅当存在时传递

3. ✅ `pixly_converter_cli.rs`
   - 删除 `find_eagle_xmp_resource` 函数
   - 简化 `merge_xmp_sidecar` 逻辑
   - 仅使用 `provided_xmp_path` 参数

4. ✅ Rust CLI 重新编译
5. ✅ Vue 应用重新构建

---

## 测试验证

### 测试 1: 选择图像 + XMP
```
选择: photo.png + photo.xmp
预期: ✅ XMP 合并成功，原 XMP 删除
```

### 测试 2: 仅选择图像
```
选择: image.png
预期: ✅ 跳过 XMP 合并，转换成功
```

### 测试 3: 选择图像 + 不匹配的 XMP
```
选择: photo.png + other.xmp
预期: ✅ 不匹配，跳过 XMP 合并
```

### 测试 4: CLI 手动指定
```bash
pixly-converter convert photo.png --format jxl --xmp-path photo.xmp
预期: ✅ XMP 合并成功
```

---

**修复完成时间**: 2025-11-XX  
**编译状态**: ✅ 成功  
**构建状态**: ✅ 成功  
**原则**: 🚫 禁止自动查找，✅ 仅用户选择
