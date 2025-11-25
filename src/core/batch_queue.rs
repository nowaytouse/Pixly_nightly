//! 🚀 batchprocessingqueue System
//!
//! providehighbatchimageconversionprocessing：
//! - parallelbatchprocessing
//! - intelligenttaskschedule
//! - progresstrace
//! - errorrecovery

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Instant;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

/// batchprocessingtask
#[derive(Debug, Clone)]
pub struct BatchTask {
/// taskID
 pub id: usize,
/// inputfile
 pub input: PathBuf,
/// outputfile
 pub output: PathBuf,
/// conversionformat
 pub format: String,
/// qualityparameter
 pub quality: Option<u32>,
/// isnoenabled AI
 pub use_ai: bool,
}

/// taskstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
/// wait
 Pending,
/// processing
 Processing,
/// alreadycompleted
 Completed,
/// failure
 Failed,
/// alreadyskip
 Skipped,
}

/// taskresult
#[derive(Debug, Clone)]
pub struct TaskResult {
/// taskID
 pub task_id: usize,
/// status
 pub status: TaskStatus,
/// inputfile
 pub input: PathBuf,
/// outputfile
 pub output: Option<PathBuf>,
/// processingtime（）
 pub duration_ms: u64,
/// originalsize
 pub original_size: u64,
/// outputsize
 pub output_size: Option<u64>,
/// compression
 pub compression_ratio: Option<f64>,
/// errorinformation
 pub error: Option<String>,
}

/// batchprocessingprogress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProgress {
/// task
 pub total: usize,
/// alreadycompleted
 pub completed: usize,
/// failure
 pub failed: usize,
/// skip
 pub skipped: usize,
/// currentprocessing
 pub processing: usize,
/// completedpercentage
 pub percent: f64,
/// alreadytime（）
 pub elapsed_secs: f64,
/// time（）
 pub eta_secs: Option<f64>,
/// averageprocessingspeed（task/）
 pub avg_speed: f64,
}

/// batchprocessingconfiguration
#[derive(Debug, Clone)]
pub struct BatchConfig {
/// concurrent
 pub concurrency: usize,
/// failure when isnocontinue
 pub continue_on_error: bool,
/// isnodisplayprogress
 pub show_progress: bool,
/// isnosavecheckpoint
 pub checkpoint: bool,
}

impl Default for BatchConfig {
 fn default() -> Self {
 Self {
 concurrency: num_cpus::get(),
 continue_on_error: true,
 show_progress: true,
 checkpoint: false,
 }
 }
}

/// batchprocessingqueue
pub struct BatchQueue {
 tasks: Vec<BatchTask>,
 results: Arc<Mutex<Vec<TaskResult>>>,
 config: BatchConfig,
 start_time: Option<Instant>,
}

impl BatchQueue {
/// createnewbatchprocessingqueue
 pub fn new(config: BatchConfig) -> Self {
 Self {
 tasks: Vec::new(),
 results: Arc::new(Mutex::new(Vec::new())),
 config,
 start_time: None,
 }
 }

/// addtask
 pub fn add_task(&mut self, task: BatchTask) {
 self.tasks.push(task);
 }

/// batchaddtask
 pub fn add_tasks(&mut self, tasks: Vec<BatchTask>) {
 self.tasks.extend(tasks);
 }

/// gettaskcount
 pub fn task_count(&self) -> usize {
 self.tasks.len()
 }

/// executebatchprocessing
 pub fn execute<F>(&mut self, processor: F) -> Result<Vec<TaskResult>>
 where
 F: Fn(&BatchTask) -> Result<TaskResult> + Send + Sync + 'static,
 {
 if self.tasks.is_empty() {
 return Ok(Vec::new());
 }

 self.start_time = Some(Instant::now());
 let total_tasks = self.tasks.len();
 let concurrency = self.config.concurrency.min(total_tasks);

 log::info!("🚀 Starting batch processing: {} tasks with {} workers",
 total_tasks, concurrency);

// createtaskchannel
 let (task_tx, task_rx): (Sender<BatchTask>, Receiver<BatchTask>) = channel();
 let (result_tx, result_rx): (Sender<TaskResult>, Receiver<TaskResult>) = channel();

// send has task
 for task in self.tasks.drain(..) {
 task_tx.send(task).context("Failed to send task")?;
 }
 drop(task_tx); // closesendend

// startworkthread
 let task_rx = Arc::new(Mutex::new(task_rx));
 let processor = Arc::new(processor);
 let mut handles = Vec::new();

 for worker_id in 0..concurrency {
 let task_rx = Arc::clone(&task_rx);
 let result_tx = result_tx.clone();
 let processor = Arc::clone(&processor);
 let continue_on_error = self.config.continue_on_error;

 let handle = thread::spawn(move || {
 loop {
// getnexttask
 let task = {
 let rx = task_rx.lock().unwrap();
 rx.recv()
 };

 match task {
 Ok(task) => {
 let task_id = task.id;
 log::debug!("Worker #{} processing task #{}: {:?}",
 worker_id, task_id, task.input);

// processingtask
 let result = processor(&task);

 match result {
 Ok(mut r) => {
 r.task_id = task_id;
 if let Err(e) = result_tx.send(r) {
 log::error!("Failed to send result for task #{}: {}", task_id, e);
 }
 }
 Err(e) => {
 log::error!("Task #{} failed: {}", task_id, e);

 let error_result = TaskResult {
 task_id,
 status: TaskStatus::Failed,
 input: task.input.clone(),
 output: None,
 duration_ms: 0,
 original_size: 0,
 output_size: None,
 compression_ratio: None,
 error: Some(e.to_string()),
 };

 if let Err(e) = result_tx.send(error_result) {
 log::error!("Failed to send error result: {}", e);
 }

 if !continue_on_error {
 break;
 }
 }
 }
 }
 Err(_) => {
// channelclose，exit
 break;
 }
 }
 }

 log::debug!("Worker #{} finished", worker_id);
 });

 handles.push(handle);
 }

 drop(result_tx); // closeresultsendend

// resultanddisplayprogress
 let results = Arc::clone(&self.results);
 let show_progress = self.config.show_progress;
 let start_time = self.start_time.unwrap();

 let progress_handle = thread::spawn(move || {
 let mut collected = Vec::new();
 let mut last_progress_time = Instant::now();

 for result in result_rx {
 collected.push(result.clone());

 if show_progress && last_progress_time.elapsed().as_millis() > 500 {
 let progress = calculate_progress(
 total_tasks,
 &collected,
 start_time.elapsed().as_secs_f64()
 );

 print_progress(&progress);
 last_progress_time = Instant::now();
 }

 results.lock().unwrap().push(result);
 }

// mostprogress
 if show_progress {
 let final_progress = calculate_progress(
 total_tasks,
 &collected,
 start_time.elapsed().as_secs_f64()
 );
 print_progress(&final_progress);
 println!(); // New line
 }
 });

// wait has workthreadcompleted
 for handle in handles {
 handle.join().expect("Worker thread panicked");
 }

// waitprogresscompleted
 progress_handle.join().expect("Progress thread panicked");

 let final_results = self.results.lock().unwrap().clone();

// printwant
 self.print_summary(&final_results);

 Ok(final_results)
 }

/// getprogress
 pub fn get_progress(&self) -> BatchProgress {
 let results = self.results.lock().unwrap();
 let elapsed = self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0);

 calculate_progress(self.tasks.len(), &results, elapsed)
 }

/// printwant
 fn print_summary(&self, results: &[TaskResult]) {
 let total = results.len();
 let completed = results.iter().filter(|r| r.status == TaskStatus::Completed).count();
 let failed = results.iter().filter(|r| r.status == TaskStatus::Failed).count();
 let skipped = results.iter().filter(|r| r.status == TaskStatus::Skipped).count();

 let total_time = self.start_time.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0);
 let avg_time = if completed > 0 {
 results.iter()
 .filter(|r| r.status == TaskStatus::Completed)
 .map(|r| r.duration_ms as f64)
 .sum::<f64>() / completed as f64
 } else {
 0.0
 };

 let total_saved = results.iter()
 .filter_map(|r| {
 if r.status == TaskStatus::Completed {
 r.compression_ratio.map(|ratio| {
 r.original_size as f64 * (1.0 - ratio)
 })
 } else {
 None
 }
 })
 .sum::<f64>();

 println!("\n📊 Batch Processing Summary");
 println!("─────────────────────────────");
 println!("Total tasks: {}", total);
 println!("✅ Succeeded: {} ({:.1}%)", completed, (completed as f64 / total as f64) * 100.0);
 println!("❌ Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
 if skipped > 0 {
 println!("⏭️ Skipped: {}", skipped);
 }
 println!("⏱️ Total time: {:.2}s", total_time);
 println!("📈 Average time: {:.0}ms/task", avg_time);
 println!("💾 Space saved: {:.2}MB", total_saved / (1024.0 * 1024.0));
 }
}

/// calculationprogress
fn calculate_progress(
 total: usize,
 results: &[TaskResult],
 elapsed_secs: f64,
) -> BatchProgress {
 let completed = results.iter().filter(|r| r.status == TaskStatus::Completed).count();
 let failed = results.iter().filter(|r| r.status == TaskStatus::Failed).count();
 let skipped = results.iter().filter(|r| r.status == TaskStatus::Skipped).count();
 let processing = total - results.len();

 let percent = if total > 0 {
 (results.len() as f64 / total as f64) * 100.0
 } else {
 0.0
 };

 let avg_speed = if elapsed_secs > 0.0 {
 results.len() as f64 / elapsed_secs
 } else {
 0.0
 };

 let eta_secs = if avg_speed > 0.0 && processing > 0 {
 Some((total - results.len()) as f64 / avg_speed)
 } else {
 None
 };

 BatchProgress {
 total,
 completed,
 failed,
 skipped,
 processing,
 percent,
 elapsed_secs,
 eta_secs,
 avg_speed,
 }
}

/// printprogress
fn print_progress(progress: &BatchProgress) {
 let bar_width = 40;
 let filled = (progress.percent / 100.0 * bar_width as f64) as usize;
 let empty = bar_width - filled;

 let bar = format!("[{}{}]",
 "█".repeat(filled),
 "░".repeat(empty)
 );

 let eta_str = if let Some(eta) = progress.eta_secs {
 format!("ETA: {:.0}s", eta)
 } else {
 "ETA: --".to_string()
 };

 print!("\r{} {:.1}% | {}/{} | {:.1} tasks/s | {} ",
 bar,
 progress.percent,
 progress.completed + progress.failed + progress.skipped,
 progress.total,
 progress.avg_speed,
 eta_str
 );

 use std::io::{self, Write};
 io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
 use super::*;
 use std::thread::sleep;
 use std::time::Duration;

 #[test]
 fn test_batch_queue_creation() {
 let config = BatchConfig::default();
 let queue = BatchQueue::new(config);
 assert_eq!(queue.task_count(), 0);
 }

 #[test]
 fn test_add_tasks() {
 let config = BatchConfig::default();
 let mut queue = BatchQueue::new(config);

 queue.add_task(BatchTask {
 id: 1,
 input: PathBuf::from("test1.png"),
 output: PathBuf::from("test1.webp"),
 format: "webp".to_string(),
 quality: Some(85),
 use_ai: false,
 });

 assert_eq!(queue.task_count(), 1);
 }

 #[test]
 fn test_batch_execution() {
 let config = BatchConfig {
 concurrency: 2,
 continue_on_error: true,
 show_progress: false,
 checkpoint: false,
 };

 let mut queue = BatchQueue::new(config);

// addtesttask
 for i in 0..5 {
 queue.add_task(BatchTask {
 id: i,
 input: PathBuf::from(format!("test{}.png", i)),
 output: PathBuf::from(format!("test{}.webp", i)),
 format: "webp".to_string(),
 quality: Some(85),
 use_ai: false,
 });
 }

// simulatedhandler
 let processor = |task: &BatchTask| -> Result<TaskResult> {
 sleep(Duration::from_millis(10)); // processwhen

 Ok(TaskResult {
 task_id: task.id,
 status: TaskStatus::Completed,
 input: task.input.clone(),
 output: Some(task.output.clone()),
 duration_ms: 10,
 original_size: 1000,
 output_size: Some(500),
 compression_ratio: Some(0.5),
 error: None,
 })
 };

 let results = queue.execute(processor).unwrap();

 assert_eq!(results.len(), 5);
 assert_eq!(results.iter().filter(|r| r.status == TaskStatus::Completed).count(), 5);
 }
}
