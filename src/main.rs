use std::env;
use serenity::{
    prelude::*,
    utils::MessageBuilder,
    model::{
        prelude::*,
        gateway::Ready,
        interactions::{
            Interaction,
            ApplicationCommandInteractionDataOptionValue as OptionValue,
            ApplicationCommandOptionType as OptionType,
            ApplicationCommand as AppCmd,
        },
    },
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

macro_rules! cry {
    ($ctx: expr, $int: expr, $e: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                $int.create_interaction_response($ctx.http, |res| res
                        .interaction_response_data(|msg| msg
                            .content(format!("Error: {}", e))))
                    .await
                    .ok();
                return;
            }
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let guild = fry!(&interaction.guild_id);
        let data = fry!(&interaction.data);
        let options = &data.options;

        let user_id = match data.name.as_str() {
            "nick" => {
                let opt = fry!(options.get(0));
                let opt = fry!(&opt.resolved);
                match opt {
                    OptionValue::User(user, _) => user.id,
                    _ => return,
                }
            }
            "ramos" => {
                UserId::from(331194780916776961u64)
            }
            _ => unreachable!(),
        };

        let nick = fry!(options.get(1));
        let nick = fry!(&nick.resolved);
        let nick = match nick {
            OptionValue::String(nick) => nick,
            _ => return,
        };

        let member = guild.member(&ctx.http, user_id).await;
        let member = cry!(ctx.clone(), &interaction, member);
        let tag = member.user.tag();
        let old_nick = &member.nick.unwrap_or(member.user.name);

        if user_id == 285601845957885952u64 {
            let msg = "Can't fix what's already perfect 🙏".to_string();
            cry!(ctx.clone(), &interaction, Err(msg));
        }

        if user_id == ctx.cache.current_user_id().await {
            let edit_result = guild.edit_nickname(&ctx.http, Some(nick)).await;
            cry!(ctx.clone(), &interaction, edit_result);
        } else {
            let edit_result = guild
                .edit_member(&ctx.http, user_id, |mem| mem.nickname(nick))
                .await;
            cry!(ctx.clone(), &interaction, edit_result);
        }

        let response = MessageBuilder::new()
            .push_mono_safe(tag)
            .push("  ")
            .push_mono_safe(old_nick)
            .push(" → ")
            .push_mono_safe(nick)
            .build();

        interaction.create_interaction_response(&ctx.http, |res| res
                .interaction_response_data(|msg| msg
                    .content(&response)))
            .await
            .ok();
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        AppCmd::create_global_application_command(&ctx.http, |cmd| cmd
                .name("nick")
                .description("Set a user's nickname")
                .create_option(|opt| opt
                    .name("user")
                    .description("The user to set nickname")
                    .kind(OptionType::User)
                    .required(true))
                .create_option(|opt| opt
                    .name("nickname")
                    .description("The nickname to set")
                    .kind(OptionType::String)
                    .required(true)))
            .await.unwrap();

        AppCmd::create_global_application_command(&ctx.http, |cmd| cmd
                .name("ramos")
                .description("Set Ramos' nickname")
                .create_option(|opt| opt
                    .name("nickname")
                    .description("The nickname to set")
                    .kind(OptionType::String)
                    .required(true)))
            .await.unwrap();
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
