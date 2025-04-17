<template>
  <div class="orders-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span>订单管理</span>
          <el-button type="primary" @click="handleCreate">新建订单</el-button>
        </div>
      </template>

      <el-table :data="orders" style="width: 100%" v-loading="loading">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="customer_name" label="客户姓名" width="120" />
        <el-table-column prop="phone" label="联系电话" width="120" />
        <el-table-column prop="prescription" label="处方信息" />
        <el-table-column prop="frame_type" label="镜框类型" width="120" />
        <el-table-column prop="lens_type" label="镜片类型" width="120" />
        <el-table-column prop="total_amount" label="总金额" width="120">
          <template #default="{ row }">
            ¥{{ row.total_amount }}
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="row.status === '已完成' ? 'success' : 'warning'">
              {{ row.status }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button link type="primary" @click="handleEdit(row)">编辑</el-button>
            <el-button link type="danger" @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <div class="pagination">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :total="total"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </el-card>

    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'create' ? '新建订单' : '编辑订单'"
      width="50%"
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="100px"
      >
        <el-form-item label="客户姓名" prop="customer_name">
          <el-input v-model="form.customer_name" />
        </el-form-item>
        <el-form-item label="联系电话" prop="phone">
          <el-input v-model="form.phone" />
        </el-form-item>
        <el-form-item label="处方信息" prop="prescription">
          <el-input
            v-model="form.prescription"
            type="textarea"
            :rows="3"
          />
        </el-form-item>
        <el-form-item label="镜框类型" prop="frame_type">
          <el-input v-model="form.frame_type" />
        </el-form-item>
        <el-form-item label="镜片类型" prop="lens_type">
          <el-input v-model="form.lens_type" />
        </el-form-item>
        <el-form-item label="总金额" prop="total_amount">
          <el-input-number
            v-model="form.total_amount"
            :precision="2"
            :step="0.1"
            :min="0"
          />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-select v-model="form.status">
            <el-option label="待处理" value="待处理" />
            <el-option label="处理中" value="处理中" />
            <el-option label="已完成" value="已完成" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'

const loading = ref(false)
const orders = ref([])
const total = ref(0)
const currentPage = ref(1)
const pageSize = ref(10)

const dialogVisible = ref(false)
const dialogType = ref<'create' | 'edit'>('create')
const formRef = ref<FormInstance>()

const form = ref({
  customer_name: '',
  phone: '',
  prescription: '',
  frame_type: '',
  lens_type: '',
  total_amount: 0,
  status: '待处理'
})

const rules: FormRules = {
  customer_name: [{ required: true, message: '请输入客户姓名', trigger: 'blur' }],
  phone: [{ required: true, message: '请输入联系电话', trigger: 'blur' }],
  prescription: [{ required: true, message: '请输入处方信息', trigger: 'blur' }],
  frame_type: [{ required: true, message: '请输入镜框类型', trigger: 'blur' }],
  lens_type: [{ required: true, message: '请输入镜片类型', trigger: 'blur' }],
  total_amount: [{ required: true, message: '请输入总金额', trigger: 'blur' }],
  status: [{ required: true, message: '请选择状态', trigger: 'change' }]
}

const fetchOrders = async () => {
  loading.value = true
  try {
    const response = await fetch(`/api/orders?page=${currentPage.value}&page_size=${pageSize.value}`)
    const data = await response.json()
    orders.value = data.items
    total.value = data.total
  } catch (error) {
    ElMessage.error('获取订单列表失败')
  } finally {
    loading.value = false
  }
}

const handleCreate = () => {
  dialogType.value = 'create'
  form.value = {
    customer_name: '',
    phone: '',
    prescription: '',
    frame_type: '',
    lens_type: '',
    total_amount: 0,
    status: '待处理'
  }
  dialogVisible.value = true
}

const handleEdit = (row: any) => {
  dialogType.value = 'edit'
  form.value = { ...row }
  dialogVisible.value = true
}

const handleDelete = async (row: any) => {
  try {
    await ElMessageBox.confirm('确认删除该订单吗？', '提示', {
      type: 'warning'
    })
    await fetch(`/api/orders/${row.id}`, { method: 'DELETE' })
    ElMessage.success('删除成功')
    await fetchOrders()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败')
    }
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return
  await formRef.value.validate(async (valid) => {
    if (valid) {
      try {
        const url = dialogType.value === 'create' ? '/api/orders' : `/api/orders/${form.value.id}`
        const method = dialogType.value === 'create' ? 'POST' : 'PUT'
        const response = await fetch(url, {
          method,
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(form.value)
        })
        if (response.ok) {
          ElMessage.success(dialogType.value === 'create' ? '创建成功' : '更新成功')
          dialogVisible.value = false
          await fetchOrders()
        } else {
          throw new Error()
        }
      } catch (error) {
        ElMessage.error(dialogType.value === 'create' ? '创建失败' : '更新失败')
      }
    }
  })
}

const handleSizeChange = (val: number) => {
  pageSize.value = val
  fetchOrders()
}

const handleCurrentChange = (val: number) => {
  currentPage.value = val
  fetchOrders()
}

onMounted(() => {
  fetchOrders()
})
</script>

<style scoped>
.orders-container {
  padding: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.pagination {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}
</style>