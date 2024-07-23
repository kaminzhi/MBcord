use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::{Guild, GuildUnavailable};
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use songbird::SerenityInit;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Handler {
    pub command_counter: Arc<Mutex<HashMap<String, u64>>>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            command_counter: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::listening("!help")).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        println!("Cache built successfully!");
        println!("Connected to {} guilds", guilds.len());

        // 初始化每個伺服器的設置
        for guild_id in guilds {
            if let Err(e) = initialize_guild_settings(&ctx, guild_id).await {
                println!(
                    "Error initializing settings for guild {}: {:?}",
                    guild_id, e
                );
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with('!') {
            let command = msg.content.split_whitespace().next().unwrap_or("");
            let mut counter = self.command_counter.lock().await;
            *counter.entry(command.to_string()).or_insert(0) += 1;

            if command == "!stats" {
                let stats = counter
                    .iter()
                    .map(|(cmd, count)| format!("{}: {}", cmd, count))
                    .collect::<Vec<String>>()
                    .join("\n");
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("Command usage stats:\n{}", stats))
                    .await
                {
                    println!("Error sending message: {:?}", e);
                }
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            println!("Joined a new guild: {}", guild.name);
            if let Err(e) = initialize_guild_settings(&ctx, guild.id).await {
                println!(
                    "Error initializing settings for new guild {}: {:?}",
                    guild.id, e
                );
            }
        }
    }

    async fn guild_delete(&self, _: Context, incomplete: GuildUnavailable, _: Option<Guild>) {
        println!("Left guild: {}", incomplete.id);
    }

    async fn voice_state_update(&self, ctx: Context, _: Option<VoiceState>, new: VoiceState) {
        if let Some(guild_id) = new.guild_id {
            let manager = songbird::get(&ctx)
                .await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();

            if let Some(handler_lock) = manager.get(guild_id) {
                let mut handler = handler_lock.lock().await;
                let should_leave = match new.channel_id {
                    Some(channel_id) => {
                        let channel = channel_id.to_channel(&ctx).await.unwrap().guild().unwrap();
                        channel.members(&ctx).await.unwrap().len() <= 1
                    }
                    None => true,
                };

                if should_leave {
                    if let Err(e) = handler.leave().await {
                        println!("Error leaving voice channel: {:?}", e);
                    }
                }
            }
        }
    }

    async fn unhandled_event(
        &self,
        _ctx: Context,
        event: Event,
        raw: UnparseableEventType,
        data: Option<Value>,
    ) {
        println!("Unhandled event: {:?}", event);
        println!("Raw event data: {:?}", raw);
        println!("Extra data: {:?}", data);
    }
}

async fn initialize_guild_settings(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<(), Box<dyn std::error::Error>> {
    // 這裡你可以初始化伺服器特定的設置
    // 例如，設置默認的前綴，歡迎消息等
    println!("Initializing settings for guild: {}", guild_id);
    // 實現你的初始化邏輯...
    Ok(())
}
