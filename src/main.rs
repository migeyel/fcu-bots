use std::env;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            Interaction,
            InteractionResponseType,
            ApplicationCommand,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction.create_interaction_response(
            &ctx.http,
            |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Received event!")
                    })
            })
            .await.ok();
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let interactions = ApplicationCommand::get_global_application_commands(&ctx.http).await;
        println!("I have the following global slash command(s): {:?}", interactions);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    client.start().await?;

    Ok(())
}
