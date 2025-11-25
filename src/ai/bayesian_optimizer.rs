/**
 * 贝叶斯参数optimizer
 * 
 * 核心target: 保证qualitybefore提下minimum化file大小
 * 
 * algorithm:
 * 1. using高斯过程建模 quality-size 关系
 * 2. usingExpected Improvement (EI) 作for采集function
 * 3. 自适应探索-利用平衡
 */
use serde::{Serialize, Deserialize};

/// optimizationtarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure OptimizationObjective {
 /// minimumqualitythreshold (SSIM)
 pub min_quality: f64,
 /// targetfilesize (bytes, None表示尽may小)
 pub target_size: Option<u64>,
 /// quality权重 (0-1, morehighmore重视quality)
 pub quality_weight: f64,
}

impl Default for OptimizationObjective {
 fn default() -> Self {
 Self {
 min_quality: 0.95, // SSIM >= 0.95
 target_size: None,
 quality_weight: 0.7, // 70%quality, 30%大小
 }
 }
}

/// parameterempty间定义
#[derive(Debug, Clone)]
pub structure ParameterSpace {
 /// qualityrange [min, max]
 pub quality_range: (u32, u32),
 /// speed/effortrange [min, max]
 pub effort_range: (u32, u32),
 /// is否trylossless
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

/// 观测点 (parameter -> result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure Observation {
 /// qualityparameter
 pub quality: u32,
 /// speedparameter
 pub effort: u32,
 /// is否lossless
 pub lossless: bool,
 /// actualquality (SSIM)
 pub actual_quality: f64,
 /// actualsize (bytes)
 pub actual_size: u64,
 /// 综合得分
 pub score: f64,
}

/// 贝叶斯optimization
pub structure BayesianOptimizer {
 /// parameterempty间
 space: ParameterSpace,
 /// optimizationtarget
 objective: OptimizationObjective,
 /// historical观测
 observations: Vec<Observation>,
 /// 最佳观测
 best_observation: Option<Observation>,
}

impl BayesianOptimizer {
 /// createnewoptimization
 pub fn new(space: ParameterSpace, objective: OptimizationObjective) -> Self {
 Self {
 space,
 objective,
 observations: Vec::new(),
 best_observation: None,
 }
 }

 /// usedefaultconfigurationcreate
 pub fn with_defaults() -> Self {
 Self::new(ParameterSpace::default(), OptimizationObjective::default())
 }

 /// suggested下a组parameter (Expected Improvement)
 pub fn suggest_next_parameters(&self) -> (u32, u32, bool) {
 if self.observations.is_empty() {
 // 初始点: etc quality, etc effort
 let quality = (self.space.quality_range.0 + self.space.quality_range.1) / 2;
 let effort = (self.space.effort_range.0 + self.space.effort_range.1) / 2;
 return (quality, effort, false);
 }

 // use Expected Improvementselectnext点
 let mut best_ei = f64::NEG_INFINITY;
 let mut best_params = (75, 6, false);

 // 网格search候选点
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

 /// calculationExpected Improvement
 fn expected_improvement(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
 // prediction该parameter得分
 let predicted_score = self.predict_score(quality, effort, lossless);
 
 // current最佳得分
 let best_score = self.best_observation
 .as_ref()
 .map(|obs| obs.score)
 .unwrap_or(f64::NEG_INFINITY);

 // 简EI: improvement + exploration bonus
 let improvement = (predicted_score - best_score).max(0.0);
 let exploration = self.exploration_bonus(quality, effort, lossless);
 
 improvement + exploration
 }

 /// prediction得分 (usehigh斯过程回归简version)
 fn predict_score(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
 if self.observations.is_empty() {
 return 0.0;
 }

 // use加权average (based ondistance)
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

 /// 探索奖励 (鼓励try未探索区域)
 fn exploration_bonus(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
 // calculationto最近观测点distance
 let min_distance = self.observations.iter()
 .map(|obs| self.parameter_distance(
 quality, effort, lossless,
 obs.quality, obs.effort, obs.lossless
 ))
 .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
 .unwrap_or(100.0);

 // distancemore远，探索奖励morehigh
 (min_distance / 50.0).min(1.0)
 }

 /// parameterdistance
 fn parameter_distance(&self, q1: u32, e1: u32, l1: bool, q2: u32, e2: u32, l2: bool) -> f64 {
 let q_diff = (q1 as f64 - q2 as f64) / 40.0; // 归a化
 let e_diff = (e1 as f64 - e2 as f64) / 5.0;
 let l_diff = if l1 == l2 { 0.0 } else { 1.0 };
 
 (q_diff * q_diff + e_diff * e_diff + l_diff * l_diff).sqrt()
 }

 /// add观测result
 pub fn add_observation(&mut self, obs: Observation) {
 // update最佳观测
 if let Some(best) = &self.best_observation {
 if obs.score > best.score {
 self.best_observation = Some(obs.clone());
 }
 } else {
 self.best_observation = Some(obs.clone());
 }

 self.observations.push(obs);
 }

 /// calculation观测得分
 pub fn calculate_score(&self, actual_quality: f64, actual_size: u64, original_size: u64) -> f64 {
 // quality得分 (SSIM)
 let quality_score = if actual_quality >= self.objective.min_quality {
 actual_quality
 } else {
 // 惩罚belowthresholdquality
 actual_quality * 0.5
 };

 // size得分 (compression率)
 let compression_ratio = actual_size as f64 / original_size as f64;
 let size_score = (1.0 - compression_ratio).max(0.0);

 // 综合得分
 self.objective.quality_weight * quality_score + 
 (1.0 - self.objective.quality_weight) * size_score
 }

 /// get最佳parameter
 pub fn get_best_parameters(&self) -> Option<(u32, u32, bool)> {
 self.best_observation.as_ref().map(|obs| {
 (obs.quality, obs.effort, obs.lossless)
 })
 }

 /// get所 has 观测
 pub fn get_observations(&self) -> &[Observation] {
 &self.observations
 }

 /// is否should停止optimization
 pub fn should_stop(&self, max_iterations: usize) -> bool {
 if self.observations.len() >= max_iterations {
 return true;
 }

 // if最近5次iteration没 has improved，停止
 if self.observations.len() >= 10 {
 let recent = &self.observations[self.observations.len() - 5..];
 let best_recent_score = recent.iter()
 .map(|obs| obs.score)
 .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
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
 
 assert!((60..=100).contains(&quality));
 assert!((4..=9).contains(&effort));
 assert!(!lossless); // 初始nottry无损
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
 
 // quality好(0.96) + compression率好(50%) = high分
 assert!(score > 0.7);
 }
}
