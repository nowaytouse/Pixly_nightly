/**
 * Eagle API 封装
 * 参考format-vue插件的实现
 */

export function useEagleAPI() {
  /**
   * 获取Eagle选中的文件
   */
  async function getSelectedItems() {
    try {
      const items = await eagle.item.getSelected()
      return items.map(item => ({
        id: item.id,
        name: item.name,
        ext: item.ext,
        size: item.size,
        filePath: item.filePath,
        thumbnail: item.thumbnail,
        width: item.width,
        height: item.height,
        tags: item.tags || [],
        folders: item.folders || []
      }))
    } catch (error) {
      console.error('[Eagle API] Failed to get selected items:', error)
      throw error
    }
  }

  /**
   * 替换Eagle中的文件
   */
  async function replaceFile(itemId, newFilePath) {
    try {
      const item = await eagle.item.getById(itemId)
      await item.replaceFile(newFilePath)
      console.log(`[Eagle API] File replaced: ${itemId}`)
    } catch (error) {
      console.error('[Eagle API] Failed to replace file:', error)
      throw error
    }
  }

  /**
   * 获取Eagle主题
   */
  function getTheme() {
    return eagle.app.theme
  }

  /**
   * 监听主题变化
   */
  function onThemeChanged(callback) {
    eagle.onThemeChanged(callback)
  }

  /**
   * 获取平台信息
   */
  function getPlatform() {
    return eagle.app.platform
  }

  /**
   * 是否为暗色主题
   */
  function isDarkTheme() {
    return eagle.app.isDarkColors()
  }

  return {
    getSelectedItems,
    replaceFile,
    getTheme,
    onThemeChanged,
    getPlatform,
    isDarkTheme
  }
}
