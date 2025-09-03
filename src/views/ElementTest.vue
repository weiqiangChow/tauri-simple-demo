<template>
  <el-form
    ref="formRef"
    style="max-width: 600px"
    :model="dynamicValidateForm"
    label-width="auto"
    class="demo-dynamic"
  >
    <el-form-item
      prop="email"
      label="Email"
      :rules="[
        {
          required: true,
          message: 'Please input email address',
          trigger: 'blur',
        },
        {
          type: 'email',
          message: 'Please input correct email address',
          trigger: ['blur', 'change'],
        },
      ]"
    >
      <el-input v-model="dynamicValidateForm.email" />
    </el-form-item>
    <el-form-item
      v-for="(domain, index) in dynamicValidateForm.domains"
      :key="domain.key"
      :label="'Domain' + index"
      :prop="'domains.' + index + '.value'"
      :rules="{
        required: true,
        message: 'domain can not be null',
        trigger: 'blur',
      }"
    >
      <el-input v-model="domain.value" />
      <el-button class="mt-2" @click.prevent="removeDomain(domain)">
        Delete
      </el-button>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="submitForm(formRef)">Submit</el-button>
      <el-button @click="addDomain">New domain</el-button>
      <el-button @click="resetForm(formRef)">Reset</el-button>
    </el-form-item>

  </el-form>
</template>

<style scoped>
  .login-container {
    max-width: 400px;
    margin: 60px auto;
    padding: 32px 24px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  }
  .login-form {
    margin-bottom: 16px;
  }
  .login-alert {
    margin-top: 16px;
  }
  /* 统一 Element Plus input 阴影表现 */
  .el-input__wrapper {
    box-shadow: none !important;
  }
  .el-input__wrapper:focus-within {
    box-shadow: 0 0 0 2px #396cd8 !important;
  }
</style>

<script lang="ts" setup>
import { reactive, ref } from 'vue'

import type { FormInstance } from 'element-plus'

const formRef = ref<FormInstance>()
const dynamicValidateForm = reactive<{
  domains: DomainItem[]
  email: string
}>({
  domains: [
    {
      key: 1,
      value: '',
    },
  ],
  email: '',
})

interface DomainItem {
  key: number
  value: string
}

const removeDomain = (item: DomainItem) => {
  const index = dynamicValidateForm.domains.indexOf(item)
  if (index !== -1) {
    dynamicValidateForm.domains.splice(index, 1)
  }
}

const addDomain = () => {
  dynamicValidateForm.domains.push({
    key: Date.now(),
    value: '',
  })
}

const submitForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.validate((valid) => {
    if (valid) {
      console.log('submit!')
    } else {
      console.log('error submit!')
    }
  })
}

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
}
</script>