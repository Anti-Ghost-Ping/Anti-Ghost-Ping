use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use twilight_cache_inmemory::InMemoryCache;
use twilight_model::{
    channel::message::{MessageFlags, MessageReference, MessageType},
    id::{
        marker::{ApplicationMarker, ChannelMarker, UserMarker},
        Id,
    },
};

pub struct AgpContext {
    pub http: twilight_http::Client,
    pub cache: InMemoryCache,
    pub db: PgPool,
    pub app_id: Id<ApplicationMarker>,
}

#[derive(Serialize)]
pub struct PostData {
    pub guild_count: u32,
    pub total_pings: u32,
}

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

#[derive(Deserialize)]
pub struct Stats {
    pub guild_count: u32,
    pub ppm: u32,
    pub total_pings: u32,
}
