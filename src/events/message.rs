use rand::Rng;
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

pub async fn on_message_delete(
    ctx: Arc<AgpContext>,
    msg: MessageDelete,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cached_msg = ctx.cache.message(msg.id).unwrap();
    let author = ctx.cache.user(cached_msg.author()).unwrap();
    let guild_id = cached_msg.guild_id().unwrap();

    if !author.bot {
        if cached_msg.mention_everyone() {
            let query: Option<GuildConfig> = sqlx::query_as!(
                GuildConfig,
                r#"SELECT * FROM guild_configs WHERE guild_id = $1"#,
                guild_id.get() as i64
            )
            .fetch_optional(&ctx.db)
            .await?;

            if let Some(config) = query {
                if config.everyone {
                    let (title, content) =
                        if cached_msg.content().len() > 2500 || config.mention_only {
                            ("Mentions:", "Message contained @everyone and/or @here ping")
                        } else {
                            ("Message:", cached_msg.content())
                        };
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
                }
            }
        } else if !(cached_msg.mentions().is_empty() && cached_msg.mention_roles().is_empty()) {
            let query: Option<GuildConfig> = sqlx::query_as!(
                GuildConfig,
                r#"SELECT * FROM guild_configs WHERE guild_id = $1"#,
                guild_id.get() as i64
            )
            .fetch_optional(&ctx.db)
            .await?;

            let config = query.unwrap_or(GuildConfig {
                // doesn't matter
                guild_id: 0,
                channel_id: None,
                // doesn't matter
                everyone: false,
                mention_only: false,
                color: None,
            });
            let (title, content) = if cached_msg.content().len() > 2500 || config.mention_only {
                (
                    "Mentions:",
                    cached_msg
                        .mentions()
                        .iter()
                        .map(|m| format!("<@{}>", m.get()))
                        .collect::<Vec<String>>()
                        .join(" "),
                )
            } else {
                ("Message:", cached_msg.content().to_string())
            };
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
        }
    }

    Ok(())
}

pub async fn on_message_update(
    ctx: Arc<AgpContext>,
    msg: MessageUpdate,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let original_msg = ctx.cache.message(msg.id);
    let new_msg = msg.content.unwrap_or_else(|| " ".to_string());

    if let Some(original_msg) = original_msg {
        info!("Message Update | {} > {}", original_msg.content(), new_msg);
    }

    Ok(())
}
