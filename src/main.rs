use std::env;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            Interaction,
            ApplicationCommandInteractionDataOptionValue as OptionValue,
            // ApplicationCommandOptionType as OptionType,
            // ApplicationCommand as AppCmd,
        },
    },
    prelude::*,
};

struct Handler;

macro_rules! fry {
    ($e: expr) => {
        match $e {
            Some(v) => v,
            None => return,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let guild = fry!(&interaction.guild_id);
        let data = fry!(&interaction.data);
        let options = &data.options;
        let user = fry!(options.get(0));
        let user = fry!(&user.resolved);
        let user = match user {
            OptionValue::User(user, _) => user,
            _ => return,
        };

        let nick = fry!(options.get(1));
        let nick = fry!(&nick.resolved);
        let nick = match nick {
            OptionValue::String(nick) => nick,
            _ => return,
        };

        let nick_res = guild.edit_member(&ctx.http, user, |mem| mem
                .nickname(nick))
            .await;

        let response = match nick_res {
            Ok(_) => String::from("Bodia!"),
            Err(e) => e.to_string(),
        };
        
        interaction.create_interaction_response(&ctx.http, |res| res
                .interaction_response_data(|msg| msg
                    .content(&response)))
            .await
            .ok();
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // AppCmd::create_global_application_command(&ctx.http, |cmd| cmd
        //         .name("nick")
        //         .description("Set a user's nickname")
        //         .create_option(|opt| opt
        //             .name("user")
        //             .description("The user to set nickname")
        //             .kind(OptionType::User)
        //             .required(true))
        //         .create_option(|opt| opt
        //             .name("nickname")
        //             .description("The nickname to set")
        //             .kind(OptionType::String)
        //             .required(true)))
        //     .await.unwrap();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application_id: u64 = env::var("APPLICATION_ID")?.parse()?;
    let token = env::var("DISCORD_TOKEN")?;

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await?;

    client.start().await?;

    Ok(())
}
