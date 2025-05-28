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
            log::info!("数据文件不存在，创建默认数据");
            return Ok(AppData::default());
        }

        // 带重试的文件读取
        for attempt in 1..=3 {
            match fs::read_to_string(&self.file_path) {
                Ok(content) => {
                    match serde_json::from_str::<AppData>(&content) {
                        Ok(data) => {
                            log::debug!("成功加载数据文件");
                            return Ok(data);
                        }
                        Err(e) => {
                            log::error!("解析JSON失败 (尝试 {}/3): {}", attempt, e);
                            if attempt == 3 {
                                // 备份损坏的文件
                                let backup_path = format!("{}.backup.{}", self.file_path, chrono::Local::now().timestamp());
                                if let Err(backup_err) = fs::copy(&self.file_path, &backup_path) {
                                    log::error!("备份损坏文件失败: {}", backup_err);
                                } else {
                                    log::info!("已备份损坏文件到: {}", backup_path);
                                }
                                return Ok(AppData::default());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("读取文件失败 (尝试 {}/3): {}", attempt, e);
                    if attempt < 3 {
                        std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
                    } else {
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(AppData::default())
    }

    pub fn save(&self, data: &AppData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 带重试的文件保存
        for attempt in 1..=3 {
            match serde_json::to_string_pretty(data) {
                Ok(content) => {
                    // 先写入临时文件，然后原子性替换
                    let temp_path = format!("{}.tmp", self.file_path);
                    match fs::write(&temp_path, &content) {
                        Ok(_) => {
                            match fs::rename(&temp_path, &self.file_path) {
                                Ok(_) => {
                                    log::debug!("成功保存数据文件");
                                    return Ok(());
                                }
                                Err(e) => {
                                    log::warn!("重命名临时文件失败 (尝试 {}/3): {}", attempt, e);
                                    // 清理临时文件
                                    let _ = fs::remove_file(&temp_path);
                                    if attempt == 3 {
                                        return Err(e.into());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("写入临时文件失败 (尝试 {}/3): {}", attempt, e);
                            if attempt == 3 {
                                return Err(e.into());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("序列化数据失败: {}", e);
                    return Err(e.into());
                }
            }

            if attempt < 3 {
                std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
            }
        }

        Err("保存文件失败，已重试3次".into())
    }
}
