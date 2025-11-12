# 🐛 Eagle Metadata Name字段错误修复

**日期**: 2025-11-06  
**Bug**: Eagle元数据的name字段包含扩展名  
**状态**: ✅ **已修复**

---

## 🔍 Bug表现

### 用户报告
> "avif转换时没有解析eagle资源库 元数据json没有被更新.. 转换为webp不会动.. avif也不会动"

### 日志证据

```javascript
// Rust报告更新成功
28-rust-cli-executor.js:117   |    Eagle Metadata: ✅ Updated
28-rust-cli-executor.js:117   |    🗑️  Deleted original file

// 但JS检测到元数据损坏
03-file-handler.js:80 [PIXLY File] ⚠️ Detected corrupted metadata: 
  name="0069QT6vly1h0wx9m63bdg30k00u07wr.avif" contains ext="avif"
  
03-file-handler.js:93 [PIXLY File] ✅ Auto-fixed metadata: 
  "0069QT6vly1h0wx9m63bdg30k00u07wr.avif" → "0069QT6vly1h0wx9m63bdg30k00u07wr"
```

**矛盾**: Rust说更新成功，但JS说元数据损坏！

---

## 🔎 根本原因

### Eagle metadata.json 格式要求

Eagle的元数据格式明确规定：

```json
{
  "name": "filename_without_ext",  // ✅ 不含扩展名
  "ext": "avif",                    // ✅ 扩展名（不含点）
  "size": 73991,
  "width": 800,
  "height": 600
}
```

### 错误的Rust实现

```rust
// ❌ 错误：file_name() 返回完整文件名（包括扩展名）
let new_filename = output_path.file_name()  // 返回 "test.avif"
    .and_then(|n| n.to_str())
    .unwrap_or("unknown");

metadata.name = new_filename.to_string();  // ❌ "test.avif"
metadata.ext = new_ext.to_string();        // ❌ "avif"
```

**结果**: 
```json
{
  "name": "test.avif",  // ❌ 包含了扩展名！
  "ext": "avif"         // ✅ 正确
}
```

### 为什么看起来"不会动"？

1. **Rust更新了metadata.json**，但name字段格式错误
2. **Eagle或JS检测到格式错误**，认为是corrupted metadata
3. **JS自动修复**，去掉name中的扩展名
4. **用户看到**: 文件"不会动"（实际上在Eagle内部已经是错误状态）

---

## ✅ 正确的修复

### 使用 `file_stem()` 而非 `file_name()`

```rust
// ✅ 正确：file_stem() 返回不含扩展名的文件名
let base_name = output_path.file_stem()  // 返回 "test"
    .and_then(|n| n.to_str())
    .unwrap_or("unknown");

metadata.name = base_name.to_string();  // ✅ "test"
metadata.ext = new_ext.to_string();     // ✅ "avif"
```

**结果**:
```json
{
  "name": "test",      // ✅ 不含扩展名
  "ext": "avif"        // ✅ 正确
}
```

---

## 📊 Rust Path API 对比

| 方法 | 输入 | 输出 | 用途 |
|------|------|------|------|
| `file_name()` | `/path/to/test.avif` | `Some("test.avif")` | 完整文件名 |
| `file_stem()` | `/path/to/test.avif` | `Some("test")` | ✅ 不含扩展名 |
| `extension()` | `/path/to/test.avif` | `Some("avif")` | ✅ 扩展名（不含点） |

**教训**: 操作Eagle元数据时，务必使用 `file_stem()`！

---

## 🧪 测试验证

### 测试步骤

1. 在Eagle库中选择一个GIF文件
2. 转换为AVIF格式
3. 检查 `.info/metadata.json`

### 预期结果

#### 修复前（❌ 错误）
```json
{
  "name": "test.avif",  // ❌ 包含扩展名
  "ext": "avif"
}
```

**Eagle表现**: 
- ⚠️ JS检测到corrupted metadata
- ⚠️ 自动修复name字段
- ⚠️ 用户感觉"不会动"

#### 修复后（✅ 正确）
```json
{
  "name": "test",       // ✅ 不含扩展名
  "ext": "avif"
}
```

**Eagle表现**:
- ✅ 元数据格式正确
- ✅ 无需自动修复
- ✅ 缩略图正常
- ✅ Eagle立即识别新文件

---

## 🔍 相关代码位置

### 修复的文件
- `pixly-rust/src/cli/conversion.rs` (line ~143)

### 修改内容
```diff
- let new_filename = output_path.file_name()
+ let base_name = output_path.file_stem()

- metadata.name = new_filename.to_string();
+ metadata.name = base_name.to_string();
```

### 相关检测代码
- `plugin/js/plugin-modules/03-file-handler.js` (line ~80-93)
  - 检测corrupted metadata
  - 自动修复name字段

---

## 📚 Eagle元数据规范

### 完整的metadata.json结构

```json
{
  "id": "MHISY2WJUIQVE",
  "name": "filename",           // ✅ 不含扩展名
  "ext": "avif",                // ✅ 不含点
  "size": 73991,
  "btime": 1699999999000,
  "mtime": 1699999999000,
  "lastModified": 1699999999000,
  "modificationTime": 1699999999000,
  "width": 800,
  "height": 600,
  "tags": ["tag1", "tag2"],
  "folders": [],
  "isDeleted": false,
  "url": "",
  "annotation": ""
}
```

### 关键规则

1. **name**: 文件名，**不含扩展名**
2. **ext**: 扩展名，**不含点**
3. **时间戳**: 毫秒级Unix时间戳
4. **尺寸**: 像素单位

---

## 🎯 质量改进

### 这次bug暴露的问题

1. **测试不足**: 没有验证Eagle元数据的正确性
2. **API选择错误**: 使用了 `file_name()` 而非 `file_stem()`
3. **隐蔽性强**: Rust报告成功，但实际格式错误

### 改进措施

1. **✅ 添加注释**: 明确标注 "Eagle的name字段不含扩展名"
2. **✅ 使用正确API**: `file_stem()` for name, `extension()` for ext
3. **🔜 添加测试**: 验证生成的metadata.json格式
4. **🔜 添加Schema验证**: 确保metadata符合Eagle规范

---

## 📖 参考资料

### Eagle文档
- Eagle API: https://developer.eagle.cool/plugin-api/v/zh-cn
- 元数据格式: https://cn.eagle.cool/support/article/...

### 相关代码
- `converter/eagle_adapter.rs`: EagleImageMetadata定义
- `cli/conversion.rs`: 转换主逻辑
- `plugin/js/03-file-handler.js`: JS端元数据检测

---

**修复完成时间**: 2025-11-06 15:00  
**修复者**: Claude (在用户bug报告指导下)  
**影响范围**: 所有格式转换（AVIF, WebP, JXL等）  
**状态**: ✅ **已编译，待测试**

---

**🎯 请在Eagle中重新测试：**
1. 转换GIF → AVIF
2. 转换GIF → WebP  
3. 检查metadata.json中name字段是否不含扩展名
4. 验证Eagle缩略图是否正常显示
