mod structures;
mod helpers;

use std::{env};
use serenity::{
    framework::{StandardFramework},
    prelude::*,
    async_trait,
    model::{gateway::Ready}
};
use structures::bot_data::Prefixes;
use tracing::{error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, _: Ready) {
        info!("Connected");
    }

}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    
    tracing_subscriber::fmt::init();

    let token = env::var("BOT_TOKEN").expect("Could not find BOT_TOKEN in the environment");

    let framework = StandardFramework::new().configure(|c| c.prefix("."));

    let prefix_map = todo!();

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<Prefixes>(prefix_map);
    }

    if let Err(why) = client.start().await {
        error!("Error Starting Client: {:?}", why);
    }
}