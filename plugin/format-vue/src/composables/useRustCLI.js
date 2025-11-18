/**
 * Rust CLI 调用封装
 * 负责与Rust转换内核通信
 */

import { ref } from 'vue'
import { logger, LOG_KEYS } from '../utils/logger'

export function useRustCLI() {
  const isConverting = ref(false)
  const progress = ref(0)
  const currentFile = ref('')

  /**
   * 执行视频转换
   */
  const convertVideos = async (files, options) => {
    isConverting.value = true
    progress.value = 0

    logger.info(LOG_KEYS.CONVERT_START, 'Starting video conversion', {
      fileCount: files.length,
      codec: options.codec,
      container: options.container
    })

    try {
      const results = []
      
      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        currentFile.value = file.name
        progress.value = Math.round((i / files.length) * 100)

        logger.info(LOG_KEYS.CONVERT_PROGRESS, 'Converting video file', {
          file: file.name,
          progress: progress.value,
          index: i + 1,
          total: files.length
        })

        // 构建Rust CLI参数
        const outputPath = file.path.replace(/\.[^.]+$/, `.${options.container}`)
        const args = [
          'video',
          file.path,
          outputPath,
          '--codec', options.codec,
          '--crf', options.crf.toString()
        ]

        // 添加视频参数
        if (options.speed !== undefined) {
          args.push('--speed', options.speed.toString())
        }
        if (options.gopSize) {
          args.push('--gop', options.gopSize.toString())
        }
        if (options.bframes !== undefined) {
          args.push('--bframes', options.bframes.toString())
        }
        if (options.refs) {
          args.push('--refs', options.refs.toString())
        }
        if (options.pixelFormat && options.pixelFormat !== 'auto') {
          args.push('--pix-fmt', options.pixelFormat)
        }
        if (options.hwAccel && options.hwAccel !== 'auto') {
          args.push('--hw-accel', options.hwAccel)
        }
        if (options.twoPass) {
          args.push('--two-pass')
        }

        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing Rust CLI', { args })

        // 调用Rust CLI
        const result = await executeRustCLI(args)
        results.push(result)
      }

      progress.value = 100
      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Video conversion completed', {
        successCount: results.length
      })
      return { success: true, results }
    } catch (error) {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Video conversion failed', {
        error: error.message,
        file: currentFile.value
      })
      return { success: false, error: error.message }
    } finally {
      isConverting.value = false
      currentFile.value = ''
    }
  }

  /**
   * 执行图像转换
   */
  const convertImages = async (files, options) => {
    isConverting.value = true
    progress.value = 0

    logger.info(LOG_KEYS.CONVERT_START, 'Starting image conversion', {
      fileCount: files.length,
      format: options.format,
      quality: options.quality
    })

    try {
      const results = []
      
      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        currentFile.value = file.name
        progress.value = Math.round((i / files.length) * 100)

        logger.info(LOG_KEYS.CONVERT_PROGRESS, 'Converting image file', {
          file: file.name,
          progress: progress.value,
          index: i + 1,
          total: files.length
        })

        // 构建Rust CLI参数
        const outputPath = file.path.replace(/\.[^.]+$/, `.${options.format}`)
        const args = [
          'convert',
          file.path,
          outputPath,
          '--quality', options.quality.toString()
        ]

        // JXL高级参数
        if (options.format === 'jxl') {
          if (options.effort !== undefined) args.push('--effort', options.effort.toString())
          if (options.distance !== undefined) args.push('--distance', options.distance.toString())
          
          // 防呆处理：JPEG无损转码和数学无损互斥
          // - jpegLossless: 仅用于JPEG输入，无损重新打包
          // - lossless: 通用无损模式，适用于所有输入
          // 优先级：lossless > jpegLossless
          if (options.lossless) {
            // 数学无损模式（全局）
            args.push('--lossless')
            logger.info(LOG_KEYS.PARAM_CHANGE, 'Using mathematical lossless mode', { 
              file: file.name 
            })
          } else if (options.jpegLossless) {
            // JPEG无损转码（仅JPEG输入有效）
            // Rust CLI会自动检测输入格式
            logger.info(LOG_KEYS.PARAM_CHANGE, 'JPEG lossless transcoding enabled', { 
              file: file.name 
            })
            // 注意：不需要传递参数，Rust CLI会自动处理JPEG输入
          }
          
          if (options.bitDepth && options.bitDepth !== 'auto') args.push('--bit-depth', options.bitDepth)
          if (options.colorSpace && options.colorSpace !== 'auto') args.push('--color-space', options.colorSpace)
          if (options.modular) args.push('--modular')
          if (options.progressive) args.push('--progressive')
        }

        // AVIF高级参数
        if (options.format === 'avif') {
          if (options.speed !== undefined) args.push('--speed', options.speed.toString())
          if (options.minQuantizer !== undefined) args.push('--min-quantizer', options.minQuantizer.toString())
          if (options.maxQuantizer !== undefined) args.push('--max-quantizer', options.maxQuantizer.toString())
          if (options.tilesRows && options.tilesRows > 1) args.push('--tiles-rows', options.tilesRows.toString())
          if (options.tilesCols && options.tilesCols > 1) args.push('--tiles-cols', options.tilesCols.toString())
          if (options.chromaSubsampling && options.chromaSubsampling !== 'auto') {
            args.push('--chroma-subsampling', options.chromaSubsampling)
          }
        }

        // WebP高级参数
        if (options.format === 'webp') {
          if (options.method !== undefined) args.push('--method', options.method.toString())
          if (options.filterStrength !== undefined) args.push('--filter-strength', options.filterStrength.toString())
          if (options.sharpness !== undefined) args.push('--sharpness', options.sharpness.toString())
          if (options.lossless) args.push('--lossless')
        }

        // HEIC高级参数
        if (options.format === 'heic') {
          if (options.encoder) args.push('--encoder', options.encoder)
          if (options.chromaSubsampling && options.chromaSubsampling !== 'auto') {
            args.push('--chroma-subsampling', options.chromaSubsampling)
          }
          if (options.lossless) args.push('--lossless')
          if (options.embedThumbnail) args.push('--embed-thumbnail')
        }

        logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Executing Rust CLI', { args })

        // 调用Rust CLI
        const result = await executeRustCLI(args)
        results.push(result)
      }

      progress.value = 100
      logger.info(LOG_KEYS.CONVERT_SUCCESS, 'Image conversion completed', {
        successCount: results.length
      })
      return { success: true, results }
    } catch (error) {
      logger.error(LOG_KEYS.CONVERT_ERROR, 'Image conversion failed', {
        error: error.message,
        file: currentFile.value
      })
      return { success: false, error: error.message }
    } finally {
      isConverting.value = false
      currentFile.value = ''
    }
  }

  /**
   * 执行Rust CLI命令
   */
  const executeRustCLI = async (args) => {
    // 检查Rust CLI是否可用
    if (!window.rustCLI) {
      const error = new Error('Rust CLI not available')
      logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Rust CLI not available')
      throw error
    }

    return new Promise((resolve, reject) => {
      const { spawn } = require('child_process')
      const rustBinary = getRustBinaryPath()

      logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Spawning Rust CLI process', { 
        binary: rustBinary,
        args 
      })

      const process = spawn(rustBinary, args)
      let stdout = ''
      let stderr = ''

      process.stdout.on('data', (data) => {
        stdout += data.toString()
        logger.debug(LOG_KEYS.RUST_CLI_STDOUT, 'Rust CLI output', { output: data.toString() })
      })

      process.stderr.on('data', (data) => {
        stderr += data.toString()
        logger.warn(LOG_KEYS.RUST_CLI_STDERR, 'Rust CLI stderr', { output: data.toString() })
      })

      process.on('close', (code) => {
        if (code === 0) {
          logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI process completed successfully', { code })
          resolve({ success: true, stdout })
        } else {
          const errorMsg = stderr || `Process exited with code ${code}`
          logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Rust CLI process failed', { 
            code, 
            stderr 
          })
          reject(new Error(errorMsg))
        }
      })

      process.on('error', (error) => {
        logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Rust CLI process error', { 
          error: error.message 
        })
        reject(error)
      })
    })
  }

  /**
   * 获取Rust二进制文件路径
   */
  const getRustBinaryPath = () => {
    const path = require('path')
    const os = require('os')
    
    const platform = os.platform()
    const binaryName = platform === 'win32' ? 'pixly_converter_cli.exe' : 'pixly_converter_cli'
    
    // 尝试多个可能的路径
    const possiblePaths = [
      path.join(__dirname, '../../target/release', binaryName),
      path.join(__dirname, '../../target/debug', binaryName),
      path.join(process.cwd(), 'target/release', binaryName),
      path.join(process.cwd(), 'target/debug', binaryName)
    ]

    logger.debug(LOG_KEYS.RUST_CLI_EXEC, 'Searching for Rust CLI binary', { 
      platform,
      binaryName,
      searchPaths: possiblePaths 
    })

    for (const p of possiblePaths) {
      const fs = require('fs')
      if (fs.existsSync(p)) {
        logger.info(LOG_KEYS.RUST_CLI_EXEC, 'Rust CLI binary found', { path: p })
        return p
      }
    }

    const error = new Error('Rust CLI binary not found. Please build the project first.')
    logger.error(LOG_KEYS.RUST_CLI_ERROR, 'Rust CLI binary not found', { 
      searchPaths: possiblePaths 
    })
    throw error
  }

  return {
    isConverting,
    progress,
    currentFile,
    convertImages,
    convertVideos
  }
}
