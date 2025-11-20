/**
 * Rust CLI 集成 Composable
 * 
 * 架构原则：
 * - Vue 层：仅负责 UI 交互和参数收集
 * - Rust 层：负责所有文件处理、AI 预测、转换执行
 * 
 * 参考：PROJECT_QUALITY_MANIFESTO.md
 */

import { ref } from 'vue'
import { logger, LOG_KEYS } from '../utils/logger'

export function useRustCLI() {
  const isAvailable = ref(false)
  const version = ref('')
  const error = ref(null)
  const processing = ref(false)

  /**
   * 初始化 Rust CLI
   */
  const init = () => {
    try {
      const { execSync } = require('child_process')
      const path = require('path')
      
      // Rust CLI 路径
      const rustPath = path.join(__dirname, '../../bin/pixly-rust')
      
      // 测试可用性
      const output = execSync(`"${rustPath}" --version`, {
        encoding: 'utf8',
        timeout: 5000
      }).trim()
      
      version.value = output.replace(/^pixly-rust\s+/, '')
      isAvailable.value = true
      
      logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI available', { version: version.value })
    } catch (err) {
      error.value = err.message
      isAvailable.value = false
      logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Rust CLI not available', { error: err.message })
    }
  }

  /**
   * 🎬 获取视频编码器推荐 (2025-11-20新增)
   */
  const getVideoCodecRecommendation = async (inputPath, qualityMode = 'balanced') => {
    if (!isAvailable.value) {
      throw new Error('Rust CLI 不可用')
    }

    try {
      const { execSync } = require('child_process')
      const path = require('path')
      const rustPath = path.join(__dirname, '../../bin/pixly-rust')

      // 调用 analyze 命令获取推荐
      const args = [
        'analyze',
        inputPath,
        '--json',
        '--recommend-video-codec',
        '--quality-mode', qualityMode
      ]

      const output = execSync(`"${rustPath}" ${args.join(' ')}`, {
        encoding: 'utf8',
        timeout: 10000
      })

      const result = JSON.parse(output)
      
      logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Video codec recommendation', {
        codec: result.recommended_codec,
        container: result.recommended_container,
        reason: result.reason
      })

      return result
    } catch (err) {
      logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Failed to get video codec recommendation', {
        error: err.message
      })
      // 返回默认推荐
      return {
        recommended_codec: 'h265',
        recommended_container: 'mp4',
        confidence: 0.8,
        reason: 'Default recommendation (H.265/MP4)',
        alternative_codecs: ['h266', 'av1', 'h264'],
        alternative_containers: ['mkv', 'webm']
      }
    }
  }

  /**
   * 执行转换
   */
  const convert = async (options) => {
    if (!isAvailable.value) {
      throw new Error('Rust CLI 不可用')
    }

    const {
      inputPath,
      outputPath,
      xmpPath = null,  // 🔥 XMP Sidecar 路径
      format,
      disableFormatChange = false,
      useAI = true,
      optimizeMode = 'balanced',
      enableFileValidation = true,
      enableSSIM = false,
      enableGPU = true,
      enablePreprocess = true,
      enableFormatCorrection = false,  // 🔥 格式自动修正
      mergeXmp = true,  // 🔥 XMP 合并（默认开启）
      normalizeFilenames = true,  // 🔥 文件名规范化（默认开启）
      // 🎬 视频 AI 功能
      enableVideoForAnimation = true,
      enableSceneDetection = false,
      enableVMAF = false,
      enableTwoPass = false,
      onProgress
    } = options

    processing.value = true

    try {
      const { spawn } = require('child_process')
      const path = require('path')
      const rustPath = path.join(__dirname, '../../bin/pixly-rust')

      // 构建参数
      const args = ['convert', inputPath, outputPath]
      
      if (useAI) {
        args.push('--ai')
        args.push('--optimize-mode', optimizeMode)
      }
      
      if (format && !disableFormatChange) args.push('--format', format)
      if (disableFormatChange) args.push('--same-format')
      
      // 🔥 AI 机器学习功能
      if (enableFileValidation) args.push('--validate-files')
      if (enableSSIM) args.push('--check-quality')
      if (!enableGPU) args.push('--no-gpu')
      if (enablePreprocess) args.push('--preprocess')
      
      // 🔥 辅助功能
      if (!mergeXmp) args.push('--merge-xmp=false')
      if (xmpPath) args.push('--xmp-path', xmpPath)
      if (normalizeFilenames) args.push('--normalize-filenames')
      
      // 🔥 格式自动修正（实验性）
      if (enableFormatCorrection) args.push('--format-correction')
      
      // 🎬 视频 AI 功能
      if (!enableVideoForAnimation) args.push('--no-video-for-animation')
      if (enableSceneDetection) args.push('--scene-detection')
      if (enableVMAF) args.push('--vmaf-validation')
      if (enableTwoPass) args.push('--two-pass')

      logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing Rust CLI', { args: args.join(' ') })

      return new Promise((resolve, reject) => {
        const proc = spawn(rustPath, args, {
          env: {
            ...process.env,
            PATH: [
              '/opt/homebrew/bin',
              '/usr/local/bin',
              process.env.PATH
            ].join(':')
          }
        })

        let stdout = ''
        let stderr = ''

        proc.stdout.on('data', (data) => {
          const text = data.toString()
          stdout += text
          
          // 解析进度
          if (onProgress) {
            const match = text.match(/(\d+)%/)
            if (match) {
              onProgress(parseInt(match[1]))
            }
          }
        })

        proc.stderr.on('data', (data) => {
          stderr += data.toString()
        })

        proc.on('close', (code) => {
          processing.value = false
          
          if (code === 0) {
            resolve({ success: true, stdout, stderr })
          } else {
            reject(new Error(stderr || `Exit code: ${code}`))
          }
        })

        proc.on('error', (err) => {
          processing.value = false
          reject(err)
        })
      })
    } catch (err) {
      processing.value = false
      throw err
    }
  }

  /**
   * 🎬 视频转换
   */
  const convertVideo = async (options) => {
    if (!isAvailable.value) {
      throw new Error('Rust CLI 不可用')
    }

    const {
      inputPath,
      outputPath,
      codec = 'h265',
      container = 'mp4',
      crf = 23,
      preset = 'medium',
      useAI = true,
      optimizeMode = 'balanced',
      enableGPU = true,
      enableVideoForAnimation = true,
      enableSceneDetection = false,
      enableVMAF = false,
      enableTwoPass = false,
      gop = null,
      bframes = null,
      refs = null,
      meMethod = null,
      pixFmt = null,
      onProgress
    } = options

    processing.value = true

    try {
      const { spawn } = require('child_process')
      const path = require('path')
      const rustPath = path.join(__dirname, '../../bin/pixly-rust')

      // 构建参数
      const args = ['video', inputPath, outputPath]
      
      args.push('--codec', codec)
      args.push('--container', container)
      args.push('--crf', crf.toString())
      args.push('--preset', preset)
      
      if (useAI) {
        args.push('--ai')
        args.push('--optimize-mode', optimizeMode)
      }
      
      if (!enableGPU) args.push('--gpu=false')
      if (!enableVideoForAnimation) args.push('--video-for-animation=false')
      if (enableSceneDetection) args.push('--scene-detection')
      if (enableVMAF) args.push('--vmaf')
      if (enableTwoPass) args.push('--two-pass')
      
      // 高级参数
      if (gop) args.push('--gop', gop.toString())
      if (bframes) args.push('--bframes', bframes.toString())
      if (refs) args.push('--refs', refs.toString())
      if (meMethod) args.push('--me-method', meMethod)
      if (pixFmt) args.push('--pix-fmt', pixFmt)

      logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Video conversion', { args: args.join(' ') })

      return new Promise((resolve, reject) => {
        const proc = spawn(rustPath, args, {
          env: {
            ...process.env,
            PATH: [
              '/opt/homebrew/bin',
              '/usr/local/bin',
              process.env.PATH
            ].join(':')
          }
        })

        let stdout = ''
        let stderr = ''

        proc.stdout.on('data', (data) => {
          const text = data.toString()
          stdout += text
          
          // 解析进度
          if (onProgress) {
            const match = text.match(/(\d+)%/)
            if (match) {
              onProgress(parseInt(match[1]))
            }
          }
        })

        proc.stderr.on('data', (data) => {
          stderr += data.toString()
        })

        proc.on('close', (code) => {
          processing.value = false
          
          if (code === 0) {
            resolve({ success: true, stdout, stderr })
          } else {
            reject(new Error(stderr || `Exit code: ${code}`))
          }
        })

        proc.on('error', (err) => {
          processing.value = false
          reject(err)
        })
      })
    } catch (err) {
      processing.value = false
      throw err
    }
  }

  /**
   * 批量转换（仅图像）
   * 🔥 视频转换请使用 convertVideo()
   */
  const batchConvert = async (files, options, onProgress) => {
    const results = []
    
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      
      try {
        if (onProgress) {
          onProgress({
            current: i + 1,
            total: files.length,
            file: file.name,
            percentage: Math.round(((i + 1) / files.length) * 100)
          })
        }

        // 🖼️ 图像转换
        const result = await convert({
          ...options,
          inputPath: file.path,
          outputPath: file.path.replace(/\.[^.]+$/, `.${options.format || 'avif'}`)
        })

        results.push({ file: file.name, success: true, result })
      } catch (err) {
        results.push({ file: file.name, success: false, error: err.message })
      }
    }

    return results
  }

  return {
    isAvailable,
    version,
    error,
    processing,
    init,
    convert,
    convertVideo,
    batchConvert,
    getVideoCodecRecommendation  // 🎬 2025-11-20新增
  }
}
