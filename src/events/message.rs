use twilight_model::{gateway::payload::incoming::MessageDelete};
use twilight_embed_builder::{EmbedBuilder, ImageSource, EmbedFieldBuilder};
use tracing::info;
use std::{sync::Arc, error::Error};

use crate::AgpContext;

pub async fn on_message_delete(ctx: Arc<AgpContext>, msg: MessageDelete) -> Result<(), Box<dyn Error + Send + Sync>> {
    let cached_msg = ctx.cache.message(msg.id);
    if let Some(cached_msg) = cached_msg {
        info!("Message Delete > {:#?}", cached_msg);
        if !cached_msg.mentions().is_empty() {
            ctx.http.create_message(msg.channel_id)
            .embeds(&[
                EmbedBuilder::new()
                    .color(16711712)
                    .title("Ghost Ping Found!")
                    .thumbnail(ImageSource::url("https://ghostping.xyz/static/assets/bot_logo.png")?)
                    .timestamp(cached_msg.timestamp())
                    .field(EmbedFieldBuilder::new("Author:", format!("<@{}>", cached_msg.author())).inline())
                    .field(EmbedFieldBuilder::new("Message:", cached_msg.content()).inline())
                    .build()?])?
            .exec()
            .await?;
        }
    }
    

    Ok(())
}