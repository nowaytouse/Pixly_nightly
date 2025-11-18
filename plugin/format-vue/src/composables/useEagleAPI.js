/**
 * Eagle API 封装
 * 负责与Eagle应用通信
 */

import { ref } from 'vue'
import { logger, LOG_KEYS } from '../utils/logger'

export function useEagleAPI() {
  const selectedItems = ref([])
  const isLoading = ref(false)

  /**
   * 加载Eagle选中的文件
   */
  const loadSelectedFiles = async () => {
    isLoading.value = true
    
    try {
      // 检查Eagle API
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Checking Eagle API availability', {
        hasEagle: !!window.eagle,
        hasItem: !!(window.eagle && window.eagle.item)
      })
      
      if (!window.eagle || !window.eagle.item) {
        const error = new Error('Eagle API not available')
        logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Eagle API not available')
        throw error
      }

      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Calling eagle.item.getSelected()')
      const items = await window.eagle.item.getSelected()
      
      logger.info(LOG_KEYS.EAGLE_API_CALL, 'Raw items received', { 
        count: items ? items.length : 0,
        firstItem: items && items[0] ? {
          id: items[0].id,
          name: items[0].name,
          hasFilePath: !!items[0].filePath
        } : null
      })
      
      if (!items || items.length === 0) {
        logger.warn(LOG_KEYS.EAGLE_API_CALL, 'No items selected in Eagle')
        selectedItems.value = []
        return []
      }
      
      selectedItems.value = items.map(item => ({
        id: item.id,
        name: item.name,
        path: item.filePath,
        size: item.size,
        ext: item.ext,
        width: item.width,
        height: item.height,
        tags: item.tags || [],
        folders: item.folders || [],
        // 缩略图路径 - Eagle协议
        thumbnail: `eagle://item/${item.id}`
      }))

      logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Files loaded successfully', { 
        count: selectedItems.value.length,
        files: selectedItems.value.map(f => f.name)
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
   */
  const refreshLibrary = async () => {
    try {
      if (window.eagle && window.eagle.library) {
        logger.info(LOG_KEYS.EAGLE_API_CALL, 'Refreshing Eagle library')
        await window.eagle.library.refresh()
        logger.info(LOG_KEYS.EAGLE_API_SUCCESS, 'Library refreshed successfully')
      }
    } catch (error) {
      logger.error(LOG_KEYS.EAGLE_API_ERROR, 'Failed to refresh Eagle library', { 
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

  return {
    selectedItems,
    isLoading,
    loadSelectedFiles,
    refreshLibrary,
    showNotification
  }
}
