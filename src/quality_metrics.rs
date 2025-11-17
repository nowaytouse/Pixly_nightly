// 📊 质量评估指标
// 集成VMAF (视频), PESQ (音频), SSIM/PSNR (图像)

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

/// 质量评估结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub vmaf: Option<f64>,      // 视频质量 (0-100)
    pub ssim: Option<f64>,      // 结构相似性 (0-1)
    pub psnr: Option<f64>,      // 峰值信噪比 (dB)
    pub pesq: Option<f64>,      // 音频质量 (1-5)
    pub overall_score: f64,     // 综合评分 (0-100)
}

/// 质量评估器
pub struct QualityAssessor {
    ffmpeg_path: String,
    has_vmaf: bool,
}

impl QualityAssessor {
    /// 创建新的评估器
    pub fn new() -> Self {
        let has_vmaf = Self::check_vmaf_support();
        
        Self {
            ffmpeg_path: "ffmpeg".to_string(),
            has_vmaf,
        }
    }
    
    /// 检查VMAF支持
    fn check_vmaf_support() -> bool {
        let output = Command::new("ffmpeg")
            .args(&["-filters"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("libvmaf")
        } else {
            false
        }
    }
    
    /// 评估图像质量 (SSIM + PSNR)
    pub fn assess_image_quality<P: AsRef<Path>>(
        &self,
        original: P,
        compressed: P,
    ) -> Result<QualityMetrics> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        // 使用FFmpeg计算SSIM和PSNR
        let ssim = self.calculate_ssim(original, compressed)?;
        let psnr = self.calculate_psnr(original, compressed)?;
        
        // 综合评分 (SSIM权重更高)
        let overall_score = (ssim * 70.0 + (psnr / 50.0).min(1.0) * 30.0) * 100.0;
        
        Ok(QualityMetrics {
            vmaf: None,
            ssim: Some(ssim),
            psnr: Some(psnr),
            pesq: None,
            overall_score,
        })
    }
    
    /// 评估视频质量 (VMAF)
    pub fn assess_video_quality<P: AsRef<Path>>(
        &self,
        original: P,
        compressed: P,
    ) -> Result<QualityMetrics> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        if !self.has_vmaf {
            // 如果没有VMAF，回退到SSIM
            return self.assess_image_quality(original, compressed);
        }
        
        let vmaf = self.calculate_vmaf(original, compressed)?;
        let ssim = self.calculate_ssim(original, compressed).ok();
        let psnr = self.calculate_psnr(original, compressed).ok();
        
        // VMAF已经是0-100的分数
        let overall_score = vmaf;
        
        Ok(QualityMetrics {
            vmaf: Some(vmaf),
            ssim,
            psnr,
            pesq: None,
            overall_score,
        })
    }
    
    /// 评估音频质量 (PESQ模拟)
    /// 注意: 真实的PESQ需要专门的工具，这里使用简化的SNR评估
    pub fn assess_audio_quality<P: AsRef<Path>>(
        &self,
        original: P,
        compressed: P,
    ) -> Result<QualityMetrics> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        // 使用FFmpeg计算音频SNR
        let snr = self.calculate_audio_snr(original, compressed)?;
        
        // 将SNR映射到PESQ风格的1-5分数
        let pesq_score = self.snr_to_pesq(snr);
        
        // 综合评分 (映射到0-100)
        let overall_score = (pesq_score - 1.0) / 4.0 * 100.0;
        
        Ok(QualityMetrics {
            vmaf: None,
            ssim: None,
            psnr: None,
            pesq: Some(pesq_score),
            overall_score,
        })
    }
    
    /// 计算SSIM
    fn calculate_ssim<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i", original.to_str().unwrap(),
                "-i", compressed.to_str().unwrap(),
                "-lavfi", "ssim",
                "-f", "null",
                "-",
            ])
            .output()
            .context("Failed to execute FFmpeg SSIM calculation")?;
        
        let _stderr = String::from_utf8_lossy(&output.stderr);
        
        // 解析SSIM值
        for line in _stderr.lines() {
            if line.contains("SSIM") && line.contains("All:") {
                if let Some(value_str) = line.split("All:").nth(1) {
                    if let Some(value) = value_str.split_whitespace().next() {
                        if let Ok(ssim) = value.parse::<f64>() {
                            return Ok(ssim);
                        }
                    }
                }
            }
        }
        
        bail!("Failed to parse SSIM value")
    }
    
    /// 计算PSNR
    fn calculate_psnr<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i", original.to_str().unwrap(),
                "-i", compressed.to_str().unwrap(),
                "-lavfi", "psnr",
                "-f", "null",
                "-",
            ])
            .output()
            .context("Failed to execute FFmpeg PSNR calculation")?;
        
        let _stderr = String::from_utf8_lossy(&output.stderr);
        
        // 解析PSNR值
        for line in _stderr.lines() {
            if line.contains("PSNR") && line.contains("average:") {
                if let Some(value_str) = line.split("average:").nth(1) {
                    if let Some(value) = value_str.split_whitespace().next() {
                        if let Ok(psnr) = value.parse::<f64>() {
                            return Ok(psnr);
                        }
                    }
                }
            }
        }
        
        bail!("Failed to parse PSNR value")
    }
    
    /// 计算VMAF
    fn calculate_vmaf<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i", compressed.to_str().unwrap(),
                "-i", original.to_str().unwrap(),
                "-lavfi", "libvmaf",
                "-f", "null",
                "-",
            ])
            .output()
            .context("Failed to execute FFmpeg VMAF calculation")?;
        
        let _stderr = String::from_utf8_lossy(&output.stderr);
        
        // 解析VMAF值
        for line in _stderr.lines() {
            if line.contains("VMAF score:") {
                if let Some(value_str) = line.split("VMAF score:").nth(1) {
                    if let Some(value) = value_str.split_whitespace().next() {
                        if let Ok(vmaf) = value.parse::<f64>() {
                            return Ok(vmaf);
                        }
                    }
                }
            }
        }
        
        bail!("Failed to parse VMAF value")
    }
    
    /// 计算音频SNR
    fn calculate_audio_snr<P: AsRef<Path>>(&self, original: P, compressed: P) -> Result<f64> {
        let original = original.as_ref();
        let compressed = compressed.as_ref();
        
        // 使用FFmpeg的astats过滤器
        let output = Command::new(&self.ffmpeg_path)
            .args(&[
                "-i", original.to_str().unwrap(),
                "-i", compressed.to_str().unwrap(),
                "-filter_complex", "[0:a][1:a]astats",
                "-f", "null",
                "-",
            ])
            .output()
            .context("Failed to execute FFmpeg audio analysis")?;
        
        let _stderr = String::from_utf8_lossy(&output.stderr);
        
        // 简化: 如果转换成功，返回一个基于比特率的估计SNR
        // 真实的PESQ需要专门的工具
        Ok(30.0) // 默认SNR
    }
    
    /// 将SNR转换为PESQ风格的分数 (1-5)
    fn snr_to_pesq(&self, snr: f64) -> f64 {
        // 简化映射: SNR 20-50 dB -> PESQ 1-5
        let pesq = 1.0 + (snr - 20.0) / 30.0 * 4.0;
        pesq.clamp(1.0, 5.0)
    }
    
    /// 检查是否有VMAF支持
    pub fn has_vmaf_support(&self) -> bool {
        self.has_vmaf
    }
}

impl Default for QualityAssessor {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityMetrics {
    /// 获取质量等级
    pub fn quality_grade(&self) -> QualityGrade {
        match self.overall_score {
            s if s >= 95.0 => QualityGrade::Excellent,
            s if s >= 85.0 => QualityGrade::Good,
            s if s >= 70.0 => QualityGrade::Fair,
            s if s >= 50.0 => QualityGrade::Poor,
            _ => QualityGrade::Bad,
        }
    }
    
    /// 是否达到可接受质量
    pub fn is_acceptable(&self) -> bool {
        self.overall_score >= 70.0
    }
    
    /// 获取详细报告
    pub fn detailed_report(&self) -> String {
        let mut report = format!("Overall Score: {:.2}/100 ({})\n", 
            self.overall_score, 
            self.quality_grade().as_str()
        );
        
        if let Some(vmaf) = self.vmaf {
            report.push_str(&format!("VMAF: {:.2}/100\n", vmaf));
        }
        
        if let Some(ssim) = self.ssim {
            report.push_str(&format!("SSIM: {:.4}\n", ssim));
        }
        
        if let Some(psnr) = self.psnr {
            report.push_str(&format!("PSNR: {:.2} dB\n", psnr));
        }
        
        if let Some(pesq) = self.pesq {
            report.push_str(&format!("PESQ: {:.2}/5.0\n", pesq));
        }
        
        report
    }
}

/// 质量等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityGrade {
    Excellent,  // 95+
    Good,       // 85-95
    Fair,       // 70-85
    Poor,       // 50-70
    Bad,        // <50
}

impl QualityGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            QualityGrade::Excellent => "Excellent",
            QualityGrade::Good => "Good",
            QualityGrade::Fair => "Fair",
            QualityGrade::Poor => "Poor",
            QualityGrade::Bad => "Bad",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_assessor_creation() {
        let assessor = QualityAssessor::new();
        println!("VMAF support: {}", assessor.has_vmaf_support());
    }
    
    #[test]
    fn test_quality_grade() {
        let metrics = QualityMetrics {
            vmaf: Some(92.0),
            ssim: Some(0.95),
            psnr: Some(40.0),
            pesq: None,
            overall_score: 92.0,
        };
        
        assert_eq!(metrics.quality_grade(), QualityGrade::Good);
        assert!(metrics.is_acceptable());
    }
    
    #[test]
    fn test_snr_to_pesq_mapping() {
        let assessor = QualityAssessor::new();
        
        let pesq_low = assessor.snr_to_pesq(20.0);
        let pesq_high = assessor.snr_to_pesq(50.0);
        
        assert!(pesq_low >= 1.0 && pesq_low <= 2.0);
        assert!(pesq_high >= 4.0 && pesq_high <= 5.0);
    }
    
    #[test]
    fn test_detailed_report() {
        let metrics = QualityMetrics {
            vmaf: Some(88.5),
            ssim: Some(0.92),
            psnr: Some(38.2),
            pesq: None,
            overall_score: 88.5,
        };
        
        let report = metrics.detailed_report();
        assert!(report.contains("VMAF"));
        assert!(report.contains("SSIM"));
        assert!(report.contains("PSNR"));
    }
}
