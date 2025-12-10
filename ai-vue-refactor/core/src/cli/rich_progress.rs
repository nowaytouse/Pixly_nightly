//! 🎨 Enhanced command-line progress bar UI
//! 
//! Modern implementation based on indicatif library、beautiful progress bar system。
//! provide：
//! - 🌈 Colorful progress bar and status indicators
//! - 🔄 dynamic Spinner animation
//! - 📊 Real-time throughput and ETA display
//! - 💅 Custom styles for different task types

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::cli::progress::{ProgressCallback, ProgressInfo, ProgressLevel, ProgressState};

/// Enhanced console progress callback
pub struct RichProgressCallback {
    multi_progress: Arc<MultiProgress>,
    progress_bars: Arc<Mutex<Option<ProgressBar>>>,
    current_style: Arc<Mutex<ProgressLevel>>,
}

impl RichProgressCallback {
    pub fn new() -> Self {
        Self {
            multi_progress: Arc::new(MultiProgress::new()),
            progress_bars: Arc::new(Mutex::new(None)),
            current_style: Arc::new(Mutex::new(ProgressLevel::Task)),
        }
    }

 /// Get style template for specific level
    fn get_style_template(level: &ProgressLevel) -> (&'static str, &'static str) {
        match level {
            ProgressLevel::Batch => (
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
                "📦"
            ),
            ProgressLevel::File => (
                "{spinner:.blue} [{elapsed_precise}] [{bar:40.green/white}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta}) {msg}",
                "📄"
            ),
            ProgressLevel::AIPrediction => (
                "{spinner:.magenta} [{elapsed_precise}] {msg} ({eta})",
                "🤖"
            ),
            _ => (
                "{spinner:.yellow} [{elapsed_precise}] [{bar:40.yellow/red}] {pos}/{len} ({eta}) {msg}",
                "⚙️"
            ),
        }
    }

 /// Create or update progress bar
    fn ensure_progress_bar(&self, info: &ProgressInfo) {
        let mut pb_guard = self.progress_bars.lock().unwrap();
        
        if pb_guard.is_none() {
            let (template, emoji) = Self::get_style_template(&info.level);
            
            let pb = if info.total > 0 {
                let pb = self.multi_progress.add(ProgressBar::new(info.total));
                pb.set_style(ProgressStyle::default_bar()
                    .template(template).unwrap()
                    .progress_chars("=>-"));
                pb
            } else {
                let pb = self.multi_progress.add(ProgressBar::new_spinner());
                pb.set_style(ProgressStyle::default_spinner()
                    .template("{spinner:.blue} {msg}").unwrap());
                pb.enable_steady_tick(Duration::from_millis(100));
                pb
            };

            pb.set_message(format!("{} {}", emoji, info.current_operation));
            *pb_guard = Some(pb);
            *self.current_style.lock().unwrap() = info.level.clone();
        }
    }
}

impl ProgressCallback for RichProgressCallback {
    fn on_progress(&self, info: &ProgressInfo) {
        self.ensure_progress_bar(info);
        
        if let Some(pb) = self.progress_bars.lock().unwrap().as_ref() {
            // updateprogress
            if info.total > 0 {
                pb.set_position(info.completed);
            }
            
 // Update message
            let (_, emoji) = Self::get_style_template(&info.level);
            let state_icon = match info.state {
                ProgressState::Running => "",
                ProgressState::Paused => "⏸️ ",
                ProgressState::Retrying => "🔁 ",
                _ => ""
            };
            
            pb.set_message(format!("{}{} {}", state_icon, emoji, info.current_operation));

            // completedstatusprocessing
            match info.state {
                ProgressState::Completed => {
                    pb.finish_with_message(format!("✅ {} Completed", info.current_operation));
                },
                ProgressState::Failed => {
                    pb.abandon_with_message(format!("❌ {} Failed", info.current_operation));
                },
                ProgressState::Cancelled => {
                    pb.abandon_with_message(format!("🚫 {} Cancelled", info.current_operation));
                },
                _ => {}
            }
        }
    }

    fn on_state_change(&self, _old_state: &ProgressState, new_state: &ProgressState, info: &ProgressInfo) {
        if let Some(pb) = self.progress_bars.lock().unwrap().as_ref() {
            match new_state {
                ProgressState::Completed => {
                    pb.finish_with_message(format!("✅ {} Done", info.current_operation));
                },
                ProgressState::Failed => {
                    pb.abandon_with_message(format!("❌ {} Failed", info.current_operation));
                },
                _ => {}
            }
        }
    }
}

impl Default for RichProgressCallback {
    fn default() -> Self {
        Self::new()
    }
}
