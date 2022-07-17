use anyhow::Result;
use std::sync::Arc;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::structs::AgpContext;

pub async fn etoggle(
    ctx: Arc<AgpContext>,
    choice: bool,
    guild_id: i64,
) -> Result<InteractionResponse> {
    match sqlx::query!("SELECT * FROM guild_configs WHERE guild_id = $1", guild_id)
        .fetch_one(&ctx.db)
        .await
    {
        Ok(_) => {
            sqlx::query!(
                "UPDATE guild_configs SET everyone = $1 WHERE guild_id = $2",
                choice,
                guild_id
            )
            .execute(&ctx.db)
            .await?
        }
        Err(_) => {
            sqlx::query!(
                "INSERT INTO guild_configs(guild_id, everyone) VALUES($1, $2)",
                guild_id,
                choice
            )
            .execute(&ctx.db)
            .await?
        }
    };

    let res = match choice {
        true => String::from("Enabled everyone and here ghost ping detection."),
        false => String::from("Disabled everyone and here ghost ping detection."),
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
