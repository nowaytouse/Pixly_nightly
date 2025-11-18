import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import { useI18n } from './composables/useI18n'
import { useEagleAPI } from './composables/useEagleAPI'

const app = createApp(App)
app.use(ElementPlus)

// 等待Eagle插件创建完成
eagle.onPluginCreate(async (plugin) => {
  console.log('[PIXLY AI Optimizer] Plugin created:', plugin)
  
  // 初始化i18n
  const { initI18n } = useI18n()
  await initI18n()
  
  // 应用主题
  const { getTheme, onThemeChanged, getPlatform } = useEagleAPI()
  applyTheme(getTheme(), getPlatform())
  
  // 监听主题变化
  onThemeChanged((theme) => {
    applyTheme(theme, getPlatform())
  })
  
  // 挂载应用
  app.mount('#app')
  
  // 设置窗口透明度
  await eagle.window.setOpacity(1)
  
  console.log('[PIXLY AI Optimizer] Ready!')
})

/**
 * 应用主题
 * 参考官方AI插件的主题系统
 */
function applyTheme(theme, platform) {
  const THEME_MAP = {
    'Auto': eagle.app.isDarkColors() ? 'gray' : 'light',
    'LIGHT': 'light',
    'LIGHTGRAY': 'lightgray',
    'GRAY': 'gray',
    'DARK': 'dark',
    'BLUE': 'blue',
    'PURPLE': 'purple'
  }
  
  const themeName = THEME_MAP[theme] || 'light'
  const htmlEl = document.querySelector('html')
  
  htmlEl.classList.add('no-transition')
  htmlEl.setAttribute('theme', themeName)
  htmlEl.setAttribute('platform', platform)
  
  setTimeout(() => {
    htmlEl.classList.remove('no-transition')
  }, 100)
  
  console.log(`[Theme] Applied: ${themeName}`)
}
