/**
 * 文件类型定义和验证
 */

export const IMAGE_FORMATS = {
  jxl: { name: 'JXL', extensions: ['.jxl'] },
  avif: { name: 'AVIF', extensions: ['.avif'] },
  webp: { name: 'WebP', extensions: ['.webp'] },
  heic: { name: 'HEIC', extensions: ['.heic', '.heif'] },
  png: { name: 'PNG', extensions: ['.png'] },
  jpg: { name: 'JPEG', extensions: ['.jpg', '.jpeg'] },
  gif: { name: 'GIF', extensions: ['.gif'] }
}

export const VIDEO_FORMATS = {
  mp4: { name: 'MP4', extensions: ['.mp4'] },
  mov: { name: 'MOV', extensions: ['.mov'] },
  webm: { name: 'WebM', extensions: ['.webm'] },
  mkv: { name: 'MKV', extensions: ['.mkv'] },
  avi: { name: 'AVI', extensions: ['.avi'] }
}

export const SUPPORTED_IMAGE_EXTENSIONS = Object.values(IMAGE_FORMATS)
  .flatMap(f => f.extensions)

export const SUPPORTED_VIDEO_EXTENSIONS = Object.values(VIDEO_FORMATS)
  .flatMap(f => f.extensions)

export function isImageFile(filename) {
  const ext = getFileExtension(filename)
  return SUPPORTED_IMAGE_EXTENSIONS.includes(ext)
}

export function isVideoFile(filename) {
  const ext = getFileExtension(filename)
  return SUPPORTED_VIDEO_EXTENSIONS.includes(ext)
}

export function getFileExtension(filename) {
  const match = filename.match(/\.[^.]+$/)
  return match ? match[0].toLowerCase() : ''
}

export function getFileType(filename) {
  if (isImageFile(filename)) return 'image'
  if (isVideoFile(filename)) return 'video'
  return 'unknown'
}

export function formatFileSize(bytes) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB'
}
