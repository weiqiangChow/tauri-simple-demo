<script setup lang="ts">
import { useDark, useToggle } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { onMounted, watch } from 'vue'

const isDark = useDark({
  // 存储键名
  storageKey: 'vueuse-color-scheme',
  // 选择器，将 class 添加到 html 元素
  selector: 'html',
  // class 名称
  attribute: 'class',
  // 深色模式的 class 名
  valueDark: 'dark',
  // 浅色模式的 class 名
  valueLight: 'light',
})

// 创建切换函数
const toggleDark = useToggle(isDark)

// 自定义切换函数，同时调用 Rust 后端
const handleThemeToggle = async () => {
  toggleDark()
  
  // 调用 Rust 后端设置窗口背景色
  try {
    await invoke('set_window_theme', { isDark: isDark.value })
  } catch (error) {
    console.error('设置窗口主题失败:', error)
  }
}

// DevTools 控制函数
const openDevTools = async () => {
  try {
    await invoke('open_devtools')
  } catch (error) {
    console.error('打开 DevTools 失败:', error)
  }
}

const closeDevTools = async () => {
  try {
    await invoke('close_devtools')
  } catch (error) {
    console.error('关闭 DevTools 失败:', error)
  }
}

// 初始化时获取系统主题并同步到 Rust 后端
onMounted(async () => {
  try {
    // 获取系统主题
    const systemIsDark = await invoke('get_system_theme') as boolean
    
    // 如果没有用户偏好设置，使用系统主题
    const storedTheme = localStorage.getItem('vueuse-color-scheme')
    if (!storedTheme) {
      isDark.value = systemIsDark
    }
    
    // 同步当前主题到 Rust 后端
    await invoke('set_window_theme', { isDark: isDark.value })
  } catch (error) {
    console.error('初始化主题失败:', error)
  }
})

// 监听主题变化，同步到 Rust 后端
watch(isDark, async (newValue) => {
  try {
    await invoke('set_window_theme', { isDark: newValue })
  } catch (error) {
    console.error('同步主题到后端失败:', error)
  }
})
</script>


<template>
  <main class="container">
    <nav class="navbar">
      <router-link to="/">Home</router-link>
      <router-link to="/great">Great</router-link>
      <router-link to="/element-test">ElementTest</router-link>
      <button @click="handleThemeToggle()" class="theme-toggle">
        {{ isDark ? '🌙' : '☀️' }}
      </button>
      <button @click="openDevTools()" class="devtools-btn">
        🔧 DevTools
      </button>
    </nav>
    <router-view />
  </main>
</template>

<style scoped>

nav.navbar {
  display: flex;
  gap: 16px;
  justify-content: center;
  align-items: center;
  margin-bottom: 24px;
}
nav.navbar a {
  color: #396cd8;
  text-decoration: none;
  font-weight: bold;
  padding: 6px 12px;
  border-radius: 4px;
  transition: background 0.2s;
}
nav.navbar a.router-link-exact-active {
  background: #e8e8e8;
}

.theme-toggle {
  font-size: 1.2em;
  padding: 8px 12px;
  margin-left: 16px;
  border: 2px solid #396cd8;
  border-radius: 8px;
  background: transparent;
  cursor: pointer;
  transition: all 0.2s ease;
}

.theme-toggle:hover {
  background: #396cd8;
  transform: scale(1.05);
}

.devtools-btn {
  font-size: 0.9em;
  padding: 6px 10px;
  margin-left: 8px;
  border: 2px solid #28a745;
  border-radius: 6px;
  background: transparent;
  color: #28a745;
  cursor: pointer;
  transition: all 0.2s ease;
}

.devtools-btn:hover {
  background: #28a745;
  color: white;
  transform: scale(1.05);
}

</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

/* 浅色模式的颜色 */
:root,
:root.light {
  color: #213547;
  background-color: #ffffff;
}

/* 深色模式的颜色 */
:root.dark {
  color: #f6f6f6;
  background-color: #242424;
}

/* 系统偏好设置的后备方案 */
@media (prefers-color-scheme: dark) {
  :root:not(.light):not(.dark) {
    color: #f6f6f6;
    background-color: #242424;
  }
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

h1 {
  text-align: center;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

</style>