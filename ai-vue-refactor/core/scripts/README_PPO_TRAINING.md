# PPO训练脚本使用指南

## 📚 概述

本目录包含用于PPO（Proximal Policy Optimization）强化学习训练的脚本，支持**图像、视频、音频**三种媒体类型的转换优化训练。

---

## 🎯 问题背景

**发现的问题**: 原有的机器学习功能过度关注图像处理，PPO功能仅支持图像，完全忽略了视频和音频！

**解决方案**: 创建全媒体类型的PPO训练系统，使用FFmpeg统一处理所有媒体格式。

---

## 📁 脚本文件

### 1. `ppo_train_ffmpeg_only.py` ⭐ **推荐使用**

**功能**: 使用纯FFmpeg方案处理所有媒体类型

**特点**:
- ✅ 支持图像、视频、音频
- ✅ 统一的FFmpeg处理流程
- ✅ 自动化训练流程
- ✅ 详细的进度显示

**使用方法**:
```bash
python3 scripts/ppo_train_ffmpeg_only.py
```

**支持的格式**:
- **图像**: WebP
- **视频**: WebM (VP9), MP4 (H.264)
- **音频**: Opus, AAC, MP3

---

### 2. `ppo_train_all_media_hybrid.py`

**功能**: 混合策略 - 图像使用Rust CLI，视频/音频使用FFmpeg

**状态**: ⚠️ 实验性 - Rust CLI对某些图像格式支持有限

**使用方法**:
```bash
python3 scripts/ppo_train_all_media_hybrid.py
```

---

### 3. `ppo_train_chromium.py`

**功能**: 原始的仅图像训练脚本

**状态**: ⚠️ 已过时 - 仅支持图像，不推荐使用

---

### 4. `ppo_train_all_media.py`

**功能**: 早期的全媒体类型尝试

**状态**: ⚠️ 已过时 - 使用Rust CLI，兼容性问题

---

## 🚀 快速开始

### 前置要求

1. **FFmpeg** (必需)
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt install ffmpeg

# 验证安装
ffmpeg -version
```

2. **Python 3.7+**
```bash
python3 --version
```

3. **Chromium测试数据** (可选)
```bash
# 如果使用其他数据集，修改脚本中的 data_dir 路径
data_dir = '/path/to/your/media/files'
```

---

### 运行训练

#### 步骤1: 小规模测试
脚本会自动进行小规模测试（每种类型2个文件）

```bash
python3 scripts/ppo_train_ffmpeg_only.py
```

#### 步骤2: 查看测试结果
```bash
cat models/ppo_training_ffmpeg_test.json
```

#### 步骤3: 全量训练
如果测试成功，脚本会自动继续全量训练

---

## 📊 训练数据格式

生成的JSON文件结构：

```json
{
  "timestamp": "2025-11-17T00:30:38.539873",
  "total_samples": 1479,
  "stats": {
    "image": 24,
    "video": 1071,
    "audio": 384
  },
  "data": [
    {
      "media_type": "image",
      "input_file": "/path/to/input.jpg",
      "input_size": 98734,
      "target_format": "webp",
      "quality": 85,
      "output_size": 89604,
      "compression_ratio": 0.9075,
      "reward": 0.3955
    }
  ]
}
```

---

## 🎨 自定义训练

### 修改数据源

编辑脚本中的 `data_dir`:

```python
data_dir = '/Users/your_username/your_media_folder'
```

### 修改质量参数

```python
# 图像质量
qualities = [70, 85, 95]  # 修改为你想要的质量值

# 视频比特率 (kbps)
qualities = [500, 1000, 2000]

# 音频比特率 (kbps)
qualities = [128, 192, 256]
```

### 修改目标格式

```python
# 图像格式
formats = ['webp', 'avif', 'jxl']  # 需要FFmpeg支持

# 视频格式
formats = ['webm', 'mp4', 'mkv']

# 音频格式
formats = ['opus', 'aac', 'mp3', 'flac']
```

### 修改奖励函数

```python
def calculate_reward(compression_ratio, quality, media_type):
    compression_score = max(0, 1 - compression_ratio)
    
    if media_type == 'image':
        quality_score = quality / 100
        # 调整权重: 压缩率 vs 质量
        return compression_score * 0.6 + quality_score * 0.4
```

---

## 📈 训练结果分析

### 查看统计信息

```bash
python3 -c "
import json
with open('models/ppo_training_all_media_20251117_003038.json', 'r') as f:
    data = json.load(f)
    print(f'总样本数: {data[\"total_samples\"]}')
    print(f'图像: {data[\"stats\"][\"image\"]}')
    print(f'视频: {data[\"stats\"][\"video\"]}')
    print(f'音频: {data[\"stats\"][\"audio\"]}')
"
```

### 分析压缩率

```bash
python3 -c "
import json
with open('models/ppo_training_all_media_20251117_003038.json', 'r') as f:
    data = json.load(f)
    ratios = [d['compression_ratio'] for d in data['data']]
    print(f'平均压缩率: {sum(ratios)/len(ratios):.2%}')
    print(f'最佳压缩率: {min(ratios):.2%}')
    print(f'最差压缩率: {max(ratios):.2%}')
"
```

---

## 🔧 故障排除

### FFmpeg未找到
```bash
# 检查FFmpeg是否在PATH中
which ffmpeg

# 如果没有，安装FFmpeg
brew install ffmpeg  # macOS
```

### 转换失败
某些文件转换失败是正常的：
- 加密视频 (CENC)
- 损坏的媒体文件
- 不支持的编码格式

### 内存不足
如果处理大文件时内存不足：
```python
# 减少sample_size
test_data = generate_training_data(media_files, sample_size=10)
```

### 超时错误
增加超时时间：
```python
# 在convert_video_ffmpeg中
timeout=300  # 从120增加到300秒
```

---

## 📝 最佳实践

1. **先测试后训练**: 始终先运行小规模测试
2. **监控磁盘空间**: 确保/tmp有足够空间
3. **备份数据**: 训练前备份重要的媒体文件
4. **检查日志**: 注意转换失败的文件
5. **验证结果**: 检查生成的JSON文件是否完整

---

## 🎯 训练目标

使用生成的训练数据来：

1. **训练PPO模型**: 学习最优的转换参数
2. **格式推荐**: 根据输入自动选择最佳输出格式
3. **质量优化**: 平衡文件大小和质量
4. **比特率预测**: 预测最优的编码比特率

---

## 📚 相关文档

- `models/TRAINING_SUMMARY_20251117.md` - 训练结果总结
- `docs/architecture/PROJECT_QUALITY_MANIFESTO.md` - 项目质量标准
- `src/ppo_model.rs` - PPO模型实现

---

## 🤝 贡献

如果你改进了训练脚本或发现了问题：

1. 创建详细的issue描述
2. 提交PR并说明改进点
3. 更新相关文档

---

**最后更新**: 2025-11-17  
**维护者**: Pixly Team
