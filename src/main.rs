mod commands;
mod handlers;
mod services;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

use crate::commands::{join::*, leave::*, play::*};
use crate::handlers::Handler;

#[group]
#[commands(join, leave, play)]
struct Music;

#[tokio::main]
async fn main() {
    // 加載 .env 文件中的環境變量
    dotenv().ok();

    // 設置日誌
    env_logger::init();

    // 從環境變量獲取 Discord token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // 設置命令框架
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").allow_dm(false).case_insensitivity(true))
        .group(&MUSIC_GROUP);

    // 創建事件處理器
    let handler = Handler::new();

    // 構建客戶端
    let mut client = Client::builder(&token)
        .event_handler(handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    // 啟動客戶端
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

