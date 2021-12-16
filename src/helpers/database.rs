use serenity::{framework::standard::CommandResult, model::id::GuildId};
use sqlx::postgres::{PgPool, PgPoolOptions};
use dashmap::DashMap;

pub async fn connect_db(connection_string: String) -> CommandResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await?;

    Ok(pool)
}

pub async fn fetch_prefixes(pool: &PgPool) -> CommandResult<DashMap<GuildId, String>> {
    let prefixes: DashMap<GuildId, String> = DashMap::new();

    let cursor = sqlx::query!("SELECT guild_id, prefix FROM main")
        .fetch_all(pool)
        .await?;

    for x in cursor {
        if let Some(prefix) = x.prefix {
            prefixes.insert(GuildId::from(x.guild_id as u64), prefix);
        }
    }

    Ok(prefixes)
}