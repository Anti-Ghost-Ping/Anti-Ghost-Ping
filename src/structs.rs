use twilight_cache_inmemory::model::CachedMessage;
use twilight_model::{
    channel::message::{MessageFlags, MessageReference, MessageType},
    gateway::payload::incoming::MessageUpdate,
    id::{
        marker::{ChannelMarker, UserMarker},
        Id,
    },
};

#[derive(Debug)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub channel_id: Option<i64>,
    pub everyone: bool,
    pub mention_only: bool,
    pub color: Option<i32>,
}

#[derive(Clone)]
pub struct Message<'a> {
    pub content: String,
    pub channel_id: Id<ChannelMarker>,
    pub author: Id<UserMarker>,
    pub reference: Option<&'a MessageReference>,
    pub kind: MessageType,
    pub flags: Option<MessageFlags>,
}

impl<'a> Message<'a> {
    pub fn from_update(msg: MessageUpdate) -> Self {
        Message {
            content: msg.content.unwrap(),
            channel_id: msg.channel_id,
            author: msg.author.unwrap().id,
            reference: None,
            kind: msg.kind.unwrap(),
            flags: None,
        }
    }

    pub fn from_cache(msg: &'a CachedMessage) -> Self {
        Message {
            content: msg.content().to_string(),
            channel_id: msg.channel_id(),
            author: msg.author(),
            reference: msg.reference(),
            kind: msg.kind(),
            flags: msg.flags(),
        }
    }
}
