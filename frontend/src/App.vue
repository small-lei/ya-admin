<template>
  <el-container class="app-wrapper">
    <el-header class="header" v-if="isLoggedIn">
      <div class="logo">眼镜管理系统</div>
      <div class="user-info">
        <el-dropdown>
          <span class="el-dropdown-link">
            {{ username }}
            <el-icon class="el-icon--right"><arrow-down /></el-icon>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="handleLogout">退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </el-header>
    <el-container>
      <el-aside width="200px" v-if="isLoggedIn">
        <el-menu
          :default-active="activeMenu"
          class="el-menu-vertical"
          @select="handleSelect"
        >
          <el-menu-item index="/orders">
            <el-icon><document /></el-icon>
            <span>订单管理</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main>
        <router-view></router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { ArrowDown, Document } from '@element-plus/icons-vue'

const router = useRouter()
const route = useRoute()
const isLoggedIn = ref(false)
const username = ref('admin') // 默认用户名，可以从用户信息接口获取
const activeMenu = ref('')

// 初始化时检查登录状态
onMounted(() => {
  checkLoginStatus()
  activeMenu.value = route.path
})

// 监听路由变化，更新激活菜单
watch(() => route.path, (newPath) => {
  activeMenu.value = newPath
})

// 检查登录状态
const checkLoginStatus = () => {
  const token = localStorage.getItem('token')
  isLoggedIn.value = !!token
  
  // 如果当前路径是根路径或登录页，且已登录，则重定向到订单页
  if ((route.path === '/' || route.path === '/login') && isLoggedIn.value) {
    router.push('/orders')
  }
  // 如果需要认证的页面但未登录，重定向到登录页
  else if (route.meta.requiresAuth && !isLoggedIn.value) {
    router.push('/login')
  }
}

const handleLogout = () => {
  // 清除登录状态和token
  localStorage.removeItem('token')
  isLoggedIn.value = false
  router.push('/login')
}

const handleSelect = (key: string) => {
  router.push(key)
}
</script>

<style scoped>
.app-wrapper {
  height: 100vh;
}

.header {
  background-color: #fff;
  border-bottom: 1px solid #dcdfe6;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-size: 20px;
  font-weight: bold;
  color: #409eff;
}

.el-menu-vertical {
  height: 100%;
  border-right: none;
}

.user-info {
  cursor: pointer;
}

.el-dropdown-link {
  display: flex;
  align-items: center;
  color: #606266;
}
</style>