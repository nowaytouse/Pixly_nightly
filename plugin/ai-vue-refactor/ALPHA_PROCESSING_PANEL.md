# 🎨 Alpha通道处理面板

## 概述

Alpha通道处理面板是AI Vue插件的高级功能，提供透明度处理的深度可视化和技术细节展示。

## 功能特性

### 1. 实时透明度检测
- ✅ 自动检测图像是否包含Alpha通道
- ✅ 统计透明像素数量和百分比
- ✅ 分类透明度类型（二值/渐变/复杂）

### 2. 处理流程可视化
展示5个关键处理步骤：
1. **Alpha通道检测** - 检查色彩空间
2. **像素级分析** - 统计Alpha值分布
3. **类型分类** - 识别透明度特征
4. **格式优化** - 选择最佳格式
5. **Alpha编码** - 应用专用参数

### 3. 格式兼容性矩阵
显示各格式对Alpha通道的支持：
- **完全支持**: WebP, AVIF, PNG, JXL
- **部分支持**: HEIC
- **不支持**: JPEG

### 4. 高级处理选项

#### Alpha质量控制 (0-100)
```rust
// Rust内核参数
--qalpha 90
```
控制透明通道的压缩质量，值越高质量越好但文件越大。

#### 预乘Alpha
```rust
// AVIF参数
premultiply_alpha: true
```
将RGB值与Alpha值预先相乘，提高某些格式的压缩效率。

#### 分离Alpha通道
将Alpha通道单独编码，可能获得更好的压缩率。

### 5. 处理统计
- 总像素数
- 不透明像素数
- 完全透明像素数
- 半透明像素数

### 6. 技术细节展示

#### 内部处理流程
详细展示Rust内核中的实际代码逻辑：
```rust
// 1. Alpha通道检测
has_alpha = img.color().has_alpha()

// 2. 像素级分析
alpha_pixels = count(pixel[3] < 255)

// 3. 类型分类
type = binary | gradient | complex

// 4. 格式兼容性检查
format_support = check_alpha_compatibility()

// 5. 编码参数优化
--qalpha 90 --premultiply true
```

#### 算法伪代码
提供完整的透明度处理算法伪代码，包括：
- Alpha通道检测逻辑
- 像素分布分析
- 类型分类算法
- 格式选择策略
- 编码参数优化

## 使用方法

### 触发条件
当选择的文件中包含可能有Alpha通道的格式时，面板自动显示：
- PNG
- WebP
- AVIF
- JXL
- HEIC
- GIF
- APNG

### 交互操作
1. **展开/折叠面板** - 点击标题栏
2. **调整Alpha质量** - 拖动滑块 (0-100)
3. **启用预乘Alpha** - 勾选复选框
4. **分离Alpha通道** - 勾选复选框
5. **查看技术细节** - 展开"技术细节"区域

## 技术实现

### 组件结构
```
AlphaProcessingPanel.vue
├── 检测状态区域
├── 处理流程区域
├── 格式兼容性区域
├── 高级选项区域
├── 统计信息区域
└── 技术细节区域
```

### 数据流
```
用户选择文件
    ↓
App.vue检测Alpha通道
    ↓
显示AlphaProcessingPanel
    ↓
用户调整参数
    ↓
触发事件 (alpha-quality-change等)
    ↓
App.vue更新状态
    ↓
传递给Rust CLI
    ↓
Rust内核处理
```

### 事件接口
```javascript
// Alpha质量变化
@alpha-quality-change="handleAlphaQualityChange"

// 预乘Alpha变化
@premultiply-change="handlePremultiplyChange"

// 分离Alpha通道变化
@separate-alpha-change="handleSeparateAlphaChange"
```

## Rust内核集成

### 相关模块
- `src/conversion_core.rs` - Alpha质量参数
- `src/modern_formats.rs` - 预乘Alpha选项
- `src/format_recommender.rs` - Alpha格式推荐
- `src/visual_quality_scorer.rs` - 透明度检测
- `src/color_quantizer.rs` - 透明度分析

### 关键代码位置
```rust
// conversion_core.rs:35
pub alpha_quality: Option<u8>,

// conversion_core.rs:846
if let Some(alpha_q) = config.alpha_quality {
    cmd.arg("--qalpha").arg(alpha_q.to_string());
}

// modern_formats.rs:565
pub premultiply_alpha: bool,

// visual_quality_scorer.rs:30
pub has_transparency: bool,
```

## 国际化支持

### 中文 (zh_CN)
- 完整的中文翻译
- 专业技术术语
- 详细的功能说明

### 英文 (en)
- 完整的英文翻译
- 标准技术术语
- 清晰的功能描述

## 样式设计

### 主题支持
- ✅ 亮色主题
- ✅ 暗色主题
- ✅ CSS变量驱动
- ✅ 平滑过渡动画

### 响应式布局
- Grid布局自适应
- 移动端友好
- 灵活的卡片系统

## 性能优化

### 计算优化
- 使用computed属性缓存计算结果
- 条件渲染减少DOM节点
- 事件防抖处理

### 渲染优化
- v-if条件渲染
- details元素懒加载
- CSS transform硬件加速

## 未来扩展

### 计划功能
- [ ] 实时Alpha通道预览
- [ ] Alpha直方图可视化
- [ ] 透明度热力图
- [ ] 批量Alpha处理统计
- [ ] Alpha通道导出功能

### 技术改进
- [ ] WebGL加速预览
- [ ] Worker线程分析
- [ ] 增量更新优化
- [ ] 缓存策略优化

## 参考资料

### Rust内核文档
- `docs/FORMAT_SUPPORT.md` - 格式支持说明
- `src/conversion_core.rs` - 转换核心逻辑

### 相关标准
- PNG Alpha通道规范
- WebP Alpha压缩
- AVIF Alpha编码
- JXL Alpha处理

---

**创建日期**: 2025-11-20  
**版本**: 1.0.0  
**状态**: ✅ 已完成并集成
