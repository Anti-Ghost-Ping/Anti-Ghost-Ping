use dashmap::DashMap;
use serenity::{
    model::id::GuildId,
    prelude::TypeMapKey
};
use std::sync::Arc;

pub struct Prefixes;

impl TypeMapKey for Prefixes {
    type Value = Arc<DashMap<GuildId, String>>;
}