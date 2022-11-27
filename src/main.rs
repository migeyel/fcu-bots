mod nickbot;
use nickbot::Handler;

use std::env;
use anyhow::Result;
use serenity::Client;
use serenity::model::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let nickbot_appid: u64 = env::var("APPLICATION_ID")?.parse()?;
    let nickbot_guildid = GuildId(env::var("NICKBOT_GUILDID")?.parse()?);
    let nickbot_fcid = RoleId(env::var("NICKBOT_ROLEID")?.parse()?);
    let nickbot_token = env::var("DISCORD_TOKEN")?;

    let handler = Handler::new(nickbot_guildid, nickbot_fcid);
    let mut client = Client::builder(nickbot_token)
        .event_handler(handler)
        .application_id(nickbot_appid)
        .await?;

    client.start().await?;

    Ok(())
}
