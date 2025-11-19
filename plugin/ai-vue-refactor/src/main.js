import { createApp } from 'vue'
import App from './App.vue'
import './styles/variables.css'

// Default dark theme (official plugin style)
document.documentElement.setAttribute('data-theme', 'dark')

createApp(App).mount('#app')
