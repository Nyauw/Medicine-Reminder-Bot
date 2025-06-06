# 药品提醒机器人 (Medicine Reminder Bot)

> 中文文档 | [English](README.md)

一个基于Rust和Telegram Bot的智能药品提醒系统，帮助你按时服药。

## 功能特性

- 🏥 **药品管理**: 添加、删除、查看药品信息
- ⏰ **智能提醒**: 支持多个时间点的定时提醒
- 🔔 **持续提醒**: 如果未确认服药，会持续提醒直到确认
- 📊 **服药记录**: 自动记录服药情况和剩余数量
- 💊 **库存管理**: 支持药品数量管理和补充提醒
- 🎯 **灵活数量**: 支持自定义服药和补充数量
- ⚡ **快捷操作**: 提供常用数量的快捷按钮选择

## 安装和配置

### 1. 创建Telegram Bot

1. 在Telegram中找到 [@BotFather](https://t.me/BotFather)
2. 发送 `/newbot` 创建新机器人
3. 按提示设置机器人名称和用户名
4. 获取Bot Token

### 2. 获取Chat ID

1. 在Telegram中找到 [@userinfobot](https://t.me/userinfobot)
2. 发送任意消息获取你的Chat ID
3. 或者将机器人添加到群组中使用群组ID

### 3. 配置环境变量

复制 `.env.example` 为 `.env` 并填入你的配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：
```
TELOXIDE_TOKEN=你的机器人Token
CHAT_ID=你的ChatID
```

### 4. 编译和运行

```bash
# 编译项目
cargo build --release

# 运行机器人
cargo run
```

## 使用方法

### 基本命令

- `/help` - 显示帮助信息
- `/add` - 添加新药品
- `/list` - 查看所有药品
- `/delete` - 删除药品
- `/refill` - 补充药品数量
- `/pending` - 查看待确认的提醒

### 添加药品示例

1. 发送 `/add`
2. 输入药品名称，如：`维生素C`
3. 输入数量，如：`30`
4. 输入提醒时间，如：`08:00,20:00`

### 提醒确认

当收到提醒消息时：
- 点击 "✅ 已服药" 确认服药（会显示数量选择界面）
- 点击 "⏰ 稍后提醒" 延迟5分钟后再次提醒

#### 服药数量选择
确认服药时，系统会提供以下选项：
- **快捷选择**：1片、2片、3片
- **自定义数量**：输入任意数量

#### 补充药品数量
使用 `/refill` 命令时，系统会提供以下选项：
- **快捷选择**：10个、20个、30个
- **自定义数量**：输入任意数量

## 提醒机制

- **首次提醒**: 在设定时间准时提醒
- **第二次提醒**: 如果5分钟内未确认，再次提醒
- **第三次提醒**: 如果10分钟内未确认，再次提醒
- **后续提醒**: 每15分钟提醒一次，直到确认

## 数据存储

所有数据保存在 `medicine_data.json` 文件中，包括：
- 药品信息（名称、数量、提醒时间）
- 待确认的提醒记录

## 项目结构

```
src/
├── main.rs          # 主程序入口
├── lib.rs           # 模块声明
├── bot.rs           # Telegram Bot处理逻辑
├── medicine.rs      # 药品数据结构
├── reminder.rs      # 提醒系统逻辑
└── storage.rs       # 数据持久化
```

## 依赖项

- `teloxide` - Telegram Bot框架
- `tokio` - 异步运行时
- `serde` - 序列化/反序列化
- `chrono` - 时间处理
- `uuid` - 唯一ID生成

## 注意事项

1. 确保机器人有发送消息的权限
2. 保持程序运行以接收和发送提醒
3. 定期备份 `medicine_data.json` 文件
4. 时间格式必须为 HH:MM（24小时制）

## 故障排除

### 常见问题

1. **机器人无响应**
   - 检查Token是否正确
   - 确认网络连接正常

2. **收不到提醒**
   - 检查Chat ID是否正确
   - 确认程序正在运行

3. **时间格式错误**
   - 使用24小时制格式：08:00, 20:30
   - 多个时间用逗号分隔

## 许可证

MIT License
