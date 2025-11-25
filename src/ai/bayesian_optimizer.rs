/**
 * parameteroptimizer
 *
 * coretarget: qualitybeforedownminimumfilesize
 *
 * algorithm:
 * 1. usinghigh quality-size 
 * 2. usingExpected Improvement (EI) forfunction
 * 3. shouldexploration-exploitation
 */
use serde::{Serialize, Deserialize};

/// optimizationtarget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationObjective {
/// minimumqualitythreshold (SSIM)
 pub min_quality: f64,
/// targetfilesize (bytes, Nonemaysmall)
 pub target_size: Option<u64>,
/// qualityweight (0-1, morehighmoreheavyquality)
 pub quality_weight: f64,
}

impl Default for OptimizationObjective {
 fn default() -> Self {
 Self {
 min_quality: 0.95, // SSIM >= 0.95
 target_size: None,
 quality_weight: 0.7, // 70%quality, 30%size
 }
 }
}

/// parameterempty
#[derive(Debug, Clone)]
pub struct ParameterSpace {
/// qualityrange [min, max]
 pub quality_range: (u32, u32),
/// speed/effortrange [min, max]
 pub effort_range: (u32, u32),
/// isnotrylossless
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

/// point (parameter -> result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
/// qualityparameter
 pub quality: u32,
/// speedparameter
 pub effort: u32,
/// isnolossless
 pub lossless: bool,
/// actualquality (SSIM)
 pub actual_quality: f64,
/// actualsize (bytes)
 pub actual_size: u64,
/// comprehensive
 pub score: f64,
}

/// optimization
pub struct BayesianOptimizer {
/// parameterempty
 space: ParameterSpace,
/// optimizationtarget
 objective: OptimizationObjective,
/// historical
 observations: Vec<Observation>,
/// most
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

/// suggesteddownagroupparameter (Expected Improvement)
 pub fn suggest_next_parameters(&self) -> (u32, u32, bool) {
 if self.observations.is_empty() {
// point: etc quality, etc effort
 let quality = (self.space.quality_range.0 + self.space.quality_range.1) / 2;
 let effort = (self.space.effort_range.0 + self.space.effort_range.1) / 2;
 return (quality, effort, false);
 }

// use Expected Improvementselectnextpoint
 let mut best_ei = f64::NEG_INFINITY;
 let mut best_params = (75, 6, false);

// searchpoint
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
// predictionparameter
 let predicted_score = self.predict_score(quality, effort, lossless);

// currentmost
 let best_score = self.best_observation
 .as_ref()
 .map(|obs| obs.score)
 .unwrap_or(f64::NEG_INFINITY);

// EI: improvement + exploration bonus
 let improvement = (predicted_score - best_score).max(0.0);
 let exploration = self.exploration_bonus(quality, effort, lossless);

 improvement + exploration
 }

/// prediction (usehighregressionversion)
 fn predict_score(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
 if self.observations.is_empty() {
 return 0.0;
 }

// useaverage (based ondistance)
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

/// explorationreward (trynot yetexploration)
 fn exploration_bonus(&self, quality: u32, effort: u32, lossless: bool) -> f64 {
// calculationtomostnearpointdistance
 let min_distance = self.observations.iter()
 .map(|obs| self.parameter_distance(
 quality, effort, lossless,
 obs.quality, obs.effort, obs.lossless
 ))
 .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
 .unwrap_or(100.0);

// distancemorefar，explorationrewardmorehigh
 (min_distance / 50.0).min(1.0)
 }

/// parameterdistance
 fn parameter_distance(&self, q1: u32, e1: u32, l1: bool, q2: u32, e2: u32, l2: bool) -> f64 {
 let q_diff = (q1 as f64 - q2 as f64) / 40.0; // a
 let e_diff = (e1 as f64 - e2 as f64) / 5.0;
 let l_diff = if l1 == l2 { 0.0 } else { 1.0 };

 (q_diff * q_diff + e_diff * e_diff + l_diff * l_diff).sqrt()
 }

/// addresult
 pub fn add_observation(&mut self, obs: Observation) {
// updatemost
 if let Some(best) = &self.best_observation {
 if obs.score > best.score {
 self.best_observation = Some(obs.clone());
 }
 } else {
 self.best_observation = Some(obs.clone());
 }

 self.observations.push(obs);
 }

/// calculation
 pub fn calculate_score(&self, actual_quality: f64, actual_size: u64, original_size: u64) -> f64 {
// quality (SSIM)
 let quality_score = if actual_quality >= self.objective.min_quality {
 actual_quality
 } else {
// penaltybelowthresholdquality
 actual_quality * 0.5
 };

// size (compression)
 let compression_ratio = actual_size as f64 / original_size as f64;
 let size_score = (1.0 - compression_ratio).max(0.0);

// comprehensive
 self.objective.quality_weight * quality_score +
 (1.0 - self.objective.quality_weight) * size_score
 }

/// getmostparameter
 pub fn get_best_parameters(&self) -> Option<(u32, u32, bool)> {
 self.best_observation.as_ref().map(|obs| {
 (obs.quality, obs.effort, obs.lossless)
 })
 }

/// get has 
 pub fn get_observations(&self) -> &[Observation] {
 &self.observations
 }

/// isnoshouldstopoptimization
 pub fn should_stop(&self, max_iterations: usize) -> bool {
 if self.observations.len() >= max_iterations {
 return true;
 }

// ifmostnear5timesiteration has improved，stop
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
 assert!(!lossless); // nottry
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

// quality(0.96) + compression(50%) = high
 assert!(score > 0.7);
 }
}
