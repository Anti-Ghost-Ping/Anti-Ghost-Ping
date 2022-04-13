use sqlx::PgPool;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;

pub struct AgpContext {
    pub http: Client,
    pub cache: InMemoryCache,
    pub db: PgPool,
}
