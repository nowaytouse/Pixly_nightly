/**
 * 国际化封装
 * 参考官方AI插件的i18n实现
 */

import { ref, computed } from 'vue'

const currentLocale = ref('zh_CN')
const messages = ref({})

export function useI18n() {
  /**
   * 加载语言包
   */
  async function loadLocale(locale) {
    try {
      const response = await fetch(`./_locales/${locale}.json`)
      const data = await response.json()
      messages.value[locale] = data
      currentLocale.value = locale
      console.log(`[i18n] Loaded locale: ${locale}`)
    } catch (error) {
      console.error(`[i18n] Failed to load locale ${locale}:`, error)
    }
  }

  /**
   * 翻译函数
   */
  function t(key, params = {}) {
    const keys = key.split('.')
    let value = messages.value[currentLocale.value]
    
    for (const k of keys) {
      if (value && typeof value === 'object') {
        value = value[k]
      } else {
        return key
      }
    }
    
    if (typeof value === 'string') {
      // 替换参数
      return value.replace(/\{\{(\w+)\}\}/g, (match, param) => {
        return params[param] || match
      })
    }
    
    return key
  }

  /**
   * 初始化i18n
   */
  async function initI18n() {
    // 从Eagle获取当前语言
    const eagleLocale = eagle.app.locale || 'zh_CN'
    await loadLocale(eagleLocale)
  }

  return {
    t,
    currentLocale: computed(() => currentLocale.value),
    loadLocale,
    initI18n
  }
}
