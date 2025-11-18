import { createApp } from 'vue'
import App from './App.vue'
import './styles/variables.css'

// 默认暗色模式（官方插件风格）
document.documentElement.setAttribute('data-theme', 'dark')

createApp(App).mount('#app')
