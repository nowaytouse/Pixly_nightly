/**
 * Rust CLI 调用封装
 * 调用pixly-rust analyze命令进行AI分析
 */

import { ref } from 'vue'

export function useRustCLI() {
  const isExecuting = ref(false)
  const lastError = ref(null)

  /**
   * 获取Rust CLI路径
   */
  function getRustCLIPath() {
    // 从format-vue插件的实现中复用
    const platform = eagle.app.platform
    const pluginPath = eagle.plugin.path
    
    if (platform === 'darwin') {
      return `${pluginPath}/../../../pixly-rust`
    } else if (platform === 'win32') {
      return `${pluginPath}/../../../pixly-rust.exe`
    } else {
      return `${pluginPath}/../../../pixly-rust`
    }
  }

  /**
   * 执行Rust CLI命令
   */
  async function executeCommand(args) {
    const { exec } = require('child_process')
    const { promisify } = require('util')
    const execAsync = promisify(exec)

    const cliPath = getRustCLIPath()
    const command = `"${cliPath}" ${args.join(' ')}`

    console.log('[Rust CLI] Executing:', command)

    try {
      isExecuting.value = true
      const { stdout, stderr } = await execAsync(command, {
        maxBuffer: 10 * 1024 * 1024 // 10MB
      })

      if (stderr) {
        console.warn('[Rust CLI] stderr:', stderr)
      }

      console.log('[Rust CLI] stdout:', stdout)
      return stdout
    } catch (error) {
      console.error('[Rust CLI] Execution failed:', error)
      lastError.value = error
      throw error
    } finally {
      isExecuting.value = false
    }
  }

  /**
   * 分析媒体文件
   * 调用: pixly-rust analyze <file> --ai --json
   */
  async function analyzeMedia(filePath) {
    try {
      const args = ['analyze', filePath, '--ai', '--json']
      const output = await executeCommand(args)
      
      // 解析JSON输出
      const result = JSON.parse(output)
      
      return {
        mediaType: result.media_type,
        features: {
          width: result.features.width,
          height: result.features.height,
          fileSize: result.features.file_size,
          format: result.features.format,
          isAnimated: result.features.is_animated || false,
          hasAlpha: result.features.has_alpha || false,
          frameCount: result.features.frame_count || 1,
          duration: result.features.duration || 0,
          complexity: result.features.complexity || 0
        },
        recommendation: {
          format: result.recommendation.format,
          params: result.recommendation.params,
          estimatedSize: result.recommendation.estimated_size,
          sizeReduction: result.recommendation.size_reduction,
          qualityScore: result.recommendation.quality_score,
          confidence: result.recommendation.confidence
        }
      }
    } catch (error) {
      // 🔥 质量宣言：失败就响亮报错，不降级！
      console.error('[Rust CLI] ❌ AI分析失败:', error)
      console.error('   Without Rust CLI, AI analysis cannot work!')
      console.error('   Please ensure pixly-rust is compiled and accessible')
      
      // 抛出错误，让用户知道真实情况
      throw new Error(`AI分析失败: ${error.message}`)
    }
  }

  /**
   * 批量分析媒体文件
   */
  async function analyzeBatch(filePaths) {
    const results = []
    for (const filePath of filePaths) {
      const result = await analyzeMedia(filePath)
      results.push(result)
    }
    return results
  }

  // 🔥 质量宣言：删除所有模拟数据函数
  // 真实的AI分析必须依赖Rust CLI，不提供fallback

  return {
    isExecuting,
    lastError,
    analyzeMedia,
    analyzeBatch,
    executeCommand
  }
}
