<template>
    <el-form ref="ruleFormRef" style="max-width: 600px" :model="ruleForm" label-width="auto">
        <el-form-item label="深浅主题色">
            <ElButton @click="handleThemeToggle()" class="theme-toggle">
                {{ isDark ? '🌙' : '☀️' }}
            </ElButton>
        </el-form-item>
        <el-form-item label="Devtools">
            <ElButton @click="openDevTools()" class="devtools-btn">
                🔧 Open DevTools
            </ElButton>
            <ElButton @click="closeDevTools()" class="devtools-btn">
                ❌ Close DevTools
            </ElButton>
        </el-form-item>
    </el-form>


</template>

<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useDark, useToggle } from '@vueuse/core'

const ruleForm = reactive({});

const isDark = useDark({
    storageKey: 'system-theme',
    selector: 'html',
    attribute: 'class',
    valueDark: 'dark',
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

</script>

<style scoped></style>
