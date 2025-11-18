# PIXLY AI 全媒体智能优化插件 - 架构设计

## 项目定位

**全媒体AI智能优化插件** - 为图片、视频、音频提供AI驱动的格式和参数优化建议

## 核心功能

### 1. 媒体类型识别
- 静态图片 (JPG/PNG/WebP/AVIF/JXL)
- 动态图片 (GIF/APNG/动态WebP)
- 透明图片 (PNG/WebP/AVIF with alpha)
- 视频文件 (MP4/MOV/WebM/MKV)
- 音频文件 (MP3/AAC/FLAC/Opus)

### 2. AI特征分析
- **图片分析**
  - 分辨率和像素密度
  - 色彩复杂度和纹理
  - 透明通道检测
  - 动画帧数和帧率
  
- **视频分析**
  - 分辨率和帧率
  - 当前编码器和码率
  - 场景复杂度
  - 音频轨道信息

- **音频分析**
  - 采样率和比特率
  - 声道数和编码格式
  - 动态范围

### 3. 智能推荐系统
- **格式推荐**
  - 基于内容类型
  - 基于目标用途
  - 基于兼容性需求
  
- **参数推荐**
  - Quality/Effort/Speed
  - CRF/Preset/Bitrate
  - 采样率/比特率

- **优化策略**
  - Size优先 (最小文件)
  - Balanced (平衡)
  - Quality优先 (最高质量)

### 4. 预览对比
- 原始文件信息
- 优化后预估效果
- 文件大小对比
- 质量评分 (SSIM)

### 5. 一键应用
- 将推荐参数传递给format-vue插件
- 或直接在AI插件内执行转换

## 技术架构

### 前端 (Vue3 + Element Plus)
```
src/
├── App.vue                 # 主应用
├── components/
│   ├── MediaAnalyzer.vue   # 媒体分析组件
│   ├── RecommendCard.vue   # 推荐卡片
│   ├── CompareView.vue     # 对比视图
│   ├── BatchList.vue       # 批量列表
│   └── StatusDialog.vue    # 状态对话框
├── composables/
│   ├── useEagleAPI.js      # Eagle API封装
│   ├── useRustCLI.js       # Rust CLI调用
│   └── useAIAnalysis.js    # AI分析逻辑
└── styles/
    ├── variables.css       # AI主题变量
    └── global.css          # 全局样式
```

### 后端 (Rust CLI)
```
pixly-rust analyze <file> --ai
    ↓
返回JSON:
{
  "media_type": "image|video|audio",
  "features": {...},
  "recommendations": {
    "format": "avif",
    "params": {
      "quality": 85,
      "effort": 6
    },
    "estimated_size": "1.2MB",
    "confidence": 0.92
  }
}
```

## UI设计参考

### 参考官方AI插件的设计元素
1. **AI渐变色** - `--color-ai-gradient-1`, `--color-ai-gradient-2`
2. **Comet动画** - 旋转渐变边框
3. **状态图标系统** - 分析中/成功/失败
4. **主题适配** - 支持Eagle所有主题
5. **模态对话框** - AI分析进度展示

### 独特的UI元素
1. **媒体类型图标** - 图片/视频/音频区分
2. **推荐卡片** - 展示多个优化方案
3. **对比滑块** - 原始vs优化效果
4. **批量分组** - 按媒体类型分组展示

## 与format-vue插件的集成

### 方案1: 参数传递
```javascript
// AI插件分析完成后
const recommendations = await analyzeMedia(file);

// 打开format-vue插件并传递参数
await eagle.plugin.open('pixly-format-converter', {
  files: [file],
  format: recommendations.format,
  params: recommendations.params
});
```

### 方案2: 独立转换
```javascript
// AI插件内直接调用Rust CLI转换
await rustCLI.convert(file, recommendations);
```

## 开发计划

### Phase 1: 基础架构 (当前)
- [x] 项目结构设计
- [ ] Vue3 + Element Plus搭建
- [ ] Eagle API集成
- [ ] Rust CLI基础调用

### Phase 2: 图片分析
- [ ] 静态图片特征提取
- [ ] 动态图片检测
- [ ] 透明通道检测
- [ ] AI推荐算法

### Phase 3: 视频分析
- [ ] 视频信息提取
- [ ] 编码器检测
- [ ] 参数推荐

### Phase 4: UI完善
- [ ] AI主题样式
- [ ] 对比视图
- [ ] 批量处理
- [ ] 状态管理

### Phase 5: 集成测试
- [ ] 与format-vue集成
- [ ] 批量测试
- [ ] 性能优化

## 质量标准

遵循 `PROJECT_QUALITY_MANIFESTO.md`:
- ✅ 真实的AI调用 (不模拟)
- ✅ 响亮的错误处理
- ✅ 完整的功能实现
- ✅ 参考官方插件设计
- ✅ 全媒体类型支持
