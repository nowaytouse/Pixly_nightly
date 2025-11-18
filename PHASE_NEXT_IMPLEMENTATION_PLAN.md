# 🚀 Phase Next: 完整ML系统实施计划

**日期**: 2025-11-18  
**目标**: 完善特征提取 + 视频ML集成 + 模型训练  
**预计时间**: 20-30小时

---

## 📋 任务优先级

### ✅ Phase 1: 完善128维特征提取 (已完成)

**目标**: 从占位符升级为真实特征提取

**完成时间**: 2025-11-18  
**实际耗时**: ~2小时

#### ✅ 1.1 Color特征 (16维) - 已完成

**文件**: `src/feature_extractor_128d.rs` - `extract_color_features()`

**已实现特征**:
1. RGB均值 (3维)
2. RGB标准差 (3维)
3. HSV均值 (3维)
4. HSV标准差 (3维)
5. 主色调 (1维)
6. 颜色数量 (1维)
7. 饱和度均值 (1维)
8. 亮度均值 (1维)

**实现方法**:
```rust
fn extract_color_features(img: &DynamicImage) -> Vec<f64> {
    let rgba = img.to_rgba8();
    let pixels: Vec<_> = rgba.pixels().collect();
    
    // RGB统计
    let (r_mean, g_mean, b_mean) = calculate_rgb_mean(&pixels);
    let (r_std, g_std, b_std) = calculate_rgb_std(&pixels);
    
    // HSV转换和统计
    let hsv_pixels = rgb_to_hsv(&pixels);
    let (h_mean, s_mean, v_mean) = calculate_hsv_mean(&hsv_pixels);
    
    // 颜色数量（量化到256色）
    let color_count = count_unique_colors(&pixels, 256);
    
    vec![
        r_mean, g_mean, b_mean,
        r_std, g_std, b_std,
        h_mean, s_mean, v_mean,
        h_std, s_std, v_std,
        dominant_hue,
        color_count as f64,
        s_mean,  // 饱和度
        v_mean,  // 亮度
    ]
}
```

#### ✅ 1.2 Texture特征 (16维) - 已完成

**文件**: `src/feature_extractor_128d.rs` - `extract_texture_features()`

**已实现特征**:
1. Sobel边缘密度 (1维)
2. Sobel X方向梯度均值 (1维)
3. Sobel Y方向梯度均值 (1维)
4. 边缘方向分布 (4维: 0°/45°/90°/135°)
5. 局部方差均值 (1维)
6. 局部方差标准差 (1维)
7. 纹理复杂度 (1维)
8. 高频成分比例 (1维)
9. 保留维度 (4维)

**实现方法**:
```rust
fn extract_texture_features(img: &DynamicImage) -> Vec<f64> {
    let gray = img.to_luma8();
    
    // Sobel边缘检测
    let sobel_x = apply_sobel_x(&gray);
    let sobel_y = apply_sobel_y(&gray);
    let edge_magnitude = calculate_edge_magnitude(&sobel_x, &sobel_y);
    
    // 边缘密度
    let edge_density = edge_magnitude.iter()
        .filter(|&&m| m > 30.0)
        .count() as f64 / edge_magnitude.len() as f64;
    
    // 边缘方向
    let edge_directions = calculate_edge_directions(&sobel_x, &sobel_y);
    
    // 局部方差（3x3窗口）
    let local_variance = calculate_local_variance(&gray, 3);
    
    vec![
        edge_density,
        sobel_x_mean,
        sobel_y_mean,
        dir_0, dir_45, dir_90, dir_135,
        local_var_mean,
        local_var_std,
        texture_complexity,
        high_freq_ratio,
        0.0, 0.0, 0.0, 0.0,  // 保留
    ]
}
```

#### 1.3 Quality特征 (16维)

**文件**: `pixly_kernel.rs` - `extract_quality_features()`

**特征列表**:
1. 噪声水平 (1维)
2. 清晰度 (1维)
3. 对比度 (1维)
4. 动态范围 (1维)
5. 压缩痕迹检测 (1维)
6. 块效应检测 (1维)
7. 振铃效应检测 (1维)
8. 色彩偏移 (1维)
9. 保留维度 (8维)

**实现方法**:
```rust
fn extract_quality_features(img: &DynamicImage) -> Vec<f64> {
    let gray = img.to_luma8();
    
    // 噪声估计（高频成分）
    let noise_level = estimate_noise(&gray);
    
    // 清晰度（拉普拉斯方差）
    let sharpness = calculate_laplacian_variance(&gray);
    
    // 对比度（标准差）
    let contrast = calculate_contrast(&gray);
    
    // 动态范围
    let dynamic_range = calculate_dynamic_range(&gray);
    
    // 压缩痕迹（DCT块检测）
    let compression_artifacts = detect_compression_artifacts(&gray);
    
    vec![
        noise_level,
        sharpness,
        contrast,
        dynamic_range,
        compression_artifacts,
        block_effect,
        ringing_effect,
        color_shift,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,  // 保留
    ]
}
```

#### 1.4 Metadata特征 (32维)

**文件**: `pixly_kernel.rs` - `extract_metadata_features()`

**特征列表**:
1. EXIF存在标志 (1维)
2. 相机制造商编码 (1维)
3. ISO值 (1维)
4. 曝光时间 (1维)
5. 光圈值 (1维)
6. 焦距 (1维)
7. 白平衡模式 (1维)
8. 闪光灯使用 (1维)
9. 拍摄日期时间戳 (1维)
10. GPS存在标志 (1维)
11. 保留维度 (22维)

**实现方法**:
```rust
fn extract_metadata_features(path: &Path) -> Vec<f64> {
    // 使用exif crate解析EXIF
    let exif_data = match read_exif(path) {
        Ok(data) => data,
        Err(_) => return vec![0.0; 32],
    };
    
    let mut features = vec![1.0];  // EXIF存在
    
    // 提取各种EXIF字段
    features.push(encode_camera_make(&exif_data));
    features.push(exif_data.iso.unwrap_or(0) as f64);
    features.push(exif_data.exposure_time.unwrap_or(0.0));
    // ...
    
    // 补齐到32维
    while features.len() < 32 {
        features.push(0.0);
    }
    
    features
}
```

---

### 🟡 Phase 2: 视频转换ML集成 (中优先级)

**目标**: 动图转视频使用Python ML预测参数

**预计时间**: 4-6小时

#### 2.1 视频特征提取

**文件**: `src/video_features.rs` (新建)

**特征列表**:
- 帧数、FPS、时长
- 分辨率、宽高比
- 码率、文件大小
- 编解码器类型
- 是否有音频
- 场景复杂度

**实现**:
```rust
pub struct VideoFeatures {
    pub frame_count: u32,
    pub fps: f64,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub bitrate: u64,
    pub file_size: u64,
    pub codec: String,
    pub has_audio: bool,
    pub scene_complexity: f64,
}

pub fn extract_video_features(path: &Path) -> Result<VideoFeatures> {
    // 使用ffprobe提取视频信息
    let output = Command::new("ffprobe")
        .args(&[
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            path.to_str().unwrap()
        ])
        .output()?;
    
    let info: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    
    // 解析JSON并构建VideoFeatures
    Ok(VideoFeatures {
        frame_count: extract_frame_count(&info),
        fps: extract_fps(&info),
        // ...
    })
}
```

#### 2.2 Python ML视频预测

**文件**: `scripts/ml_bridge.py` - 扩展

**新增**:
```python
def predict_video_params(features: Dict, target_codec: str, quality_mode: str) -> Dict:
    """
    视频参数预测
    
    返回:
    - codec: h264/h265/vp9/av1
    - crf: 质量参数
    - preset: 编码速度
    - two_pass: 是否使用两次编码
    """
    # 基于视频特征的智能预测
    if features['frame_count'] > 1000:
        # 长视频 → 两次编码
        two_pass = True
    
    if features['scene_complexity'] > 0.7:
        # 复杂场景 → 更高质量
        crf = 18
    else:
        crf = 23
    
    return {
        'codec': 'h265',  # 默认H.265
        'crf': crf,
        'preset': 'medium',
        'two_pass': two_pass,
        'confidence': 0.8
    }
```

#### 2.3 Rust集成

**文件**: `pixly_converter_cli.rs` - video命令

**修改**:
```rust
"video" => {
    if ai {
        println!("🤖 Calling Python ML for video parameter prediction...");
        
        // 提取视频特征
        let video_features = extract_video_features(&input)?;
        
        // 转换为128维向量（视频特定）
        let feature_vector = video_features_to_128d(&video_features);
        
        // 调用Python ML
        let ml_request = MLPredictRequest {
            features: feature_vector,
            target_format: "video".to_string(),
            quality_mode: "balanced".to_string(),
        };
        
        match call_python_ml(&ml_request) {
            Ok(ml_response) => {
                println!("✅ Python ML video prediction:");
                println!("   Codec: {}", ml_response.format_options.get("codec").unwrap_or(&"h265".to_string()));
                println!("   CRF: {}", ml_response.quality);
                println!("   Confidence: {:.2}", ml_response.confidence);
                
                // 使用ML预测的参数
                codec = Some(ml_response.format_options.get("codec").unwrap().clone());
                crf = Some(ml_response.quality as u32);
            }
            Err(e) => {
                eprintln!("⚠️ Python ML failed: {}, using defaults", e);
            }
        }
    }
    
    // 继续转换...
}
```

---

### 🟡 Phase 3: 模型训练 (中优先级)

**目标**: 训练真实的LightGBM和PPO模型

**预计时间**: 10-15小时

#### 3.1 数据收集

**文件**: `scripts/collect_training_data.py` (新建)

**功能**:
```python
def collect_conversion_data(data_dir: str, output_file: str):
    """
    收集真实转换数据
    
    流程:
    1. 扫描测试数据目录
    2. 对每个文件执行多种参数转换
    3. 记录输入特征、参数、输出结果
    4. 保存为训练数据集
    """
    training_samples = []
    
    for image_file in scan_images(data_dir):
        # 提取特征
        features = extract_features(image_file)
        
        # 尝试多种参数组合
        for quality in [60, 70, 80, 90]:
            for effort in [4, 6, 8]:
                # 执行转换
                result = convert_image(image_file, quality, effort)
                
                # 记录样本
                sample = {
                    'features': features,
                    'quality': quality,
                    'effort': effort,
                    'output_size': result.size,
                    'ssim': result.ssim,
                    'processing_time': result.time
                }
                training_samples.append(sample)
    
    # 保存数据集
    save_dataset(training_samples, output_file)
```

#### 3.2 LightGBM训练

**文件**: `scripts/train_lightgbm.py` (新建)

**功能**:
```python
import lightgbm as lgb
from sklearn.model_selection import train_test_split

def train_lightgbm_model(dataset_file: str, target_format: str):
    """
    训练LightGBM模型
    """
    # 加载数据
    data = load_dataset(dataset_file)
    X = np.array([s['features'] for s in data])
    y_quality = np.array([s['quality'] for s in data])
    y_effort = np.array([s['effort'] for s in data])
    
    # 分割训练/测试集
    X_train, X_test, y_train, y_test = train_test_split(
        X, y_quality, test_size=0.2, random_state=42
    )
    
    # 训练质量预测模型
    quality_model = lgb.LGBMRegressor(
        n_estimators=200,
        learning_rate=0.05,
        max_depth=7,
        num_leaves=31,
        min_child_samples=20
    )
    quality_model.fit(X_train, y_train)
    
    # 评估
    score = quality_model.score(X_test, y_test)
    print(f"Quality model R² score: {score:.4f}")
    
    # 保存模型
    quality_model.booster_.save_model(f'models/lightgbm_{target_format}_quality.txt')
    
    # 同样训练effort模型
    # ...
```

#### 3.3 PPO训练

**文件**: `scripts/train_ppo.py` (新建)

**功能**:
```python
import torch
import torch.nn as nn
from torch.distributions import Normal

class ActorNetwork(nn.Module):
    """PPO Actor网络"""
    def __init__(self, state_dim=128, action_dim=2):
        super().__init__()
        self.fc = nn.Sequential(
            nn.Linear(state_dim, 256),
            nn.ReLU(),
            nn.Linear(256, 128),
            nn.ReLU(),
            nn.Linear(128, action_dim)
        )
    
    def forward(self, state):
        return self.fc(state)

def train_ppo_model(dataset_file: str, epochs: int = 100):
    """
    训练PPO模型
    """
    # 加载数据
    data = load_dataset(dataset_file)
    
    # 初始化网络
    actor = ActorNetwork()
    critic = CriticNetwork()
    
    # PPO训练循环
    for epoch in range(epochs):
        # 收集轨迹
        trajectories = collect_trajectories(actor, data)
        
        # 计算优势函数
        advantages = calculate_advantages(trajectories, critic)
        
        # 更新策略
        update_policy(actor, critic, trajectories, advantages)
        
        if epoch % 10 == 0:
            print(f"Epoch {epoch}: avg_reward={avg_reward:.4f}")
    
    # 保存模型
    torch.save(actor.state_dict(), 'models/ppo/actor_network.pth')
    torch.save(critic.state_dict(), 'models/ppo/critic_network.pth')
```

---

## 📊 实施时间表

| Phase | 任务 | 预计时间 | 优先级 |
|-------|------|---------|--------|
| **1.1** | Color特征提取 | 2-3h | 🔴 |
| **1.2** | Texture特征提取 | 3-4h | 🔴 |
| **1.3** | Quality特征提取 | 2-3h | 🔴 |
| **1.4** | Metadata特征提取 | 1-2h | 🔴 |
| **2.1** | 视频特征提取 | 2h | 🟡 |
| **2.2** | Python视频预测 | 1h | 🟡 |
| **2.3** | Rust视频集成 | 1-2h | 🟡 |
| **3.1** | 数据收集脚本 | 3-4h | 🟡 |
| **3.2** | LightGBM训练 | 3-4h | 🟡 |
| **3.3** | PPO训练 | 4-6h | 🟡 |
| **总计** | | **22-33h** | |

---

## 🎯 成功标准

### Phase 1完成标准
- [ ] 128维特征全部为真实提取（无占位符）
- [ ] Color特征包含RGB/HSV统计
- [ ] Texture特征包含Sobel边缘检测
- [ ] Quality特征包含噪声/清晰度检测
- [ ] 编译成功，无警告

### Phase 2完成标准
- [ ] 视频特征提取正常工作
- [ ] Python ML可以预测视频参数
- [ ] 动图转视频使用ML预测
- [ ] 日志清晰显示ML预测结果

### Phase 3完成标准
- [ ] 收集至少1000个训练样本
- [ ] LightGBM模型R²>0.8
- [ ] PPO模型收敛
- [ ] 模型文件正确保存
- [ ] Python ML自动加载模型

---

## 🚀 立即开始

**第一步**: Phase 1.1 - Color特征提取  
**第二步**: Phase 1.2 - Texture特征提取  
**第三步**: Phase 1.3 - Quality特征提取

准备好了吗？我们从Color特征开始！
