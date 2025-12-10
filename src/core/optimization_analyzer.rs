// Optimization analyzer - 媒体优化状态分析器

use std::path::Path;
use std::process::Command;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

/// 优化状态等级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum OptimizationStatus {
    AlreadyOptimized {
        current_size: u64,
        predicted_size: u64,
        savings_percent: f64,
        reason: String,
    },
    MinorImprovement {
        current_size: u64,
        predicted_size: u64,
        savings_percent: f64,
        recommended_format: String,
    },
    SignificantImprovement {
        current_size: u64,
        predicted_size: u64,
        savings_percent: f64,
        recommended_format: String,
        estimated_time: f64,
    },
    CriticalImprovement {
        current_size: u64,
        predicted_size: u64,
        savings_percent: f64,
        recommended_format: String,
        estimated_time: f64,
    },
}

/// 文件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Image,
    AnimatedImage,
    Video,
}

/// 优化报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationReport {
    pub status: OptimizationStatus,
    pub can_skip: bool,
    pub confidence: f64,
    pub method: String,
}

/// Python 预测响应
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PythonPrediction {
    current_size: u64,
    predicted_size: u64,
    savings_percent: f64,
    current_format: String,
    recommended_format: String,
    confidence: f64,
    method: String,
}

/// 主分析函数
pub fn analyze_file(path: &Path, use_precise: bool, format_hint: Option<&str>) -> Result<OptimizationReport> {
    // 检测文件类型和格式
    let current_format = detect_format(path, format_hint)?;
    let file_type = detect_file_type(&current_format)?;
    let target_format = select_optimal_format(&current_format);
    
    // 调用 Python 预测器
    let prediction = call_python_predictor(
        path,
        &file_type,
        &current_format,
        &target_format,
        use_precise,
    )?;
    
    // 根据节省百分比分类
    let status = classify_by_savings(
        prediction.current_size,
        prediction.predicted_size,
        &target_format,
    );
    
    // 判断是否可跳过
    let can_skip = matches!(status, OptimizationStatus::AlreadyOptimized { .. });
    
    Ok(OptimizationReport {
        status,
        can_skip,
        confidence: prediction.confidence,
        method: prediction.method,
    })
}

/// 调用 Python 预测器
fn call_python_predictor(
    path: &Path,
    file_type: &FileType,
    current_fmt: &str,
    target_fmt: &str,
    use_precise: bool,
) -> Result<PythonPrediction> {
    let file_type_str = match file_type {
        FileType::Image => "image",
        FileType::AnimatedImage => "animated",
        FileType::Video => "video",
    };
    
    let mut cmd = Command::new("python3");
    cmd.arg("src/ai/optimization_predictor.py")
        .arg(path.to_str().context("Invalid path")?)
        .arg(file_type_str)
        .arg(current_fmt)
        .arg(target_fmt);
    
    if use_precise {
        cmd.arg("--precise");
    }
    
    let output = cmd.output()
        .context("Failed to execute Python predictor")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Python prediction failed: {}", stderr);
    }
    
    let prediction: PythonPrediction = serde_json::from_slice(&output.stdout)
        .context("Failed to parse Python response")?;
    
    Ok(prediction)
}

/// 检测文件类型
fn detect_file_type(format: &str) -> Result<FileType> {
    Ok(match format {
        "mp4" | "mov" | "avi" | "mkv" | "webm" => FileType::Video,
        "gif" => FileType::AnimatedImage,
        _ => FileType::Image,
    })
}

/// 检测格式
fn detect_format(path: &Path, hint: Option<&str>) -> Result<String> {
    if let Some(format) = hint {
        return Ok(format.to_lowercase());
    }

    let ext = path.extension()
        .and_then(|e| e.to_str())
        .context("No file extension and no format hint provided")?
        .to_lowercase();
    
    Ok(ext)
}

/// 选择最优格式
fn select_optimal_format(current: &str) -> String {
    match current {
        "png" | "jpeg" | "jpg" => "jxl",
        "gif" | "apng" => "webp",
        "webp" if is_old_webp() => "jxl",
        "h264" | "mpeg4" => "h266",  // 激进：直接H.266
        "h265" => "h266",
        "vp9" => "h265",
        _ => "jxl",
    }.to_string()
}

fn is_old_webp() -> bool {
    // TODO: 检测WebP是否为旧版本
    false
}

/// 根据节省百分比分类
fn classify_by_savings(
    current_size: u64,
    predicted_size: u64,
    recommended_format: &str,
) -> OptimizationStatus {
    let savings_bytes = current_size.saturating_sub(predicted_size);
    let savings_percent = if current_size > 0 {
        (savings_bytes as f64 / current_size as f64) * 100.0
    } else {
        0.0
    };
    
    // 估算转换时间
    let estimated_time = (current_size as f64 / 1_000_000.0) * 0.5;
    
    match savings_percent {
        s if s < 5.0 => OptimizationStatus::AlreadyOptimized {
            current_size,
            predicted_size,
            savings_percent: s,
            reason: format!(
                "文件已接近最优，重新编码仅可节省{:.1}%",
                s
            ),
        },
        
        s if s < 15.0 => OptimizationStatus::MinorImprovement {
            current_size,
            predicted_size,
            savings_percent: s,
            recommended_format: recommended_format.to_string(),
        },
        
        s if s < 40.0 => OptimizationStatus::SignificantImprovement {
            current_size,
            predicted_size,
            savings_percent: s,
            recommended_format: recommended_format.to_string(),
            estimated_time,
        },
        
        s => OptimizationStatus::CriticalImprovement {
            current_size,
            predicted_size,
            savings_percent: s,
            recommended_format: recommended_format.to_string(),
            estimated_time,
        },
    }
}
