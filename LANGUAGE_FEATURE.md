# 语言切换功能 / Language Switching Feature

## 概述 / Overview

药品提醒机器人现在支持中文和英文两种语言，用户可以通过简单的命令在启动后随时切换语言。

The Medicine Reminder Bot now supports both Chinese and English languages. Users can switch between languages at any time after startup using simple commands.

## 功能特性 / Features

### 支持的语言 / Supported Languages
- 🇨🇳 **中文** (默认) / Chinese (Default)
- 🇺🇸 **English**

### 语言切换 / Language Switching
- 使用 `/language` 命令打开语言选择界面
- Use `/language` command to open language selection interface
- 点击按钮即可切换语言
- Click buttons to switch languages
- 设置会自动保存
- Settings are automatically saved

## 使用方法 / Usage

### 切换语言 / Switch Language

1. 发送命令 / Send command: `/language`
2. 选择语言 / Select language:
   - 🇨🇳 中文
   - 🇺🇸 English
3. 确认切换 / Confirm switch

### 语言影响的内容 / Content Affected by Language

切换语言后，以下内容会相应改变：
After switching languages, the following content will change accordingly:

- ✅ 所有命令帮助文本 / All command help text
- ✅ 系统消息和提示 / System messages and prompts
- ✅ 药品提醒消息 / Medicine reminder messages
- ✅ 按钮文本 / Button text
- ✅ 错误消息 / Error messages
- ✅ 确认消息 / Confirmation messages

## 技术实现 / Technical Implementation

### 数据存储 / Data Storage
- 语言偏好保存在 `medicine_data.json` 文件中
- Language preference is saved in `medicine_data.json` file
- 向后兼容，现有数据会自动使用中文作为默认语言
- Backward compatible, existing data will automatically use Chinese as default

### 本地化系统 / Localization System
- 使用静态文本结构提供高性能
- Uses static text structures for high performance
- 支持动态消息格式化
- Supports dynamic message formatting
- 易于扩展新语言
- Easy to extend with new languages

## 命令对比 / Command Comparison

### 中文界面 / Chinese Interface
```
🏥 药品提醒助手

📋 可用命令：
/add - 添加新药品
/list - 查看所有药品
/delete - 删除药品
/refill - 补充药品数量
/pending - 查看待确认的提醒
/language - 切换语言
/help - 显示此帮助信息
```

### English Interface
```
🏥 Medicine Reminder Assistant

📋 Available Commands:
/add - Add new medicine
/list - View all medicines
/delete - Delete medicine
/refill - Refill medicine quantity
/pending - View pending reminders
/language - Switch language
/help - Show this help message
```

## 提醒消息示例 / Reminder Message Examples

### 中文提醒 / Chinese Reminder
```
🔔 吃药提醒！

💊 药品：维生素C
⏰ 时间：08:00

请点击下面的按钮确认已服药：
[✅ 已服药] [⏰ 稍后提醒]
```

### English Reminder
```
🔔 Medicine Reminder!

💊 Medicine：Vitamin C
⏰ Time：08:00

Please click the button below to confirm you have taken the medicine：
[✅ Taken] [⏰ Snooze]
```

## 注意事项 / Notes

1. **默认语言** / Default Language
   - 新用户默认使用中文
   - New users default to Chinese
   - 可随时切换到英文
   - Can switch to English at any time

2. **数据兼容性** / Data Compatibility
   - 现有用户数据完全兼容
   - Existing user data is fully compatible
   - 升级后自动使用中文界面
   - Automatically uses Chinese interface after upgrade

3. **持久化设置** / Persistent Settings
   - 语言设置会永久保存
   - Language settings are permanently saved
   - 重启机器人后保持选择的语言
   - Maintains selected language after bot restart