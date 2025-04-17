<template>
  <div class="login-container">
    <el-card class="login-card">
      <template #header>
        <h2 class="login-title">眼镜管理系统</h2>
      </template>
      <el-form
        ref="loginForm"
        :model="loginData"
        :rules="rules"
        label-width="0"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="username">
          <el-input
            v-model="loginData.username"
            placeholder="用户名"
            prefix-icon="User"
          />
        </el-form-item>
        <el-form-item prop="password">
          <el-input
            v-model="loginData.password"
            type="password"
            placeholder="密码"
            prefix-icon="Lock"
            show-password
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" native-type="submit" :loading="loading" class="login-button">
            登录
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock } from '@element-plus/icons-vue'
import type { FormInstance } from 'element-plus'
import { authApi } from '../api'

const router = useRouter()
const loginForm = ref<FormInstance>()
const loading = ref(false)

const loginData = reactive({
  username: '',
  password: ''
})

const rules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

const handleLogin = async () => {
  if (!loginForm.value) return

  await loginForm.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        console.log('正在调用登录API...');
        
        // 使用API服务调用登录接口
        const response = await authApi.login(loginData.username, loginData.password);
        
        // 登录成功处理
        if (response && response.token) {
          localStorage.setItem('token', response.token);
          ElMessage.success('登录成功');
          window.location.href = '/orders';
        } else {
          ElMessage.error('登录失败，返回数据格式异常');
        }
      } catch (error: any) {
        console.error('登录请求异常:', error);
        
        // 显示具体错误信息
        const errorMessage = error.message || '网络错误，无法连接到服务器';
        ElMessage.error(errorMessage);
        
        // 开发环境下的调试选项
        if (import.meta.env.DEV) {
          const useDevLogin = confirm('是否使用开发模式登录? (仅用于前端开发测试)');
          if (useDevLogin) {
            localStorage.setItem('token', 'dev-token');
            ElMessage.warning('使用开发模式登录');
            window.location.href = '/orders';
          }
        }
      } finally {
        loading.value = false;
      }
    }
  });
};
</script>

<style scoped>
.login-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f5f7fa;
}

.login-card {
  width: 400px;
}

.login-title {
  text-align: center;
  color: #409eff;
  margin: 0;
}

.login-button {
  width: 100%;
}
</style>