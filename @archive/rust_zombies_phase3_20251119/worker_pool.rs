// 动态Worker池 - 基于文件大小动态调整并发数
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/concurrency/dynamic_pool.go

use std::sync::{Arc, Condvar, Mutex};

pub struct WorkerPool {
    max_workers: usize,
    available_slots: Arc<(Mutex<usize>, Condvar)>,
    active_workers: Arc<Mutex<usize>>,
}

impl WorkerPool {
    /// 创建动态worker池
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            available_slots: Arc::new((Mutex::new(max_workers), Condvar::new())),
            active_workers: Arc::new(Mutex::new(0)),
        }
    }

    /// 获取worker slot (基于文件大小动态调整)
    /// 返回需要占用的slot数量
    pub fn acquire(&self, megapixels: f64) -> WorkerGuard {
        // 动态策略:
        // < 2MP:  1 slot (小文件,并发处理)
        // 2-8MP:  2 slots (中等文件,适度限制)
        // 8-32MP: 4 slots (大文件,严格限制)
        // > 32MP: maxWorkers/2 (超大文件,限制为pool的50%,避免死锁)

        let slots_needed = if megapixels < 2.0 {
            1
        } else if megapixels < 8.0 {
            2
        } else if megapixels < 32.0 {
            4
        } else {
            // 防止死锁：超大文件最多占用50%slots
            // 确保至少还有其他goroutine可以运行
            let max_slots = self.max_workers / 2;
            max_slots.max(4) // 至少4个slots
        };

        // 确保不超过最大worker数的50%(避免死锁)
        let max_slots_per_file = (self.max_workers / 2).max(1);
        let slots_needed = slots_needed.min(max_slots_per_file);

        // 占用所需的slots
        let (lock, cvar) = &*self.available_slots;
        let mut available = lock.lock().unwrap();
        while *available < slots_needed {
            available = cvar.wait(available).unwrap();
        }
        *available -= slots_needed;
        drop(available);

        // 更新活跃worker数
        {
            let mut active = self.active_workers.lock().unwrap();
            *active += slots_needed;
        }

        WorkerGuard {
            available_slots: Arc::clone(&self.available_slots),
            active_workers: Arc::clone(&self.active_workers),
            slots: slots_needed,
        }
    }

    /// 获取当前活跃worker数
    pub fn get_active_workers(&self) -> usize {
        *self.active_workers.lock().unwrap()
    }
}

impl Clone for WorkerPool {
    fn clone(&self) -> Self {
        Self {
            max_workers: self.max_workers,
            available_slots: Arc::clone(&self.available_slots),
            active_workers: Arc::clone(&self.active_workers),
        }
    }
}

/// Worker守卫 - 自动释放slots
pub struct WorkerGuard {
    available_slots: Arc<(Mutex<usize>, Condvar)>,
    active_workers: Arc<Mutex<usize>>,
    slots: usize,
}

impl Drop for WorkerGuard {
    fn drop(&mut self) {
        // 释放slots
        let (lock, cvar) = &*self.available_slots;
        let mut available = lock.lock().unwrap();
        *available += self.slots;
        cvar.notify_all();
        drop(available);

        // 更新活跃worker数
        let mut active = self.active_workers.lock().unwrap();
        *active = active.saturating_sub(self.slots);
    }
}

/// 计算百万像素数
pub fn calculate_megapixels(width: u32, height: u32) -> f64 {
    if width == 0 || height == 0 {
        return 0.0;
    }
    (width as f64 * height as f64) / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_megapixels() {
        assert_eq!(calculate_megapixels(1000, 1000), 1.0);
        assert_eq!(calculate_megapixels(2000, 2000), 4.0);
        assert_eq!(calculate_megapixels(0, 1000), 0.0);
    }

    #[test]
    fn test_worker_pool_basic() {
        let pool = WorkerPool::new(10);
        
        // 小文件应该占用1个slot
        let guard = pool.acquire(1.0);
        assert_eq!(pool.get_active_workers(), 1);
        drop(guard);
        
        assert_eq!(pool.get_active_workers(), 0);
    }

    #[test]
    fn test_worker_pool_large_file() {
        let pool = WorkerPool::new(10);
        
        // 超大文件应该占用5个slots (10/2)
        let guard = pool.acquire(50.0);
        assert_eq!(pool.get_active_workers(), 5);
        drop(guard);
        
        assert_eq!(pool.get_active_workers(), 0);
    }
}
