use std::sync::Arc;

use twilight_cache_inmemory::model::CachedMessage;
use twilight_model::channel::message::{MessageFlags, MessageType};

use crate::context::AgpContext;

pub fn get_reply(ctx: Arc<AgpContext>, message: CachedMessage) -> Option<CachedMessage> {
    if let Some(reply) = message.reference() {
        if check_user_message(&message) && !check_message_crosspost(&message) {
            let cached_reply = ctx.cache.message(reply.message_id.unwrap());
            if let Some(reply) = cached_reply {
                return Some(reply.clone());
            } else {
                return None;
            }
        }
    }
    None
}

fn check_user_message(message: &CachedMessage) -> bool {
    match message.kind() {
        MessageType::Regular | MessageType::Reply | MessageType::ThreadStarterMessage => true,
        _ => false,
    }
}

fn check_message_crosspost(message: &CachedMessage) -> bool {
    if let Some(flags) = message.flags() {
        if flags.contains(MessageFlags::IS_CROSSPOST) {
            return true;
        }
    }
    false
}
