# PIXLY 插件系统

## 🎯 概述

PIXLY 提供两个专业化的独立插件版本，满足不同用户群体的需求：

- **🤖 AI 版本**: 智能多媒体处理 - 最简化的 AI 驱动体验
- **⚙️ Format 版本**: 专业格式转换 - 最完整的参数控制能力

## 📦 版本选择

### 我应该使用哪个版本？

#### 选择 AI 版本，如果你：
- ✅ 需要快速批量处理文件
- ✅ 不想了解复杂的技术参数
- ✅ 信任 AI 自动优化
- ✅ 追求效率和便捷性
- ✅ 处理日常照片和视频

#### 选择 Format 版本，如果你：
- ✅ 需要精确控制每个参数
- ✅ 了解图像/视频编码技术
- ✅ 处理专业摄影/视频作品
- ✅ 需要特定的输出要求
- ✅ 进行格式实验和测试

#### 两个版本都安装，如果你：
- ✅ 既有日常处理需求，又有专业需求
- ✅ 想根据不同场景选择不同工具
- ✅ 需要对比 AI 和手动的效果

## 🚀 快速开始

### AI 版本使用流程

```
1. 选择文件
   ↓
2. 查看 AI 识别的媒体类型
   ↓
3. 选择预设模式（平衡/质量/快速）
   ↓
4. 调整 AI 功能开关（可选）
   ↓
5. 点击"开始智能转换"
   ↓
6. 等待完成
```

**预计时间**: 3-5 分钟（100 张照片）

### Format 版本使用流程

```
1. 选择文件
   ↓
2. 选择转换类型（图像/视频）
   ↓
3. 选择输出格式
   ↓
4. 调整质量参数
   ↓
5. 展开高级选项，精细调整
   ↓
6. 点击"开始转换"
   ↓
7. 等待完成
```

**预计时间**: 取决于参数设置

## 📖 详细文档

### 核心文档
- [重构完成说明（中文）](./重构完成说明_CN.md) - 完整的重构说明
- [Refactoring Report (English)](./REFACTORING_COMPLETE_V2.md) - 英文重构报告
- [版本对比图表](./COMPARISON_CHART.md) - 详细的功能对比

### AI 版本文档
- 位置: `plugin/ai/`
- 主文件: `index.html`
- 核心逻辑: `js/ai-core.js`
- 样式: `css/ai-styles.css`

### Format 版本文档
- 位置: `plugin/format/`
- 主文件: `index.html`
- 核心逻辑: `js/format-core.js`
- 样式: `css/format-styles.css`

## 🎨 功能特性

### AI 版本特性

#### 智能预设
- **平衡模式** (推荐): ML + 平衡质量 + SSIM 验证
- **质量优先**: 完整 AI 配置 + 最高质量
- **快速模式**: 智能通道 + 高级功能

#### AI 功能
- 🎯 **智能质量预测**: 基于内容复杂度自动调整
- 📦 **格式智能推荐**: 自动选择最优输出格式
- 🔬 **质量验证**: SSIM/VMAF 自动验证
- 🎬 **动图转视频**: 大型动图推荐视频格式

#### 自动化
- ✅ GPU 自动检测
- ✅ 媒体类型自动识别
- ✅ 参数自动优化
- ✅ 批量并行处理

### Format 版本特性

#### 图像格式
- **JXL**: Effort, Distance, Modular, Progressive, Responsive, Gaborish, Photon Noise, Decoding Speed, Bit Depth, Color Space, Patches
- **AVIF**: Speed, Min/Max Quantizer, Chroma Subsampling, Bit Depth, Tiles, Premultiplied Alpha
- **HEIC**: Quality, Encoder, Chroma Subsampling, Lossless, Thumbnail Embed
- **WebP**: Quality, Lossless, Method
- **PNG**: Compression Level, Filter
- **JPEG**: Quality, Subsampling, Progressive

#### 视频编码
- **容器**: MP4, MOV, MKV, WebM
- **编码器**: H.265, H.266, AV1, VP9
- **质量控制**: CRF, Rate Control, Bitrate
- **编码结构**: GOP Size, B-frames, Reference Frames
- **运动估计**: ME Range, Subme, ME Method
- **滤波器**: Deblock (Alpha/Beta)

## � 系统要求

### 最低要求
- **操作系统**: Windows 10+, macOS 10.15+, Linux
- **内存**: 4GB RAM
- **存储**: 100MB 可用空间
- **Eagle**: 3.0+ (如果使用 Eagle 集成)

### 推荐配置
- **操作系统**: Windows 11, macOS 12+, Linux (最新)
- **内存**: 8GB+ RAM
- **GPU**: 支持硬件加速的显卡
- **存储**: 500MB+ 可用空间

### GPU 加速支持
- ✅ NVIDIA (NVENC)
- ✅ AMD (AMF)
- ✅ Intel (QSV)
- ✅ Apple Silicon (VideoToolbox)

## 🔧 安装指南

### 方法 1: Eagle 插件市场（推荐）
1. 打开 Eagle
2. 进入插件市场
3. 搜索 "PIXLY AI" 或 "PIXLY Format"
4. 点击安装

### 方法 2: 手动安装
1. 下载对应版本的文件夹
2. 复制到 Eagle 插件目录
3. 重启 Eagle
4. 在插件列表中启用

### 方法 3: 开发模式
1. Clone 仓库
2. 在 Eagle 中启用开发者模式
3. 加载插件文件夹
4. 开始使用

## 📊 性能基准

### AI 版本性能

| 场景 | 文件数量 | 处理时间 | GPU 加速 |
|------|---------|---------|---------|
| 日常照片 | 100 张 | 3-5 分钟 | ✅ |
| 高清照片 | 50 张 | 5-8 分钟 | ✅ |
| 动图 | 20 个 | 2-4 分钟 | ✅ |
| 视频 | 10 个 | 10-15 分钟 | ✅ |

### Format 版本性能

| 场景 | 文件数量 | 处理时间 | 取决于 |
|------|---------|---------|--------|
| 精细图像 | 10 张 | 5-10 分钟 | 参数设置 |
| 专业视频 | 5 个 | 15-30 分钟 | 编码参数 |
| 实验测试 | 1 个 | 1-5 分钟 | 参数复杂度 |

## 🎓 使用技巧

### AI 版本技巧

1. **批量处理**: 一次选择多个文件，AI 会自动并行处理
2. **预设选择**: 
   - 日常使用 → 平衡模式
   - 存档保存 → 质量优先
   - 快速分享 → 快速模式
3. **AI 功能**: 
   - 保持"智能质量预测"开启
   - 大文件开启"动图转视频"
4. **查看推荐**: 注意 AI 推荐卡片的建议

### Format 版本技巧

1. **参数预设**: 
   - JXL: Effort=7, Distance=1.0 (日常)
   - AVIF: Speed=6, Quantizer=20-40 (平衡)
   - H.265: CRF=23, Preset=medium (推荐)
2. **质量控制**:
   - 图像: 85-95 为最佳平衡点
   - 视频: CRF 18-23 为高质量
3. **高级参数**:
   - 不确定时使用默认值
   - 逐个调整，观察效果
4. **保存设置**: 记录最佳参数组合

## 🐛 常见问题

### AI 版本

**Q: AI 推荐的格式不是我想要的？**
A: AI 版本专注自动化，如需指定格式请使用 Format 版本。

**Q: 处理速度慢？**
A: 检查 GPU 是否启用，尝试"快速模式"。

**Q: 质量不满意？**
A: 切换到"质量优先"模式，或使用 Format 版本手动调整。

### Format 版本

**Q: 参数太多不知道怎么设置？**
A: 使用默认值即可，或参考"使用技巧"部分。

**Q: 转换失败？**
A: 检查参数范围，查看日志输出。

**Q: 想要快速处理？**
A: 建议使用 AI 版本，Format 版本适合精细处理。

### 通用问题

**Q: 两个版本可以同时安装吗？**
A: 可以，它们完全独立。

**Q: 如何选择版本？**
A: 参考"版本选择"部分。

**Q: 支持哪些格式？**
A: 参考"功能特性"部分。

## 🔗 相关链接

- [GitHub 仓库](https://github.com/your-repo/pixly)
- [问题反馈](https://github.com/your-repo/pixly/issues)
- [更新日志](./CHANGELOG.md)
- [贡献指南](./CONTRIBUTING.md)

## 📄 许可证

MIT License - 详见 [LICENSE](../LICENSE) 文件

## 🙏 致谢

感谢所有贡献者和用户的支持！

---

**最后更新**: 2025-01-17  
**版本**: V2.0  
**维护者**: PIXLY Team
