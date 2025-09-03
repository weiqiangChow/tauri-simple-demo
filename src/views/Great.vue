<template>
  <div>
    <h2>打招呼页面</h2>
    <el-form :model="form" @submit.prevent="greet">
      <el-input id="greet-input" v-model="form.name" placeholder="请输入名字..." />
      <el-button type="primary" @click="greet">Greet</el-button>
    </el-form>
    <p>{{ greetMsg }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";

const form = reactive({
  name: "",
  age: 0
});
const greetMsg = ref("");

async function greet() {
  greetMsg.value = await invoke("say_hello", { userName: form.name });
}
</script>

<style scoped>
h2 {
  color: #24c8db;
}
.row {
  display: flex;
  justify-content: center;
  margin-top: 16px;
}
#greet-input {
  margin-right: 5px;
}
</style>
