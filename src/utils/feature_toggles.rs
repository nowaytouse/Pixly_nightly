// 🎛️ featuremodule
// Unifiedmanagement has helperfeaturestatus

/// featureconfiguration
#[derive(Debug, Clone, Default)]
pub struct FeatureToggles {
// ═══════════════════════════════════════════════════
// 🧠 AI machinelearningfeature
// ═══════════════════════════════════════════════════

/// 🎯 intelligentparameterprediction
 pub enable_ai_prediction: bool,

/// 🔒 AI filevalidation (Magika)
 pub enable_file_validation: bool,

/// 📊 SSIM qualityvalidation
 pub enable_ssim: bool,

/// ⚡ GPU hard
 pub enable_gpu: bool,

/// 🔗 intelligentpreprocessing
 pub enable_preprocess: bool,

/// 🔧 formatautopositive
 pub enable_format_correction: bool,

// ═══════════════════════════════════════════════════
// 🎬 video AI feature
// ═══════════════════════════════════════════════════

/// 🎬 videorecommended
 pub enable_video_for_animation: bool,

/// 🎞️ detection
 pub enable_scene_detection: bool,

/// 📊 VMAF qualityvalidation
 pub enable_vmaf: bool,

/// 🔄 Two-Pass encoding
 pub enable_two_pass: bool,
}

impl FeatureToggles {
/// createnewfeatureconfiguration（ has featuredefaultclose）
 pub fn new() -> Self {
 Self::default()
 }

/// createrecommendedconfiguration（feature）
 pub fn recommended() -> Self {
 Self {
 enable_ai_prediction: true,
 enable_file_validation: true,
 enable_ssim: false, // performancerelativelylarge，defaultclose
 enable_gpu: true,
 enable_preprocess: true,
 enable_format_correction: false, // realfunction
 enable_video_for_animation: true,
 enable_scene_detection: false, // performancerelativelylarge
 enable_vmaf: false, // performancerelativelylarge
 enable_two_pass: false, // whenhigh
 }
 }

/// createmaximumperformanceconfiguration（ has optimizationfeature）
 pub fn max_performance() -> Self {
 Self {
 enable_ai_prediction: true,
 enable_file_validation: true,
 enable_ssim: true,
 enable_gpu: true,
 enable_preprocess: true,
 enable_format_correction: true,
 enable_video_for_animation: true,
 enable_scene_detection: true,
 enable_vmaf: true,
 enable_two_pass: true,
 }
 }

/// checkisno has anywhatAIfeatureenabled
 pub fn has_ai_features(&self) -> bool {
 self.enable_ai_prediction
 || self.enable_file_validation
 || self.enable_preprocess
 || self.enable_format_correction
 }

/// checkisno has anywhatvideo AIfeatureenabled
 pub fn has_video_ai_features(&self) -> bool {
 self.enable_video_for_animation
 || self.enable_scene_detection
 || self.enable_vmaf
 || self.enable_two_pass
 }

/// checkisno has anywhatqualityvalidationfeatureenabled
 pub fn has_quality_checks(&self) -> bool {
 self.enable_ssim || self.enable_vmaf
 }

/// generatefeaturewant
 pub fn summary(&self) -> String {
 let mut features = Vec::new();

 if self.enable_ai_prediction {
 features.push("AI");
 }
 if self.enable_file_validation {
 features.push("FileVal");
 }
 if self.enable_ssim {
 features.push("SSIM");
 }
 if self.enable_gpu {
 features.push("GPU");
 }
 if self.enable_preprocess {
 features.push("Preproc");
 }
 if self.enable_format_correction {
 features.push("FmtCorr");
 }
 if self.enable_video_for_animation {
 features.push("Vid4Anim");
 }
 if self.enable_scene_detection {
 features.push("SceneDet");
 }
 if self.enable_vmaf {
 features.push("VMAF");
 }
 if self.enable_two_pass {
 features.push("2Pass");
 }

 if features.is_empty() {
 "None".to_string()
 } else {
 features.join(", ")
 }
 }

/// validationfeatureconfiguration
 pub fn validate(&self) -> anyhow::Result<()> {
// SSIM and VMAF cannot be enabled simultaneously (performance consideration)
 if self.enable_ssim && self.enable_vmaf {
 anyhow::bail!("SSIM and VMAF cannot be enabled simultaneously (performance impact)");
 }

// Two-Pass requires scene detection support
 if self.enable_two_pass && !self.enable_scene_detection {
 log::warn!("Warning: Two-Pass encoding works best with scene detection enabled");
 }

 Ok(())
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_default_config() {
 let toggles = FeatureToggles::new();
 assert!(!toggles.enable_ai_prediction);
 assert!(!toggles.has_ai_features());
 }

 #[test]
 fn test_recommended_config() {
 let toggles = FeatureToggles::recommended();
 assert!(toggles.enable_ai_prediction);
 assert!(toggles.enable_gpu);
 assert!(toggles.has_ai_features());
 }

 #[test]
 fn test_max_performance_config() {
 let toggles = FeatureToggles::max_performance();
 assert!(toggles.enable_ssim);
 assert!(toggles.enable_two_pass);
 assert!(toggles.has_quality_checks());
 }
}
