use anyhow::Result;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::SystemTime;
use twilight_model::id::marker::ChannelMarker;
use twilight_model::id::Id;
use twilight_model::{
    util::Timestamp,
    gateway::payload::incoming::{MessageDelete, MessageUpdate},
};

use crate::{
    helpers::{embed::AlertEmbed, message},
    structs::{GuildConfig, Message},
    AgpContext,
};

pub async fn handle_ghost_ping(
    ctx: &Arc<AgpContext>,
    msg: Message<'_>,
    config: GuildConfig,
    title: &str,
    content: &str,
) -> Result<()> {
    let reply = message::get_reply(Arc::clone(ctx), msg.clone());
    let channel = match config.channel_id {
        Some(channel) => Id::<ChannelMarker>::new(channel as u64),
        None => msg.channel_id,
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
        author: msg.author,
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

    ctx.increment_stats().await?;

    Ok(())
}

pub async fn on_message_delete(ctx: Arc<AgpContext>, msg: MessageDelete) -> Result<()> {
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

            let config = query.unwrap_or(GuildConfig {
                guild_id: 0,
                channel_id: None,
                everyone: false,
                mention_only: false,
                color: None,
            });

            let (title, content) = if cached_msg.content().len() > 2500 || config.mention_only {
                ("Mentions:", "Message contained @everyone and/or @here ping")
            } else {
                ("Message:", cached_msg.content())
            };
            handle_ghost_ping(
                &ctx,
                Message::from_cache(&cached_msg),
                config,
                title,
                content,
            )
            .await?;
        } else if !(cached_msg.mentions().is_empty() && cached_msg.mention_roles().is_empty()) {
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

            let (title, content) = if cached_msg.content().len() > 2500 || config.mention_only {
                (
                    "Mentions:",
                    cached_msg
                        .mentions()
                        .iter()
                        .map(|m| format!("<@{}>", m.get()))
                        .collect::<Vec<String>>()
                        .join(" ")
                        + &cached_msg
                            .mention_roles()
                            .iter()
                            .map(|m| format!("<@&{}>", m.get()))
                            .collect::<Vec<String>>()
                            .join(" "),
                )
            } else {
                ("Message:", cached_msg.content().to_string())
            };
            let converted_msg = Message::from_cache(&cached_msg);
            handle_ghost_ping(&ctx, converted_msg, config, title, &content).await?;
        }
    }

    Ok(())
}

pub async fn on_message_update(ctx: Arc<AgpContext>, msg: MessageUpdate) -> Result<()> {
    let original_msg = ctx.cache.message(msg.id);
    let guild_id = msg.guild_id.unwrap();

    if let Some(original_msg) = original_msg {
        let author = ctx.cache.user(original_msg.author()).unwrap();
        if !author.bot {
            let mut orig_mentions = HashSet::new();
            let mut orig_role_mentions = HashSet::new();
            let mut new_mentions = HashSet::new();
            let mut new_role_mentions = HashSet::new();
            for mention in original_msg.mentions().iter() {
                orig_mentions.insert(*mention);
            }
            for mention in msg.mentions.as_ref().unwrap() {
                new_mentions.insert(mention.id);
            }
            for mention in original_msg.mention_roles().iter() {
                orig_role_mentions.insert(*mention);
            }
            for mention in msg.mention_roles.as_ref().unwrap() {
                new_role_mentions.insert(*mention);
            }
            if original_msg.mention_everyone() && !msg.mention_everyone.unwrap() {
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

                let (title, content) =
                    if msg.content.as_ref().unwrap().len() > 2500 || config.mention_only {
                        (
                            "Mentions:",
                            String::from("Message contained @everyone and/or @here ping"),
                        )
                    } else {
                        ("Message:", original_msg.content().to_string())
                    };
                let converted_msg = Message::from_update(msg);
                handle_ghost_ping(&ctx, converted_msg, config, title, &content).await?;
            } else if !orig_mentions.is_subset(&new_mentions)
                || !orig_role_mentions.is_subset(&new_role_mentions)
            {
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

                let (title, content) =
                    if msg.content.as_ref().unwrap().len() > 2500 || config.mention_only {
                        (
                            "Mentions:",
                            original_msg
                                .mentions()
                                .iter()
                                .map(|m| format!("<@{}>", m.get()))
                                .collect::<Vec<String>>()
                                .join(" ")
                                + &original_msg
                                    .mention_roles()
                                    .iter()
                                    .map(|m| format!("<@&{}>", m.get()))
                                    .collect::<Vec<String>>()
                                    .join(" "),
                        )
                    } else {
                        ("Message:", original_msg.content().to_string())
                    };
                let converted_msg = Message::from_update(msg);
                handle_ghost_ping(&ctx, converted_msg, config, title, &content).await?;
            }
        }
    }

    Ok(())
}
