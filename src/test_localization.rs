#[cfg(test)]
mod tests {
    use crate::{localization, storage::{Language, UserSettings, AppData}};

    #[test]
    fn test_chinese_localization() {
        let language = Language::Chinese;
        let text = localization::get_text(&language);
        
        assert_eq!(text.help_title, "🏥 药品提醒助手");
        assert_eq!(text.startup_message, "🤖 药品提醒机器人已启动！\n\n使用 /help 查看可用命令。");
        assert_eq!(text.taken_button, "✅ 已服药");
        assert_eq!(text.snooze_button, "⏰ 稍后提醒");
    }

    #[test]
    fn test_english_localization() {
        let language = Language::English;
        let text = localization::get_text(&language);
        
        assert_eq!(text.help_title, "🏥 Medicine Reminder Assistant");
        assert_eq!(text.startup_message, "🤖 Medicine Reminder Bot started!\n\nUse /help to see available commands.");
        assert_eq!(text.taken_button, "✅ Taken");
        assert_eq!(text.snooze_button, "⏰ Snooze");
    }

    #[test]
    fn test_help_message_formatting() {
        let chinese_help = localization::format_help_message(&Language::Chinese);
        assert!(chinese_help.contains("药品提醒助手"));
        assert!(chinese_help.contains("/language - 切换语言"));
        
        let english_help = localization::format_help_message(&Language::English);
        assert!(english_help.contains("Medicine Reminder Assistant"));
        assert!(english_help.contains("/language - Switch language"));
    }

    #[test]
    fn test_reminder_message_formatting() {
        let chinese_msg = localization::format_reminder_message(
            &Language::Chinese, 
            "维生素C", 
            "08:00"
        );
        assert!(chinese_msg.contains("🔔 吃药提醒！"));
        assert!(chinese_msg.contains("药品：维生素C"));
        assert!(chinese_msg.contains("时间：08:00"));
        
        let english_msg = localization::format_reminder_message(
            &Language::English, 
            "Vitamin C", 
            "08:00"
        );
        assert!(english_msg.contains("🔔 Medicine Reminder!"));
        assert!(english_msg.contains("Medicine：Vitamin C"));
        assert!(english_msg.contains("Time：08:00"));
    }

    #[test]
    fn test_default_language() {
        let settings = UserSettings::default();
        assert_eq!(settings.language, Language::Chinese);
        
        let app_data = AppData::default();
        assert_eq!(app_data.user_settings.language, Language::Chinese);
    }

    #[test]
    fn test_language_enum() {
        assert_eq!(Language::default(), Language::Chinese);
        
        // Test serialization/deserialization would work
        let chinese = Language::Chinese;
        let english = Language::English;
        
        assert_ne!(chinese, english);
        assert_eq!(chinese, Language::Chinese);
        assert_eq!(english, Language::English);
    }
}
