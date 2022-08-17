<script setup lang="ts">
import { ref, reactive } from "vue";
import { useAuthStore } from "~/stores/auth.store";

const form = reactive({
  code: "",
  mail: "",
});
let disabled: boolean = false;
async function onSubmit() {
  disabled = true;
  const authStore = useAuthStore();
  await authStore.login(form.code, form.mail);
  disabled = false;
}
</script>

<template>
  <div>
    <div class="alert alert-info">
      <el-row justify="center">
        <el-col :span="10">
          <h2>Login</h2>
          <p>Enter your code here to login.</p>
        </el-col>
      </el-row>
      <el-row class="row-bg" justify="center">
        <el-col :span="10">
          <el-form class="form" :model="form">
            <el-alert type="info" show-icon :closable="true">
              <p>Code was given to you by your administrator.</p>
            </el-alert>
            <el-form-item label="Your email address">
              <el-input
                placeholder="Enter here your email"
                :disabled="disabled"
                v-model="form.mail"
              />
            </el-form-item>
            <el-form-item label="Access code">
              <el-input
                placeholder="Enter here your code"
                :disabled="disabled"
                v-model="form.code"
                type="password"
                show-password
              />
            </el-form-item>

            <el-form-item class="form">
              <el-button type="primary" @click="onSubmit">Login</el-button>
            </el-form-item>
          </el-form>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<style>
.form {
  justify-content: center;
  text-align: center;
}
</style>
