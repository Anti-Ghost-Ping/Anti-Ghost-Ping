use anyhow::Result;
use std::sync::Arc;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::structs::AgpContext;

pub async fn redirect(
    ctx: Arc<AgpContext>,
    channel: Option<i64>,
    guild_id: i64,
) -> Result<InteractionResponse> {
    let res: String = match channel {
        Some(id) => {
            match sqlx::query!("SELECT * FROM guild_configs WHERE guild_id = $1", guild_id)
                .fetch_one(&ctx.db)
                .await
            {
                Ok(_) => {
                    sqlx::query!(
                        "UPDATE guild_configs SET channel_id = $1 WHERE guild_id = $2",
                        id,
                        guild_id
                    )
                    .execute(&ctx.db)
                    .await?
                }
                Err(_) => {
                    sqlx::query!(
                        "INSERT INTO guild_configs(guild_id, channel_id) VALUES($1, $2)",
                        guild_id,
                        id
                    )
                    .execute(&ctx.db)
                    .await?
                }
            };
            format!("Set default ghost ping alert channel to: <#{}>", id)
        }
        None => {
            sqlx::query!(
                "UPDATE guild_configs SET channel_id = NULL WHERE guild_id = $1",
                guild_id
            )
            .execute(&ctx.db)
            .await?;

            String::from("Removed default ghost ping output channel.")
        }
    };

    let resp = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: Some(res),
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: None,
        }),
    };
    Ok(resp)
}
