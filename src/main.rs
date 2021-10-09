use std::env;
use anyhow::anyhow;
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

macro_rules! how {
    ($e: expr, $($err: tt)*) => {
        match $e {
            Some(v) => Ok(v),
            None => Err(anyhow!($($err)*)),
        }
    }
}

impl Handler {
    async fn handle(
        &self,
        ctx: &Context,
        interaction: &Interaction
    ) -> anyhow::Result<()> {
        let guild = how!(&interaction.guild_id, "Couldn't get guild ID")?;
        let data = how!(&interaction.data, "Couldn't get interaction data")?;
        let options = &data.options;

        let user_id = match data.name.as_str() {
            "nick" => {
                let opt = how!(options.get(0), "Command requires argument #1")?;
                let opt = how!(&opt.resolved, "Couldn't resolve argument obj")?;
                match opt {
                    OptionValue::User(user, _) => user.id,
                    _ => return Err(anyhow!("Invalid type for argument #1")),
                }
            }
            "ramos" => {
                UserId::from(331194780916776961u64)
            }
            _ => unreachable!(),
        };

        let nick = how!(options.get(1), "Command requires argument #2")?;
        let nick = how!(&nick.resolved, "Couldn't resolve argument #2")?;
        let nick = match nick {
            OptionValue::String(nick) => nick,
            _ => return Err(anyhow!("Invalid type for argument #2")),
        };

        let member = guild.member(&ctx.http, user_id).await?;
        let tag = member.user.tag();
        let old_nick = &member.nick.unwrap_or(member.user.name);

        if user_id == 285601845957885952u64 {
            return Err(anyhow!("Can't fix what's already perfect 🙏"));
        } else if user_id == ctx.cache.current_user_id().await {
            guild.edit_nickname(&ctx.http, Some(nick)).await?;
        } else {
            guild
                .edit_member(&ctx.http, user_id, |mem| mem.nickname(nick))
                .await?;
        }

        let response = MessageBuilder::new()
            .push_mono_safe(tag)
            .push("  ")
            .push_mono_safe(old_nick)
            .push(" → ")
            .push_mono_safe(nick)
            .build();

        interaction.create_interaction_response(
            &ctx.http,
            |res| res.interaction_response_data(|msg| msg.content(&response)),
        ).await.ok();

        Ok(())
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(err) = self.handle(&ctx, &interaction).await {
            let emsg = format!("Error: {}", err);
            interaction.create_interaction_response(
                ctx.http,
                |res| res.interaction_response_data(|msg| msg.content(emsg)),
            ).await.ok();
        }
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
