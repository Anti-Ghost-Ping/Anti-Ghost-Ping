use futures::StreamExt;
use std::{env, sync::Arc};
use tracing::info;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
use twilight_http::Client;
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{ActivityType, MinimalActivity, Status},
};

mod context;
mod events;
mod helpers;
mod structs;

use context::AgpContext;
use events::{guild, message};
use helpers::database::db_connect;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::MESSAGE_CONTENT;
    let scheme = ShardScheme::Auto;

    let (cluster, mut events) = Cluster::builder(token.to_owned(), intents)
        .shard_scheme(scheme)
        .presence(UpdatePresencePayload::new(
            vec![MinimalActivity {
                kind: ActivityType::Playing,
                name: "/help | https://ghostping.xyz".to_string(),
                url: None,
            }
            .into()],
            false,
            None,
            Status::Online,
        )?)
        .build()
        .await?;

    let cluster = Arc::new(cluster);
    let cluster_spawn = Arc::clone(&cluster);

    let http = Client::new(token);
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE | ResourceType::USER)
        .build();
    let db = db_connect(&env::var("DATABASE_URL")?).await?;

    let agp_ctx = Arc::new(AgpContext { http, cache, db });

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        tokio::spawn(handle_event(id, event, Arc::clone(&agp_ctx)));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    ctx: Arc<AgpContext>,
) -> Result<()> {
    match &event {
        Event::Ready(_) => {
            info!("Shard {} is ready!", shard_id)
        }
        Event::MessageDelete(msg) => {
            message::on_message_delete(Arc::clone(&ctx), msg.to_owned()).await?;
        }
        Event::MessageUpdate(msg) => {
            message::on_message_update(Arc::clone(&ctx), *msg.to_owned()).await?;
        }
        Event::GuildDelete(guild) => {
            guild::on_guild_leave(Arc::clone(&ctx), guild).await?;
        }
        _ => (),
    }
    ctx.cache.update(&event);

    Ok(())
}
