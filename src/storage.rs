use crate::{MedicineStore, PendingReminders};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Chinese
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: Language,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            language: Language::Chinese,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub medicines: MedicineStore,
    pub pending_reminders: PendingReminders,
    #[serde(default)]
    pub user_settings: UserSettings,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            medicines: MedicineStore::new(),
            pending_reminders: PendingReminders::new(),
            user_settings: UserSettings::default(),
        }
    }
}

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn load(&self) -> Result<AppData, Box<dyn std::error::Error + Send + Sync>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(AppData::default());
        }

        let content = fs::read_to_string(&self.file_path)?;
        let data: AppData = serde_json::from_str(&content)?;
        Ok(data)
    }

    pub fn save(&self, data: &AppData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}
