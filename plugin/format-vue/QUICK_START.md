# PIXLY Format Vue - 快速开始

**版本**: 1.0.0  
**更新**: 2025-11-18

---

## 🚀 5分钟快速上手

### 1. 安装依赖工具

```bash
# macOS
brew install jpeg-xl libavif ffmpeg exiftool

# Linux
apt install libjxl-tools libavif-bin ffmpeg libimage-exiftool-perl
```

### 2. 编译Rust内核

```bash
cd /path/to/pixly
cargo build --release
```

### 3. 放置二进制文件

将`target/release/pixly-converter`复制到：
- `plugin/format-vue/bin/pixly-converter`

或确保在系统PATH中。

### 4. 在Eagle中使用

1. 打开Eagle
2. 选择要转换的文件
3. 打开PIXLY Format插件
4. 选择输出格式
5. 调整参数（可选）
6. 点击"开始转换"

---

## 📷 图像转换

### 支持格式
- **JXL** - 次世代格式，最佳压缩率
- **AVIF** - AV1编码，Web优化
- **WebP** - Google格式，广泛支持
- **HEIC** - Apple生态，高效压缩

### 快速设置

#### 极致质量（推荐）
- 质量：95
- JXL Effort：7
- AVIF Speed：2

#### 平衡模式
- 质量：90
- JXL Effort：5
- AVIF Speed：4

#### 快速压缩
- 质量：85
- JXL Effort：3
- AVIF Speed：6

---

## 🎬 视频转换

### 支持容器
- MP4, MOV, WebM, MKV

### 快速设置

#### 高质量
- CRF：18
- GOP：120
- B-Frames：3

#### 平衡
- CRF：23
- GOP：60
- B-Frames：2

#### 小文件
- CRF：28
- GOP：30
- B-Frames：1

---

## 🔧 自动化功能

### XMP合并（默认启用）
- ✅ 自动检测XMP sidecar文件
- ✅ 自动合并到输出文件
- ✅ 自动删除原XMP文件
- ✅ 支持Eagle独立XMP资源

**无需任何操作，自动完成！**

### 文件名规范化（可选）
- 处理特殊字符
- 处理空格
- 避免编码器兼容性问题

**在高级设置中启用**

---

## ❓ 常见问题

### Q: 转换失败，提示"JXL编码器未安装"
**A**: 安装JXL工具
```bash
brew install jpeg-xl
```

### Q: 转换失败，提示"AVIF编码器未安装"
**A**: 安装AVIF工具
```bash
brew install libavif
```

### Q: XMP文件没有合并
**A**: 
1. 检查XMP文件名是否匹配（`photo.jpg` → `photo.xmp`）
2. 查看控制台日志确认XMP检测
3. 确保安装了exiftool

### Q: 文件名有特殊字符，转换失败
**A**: 启用"规范化文件名"选项

### Q: 进度条不动
**A**: 
1. 检查控制台是否有错误
2. 确认pixly-converter可执行
3. 查看Rust CLI输出

---

## 🎯 最佳实践

### 1. 选择合适的格式

| 用途 | 推荐格式 | 原因 |
|------|---------|------|
| 照片归档 | JXL | 最佳压缩率，支持无损 |
| Web展示 | AVIF | 浏览器支持好，压缩率高 |
| 兼容性优先 | WebP | 广泛支持 |
| Apple生态 | HEIC | 系统原生支持 |

### 2. 质量设置建议

| 内容类型 | 质量 | 说明 |
|---------|------|------|
| 照片 | 90-95 | 保持细节 |
| 插画 | 85-90 | 平衡质量和大小 |
| 截图 | 95-100 | 文字清晰 |
| 图标 | 100 | 无损 |

### 3. 批量转换技巧

1. **分批处理**: 每批10-20个文件
2. **统一参数**: 相同类型文件使用相同参数
3. **检查结果**: 转换完成后抽查质量
4. **备份原文件**: 重要文件先备份

---

## 🔍 调试技巧

### 查看详细日志
打开浏览器控制台（F12），查看：
- `[PIXLY]` - 插件日志
- `[pixly-converter]` - Rust CLI输出

### 常见日志

#### 成功转换
```
[PIXLY] Converting file
[pixly-converter] 🔄 Converting: input.jpg
[pixly-converter] ✅ Conversion complete!
```

#### XMP合并
```
[pixly-converter] 📎 Found XMP sidecar
[pixly-converter] ✅ XMP merge verified
[pixly-converter] 🗑️ XMP sidecar deleted
```

#### 错误
```
[PIXLY] Conversion failed
[pixly-converter stderr] Error: ...
```

---

## 📞 获取帮助

### 文档
- 完整文档：`IMPLEMENTATION_COMPLETE.md`
- 验证清单：`VERIFICATION_CHECKLIST.md`
- 修复报告：`VUE_CONVERSION_FIX_REPORT.md`

### 问题排查
1. 检查控制台日志
2. 验证工具安装
3. 确认文件路径
4. 查看错误提示

---

## ✅ 快速检查清单

开始使用前，确认：
- [ ] 已安装外部工具（jpeg-xl, libavif, ffmpeg, exiftool）
- [ ] 已编译Rust内核
- [ ] pixly-converter可执行
- [ ] 在Eagle中选择了文件
- [ ] 插件正常加载

---

**🎉 准备就绪！开始转换吧！**

**💡 提示**: 首次使用建议先用少量文件测试，确认一切正常后再批量转换。
