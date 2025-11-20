import { createApp } from 'vue'
import App from './App.vue'
import './styles/variables.css'

// Default dark theme (official plugin style)
document.documentElement.setAttribute('data-theme', 'dark')

const app = createApp(App)

// Note: $t will be provided by App.vue's provide('t', t)
// Child components can use inject('t') or access via $t in templates

app.mount('#app')
