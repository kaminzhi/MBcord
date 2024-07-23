use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::SerenityInit;

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.leave().await?;
        check_msg(msg.channel_id.say(&ctx.http, "已離開語音頻道!").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "目前不在任何語音頻道中")
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
