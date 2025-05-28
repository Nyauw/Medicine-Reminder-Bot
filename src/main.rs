use medicine_reminder::{bot, ReminderService, Storage};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let reminder_handle = tokio::spawn(async move {
        if let Err(e) = reminder_service_clone.start_reminder_loop().await {
            log::error!("提醒循环出错: {}", e);
        }
    });

    // 发送启动消息（带超时）
    let startup_data = reminder_service.get_data().await;
    let startup_language = &startup_data.user_settings.language;
    let startup_text = medicine_reminder::localization::get_text(startup_language);

    let send_startup_msg = async {
        if let Err(e) = bot
            .send_message(chat_id, startup_text.startup_message)
            .await
        {
            log::error!("发送启动消息失败: {}", e);
        }
    };

    // 5秒超时发送启动消息
    if let Err(_) = tokio::time::timeout(Duration::from_secs(5), send_startup_msg).await {
        log::warn!("发送启动消息超时");
    }

    log::info!("机器人已启动，等待消息...");

    // 创建调度器
    let mut dispatcher = Dispatcher::builder(bot, bot::schema())
        .dependencies(dptree::deps![
            teloxide::dispatching::dialogue::InMemStorage::<bot::State>::new(),
            reminder_service
        ])
        .build();

    // 设置优雅关闭
    tokio::select! {
        _ = dispatcher.dispatch() => {
            log::info!("调度器已停止");
        }
        _ = signal::ctrl_c() => {
            log::info!("收到 Ctrl+C 信号，正在关闭...");
            reminder_handle.abort();
        }
    }

    log::info!("程序已安全关闭");
    Ok(())
}
