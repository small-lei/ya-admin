// API基础URL
const API_BASE_URL = '/api';

// 请求选项的默认值
const defaultOptions = {
  headers: {
    'Content-Type': 'application/json'
  },
  credentials: 'include' as RequestCredentials
};

// 添加认证token到请求头
const getAuthHeaders = () => {
  const token = localStorage.getItem('token');
  return token ? { 'Authorization': `Bearer ${token}` } : {};
};

// 封装的API请求函数
export async function apiRequest(
  endpoint: string, 
  method: string = 'GET', 
  data?: any, 
  requiresAuth: boolean = false
) {
  try {
    // 构建请求选项
    const options: RequestInit = {
      ...defaultOptions,
      method,
      headers: {
        ...defaultOptions.headers,
        ...(requiresAuth ? getAuthHeaders() : {})
      }
    };

    // 添加请求体
    if (data && (method === 'POST' || method === 'PUT' || method === 'PATCH')) {
      options.body = JSON.stringify(data);
    }

    // 发送请求
    console.log(`发送 ${method} 请求到 ${endpoint}`, data);
    const response = await fetch(`${API_BASE_URL}${endpoint}`, options);
    
    // 解析响应
    let responseData;
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      responseData = await response.json();
    } else {
      responseData = await response.text();
    }
    
    console.log(`${endpoint} 响应:`, response.status, responseData);

    // 处理非成功响应
    if (!response.ok) {
      throw {
        status: response.status,
        data: responseData,
        message: responseData.message || '请求失败'
      };
    }

    return responseData;
  } catch (error) {
    console.error(`API请求错误: ${endpoint}`, error);
    throw error;
  }
}

// 认证相关API
export const authApi = {
  login: (username: string, password: string) => 
    apiRequest('/auth/login', 'POST', { username, password })
};

// 订单相关API
export const ordersApi = {
  getOrders: () => apiRequest('/orders', 'GET', undefined, true),
  createOrder: (orderData: any) => apiRequest('/orders', 'POST', orderData, true),
  updateOrder: (id: number, orderData: any) => apiRequest(`/orders/${id}`, 'PUT', orderData, true),
  deleteOrder: (id: number) => apiRequest(`/orders/${id}`, 'DELETE', undefined, true)
}; 