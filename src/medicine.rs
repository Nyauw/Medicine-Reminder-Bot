use chrono::{DateTime, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medicine {
    pub id: Uuid,
    pub name: String,
    pub quantity: u32,
    pub reminder_times: Vec<NaiveTime>,
    pub created_at: DateTime<Local>,
    pub is_active: bool,
}

impl Medicine {
    pub fn new(name: String, quantity: u32, reminder_times: Vec<NaiveTime>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            quantity,
            reminder_times,
            created_at: Local::now(),
            is_active: true,
        }
    }

    pub fn take_dose(&mut self, amount: u32) -> bool {
        if self.quantity >= amount {
            self.quantity -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_quantity(&mut self, amount: u32) {
        self.quantity += amount;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReminder {
    pub id: Uuid,
    pub medicine_id: Uuid,
    pub medicine_name: String,
    pub scheduled_time: DateTime<Local>,
    pub last_reminder_time: DateTime<Local>,
    pub reminder_count: u32,
    pub is_confirmed: bool,
}

impl PendingReminder {
    pub fn new(medicine_id: Uuid, medicine_name: String, scheduled_time: DateTime<Local>) -> Self {
        Self {
            id: Uuid::new_v4(),
            medicine_id,
            medicine_name,
            scheduled_time,
            last_reminder_time: scheduled_time,
            reminder_count: 1,
            is_confirmed: false,
        }
    }

    pub fn increment_reminder(&mut self) {
        self.reminder_count += 1;
        self.last_reminder_time = Local::now();
    }

    pub fn confirm(&mut self) {
        self.is_confirmed = true;
    }
}

pub type MedicineStore = HashMap<Uuid, Medicine>;
pub type PendingReminders = HashMap<Uuid, PendingReminder>;
