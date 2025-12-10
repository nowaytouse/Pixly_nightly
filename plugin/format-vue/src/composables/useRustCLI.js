/**
 * Rust CLI 调用封装
 * 🔥 基于新的 pixly-converter 内核
 * 命令格式: pixly-converter convert <INPUT> --format <FORMAT> [OPTIONS]
 */

import { ref } from 'vue'
import { logger, LOG_KEYS } from '../utils/logger'
import { useI18n } from './useI18n'

export function useRustCLI() {
  const { t } = useI18n()
  const isConverting = ref(false)
  const progress = ref(0)
  const currentFile = ref('')
  const rustBinaryPath = ref(null)

  /**
   * 初始化：查找 pixly-eagle-core 共享二进制文件
   */
  const initRustCLI = async () => {
    if (rustBinaryPath.value) {
      logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI already initialized', { path: rustBinaryPath.value })
      return rustBinaryPath.value
    }

    const { spawn } = require('child_process')
    const path = require('path')
    const fs = require('fs')

    // 🔥 获取插件根目录
    // Eagle 环境：从 window.location 获取实际路径
    // 开发环境：使用 __dirname
    let pluginRoot
    if (window.location && window.location.pathname) {
      // Eagle: file:///path/to/eagle-plugins/xxx/dist/index.html
      // 需要解析出插件目录（dist 的父目录）
      const htmlPath = decodeURIComponent(window.location.pathname)

      // 检查是否在 dist/ 目录中（Eagle 环境）
      if (htmlPath.includes('/dist/')) {
        // 从 /path/to/plugin/dist/index.html 提取 /path/to/plugin
        pluginRoot = path.dirname(path.dirname(htmlPath))
      } else {
        // 开发环境或其他情况
        pluginRoot = path.dirname(htmlPath)
      }
    } else {
      // Fallback: 使用 __dirname（Node.js 环境）
      pluginRoot = path.resolve(__dirname, '../..')
    }

    // 🔥 共享二进制路径（符号链接或复制）
    const possiblePaths = [
      path.join(pluginRoot, 'dist/bin/pixly-eagle-core'),     // Eagle: plugin/dist/bin/
      path.join(pluginRoot, 'bin/pixly-eagle-core'),           // 开发: plugin/bin/ (符号链接)
      path.join(pluginRoot, '../shared/bin/pixly-eagle-core'), // 开发: plugin/shared/bin/
      // 移除所有外部路径，仅使用插件内嵌或共享二进制
    ]

    logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Searching for pixly-eagle-core', {
      pluginRoot,
      searchPaths: possiblePaths.length
    })

    // 生产模式：检测 Eagle 环境
    const isEagleEnv = typeof window !== 'undefined' &&
      window.eagle !== undefined &&
      typeof window.eagle.plugin !== 'undefined'

    const isDev = process.env.NODE_ENV === 'development'

    if (!isDev && !isEagleEnv) {
      const errorMsg = 'This plugin can ONLY run inside Eagle.'
      logger.error(LOG_KEYS.RUST_CLI_ERROR, errorMsg)
      throw new Error(errorMsg)
    }

    // 设置环境变量供 Rust 检测
    if (isEagleEnv) {
      process.env.EAGLE_PLUGIN = 'true'
      logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Eagle environment detected', { EAGLE_PLUGIN: 'true' })
    } else {
      logger.warn(LOG_KEYS.RUST_CLI_EXEC, 'Development mode: Eagle environment not detected')
    }

    for (const p of possiblePaths) {
      try {
        const resolved = path.resolve(p)

        // 检查文件是否存在
        if (!fs.existsSync(resolved)) {
          logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Path not found', { path: resolved })
          continue
        }
        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Found file', { path: resolved })

        // 测试执行（开发模式传递 --dev 参数）
        const testArgs = isDev ? ['--dev', '--version'] : ['--version']
        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Testing executable', { path: p, args: testArgs })

        const proc = spawn(p, testArgs, {
          timeout: 3000,
          env: {
            ...process.env,
            EAGLE_PLUGIN: isEagleEnv ? 'true' : undefined
          }
        })
        let output = ''
        let error = ''

        proc.stdout.on('data', (data) => {
          output += data.toString()
        })

        proc.stderr.on('data', (data) => {
          error += data.toString()
        })

        const success = await new Promise((resolve) => {
          proc.on('close', (code) => {
            logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Version check result', {
              path: p,
              code,
              output: output.trim(),
              error: error.trim()
            })
            resolve(code === 0)
          })
          proc.on('error', (err) => {
            logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Spawn error', { path: p, error: err.message })
            resolve(false)
          })
        })

        if (success) {
          rustBinaryPath.value = p
          logger.info(LOG_KEYS.RUST_CLI_EXEC, '✅ Found pixly-eagle-core', {
            path: p,
            version: output.trim(),
            devMode: isDev
          })
          return p
        }
      } catch (e) {
        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Error testing path', { path: p, error: e.message })
        continue
      }
    }

    // 🔥 未找到，提供详细错误信息
    const errorMsg = `pixly-eagle-core not found. Searched paths:\n${possiblePaths.map(p => `  - ${p}`).join('\n')}\n\nPlease:\n1. Run setup: cd ${pluginRoot} && npm run setup\n2. Or build: cd ${path.join(pluginRoot, '../shared')} && bash build.sh`

    logger.error(LOG_KEYS.RUST_CLI_ERROR, errorMsg)
    throw new Error(errorMsg)
  }

  /**
   * 执行图像转换
   * 🔥 修复：正确的命令格式 pixly-converter convert <INPUT> --format <FORMAT> [OPTIONS]
   * @param {Array} files - 文件列表
   * @param {Object} options - 转换选项
   * @param {Function} onProgress - 进度回调 (fileIndex, fileName, status)
   */
  const convertImages = async (files, options, onProgress = null) => {
    isConverting.value = true
    progress.value = 0

    try {
      // 🔥 初始化Rust CLI
      logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Initializing Rust CLI')
      await initRustCLI()
      logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI initialized', { path: rustBinaryPath.value })

      const results = []
      const path = require('path')

      // 🔥 过滤掉 XMP 文件，只转换媒体文件
      const mediaFiles = files.filter(f => !f.isXmp)

      logger.info(LOG_KEYS.CONVERT_START, 'Starting batch conversion', {
        total: files.length,
        media: mediaFiles.length,
        xmp: files.length - mediaFiles.length,
        mediaFileDetails: mediaFiles.map(f => ({
          name: f.name,
          ext: f.ext,
          hasPath: !!f.path,
          hasXmp: f.hasXmp,
          xmpPath: f.xmpPath
        }))
      })

      if (mediaFiles.length === 0) {
        throw new Error('No media files to convert (all files are XMP or invalid)')
      }

      for (let i = 0; i < mediaFiles.length; i++) {
        const file = mediaFiles[i]
        currentFile.value = file.name
        progress.value = Math.round((i / mediaFiles.length) * 100)

        // 🔍 调用进度回调 - 开始处理
        if (onProgress) {
          onProgress(i + 1, mediaFiles.length, file.name, 'processing')
        }

        // 🔥 验证文件路径
        if (!file.path) {
          logger.error(LOG_KEYS.CONVERT_ERROR, 'File path is undefined', {
            file: file.name,
            fileObject: file
          })
          throw new Error(`File path is undefined for: ${file.name}`)
        }

        // 🔧 Bug Fix: 清理格式字符串中可能存在的引号（必须在生成输出路径之前）
        const cleanFormat = (options.format || 'avif').replace(/^["']|["']$/g, '').toLowerCase()

        // 🔥 生成输出路径（原地替换：同目录，新扩展名）
        const inputPath = file.path
        const outputPath = path.join(
          path.dirname(inputPath),
          `${path.basename(inputPath, path.extname(inputPath))}.${cleanFormat}`
        )

        logger.info(LOG_KEYS.CONVERT_START, 'Converting file', {
          input: inputPath,
          output: outputPath,
          format: cleanFormat
        })

        // 🔥 正确的命令格式：convert <INPUT> --format <FORMAT> --quality <Q> [OPTIONS]
        const args = [
          'convert',
          inputPath,
          '--format', cleanFormat,
          '--quality', options.quality.toString()
        ]

        // JXL 参数
        if (cleanFormat === 'jxl') {
          if (options.effort !== undefined) args.push('--effort', options.effort.toString())
          if (options.distance !== undefined) args.push('--distance', options.distance.toString())
          if (options.lossless) args.push('--lossless')
          if (options.jpegLossless) args.push('--jpeg-lossless')
          if (options.modular) args.push('--modular')
          if (options.progressive) args.push('--progressive')
          if (options.bitDepth) args.push('--bit-depth', options.bitDepth)
          if (options.colorSpace) args.push('--color-space', options.colorSpace)
        }

        // AVIF 参数
        if (cleanFormat === 'avif') {
          if (options.speed !== undefined) args.push('--speed', options.speed.toString())
          if (options.minQuantizer !== undefined) args.push('--min-quantizer', options.minQuantizer.toString())
          if (options.maxQuantizer !== undefined) args.push('--max-quantizer', options.maxQuantizer.toString())
          if (options.chroma) args.push('--chroma', options.chroma)
          if (options.tiles) args.push('--tiles', options.tiles)
        }

        // WebP 参数
        if (cleanFormat === 'webp') {
          if (options.method !== undefined) args.push('--method', options.method.toString())
          if (options.lossless) args.push('--lossless')
          if (options.filterStrength !== undefined) args.push('--filter-strength', options.filterStrength.toString())
          if (options.sharpness !== undefined) args.push('--sharpness', options.sharpness.toString())
        }

        // HEIC 参数
        if (cleanFormat === 'heic') {
          if (options.encoder) args.push('--encoder', options.encoder)
          if (options.lossless) args.push('--lossless')
          if (options.thumbnail) args.push('--thumbnail')
          if (options.chroma) args.push('--chroma', options.chroma)
        }

        // 🔥 快捷工具选项
        const tools = options.quickTools || {}

        // XMP合并
        if (tools.autoMergeXmp !== false) {
          args.push('--merge-xmp')

          // 如果文件有 XMP 路径，直接传递给 Rust CLI（避免扫描）
          if (file.xmpPath) {
            args.push('--xmp-path', file.xmpPath)
            logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Passing XMP path to Rust CLI', {
              xmpPath: file.xmpPath
            })
          }
        }

        // 文件名规范化
        if (tools.normalizeFilenames) {
          args.push('--normalize-filenames')
        }

        // AI 文件验证 (Magika)
        if (tools.fileValidation) {
          args.push('--validate-files')
        }

        // 格式修正
        if (tools.formatCorrection) {
          args.push('--format-correction')
        }

        // 🔥 AI 智能选项
        const ai = options.aiOptions || {}

        // 启用 AI 模式 (统一参数)
        if (ai.smartQuality || ai.autoOptimize) {
          args.push('--ai')
          if (options.optimizeMode) {
            args.push('--optimize-mode', options.optimizeMode)
          }
        }

        // SSIM 质量验证
        if (ai.ssimValidation) {
          args.push('--check-quality')
        }

        // 智能预处理
        if (ai.smartPreprocess) {
          args.push('--preprocess')
        }

        // GPU 加速 (默认启用)
        if (ai.gpuAccel !== false) {
          args.push('--gpu')
        }

        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing command', {
          args: args.join(' ')
        })

        try {
          const result = await executeRustCLI(args)
          results.push({
            success: true,
            file: file.name,
            input: inputPath,
            output: outputPath,
            hasXmp: file.hasXmp,
            xmpId: file.xmpId, // 🔥 传递 XMP ID 用于删除
            stdout: result.stdout
          })
          logger.info(LOG_KEYS.CONVERT_SUCCESS, 'File converted successfully', {
            file: file.name,
            hasXmp: file.hasXmp,
            xmpId: file.xmpId
          })

          // 🔍 调用进度回调 - 成功
          if (onProgress) {
            onProgress(i + 1, mediaFiles.length, file.name, 'success')
          }
        } catch (fileError) {
          results.push({
            success: false,
            file: file.name,
            input: inputPath,
            error: fileError.message
          })
          logger.error(LOG_KEYS.CONVERT_ERROR, 'File conversion failed', {
            file: file.name,
            error: fileError.message
          })

          // 🔍 调用进度回调 - 失败
          if (onProgress) {
            onProgress(i + 1, mediaFiles.length, file.name, 'error', fileError.message)
          }
        }
      }

      progress.value = 100

      // 🔥 统计转换结果
      const successCount = results.filter(r => r.success).length
      const failCount = results.filter(r => !r.success).length
      const xmpMergedCount = results.filter(r => r.success && r.hasXmp).length

      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Batch conversion complete', {
        total: mediaFiles.length,
        success: successCount,
        failed: failCount,
        xmpMerged: xmpMergedCount
      })

      return {
        success: successCount > 0,
        results,
        summary: {
          total: mediaFiles.length,
          success: successCount,
          failed: failCount,
          xmpMerged: xmpMergedCount
        }
      }
    } catch (error) {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Batch conversion failed', {
        error: error.message,
        stack: error.stack,
        rustBinaryPath: rustBinaryPath.value
      })

      return { success: false, error: error.message }
    } finally {
      isConverting.value = false
      currentFile.value = ''
    }
  }

  /**
   * 执行视频转换
   * 🔥 修复：正确的命令格式和参数传递
   */
  const convertVideos = async (files, options) => {
    isConverting.value = true
    progress.value = 0

    try {
      await initRustCLI()
      const results = []
      const path = require('path')

      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        currentFile.value = file.name
        progress.value = Math.round((i / files.length) * 100)

        // 🔥 生成输出路径
        const inputPath = file.path
        const container = options.container || 'mp4'
        const outputPath = path.join(
          path.dirname(inputPath),
          `${path.basename(inputPath, path.extname(inputPath))}.${container}`
        )

        logger.info(LOG_KEYS.CONVERT_START, 'Converting video', {
          input: inputPath,
          output: outputPath,
          container
        })

        // 🔥 正确的命令格式：convert <INPUT> --format <CONTAINER> [VIDEO_OPTIONS]
        const args = [
          'convert',
          inputPath,
          '--format', container
        ]

        // 🔥 视频编码参数 (2025-11-20更新: 添加H.266支持)
        if (options.codec) args.push('--codec', options.codec)  // h264/h265/h266/av1/vp9
        if (options.crf !== undefined) args.push('--crf', options.crf.toString())
        if (options.preset) args.push('--preset', options.preset)  // faster/fast/medium/slow/slower
        if (options.gop !== undefined) args.push('--gop', options.gop.toString())
        if (options.bframes !== undefined) args.push('--bframes', options.bframes.toString())
        if (options.refs !== undefined) args.push('--refs', options.refs.toString())
        if (options.rateControl) args.push('--rate-control', options.rateControl)
        if (options.meMethod) args.push('--me-method', options.meMethod)
        if (options.pixFmt) args.push('--pix-fmt', options.pixFmt)
        if (options.twoPass) args.push('--two-pass')  // Two-pass encoding
        if (options.hwAccel) args.push('--hw-accel', options.hwAccel)  // Hardware acceleration

        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing command', {
          args: args.join(' ')
        })

        const result = await executeRustCLI(args)
        results.push({
          success: true,
          input: inputPath,
          output: outputPath,
          stdout: result.stdout
        })
      }

      progress.value = 100
      return { success: true, results }
    } catch (error) {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Video conversion failed', {
        error: error.message
      })
      return { success: false, error: error.message }
    } finally {
      isConverting.value = false
      currentFile.value = ''
    }
  }

  /**
   * 执行 Rust CLI 命令
   * 🔥 修复：添加 --dev 参数支持和 EAGLE_PLUGIN 环境变量
   */
  const executeRustCLI = (args) => {
    return new Promise((resolve, reject) => {
      const { spawn } = require('child_process')

      // 开发模式：添加 --dev 参数
      const isDev = process.env.NODE_ENV === 'development'
      const isEagleEnv = typeof window !== 'undefined' && window.eagle !== undefined

      const finalArgs = isDev ? ['--dev', ...args] : args

      logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing pixly-eagle-core', {
        args: finalArgs.join(' '),
        devMode: isDev,
        eagleEnv: isEagleEnv
      })

      // 🔥 设置完整的PATH环境变量（包含Homebrew等工具路径）
      const fullPath = [
        '/opt/homebrew/bin',      // macOS Homebrew (Apple Silicon)
        '/opt/homebrew/sbin',
        '/usr/local/bin',          // macOS Homebrew (Intel) / Linux
        '/usr/bin',
        '/bin',
        '/usr/sbin',
        '/sbin',
        process.env.PATH || ''
      ].filter(Boolean).join(':')

      const proc = spawn(rustBinaryPath.value, finalArgs, {
        env: {
          ...process.env,
          PATH: fullPath,
          EAGLE_PLUGIN: isEagleEnv ? 'true' : undefined
        }
      })

      let stdout = ''
      let stderr = ''

      proc.stdout.on('data', (data) => {
        const text = data.toString()
        stdout += text

        // 🔥 解析进度信息
        const lines = text.split('\n')
        for (const line of lines) {
          if (line.trim()) {
            logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI output', { line })

            // 检测转换完成
            if (line.includes('✅ Conversion complete')) {
              progress.value = 100
            }
          }
        }
      })

      proc.stderr.on('data', (data) => {
        const text = data.toString()
        stderr += text
        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI stderr', { text })
      })

      proc.on('close', (code) => {
        if (code === 0) {
          logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Conversion completed', {
            stdout: stdout.substring(0, 200)
          })
          resolve({ success: true, stdout, stderr })
        } else {
          logger.error(LOG_KEYS.CONVERT_ERROR, 'Conversion failed', {
            code,
            stderr: stderr.substring(0, 500)
          })

          // 🔥 提供友好的错误信息（使用国际化）
          let errorMessage = stderr || `Exit code: ${code}`

          // 检测常见错误
          if (stderr.includes('cjxl') && stderr.includes('not found')) {
            errorMessage = t('errors.jxlNotInstalled')
          } else if (stderr.includes('avifenc') && stderr.includes('not found')) {
            errorMessage = t('errors.avifNotInstalled')
          } else if (stderr.includes('No such file or directory')) {
            errorMessage = t('errors.fileNotFound')
          }

          reject(new Error(errorMessage))
        }
      })

      proc.on('error', (err) => {
        logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Spawn failed', { error: err.message })
        reject(err)
      })
    })
  }

  return {
    isConverting,
    progress,
    currentFile,
    convertImages,
    convertVideos
  }
}
