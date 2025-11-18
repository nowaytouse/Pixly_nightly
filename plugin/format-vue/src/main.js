import { createApp } from 'vue'
import App from './App.vue'
import './styles/global.css'
import { logger, LOG_KEYS } from './utils/logger'

logger.info(LOG_KEYS.APP_INIT, 'PIXLY Format Vue application initializing')

const app = createApp(App)

app.config.errorHandler = (err, instance, info) => {
  logger.error(LOG_KEYS.APP_ERROR, 'Vue application error', {
    error: err.message,
    info,
    stack: err.stack
  })
}

app.mount('#app')

logger.info(LOG_KEYS.APP_MOUNT, 'PIXLY Format Vue application mounted successfully')
