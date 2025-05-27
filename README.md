# Medicine Reminder Bot

> [中文文档](README_zh.md) | English

An intelligent medicine reminder system built with Rust and Telegram Bot to help you take medications on time.

## Features

- 🏥 **Medicine Management**: Add, delete, and view medicine information
- ⏰ **Smart Reminders**: Support multiple reminder times per day
- 🔔 **Persistent Reminders**: Continuous reminders until confirmation if not acknowledged
- 📊 **Medication Records**: Automatic tracking of medication intake and remaining quantities
- 💊 **Inventory Management**: Medicine quantity management with refill reminders
- 🎯 **Flexible Quantities**: Support for custom medication and refill amounts
- ⚡ **Quick Actions**: Convenient quick-select buttons for common quantities

## Installation and Configuration

### 1. Create a Telegram Bot

1. Find [@BotFather](https://t.me/BotFather) on Telegram
2. Send `/newbot` to create a new bot
3. Follow the prompts to set bot name and username
4. Get your Bot Token

### 2. Get Chat ID

1. Find [@userinfobot](https://t.me/userinfobot) on Telegram
2. Send any message to get your Chat ID
3. Or add the bot to a group and use the group ID

### 3. Configure Environment Variables

Copy `.env.example` to `.env` and fill in your configuration:

```bash
cp .env.example .env
```

Edit the `.env` file:
```
TELOXIDE_TOKEN=your_bot_token
CHAT_ID=your_chat_id
```

### 4. Build and Run

```bash
# Build the project
cargo build --release

# Run the bot
cargo run
```

## Usage

### Basic Commands

- `/help` - Show help information
- `/add` - Add new medicine
- `/list` - View all medicines
- `/delete` - Delete medicine
- `/refill` - Refill medicine quantity
- `/pending` - View pending reminders

### Adding Medicine Example

1. Send `/add`
2. Enter medicine name, e.g., `Vitamin C`
3. Enter quantity, e.g., `30`
4. Enter reminder times, e.g., `08:00,20:00`

### Reminder Confirmation

When you receive a reminder message:
- Click "✅ Taken" to confirm medication (will show quantity selection interface)
- Click "⏰ Remind Later" to delay reminder by 5 minutes

#### Medication Quantity Selection
When confirming medication intake, the system provides:
- **Quick Select**: 1 pill, 2 pills, 3 pills
- **Custom Amount**: Enter any quantity

#### Refilling Medicine Quantity
When using the `/refill` command, the system provides:
- **Quick Select**: 10, 20, 30 units
- **Custom Amount**: Enter any quantity

## Reminder Mechanism

- **First Reminder**: Prompt reminder at scheduled time
- **Second Reminder**: Remind again if not confirmed within 5 minutes
- **Third Reminder**: Remind again if not confirmed within 10 minutes
- **Subsequent Reminders**: Remind every 15 minutes until confirmed

## Data Storage

All data is saved in the `medicine_data.json` file, including:
- Medicine information (name, quantity, reminder times)
- Pending reminder records

## Project Structure

```
src/
├── main.rs          # Main program entry
├── lib.rs           # Module declarations
├── bot.rs           # Telegram Bot handling logic
├── medicine.rs      # Medicine data structures
├── reminder.rs      # Reminder system logic
└── storage.rs       # Data persistence
```

## Dependencies

- `teloxide` - Telegram Bot framework
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `chrono` - Time handling
- `uuid` - Unique ID generation

## Important Notes

1. Ensure the bot has permission to send messages
2. Keep the program running to receive and send reminders
3. Regularly backup the `medicine_data.json` file
4. Time format must be HH:MM (24-hour format)

## Troubleshooting

### Common Issues

1. **Bot Not Responding**
   - Check if the Token is correct
   - Confirm network connection is working

2. **Not Receiving Reminders**
   - Check if Chat ID is correct
   - Confirm the program is running

3. **Time Format Error**
   - Use 24-hour format: 08:00, 20:30
   - Separate multiple times with commas

## License

MIT License
