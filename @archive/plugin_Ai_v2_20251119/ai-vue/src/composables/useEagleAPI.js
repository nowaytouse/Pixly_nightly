/**
 * 🦅 Eagle API 集成 Composable
 * 
 * 封装 Eagle Plugin API 调用
 * 参考：https://developer.eagle.cool/plugin-api
 */

import { ref } from 'vue'
import { ElMessage } from 'element-plus'

export function useEagleAPI() {
  const isEagleAvailable = ref(false)
  const selectedItems = ref([])

  /**
   * 检测 Eagle API 是否可用
   */
  const detectEagle = () => {
    isEagleAvailable.value = typeof window.eagle !== 'undefined'
    
    if (isEagleAvailable.value) {
      console.log('✅ Eagle API detected')
    } else {
      console.warn('⚠️ Eagle API not available (running in dev mode)')
    }
    
    return isEagleAvailable.value
  }

  /**
   * 获取选中的文件
   */
  const getSelectedItems = async () => {
    if (!isEagleAvailable.value) {
      console.warn('Eagle API not available, returning mock data')
      return getMockItems()
    }

    try {
      const items = await window.eagle.item.getSelected()
      
      selectedItems.value = items.map(item => ({
        id: item.id,
        name: item.name,
        ext: item.ext,
        path: item.filePath,
        thumbnail: item.thumbnail,
        width: item.width || 0,
        height: item.height || 0,
        size: item.size || 0,
        tags: item.tags || [],
        folders: item.folders || []
      }))

      console.log(`✅ Loaded ${selectedItems.value.length} items from Eagle`)
      return selectedItems.value
    } catch (error) {
      console.error('❌ Failed to get selected items:', error)
      ElMessage.error('获取 Eagle 文件失败')
      throw error
    }
  }

  /**
   * 刷新 Eagle 库
   */
  const refreshLibrary = async () => {
    if (!isEagleAvailable.value) {
      console.warn('Eagle API not available')
      return
    }

    try {
      await window.eagle.library.refresh()
      console.log('✅ Eagle library refreshed')
    } catch (error) {
      console.error('❌ Failed to refresh library:', error)
    }
  }

  /**
   * 显示通知
   */
  const showNotification = (options) => {
    if (!isEagleAvailable.value) {
      console.log('Notification:', options)
      return
    }

    window.eagle.notification.show(options)
  }

  /**
   * Mock 数据（用于本地开发）
   */
  const getMockItems = () => {
    return [
      {
        id: 'mock-1',
        name: 'sample-photo-1.jpg',
        ext: 'jpg',
        path: '/mock/path/sample-photo-1.jpg',
        thumbnail: 'https://picsum.photos/200/200?random=1',
        width: 1920,
        height: 1080,
        size: 2048576,
        tags: ['photo', 'landscape'],
        folders: []
      },
      {
        id: 'mock-2',
        name: 'sample-photo-2.png',
        ext: 'png',
        path: '/mock/path/sample-photo-2.png',
        thumbnail: 'https://picsum.photos/200/200?random=2',
        width: 1280,
        height: 720,
        size: 1048576,
        tags: ['photo'],
        folders: []
      },
      {
        id: 'mock-3',
        name: 'sample-photo-3.webp',
        ext: 'webp',
        path: '/mock/path/sample-photo-3.webp',
        thumbnail: 'https://picsum.photos/200/200?random=3',
        width: 2560,
        height: 1440,
        size: 3145728,
        tags: ['photo', 'portrait'],
        folders: []
      }
    ]
  }

  return {
    // State
    isEagleAvailable,
    selectedItems,

    // Methods
    detectEagle,
    getSelectedItems,
    refreshLibrary,
    showNotification,
    getMockItems
  }
}
