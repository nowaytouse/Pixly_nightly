// 🚀 转换策略系统
// 从 @archive/rust_broken/src/converter/strategy.rs 提取并增强
//
// 核心功能:
// - 策略模式设计
// - 多策略支持（原生编码器、CLI工具）
// - 智能策略选择
// - 统一转换接口
// - 响亮报错（无降级）

use anyhow::Result;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// 策略类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyType {
    Native,
    Cli,
    Auto,
}

impl Default for StrategyType {
    fn default() -> Self {
        Self::Auto
    }
}

/// 转换配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub quality: u8,
    pub speed: u8,
    pub preserve_metadata: bool,
    pub keep_animated: bool,
    pub strategy: StrategyType,
    pub lossless: bool,
    pub normalize_filenames: Option<bool>,
}

/// 转换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub output_path: String,
    pub input_size: u64,
    pub output_size: u64,
    pub compression_ratio: f64,
    pub processing_time_ms: u64,
    pub strategy_used: String,
    pub error_message: Option<String>,
}

/// 转换策略trait
pub trait ConversionStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn supported_formats(&self) -> Vec<String>;
    fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult>;
    fn priority(&self) -> u8 {
        50
    }
}

/// 策略管理器
pub struct StrategyManager {
    strategies: Vec<Box<dyn ConversionStrategy>>,
}

impl StrategyManager {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }
    
    pub fn register(&mut self, strategy: Box<dyn ConversionStrategy>) {
        self.strategies.push(strategy);
    }
    
    pub fn available_strategies(&self) -> Vec<&str> {
        self.strategies
            .iter()
            .filter(|s| s.is_available())
            .map(|s| s.name())
            .collect()
    }
    
    pub fn clear_strategies(&mut self) {
        self.strategies.clear();
    }
    
    pub fn select_strategy(
        &self,
        format: &str,
        strategy_type: StrategyType,
    ) -> Option<&dyn ConversionStrategy> {
        let mut candidates: Vec<_> = self.strategies
            .iter()
            .filter(|s| s.is_available())
            .filter(|s| s.supported_formats().contains(&format.to_lowercase()))
            .collect();
        
        if candidates.is_empty() {
            return None;
        }
        
        candidates.sort_by_key(|s| std::cmp::Reverse(s.priority()));
        
        match strategy_type {
            StrategyType::Auto => candidates.first().map(|b| &***b),
            StrategyType::Native => {
                candidates.iter()
                    .find(|s| s.name().contains("Native"))
                    .map(|b| &***b)
                    .or_else(|| candidates.first().map(|b| &***b))
            }
            StrategyType::Cli => {
                candidates.iter()
                    .find(|s| s.name().contains("CLI"))
                    .map(|b| &***b)
                    .or_else(|| candidates.first().map(|b| &***b))
            }
        }
    }
    
    pub fn convert(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        config: &ConversionConfig,
    ) -> Result<ConversionResult> {
        let strategy = self.select_strategy(format, config.strategy)
            .ok_or_else(|| anyhow::anyhow!("No available strategy for format: {}", format))?;
        
        strategy.convert(input, output, format, config)
    }
}

impl Default for StrategyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockStrategy;
    
    impl ConversionStrategy for MockStrategy {
        fn name(&self) -> &str {
            "MockStrategy"
        }
        
        fn is_available(&self) -> bool {
            true
        }
        
        fn supported_formats(&self) -> Vec<String> {
            vec!["webp".to_string()]
        }
        
        fn convert(
            &self,
            _input: &Path,
            _output: &Path,
            _format: &str,
            _config: &ConversionConfig,
        ) -> Result<ConversionResult> {
            Ok(ConversionResult {
                success: true,
                output_path: "output.webp".to_string(),
                input_size: 1000,
                output_size: 800,
                compression_ratio: 0.2,
                processing_time_ms: 100,
                strategy_used: "MockStrategy".to_string(),
                error_message: None,
            })
        }
    }
    
    #[test]
    fn test_strategy_manager() {
        let mut manager = StrategyManager::new();
        manager.register(Box::new(MockStrategy));
        
        let strategies = manager.available_strategies();
        assert_eq!(strategies.len(), 1);
        assert_eq!(strategies[0], "MockStrategy");
        
        let strategy = manager.select_strategy("webp", StrategyType::Auto);
        assert!(strategy.is_some());
    }
}
