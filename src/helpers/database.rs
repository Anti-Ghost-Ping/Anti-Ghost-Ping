use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::Result;

pub async fn db_connect(
    connection_string: &str,
) -> Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await?)
}
