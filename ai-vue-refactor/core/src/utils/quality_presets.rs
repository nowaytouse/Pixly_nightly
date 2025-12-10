// src/quality_presets.rs
//! 🎯 CLI-001: qualitypresetSystem
//!
//! providequalityconfiguration，select

use serde::{Serialize, Deserialize};

/// qualitypresetlevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityPreset {
/// draftquality - mostfastspeed，minimumfile
 Draft,
/// standardquality - balancedspeed and quality
 Standard,
/// highquality - priorityquality
 High,
/// highest quality - nearlossless
 Maximum,
}

/// presetconfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetConfig {
 pub quality: u32,
 pub effort: u32,
 pub description: &'static str,
}

impl QualityPreset {
/// get has preset
 pub fn all() -> Vec<Self> {
 vec![
 Self::Draft,
 Self::Standard,
 Self::High,
 Self::Maximum,
 ]
 }

/// fromstringparse（note： not isstd::str::From Str trait）
 pub fn parse_preset(s: &str) -> Option<Self> {
 match s.to_lowercase().as_str() {
 "draft" | "d" => Some(Self::Draft),
 "standard" | "std" | "s" => Some(Self::Standard),
 "high" | "h" => Some(Self::High),
 "maximum" | "max" | "m" => Some(Self::Maximum),
 _ => None,
 }
 }

/// forstring
 pub fn as_str(&self) -> &'static str {
 match self {
 Self::Draft => "draft",
 Self::Standard => "standard",
 Self::High => "high",
 Self::Maximum => "maximum",
 }
 }

/// getWebPconfiguration
 pub fn webp_config(&self) -> PresetConfig {
 match self {
 Self::Draft => PresetConfig {
 quality: 70,
 effort: 3,
 description: "Draft: Fast encoding, smaller files",
 },
 Self::Standard => PresetConfig {
 quality: 80,
 effort: 4,
 description: "Standard: Balanced quality and speed",
 },
 Self::High => PresetConfig {
 quality: 90,
 effort: 5,
 description: "High: Better quality, slower encoding",
 },
 Self::Maximum => PresetConfig {
 quality: 95,
 effort: 6,
 description: "Maximum: Near-lossless quality",
 },
 }
 }

/// getAVIFconfiguration
 pub fn avif_config(&self) -> PresetConfig {
 match self {
 Self::Draft => PresetConfig {
 quality: 65,
 effort: 3,
 description: "Draft: Fast encoding, smaller files",
 },
 Self::Standard => PresetConfig {
 quality: 75,
 effort: 4,
 description: "Standard: Balanced quality and speed",
 },
 Self::High => PresetConfig {
 quality: 85,
 effort: 6,
 description: "High: Better quality, slower encoding",
 },
 Self::Maximum => PresetConfig {
 quality: 92,
 effort: 8,
 description: "Maximum: Near-lossless quality",
 },
 }
 }

/// getJXLconfiguration
 pub fn jxl_config(&self) -> PresetConfig {
 match self {
 Self::Draft => PresetConfig {
 quality: 75,
 effort: 5,
 description: "Draft: Fast encoding",
 },
 Self::Standard => PresetConfig {
 quality: 85,
 effort: 7,
 description: "Standard: Balanced quality",
 },
 Self::High => PresetConfig {
 quality: 92,
 effort: 8,
 description: "High: Better quality",
 },
 Self::Maximum => PresetConfig {
 quality: 98,
 effort: 9,
 description: "Maximum: Near-lossless",
 },
 }
 }

/// based onformatgetconfiguration
 pub fn config_for_format(&self, format: &str) -> PresetConfig {
 match format.to_lowercase().as_str() {
 "webp" => self.webp_config(),
 "avif" => self.avif_config(),
 "jxl" => self.jxl_config(),
 "jpeg" | "jpg" => PresetConfig {
 quality: match self {
 Self::Draft => 75,
 Self::Standard => 85,
 Self::High => 92,
 Self::Maximum => 98,
 },
 effort: 0, // JPEGnotusingeffort
 description: match self {
 Self::Draft => "Draft: Smaller files",
 Self::Standard => "Standard: Balanced",
 Self::High => "High: Better quality",
 Self::Maximum => "Maximum: Near-lossless",
 },
 },
 "png" => PresetConfig {
 quality: match self {
 Self::Draft => 70,
 Self::Standard => 80,
 Self::High => 90,
 Self::Maximum => 95,
 },
 effort: match self {
 Self::Draft => 1,
 Self::Standard => 3,
 Self::High => 5,
 Self::Maximum => 9,
 },
 description: match self {
 Self::Draft => "Draft: Fast compression",
 Self::Standard => "Standard: Balanced",
 Self::High => "High: Better compression",
 Self::Maximum => "Maximum: Best compression",
 },
 },
 _ => self.webp_config(), // defaultusingWebPconfig
 }
 }

/// display has presetinformation
 pub fn display_all() {
 println!("📊 Available Quality Presets:\n");

 for preset in Self::all() {
 println!(" {} - {}", preset.as_str(), preset.description());
 println!(" WebP: Q{}, E{}",
 preset.webp_config().quality,
 preset.webp_config().effort);
 println!(" AVIF: Q{}, E{}",
 preset.avif_config().quality,
 preset.avif_config().effort);
 println!(" JXL: Q{}, E{}",
 preset.jxl_config().quality,
 preset.jxl_config().effort);
 println!();
 }
 }

/// getdescription
 fn description(&self) -> &'static str {
 match self {
 Self::Draft => "Fast encoding, smaller files",
 Self::Standard => "Balanced quality and speed",
 Self::High => "Better quality, slower encoding",
 Self::Maximum => "Near-lossless quality",
 }
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_preset_parsing() {
 assert_eq!(QualityPreset::parse_preset("draft"), Some(QualityPreset::Draft));
 assert_eq!(QualityPreset::parse_preset("std"), Some(QualityPreset::Standard));
 assert_eq!(QualityPreset::parse_preset("high"), Some(QualityPreset::High));
 assert_eq!(QualityPreset::parse_preset("max"), Some(QualityPreset::Maximum));
 }

 #[test]
 fn test_webp_config() {
 let config = QualityPreset::Standard.webp_config();
 assert_eq!(config.quality, 80);
 assert_eq!(config.effort, 4);
 }

 #[test]
 fn test_format_config() {
 let config = QualityPreset::High.config_for_format("avif");
 assert_eq!(config.quality, 85);
 assert_eq!(config.effort, 6);
 }
}
