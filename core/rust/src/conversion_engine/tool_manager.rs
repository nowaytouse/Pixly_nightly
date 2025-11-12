/*
🔧 PIXLY v3.1 工具调度系统

Rust统一工具管理，替代分散的工具调用：
- 统一工具检测与版本管理
- 智能工具选择算法
- 工具性能监控
- 错误恢复与重试机制
- 工具能力动态评估

支持 cjxl/avifenc/cwebp/ffmpeg/magick 等所有转换工具
*/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConversionTool {
    CJXL,
    DJXL,
    AVIFENC,
    AVIFDEC,
    CWEBP,
    DWEBP,
    FFMPEG,
    MAGICK,
}

impl ConversionTool {
    pub fn get_executable_name(&self) -> &'static str {
        match self {
            ConversionTool::CJXL => "cjxl",
            ConversionTool::DJXL => "djxl",
            ConversionTool::AVIFENC => "avifenc",
            ConversionTool::AVIFDEC => "avifdec",
            ConversionTool::CWEBP => "cwebp",
            ConversionTool::DWEBP => "dwebp",
            ConversionTool::FFMPEG => "ffmpeg",
            ConversionTool::MAGICK => "magick",
        }
    }
    
    pub fn get_supported_formats(&self) -> Vec<&'static str> {
        match self {
            ConversionTool::CJXL => vec!["jxl"],
            ConversionTool::DJXL => vec!["jpeg", "png", "ppm"],
            ConversionTool::AVIFENC => vec!["avif"],
            ConversionTool::AVIFDEC => vec!["jpeg", "png", "ppm"],
            ConversionTool::CWEBP => vec!["webp"],
            ConversionTool::DWEBP => vec!["png", "ppm", "pgm"],
            ConversionTool::FFMPEG => vec!["jpeg", "png", "webp", "mp4", "webm"],
            ConversionTool::MAGICK => vec!["jpeg", "png", "bmp", "tiff", "gif"],
        }
    }
    
    pub fn is_encoder(&self) -> bool {
        matches!(self, ConversionTool::CJXL | ConversionTool::AVIFENC | 
                      ConversionTool::CWEBP | ConversionTool::FFMPEG | 
                      ConversionTool::MAGICK)
    }
    
    pub fn is_decoder(&self) -> bool {
        matches!(self, ConversionTool::DJXL | ConversionTool::AVIFDEC | 
                      ConversionTool::DWEBP | ConversionTool::FFMPEG | 
                      ConversionTool::MAGICK)
    }
}

impl std::fmt::Display for ConversionTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_executable_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub tool: ConversionTool,
    pub version: String,
    pub path: PathBuf,
    pub available: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub performance_score: f32,  // 0.0-1.0
    pub error_count: u32,
    pub success_count: u32,
    pub avg_execution_time_ms: f32,
    pub supported_features: Vec<String>,
}

impl ToolInfo {
    pub fn success_rate(&self) -> f32 {
        let total = self.success_count + self.error_count;
        if total == 0 {
            1.0
        } else {
            self.success_count as f32 / total as f32
        }
    }
    
    pub fn reliability_score(&self) -> f32 {
        self.success_rate() * self.performance_score
    }
}

#[derive(Debug, Clone)]
pub struct ToolSelectionCriteria {
    pub target_format: String,
    pub quality_priority: bool,      // true = 质量优先, false = 速度优先
    pub compatibility_required: bool, // 是否需要最大兼容性
    pub lossless_required: bool,     // 是否需要无损压缩
    pub batch_processing: bool,      // 是否批量处理
}

pub struct ToolManager {
    tools: Arc<RwLock<HashMap<ConversionTool, ToolInfo>>>,
    tool_paths: HashMap<ConversionTool, Vec<PathBuf>>,
    format_tool_map: HashMap<String, Vec<ConversionTool>>,
}

impl ToolManager {
    pub async fn new() -> Result<Self> {
        let mut manager = Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            tool_paths: Self::create_tool_paths(),
            format_tool_map: Self::create_format_tool_map(),
        };
        
        // 初始化检测所有工具
        manager.detect_all_tools().await?;
        
        info!("🔧 工具管理器初始化完成");
        Ok(manager)
    }
    
    fn create_tool_paths() -> HashMap<ConversionTool, Vec<PathBuf>> {
        let mut paths = HashMap::new();
        
        // 常见安装路径
        let common_paths = vec![
            "/usr/local/bin",
            "/opt/homebrew/bin",
            "/usr/bin",
            "/bin",
            "C:/Program Files/ImageMagick",
            "C:/ffmpeg/bin",
            "C:/tools",
        ];
        
        for tool in [
            ConversionTool::CJXL, ConversionTool::DJXL,
            ConversionTool::AVIFENC, ConversionTool::AVIFDEC,
            ConversionTool::CWEBP, ConversionTool::DWEBP,
            ConversionTool::FFMPEG, ConversionTool::MAGICK,
        ] {
            let mut tool_paths = Vec::new();
            
            for base_path in &common_paths {
                let mut path = PathBuf::from(base_path);
                path.push(tool.get_executable_name());
                tool_paths.push(path.clone());
                
                // Windows executable
                path.set_extension("exe");
                tool_paths.push(path);
            }
            
            paths.insert(tool, tool_paths);
        }
        
        paths
    }
    
    fn create_format_tool_map() -> HashMap<String, Vec<ConversionTool>> {
        let mut map = HashMap::new();
        
        map.insert("jxl".to_string(), vec![ConversionTool::CJXL]);
        map.insert("avif".to_string(), vec![ConversionTool::AVIFENC]);
        map.insert("webp".to_string(), vec![ConversionTool::CWEBP]);
        map.insert("jpeg".to_string(), vec![ConversionTool::FFMPEG, ConversionTool::MAGICK]);
        map.insert("jpg".to_string(), vec![ConversionTool::FFMPEG, ConversionTool::MAGICK]);
        map.insert("png".to_string(), vec![ConversionTool::MAGICK, ConversionTool::FFMPEG]);
        map.insert("gif".to_string(), vec![ConversionTool::MAGICK]);
        map.insert("bmp".to_string(), vec![ConversionTool::MAGICK]);
        map.insert("tiff".to_string(), vec![ConversionTool::MAGICK]);
        
        map
    }
    
    async fn detect_all_tools(&mut self) -> Result<()> {
        let mut tools = self.tools.write().await;
        
        for (tool, search_paths) in &self.tool_paths {
            match self.detect_tool_at_paths(tool, search_paths).await {
                Ok(tool_info) => {
                    info!("✅ 检测到工具: {} v{} at {}", 
                          tool, tool_info.version, tool_info.path.display());
                    tools.insert(*tool, tool_info);
                }
                Err(e) => {
                    warn!("⚠️ 未找到工具 {}: {}", tool, e);
                    // 创建不可用的工具信息
                    tools.insert(*tool, ToolInfo {
                        tool: *tool,
                        version: "unknown".to_string(),
                        path: PathBuf::new(),
                        available: false,
                        last_check: chrono::Utc::now(),
                        performance_score: 0.0,
                        error_count: 0,
                        success_count: 0,
                        avg_execution_time_ms: 0.0,
                        supported_features: vec![],
                    });
                }
            }
        }
        
        Ok(())
    }
    
    async fn detect_tool_at_paths(&self, tool: &ConversionTool, search_paths: &[PathBuf]) -> Result<ToolInfo> {
        for path in search_paths {
            if path.exists() {
                match self.get_tool_version(tool, path).await {
                    Ok(version) => {
                        let features = self.detect_tool_features(tool, path).await;
                        
                        return Ok(ToolInfo {
                            tool: *tool,
                            version,
                            path: path.clone(),
                            available: true,
                            last_check: chrono::Utc::now(),
                            performance_score: 1.0, // 初始评分
                            error_count: 0,
                            success_count: 0,
                            avg_execution_time_ms: 0.0,
                            supported_features: features,
                        });
                    }
                    Err(e) => {
                        debug!("工具版本检测失败 {} at {}: {}", tool, path.display(), e);
                        continue;
                    }
                }
            }
        }
        
        Err(anyhow!("工具未找到: {}", tool))
    }
    
    async fn get_tool_version(&self, tool: &ConversionTool, path: &Path) -> Result<String> {
        let version_args = match tool {
            ConversionTool::CJXL | ConversionTool::DJXL => vec!["--version"],
            ConversionTool::AVIFENC | ConversionTool::AVIFDEC => vec!["--version"],
            ConversionTool::CWEBP | ConversionTool::DWEBP => vec!["-version"],
            ConversionTool::FFMPEG => vec!["-version"],
            ConversionTool::MAGICK => vec!["-version"],
        };
        
        let output = Command::new(path)
            .args(&version_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if output.status.success() {
            let version_text = String::from_utf8_lossy(&output.stdout);
            self.parse_version_string(tool, &version_text)
        } else {
            Err(anyhow!("版本检测命令失败"))
        }
    }
    
    fn parse_version_string(&self, tool: &ConversionTool, version_text: &str) -> Result<String> {
        // 不同工具的版本字符串解析
        match tool {
            ConversionTool::CJXL | ConversionTool::DJXL => {
                // JXL: "cjxl [version]"
                if let Some(line) = version_text.lines().next() {
                    if let Some(version) = line.split_whitespace().nth(1) {
                        return Ok(version.to_string());
                    }
                }
            }
            ConversionTool::AVIFENC | ConversionTool::AVIFDEC => {
                // AVIF: "avifenc version x.x.x"
                for line in version_text.lines() {
                    if line.contains("version") {
                        if let Some(version) = line.split_whitespace().last() {
                            return Ok(version.to_string());
                        }
                    }
                }
            }
            ConversionTool::CWEBP | ConversionTool::DWEBP => {
                // WebP: "x.x.x"
                if let Some(line) = version_text.lines().next() {
                    if let Some(version) = line.split_whitespace().next() {
                        return Ok(version.to_string());
                    }
                }
            }
            ConversionTool::FFMPEG => {
                // FFmpeg: "ffmpeg version x.x.x"
                for line in version_text.lines() {
                    if line.starts_with("ffmpeg version") {
                        if let Some(version) = line.split_whitespace().nth(2) {
                            return Ok(version.to_string());
                        }
                    }
                }
            }
            ConversionTool::MAGICK => {
                // ImageMagick: "Version: ImageMagick x.x.x"
                for line in version_text.lines() {
                    if line.contains("ImageMagick") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        for (i, part) in parts.iter().enumerate() {
                            if *part == "ImageMagick" && i + 1 < parts.len() {
                                return Ok(parts[i + 1].to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok("unknown".to_string())
    }
    
    async fn detect_tool_features(&self, tool: &ConversionTool, _path: &Path) -> Vec<String> {
        // 检测工具支持的特性
        // 这里是简化实现，实际应该运行工具并解析help输出
        match tool {
            ConversionTool::CJXL => vec![
                "lossless".to_string(),
                "progressive".to_string(),
                "modular".to_string(),
                "gaborish".to_string(),
            ],
            ConversionTool::AVIFENC => vec![
                "lossless".to_string(),
                "tiles".to_string(),
                "hdr".to_string(),
            ],
            ConversionTool::CWEBP => vec![
                "lossless".to_string(),
                "auto_filter".to_string(),
                "preprocessing".to_string(),
            ],
            ConversionTool::FFMPEG => vec![
                "hardware_acceleration".to_string(),
                "filtering".to_string(),
                "metadata".to_string(),
            ],
            ConversionTool::MAGICK => vec![
                "color_profiles".to_string(),
                "transparency".to_string(),
                "metadata".to_string(),
            ],
            _ => vec![],
        }
    }
    
    pub async fn select_optimal_tool(&self, target_format: &str) -> Result<ConversionTool> {
        let criteria = ToolSelectionCriteria {
            target_format: target_format.to_string(),
            quality_priority: true,
            compatibility_required: false,
            lossless_required: false,
            batch_processing: false,
        };
        
        self.select_tool_by_criteria(&criteria).await
    }
    
    pub async fn select_tool_by_criteria(&self, criteria: &ToolSelectionCriteria) -> Result<ConversionTool> {
        let tools = self.tools.read().await;
        
        // 获取支持目标格式的工具
        let candidate_tools = self.format_tool_map
            .get(&criteria.target_format)
            .ok_or_else(|| anyhow!("不支持的格式: {}", criteria.target_format))?;
        
        // 过滤可用工具
        let available_tools: Vec<&ConversionTool> = candidate_tools
            .iter()
            .filter(|tool| {
                tools.get(tool)
                    .map(|info| info.available)
                    .unwrap_or(false)
            })
            .collect();
        
        if available_tools.is_empty() {
            return Err(anyhow!("没有可用的工具支持格式: {}", criteria.target_format));
        }
        
        // 根据标准选择最佳工具
        let best_tool = available_tools
            .iter()
            .max_by(|a, b| {
                let score_a = self.calculate_tool_score(a, criteria, &tools);
                let score_b = self.calculate_tool_score(b, criteria, &tools);
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap();
        
        Ok(**best_tool)
    }
    
    fn calculate_tool_score(&self, tool: &ConversionTool, criteria: &ToolSelectionCriteria, 
                           tools: &HashMap<ConversionTool, ToolInfo>) -> f32 {
        let tool_info = match tools.get(tool) {
            Some(info) => info,
            None => return 0.0,
        };
        
        let mut score = tool_info.reliability_score();
        
        // 质量优先权重
        if criteria.quality_priority {
            match tool {
                ConversionTool::CJXL => score *= 1.2,      // JXL质量最佳
                ConversionTool::AVIFENC => score *= 1.1,   // AVIF质量很好
                ConversionTool::CWEBP => score *= 1.0,     // WebP质量标准
                ConversionTool::FFMPEG => score *= 0.9,    // FFmpeg质量一般
                ConversionTool::MAGICK => score *= 0.8,    // ImageMagick质量较低
                _ => {}
            }
        }
        
        // 速度优先权重（与质量相反）
        if !criteria.quality_priority {
            match tool {
                ConversionTool::CWEBP => score *= 1.2,     // WebP速度最快
                ConversionTool::MAGICK => score *= 1.1,    // ImageMagick速度快
                ConversionTool::FFMPEG => score *= 1.0,    // FFmpeg速度标准
                ConversionTool::AVIFENC => score *= 0.8,   // AVIF速度慢
                ConversionTool::CJXL => score *= 0.9,      // JXL速度较慢
                _ => {}
            }
        }
        
        // 无损要求
        if criteria.lossless_required {
            if tool_info.supported_features.contains(&"lossless".to_string()) {
                score *= 1.1;
            } else {
                score *= 0.5; // 大幅降低不支持无损的工具评分
            }
        }
        
        // 执行时间惩罚
        if tool_info.avg_execution_time_ms > 1000.0 {
            score *= 0.9;
        }
        
        score
    }
    
    pub async fn update_tool_performance(&self, tool: &ConversionTool, 
                                       execution_time_ms: f32, success: bool) {
        let mut tools = self.tools.write().await;
        if let Some(tool_info) = tools.get_mut(tool) {
            // 更新统计
            if success {
                tool_info.success_count += 1;
            } else {
                tool_info.error_count += 1;
            }
            
            // 更新平均执行时间（移动平均）
            if tool_info.avg_execution_time_ms == 0.0 {
                tool_info.avg_execution_time_ms = execution_time_ms;
            } else {
                tool_info.avg_execution_time_ms = 
                    tool_info.avg_execution_time_ms * 0.9 + execution_time_ms * 0.1;
            }
            
            // 更新性能评分
            tool_info.performance_score = self.calculate_performance_score(tool_info);
        }
    }
    
    fn calculate_performance_score(&self, tool_info: &ToolInfo) -> f32 {
        let success_rate = tool_info.success_rate();
        let speed_score = if tool_info.avg_execution_time_ms > 0.0 {
            1.0 / (1.0 + tool_info.avg_execution_time_ms / 1000.0)
        } else {
            1.0
        };
        
        // 综合评分：成功率70% + 速度30%
        success_rate * 0.7 + speed_score * 0.3
    }
    
    pub async fn get_tool_info(&self, tool: &ConversionTool) -> Option<ToolInfo> {
        let tools = self.tools.read().await;
        tools.get(tool).cloned()
    }
    
    pub async fn list_available_tools(&self) -> Vec<ConversionTool> {
        let tools = self.tools.read().await;
        tools.values()
            .filter(|info| info.available)
            .map(|info| info.tool)
            .collect()
    }
    
    pub async fn get_tools_for_format(&self, format: &str) -> Vec<ConversionTool> {
        self.format_tool_map
            .get(format)
            .cloned()
            .unwrap_or_default()
    }
    
    pub async fn refresh_tool_availability(&mut self) -> Result<()> {
        info!("🔄 刷新工具可用性检查");
        self.detect_all_tools().await
    }
    
    pub async fn get_manager_stats(&self) -> ToolManagerStats {
        let tools = self.tools.read().await;
        let available_count = tools.values().filter(|info| info.available).count();
        let total_conversions: u32 = tools.values()
            .map(|info| info.success_count + info.error_count)
            .sum();
        let total_success: u32 = tools.values()
            .map(|info| info.success_count)
            .sum();
        
        ToolManagerStats {
            total_tools: tools.len(),
            available_tools: available_count,
            total_conversions,
            overall_success_rate: if total_conversions > 0 {
                total_success as f32 / total_conversions as f32
            } else {
                0.0
            },
            supported_formats: self.format_tool_map.keys().len(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ToolManagerStats {
    pub total_tools: usize,
    pub available_tools: usize,
    pub total_conversions: u32,
    pub overall_success_rate: f32,
    pub supported_formats: usize,
}
