# ML特征工程参考文档

**创建日期**: 2025-11-20  
**来源**: 从废弃的 `ml_predictor.rs` 提取  
**目的**: 保留有价值的特征工程知识

## 📊 12维特征定义（历史参考）

这是早期的特征定义，后来升级为128维特征。

### 特征列表

| # | 特征名 | 含义 | 数据类型 |
|---|--------|------|----------|
| 1 | width | 图像宽度（像素） | f64 |
| 2 | height | 图像高度（像素） | f64 |
| 3 | pixels | 总像素数 (width × height) | f64 |
| 4 | aspect_ratio | 宽高比 (width / height) | f64 |
| 5 | has_alpha | 是否有透明通道 (0/1) | f64 |
| 6 | edge_strength | 边缘强度 (complexity × 100) | f64 |
| 7 | texture_complexity | 纹理复杂度 (complexity × 15) | f64 |
| 8 | noise_level | 噪声水平 | f64 |
| 9 | detail_level | 细节水平（文件大小） | f64 |
| 10 | compression_score | 压缩分数 | f64 |
| 11 | high_freq_energy | 高频能量 | f64 |
| 12 | low_freq_energy | 低频能量 | f64 |

### 标准化参数（WebP模型）

**来源**: 历史训练数据统计

**Scaler Mean**:
```
[1288.93, 2524.03, 3790879.02, 0.85, 0.05, 48.95, 10.22, 0.36, 7019092196.19, 0.90, 0.96, 0.78]
```

**Scaler Std**:
```
[810.72, 2828.35, 7017255.06, 0.51, 0.21, 30.00, 7.54, 1.39, 29000190505.96, 0.11, 0.09, 0.40]
```

### 特征提取逻辑

```rust
// 从ImageFeatures创建ML特征
pub fn from_image_features(features: &ImageFeatures) -> MLFeatures {
    let pixels = (features.width as f64) * (features.height as f64);
    let aspect_ratio = if features.height > 0 {
        features.width as f64 / features.height as f64
    } else {
        1.0
    };
    
    MLFeatures {
        width: features.width as f64,
        height: features.height as f64,
        pixels,
        aspect_ratio,
        has_alpha: if features.has_alpha { 1.0 } else { 0.0 },
        edge_strength: features.complexity * 100.0,
        texture_complexity: features.complexity * 15.0,
        noise_level: 0.3,  // 默认值
        detail_level: features.file_size as f64,
        compression_score: 0.9,
        high_freq_energy: 0.95,
        low_freq_energy: 0.78,
    }
}
```

## 🚀 升级到128维特征

**当前系统**: `feature_extractor_128d.rs`

**改进**:
1. ✅ 更丰富的特征维度（12 → 128）
2. ✅ 更精确的图像分析
3. ✅ 支持更多格式和场景
4. ✅ 与Python ML Bridge完全集成

**参考**: `src/feature_extractor_128d.rs`

## 📝 为什么废弃12维版本？

1. **伪装的ML实现** - `ml_predictor.rs` 声称使用LightGBM，实际是硬编码规则
2. **从未被使用** - CLI从不调用，只在测试中存在
3. **已有更好替代** - Python ML Bridge + 128维特征
4. **违反真实性原则** - 注释说"真实AI"但实际是if-else

## 🎓 教训

1. **特征工程很重要** - 12维定义有参考价值
2. **实现要真实** - 不要伪装成ML
3. **集成要完整** - 要么真正使用，要么不要保留
4. **文档化知识** - 即使删除代码，也要保留知识

## 🔗 相关文件

- 当前特征提取: `src/feature_extractor_128d.rs`
- Python ML Bridge: `scripts/ml_bridge.py`
- 训练数据: `models/training_data_normalized.json`
- LightGBM模型: `models/lightgbm_quality_128d.txt`

---

**注**: 本文档仅作历史参考，实际开发请使用128维特征系统。
