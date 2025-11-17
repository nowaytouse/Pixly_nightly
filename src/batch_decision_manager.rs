//! 批量决策管理器
//! 
//! 智能处理损坏文件和低品质文件的批量决策系统

use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use anyhow::Result;

/// 决策类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecisionType {
    CorruptedFiles,
    LowQualityFiles,
    UnsupportedFormat,
    OversizedFiles,
}

/// 用户决策选择
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserDecisionChoice {
    TryRepair,      // 尝试修复
    DeleteAll,      // 删除所有
    Abort,          // 中止处理
    Ignore,         // 忽略错误
    Skip,           // 跳过文件
    ForceConvert,   // 强制转换
    AutoDecide,     // 自动决策
}

/// 损坏类型
#[derive(Debug, Clone)]
pub enum CorruptionType {
    InvalidHeader,      // 无效文件头
    TruncatedData,      // 数据截断
    UnsupportedFormat,  // 不支持的格式
    MissingMetadata,    // 缺失元数据
    CorruptedPixels,    // 像素损坏
}

/// 处理模式
#[derive(Debug, Clone)]
pub enum ProcessingMode {
    Skip,       // 跳过
    Convert,    // 转换
    Delete,     // 删除
    Repair,     // 修复
    Ignore,     // 忽略
}

/// 损坏文件信息
#[derive(Debug, Clone)]
pub struct CorruptedFile {
    pub file_path: PathBuf,
    pub corruption_type: CorruptionType,
    pub error_message: String,
    pub file_size: u64,
    pub can_repair: bool,
    pub repair_confidence: f32,  // 修复成功率 0.0-1.0
}

/// 低品质文件信息
#[derive(Debug, Clone)]
pub struct LowQualityFile {
    pub file_path: PathBuf,
    pub quality_score: f64,
    pub quality_issues: Vec<String>,
    pub file_size: u64,
    pub can_convert: bool,
    pub recommended_action: ProcessingMode,
}

/// 批量决策管理器
pub struct BatchDecisionManager {
    interactive_mode: bool,
    #[allow(dead_code)]
    countdown_duration: Duration,
    corrupted_files: Vec<CorruptedFile>,
    low_quality_files: Vec<LowQualityFile>,
    decisions: HashMap<DecisionType, UserDecisionChoice>,
    auto_repair_threshold: f32,  // 自动修复阈值
    auto_skip_threshold: f64,    // 自动跳过质量阈值
}

impl BatchDecisionManager {
    /// 创建新的批量决策管理器
    pub fn new(interactive_mode: bool) -> Self {
        Self {
            interactive_mode,
            countdown_duration: Duration::from_secs(5),
            corrupted_files: Vec::new(),
            low_quality_files: Vec::new(),
            decisions: HashMap::new(),
            auto_repair_threshold: 0.7,  // 70%以上成功率自动修复
            auto_skip_threshold: 0.3,    // 30%以下质量自动跳过
        }
    }
    
    /// 使用自定义配置创建
    pub fn with_config(
        interactive_mode: bool,
        countdown_duration: Duration,
        auto_repair_threshold: f32,
        auto_skip_threshold: f64,
    ) -> Self {
        Self {
            interactive_mode,
            countdown_duration,
            corrupted_files: Vec::new(),
            low_quality_files: Vec::new(),
            decisions: HashMap::new(),
            auto_repair_threshold,
            auto_skip_threshold,
        }
    }
    
    /// 添加损坏文件
    pub fn add_corrupted_file(&mut self, file: CorruptedFile) -> Result<()> {
        println!("⚠️  Corrupted file detected: {:?}", file.file_path);
        println!("   Type: {:?}", file.corruption_type);
        println!("   Error: {}", file.error_message);
        println!("   Repairable: {}, Success rate: {:.1}%", file.can_repair, file.repair_confidence * 100.0);
        
        self.corrupted_files.push(file);
        Ok(())
    }
    
    /// 添加低品质文件
    pub fn add_low_quality_file(&mut self, file: LowQualityFile) -> Result<()> {
        println!("⚠️  Low quality file detected: {:?}", file.file_path);
        println!("   Quality score: {:.2}", file.quality_score);
        println!("   Issues: {:?}", file.quality_issues);
        println!("   Recommended action: {:?}", file.recommended_action);
        
        self.low_quality_files.push(file);
        Ok(())
    }
    
    /// 设置决策
    pub fn set_decision(&mut self, decision_type: DecisionType, choice: UserDecisionChoice) {
        self.decisions.insert(decision_type, choice);
    }
    
    /// 获取决策
    pub fn get_decision(&self, decision_type: &DecisionType) -> Option<&UserDecisionChoice> {
        self.decisions.get(decision_type)
    }
    
    /// 自动决策损坏文件
    fn auto_decide_corrupted(&self, file: &CorruptedFile) -> ProcessingMode {
        if file.can_repair && file.repair_confidence >= self.auto_repair_threshold {
            ProcessingMode::Repair
        } else if file.can_repair && file.repair_confidence >= 0.3 {
            ProcessingMode::Skip  // 成功率不高，跳过
        } else {
            ProcessingMode::Delete  // 无法修复，删除
        }
    }
    
    /// 自动决策低品质文件
    fn auto_decide_low_quality(&self, file: &LowQualityFile) -> ProcessingMode {
        if file.quality_score < self.auto_skip_threshold {
            ProcessingMode::Skip
        } else if file.can_convert {
            ProcessingMode::Convert
        } else {
            ProcessingMode::Ignore
        }
    }
    
    /// 处理批量决策
    pub fn process_batch_decisions(&mut self) -> Result<BatchDecisionResult> {
        let mut result = BatchDecisionResult {
            total_files: self.corrupted_files.len() + self.low_quality_files.len(),
            successful_files: 0,
            failed_files: 0,
            skipped_files: 0,
            repaired_files: 0,
            deleted_files: 0,
            converted_files: 0,
        };
        
        // 处理损坏文件
        for file in &self.corrupted_files {
            let mode = if self.interactive_mode {
                // 交互模式：询问用户
                self.ask_user_decision_corrupted(file)?
            } else {
                // 自动模式：自动决策
                self.auto_decide_corrupted(file)
            };
            
            match mode {
                ProcessingMode::Repair => {
                    println!("🔧 Repair: {:?}", file.file_path);
                    result.repaired_files += 1;
                    result.successful_files += 1;
                }
                ProcessingMode::Delete => {
                    println!("🗑️  Delete: {:?}", file.file_path);
                    result.deleted_files += 1;
                }
                ProcessingMode::Skip => {
                    println!("⏭️  Skip: {:?}", file.file_path);
                    result.skipped_files += 1;
                }
                _ => {
                    result.failed_files += 1;
                }
            }
        }
        
        // 处理低品质文件
        for file in &self.low_quality_files {
            let mode = if self.interactive_mode {
                self.ask_user_decision_low_quality(file)?
            } else {
                self.auto_decide_low_quality(file)
            };
            
            match mode {
                ProcessingMode::Convert => {
                    println!("🔄 Convert: {:?}", file.file_path);
                    result.converted_files += 1;
                    result.successful_files += 1;
                }
                ProcessingMode::Skip => {
                    println!("⏭️  Skip: {:?}", file.file_path);
                    result.skipped_files += 1;
                }
                ProcessingMode::Ignore => {
                    println!("👁️  Ignore: {:?}", file.file_path);
                    result.successful_files += 1;
                }
                _ => {
                    result.failed_files += 1;
                }
            }
        }
        
        Ok(result)
    }
    
    /// 询问用户决策（损坏文件）
    fn ask_user_decision_corrupted(&self, _file: &CorruptedFile) -> Result<ProcessingMode> {
        // 简化实现：自动决策
        Ok(ProcessingMode::Skip)
    }
    
    /// 询问用户决策（低品质文件）
    fn ask_user_decision_low_quality(&self, _file: &LowQualityFile) -> Result<ProcessingMode> {
        // 简化实现：自动决策
        Ok(ProcessingMode::Skip)
    }
    
    /// 获取统计信息
    pub fn get_statistics(&self) -> DecisionStatistics {
        DecisionStatistics {
            total_corrupted: self.corrupted_files.len(),
            total_low_quality: self.low_quality_files.len(),
            repairable_count: self.corrupted_files.iter().filter(|f| f.can_repair).count(),
            convertible_count: self.low_quality_files.iter().filter(|f| f.can_convert).count(),
        }
    }
    
    /// 清空所有文件记录
    pub fn clear(&mut self) {
        self.corrupted_files.clear();
        self.low_quality_files.clear();
        self.decisions.clear();
    }
}

/// 批量决策结果
#[derive(Debug, Clone)]
pub struct BatchDecisionResult {
    pub total_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub skipped_files: usize,
    pub repaired_files: usize,
    pub deleted_files: usize,
    pub converted_files: usize,
}

/// 决策统计信息
#[derive(Debug, Clone)]
pub struct DecisionStatistics {
    pub total_corrupted: usize,
    pub total_low_quality: usize,
    pub repairable_count: usize,
    pub convertible_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_decision_manager_creation() {
        let manager = BatchDecisionManager::new(false);
        assert!(!manager.interactive_mode);
        assert_eq!(manager.corrupted_files.len(), 0);
    }
    
    #[test]
    fn test_add_corrupted_file() {
        let mut manager = BatchDecisionManager::new(false);
        let file = CorruptedFile {
            file_path: PathBuf::from("test.jpg"),
            corruption_type: CorruptionType::InvalidHeader,
            error_message: "Invalid header".to_string(),
            file_size: 1024,
            can_repair: true,
            repair_confidence: 0.8,
        };
        
        assert!(manager.add_corrupted_file(file).is_ok());
        assert_eq!(manager.corrupted_files.len(), 1);
    }
    
    #[test]
    fn test_add_low_quality_file() {
        let mut manager = BatchDecisionManager::new(false);
        let file = LowQualityFile {
            file_path: PathBuf::from("test.jpg"),
            quality_score: 0.5,
            quality_issues: vec!["Low resolution".to_string()],
            file_size: 1024,
            can_convert: true,
            recommended_action: ProcessingMode::Convert,
        };
        
        assert!(manager.add_low_quality_file(file).is_ok());
        assert_eq!(manager.low_quality_files.len(), 1);
    }
    
    #[test]
    fn test_auto_decision() {
        let manager = BatchDecisionManager::new(false);
        
        let high_confidence_file = CorruptedFile {
            file_path: PathBuf::from("test.jpg"),
            corruption_type: CorruptionType::TruncatedData,
            error_message: "Truncated".to_string(),
            file_size: 1024,
            can_repair: true,
            repair_confidence: 0.9,
        };
        
        let mode = manager.auto_decide_corrupted(&high_confidence_file);
        assert!(matches!(mode, ProcessingMode::Repair));
    }
    
    #[test]
    fn test_statistics() {
        let mut manager = BatchDecisionManager::new(false);
        
        manager.add_corrupted_file(CorruptedFile {
            file_path: PathBuf::from("test1.jpg"),
            corruption_type: CorruptionType::InvalidHeader,
            error_message: "Error".to_string(),
            file_size: 1024,
            can_repair: true,
            repair_confidence: 0.8,
        }).unwrap();
        
        manager.add_low_quality_file(LowQualityFile {
            file_path: PathBuf::from("test2.jpg"),
            quality_score: 0.4,
            quality_issues: vec!["Low quality".to_string()],
            file_size: 2048,
            can_convert: true,
            recommended_action: ProcessingMode::Convert,
        }).unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_corrupted, 1);
        assert_eq!(stats.total_low_quality, 1);
        assert_eq!(stats.repairable_count, 1);
        assert_eq!(stats.convertible_count, 1);
    }
}
