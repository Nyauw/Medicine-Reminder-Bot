use crate::storage::Language;

pub struct LocalizedText {
    pub help_title: &'static str,
    pub help_commands: &'static str,
    pub help_usage: &'static str,
    pub startup_message: &'static str,
    pub add_medicine_prompt: &'static str,
    pub enter_medicine_name: &'static str,
    pub enter_quantity: &'static str,
    pub enter_reminder_times: &'static str,
    pub medicine_added: &'static str,
    pub invalid_time_format: &'static str,
    pub invalid_quantity: &'static str,
    pub no_medicines: &'static str,
    pub medicines_list: &'static str,
    pub select_medicine_to_delete: &'static str,
    pub medicine_deleted: &'static str,
    pub medicine_not_found: &'static str,
    pub select_medicine_to_refill: &'static str,
    pub enter_refill_amount: &'static str,
    pub medicine_refilled: &'static str,
    pub no_pending_reminders: &'static str,
    pub pending_reminders_title: &'static str,
    pub reminder_message: &'static str,
    pub taken_button: &'static str,
    pub snooze_button: &'static str,
    pub select_dose_amount: &'static str,
    pub dose_confirmed: &'static str,
    pub insufficient_quantity: &'static str,
    pub language_changed: &'static str,
    pub current_language: &'static str,
    pub select_language: &'static str,
    pub chinese_button: &'static str,
    pub english_button: &'static str,
    pub custom_amount_button: &'static str,
    pub enter_custom_amount: &'static str,
    pub pills_unit: &'static str,
    pub pieces_unit: &'static str,
}

const CHINESE_TEXT: LocalizedText = LocalizedText {
    help_title: "🏥 药品提醒助手",
    help_commands: "📋 可用命令：\n\
        /add - 添加新药品\n\
        /list - 查看所有药品\n\
        /delete - 删除药品\n\
        /refill - 补充药品数量\n\
        /pending - 查看待确认的提醒\n\
        /language - 切换语言\n\
        /help - 显示此帮助信息",
    help_usage: "💡 使用说明：\n\
        1. 使用 /add 添加药品，设置名称、数量和提醒时间\n\
        2. 系统会在设定时间自动提醒\n\
        3. 收到提醒后请点击确认按钮\n\
        4. 如果不确认，系统会持续提醒",
    startup_message: "🤖 药品提醒机器人已启动！\n\n使用 /help 查看可用命令。",
    add_medicine_prompt: "请输入药品名称：",
    enter_medicine_name: "请输入药品名称：",
    enter_quantity: "请输入药品数量：",
    enter_reminder_times: "请输入提醒时间（格式：HH:MM，多个时间用逗号分隔）：\n例如：08:00,20:00",
    medicine_added: "✅ 药品添加成功！",
    invalid_time_format: "❌ 时间格式错误！请使用 HH:MM 格式，例如：08:00,20:00",
    invalid_quantity: "❌ 数量格式错误！请输入有效的数字。",
    no_medicines: "📭 暂无药品记录。使用 /add 添加新药品。",
    medicines_list: "💊 您的药品列表：",
    select_medicine_to_delete: "请选择要删除的药品：",
    medicine_deleted: "✅ 药品已删除",
    medicine_not_found: "❌ 未找到该药品",
    select_medicine_to_refill: "请选择要补充的药品：",
    enter_refill_amount: "请输入补充数量：",
    medicine_refilled: "✅ 药品数量已补充",
    no_pending_reminders: "📭 暂无待确认的提醒。",
    pending_reminders_title: "⏰ 待确认的提醒：",
    reminder_message: "🔔 吃药提醒！",
    taken_button: "✅ 已服药",
    snooze_button: "⏰ 稍后提醒",
    select_dose_amount: "请选择服用数量：",
    dose_confirmed: "✅ 已确认服药",
    insufficient_quantity: "❌ 药品数量不足！请先补充。",
    language_changed: "✅ 语言已切换",
    current_language: "当前语言：中文",
    select_language: "请选择语言：",
    chinese_button: "🇨🇳 中文",
    english_button: "🇺🇸 English",
    custom_amount_button: "自定义数量",
    enter_custom_amount: "请输入自定义数量：",
    pills_unit: "片",
    pieces_unit: "个",
};

const ENGLISH_TEXT: LocalizedText = LocalizedText {
    help_title: "🏥 Medicine Reminder Assistant",
    help_commands: "📋 Available Commands:\n\
        /add - Add new medicine\n\
        /list - View all medicines\n\
        /delete - Delete medicine\n\
        /refill - Refill medicine quantity\n\
        /pending - View pending reminders\n\
        /language - Switch language\n\
        /help - Show this help message",
    help_usage: "💡 Usage Instructions:\n\
        1. Use /add to add medicine, set name, quantity and reminder times\n\
        2. System will automatically remind at scheduled times\n\
        3. Click confirm button when you receive reminders\n\
        4. If not confirmed, system will keep reminding",
    startup_message: "🤖 Medicine Reminder Bot started!\n\nUse /help to see available commands.",
    add_medicine_prompt: "Please enter medicine name:",
    enter_medicine_name: "Please enter medicine name:",
    enter_quantity: "Please enter medicine quantity:",
    enter_reminder_times: "Please enter reminder times (format: HH:MM, separate multiple times with commas):\nExample: 08:00,20:00",
    medicine_added: "✅ Medicine added successfully!",
    invalid_time_format: "❌ Invalid time format! Please use HH:MM format, example: 08:00,20:00",
    invalid_quantity: "❌ Invalid quantity format! Please enter a valid number.",
    no_medicines: "📭 No medicine records. Use /add to add new medicine.",
    medicines_list: "💊 Your Medicine List:",
    select_medicine_to_delete: "Please select medicine to delete:",
    medicine_deleted: "✅ Medicine deleted",
    medicine_not_found: "❌ Medicine not found",
    select_medicine_to_refill: "Please select medicine to refill:",
    enter_refill_amount: "Please enter refill amount:",
    medicine_refilled: "✅ Medicine quantity refilled",
    no_pending_reminders: "📭 No pending reminders.",
    pending_reminders_title: "⏰ Pending Reminders:",
    reminder_message: "🔔 Medicine Reminder!",
    taken_button: "✅ Taken",
    snooze_button: "⏰ Snooze",
    select_dose_amount: "Please select dose amount:",
    dose_confirmed: "✅ Dose confirmed",
    insufficient_quantity: "❌ Insufficient quantity! Please refill first.",
    language_changed: "✅ Language switched",
    current_language: "Current language: English",
    select_language: "Please select language:",
    chinese_button: "🇨🇳 中文",
    english_button: "🇺🇸 English",
    custom_amount_button: "Custom Amount",
    enter_custom_amount: "Please enter custom amount:",
    pills_unit: "pills",
    pieces_unit: "pcs",
};

pub fn get_text(language: &Language) -> &'static LocalizedText {
    match language {
        Language::Chinese => &CHINESE_TEXT,
        Language::English => &ENGLISH_TEXT,
    }
}

pub fn format_help_message(language: &Language) -> String {
    let text = get_text(language);
    format!("{}\n\n{}\n\n{}", text.help_title, text.help_commands, text.help_usage)
}

pub fn format_reminder_message(language: &Language, medicine_name: &str, time: &str) -> String {
    let text = get_text(language);
    format!(
        "{}\n\n💊 {}：{}\n⏰ {}：{}\n\n{}：",
        text.reminder_message,
        if matches!(language, Language::Chinese) { "药品" } else { "Medicine" },
        medicine_name,
        if matches!(language, Language::Chinese) { "时间" } else { "Time" },
        time,
        if matches!(language, Language::Chinese) { "请点击下面的按钮确认已服药" } else { "Please click the button below to confirm you have taken the medicine" }
    )
}

pub fn format_medicine_list(language: &Language, medicines: &[(String, u32, String)]) -> String {
    let text = get_text(language);
    let mut result = format!("{}\n\n", text.medicines_list);
    
    for (i, (name, quantity, times)) in medicines.iter().enumerate() {
        result.push_str(&format!(
            "{}. {} - {} {} ({})\n",
            i + 1,
            name,
            quantity,
            if matches!(language, Language::Chinese) { "个" } else { "pcs" },
            times
        ));
    }
    
    result
}
