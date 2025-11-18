/**
 * Eagle API 封装
 * 负责与Eagle应用通信
 * 
 * 🔥 重要：必须在 eagle.onPluginCreate 回调后才能调用 Eagle API
 */

import { ref } from 'vue'
import { logger, LOG_KEYS } from '../utils/logger'
import { isImageFile, isVideoFile } from '../utils/fileTypes'

export function useEagleAPI() {
  const selectedItems = ref([])
  const isLoading = ref(false)
  const isEagleReady = ref(false)

  /**
   * 加载Eagle选中的文件
   * 🔥 必须在 eagle.onPluginCreate 回调后调用
   */
  const loadSelectedFiles = async () => {
    isLoading.value = true
    
    try {
      // 检查Eagle API
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Checking Eagle API availability', {
        hasEagle: !!window.eagle,
        hasItem: !!(window.eagle && window.eagle.item),
        isReady: isEagleReady.value
      })
      
      if (!window.eagle || !window.eagle.item) {
        const error = new Error('Eagle API not available')
        logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Eagle API not available')
        throw error
      }

      if (!isEagleReady.value) {
        logger.warn(LOG_KEYS.EAGLE_API_CALL, 'Eagle not ready, waiting for plugin-create event')
        return []
      }

      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Calling eagle.item.getSelected()')
      const items = await window.eagle.item.getSelected()
      
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Raw items received', { 
        count: items ? items.length : 0,
        firstItem: items && items[0] ? {
          id: items[0].id,
          name: items[0].name,
          ext: items[0].ext,
          filePath: items[0].filePath,
          hasFilePath: !!items[0].filePath,
          hasThumbnailURL: !!items[0].thumbnailURL,
          thumbnailURL: items[0].thumbnailURL,
          allKeys: Object.keys(items[0])
        } : null
      })
      
      if (!items || items.length === 0) {
        logger.warn(LOG_KEYS.EAGLE_API_CALL, 'No items selected in Eagle')
        selectedItems.value = []
        return []
      }
      
      // 🔥 分离图像/视频文件和 XMP 文件
      const mediaFiles = []
      const xmpFiles = []
      const allFiles = [] // 包含所有文件（用于显示）
      
      items.forEach(item => {
        const filename = `${item.name}.${item.ext}`
        
        if (item.ext.toLowerCase() === 'xmp') {
          // XMP 文件 - 保留用于配对
          xmpFiles.push(item)
          // 🔥 不要展开 Proxy 对象，直接标记
          item._isXmp = true
          allFiles.push(item)
        } else if (isImageFile(filename) || isVideoFile(filename)) {
          // 媒体文件
          mediaFiles.push(item)
          // 🔥 不要展开 Proxy 对象，直接标记
          item._isXmp = false
          allFiles.push(item)
        } else {
          // 其他文件（缩略图等）- 忽略
          logger.warn(LOG_KEYS.EAGLE_API_CALL, 'Skipping non-media file', {
            name: item.name,
            ext: item.ext
          })
        }
      })
      
      if (mediaFiles.length === 0) {
        logger.warn(LOG_KEYS.EAGLE_API_CALL, 'No valid media files selected')
        selectedItems.value = []
        return []
      }
      
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Separated files', {
        total: items.length,
        media: mediaFiles.length,
        xmp: xmpFiles.length,
        ignored: items.length - mediaFiles.length - xmpFiles.length
      })
      
      // 🔥 为每个媒体文件匹配对应的 XMP（基于文件名）
      const xmpMap = new Map()
      const xmpIdMap = new Map()
      xmpFiles.forEach(xmp => {
        xmpMap.set(xmp.name, xmp.filePath)
        xmpIdMap.set(xmp.name, xmp.id) // 🔥 保存 XMP 的 ID 用于删除
      })
      
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'XMP files available', {
        count: xmpMap.size,
        names: Array.from(xmpMap.keys())
      })
      
      // 🔥 构建完整文件列表（包含 XMP，用于显示）
      selectedItems.value = allFiles.map(item => {
        // 处理缩略图路径
        let thumbnail = null
        if (item.thumbnailURL) {
          if (!item.thumbnailURL.startsWith('http') && !item.thumbnailURL.startsWith('file://')) {
            thumbnail = `file://${item.thumbnailURL}`
          } else {
            thumbnail = item.thumbnailURL
          }
        }
        
        // 如果是媒体文件，匹配对应的 XMP
        let xmpPath = null
        let xmpId = null
        let hasXmp = false
        
        if (!item._isXmp) {
          xmpPath = xmpMap.get(item.name) || null
          xmpId = xmpIdMap.get(item.name) || null
          hasXmp = !!xmpPath
          
          if (hasXmp) {
            logger.info(LOG_KEYS.EAGLE_API_CALL, 'Matched XMP for media file', {
              media: item.name,
              xmp: xmpPath,
              xmpId: xmpId
            })
          }
        }
        
        // 🔥 直接返回原始对象，添加额外字段
        return {
          id: item.id,
          name: item.name,
          path: item.filePath,
          xmpPath: xmpPath,
          xmpId: xmpId, // 🔥 XMP 的 Eagle ID
          hasXmp: hasXmp,
          isXmp: item._isXmp || false,
          size: item.size,
          ext: item.ext,
          width: item.width,
          height: item.height,
          tags: item.tags || [],
          folders: item.folders || [],
          isAnimated: item.isAnimated || false,
          thumbnail: thumbnail
        }
      })

      logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Files loaded successfully', { 
        count: selectedItems.value.length,
        files: selectedItems.value.map(f => ({
          name: f.name,
          ext: f.ext,
          isXmp: f.isXmp,
          hasXmp: f.hasXmp,
          hasThumbnail: !!f.thumbnail,
          hasPath: !!f.path
        }))
      })

      return selectedItems.value
    } catch (error) {
      logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Failed to load Eagle files', { 
        error: error.message,
        stack: error.stack
      })
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 刷新Eagle库
   * 🔥 注意：Eagle API 不提供 library.refresh() 方法
   * 用户需要手动刷新 Eagle 或重新选择文件
   */
  const refreshLibrary = async () => {
    try {
      // 🔥 Eagle API 不支持 library.refresh()
      // 只能重新加载选中的文件
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Reloading selected files (Eagle does not support library.refresh)')
      await loadSelectedFiles()
      logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Files reloaded successfully')
    } catch (error) {
      logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Failed to reload files', { 
        error: error.message 
      })
    }
  }

  /**
   * 显示通知
   */
  const showNotification = (message, type = 'info') => {
    if (window.eagle && window.eagle.notification) {
      window.eagle.notification.show({
        title: 'PIXLY Format',
        description: message,
        type: type // 'info', 'success', 'warning', 'error'
      })
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Notification shown', { type, message })
    } else {
      // Fallback: log only
      logger.warn(LOG_KEYS.EAGLE_API_ERROR, 'Eagle notification API not available, message logged', { 
        type, 
        message 
      })
    }
  }

  /**
   * 设置 Eagle 就绪状态
   * 🔥 必须在 eagle.onPluginCreate 回调中调用
   */
  const setEagleReady = () => {
    isEagleReady.value = true
    logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Eagle API ready')
  }

  return {
    selectedItems,
    isLoading,
    isEagleReady,
    loadSelectedFiles,
    refreshLibrary,
    showNotification,
    setEagleReady
  }
}
