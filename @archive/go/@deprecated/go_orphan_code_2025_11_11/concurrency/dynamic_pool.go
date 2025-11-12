package concurrency

import (
	"sync"

	"go.uber.org/zap"
)

// WorkerPool 动态worker池
// 🎯 基于文件大小/分辨率动态调整并发数
type WorkerPool struct {
	maxWorkers     int           // 最大worker数
	availableSlots chan struct{} // 可用slots
	activeWorkers  int           // 当前活跃worker数
	mutex          sync.RWMutex
	logger         *zap.Logger
}

// NewWorkerPool 创建动态worker池
func NewWorkerPool(maxWorkers int, logger *zap.Logger) *WorkerPool {
	return &WorkerPool{
		maxWorkers:     maxWorkers,
		availableSlots: make(chan struct{}, maxWorkers),
		logger:         logger,
	}
}

// Acquire 获取worker slot (基于文件大小动态调整)
// 返回需要占用的slot数量
func (wp *WorkerPool) Acquire(megapixels float64) int {
	// 🎯 动态策略:
	// < 2MP:  1 slot (小文件,并发处理)
	// 2-8MP:  2 slots (中等文件,适度限制)
	// 8-32MP: 4 slots (大文件,严格限制)
	// > 32MP: maxWorkers/2 (超大文件,限制为pool的50%!避(MISSING)免死锁)

	var slotsNeeded int
	if megapixels < 2.0 {
		slotsNeeded = 1
	} else if megapixels < 8.0 {
		slotsNeeded = 2
	} else if megapixels < 32.0 {
		slotsNeeded = 4
	} else {
		// 🔒 防止死锁：超大文件最多占用50%!s(MISSING)lots
		// 确保至少还有其他goroutine可以运行
		slotsNeeded = wp.maxWorkers / 2
		if slotsNeeded < 4 {
			slotsNeeded = 4 // 至少4个slots
		}
	}

	// 确保不超过最大worker数的50%!(MISSING)避免死锁)
	maxSlotsPerFile := wp.maxWorkers / 2
	if maxSlotsPerFile < 1 {
		maxSlotsPerFile = 1
	}
	if slotsNeeded > maxSlotsPerFile {
		slotsNeeded = maxSlotsPerFile
	}

	wp.logger.Debug("动态并发控制",
		zap.Float64("megapixels", megapixels),
		zap.Int("slots_needed", slotsNeeded),
		zap.Int("max_workers", wp.maxWorkers))

	// 占用所需的slots
	for i := 0; i < slotsNeeded; i++ {
		wp.availableSlots <- struct{}{}
	}

	wp.mutex.Lock()
	wp.activeWorkers += slotsNeeded
	wp.mutex.Unlock()

	return slotsNeeded
}

// Release 释放worker slots
func (wp *WorkerPool) Release(slots int) {
	for i := 0; i < slots; i++ {
		<-wp.availableSlots
	}

	wp.mutex.Lock()
	wp.activeWorkers -= slots
	wp.mutex.Unlock()
}

// GetActiveWorkers 获取当前活跃worker数
func (wp *WorkerPool) GetActiveWorkers() int {
	wp.mutex.RLock()
	defer wp.mutex.RUnlock()
	return wp.activeWorkers
}

// CalculateMegapixels 计算百万像素数
func CalculateMegapixels(width, height int) float64 {
	if width <= 0 || height <= 0 {
		return 0
	}
	return float64(width*height) / 1_000_000.0
}
