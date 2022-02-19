use std::{error::Error, sync::Arc};
use tracing::info;
use twilight_model::gateway::payload::incoming::{MessageDelete, MessageUpdate};

use crate::{helpers::embed::AlertEmbed, AgpContext};

pub async fn on_message_delete(
    ctx: Arc<AgpContext>,
    msg: MessageDelete,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cached_msg = ctx.cache.message(msg.id);

    if let Some(cached_msg) = cached_msg {
        info!("Message Delete | {}", cached_msg.content());

        if !cached_msg.mentions().is_empty() {
            let embed = AlertEmbed {
                author: cached_msg.author(),
                content: cached_msg.content().to_string(),
                timestamp: cached_msg.timestamp(),
                reply: None,
            };

            ctx.http
                .create_message(msg.channel_id)
                .embeds(&[embed.create_embed()])?
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

    if let Some(original_msg) = original_msg {
        info!(
            "Message Update | {} > {}",
            original_msg.content(),
            msg.content.unwrap_or_else(|| "".to_string())
        );
    }

    Ok(())
}
