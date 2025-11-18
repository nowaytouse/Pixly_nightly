/**
 * 🔥 Rust CLI 集成 Composable
 * 
 * 架构原则：
 * - Vue 层：仅负责 UI 交互和参数收集
 * - Rust 层：负责所有文件处理、AI 预测、转换执行
 * - 禁止：在 JS 中实现转换逻辑、参数计算、文件处理
 * 
 * 参考：PROJECT_QUALITY_MANIFESTO.md - Eagle插件架构原则
 */

import { ref } from 'vue'
import { ElMessage } from 'element-plus'

export function useRustCLI() {
  const isRustAvailable = ref(false)
  const rustVersion = ref('')
  const isProcessing = ref(false)

  /**
   * 检测 Rust CLI 是否可用
   */
  const detectRustCLI = async () => {
    try {
      // 尝试调用 Rust CLI 获取版本
      const result = await executeCommand(['--version'])
      
      if (result.success) {
        isRustAvailable.value = true
        rustVersion.value = result.stdout.trim()
        console.log('✅ Rust CLI detected:', rustVersion.value)
        return true
      }
    } catch (error) {
      console.error('❌ Rust CLI not available:', error)
      isRustAvailable.value = false
      
      // 响亮的错误提示
      ElMessage.error({
        message: '❌ Rust 转换内核未找到！请确保 pixly-rust 已编译。',
        duration: 5000,
        showClose: true
      })
    }
    return false
  }

  /**
   * 执行 Rust CLI 命令
   * @param {Array} args - 命令参数
   * @returns {Promise<Object>} - { success, stdout, stderr }
   */
  const executeCommand = async (args) => {
    return new Promise((resolve, reject) => {
      const { exec } = require('child_process')
      const path = require('path')
      
      // Rust CLI 路径（相对于插件根目录）
      const rustCLI = path.join(__dirname, '../../bin/pixly-rust')
      
      const command = `"${rustCLI}" ${args.join(' ')}`
      
      console.log('[Rust CLI] Executing:', command)
      
      exec(command, { maxBuffer: 10 * 1024 * 1024 }, (error, stdout, stderr) => {
        if (error) {
          console.error('[Rust CLI] Error:', error)
          reject({
            success: false,
            error: error.message,
            stdout: stdout || '',
            stderr: stderr || ''
          })
          return
        }
        
        resolve({
          success: true,
          stdout: stdout || '',
          stderr: stderr || ''
        })
      })
    })
  }

  /**
   * AI 智能转换（核心功能）
   * @param {Object} options - 转换选项
   * @returns {Promise<Object>} - 转换结果
   */
  const convertWithAI = async (options) => {
    const {
      inputPath,
      outputPath,
      mode = 'smart',           // smart | manual
      optimizeTarget = 'balanced', // balanced | quality | size
      outputFormat = 'auto',    // auto | avif | jxl | webp | heic
      quality = null,           // 手动模式质量
      speed = null,             // 手动模式速度
      onProgress = null         // 进度回调
    } = options

    if (!isRustAvailable.value) {
      throw new Error('Rust CLI not available')
    }

    isProcessing.value = true

    try {
      // 构建 Rust CLI 参数
      const args = ['convert', inputPath, outputPath]

      // 智能模式：使用 AI 预测
      if (mode === 'smart') {
        args.push('--ai')
        args.push('--optimize-mode', optimizeTarget)
        
        if (outputFormat !== 'auto') {
          args.push('--format', outputFormat)
        }
      } 
      // 手动模式：用户指定参数
      else {
        if (quality !== null) {
          args.push('--quality', quality.toString())
        }
        if (speed !== null) {
          args.push('--speed', speed.toString())
        }
        if (outputFormat !== 'auto') {
          args.push('--format', outputFormat)
        }
      }

      console.log('[Rust CLI] 🤖 Starting AI conversion:', args)

      // 执行转换
      const result = await executeCommand(args)

      if (result.success) {
        console.log('[Rust CLI] ✅ Conversion completed')
        return {
          success: true,
          message: 'Conversion completed successfully',
          output: result.stdout
        }
      } else {
        throw new Error(result.stderr || 'Conversion failed')
      }
    } catch (error) {
      console.error('[Rust CLI] ❌ Conversion failed:', error)
      throw error
    } finally {
      isProcessing.value = false
    }
  }

  /**
   * 批量转换
   * @param {Array} files - 文件列表
   * @param {Object} options - 转换选项
   * @param {Function} onProgress - 进度回调
   */
  const batchConvert = async (files, options, onProgress) => {
    const results = []
    
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      
      try {
        // 更新进度
        if (onProgress) {
          onProgress({
            current: i + 1,
            total: files.length,
            percentage: Math.round(((i + 1) / files.length) * 100),
            currentFile: file.name
          })
        }

        // 构建输出路径
        const outputPath = file.path.replace(/\.[^.]+$/, `.${options.outputFormat || 'avif'}`)

        // 执行转换
        const result = await convertWithAI({
          ...options,
          inputPath: file.path,
          outputPath: outputPath
        })

        results.push({
          file: file.name,
          success: true,
          result
        })
      } catch (error) {
        results.push({
          file: file.name,
          success: false,
          error: error.message
        })
      }
    }

    return results
  }

  /**
   * 分析文件（获取 AI 推荐）
   * @param {String} filePath - 文件路径
   * @returns {Promise<Object>} - AI 分析结果
   */
  const analyzeFile = async (filePath) => {
    if (!isRustAvailable.value) {
      throw new Error('Rust CLI not available')
    }

    try {
      const args = ['analyze', filePath, '--json']
      const result = await executeCommand(args)

      if (result.success) {
        // 解析 JSON 输出
        const analysis = JSON.parse(result.stdout)
        return analysis
      } else {
        throw new Error(result.stderr || 'Analysis failed')
      }
    } catch (error) {
      console.error('[Rust CLI] ❌ Analysis failed:', error)
      throw error
    }
  }

  return {
    // State
    isRustAvailable,
    rustVersion,
    isProcessing,

    // Methods
    detectRustCLI,
    executeCommand,
    convertWithAI,
    batchConvert,
    analyzeFile
  }
}
