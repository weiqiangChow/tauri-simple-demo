import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import { useInit } from './init'
const { initPlatform } = useInit()

import 'element-plus/theme-chalk/dark/css-vars.css'
import './style/platform.css'

const app = createApp(App)


app.use(router)
app.mount('#app')


initPlatform()
