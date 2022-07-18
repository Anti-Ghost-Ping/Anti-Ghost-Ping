use anyhow::Result;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};

pub async fn db_connect() -> Result<PgPool> {
    let mut options = PgConnectOptions::new()
        .host("database")
        .port(5432)
        .username("postgres")
        .password("password")
        .database("postgres");

    options.disable_statement_logging();

    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?)
}
