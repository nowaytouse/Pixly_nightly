/**
 * Eagle API 封装
 * 负责与Eagle应用通信
 */

import { ref } from 'vue'

export function useEagleAPI() {
  const selectedItems = ref([])
  const isLoading = ref(false)

  /**
   * 加载Eagle选中的文件
   */
  const loadSelectedFiles = async () => {
    isLoading.value = true
    
    try {
      if (!window.eagle || !window.eagle.item) {
        throw new Error('Eagle API not available')
      }

      const items = await window.eagle.item.getSelected()
      
      selectedItems.value = items.map(item => ({
        id: item.id,
        name: item.name,
        path: item.filePath,
        size: item.size,
        ext: item.ext,
        width: item.width,
        height: item.height,
        tags: item.tags || [],
        folders: item.folders || []
      }))

      return selectedItems.value
    } catch (error) {
      console.error('Failed to load Eagle files:', error)
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
        await window.eagle.library.refresh()
      }
    } catch (error) {
      console.error('Failed to refresh Eagle library:', error)
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
    } else {
      // Fallback to console
      console.log(`[${type.toUpperCase()}]`, message)
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
