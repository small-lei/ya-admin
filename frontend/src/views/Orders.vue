<template>
  <div class="orders-container">
    <div class="toolbar">
      <el-button type="primary" @click="handleCreate">
        创建订单
      </el-button>
    </div>

    <el-table :data="orders" v-loading="loading" border>
      <el-table-column prop="id" label="订单编号" width="120" />
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
          <el-tag :type="row.status === 'completed' ? 'success' : 'warning'">
            {{ row.status === 'completed' ? '已完成' : '处理中' }}
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

    <el-pagination
      v-model:current-page="currentPage"
      v-model:page-size="pageSize"
      :total="total"
      :page-sizes="[10, 20, 50, 100]"
      layout="total, sizes, prev, pager, next"
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      class="pagination"
    />

    <el-dialog
      v-model="dialogVisible"
      :title="dialogType === 'create' ? '创建订单' : '编辑订单'"
      width="50%"
    >
      <el-form
        ref="orderForm"
        :model="orderData"
        :rules="rules"
        label-width="100px"
      >
        <el-form-item label="客户姓名" prop="customer_name">
          <el-input v-model="orderData.customer_name" />
        </el-form-item>
        <el-form-item label="联系电话" prop="phone">
          <el-input v-model="orderData.phone" />
        </el-form-item>
        <el-form-item label="处方信息" prop="prescription">
          <el-input
            v-model="orderData.prescription"
            type="textarea"
            :rows="3"
          />
        </el-form-item>
        <el-form-item label="镜框类型" prop="frame_type">
          <el-select v-model="orderData.frame_type" placeholder="请选择镜框类型">
            <el-option label="全框" value="full" />
            <el-option label="半框" value="half" />
            <el-option label="无框" value="rimless" />
          </el-select>
        </el-form-item>
        <el-form-item label="镜片类型" prop="lens_type">
          <el-select v-model="orderData.lens_type" placeholder="请选择镜片类型">
            <el-option label="单光" value="single" />
            <el-option label="双光" value="bifocal" />
            <el-option label="渐进" value="progressive" />
          </el-select>
        </el-form-item>
        <el-form-item label="总金额" prop="total_amount">
          <el-input-number
            v-model="orderData.total_amount"
            :precision="2"
            :step="100"
            :min="0"
          />
        </el-form-item>
        <el-form-item label="状态" prop="status">
          <el-select v-model="orderData.status" placeholder="请选择状态">
            <el-option label="处理中" value="processing" />
            <el-option label="已完成" value="completed" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">取消</el-button>
          <el-button type="primary" @click="handleSubmit" :loading="submitting">
            确定
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { FormInstance } from 'element-plus'

const loading = ref(false)
const submitting = ref(false)
const dialogVisible = ref(false)
const dialogType = ref<'create' | 'edit'>('create')
const orderForm = ref<FormInstance>()

const currentPage = ref(1)
const pageSize = ref(10)
const total = ref(0)
const orders = ref([])

const orderData = reactive({
  id: '',
  customer_name: '',
  phone: '',
  prescription: '',
  frame_type: '',
  lens_type: '',
  total_amount: 0,
  status: 'processing'
})

const rules = {
  customer_name: [{ required: true, message: '请输入客户姓名', trigger: 'blur' }],
  phone: [{ required: true, message: '请输入联系电话', trigger: 'blur' }],
  prescription: [{ required: true, message: '请输入处方信息', trigger: 'blur' }],
  frame_type: [{ required: true, message: '请选择镜框类型', trigger: 'change' }],
  lens_type: [{ required: true, message: '请选择镜片类型', trigger: 'change' }],
  total_amount: [{ required: true, message: '请输入总金额', trigger: 'blur' }],
  status: [{ required: true, message: '请选择状态', trigger: 'change' }]
}

const fetchOrders = async () => {
  loading.value = true
  try {
    const response = await fetch(`http://localhost:3000/api/orders?page=${currentPage.value}&page_size=${pageSize.value}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('token')}`
      }
    })
    const data = await response.json()
    if (response.ok) {
      orders.value = data.items
      total.value = data.total
    } else {
      ElMessage.error(data.message || '获取订单列表失败')
    }
  } catch (error) {
    ElMessage.error('网络错误，请稍后重试')
  } finally {
    loading.value = false
  }
}

const resetOrderData = () => {
  Object.assign(orderData, {
    id: '',
    customer_name: '',
    phone: '',
    prescription: '',
    frame_type: '',
    lens_type: '',
    total_amount: 0,
    status: 'processing'
  })
}

const handleCreate = () => {
  dialogType.value = 'create'
  resetOrderData()
  dialogVisible.value = true
}

const handleEdit = (row: any) => {
  dialogType.value = 'edit'
  Object.assign(orderData, row)
  dialogVisible.value = true
}

const handleDelete = async (row: any) => {
  try {
    await ElMessageBox.confirm('确定要删除这个订单吗？', '提示', {
      type: 'warning'
    })
    
    const response = await fetch(`http://localhost:3000/api/orders/${row.id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('token')}`
      }
    })

    if (response.ok) {
      ElMessage.success('删除成功')
      fetchOrders()
    } else {
      const data = await response.json()
      ElMessage.error(data.message || '删除失败')
    }
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('操作失败，请重试')
    }
  }
}

const handleSubmit = async () => {
  if (!orderForm.value) return

  await orderForm.value.validate(async (valid) => {
    if (valid) {
      submitting.value = true
      try {
        const url = dialogType.value === 'create'
          ? 'http://localhost:3000/api/orders'
          : `http://localhost:3000/api/orders/${orderData.id}`

        const response = await fetch(url, {
          method: dialogType.value === 'create' ? 'POST' : 'PUT',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${localStorage.getItem('token')}`
          },
          body: JSON.stringify(orderData)
        })

        const data = await response.json()
        if (response.ok) {
          ElMessage.success(dialogType.value === 'create' ? '创建成功' : '更新成功')
          dialogVisible.value = false
          fetchOrders()
        } else {
          ElMessage.error(data.message || '操作失败')
        }
      } catch (error) {
        ElMessage.error('网络错误，请稍后重试')
      } finally {
        submitting.value = false
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

.toolbar {
  margin-bottom: 20px;
}

.pagination {
  margin-top: 20px;
  justify-content: flex-end;
}
</style>