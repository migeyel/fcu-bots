mod nickbot;
use nickbot::Handler;

use std::env;
use anyhow::Result;
use serenity::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let application_id: u64 = env::var("APPLICATION_ID")?.parse()?;
    let token = env::var("DISCORD_TOKEN")?;

    let handler = Handler;

    let mut client = Client::builder(token)
        .event_handler(handler)
        .application_id(application_id)
        .await?;

    client.start().await?;

    Ok(())
}
