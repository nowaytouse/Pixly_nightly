/**
 * GIF Optimizer - 动画 GIF 优化模块
 * 
 * 🔥 Phase 40.24.2: 动画 GIF 处理优化
 * 
 * 提供帧优化、色彩优化和智能压缩功能。
 * 
 * @module gif_optimizer
 */
// 🔧 统一日志系统
use tracing::{info, warn, debug};

use anyhow::{Result, Context, bail};
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

/// GIF 优化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GifOptimizationConfig {
    /// 色彩优化级别 (1-3)
    pub color_optimization: u8,
    
    /// 帧优化
    pub frame_optimization: FrameOptimization,
    
    /// 是否启用有损压缩
    pub lossy_compression: bool,
    
    /// 有损压缩质量 (0-200, 默认80)
    pub lossy_quality: u8,
    
    /// 是否移除元数据
    pub strip_metadata: bool,
    
    /// 目标帧率限制 (None=不限制)
    pub max_fps: Option<u8>,
    
    /// 目标宽度限制 (None=不限制)
    pub max_width: Option<u32>,
}

impl Default for GifOptimizationConfig {
    fn default() -> Self {
        Self {
            color_optimization: 2,
            frame_optimization: FrameOptimization::Balanced,
            lossy_compression: false,
            lossy_quality: 80,
            strip_metadata: true,
            max_fps: None,
            max_width: None,
        }
    }
}

/// 帧优化级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameOptimization {
    /// 不优化帧
    None,
    /// 基础优化（重复帧检测）
    Basic,
    /// 平衡优化（重复帧+区域更新）
    Balanced,
    /// 激进优化（所有优化+帧采样）
    Aggressive,
}

impl FrameOptimization {
    /// 获取 gifsicle 优化级别
    pub fn gifsicle_level(&self) -> &str {
        match self {
            FrameOptimization::None => "O1",
            FrameOptimization::Basic => "O2",
            FrameOptimization::Balanced => "O3",
            FrameOptimization::Aggressive => "O3",
        }
    }
}

/// GIF 优化策略
pub struct GifOptimizer {
    config: GifOptimizationConfig,
}

impl GifOptimizer {
    /// 创建新的优化器
    pub fn new(config: GifOptimizationConfig) -> Self {
        Self { config }
    }
    
    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(GifOptimizationConfig::default())
    }
    
    /// 为 Web 优化的配置
    pub fn for_web() -> Self {
        Self::new(GifOptimizationConfig {
            color_optimization: 3,
            frame_optimization: FrameOptimization::Aggressive,
            lossy_compression: true,
            lossy_quality: 80,
            strip_metadata: true,
            max_fps: Some(30),
            max_width: Some(800),
        })
    }
    
    /// 为质量优化的配置
    pub fn for_quality() -> Self {
        Self::new(GifOptimizationConfig {
            color_optimization: 1,
            frame_optimization: FrameOptimization::Basic,
            lossy_compression: false,
            lossy_quality: 100,
            strip_metadata: false,
            max_fps: None,
            max_width: None,
        })
    }
    
    /// 检查工具是否可用
    fn check_tool(tool: &str) -> bool {
        Command::new(tool)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// 优化 GIF 文件（多阶段优化）
    /// 
    /// 🔥 Phase 46.14+: F-001 多阶段迭代优化
    /// 使用多轮gifsicle优化，直到无法进一步压缩：
    /// 1. ffmpeg - 帧率和尺寸调整（预处理）
    /// 2. gifsicle Pass 1 - 无损优化（O3）
    /// 3. gifsicle Pass 2 - 色彩优化
    /// 4. gifsicle Pass 3 - 有损压缩（如果启用）
    /// 5. gifsicle Pass N - 迭代优化直到收益<1%
    /// 
    pub fn optimize(&self, input: &Path, output: &Path) -> Result<OptimizationResult> {
        let start = std::time::Instant::now();
        let original_size = std::fs::metadata(input)?.len();
        
        info!("🎨 Starting multi-stage GIF optimization: {:?}", input.file_name());
        info!("   Original size: {} bytes ({:.2} MB)", original_size, original_size as f64 / 1_048_576.0);
        
        // 创建临时文件
        let temp_dir = std::env::temp_dir();
        let temp_input = temp_dir.join(format!("pixly_gif_input_{}.gif", std::process::id()));
        let temp_output = temp_dir.join(format!("pixly_gif_output_{}.gif", std::process::id()));
        
        // 步骤1: 尺寸和帧率调整 (预处理)
        let current = if self.config.max_width.is_some() || self.config.max_fps.is_some() {
            info!("📐 Stage 1: Resizing and FPS adjustment");
            self.resize_and_adjust_fps(input, &temp_input)?;
            &temp_input
        } else {
            input
        };
        
        // 步骤2-N: 多阶段gifsicle优化
        if Self::check_tool("gifsicle") {
            let final_size = self.multi_pass_gifsicle_optimization(current, output)?;
            
            // 清理临时文件
            let _ = std::fs::remove_file(&temp_input);
            let _ = std::fs::remove_file(&temp_output);
            
            let elapsed = start.elapsed();
            let reduction = if final_size < original_size {
                ((original_size - final_size) as f64 / original_size as f64) * 100.0
            } else {
                0.0
            };
            
            info!("✅ Multi-stage optimization complete!");
            info!("   Final size: {} bytes ({:.2} MB)", final_size, final_size as f64 / 1_048_576.0);
            info!("   Reduction: {:.1}%", reduction);
            info!("   Time: {:.2}s", elapsed.as_secs_f64());
            
            Ok(OptimizationResult {
                original_size,
                optimized_size: final_size,
                reduction_percent: reduction,
                elapsed_ms: elapsed.as_millis() as u64,
            })
        } else {
            warn!("⚠️  gifsicle not found, copying without optimization");
            std::fs::copy(current, output)?;
            let _ = std::fs::remove_file(&temp_input);
            
            Ok(OptimizationResult {
                original_size,
                optimized_size: original_size,
                reduction_percent: 0.0,
                elapsed_ms: start.elapsed().as_millis() as u64,
            })
        }
    }
    
    /// 多轮gifsicle优化（F-001核心实现）
    /// 
    /// 迭代优化直到收益<1%或达到最大轮数
    fn multi_pass_gifsicle_optimization(&self, input: &Path, output: &Path) -> Result<u64> {
        let temp_dir = std::env::temp_dir();
        let pass_temp = temp_dir.join(format!("pixly_gif_pass_{}.gif", std::process::id()));
        
        let mut current_size = std::fs::metadata(input)?.len();
        let initial_size = current_size;
        
        info!("🔄 Starting multi-pass gifsicle optimization");
        
        // Pass 1: 无损优化（O3）
        info!("   Pass 1: Lossless optimization (O3)");
        self.gifsicle_pass(input, &pass_temp, &["-O3", "--no-comments", "--no-names"])?;
        let pass1_size = std::fs::metadata(&pass_temp)?.len();
        let pass1_reduction = ((current_size - pass1_size) as f64 / current_size as f64) * 100.0;
        info!("   → Size: {} bytes ({:.1}% reduction)", pass1_size, pass1_reduction);
        std::fs::copy(&pass_temp, output)?;
        current_size = pass1_size;
        
        // Pass 2: 色彩优化
        if self.config.color_optimization > 1 {
            info!("   Pass 2: Color optimization");
            let colors = 256u32 / self.config.color_optimization as u32;
            self.gifsicle_pass(output, &pass_temp, &["-O3", "--colors", &colors.to_string()])?;
            let pass2_size = std::fs::metadata(&pass_temp)?.len();
            let pass2_reduction = ((current_size - pass2_size) as f64 / current_size as f64) * 100.0;
            info!("   → Size: {} bytes ({:.1}% reduction)", pass2_size, pass2_reduction);
            
            if pass2_reduction > 0.5 {
                std::fs::copy(&pass_temp, output)?;
                current_size = pass2_size;
            } else {
                info!("   → Skipped (no improvement)");
            }
        }
        
        // Pass 3: 有损压缩
        if self.config.lossy_compression {
            info!("   Pass 3: Lossy compression (quality={})", self.config.lossy_quality);
            let lossy_arg = format!("--lossy={}", self.config.lossy_quality);
            self.gifsicle_pass(output, &pass_temp, &["-O3", &lossy_arg])?;
            let pass3_size = std::fs::metadata(&pass_temp)?.len();
            let pass3_reduction = ((current_size - pass3_size) as f64 / current_size as f64) * 100.0;
            info!("   → Size: {} bytes ({:.1}% reduction)", pass3_size, pass3_reduction);
            
            if pass3_reduction > 0.5 {
                std::fs::copy(&pass_temp, output)?;
                current_size = pass3_size;
            } else {
                info!("   → Skipped (no improvement)");
            }
        }
        
        // Pass 4-N: 迭代优化（最多5轮）
        let max_iterations = 5;
        let mut iteration = 4;
        loop {
            if iteration > max_iterations {
                info!("   Max iterations ({}) reached", max_iterations);
                break;
            }
            
            info!("   Pass {}: Iterative optimization", iteration);
            self.gifsicle_pass(output, &pass_temp, &["-O3", "--careful"])?;
            let iter_size = std::fs::metadata(&pass_temp)?.len();
            let iter_reduction = ((current_size - iter_size) as f64 / current_size as f64) * 100.0;
            
            if iter_reduction > 1.0 {
                info!("   → Size: {} bytes ({:.1}% reduction)", iter_size, iter_reduction);
                std::fs::copy(&pass_temp, output)?;
                current_size = iter_size;
                iteration += 1;
            } else {
                info!("   → Converged (reduction < 1%)");
                break;
            }
        }
        
        // 清理
        let _ = std::fs::remove_file(&pass_temp);
        
        let total_reduction = ((initial_size - current_size) as f64 / initial_size as f64) * 100.0;
        info!("🎯 Total reduction: {:.1}% ({} → {} bytes)", 
              total_reduction, initial_size, current_size);
        
        Ok(current_size)
    }
    
    /// 单轮gifsicle优化
    fn gifsicle_pass(&self, input: &Path, output: &Path, args: &[&str]) -> Result<()> {
        let mut cmd = Command::new("gifsicle");
        
        for arg in args {
            cmd.arg(arg);
        }
        
        cmd.arg(input).arg("-o").arg(output);
        
        let output_result = cmd.output()
            .context("Failed to execute gifsicle")?;
        
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            bail!("gifsicle pass failed: {}", stderr);
        }
        
        Ok(())
    }
    
    /// 使用 gifsicle 优化（旧方法，已废弃，保留供参考）
    #[allow(dead_code)]
    fn optimize_with_gifsicle(&self, input: &Path, output: &Path) -> Result<()> {
        debug!("🔧 Running gifsicle optimization...");
        
        let mut cmd = Command::new("gifsicle");
        
        // 基础优化级别
        let opt_level = self.config.frame_optimization.gifsicle_level();
        cmd.arg(format!("-{}", opt_level));
        
        // 色彩优化
        if self.config.color_optimization > 1 {
            let colors = 256u32 / self.config.color_optimization as u32;
            cmd.arg("--colors").arg(colors.to_string());
        }
        
        // 有损压缩
        if self.config.lossy_compression {
            cmd.arg(format!("--lossy={}", self.config.lossy_quality));
        }
        
        // 移除元数据
        if self.config.strip_metadata {
            cmd.arg("--no-comments")
               .arg("--no-names")
               .arg("--no-extensions");
        }
        
        // 输入输出
        cmd.arg(input)
           .arg("-o")
           .arg(output);
        
        debug!("   Command: {:?}", cmd);
        
        let output_result = cmd.output()
            .context("Failed to execute gifsicle")?;
        
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            bail!("gifsicle failed: {}", stderr);
        }
        
        Ok(())
    }
    
    /// 调整尺寸和帧率
    fn resize_and_adjust_fps(&self, input: &Path, output: &Path) -> Result<()> {
        if !Self::check_tool("ffmpeg") {
            bail!("ffmpeg not found, required for resizing/fps adjustment");
        }
        
        debug!("🔧 Running ffmpeg resize/fps adjustment...");
        
        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-i").arg(input);
        
        // 帧率过滤器
        let mut filters = Vec::new();
        
        if let Some(max_fps) = self.config.max_fps {
            filters.push(format!("fps={}", max_fps));
        }
        
        if let Some(max_width) = self.config.max_width {
            filters.push(format!("scale='min({},iw):-1:flags=lanczos'", max_width));
        }
        
        if !filters.is_empty() {
            cmd.arg("-vf").arg(filters.join(","));
        }
        
        cmd.arg("-y")
           .arg(output);
        
        debug!("   Command: {:?}", cmd);
        
        let output_result = cmd.output()
            .context("Failed to execute ffmpeg")?;
        
        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            bail!("ffmpeg failed: {}", stderr);
        }
        
        Ok(())
    }
    
    /// 分析 GIF 信息
    pub fn analyze(input: &Path) -> Result<GifInfo> {
        if !Self::check_tool("ffprobe") {
            bail!("ffprobe not found, required for GIF analysis");
        }
        
        // 使用 ffprobe 获取信息
        let output = Command::new("ffprobe")
            .arg("-v").arg("error")
            .arg("-select_streams").arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height,nb_frames,r_frame_rate,duration")
            .arg("-of").arg("json")
            .arg(input)
            .output()
            .context("Failed to execute ffprobe")?;
        
        if !output.status.success() {
            bail!("ffprobe failed");
        }
        
        let json_str = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&json_str)
            .context("Failed to parse ffprobe output")?;
        
        let stream = &json["streams"][0];
        
        let width = stream["width"].as_u64().unwrap_or(0) as u32;
        let height = stream["height"].as_u64().unwrap_or(0) as u32;
        let frame_count = stream["nb_frames"].as_str()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let duration = stream["duration"].as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        
        // 计算帧率
        let fps = if duration > 0.0 && frame_count > 0 {
            frame_count as f64 / duration
        } else {
            10.0 // 默认
        };
        
        let file_size = std::fs::metadata(input)?.len();
        
        Ok(GifInfo {
            width,
            height,
            frame_count,
            duration,
            fps,
            file_size,
        })
    }
    
    /// 估算优化后的大小
    pub fn estimate_optimized_size(&self, info: &GifInfo) -> u64 {
        let mut estimated_size = info.file_size as f64;
        
        // 帧优化估算
        let frame_reduction = match self.config.frame_optimization {
            FrameOptimization::None => 1.0,
            FrameOptimization::Basic => 0.85,
            FrameOptimization::Balanced => 0.70,
            FrameOptimization::Aggressive => 0.55,
        };
        estimated_size *= frame_reduction;
        
        // 色彩优化估算
        let color_reduction = match self.config.color_optimization {
            1 => 1.0,
            2 => 0.85,
            _ => 0.70,
        };
        estimated_size *= color_reduction;
        
        // 有损压缩估算
        if self.config.lossy_compression {
            let lossy_factor = self.config.lossy_quality as f64 / 100.0;
            estimated_size *= lossy_factor;
        }
        
        // 尺寸调整估算
        if let Some(max_width) = self.config.max_width {
            if info.width > max_width {
                let scale_factor = (max_width as f64 / info.width as f64).powi(2);
                estimated_size *= scale_factor;
            }
        }
        
        estimated_size as u64
    }
}

/// GIF 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GifInfo {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
    pub duration: f64,
    pub fps: f64,
    pub file_size: u64,
}

impl GifInfo {
    /// 获取可读的文件大小
    pub fn file_size_readable(&self) -> String {
        if self.file_size < 1024 {
            format!("{} B", self.file_size)
        } else if self.file_size < 1024 * 1024 {
            format!("{:.1} KB", self.file_size as f64 / 1024.0)
        } else {
            format!("{:.2} MB", self.file_size as f64 / (1024.0 * 1024.0))
        }
    }
}

/// 优化结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_size: u64,
    pub optimized_size: u64,
    pub reduction_percent: f64,
    pub elapsed_ms: u64,
}

impl OptimizationResult {
    /// 是否有改善
    pub fn is_improved(&self) -> bool {
        self.optimized_size < self.original_size
    }
    
    /// 节省的字节数
    pub fn bytes_saved(&self) -> u64 {
        if self.is_improved() {
            self.original_size - self.optimized_size
        } else {
            0
        }
    }
}
