use std::sync::atomic::Ordering;

use anyhow::Result;
use reqwest::header;
use reqwest::Client;

use crate::structs::AgpContext;
use crate::structs::PostData;
use crate::structs::Stats;

pub async fn create_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();

    let mut auth = header::HeaderValue::from_str(std::env::var("DISCORD_TOKEN")?.as_str())?;
    auth.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth);

    let client = Client::builder().default_headers(headers).build()?;

    return Ok(client);
}

impl AgpContext {
    pub async fn increment_stats(&self) -> Result<()> {
        let pings = self.stats.total_pings.load(Ordering::Relaxed);
        let guilds = self.stats.guild_count.load(Ordering::Relaxed);
        let post_data = PostData {
            guild_count: guilds,
            total_pings: pings,
        };

        self.reqwest
            .patch("https://ghostping.xyz/api/stats")
            .json(&post_data)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_stats(&self) -> Result<Stats> {
        let resp: Stats = self
            .reqwest
            .get("https://ghostping.xyz/api/stats")
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}
