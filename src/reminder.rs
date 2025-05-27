use crate::{localization, AppData, PendingReminder, Storage};
use chrono::{Duration, Local, NaiveTime};
use std::sync::Arc;
use teloxide::{prelude::*, types::ChatId};
use tokio::sync::Mutex;
use tokio::time::interval;
use uuid::Uuid;

pub struct ReminderService {
    storage: Storage,
    data: Arc<Mutex<AppData>>,
    bot: Bot,
    chat_id: ChatId,
}

impl ReminderService {
    pub fn new(storage: Storage, bot: Bot, chat_id: ChatId) -> Self {
        let data = Arc::new(Mutex::new(storage.load().unwrap_or_default()));
        Self {
            storage,
            data,
            bot,
            chat_id,
        }
    }

    pub async fn start_reminder_loop(&self) {
        let mut interval = interval(tokio::time::Duration::from_secs(60)); // 每分钟检查一次

        loop {
            interval.tick().await;
            self.check_and_send_reminders().await;
            self.check_pending_reminders().await;
        }
    }

    async fn check_and_send_reminders(&self) {
        let now = Local::now();
        let current_time = now.time();
        let mut new_reminders = Vec::new();

        // 首先收集需要创建的提醒
        {
            let data = self.data.lock().await;
            for medicine in data.medicines.values() {
                if !medicine.is_active || medicine.quantity == 0 {
                    continue;
                }

                for &reminder_time in &medicine.reminder_times {
                    // 检查是否到了提醒时间（允许1分钟的误差）
                    if self.is_time_to_remind(current_time, reminder_time) {
                        // 检查是否已经有待确认的提醒
                        let has_pending = data
                            .pending_reminders
                            .values()
                            .any(|r| r.medicine_id == medicine.id && !r.is_confirmed);

                        if !has_pending {
                            let scheduled_time = now
                                .date_naive()
                                .and_time(reminder_time)
                                .and_local_timezone(Local)
                                .unwrap();

                            let reminder = PendingReminder::new(
                                medicine.id,
                                medicine.name.clone(),
                                scheduled_time,
                            );

                            new_reminders.push(reminder);
                        }
                    }
                }
            }
        }

        // 然后发送提醒并保存
        for reminder in new_reminders {
            self.send_reminder_message(&reminder).await;
            let mut data = self.data.lock().await;
            data.pending_reminders.insert(reminder.id, reminder);
            if let Err(e) = self.storage.save(&data) {
                log::error!("Failed to save data: {}", e);
            }
        }
    }

    async fn check_pending_reminders(&self) {
        let mut data = self.data.lock().await;
        let now = Local::now();
        let mut to_remind = Vec::new();

        for reminder in data.pending_reminders.values_mut() {
            if !reminder.is_confirmed {
                let time_since_last = now.signed_duration_since(reminder.last_reminder_time);

                // 根据提醒次数调整间隔：第1次后5分钟，第2次后10分钟，之后每15分钟
                let interval_minutes = match reminder.reminder_count {
                    1 => 5,
                    2 => 10,
                    _ => 15,
                };

                if time_since_last >= Duration::minutes(interval_minutes) {
                    reminder.increment_reminder();
                    to_remind.push(reminder.clone());
                }
            }
        }

        for reminder in to_remind {
            self.send_follow_up_reminder(&reminder).await;
        }

        if let Err(e) = self.storage.save(&data) {
            log::error!("Failed to save data: {}", e);
        }
    }

    fn is_time_to_remind(&self, current_time: NaiveTime, reminder_time: NaiveTime) -> bool {
        let diff = if current_time >= reminder_time {
            current_time - reminder_time
        } else {
            reminder_time - current_time
        };

        diff <= chrono::Duration::minutes(1)
    }

    async fn send_reminder_message(&self, reminder: &PendingReminder) {
        let data = self.data.lock().await;
        let language = &data.user_settings.language;
        let text = localization::get_text(language);

        let message = localization::format_reminder_message(
            language,
            &reminder.medicine_name,
            &reminder.scheduled_time.format("%H:%M").to_string()
        );

        let keyboard = teloxide::types::InlineKeyboardMarkup::new(vec![vec![
            teloxide::types::InlineKeyboardButton::callback(
                text.taken_button,
                format!("confirm_{}", reminder.id),
            ),
            teloxide::types::InlineKeyboardButton::callback(
                text.snooze_button,
                format!("snooze_{}", reminder.id),
            ),
        ]]);

        if let Err(e) = self
            .bot
            .send_message(self.chat_id, message)
            .reply_markup(keyboard)
            .await
        {
            log::error!("Failed to send reminder message: {}", e);
        }
    }

    async fn send_follow_up_reminder(&self, reminder: &PendingReminder) {
        let data = self.data.lock().await;
        let language = &data.user_settings.language;
        let text = localization::get_text(language);

        let message = format!(
            "🔔 {}！\n\n💊 {}：{}\n⏰ {}：{}\n📊 {}：{}\n\n{}：",
            if matches!(language, crate::storage::Language::Chinese) { "再次提醒吃药" } else { "Medicine Reminder Again" },
            if matches!(language, crate::storage::Language::Chinese) { "药品" } else { "Medicine" },
            reminder.medicine_name,
            if matches!(language, crate::storage::Language::Chinese) { "原定时间" } else { "Scheduled time" },
            reminder.scheduled_time.format("%H:%M"),
            if matches!(language, crate::storage::Language::Chinese) { "提醒次数" } else { "Reminder count" },
            reminder.reminder_count,
            if matches!(language, crate::storage::Language::Chinese) { "请确认是否已服药" } else { "Please confirm if you have taken the medicine" }
        );

        let keyboard = teloxide::types::InlineKeyboardMarkup::new(vec![vec![
            teloxide::types::InlineKeyboardButton::callback(
                text.taken_button,
                format!("confirm_{}", reminder.id),
            ),
            teloxide::types::InlineKeyboardButton::callback(
                text.snooze_button,
                format!("snooze_{}", reminder.id),
            ),
        ]]);

        if let Err(e) = self
            .bot
            .send_message(self.chat_id, message)
            .reply_markup(keyboard)
            .await
        {
            log::error!("Failed to send follow-up reminder: {}", e);
        }
    }

    pub async fn confirm_medicine(&self, reminder_id: Uuid) -> Result<String, String> {
        let mut data = self.data.lock().await;

        if let Some(reminder) = data.pending_reminders.get_mut(&reminder_id) {
            reminder.confirm();
            let medicine_id = reminder.medicine_id;
            let medicine_name = reminder.medicine_name.clone();

            // 减少药品数量（默认1个）
            if let Some(medicine) = data.medicines.get_mut(&medicine_id) {
                if medicine.take_dose(1) {
                    let response = format!(
                        "✅ 已确认服药：{}\n💊 服用数量：1\n📦 剩余数量：{}",
                        medicine_name,
                        medicine.quantity
                    );

                    if let Err(e) = self.storage.save(&data) {
                        log::error!("Failed to save data: {}", e);
                    }

                    Ok(response)
                } else {
                    Err(format!("药品数量不足，当前剩余：{}", medicine.quantity))
                }
            } else {
                Err("药品信息未找到".to_string())
            }
        } else {
            Err("提醒信息未找到".to_string())
        }
    }

    pub async fn snooze_reminder(&self, reminder_id: Uuid) -> Result<String, String> {
        let mut data = self.data.lock().await;
        let language = data.user_settings.language.clone();

        if let Some(reminder) = data.pending_reminders.get_mut(&reminder_id) {
            // 重置最后提醒时间，延迟5分钟后再次提醒
            reminder.last_reminder_time = Local::now();

            if let Err(e) = self.storage.save(&data) {
                log::error!("Failed to save data: {}", e);
            }

            let response = if matches!(language, crate::storage::Language::Chinese) {
                "⏰ 已延迟提醒，5分钟后将再次提醒"
            } else {
                "⏰ Reminder snoozed, will remind again in 5 minutes"
            };
            Ok(response.to_string())
        } else {
            let error_msg = if matches!(language, crate::storage::Language::Chinese) {
                "提醒信息未找到"
            } else {
                "Reminder information not found"
            };
            Err(error_msg.to_string())
        }
    }

    pub async fn get_data(&self) -> AppData {
        let data = self.data.lock().await;
        data.clone()
    }

    pub async fn update_data<F>(&self, updater: F) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnOnce(&mut AppData),
    {
        let mut data = self.data.lock().await;
        updater(&mut data);
        self.storage.save(&data)?;
        Ok(())
    }

    pub async fn confirm_medicine_with_amount(&self, reminder_id: Uuid, amount: u32) -> Result<String, String> {
        let mut data = self.data.lock().await;
        let language = data.user_settings.language.clone();
        let text = localization::get_text(&language);

        if let Some(reminder) = data.pending_reminders.get_mut(&reminder_id) {
            reminder.confirm();
            let medicine_id = reminder.medicine_id;
            let medicine_name = reminder.medicine_name.clone();

            // 减少药品数量
            if let Some(medicine) = data.medicines.get_mut(&medicine_id) {
                if medicine.take_dose(amount) {
                    let response = format!(
                        "{}: {}\n💊 {}: {}\n📦 {}: {}",
                        text.dose_confirmed.trim_end_matches("✅ "),
                        medicine_name,
                        if matches!(language, crate::storage::Language::Chinese) { "服用数量" } else { "Dose amount" },
                        amount,
                        if matches!(language, crate::storage::Language::Chinese) { "剩余数量" } else { "Remaining" },
                        medicine.quantity
                    );

                    if let Err(e) = self.storage.save(&data) {
                        log::error!("Failed to save data: {}", e);
                    }

                    Ok(response)
                } else {
                    let error_msg = if matches!(language, crate::storage::Language::Chinese) {
                        format!("药品数量不足，当前剩余：{}", medicine.quantity)
                    } else {
                        format!("Insufficient quantity, remaining: {}", medicine.quantity)
                    };
                    Err(error_msg)
                }
            } else {
                let error_msg = if matches!(language, crate::storage::Language::Chinese) {
                    "药品信息未找到"
                } else {
                    "Medicine information not found"
                };
                Err(error_msg.to_string())
            }
        } else {
            let error_msg = if matches!(language, crate::storage::Language::Chinese) {
                "提醒信息未找到"
            } else {
                "Reminder information not found"
            };
            Err(error_msg.to_string())
        }
    }
}
