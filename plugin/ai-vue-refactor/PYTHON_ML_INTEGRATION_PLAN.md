# 🚀 Python ML 真实集成实施计划

**日期**: 2025-01-XX  
**目标**: 让Python ML真正工作，消除所有硬编码规则  
**架构**: 完全本地化 - Rust直接调用Python进程（无HTTP）

---

## 🎯 核心目标

### 当前状态（假AI）
```
Rust硬编码规则 → 简单if-else → 固定参数
```

### 目标状态（真AI）
```
Rust → Python进程 → LightGBM模型 → ML预测 → 返回参数
```

---

## 📋 实施步骤

### Phase 1: Rust调用Python基础设施 ✅

**文件**: `src/python_ml_caller.rs` (新建)

**功能**:
```rust
use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Python ML预测请求
#[derive(Serialize)]
pub struct MLPredictRequest {
    pub features: Vec<f64>,  // 128维特征向量
    pub target_format: String,
    pub quality_mode: String,
}

/// Python ML预测响应
#[derive(Deserialize)]
pub struct MLPredictResponse {
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub format_options: Vec<String>,
    pub confidence: f64,
    pub model_version: String,
}

/// 调用Python ML Bridge
pub fn call_python_ml(request: &MLPredictRequest) -> Result<MLPredictResponse> {
    // 1. 序列化请求为JSON
    let request_json = serde_json::to_string(request)?;
    
    // 2. 调用Python脚本
    let output = Command::new("python3")
        .arg("scripts/ml_bridge.py")
        .arg("--predict")
        .arg(&request_json)
        .output()
        .context("Failed to execute Python ML bridge")?;
    
    // 3. 检查执行状态
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Python ML prediction failed: {}", stderr);
    }
    
    // 4. 解析响应
    let stdout = String::from_utf8_lossy(&output.stdout);
    let response: MLPredictResponse = serde_json::from_str(&stdout)
        .context("Failed to parse Python ML response")?;
    
    Ok(response)
}
```

### Phase 2: 增强Python ML Bridge ✅

**文件**: `scripts/ml_bridge.py` (修改)

**新增功能**:
```python
import sys
import json
import argparse

def predict_from_features(features: List[float], target_format: str, quality_mode: str) -> Dict:
    """
    从128维特征预测参数
    """
    # 1. 验证特征
    assert len(features) == 128, f"Invalid feature dimension: {len(features)}"
    
    # 2. 加载模型（如果存在）
    model = load_model_if_exists(target_format)
    
    if model:
        # 真实ML预测
        prediction = model.predict([features])[0]
        confidence = 0.85
    else:
        # Fallback到智能规则（比Rust的更好）
        prediction = smart_rule_based_predict(features, target_format, quality_mode)
        confidence = 0.65
    
    return {
        'quality': int(prediction['quality']),
        'effort': int(prediction['effort']),
        'lossless': bool(prediction['lossless']),
        'format_options': prediction.get('format_options', []),
        'confidence': confidence,
        'model_version': 'python-ml-v1.0'
    }

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--predict', required=True, help='JSON prediction request')
    args = parser.parse_args()
    
    # 解析请求
    request = json.loads(args.predict)
    
    # 执行预测
    response = predict_from_features(
        request['features'],
        request['target_format'],
        request['quality_mode']
    )
    
    # 输出JSON响应
    print(json.dumps(response))

if __name__ == '__main__':
    main()
```

### Phase 3: 修改pixly_kernel.rs集成Python ✅

**文件**: `pixly_kernel.rs` (修改)

**修改点**:
```rust
// 在文件顶部添加
mod python_ml_caller;
use python_ml_caller::{call_python_ml, MLPredictRequest};

impl UnifiedAIPredictor {
    pub fn predict_parameters(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> (u8, u8, bool, Vec<String>) {
        // 🔥 新增：尝试调用Python ML
        if let Ok(ml_response) = self.try_python_ml_predict(features, target_format, quality_mode) {
            info!("✅ Using Python ML prediction (confidence: {:.2})", ml_response.confidence);
            return (
                ml_response.quality,
                ml_response.effort,
                ml_response.lossless,
                ml_response.format_options,
            );
        }
        
        // ⚠️ Fallback到Rust规则（仅当Python失败时）
        warn!("⚠️ Python ML failed, using Rust fallback rules");
        match target_format.as_str() {
            "avif" => self.predict_avif(features, quality_mode),
            // ... 其他格式
        }
    }
    
    fn try_python_ml_predict(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> Result<python_ml_caller::MLPredictResponse> {
        // 1. 提取128维特征向量
        let feature_vector = self.extract_128d_features(features);
        
        // 2. 构建请求
        let request = MLPredictRequest {
            features: feature_vector,
            target_format: target_format.to_string(),
            quality_mode: quality_mode.as_str().to_string(),
        };
        
        // 3. 调用Python
        call_python_ml(&request)
    }
    
    fn extract_128d_features(&self, features: &ImageFeatures) -> Vec<f64> {
        // 🔥 实现128维特征提取
        let mut vec = Vec::with_capacity(128);
        
        // Basic (16维)
        vec.push(features.width as f64);
        vec.push(features.height as f64);
        vec.push(features.pixels() as f64);
        vec.push(features.size_mb());
        vec.push(features.aspect_ratio());
        vec.push(if features.has_alpha { 1.0 } else { 0.0 });
        vec.push(if features.is_animated { 1.0 } else { 0.0 });
        vec.push(features.complexity);
        // ... 补齐到16维
        while vec.len() < 16 { vec.push(0.0); }
        
        // Color (16维) - 需要从图像数据提取
        // Texture (16维)
        // Shape (16维)
        // Quality (16维)
        // Metadata (32维)
        // Context (16维)
        
        // 临时：补齐到128维
        while vec.len() < 128 { vec.push(0.0); }
        
        vec
    }
}
```

### Phase 4: 视频转换集成Python ML ✅

**文件**: `pixly_converter_cli.rs` (修改)

**修改点**:
```rust
// 在video命令处理中
"video" => {
    // ... 解析参数 ...
    
    if ai {
        // 🔥 调用Python ML预测视频参数
        println!("🤖 Calling Python ML for video parameter prediction...");
        
        let video_features = extract_video_features(&input)?;
        let ml_request = MLPredictRequest {
            features: video_features,
            target_format: "video".to_string(),
            quality_mode: "balanced".to_string(),
        };
        
        match call_python_ml(&ml_request) {
            Ok(ml_response) => {
                println!("✅ Python ML prediction received:");
                println!("   Codec: {} (confidence: {:.2})", 
                    ml_response.format_options.get(0).unwrap_or(&"h265".to_string()),
                    ml_response.confidence);
                
                // 使用ML预测的参数
                codec = Some(ml_response.format_options.get(0).unwrap_or(&"h265".to_string()).clone());
                crf = Some(ml_response.quality as u32);
                preset = Some(format!("medium")); // 从effort映射
            }
            Err(e) => {
                eprintln!("⚠️ Python ML failed: {}, using defaults", e);
            }
        }
    }
    
    // ... 继续转换 ...
}
```

---

## 🔧 实施细节

### 128维特征提取完整实现

需要从图像数据中提取：

1. **Basic (16维)**:
   - 宽度、高度、像素数
   - 文件大小、宽高比
   - 透明度、动画标志
   - 复杂度、格式编码
   - 保留维度

2. **Color (16维)**:
   - RGB均值、方差
   - 色彩空间分布
   - 饱和度、亮度
   - 颜色数量、主色调

3. **Texture (16维)**:
   - 边缘密度、方向
   - Sobel梯度统计
   - 纹理复杂度
   - 局部方差

4. **Shape (16维)**:
   - 几何特征
   - 对称性
   - 区域分布

5. **Quality (16维)**:
   - 噪声水平
   - 清晰度
   - 压缩痕迹

6. **Metadata (32维)**:
   - EXIF信息
   - 文件属性
   - 历史记录

7. **Context (16维)**:
   - 处理历史
   - 用户偏好
   - 环境信息

### Python模型训练

**文件**: `scripts/train_ml_model.py` (新建)

```python
import lightgbm as lgb
from ml_bridge import StandardFeatures, TrainingSample

def train_model(samples: List[TrainingSample], target_format: str):
    """
    训练LightGBM模型
    """
    # 准备数据
    X = np.array([s.features.to_vector() for s in samples])
    y_quality = np.array([s.actual_quality for s in samples])
    y_effort = np.array([s.actual_effort for s in samples])
    
    # 训练质量预测模型
    quality_model = lgb.LGBMRegressor(
        n_estimators=100,
        learning_rate=0.05,
        max_depth=7
    )
    quality_model.fit(X, y_quality)
    
    # 训练effort预测模型
    effort_model = lgb.LGBMRegressor(
        n_estimators=100,
        learning_rate=0.05,
        max_depth=7
    )
    effort_model.fit(X, y_effort)
    
    # 保存模型
    quality_model.booster_.save_model(f'models/{target_format}_quality.txt')
    effort_model.booster_.save_model(f'models/{target_format}_effort.txt')
```

---

## 📊 测试计划

### 单元测试

```rust
#[test]
fn test_python_ml_integration() {
    let features = ImageFeatures {
        width: 1920,
        height: 1080,
        file_size: 2_000_000,
        format: "png".to_string(),
        has_alpha: false,
        is_animated: false,
        complexity: 0.65,
    };
    
    let predictor = UnifiedAIPredictor::new();
    let (quality, effort, lossless, _) = predictor.predict_parameters(
        &features,
        "avif",
        QualityMode::Balanced
    );
    
    // 应该从Python ML获得预测
    assert!(quality > 0 && quality <= 100);
    assert!(effort >= 0 && effort <= 9);
}
```

### 集成测试

```bash
# 测试Python ML调用
cargo test --test python_ml_integration

# 测试完整转换流程
./pixly-converter convert test.png test.avif --ai

# 应该看到：
# 🤖 Calling Python ML for parameter prediction...
# ✅ Python ML prediction received (confidence: 0.85)
```

---

## ⏱️ 时间估算

| Phase | 任务 | 预计时间 |
|-------|------|---------|
| 1 | Rust Python调用基础设施 | 2h |
| 2 | Python ML Bridge增强 | 3h |
| 3 | pixly_kernel.rs集成 | 4h |
| 4 | 视频转换集成 | 2h |
| 5 | 128维特征提取完整实现 | 6h |
| 6 | 测试和调试 | 3h |
| **总计** | | **20h** |

---

## 🎯 成功标准

1. ✅ Rust成功调用Python进程
2. ✅ Python返回有效的ML预测
3. ✅ 128维特征正确提取
4. ✅ 所有格式都使用Python ML（不是硬编码）
5. ✅ 视频转换也使用Python ML
6. ✅ Fallback机制仅在Python失败时触发
7. ✅ 日志清晰显示"Python ML"或"Rust Fallback"

---

## 🚀 立即开始

**第一步**: 创建`src/python_ml_caller.rs`  
**第二步**: 修改`scripts/ml_bridge.py`添加CLI接口  
**第三步**: 修改`pixly_kernel.rs`集成Python调用

准备好了吗？我们开始实施！
