/*
⚡ PIXLY v3.1 进程池管理器

高性能进程池，支持并发转换任务：
- 异步进程执行
- 智能负载均衡
- 进程复用优化
- 资源限制管理
- 错误恢复机制

确保Rust作为唯一执行层的高效率运行
*/

use std::collections::VecDeque;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{timeout, Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub executable: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub timeout_seconds: u64,
    pub capture_output: bool,
    pub environment_vars: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u64,
    pub process_id: Option<u32>,
}

#[derive(Debug, Clone)]
struct ProcessSlot {
    pub id: usize,
    pub in_use: bool,
    pub last_used: Instant,
    pub total_executions: u64,
    pub success_count: u64,
    pub error_count: u64,
}

impl ProcessSlot {
    fn new(id: usize) -> Self {
        Self {
            id,
            in_use: false,
            last_used: Instant::now(),
            total_executions: 0,
            success_count: 0,
            error_count: 0,
        }
    }
    
    fn success_rate(&self) -> f32 {
        if self.total_executions == 0 {
            1.0
        } else {
            self.success_count as f32 / self.total_executions as f32
        }
    }
}

pub struct ProcessPool {
    slots: Arc<Mutex<Vec<ProcessSlot>>>,
    semaphore: Arc<Semaphore>,
    max_processes: usize,
    pending_commands: Arc<Mutex<VecDeque<CommandRequest>>>,
    pool_stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug, Clone, Default)]
struct PoolStats {
    total_commands_executed: u64,
    total_execution_time_ms: u64,
    active_processes: usize,
    peak_concurrent_processes: usize,
    average_wait_time_ms: f32,
    pool_efficiency: f32,
}

impl ProcessPool {
    pub fn new(max_processes: usize) -> Self {
        let mut slots = Vec::with_capacity(max_processes);
        for i in 0..max_processes {
            slots.push(ProcessSlot::new(i));
        }

        info!("⚡ 进程池初始化: 最大进程数={}", max_processes);

        Self {
            slots: Arc::new(Mutex::new(slots)),
            semaphore: Arc::new(Semaphore::new(max_processes)),
            max_processes,
            pending_commands: Arc::new(Mutex::new(VecDeque::new())),
            pool_stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }

    pub async fn execute_command(&self, request: CommandRequest) -> Result<CommandResult> {
        let start_time = Instant::now();
        
        // 获取进程槽
        let _permit = self.semaphore.acquire().await?;
        let wait_time = start_time.elapsed();
        
        // 更新等待时间统计
        self.update_wait_time_stats(wait_time.as_millis() as f32).await;
        
        // 分配进程槽
        let slot_id = self.allocate_slot().await?;
        
        debug!("🔄 执行命令: {} (槽位: {})", request.executable, slot_id);
        
        // 执行命令
        let result = self.execute_with_slot(request, slot_id).await;
        
        // 释放进程槽
        self.release_slot(slot_id, &result).await;
        
        // 更新全局统计
        self.update_pool_stats(&result, start_time.elapsed()).await;
        
        result
    }

    async fn allocate_slot(&self) -> Result<usize> {
        let mut slots = self.slots.lock().await;
        
        // 寻找空闲槽位
        for slot in slots.iter_mut() {
            if !slot.in_use {
                slot.in_use = true;
                slot.last_used = Instant::now();
                return Ok(slot.id);
            }
        }
        
        Err(anyhow!("没有可用的进程槽位"))
    }

    async fn release_slot(&self, slot_id: usize, result: &Result<CommandResult>) {
        let mut slots = self.slots.lock().await;
        
        if let Some(slot) = slots.get_mut(slot_id) {
            slot.in_use = false;
            slot.total_executions += 1;
            
            match result {
                Ok(cmd_result) if cmd_result.success => {
                    slot.success_count += 1;
                }
                _ => {
                    slot.error_count += 1;
                }
            }
        }
    }

    async fn execute_with_slot(&self, request: CommandRequest, slot_id: usize) -> Result<CommandResult> {
        let execution_start = Instant::now();
        
        // 构建命令
        let mut command = Command::new(&request.executable);
        command.args(&request.args);
        
        // 设置工作目录
        if let Some(working_dir) = &request.working_dir {
            command.current_dir(working_dir);
        }
        
        // 设置环境变量
        for (key, value) in &request.environment_vars {
            command.env(key, value);
        }
        
        // 配置输出捕获
        if request.capture_output {
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());
        } else {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }
        
        // 执行命令（带超时）
        let timeout_duration = Duration::from_secs(request.timeout_seconds);
        let execution_future = command.output();
        
        match timeout(timeout_duration, execution_future).await {
            Ok(Ok(output)) => {
                let execution_time = execution_start.elapsed();
                
                let result = CommandResult {
                    success: output.status.success(),
                    exit_code: output.status.code().unwrap_or(-1),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    execution_time_ms: execution_time.as_millis() as u64,
                    process_id: None, // 进程已结束，无法获取PID
                };
                
                if result.success {
                    debug!("✅ 命令执行成功: {} ({}ms, 槽位: {})", 
                           request.executable, result.execution_time_ms, slot_id);
                } else {
                    warn!("⚠️ 命令执行失败: {} 退出码: {} (槽位: {})", 
                          request.executable, result.exit_code, slot_id);
                }
                
                Ok(result)
            }
            Ok(Err(e)) => {
                error!("❌ 命令启动失败: {} - {}", request.executable, e);
                Err(anyhow!("命令启动失败: {}", e))
            }
            Err(_) => {
                error!("⏰ 命令执行超时: {} ({}s)", request.executable, request.timeout_seconds);
                Err(anyhow!("命令执行超时"))
            }
        }
    }

    async fn update_wait_time_stats(&self, wait_time_ms: f32) {
        let mut stats = self.pool_stats.lock().await;
        
        // 更新平均等待时间（移动平均）
        if stats.average_wait_time_ms == 0.0 {
            stats.average_wait_time_ms = wait_time_ms;
        } else {
            stats.average_wait_time_ms = stats.average_wait_time_ms * 0.9 + wait_time_ms * 0.1;
        }
    }

    async fn update_pool_stats(&self, result: &Result<CommandResult>, total_duration: Duration) {
        let mut stats = self.pool_stats.lock().await;
        
        stats.total_commands_executed += 1;
        stats.total_execution_time_ms += total_duration.as_millis() as u64;
        
        // 更新当前活跃进程数
        let active_count = self.semaphore.available_permits();
        stats.active_processes = self.max_processes - active_count;
        
        // 更新峰值并发数
        if stats.active_processes > stats.peak_concurrent_processes {
            stats.peak_concurrent_processes = stats.active_processes;
        }
        
        // 计算池效率（成功率 × 利用率）
        let utilization = stats.active_processes as f32 / self.max_processes as f32;
        let success_rate = if let Ok(cmd_result) = result {
            if cmd_result.success { 1.0 } else { 0.0 }
        } else {
            0.0
        };
        
        stats.pool_efficiency = (stats.pool_efficiency * 0.95 + (success_rate * utilization) * 0.05)
            .min(1.0).max(0.0);
    }

    pub async fn execute_batch(&self, requests: Vec<CommandRequest>) -> Vec<Result<CommandResult>> {
        info!("🔄 批量执行命令: {} 个任务", requests.len());
        
        let mut handles = Vec::new();
        
        for request in requests {
            let pool = self.clone();
            let handle = tokio::spawn(async move {
                pool.execute_command(request).await
            });
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("批量任务执行失败: {}", e);
                    results.push(Err(anyhow!("任务执行失败: {}", e)));
                }
            }
        }
        
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!("✅ 批量执行完成: {}/{} 成功", success_count, results.len());
        
        results
    }

    pub async fn get_pool_stats(&self) -> PoolStats {
        self.pool_stats.lock().await.clone()
    }

    pub async fn get_slot_stats(&self) -> Vec<SlotStats> {
        let slots = self.slots.lock().await;
        
        slots.iter().map(|slot| SlotStats {
            id: slot.id,
            in_use: slot.in_use,
            total_executions: slot.total_executions,
            success_rate: slot.success_rate(),
            idle_time_seconds: slot.last_used.elapsed().as_secs(),
        }).collect()
    }

    pub async fn health_check(&self) -> PoolHealthStatus {
        let stats = self.get_pool_stats().await;
        let slot_stats = self.get_slot_stats().await;
        
        let available_slots = self.semaphore.available_permits();
        let busy_slots = self.max_processes - available_slots;
        
        // 计算健康评分
        let efficiency_score = stats.pool_efficiency;
        let utilization_score = busy_slots as f32 / self.max_processes as f32;
        let avg_success_rate = slot_stats.iter()
            .map(|s| s.success_rate)
            .sum::<f32>() / slot_stats.len() as f32;
        
        let health_score = (efficiency_score * 0.4 + utilization_score * 0.3 + avg_success_rate * 0.3)
            .min(1.0).max(0.0);
        
        let health_status = if health_score > 0.8 {
            "excellent"
        } else if health_score > 0.6 {
            "good"
        } else if health_score > 0.4 {
            "fair"
        } else {
            "poor"
        };
        
        PoolHealthStatus {
            overall_health: health_status.to_string(),
            health_score,
            available_slots,
            busy_slots,
            total_slots: self.max_processes,
            average_wait_time_ms: stats.average_wait_time_ms,
            pool_efficiency: stats.pool_efficiency,
            peak_concurrent_processes: stats.peak_concurrent_processes,
        }
    }

    pub async fn cleanup_idle_resources(&self) {
        // 这里可以实现清理空闲资源的逻辑
        // 比如清理临时文件、释放内存等
        debug!("🧹 清理空闲资源");
    }

    pub fn get_max_processes(&self) -> usize {
        self.max_processes
    }

    pub async fn get_available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

// 实现Clone以支持跨任务传递
impl Clone for ProcessPool {
    fn clone(&self) -> Self {
        Self {
            slots: Arc::clone(&self.slots),
            semaphore: Arc::clone(&self.semaphore),
            max_processes: self.max_processes,
            pending_commands: Arc::clone(&self.pending_commands),
            pool_stats: Arc::clone(&self.pool_stats),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SlotStats {
    pub id: usize,
    pub in_use: bool,
    pub total_executions: u64,
    pub success_rate: f32,
    pub idle_time_seconds: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolHealthStatus {
    pub overall_health: String,
    pub health_score: f32,
    pub available_slots: usize,
    pub busy_slots: usize,
    pub total_slots: usize,
    pub average_wait_time_ms: f32,
    pub pool_efficiency: f32,
    pub peak_concurrent_processes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_process_pool_creation() {
        let pool = ProcessPool::new(4);
        assert_eq!(pool.get_max_processes(), 4);
        assert_eq!(pool.get_available_permits().await, 4);
    }
    
    #[tokio::test]
    async fn test_simple_command_execution() {
        let pool = ProcessPool::new(2);
        
        let request = CommandRequest {
            executable: "echo".to_string(),
            args: vec!["hello".to_string()],
            working_dir: None,
            timeout_seconds: 5,
            capture_output: true,
            environment_vars: vec![],
        };
        
        let result = pool.execute_command(request).await;
        assert!(result.is_ok());
        
        let cmd_result = result.unwrap();
        assert!(cmd_result.success);
        assert!(cmd_result.stdout.contains("hello"));
    }
    
    #[tokio::test]
    async fn test_pool_health_check() {
        let pool = ProcessPool::new(4);
        let health = pool.health_check().await;
        
        assert_eq!(health.total_slots, 4);
        assert_eq!(health.available_slots, 4);
        assert_eq!(health.busy_slots, 0);
    }
}
