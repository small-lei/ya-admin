-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建用户表更新触发器
CREATE TRIGGER IF NOT EXISTS users_updated_at 
AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 创建订单表
CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    prescription TEXT NOT NULL,
    frame_type TEXT NOT NULL,
    lens_type TEXT NOT NULL,
    total_amount REAL NOT NULL,
    status TEXT NOT NULL,
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- 创建订单表更新触发器
CREATE TRIGGER IF NOT EXISTS orders_updated_at 
AFTER UPDATE ON orders
BEGIN
    UPDATE orders SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 添加测试用户数据 (密码为 'password' 的哈希值)
INSERT INTO users (username, password_hash) VALUES 
('admin', '$2b$12$A7x.X5ELfUi3JMHItk2M.e1nHRlrhKOLcPJh2ZfAGXy0t3Ypqcqb2');

-- 添加测试订单数据
INSERT INTO orders (customer_name, phone, prescription, frame_type, lens_type, total_amount, status, created_by) VALUES
('张三', '13800138000', '左眼: -2.50, 右眼: -2.75', '全框', '防蓝光镜片', 680.00, '已付款', 1),
('李四', '13900139000', '左眼: -3.00, 右眼: -3.25', '半框', '变色镜片', 880.00, '制作中', 1),
('王五', '13700137000', '左眼: -1.50, 右眼: -1.75', '无框', '标准镜片', 580.00, '已完成', 1); 