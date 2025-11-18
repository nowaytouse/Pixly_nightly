/**
 * Rust CLI 调用封装
 * 负责与Rust转换内核通信
 */

import { ref } from 'vue'

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

    try {
      const results = []
      
      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        currentFile.value = file.name
        progress.value = Math.round((i / files.length) * 100)

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
        if (options.preset) {
          args.push('--preset', options.preset)
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

        // 调用Rust CLI
        const result = await executeRustCLI(args)
        results.push(result)
      }

      progress.value = 100
      return { success: true, results }
    } catch (error) {
      console.error('Video conversion failed:', error)
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

    try {
      const results = []
      
      for (let i = 0; i < files.length; i++) {
        const file = files[i]
        currentFile.value = file.name
        progress.value = Math.round((i / files.length) * 100)

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
          if (options.effort !== undefined) {
            args.push('--effort', options.effort.toString())
          }
          if (options.distance !== undefined) {
            args.push('--distance', options.distance.toString())
          }
          if (options.jpegLossless) {
            args.push('--jpeg-lossless')
          }
          if (options.lossless) {
            args.push('--lossless')
          }
        }

        // AVIF高级参数
        if (options.format === 'avif') {
          if (options.speed !== undefined) {
            args.push('--speed', options.speed.toString())
          }
          if (options.minQuantizer !== undefined) {
            args.push('--min-quantizer', options.minQuantizer.toString())
          }
          if (options.maxQuantizer !== undefined) {
            args.push('--max-quantizer', options.maxQuantizer.toString())
          }
        }

        // WebP高级参数
        if (options.format === 'webp') {
          if (options.method !== undefined) {
            args.push('--method', options.method.toString())
          }
          if (options.lossless) {
            args.push('--lossless')
          }
        }

        // HEIC高级参数
        if (options.format === 'heic') {
          if (options.encoder) {
            args.push('--encoder', options.encoder)
          }
          if (options.lossless) {
            args.push('--lossless')
          }
        }

        // 调用Rust CLI
        const result = await executeRustCLI(args)
        results.push(result)
      }

      progress.value = 100
      return { success: true, results }
    } catch (error) {
      console.error('Conversion failed:', error)
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
      throw new Error('Rust CLI not available')
    }

    return new Promise((resolve, reject) => {
      const { spawn } = require('child_process')
      const rustBinary = getRustBinaryPath()

      const process = spawn(rustBinary, args)
      let stdout = ''
      let stderr = ''

      process.stdout.on('data', (data) => {
        stdout += data.toString()
        console.log('[Rust CLI]', data.toString())
      })

      process.stderr.on('data', (data) => {
        stderr += data.toString()
        console.error('[Rust CLI Error]', data.toString())
      })

      process.on('close', (code) => {
        if (code === 0) {
          resolve({ success: true, stdout })
        } else {
          reject(new Error(stderr || `Process exited with code ${code}`))
        }
      })

      process.on('error', (error) => {
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

    for (const p of possiblePaths) {
      const fs = require('fs')
      if (fs.existsSync(p)) {
        return p
      }
    }

    throw new Error('Rust CLI binary not found. Please build the project first.')
  }

  return {
    isConverting,
    progress,
    currentFile,
    convertImages,
    convertVideos
  }
}
