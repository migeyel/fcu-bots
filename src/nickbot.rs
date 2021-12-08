use anyhow::{anyhow, Result};
use serenity::{
    prelude::*,
    utils::MessageBuilder,
    model::{
        prelude::*,
        gateway::Ready,
        interactions::{
            Interaction,
            ApplicationCommandInteractionDataOption as Option,
            ApplicationCommandInteractionDataOptionValue as OptionValue,
            ApplicationCommandOptionType as OptionType,
            ApplicationCommand as AppCmd,
        },
    },
};

macro_rules! how {
    ($e: expr, $($err: tt)*) => {
        match $e {
            Some(v) => Ok(v),
            None => Err(anyhow!($($err)*)),
        }
    }
}

pub struct Handler;

impl Handler {
    async fn set_nick(
        &self,
        ctx: &Context,
        int: &Interaction,
        user: UserId,
        nick: &str,
    ) -> Result<()> {
        let guild = how!(&int.guild_id, "Couldn't get guild ID")?;
        let member = guild.member(&ctx.http, user).await?;
        let tag = member.user.tag();
        let old_nick = &member.nick.unwrap_or(member.user.name);

        if user == 285601845957885952u64 {
            return Err(anyhow!("Can't fix what's already perfect 🙏"));
        } else if user == ctx.cache.current_user_id().await {
            guild.edit_nickname(&ctx.http, Some(&nick)).await?;
        } else {
            guild
                .edit_member(&ctx.http, user, |mem| mem.nickname(nick))
                .await?;
        }

        let response = MessageBuilder::new()
            .push_mono_safe(tag)
            .push("  ")
            .push_mono_safe(old_nick)
            .push(" → ")
            .push_mono_safe(nick)
            .build();

        int.create_interaction_response(
            &ctx.http,
            |res| res.interaction_response_data(|msg| msg.content(&response)),
        ).await?;

        Ok(())
    }

    async fn cmd_nick(
        &self,
        ctx: &Context,
        int: &Interaction,
        opts: &[Option]
    ) -> Result<()> {
        let user = match opts.get(0) {
            Some(Option {
                resolved: Some(OptionValue::User(user, _)),
                ..
            }) => user.id,
            Some(_) => return Err(anyhow!("Invalid argument #1")),
            None => return Err(anyhow!("Command requires argument #1")),
        };

        let nick = match opts.get(1) {
            Some(Option {
                resolved: Some(OptionValue::String(nick)),
                ..
            }) => nick,
            Some(_) => return Err(anyhow!("Invalid argument #2")),
            None => return Err(anyhow!("Command requires argument #2")),
        };

        self.set_nick(ctx, int, user, nick).await
    }

    async fn cmd_ramos(
        &self,
        ctx: &Context,
        int: &Interaction,
        opts: &[Option],
    ) -> Result<()> {
        let nick = match opts.get(0) {
            Some(Option {
                resolved: Some(OptionValue::String(nick)),
                ..
            }) => nick,
            Some(_) => return Err(anyhow!("Invalid argument #1")),
            None => return Err(anyhow!("Command requires argument #1")),
        };

        self.set_nick(ctx, int, 331194780916776961u64.into(), nick).await
    }

    async fn handle_fallible(
        &self,
        ctx: &Context,
        int: &Interaction,
    ) -> Result<()> {
        let data = how!(&int.data, "Couldn't get interaction data")?;
        match data.name.as_str() {
            "nick" => self.cmd_nick(ctx, int, &data.options[..]).await,
            "ramos" => self.cmd_ramos(ctx, int, &data.options[..]).await,
            _ => unreachable!(),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        if let Err(err) = self.handle_fallible(&ctx, &int).await {
            let emsg = format!("Error: {}", err);
            int.create_interaction_response(
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
