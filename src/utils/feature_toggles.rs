// 🎛️ feature开关module
// Unifiedmanagement所 has helperfeature开关status

/// feature开关configuration
#[derive(Debug, Clone, Default)]
pub structure FeatureToggles {
 // ═══════════════════════════════════════════════════
 // 🧠 AI 机学习feature
 // ═══════════════════════════════════════════════════
 
 /// 🎯 intelligentparameterprediction
 pub enable_ai_prediction: bool,
 
 /// 🔒 AI filevalidation (Magika)
 pub enable_file_validation: bool,
 
 /// 📊 SSIM qualityvalidation
 pub enable_ssim: bool,
 
 /// ⚡ GPU 硬件加速
 pub enable_gpu: bool,
 
 /// 🔗 intelligentpreprocessing
 pub enable_preprocess: bool,
 
 /// 🔧 formatauto修正
 pub enable_format_correction: bool,
 
 // ═══════════════════════════════════════════════════
 // 🎬 video AI feature
 // ═══════════════════════════════════════════════════
 
 /// 🎬 动图转videorecommended
 pub enable_video_for_animation: bool,
 
 /// 🎞️ 场景detection
 pub enable_scene_detection: bool,
 
 /// 📊 VMAF qualityvalidation
 pub enable_vmaf: bool,
 
 /// 🔄 Two-Pass encoding
 pub enable_two_pass: bool,
}

impl FeatureToggles {
 /// createnewfeature开关configuration（所 has featuredefault关闭）
 pub fn new() -> Self {
 Self::default()
 }
 
 /// createrecommendedconfiguration（常用feature开启）
 pub fn recommended() -> Self {
 Self {
 enable_ai_prediction: true,
 enable_file_validation: true,
 enable_ssim: false, // performance影响较大，default关闭
 enable_gpu: true,
 enable_preprocess: true,
 enable_format_correction: false, // 实验性功能
 enable_video_for_animation: true,
 enable_scene_detection: false, // performance影响较大
 enable_vmaf: false, // performance影响较大
 enable_two_pass: false, // when间成本高
 }
 }
 
 /// createmaximumperformanceconfiguration（所 has optimizationfeature开启）
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
 
 /// checkis否 has 任何AIfeatureenabled
 pub fn has_ai_features(&self) -> bool {
 self.enable_ai_prediction
 || self.enable_file_validation
 || self.enable_preprocess
 || self.enable_format_correction
 }
 
 /// checkis否 has 任何video AIfeatureenabled
 pub fn has_video_ai_features(&self) -> bool {
 self.enable_video_for_animation
 || self.enable_scene_detection
 || self.enable_vmaf
 || self.enable_two_pass
 }
 
 /// checkis否 has 任何qualityvalidationfeatureenabled
 pub fn has_quality_checks(&self) -> bool {
 self.enable_ssim || self.enable_vmaf
 }
 
 /// generatefeature摘要
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
 
 /// validationfeature开关configuration
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
