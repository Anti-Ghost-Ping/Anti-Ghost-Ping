use anyhow::Result;
use std::sync::Arc;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::{
    helpers::embed::{create_color_preview, create_error_embed},
    structs::AgpContext,
};

pub async fn setcolor(
    ctx: Arc<AgpContext>,
    color: Result<i32>,
    guild_id: i64,
) -> Result<InteractionResponse> {
    let embed = match color {
        Ok(int) => {
            match sqlx::query!("SELECT * FROM guild_configs WHERE guild_id = $1", guild_id).fetch_one(&ctx.db).await {
                Ok(_) => {
                    sqlx::query!(
                        "UPDATE guild_configs SET color = $1 WHERE guild_id = $2",
                        int,
                        guild_id
                    )
                    .execute(&ctx.db)
                    .await?
                }
                Err(_) => {
                    sqlx::query!(
                        "INSERT INTO guild_configs(guild_id, color) VALUES($1, $2)",
                        guild_id,
                        int
                    )
                    .execute(&ctx.db)
                    .await?
                }
            };
            create_color_preview(int)
        }
        Err(_) => {
            create_error_embed("An Error occurred parsing your color.", "Make sure your color is a valid color or hex code prefixed with a `#`\nExample: `#2a4f87`")
        }
    };

    let resp = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: None,
            custom_id: None,
            embeds: Some(vec![embed]),
            flags: None,
            title: None,
            tts: None,
        }),
    };
    Ok(resp)
}
