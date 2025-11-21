/**
 * 🚀 Format-Vue 性能基准测试
 * 
 * 测试项目：
 * 1. 组件渲染性能
 * 2. 日志系统性能
 * 3. 文件列表渲染性能
 * 4. Watch响应性能
 * 5. 内存使用情况
 */

import { performance } from 'perf_hooks'
import { writeFileSync } from 'fs'

class PerformanceBenchmark {
  constructor() {
    this.results = []
    this.memoryBaseline = null
  }

  /**
   * 记录内存基线
   */
  recordMemoryBaseline() {
    if (global.gc) {
      global.gc()
    }
    this.memoryBaseline = process.memoryUsage()
    console.log('📊 Memory Baseline:', {
      heapUsed: `${(this.memoryBaseline.heapUsed / 1024 / 1024).toFixed(2)} MB`,
      heapTotal: `${(this.memoryBaseline.heapTotal / 1024 / 1024).toFixed(2)} MB`
    })
  }

  /**
   * 测试函数执行时间
   */
  async benchmark(name, fn, iterations = 1000) {
    console.log(`\n🔍 Testing: ${name}`)
    
    // 预热
    for (let i = 0; i < 10; i++) {
      await fn()
    }

    // 正式测试
    const times = []
    for (let i = 0; i < iterations; i++) {
      const start = performance.now()
      await fn()
      const end = performance.now()
      times.push(end - start)
    }

    // 统计
    const avg = times.reduce((a, b) => a + b, 0) / times.length
    const min = Math.min(...times)
    const max = Math.max(...times)
    const p95 = times.sort((a, b) => a - b)[Math.floor(times.length * 0.95)]

    const result = {
      name,
      iterations,
      avg: avg.toFixed(3),
      min: min.toFixed(3),
      max: max.toFixed(3),
      p95: p95.toFixed(3),
      unit: 'ms'
    }

    this.results.push(result)

    console.log(`  ✅ Avg: ${result.avg}ms | Min: ${result.min}ms | Max: ${result.max}ms | P95: ${result.p95}ms`)

    return result
  }

  /**
   * 测试内存使用
   */
  async benchmarkMemory(name, fn, iterations = 100) {
    console.log(`\n💾 Memory Test: ${name}`)

    if (global.gc) {
      global.gc()
    }

    const before = process.memoryUsage()

    for (let i = 0; i < iterations; i++) {
      await fn()
    }

    if (global.gc) {
      global.gc()
    }

    const after = process.memoryUsage()

    const heapDiff = (after.heapUsed - before.heapUsed) / 1024 / 1024
    const result = {
      name,
      iterations,
      heapDiff: heapDiff.toFixed(2),
      unit: 'MB'
    }

    this.results.push(result)

    console.log(`  ✅ Heap Diff: ${result.heapDiff} MB`)

    return result
  }

  /**
   * 生成报告
   */
  generateReport() {
    console.log('\n' + '='.repeat(80))
    console.log('📊 PERFORMANCE BENCHMARK REPORT')
    console.log('='.repeat(80))

    console.log('\n⏱️  Timing Results:')
    console.log('─'.repeat(80))
    this.results
      .filter(r => r.unit === 'ms')
      .forEach(r => {
        console.log(`${r.name.padEnd(40)} | Avg: ${r.avg}ms | P95: ${r.p95}ms`)
      })

    console.log('\n💾 Memory Results:')
    console.log('─'.repeat(80))
    this.results
      .filter(r => r.unit === 'MB')
      .forEach(r => {
        console.log(`${r.name.padEnd(40)} | Heap: ${r.heapDiff} MB`)
      })

    console.log('\n' + '='.repeat(80))

    return this.results
  }

  /**
   * 保存报告到文件
   */
  saveReport(filename = 'performance-report.json') {
    const report = {
      timestamp: new Date().toISOString(),
      baseline: this.memoryBaseline,
      results: this.results
    }
    writeFileSync(filename, JSON.stringify(report, null, 2))
    console.log(`\n💾 Report saved to: ${filename}`)
  }
}

// 模拟测试场景
async function runBenchmarks() {
  const bench = new PerformanceBenchmark()
  
  bench.recordMemoryBaseline()

  // 1. 测试日志添加性能
  await bench.benchmark('Log Entry Creation', () => {
    const now = new Date()
    const time = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`
    const log = {
      time,
      icon: '📝',
      message: 'Test log message',
      type: 'info'
    }
    return log
  }, 10000)

  // 2. 测试数组push性能
  const logs = []
  await bench.benchmark('Array Push (1000 items)', () => {
    logs.push({
      time: '12:00:00',
      icon: '📝',
      message: 'Test',
      type: 'info'
    })
  }, 1000)

  // 3. 测试对象深拷贝性能
  const obj = { a: 1, b: 2, c: { d: 3, e: 4 } }
  await bench.benchmark('Object Spread Copy', () => {
    return { ...obj }
  }, 10000)

  // 4. 测试setTimeout创建性能
  await bench.benchmark('setTimeout Creation', () => {
    return new Promise(resolve => {
      const timer = setTimeout(() => {
        clearTimeout(timer)
        resolve()
      }, 0)
    })
  }, 1000)

  // 5. 测试防抖setTimeout性能
  let timer = null
  await bench.benchmark('Debounced setTimeout', () => {
    if (timer) clearTimeout(timer)
    return new Promise(resolve => {
      timer = setTimeout(() => {
        timer = null
        resolve()
      }, 0)
    })
  }, 1000)

  // 6. 测试文件列表渲染数据准备
  await bench.benchmark('File List Data Prep (100 files)', () => {
    const files = []
    for (let i = 0; i < 100; i++) {
      files.push({
        id: `file-${i}`,
        name: `image-${i}.jpg`,
        ext: 'jpg',
        size: 1024 * 1024 * 2,
        thumbnail: null
      })
    }
    return files
  }, 100)

  // 7. 内存测试：大量日志
  await bench.benchmarkMemory('1000 Log Entries', () => {
    const logs = []
    for (let i = 0; i < 1000; i++) {
      logs.push({
        time: '12:00:00',
        icon: '📝',
        message: `Log message ${i}`,
        type: 'info'
      })
    }
    return logs
  }, 10)

  // 8. 内存测试：文件列表
  await bench.benchmarkMemory('1000 File Objects', () => {
    const files = []
    for (let i = 0; i < 1000; i++) {
      files.push({
        id: `file-${i}`,
        name: `image-${i}.jpg`,
        ext: 'jpg',
        size: 1024 * 1024 * 2,
        path: `/path/to/image-${i}.jpg`,
        thumbnail: null
      })
    }
    return files
  }, 10)

  bench.generateReport()
  bench.saveReport('plugin/format-vue/performance-report.json')
}

// 运行基准测试
console.log('🚀 Starting Format-Vue Performance Benchmark...\n')
runBenchmarks().catch(console.error)

export { PerformanceBenchmark }
