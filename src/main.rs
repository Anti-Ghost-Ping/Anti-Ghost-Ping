use futures::StreamExt;
use std::{env, sync::Arc};
use tracing::info;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Cluster, Event, Intents};
use twilight_http::Client;

#[allow(unused_imports)]
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status},
    },
    id::Id,
};

mod commands;
mod events;
mod helpers;
mod structs;

use anyhow::Result;
use events::*;
use helpers::database::db_connect;
use structs::AgpContext;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILD_MESSAGES | Intents::GUILDS | Intents::MESSAGE_CONTENT;

    let (cluster, mut events) = Cluster::builder(token.to_owned(), intents)
        .presence(UpdatePresencePayload::new(
            vec![MinimalActivity {
                kind: ActivityType::Playing,
                name: "/ | https://ghostping.xyz".to_string(),
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
        .message_cache_size(25)
        .build();
    let db = db_connect(&env::var("DATABASE_URL")?).await?;

    sqlx::migrate!().run(&db).await?;

    let current_app = http
        .current_user_application()
        .exec()
        .await?
        .model()
        .await?;

    let interaction = http.interaction(current_app.id);

    interaction
        .set_global_commands(&helpers::commands())
        .exec()
        .await?;

    let agp_ctx = Arc::new(AgpContext {
        http,
        cache,
        db,
        app_id: current_app.id,
    });

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        tokio::spawn(handle_event(id, event, Arc::clone(&agp_ctx)));
    }

    Ok(())
}

async fn handle_event(shard_id: u64, event: Event, ctx: Arc<AgpContext>) -> Result<()> {
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
        Event::InteractionCreate(interaction) => {
            interaction::handle_interaction(Arc::clone(&ctx), interaction.0.to_owned()).await?;
        }
        _ => (),
    }
    ctx.cache.update(&event);

    Ok(())
}
