# 🔥 重大功能缺失报告

**日期**: 2025-11-06 16:30  
**状态**: ❌ **紧急修复中**

---

## 缺失功能清单

### 1. ✅ Rust CLI路径解析错误 (已修复)
**错误**: `window.PIXLY_PATH_RESOLVER.getRustBinary is not a function`

**原因**: 
- 调用了不存在的`getRustBinary()`方法
- 实际方法名是`getRustCLIPath()`

**修复**: 
- 文件: `plugin/js/plugin-modules/28-rust-cli-executor.js:34`
- 改为: `window.PIXLY_PATH_RESOLVER.getRustCLIPath()`

---

### 2. 🔥 XMP合并功能完全缺失 (紧急)

**问题**: 
- 当前`04-conversion-core.js`完全没有XMP合并功能
- 旧版`04-conversion.js.backup`中有完整实现

**影响**:
- RAW图像转换时，侧车XMP文件中的编辑信息会丢失
- 用户的所有Lightroom/Capture One调整会消失

**旧版实现流程**:
```javascript
// 步骤1: 文件名规范化（文件系统层面）
if (autoNormalizeNames) {
    const result = await normalizeFileNamesInternal(validFiles);
}

// 步骤2: XMP合并（文件系统层面）
if (autoMergeXmp) {
    const xmpResult = await mergeXmpFiles(validFiles);
}
```

**关键函数**:
- `mergeXmpFiles()`: 查找并合并.xmp侧车文件
- `normalizeFileNamesInternal()`: 文件名规范化

---

### 3. 🔥 文件名规范化功能缺失 (已被spawnSync替代)

**状态**: ⚠️ 不需要修复（已被更好的方案替代）

**原因**: 
- 旧版通过重命名文件规避shell特殊字符
- 新版使用`spawnSync`数组参数，直接支持特殊字符

---

## 修复优先级

### P0 (紧急 - 阻塞功能)
1. ✅ Rust CLI路径解析
2. 🔥 XMP合并功能

### P1 (重要 - 影响体验)
3. GIF动画转换

---

## XMP合并功能详细分析

### 什么是XMP侧车文件？

RAW图像的非破坏性编辑信息存储在`.xmp`文件中：
```
photo.CR2           # 原始RAW文件
photo.xmp           # 侧车文件（包含所有编辑）
```

### 为什么需要合并？

没有XMP合并时：
```
CR2 → JPEG 转换
   ↓
只使用CR2原始数据
忽略.xmp中的所有调整
   ↓
输出的JPEG是"未处理"状态
```

有XMP合并时：
```
CR2 + XMP → JPEG 转换
   ↓
应用XMP中的所有调整：
- 曝光、对比度、饱和度
- 白平衡、色温
- 局部调整、蒙版
- 裁剪、旋转
   ↓
输出的JPEG包含所有编辑
```

### 实现方案

#### 方案A: JS端实现（旧版方式）
```javascript
// 1. 查找XMP文件
const xmpPath = imagePath.replace(/\.\w+$/, '.xmp');

// 2. 使用exiftool合并
execSync(`exiftool -tagsfromfile "${xmpPath}" -all:all "${imagePath}"`);

// 3. 转换已合并的图像
await rustCLI.convert(imagePath, outputPath, options);
```

**优点**: 实现简单  
**缺点**: 需要临时修改原文件

#### 方案B: Rust端实现（推荐）
```rust
// 在Rust转换前处理XMP
pub fn apply_xmp_sidecar(input: &Path, xmp_path: &Path) -> Result<()> {
    // 1. 读取XMP
    let xmp_data = std::fs::read_to_string(xmp_path)?;
    
    // 2. 解析XMP调整
    let adjustments = parse_xmp_adjustments(&xmp_data)?;
    
    // 3. 应用到图像
    apply_adjustments_to_image(input, &adjustments)?;
    
    Ok(())
}
```

**优点**: 
- 不修改原文件
- 性能更好
- 更可靠

**缺点**: 需要实现XMP解析

---

## 当前架构中XMP合并的缺失路径

### 旧架构（有XMP合并）
```
JS Plugin
  ↓
04-conversion.js
  ↓ mergeXmpFiles()
exiftool合并XMP
  ↓
Rust CLI转换
  ↓
输出 ✅ 包含XMP调整
```

### 新架构（无XMP合并）
```
JS Plugin
  ↓
04-conversion-core.js
  ↓ ❌ 没有XMP处理
Rust CLI转换
  ↓
输出 ❌ 不包含XMP调整
```

---

## 修复计划

### 短期方案（JS端）
1. 从`04-conversion.js.backup`恢复`mergeXmpFiles()`函数
2. 在`04-conversion-core.js`中调用
3. 添加UI开关（默认启用）

### 长期方案（Rust端）
1. 在`pixly-rust`中实现XMP解析
2. 添加`--xmp-sidecar` CLI参数
3. 在转换前自动应用XMP调整

---

## 测试验证

### 测试场景
1. 准备一个RAW文件 + XMP侧车文件
2. 在Lightroom中做明显调整（如增加曝光+2EV）
3. 转换为JPEG
4. 对比：
   - ❌ 无XMP合并：JPEG很暗（原始曝光）
   - ✅ 有XMP合并：JPEG正常（应用了+2EV）

---

**下一步**: 
1. ✅ 立即测试Rust CLI路径修复
2. 🔥 恢复XMP合并功能
3. 📝 添加XMP功能测试用例

---

**报告时间**: 2025-11-06 16:30  
**修复者**: Claude  
**状态**: Rust CLI路径已修复，XMP功能待恢复
