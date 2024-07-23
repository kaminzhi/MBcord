use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::input::Restartable;

use crate::services::{spotify::SpotifyService, youtube};

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "你需要先加入一個語音頻道!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let query = args.rest();
        if query.is_empty() {
            msg.reply(ctx, "請提供要播放的歌曲名稱或 URL").await?;
            return Ok(());
        }

        msg.channel_id.say(&ctx.http, "🔍 正在搜索...").await?;

        let source = if query.starts_with("https://") {
            // 直接使用 URL
            match Restartable::ytdl(query, true).await {
                Ok(source) => source.into(),
                Err(why) => {
                    msg.reply(ctx, &format!("錯誤: 無法播放此 URL: {}", why))
                        .await?;
                    return Ok(());
                }
            }
        } else {
            // 搜索歌曲
            let spotify = match SpotifyService::new().await {
                Ok(spotify) => spotify,
                Err(why) => {
                    msg.reply(ctx, &format!("錯誤: 無法連接到 Spotify: {}", why))
                        .await?;
                    return Ok(());
                }
            };

            let track_info = match spotify.search_track(query).await {
                Ok(info) => info,
                Err(_) => query.to_string(),
            };

            match youtube::search_and_get_source(&track_info).await {
                Ok(source) => source,
                Err(why) => {
                    msg.reply(ctx, &format!("錯誤: 無法找到歌曲: {}", why))
                        .await?;
                    return Ok(());
                }
            }
        };

        // 獲取視頻信息
        let video_info = match youtube::get_video_info(&source.metadata.source_url).await {
            Ok(info) => info,
            Err(_) => "未知歌曲".to_string(),
        };

        handler.play_source(source);

        msg.channel_id
            .say(&ctx.http, format!("🎵 正在播放: {}", video_info))
            .await?;
    } else {
        msg.reply(ctx, "錯誤: 無法加入語音頻道").await?;
    }

    Ok(())
}

