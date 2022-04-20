use rand::Rng;
use twilight_cache_inmemory::model::CachedMessage;
use std::time::SystemTime;
use std::{error::Error, sync::Arc};
use tracing::info;
use twilight_model::id::marker::ChannelMarker;
use twilight_model::id::Id;
use twilight_model::{
    datetime::Timestamp,
    gateway::payload::incoming::{MessageDelete, MessageUpdate},
};

use crate::{
    helpers::{embed::AlertEmbed, message},
    structs::GuildConfig,
    AgpContext,
};

pub async fn handle_ghost_ping(
    ctx: &Arc<AgpContext>,
    cached_msg: CachedMessage,
    config: GuildConfig,
    title: &str,
    content: &str
) -> Result<(), Box<dyn Error + Send + Sync>> {

    let reply = message::get_reply(Arc::clone(&ctx), cached_msg.clone());
    let channel = match config.channel_id {
        Some(channel) => Id::<ChannelMarker>::new(channel as u64),
        None => cached_msg.channel_id(),
    };
    let time = Timestamp::from_secs(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs() as i64,
    )?;
    let color = if let Some(color) = config.color {
        if color == -1 {
            let mut rng = rand::thread_rng();
            let r = rng.gen_range(0..255);
            let g = rng.gen_range(0..255);
            let b = rng.gen_range(0..255);
            (r << 16) + (g << 8) + b
        } else {
            color
        }
    } else {
        16711712
    };
    let embed = AlertEmbed {
        author: cached_msg.author(),
        color: color as u32,
        content: content.to_string(),
        timestamp: time,
        reply,
        field_title: title.to_string(),
    };

    ctx.http
        .create_message(channel)
        .embeds(&[embed.build()])?
        .exec()
        .await?;

    Ok(())
}

pub async fn on_message_delete(
    ctx: Arc<AgpContext>,
    msg: MessageDelete,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cached_msg = ctx.cache.message(msg.id).unwrap();

    let author = ctx.cache.user(cached_msg.author()).unwrap();
    let guild_id = cached_msg.guild_id().unwrap();
    
    let query: Option<GuildConfig> = sqlx::query_as!(
        GuildConfig,
        r#"SELECT * FROM guild_configs WHERE guild_id = $1"#,
        guild_id.get() as i64
    )
    .fetch_optional(&ctx.db)
    .await?;

    let config = query.unwrap_or(GuildConfig {
        guild_id: 0,
        channel_id: None,
        everyone: false,
        mention_only: false,
        color: None,
    });

    if !author.bot {
        if cached_msg.mention_everyone() {
            let (title, content) =
                if cached_msg.content().len() > 2500 || config.mention_only {
                    ("Mentions:", "Message contained @everyone and/or @here ping")
                } else {
                    ("Message:", cached_msg.content())
                };
            handle_ghost_ping(&ctx, cached_msg.clone(), config, title, content).await?;
        } else if !(cached_msg.mentions().is_empty() && cached_msg.mention_roles().is_empty()) {
            let (title, content) = if cached_msg.content().len() > 2500 || config.mention_only {
                (
                    "Mentions:",
                    cached_msg
                        .mentions()
                        .iter()
                        .map(|m| format!("<@{}>", m.get()))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            } else {
                ("Message:", cached_msg.content().to_string())
            };
            handle_ghost_ping(&ctx, cached_msg.clone(), config, title, &content).await?;
        }
    }

    Ok(())
}

pub async fn on_message_update(
    ctx: Arc<AgpContext>,
    msg: MessageUpdate,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let original_msg = ctx.cache.message(msg.id);
    let guild_id = msg.guild_id.unwrap();
    
    let query: Option<GuildConfig> = sqlx::query_as!(
        GuildConfig,
        r#"SELECT * FROM guild_configs WHERE guild_id = $1"#,
        guild_id.get() as i64
    )
    .fetch_optional(&ctx.db)
    .await?;

    let config = query.unwrap_or(GuildConfig {
        guild_id: 0,
        channel_id: None,
        everyone: false,
        mention_only: false,
        color: None,
    });
    if let Some(original_msg) = original_msg {
        let author = ctx.cache.user(original_msg.author()).unwrap();
        if !author.bot {
            if original_msg.mention_everyone() {
                let (title, content) =
                    if msg.content.unwrap().len() > 2500 || config.mention_only {
                        ("Mentions:", String::from("Message contained @everyone and/or @here ping"))
                    } else {
                        ("Message:", msg.content.unwrap())
                    };
                handle_ghost_ping(&ctx, idk, config, title, &content);
            }
        }
    }

    Ok(())
}
