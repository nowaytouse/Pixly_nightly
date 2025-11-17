/**
 * 贝叶斯参数优化器
 * 
 * 核心目标: 在保证质量的前提下最小化文件大小
 * 
 * 算法:
 * 1. 使用高斯过程建模 quality-size 关系
 * 2. 使用Expected Improvement (EI) 作为采集函数
 * 3. 自适应探索-利用平衡
 */
use serde::{Serialize, Deserialize};

/// 优化目标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationObjective {
    /// 最小质量阈值 (SSIM)
    pub min_quality: f64,
    /// 目标文件大小 (bytes, None表示尽可能小)
    pub target_size: Option<u64>,
    /// 质量权重 (0-1, 越高越重视质量)
    pub quality_weight: f64,
}

impl Default for OptimizationObjective {
    fn default() -> Self {
        Self {
            min_quality: 0.95, // SSIM >= 0.95
            target_size: None,
            quality_weight: 0.7, // 70%质量, 30%大小
        }
    }
}

/// 参数空间定义
#[derive(Debug, Clone)]
pub struct ParameterSpace {
    /// 质量范围 [min, max]
    pub quality_range: (u32, u32),
    /// 速度/effort范围 [min, max]
    pub effort_range: (u32, u32),
    /// 是否尝试无损
    pub try_lossless: bool,
}

impl Default for ParameterSpace {
    fn default() -> Self {
        Self {
            quality_range: (60, 100),
            effort_range: (4, 9),
            try_lossless: true,
        }
    }
}

/// 观测点 (参数 -> 结果)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// 质量参数
    pub quality: u32,
    /// 速度参数
    pub effort: u32,
    /// 是否无损
    pub lossless: bool,
    /// 实际质量 (SSIM)
    pub actual_quality: f64,
    /// 实际大小 (bytes)
    pub actual_size: u64,
    /// 综合得分
    pub score: f64,
}

/// 贝叶斯优化器
pub struct BayesianOptimizer {
    /// 参数空间
    space: ParameterSpace,
    /// 优化目标
    objective: OptimizationObjective,
    /// 历史观测
    observations: Vec<Observation>,
    /// 最佳观测
    best_observation: Option<Observation>,
}

impl BayesianOptimizer {
    /// 创建新的优化器
    pub fn new(space: ParameterSpace, objective: OptimizationObjective) -> Self {
        Self {
            space,
            objective,
            observations: Vec::new(),
            best_observation: None,
        }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(ParameterSpace::default(), OptimizationObjective::default())
    }

    /// 建议下一组参数 (Expected Improvement)
    pub fn suggest_next_parameters(&self) -> (u32, u32, bool) {
        if self.observations.is_empty() {
            // 初始点: 中等质量, 中等effort
            let quality = (self.space.quality_range.0 + self.space.quality_range.1) / 2;
            let effort = (self.space.effort_range.0 + self.space.effort_range.1) / 2;
            return (quality, effort, false);
        }

        // 使用Expected Improvement选择下一个点
        let mut best_ei = f64::NEG_INFINITY;
        let mut best_params = (75, 6, false);

        // 网格搜索候选点
        for quality in (self.space.quality_range.0..=self.space.quality_range.1).step_by(5) {
            for effort in self.space.effort_range.0..=self.space.effort_range.1 {
                for &lossless in &[false, true] {
                    if !self.space.try_lossless && lossless {
                        continue;
                    }

                    let ei = self.expected_improvement(quality, effort, lossless);
                    if ei > best_ei {
                        best_ei = ei;
                        best_params = (quality, effort, lossless);
                    }
                }
            }
        }

        best_params
    }

    /// 计算Expected Improvement
    fn expected_improvement(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
        // 预测该参数的得分
        let predicted_score = self.predict_score(quality, effort, lossless);
        
        // 当前最佳得分
        let best_score = self.best_observation
            .as_ref()
            .map(|obs| obs.score)
            .unwrap_or(f64::NEG_INFINITY);

        // 简化的EI: improvement + exploration bonus
        let improvement = (predicted_score - best_score).max(0.0);
        let exploration = self.exploration_bonus(quality, effort, lossless);
        
        improvement + exploration
    }

    /// 预测得分 (使用高斯过程回归的简化版本)
    fn predict_score(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
        if self.observations.is_empty() {
            return 0.0;
        }

        // 使用加权平均 (基于距离)
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;

        for obs in &self.observations {
            let distance = self.parameter_distance(
                quality, effort, lossless,
                obs.quality, obs.effort, obs.lossless
            );
            
            // RBF kernel
            let weight = (-distance * distance / 100.0).exp();
            weighted_sum += obs.score * weight;
            weight_sum += weight;
        }

        if weight_sum > 0.0 {
            weighted_sum / weight_sum
        } else {
            0.0
        }
    }

    /// 探索奖励 (鼓励尝试未探索的区域)
    fn exploration_bonus(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
        // 计算到最近观测点的距离
        let min_distance = self.observations.iter()
            .map(|obs| self.parameter_distance(
                quality, effort, lossless,
                obs.quality, obs.effort, obs.lossless
            ))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(100.0);

        // 距离越远，探索奖励越高
        (min_distance / 50.0).min(1.0)
    }

    /// 参数距离
    fn parameter_distance(&self, q1: u32, e1: u32, l1: bool, q2: u32, e2: u32, l2: bool) -> f64 {
        let q_diff = (q1 as f64 - q2 as f64) / 40.0; // 归一化
        let e_diff = (e1 as f64 - e2 as f64) / 5.0;
        let l_diff = if l1 == l2 { 0.0 } else { 1.0 };
        
        (q_diff * q_diff + e_diff * e_diff + l_diff * l_diff).sqrt()
    }

    /// 添加观测结果
    pub fn add_observation(&mut self, obs: Observation) {
        // 更新最佳观测
        if let Some(best) = &self.best_observation {
            if obs.score > best.score {
                self.best_observation = Some(obs.clone());
            }
        } else {
            self.best_observation = Some(obs.clone());
        }

        self.observations.push(obs);
    }

    /// 计算观测得分
    pub fn calculate_score(&self, actual_quality: f64, actual_size: u64, original_size: u64) -> f64 {
        // 质量得分 (SSIM)
        let quality_score = if actual_quality >= self.objective.min_quality {
            actual_quality
        } else {
            // 惩罚低于阈值的质量
            actual_quality * 0.5
        };

        // 大小得分 (压缩率)
        let compression_ratio = actual_size as f64 / original_size as f64;
        let size_score = (1.0 - compression_ratio).max(0.0);

        // 综合得分
        self.objective.quality_weight * quality_score + 
        (1.0 - self.objective.quality_weight) * size_score
    }

    /// 获取最佳参数
    pub fn get_best_parameters(&self) -> Option<(u32, u32, bool)> {
        self.best_observation.as_ref().map(|obs| {
            (obs.quality, obs.effort, obs.lossless)
        })
    }

    /// 获取所有观测
    pub fn get_observations(&self) -> &[Observation] {
        &self.observations
    }

    /// 是否应该停止优化
    pub fn should_stop(&self, max_iterations: usize) -> bool {
        if self.observations.len() >= max_iterations {
            return true;
        }

        // 如果最近5次迭代没有改进，停止
        if self.observations.len() >= 10 {
            let recent = &self.observations[self.observations.len() - 5..];
            let best_recent_score = recent.iter()
                .map(|obs| obs.score)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);
            
            if let Some(best) = &self.best_observation
                && (best.score - best_recent_score).abs() < 0.01 {
                    return true;
                }
        }

        false
    }
}

impl Default for BayesianOptimizer {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayesian_optimizer_creation() {
        let optimizer = BayesianOptimizer::with_defaults();
        assert_eq!(optimizer.observations.len(), 0);
    }

    #[test]
    fn test_suggest_initial_parameters() {
        let optimizer = BayesianOptimizer::with_defaults();
        let (quality, effort, lossless) = optimizer.suggest_next_parameters();
        
        assert!(quality >= 60 && quality <= 100);
        assert!(effort >= 4 && effort <= 9);
        assert!(!lossless); // 初始不尝试无损
    }

    #[test]
    fn test_add_observation_and_best() {
        let mut optimizer = BayesianOptimizer::with_defaults();
        
        let obs1 = Observation {
            quality: 75,
            effort: 6,
            lossless: false,
            actual_quality: 0.96,
            actual_size: 50000,
            score: 0.85,
        };
        
        optimizer.add_observation(obs1);
        assert_eq!(optimizer.observations.len(), 1);
        assert!(optimizer.best_observation.is_some());
    }

    #[test]
    fn test_score_calculation() {
        let optimizer = BayesianOptimizer::with_defaults();
        let score = optimizer.calculate_score(0.96, 50000, 100000);
        
        // 质量好(0.96) + 压缩率好(50%) = 高分
        assert!(score > 0.7);
    }
}
