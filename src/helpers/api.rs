use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::header;

use crate::structs::AgpContext;

pub async fn create_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();

    let mut auth = header::HeaderValue::from_str(std::env::var("DISCORD_TOKEN")?.as_str())?;
    auth.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth);

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    return Ok(client);
}

impl AgpContext {
    pub async fn increment_stats(&self) -> Result<()> {
        

        Ok(())
    }
    
    pub async fn get_stats(&self) -> Result<()> {
        Ok(())
    }   
}