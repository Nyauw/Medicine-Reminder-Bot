use medicine_reminder::{bot, ReminderService, Storage};
use std::env;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ChatId;

#[tokio::main]
async fn main() {
    // 加载 .env 文件
    if let Err(e) = dotenv::dotenv() {
        println!("警告: 无法加载 .env 文件: {}", e);
        println!("请确保 .env 文件存在并包含必要的配置");
    }

    // 初始化日志
    env_logger::init();
    log::info!("启动药品提醒机器人...");

    // 从环境变量获取配置
    let bot_token = env::var("TELOXIDE_TOKEN")
        .expect("请设置 TELOXIDE_TOKEN 环境变量");

    let chat_id_str = env::var("CHAT_ID")
        .expect("请设置 CHAT_ID 环境变量");

    let chat_id = ChatId(chat_id_str.parse::<i64>()
        .expect("CHAT_ID 必须是有效的数字"));

    // 创建Bot实例
    let bot = Bot::new(bot_token);

    // 创建存储和提醒服务
    let storage = Storage::new("medicine_data.json");
    let reminder_service = Arc::new(ReminderService::new(
        storage,
        bot.clone(),
        chat_id,
    ));

    // 启动提醒循环（在后台运行）
    let reminder_service_clone = reminder_service.clone();
    tokio::spawn(async move {
        reminder_service_clone.start_reminder_loop().await;
    });

    // 发送启动消息
    if let Err(e) = bot
        .send_message(chat_id, "🤖 药品提醒机器人已启动！\n\n使用 /help 查看可用命令。")
        .await
    {
        log::error!("发送启动消息失败: {}", e);
    }

    log::info!("机器人已启动，等待消息...");

    // 启动Bot
    Dispatcher::builder(bot, bot::schema())
        .dependencies(dptree::deps![
            teloxide::dispatching::dialogue::InMemStorage::<bot::State>::new(),
            reminder_service
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
