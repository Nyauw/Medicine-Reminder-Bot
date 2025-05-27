use crate::{Medicine, ReminderService};
use chrono::NaiveTime;
use std::sync::Arc;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use uuid::Uuid;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveMedicineName,
    ReceiveQuantity { name: String },
    ReceiveReminderTimes { name: String, quantity: u32 },
    ReceiveConfirmDoseAmount { reminder_id: String },
    ReceiveRefillAmount { medicine_id: String },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持的命令：")]
pub enum Command {
    #[command(description = "显示帮助信息")]
    Help,
    #[command(description = "开始添加新药品")]
    Add,
    #[command(description = "查看所有药品")]
    List,
    #[command(description = "删除药品")]
    Delete,
    #[command(description = "添加药品数量")]
    Refill,
    #[command(description = "查看待确认的提醒")]
    Pending,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Add].endpoint(start_add_medicine))
                .branch(case![Command::List].endpoint(list_medicines))
                .branch(case![Command::Delete].endpoint(delete_medicine))
                .branch(case![Command::Refill].endpoint(refill_medicine))
                .branch(case![Command::Pending].endpoint(show_pending)),
        )
        .branch(case![State::ReceiveMedicineName].endpoint(receive_medicine_name))
        .branch(case![State::ReceiveQuantity { name }].endpoint(receive_quantity))
        .branch(case![State::ReceiveReminderTimes { name, quantity }].endpoint(receive_reminder_times))
        .branch(case![State::ReceiveConfirmDoseAmount { reminder_id }].endpoint(receive_confirm_dose_amount))
        .branch(case![State::ReceiveRefillAmount { medicine_id }].endpoint(receive_refill_amount));

    let message_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(command_handler)
        .branch(case![State::ReceiveMedicineName].endpoint(receive_medicine_name))
        .branch(case![State::ReceiveQuantity { name }].endpoint(receive_quantity))
        .branch(case![State::ReceiveReminderTimes { name, quantity }].endpoint(receive_reminder_times))
        .branch(case![State::ReceiveConfirmDoseAmount { reminder_id }].endpoint(receive_confirm_dose_amount))
        .branch(case![State::ReceiveRefillAmount { medicine_id }].endpoint(receive_refill_amount));

    let callback_query_handler = Update::filter_callback_query()
        .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
        .endpoint(handle_callback);

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let help_text = "🏥 药品提醒助手\n\n\
        📋 可用命令：\n\
        /add - 添加新药品\n\
        /list - 查看所有药品\n\
        /delete - 删除药品\n\
        /refill - 补充药品数量\n\
        /pending - 查看待确认的提醒\n\
        /help - 显示此帮助信息\n\n\
        💡 使用说明：\n\
        1. 使用 /add 添加药品，设置名称、数量和提醒时间\n\
        2. 系统会在设定时间自动提醒\n\
        3. 收到提醒后请点击确认按钮\n\
        4. 如果不确认，系统会持续提醒";

    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}

async fn start_add_medicine(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "请输入药品名称：").await?;
    dialogue.update(State::ReceiveMedicineName).await?;
    Ok(())
}

async fn receive_medicine_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(name) => {
            bot.send_message(msg.chat.id, format!("药品名称：{}\n请输入药品数量：", name))
                .await?;
            dialogue
                .update(State::ReceiveQuantity {
                    name: name.to_string(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "请输入有效的药品名称").await?;
        }
    }
    Ok(())
}

async fn receive_quantity(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    name: String,
) -> HandlerResult {
    match msg.text().and_then(|text| text.parse::<u32>().ok()) {
        Some(quantity) => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "药品：{}\n数量：{}\n\n请输入提醒时间（格式：HH:MM，多个时间用逗号分隔）\n例如：08:00,20:00",
                    name, quantity
                ),
            )
            .await?;
            dialogue
                .update(State::ReceiveReminderTimes { name, quantity })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "请输入有效的数量（正整数）").await?;
        }
    }
    Ok(())
}

async fn receive_reminder_times(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (name, quantity): (String, u32),
    reminder_service: Arc<ReminderService>,
) -> HandlerResult {
    match msg.text() {
        Some(times_str) => {
            let times: Result<Vec<NaiveTime>, _> = times_str
                .split(',')
                .map(|s| s.trim())
                .map(|s| NaiveTime::parse_from_str(s, "%H:%M"))
                .collect();

            match times {
                Ok(reminder_times) if !reminder_times.is_empty() => {
                    let medicine = Medicine::new(name.clone(), quantity, reminder_times.clone());

                    if let Err(e) = reminder_service
                        .update_data(|data| {
                            data.medicines.insert(medicine.id, medicine);
                        })
                        .await {
                        log::error!("Failed to save medicine: {}", e);
                        bot.send_message(msg.chat.id, "❌ 保存药品信息失败").await?;
                        return Ok(());
                    }

                    let times_display: Vec<String> = reminder_times
                        .iter()
                        .map(|t| t.format("%H:%M").to_string())
                        .collect();

                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "✅ 药品添加成功！\n\n💊 名称：{}\n📦 数量：{}\n⏰ 提醒时间：{}",
                            name,
                            quantity,
                            times_display.join(", ")
                        ),
                    )
                    .await?;

                    dialogue.update(State::Start).await?;
                }
                _ => {
                    bot.send_message(
                        msg.chat.id,
                        "时间格式错误，请使用 HH:MM 格式，多个时间用逗号分隔\n例如：08:00,20:00",
                    )
                    .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "请输入提醒时间").await?;
        }
    }
    Ok(())
}

async fn list_medicines(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;

    if data.medicines.is_empty() {
        bot.send_message(msg.chat.id, "📭 暂无药品记录").await?;
        return Ok(());
    }

    let mut message = "💊 药品列表：\n\n".to_string();
    for (i, medicine) in data.medicines.values().enumerate() {
        let status = if medicine.is_active { "🟢" } else { "🔴" };
        let times: Vec<String> = medicine
            .reminder_times
            .iter()
            .map(|t| t.format("%H:%M").to_string())
            .collect();

        message.push_str(&format!(
            "{}. {} {}\n📦 数量：{}\n⏰ 提醒时间：{}\n\n",
            i + 1,
            status,
            medicine.name,
            medicine.quantity,
            times.join(", ")
        ));
    }

    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

async fn delete_medicine(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;

    if data.medicines.is_empty() {
        bot.send_message(msg.chat.id, "📭 暂无药品可删除").await?;
        return Ok(());
    }

    let mut keyboard = Vec::new();
    for medicine in data.medicines.values() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            format!("🗑️ {}", medicine.name),
            format!("delete_{}", medicine.id),
        )]);
    }

    let markup = InlineKeyboardMarkup::new(keyboard);
    bot.send_message(msg.chat.id, "请选择要删除的药品：")
        .reply_markup(markup)
        .await?;
    Ok(())
}

async fn refill_medicine(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;

    if data.medicines.is_empty() {
        bot.send_message(msg.chat.id, "📭 暂无药品可补充").await?;
        return Ok(());
    }

    let mut keyboard = Vec::new();
    for medicine in data.medicines.values() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            format!("💊 {} (剩余: {})", medicine.name, medicine.quantity),
            format!("refill_{}", medicine.id),
        )]);
    }

    let markup = InlineKeyboardMarkup::new(keyboard);
    bot.send_message(msg.chat.id, "请选择要补充的药品：")
        .reply_markup(markup)
        .await?;
    Ok(())
}

async fn show_pending(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;

    let pending: Vec<_> = data
        .pending_reminders
        .values()
        .filter(|r| !r.is_confirmed)
        .collect();

    if pending.is_empty() {
        bot.send_message(msg.chat.id, "✅ 暂无待确认的提醒").await?;
        return Ok(());
    }

    let mut message = "⏰ 待确认的提醒：\n\n".to_string();
    for (i, reminder) in pending.iter().enumerate() {
        message.push_str(&format!(
            "{}. 💊 {}\n⏰ 时间：{}\n📊 提醒次数：{}\n\n",
            i + 1,
            reminder.medicine_name,
            reminder.scheduled_time.format("%H:%M"),
            reminder.reminder_count
        ));
    }

    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

async fn handle_callback(bot: Bot, q: CallbackQuery, reminder_service: Arc<ReminderService>, dialogue: MyDialogue) -> HandlerResult {
    if let Some(data) = &q.data {

        if let Some(chat_id) = q.message.as_ref().map(|m| m.chat.id) {
            if data.starts_with("confirm_") {
                let reminder_id = data.strip_prefix("confirm_").unwrap();
                if let Ok(_uuid) = Uuid::parse_str(reminder_id) {
                    // 显示数量选择界面
                    let keyboard = vec![
                        vec![
                            InlineKeyboardButton::callback("1片", format!("dose_1_{}", reminder_id)),
                            InlineKeyboardButton::callback("2片", format!("dose_2_{}", reminder_id)),
                            InlineKeyboardButton::callback("3片", format!("dose_3_{}", reminder_id)),
                        ],
                        vec![
                            InlineKeyboardButton::callback("自定义数量", format!("dose_custom_{}", reminder_id)),
                        ],
                    ];
                    let markup = InlineKeyboardMarkup::new(keyboard);
                    bot.send_message(chat_id, "请选择服用数量：")
                        .reply_markup(markup)
                        .await?;
                }
            } else if data.starts_with("snooze_") {
                let reminder_id = data.strip_prefix("snooze_").unwrap();
                if let Ok(uuid) = Uuid::parse_str(reminder_id) {
                    match reminder_service.snooze_reminder(uuid).await {
                        Ok(response) => {
                            bot.send_message(chat_id, response).await?;
                        }
                        Err(error) => {
                            bot.send_message(chat_id, format!("❌ 错误：{}", error)).await?;
                        }
                    }
                }
            } else if data.starts_with("delete_") {
                let medicine_id = data.strip_prefix("delete_").unwrap();
                if let Ok(uuid) = Uuid::parse_str(medicine_id) {
                    if let Err(e) = reminder_service
                        .update_data(|app_data| {
                            app_data.medicines.remove(&uuid);
                        })
                        .await {
                        log::error!("Failed to delete medicine: {}", e);
                        bot.send_message(chat_id, "❌ 删除药品失败").await?;
                    } else {
                        bot.send_message(chat_id, "✅ 药品已删除").await?;
                    }
                }
            } else if data.starts_with("refill_") {
                let medicine_id = data.strip_prefix("refill_").unwrap();
                if let Ok(_uuid) = Uuid::parse_str(medicine_id) {
                    // 显示数量选择界面
                    let keyboard = vec![
                        vec![
                            InlineKeyboardButton::callback("10个", format!("refill_10_{}", medicine_id)),
                            InlineKeyboardButton::callback("20个", format!("refill_20_{}", medicine_id)),
                            InlineKeyboardButton::callback("30个", format!("refill_30_{}", medicine_id)),
                        ],
                        vec![
                            InlineKeyboardButton::callback("自定义数量", format!("refill_custom_{}", medicine_id)),
                        ],
                    ];
                    let markup = InlineKeyboardMarkup::new(keyboard);
                    bot.send_message(chat_id, "请选择补充数量：")
                        .reply_markup(markup)
                        .await?;
                }
            } else if data.starts_with("dose_") {
                // 处理服药数量选择
                if data.starts_with("dose_custom_") {
                    let reminder_id = data.strip_prefix("dose_custom_").unwrap();
                    bot.send_message(chat_id, "请输入服用数量：").await?;
                    dialogue.update(State::ReceiveConfirmDoseAmount {
                        reminder_id: reminder_id.to_string()
                    }).await?;
                } else {
                    // 处理预设数量
                    let parts: Vec<&str> = data.splitn(3, '_').collect();
                    if parts.len() == 3 {
                        if let (Ok(amount), Ok(uuid)) = (parts[1].parse::<u32>(), Uuid::parse_str(parts[2])) {
                            match reminder_service.confirm_medicine_with_amount(uuid, amount).await {
                                Ok(response) => {
                                    bot.send_message(chat_id, response).await?;
                                }
                                Err(error) => {
                                    bot.send_message(chat_id, format!("❌ 错误：{}", error)).await?;
                                }
                            }
                        }
                    }
                }
            } else if data.starts_with("refill_") && data.contains("_") {
                // 处理补充数量选择
                if data.starts_with("refill_custom_") {
                    let medicine_id = data.strip_prefix("refill_custom_").unwrap();
                    bot.send_message(chat_id, "请输入补充数量：").await?;
                    dialogue.update(State::ReceiveRefillAmount {
                        medicine_id: medicine_id.to_string()
                    }).await?;
                } else {
                    // 处理预设数量
                    let parts: Vec<&str> = data.splitn(3, '_').collect();
                    if parts.len() == 3 && parts[0] == "refill" {
                        if let (Ok(amount), Ok(uuid)) = (parts[1].parse::<u32>(), Uuid::parse_str(parts[2])) {
                            if let Err(e) = reminder_service
                                .update_data(|app_data| {
                                    if let Some(medicine) = app_data.medicines.get_mut(&uuid) {
                                        medicine.add_quantity(amount);
                                    }
                                })
                                .await {
                                log::error!("Failed to refill medicine: {}", e);
                                bot.send_message(chat_id, "❌ 补充药品失败").await?;
                            } else {
                                bot.send_message(chat_id, format!("✅ 已补充{}个药品", amount)).await?;
                            }
                        }
                    }
                }
            }
        }
    }

    bot.answer_callback_query(q.id).await?;
    Ok(())
}

async fn receive_confirm_dose_amount(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    reminder_id: String,
    reminder_service: Arc<ReminderService>,
) -> HandlerResult {
    match msg.text().and_then(|text| text.parse::<u32>().ok()) {
        Some(amount) if amount > 0 => {
            if let Ok(uuid) = Uuid::parse_str(&reminder_id) {
                match reminder_service.confirm_medicine_with_amount(uuid, amount).await {
                    Ok(response) => {
                        bot.send_message(msg.chat.id, response).await?;
                        dialogue.update(State::Start).await?;
                    }
                    Err(error) => {
                        bot.send_message(msg.chat.id, format!("❌ 错误：{}", error)).await?;
                        dialogue.update(State::Start).await?;
                    }
                }
            } else {
                bot.send_message(msg.chat.id, "❌ 无效的提醒ID").await?;
                dialogue.update(State::Start).await?;
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "请输入有效的数量（正整数）：").await?;
        }
    }
    Ok(())
}

async fn receive_refill_amount(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    medicine_id: String,
    reminder_service: Arc<ReminderService>,
) -> HandlerResult {
    match msg.text().and_then(|text| text.parse::<u32>().ok()) {
        Some(amount) if amount > 0 => {
            if let Ok(uuid) = Uuid::parse_str(&medicine_id) {
                if let Err(e) = reminder_service
                    .update_data(|app_data| {
                        if let Some(medicine) = app_data.medicines.get_mut(&uuid) {
                            medicine.add_quantity(amount);
                        }
                    })
                    .await {
                    log::error!("Failed to refill medicine: {}", e);
                    bot.send_message(msg.chat.id, "❌ 补充药品失败").await?;
                } else {
                    bot.send_message(msg.chat.id, format!("✅ 已补充{}个药品", amount)).await?;
                }
                dialogue.update(State::Start).await?;
            } else {
                bot.send_message(msg.chat.id, "❌ 无效的药品ID").await?;
                dialogue.update(State::Start).await?;
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "请输入有效的数量（正整数）：").await?;
        }
    }
    Ok(())
}

