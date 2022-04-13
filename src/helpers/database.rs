use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn db_connect(
    connection_string: &str,
) -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(connection_string)
        .await?)
}
