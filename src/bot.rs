use crate::{localization, storage::Language, Medicine, ReminderService};
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
    #[command(description = "切换语言")]
    Language,
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
                .branch(case![Command::Pending].endpoint(show_pending))
                .branch(case![Command::Language].endpoint(show_language_selection)),
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

async fn help(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let help_text = localization::format_help_message(language);

    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}

async fn show_language_selection(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback(text.chinese_button, "lang_chinese"),
            InlineKeyboardButton::callback(text.english_button, "lang_english"),
        ],
    ];
    let markup = InlineKeyboardMarkup::new(keyboard);

    let current_lang = if matches!(language, Language::Chinese) {
        "当前语言：中文"
    } else {
        "Current language: English"
    };

    let message = format!("{}\n\n{}", current_lang, text.select_language);

    bot.send_message(msg.chat.id, message)
        .reply_markup(markup)
        .await?;
    Ok(())
}

async fn start_add_medicine(bot: Bot, dialogue: MyDialogue, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    bot.send_message(msg.chat.id, text.enter_medicine_name).await?;
    dialogue.update(State::ReceiveMedicineName).await?;
    Ok(())
}

async fn receive_medicine_name(bot: Bot, dialogue: MyDialogue, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    match msg.text() {
        Some(name) => {
            let message = format!("{}：{}\n{}",
                if matches!(language, Language::Chinese) { "药品名称" } else { "Medicine name" },
                name,
                text.enter_quantity
            );
            bot.send_message(msg.chat.id, message).await?;
            dialogue
                .update(State::ReceiveQuantity {
                    name: name.to_string(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, text.enter_medicine_name).await?;
        }
    }
    Ok(())
}

async fn receive_quantity(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    name: String,
    reminder_service: Arc<ReminderService>,
) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    match msg.text().and_then(|text| text.parse::<u32>().ok()) {
        Some(quantity) => {
            let message = format!(
                "{}：{}\n{}：{}\n\n{}",
                if matches!(language, Language::Chinese) { "药品" } else { "Medicine" },
                name,
                if matches!(language, Language::Chinese) { "数量" } else { "Quantity" },
                quantity,
                text.enter_reminder_times
            );
            bot.send_message(msg.chat.id, message).await?;
            dialogue
                .update(State::ReceiveReminderTimes { name, quantity })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, text.invalid_quantity).await?;
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
                    let data = reminder_service.get_data().await;
                    let language = &data.user_settings.language;
                    let text = localization::get_text(language);

                    let medicine = Medicine::new(name.clone(), quantity, reminder_times.clone());

                    if let Err(e) = reminder_service
                        .update_data(|data| {
                            data.medicines.insert(medicine.id, medicine);
                        })
                        .await {
                        log::error!("Failed to save medicine: {}", e);
                        let error_msg = if matches!(language, Language::Chinese) {
                            "❌ 保存药品信息失败"
                        } else {
                            "❌ Failed to save medicine information"
                        };
                        bot.send_message(msg.chat.id, error_msg).await?;
                        return Ok(());
                    }

                    let times_display: Vec<String> = reminder_times
                        .iter()
                        .map(|t| t.format("%H:%M").to_string())
                        .collect();

                    let message = format!(
                        "{}\n\n💊 {}：{}\n📦 {}：{}\n⏰ {}：{}",
                        text.medicine_added,
                        if matches!(language, Language::Chinese) { "名称" } else { "Name" },
                        name,
                        if matches!(language, Language::Chinese) { "数量" } else { "Quantity" },
                        quantity,
                        if matches!(language, Language::Chinese) { "提醒时间" } else { "Reminder times" },
                        times_display.join(", ")
                    );

                    bot.send_message(msg.chat.id, message).await?;
                    dialogue.update(State::Start).await?;
                }
                _ => {
                    let data = reminder_service.get_data().await;
                    let language = &data.user_settings.language;
                    let text = localization::get_text(language);
                    bot.send_message(msg.chat.id, text.invalid_time_format).await?;
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
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    if data.medicines.is_empty() {
        bot.send_message(msg.chat.id, text.no_medicines).await?;
        return Ok(());
    }

    let mut message = format!("{}\n\n", text.medicines_list);
    for (i, medicine) in data.medicines.values().enumerate() {
        let status = if medicine.is_active { "🟢" } else { "🔴" };
        let times: Vec<String> = medicine
            .reminder_times
            .iter()
            .map(|t| t.format("%H:%M").to_string())
            .collect();

        message.push_str(&format!(
            "{}. {} {}\n📦 {}：{}\n⏰ {}：{}\n\n",
            i + 1,
            status,
            medicine.name,
            if matches!(language, Language::Chinese) { "数量" } else { "Quantity" },
            medicine.quantity,
            if matches!(language, Language::Chinese) { "提醒时间" } else { "Reminder times" },
            times.join(", ")
        ));
    }

    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

async fn delete_medicine(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    if data.medicines.is_empty() {
        let no_medicines_msg = if matches!(language, Language::Chinese) {
            "📭 暂无药品可删除"
        } else {
            "📭 No medicines to delete"
        };
        bot.send_message(msg.chat.id, no_medicines_msg).await?;
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
    bot.send_message(msg.chat.id, text.select_medicine_to_delete)
        .reply_markup(markup)
        .await?;
    Ok(())
}

async fn refill_medicine(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    if data.medicines.is_empty() {
        let no_medicines_msg = if matches!(language, Language::Chinese) {
            "📭 暂无药品可补充"
        } else {
            "📭 No medicines to refill"
        };
        bot.send_message(msg.chat.id, no_medicines_msg).await?;
        return Ok(());
    }

    let mut keyboard = Vec::new();
    for medicine in data.medicines.values() {
        let remaining_text = if matches!(language, Language::Chinese) {
            format!("💊 {} (剩余: {})", medicine.name, medicine.quantity)
        } else {
            format!("💊 {} (remaining: {})", medicine.name, medicine.quantity)
        };
        keyboard.push(vec![InlineKeyboardButton::callback(
            remaining_text,
            format!("refill_{}", medicine.id),
        )]);
    }

    let markup = InlineKeyboardMarkup::new(keyboard);
    bot.send_message(msg.chat.id, text.select_medicine_to_refill)
        .reply_markup(markup)
        .await?;
    Ok(())
}

async fn show_pending(bot: Bot, msg: Message, reminder_service: Arc<ReminderService>) -> HandlerResult {
    let data = reminder_service.get_data().await;
    let language = &data.user_settings.language;
    let text = localization::get_text(language);

    let pending: Vec<_> = data
        .pending_reminders
        .values()
        .filter(|r| !r.is_confirmed)
        .collect();

    if pending.is_empty() {
        bot.send_message(msg.chat.id, text.no_pending_reminders).await?;
        return Ok(());
    }

    let mut message = format!("{}\n\n", text.pending_reminders_title);
    for (i, reminder) in pending.iter().enumerate() {
        message.push_str(&format!(
            "{}. 💊 {}\n⏰ {}：{}\n📊 {}：{}\n\n",
            i + 1,
            reminder.medicine_name,
            if matches!(language, Language::Chinese) { "时间" } else { "Time" },
            reminder.scheduled_time.format("%H:%M"),
            if matches!(language, Language::Chinese) { "提醒次数" } else { "Reminder count" },
            reminder.reminder_count
        ));
    }

    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

async fn handle_callback(bot: Bot, q: CallbackQuery, reminder_service: Arc<ReminderService>, dialogue: MyDialogue) -> HandlerResult {
    if let Some(data) = &q.data {

        if let Some(chat_id) = q.message.as_ref().map(|m| m.chat.id) {
            if data.starts_with("lang_") {
                let new_language = if data == "lang_chinese" {
                    Language::Chinese
                } else if data == "lang_english" {
                    Language::English
                } else {
                    return Ok(());
                };

                if let Err(e) = reminder_service
                    .update_data(|app_data| {
                        app_data.user_settings.language = new_language.clone();
                    })
                    .await {
                    log::error!("Failed to update language: {}", e);
                    bot.send_message(chat_id, "❌ Failed to update language / 更新语言失败").await?;
                } else {
                    let text = localization::get_text(&new_language);
                    bot.send_message(chat_id, text.language_changed).await?;
                }
            } else if data.starts_with("confirm_") {
                let reminder_id = data.strip_prefix("confirm_").unwrap();
                if let Ok(_uuid) = Uuid::parse_str(reminder_id) {
                    let current_data = reminder_service.get_data().await;
                    let language = &current_data.user_settings.language;
                    let text = localization::get_text(language);

                    // 显示数量选择界面
                    let keyboard = vec![
                        vec![
                            InlineKeyboardButton::callback(
                                format!("1{}", text.pills_unit),
                                format!("dose_1_{}", reminder_id)
                            ),
                            InlineKeyboardButton::callback(
                                format!("2{}", text.pills_unit),
                                format!("dose_2_{}", reminder_id)
                            ),
                            InlineKeyboardButton::callback(
                                format!("3{}", text.pills_unit),
                                format!("dose_3_{}", reminder_id)
                            ),
                        ],
                        vec![
                            InlineKeyboardButton::callback(text.custom_amount_button, format!("dose_custom_{}", reminder_id)),
                        ],
                    ];
                    let markup = InlineKeyboardMarkup::new(keyboard);
                    bot.send_message(chat_id, text.select_dose_amount)
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
                    let current_data = reminder_service.get_data().await;
                    let language = &current_data.user_settings.language;
                    let text = localization::get_text(language);

                    // 显示数量选择界面
                    let keyboard = vec![
                        vec![
                            InlineKeyboardButton::callback(
                                format!("10{}", text.pieces_unit),
                                format!("refill_10_{}", medicine_id)
                            ),
                            InlineKeyboardButton::callback(
                                format!("20{}", text.pieces_unit),
                                format!("refill_20_{}", medicine_id)
                            ),
                            InlineKeyboardButton::callback(
                                format!("30{}", text.pieces_unit),
                                format!("refill_30_{}", medicine_id)
                            ),
                        ],
                        vec![
                            InlineKeyboardButton::callback(text.custom_amount_button, format!("refill_custom_{}", medicine_id)),
                        ],
                    ];
                    let markup = InlineKeyboardMarkup::new(keyboard);
                    bot.send_message(chat_id, text.enter_refill_amount)
                        .reply_markup(markup)
                        .await?;
                }
            } else if data.starts_with("dose_") {
                // 处理服药数量选择
                if data.starts_with("dose_custom_") {
                    let reminder_id = data.strip_prefix("dose_custom_").unwrap();
                    let current_data = reminder_service.get_data().await;
                    let language = &current_data.user_settings.language;
                    let text = localization::get_text(language);

                    bot.send_message(chat_id, text.enter_custom_amount).await?;
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
                    let current_data = reminder_service.get_data().await;
                    let language = &current_data.user_settings.language;
                    let text = localization::get_text(language);

                    bot.send_message(chat_id, text.enter_refill_amount).await?;
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

