mod commands;
mod handlers;
mod services;

use dotenv::dotenv;
use serenity::framework::standard::StandardFramework;
use serenity::Client;
use songbird::SerenityInit;
use std::env;

use crate::commands::MUSIC_GROUP;
use crate::handlers::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Discord token not set");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&MUSIC_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
