use twilight_cache_inmemory::model::CachedMessage;
use twilight_model::{
    channel::embed::Embed,
    id::{marker::UserMarker, Id},
    util::Timestamp,
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, ImageSource};

use super::color::gen_color;

pub struct AlertEmbed {
    pub author: Id<UserMarker>,
    pub color: u32,
    pub content: String,
    pub field_title: String,
    pub reply: Option<CachedMessage>,
    pub timestamp: Timestamp,
}

impl AlertEmbed {
    pub fn build(&self) -> Embed {
        let embed = EmbedBuilder::new()
            .color(self.color)
            .title("Ghost Ping Found!")
            .thumbnail(
                ImageSource::url("https://ghostping.xyz/static/assets/bot_logo.png").unwrap(),
            )
            .timestamp(self.timestamp.to_owned())
            .field(EmbedFieldBuilder::new("Author:", format!("<@{}>", self.author)).inline())
            .field(
                EmbedFieldBuilder::new(self.field_title.to_owned(), self.content.to_owned())
                    .inline(),
            );

        embed.build()
    }
}

pub fn create_color_preview(color: i32) -> Embed {
    let mut embed_color = color;
    if color == -1 {
        embed_color = gen_color();
    }

    let embed = EmbedBuilder::new()
        .color(embed_color as u32)
        .title("Example Ghost Ping Alert")
        .thumbnail(ImageSource::url("https://ghostping.xyz/static/assets/bot_logo.png").unwrap());

    embed.build()
}

pub fn create_error_embed(msg: &str, desc: &str) -> Embed {
    let embed = EmbedBuilder::new()
        .color(13057565)
        .title(msg)
        .description(desc)
        .thumbnail(ImageSource::url("https://ghostping.xyz/static/assets/bot_logo.png").unwrap());

    embed.build()
}
