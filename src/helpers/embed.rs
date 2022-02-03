use twilight_embed_builder::{EmbedBuilder, ImageSource, EmbedFieldBuilder};
use twilight_model::{channel::embed::Embed, gateway::payload::incoming::MessageUpdate, id::{marker::UserMarker, Id}, datetime::Timestamp};

pub struct AlertEmbed {
    pub author: Id<UserMarker>,
    pub content: String,
    pub timestamp: Timestamp,
    pub reply: Option<MessageUpdate>
}

impl AlertEmbed {
    pub fn create_embed(&self) -> Embed {
        let embed = EmbedBuilder::new()
                        .color(16711712)
                        .title("Ghost Ping Found!")
                        .thumbnail(ImageSource::url("https://ghostping.xyz/static/assets/bot_logo.png").unwrap())
                        .timestamp(self.timestamp.to_owned())
                        .field(EmbedFieldBuilder::new("Author:", format!("<@{}>", self.author.to_owned())).inline())
                        .field(EmbedFieldBuilder::new("Message:", self.content.to_owned()).inline());

        let embed = match self.reply.to_owned() {
            Some(msg) => embed.field(EmbedFieldBuilder::new("Replied:", msg.content.unwrap_or("N/A".to_string())).inline()),
            None => embed
        };

        embed.build().unwrap()
    }
}