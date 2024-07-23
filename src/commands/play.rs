use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::SerenityInit;

use crate::services::spotify;
use crate::services::youtube;

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "請提供一個YouTube或Spotify URL")
                    .await,
            );
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = if url.contains("youtube.com") || url.contains("youtu.be") {
            youtube::get_source(&url).await?
        } else if url.contains("spotify.com") {
            spotify::get_source(&url).await?
        } else {
            check_msg(msg.channel_id.say(&ctx.http, "不支持的URL格式").await);
            return Ok(());
        };

        handler.play_source(source);
        check_msg(msg.channel_id.say(&ctx.http, "開始播放音樂!").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "機器人需要先加入語音頻道")
                .await,
        );
    }

    Ok(())
}

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

