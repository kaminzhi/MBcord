use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::SerenityInit;

#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "需要先加入一個語音頻道!").await);
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    match _handler {
        Ok(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "已加入語音頻道!").await);
        }
        Err(e) => {
            println!("Error joining voice channel: {:?}", e);
            check_msg(msg.channel_id.say(&ctx.http, "加入語音頻道時出錯").await);
        }
    }

    Ok(())
}

/// 檢查消息發送是否成功
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
