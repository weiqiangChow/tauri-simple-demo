<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted } from 'vue'

// 初始化时获取系统主题并同步到 Rust 后端
onMounted(async () => {
  try {
    let isDark;
    /**
     * 获取系统主题的命令
     * 对于 macOS，返回 true 表示深色模式，false 表示浅色模式
     */
    const systemIsDark = await invoke<boolean>('get_system_theme')
    // 如果没有用户偏好设置，使用系统主题
    const theme = localStorage.getItem('system-theme')
    if (theme) {
      isDark = theme === 'dark'
    } else {
      isDark = systemIsDark
    }
    // 同步当前主题到 Rust 后端
    const res = await invoke<string>('set_window_theme', { isDark: isDark })
    console.log('初始化主题结果:', res)
  } catch (error) {
    console.error('初始化主题失败:', error)
  }
})
</script>


<template>
  <main class="container">
    <nav class="navbar">
      <router-link to="/">Home</router-link>
      <router-link to="/great">Great</router-link>
      <router-link to="/element-test">ElementTest</router-link>
      <router-link to="/config-setting">ConfigSetting</router-link>
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