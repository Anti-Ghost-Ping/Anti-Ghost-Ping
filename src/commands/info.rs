use anyhow::Result;
use std::sync::Arc;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::structs::AgpContext;

pub async fn info(ctx: Arc<AgpContext>) -> Result<InteractionResponse> {
    let resp = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: Some("hi".to_string()),
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: None,
        }),
    };
    Ok(resp)
}
