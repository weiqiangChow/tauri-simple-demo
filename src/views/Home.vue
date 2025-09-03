<template>
  <div>
    <h2>首页内容</h2>
    <p>这是 Home 页面。</p>
  </div>
</template>

<script setup lang="ts">
// ...可根据需要添加逻辑
</script>

<style scoped>
nav.navbar {
  display: flex;
  gap: 16px;
  justify-content: center;
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
h2 {
  color: #396cd8;
}
</style>
