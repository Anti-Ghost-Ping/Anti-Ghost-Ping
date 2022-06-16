use twilight_cache_inmemory::model::CachedMessage;
use twilight_model::{
    channel::embed::Embed,
    util::Timestamp,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, ImageSource};

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
