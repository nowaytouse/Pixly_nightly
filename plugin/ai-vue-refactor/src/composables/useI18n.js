/**
 * i18n国际化系统
 */

import { ref, computed } from 'vue'
import zhCN from '../i18n/zh_CN.json'
import en from '../i18n/en.json'

const messages = {
  zh_CN: zhCN,
  en: en
}

const currentLocale = ref('zh_CN')

export function useI18n() {
  const t = (key, params = {}) => {
    const keys = key.split('.')
    let value = messages[currentLocale.value]
    
    for (const k of keys) {
      if (value && typeof value === 'object') {
        value = value[k]
      } else {
        return key
      }
    }
    
    if (typeof value === 'string') {
      // 替换参数 {count} -> 实际值
      return value.replace(/\{(\w+)\}/g, (match, param) => {
        return params[param] !== undefined ? params[param] : match
      })
    }
    
    return key
  }

  const setLocale = (locale) => {
    if (messages[locale]) {
      currentLocale.value = locale
    }
  }

  const locale = computed(() => currentLocale.value)

  return {
    t,
    setLocale,
    locale
  }
}
