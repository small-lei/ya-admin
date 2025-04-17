-- 创建数据库
CREATE DATABASE IF NOT EXISTS ya_admin;
USE ya_admin;

-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 创建订单表
CREATE TABLE IF NOT EXISTS orders (
    id INT AUTO_INCREMENT PRIMARY KEY,
    customer_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    prescription TEXT NOT NULL,
    frame_type VARCHAR(100) NOT NULL,
    lens_type VARCHAR(100) NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    status VARCHAR(20) NOT NULL,
    created_by INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- 添加测试用户数据 (密码为 'password' 的哈希值)
INSERT INTO users (username, password_hash) VALUES 
('admin', '$2b$12$A7x.X5ELfUi3JMHItk2M.e1nHRlrhKOLcPJh2ZfAGXy0t3Ypqcqb2');

-- 添加测试订单数据
INSERT INTO orders (customer_name, phone, prescription, frame_type, lens_type, total_amount, status, created_by) VALUES
('张三', '13800138000', '左眼: -2.50, 右眼: -2.75', '全框', '防蓝光镜片', 680.00, '已付款', 1),
('李四', '13900139000', '左眼: -3.00, 右眼: -3.25', '半框', '变色镜片', 880.00, '制作中', 1),
('王五', '13700137000', '左眼: -1.50, 右眼: -1.75', '无框', '标准镜片', 580.00, '已完成', 1); 