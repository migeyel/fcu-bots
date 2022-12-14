use anyhow::{anyhow, Result};
use serenity::{
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommandInteraction as ACInt,
                ApplicationCommandInteractionDataOption as Option,
                ApplicationCommandInteractionDataOptionValue as OptionValue,
                ApplicationCommandOptionType as OptionType,
            },
            Interaction,
        },
        prelude::*,
    },
    prelude::*,
    utils::MessageBuilder,
};

macro_rules! how {
    ($e: expr, $($err: tt)*) => {
        match $e {
            Some(v) => Ok(v),
            None => Err(anyhow!($($err)*)),
        }
    }
}

pub struct Handler {
    guild_id: GuildId,
    fanclub_role: RoleId,
}

impl Handler {
    pub fn new(guild_id: GuildId, fanclub_role: RoleId) -> Self {
        Self {
            guild_id,
            fanclub_role,
        }
    }

    async fn set_nick(
        &self,
        ctx: &Context,
        int: &ACInt,
        user: UserId,
        nick: &str,
    ) -> Result<()> {
        let guild = how!(&int.guild_id, "Couldn't get guild ID")?;
        let member = guild.member(&ctx.http, user).await?;
        let tag = member.user.tag();
        let old_nick = &member.nick.unwrap_or(member.user.name);

        if user == UserId(285601845957885952) {
            return Err(anyhow!("Can't fix what's already perfect 🙏"));
        } else if user == ctx.cache.current_user_id().await {
            guild.edit_nickname(&ctx.http, Some(nick)).await?;
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

        self.response(ctx, int, &response).await
    }

    async fn cmd_nick(&self, ctx: &Context, int: &ACInt) -> Result<()> {
        let user = match int.data.options.get(0) {
            Some(Option {
                resolved: Some(OptionValue::User(user, _)),
                ..
            }) => user.id,
            Some(_) => return Err(anyhow!("Invalid argument #1")),
            None => return Err(anyhow!("Command requires argument #1")),
        };

        let nick = match int.data.options.get(1) {
            Some(Option {
                resolved: Some(OptionValue::String(nick)),
                ..
            }) => nick,
            Some(_) => return Err(anyhow!("Invalid argument #2")),
            None => return Err(anyhow!("Command requires argument #2")),
        };

        self.set_nick(ctx, int, user, nick).await
    }

    async fn cmd_ramos(&self, ctx: &Context, int: &ACInt) -> Result<()> {
        let nick = match int.data.options.get(0) {
            Some(Option {
                resolved: Some(OptionValue::String(nick)),
                ..
            }) => nick,
            Some(_) => return Err(anyhow!("Invalid argument #1")),
            None => return Err(anyhow!("Command requires argument #1")),
        };

        self.set_nick(ctx, int, UserId(331194780916776961), nick)
            .await
    }

    async fn cmd_join(&self, ctx: &Context, int: &ACInt) -> Result<()> {
        let mut member = match int.member.clone() {
            Some(member) => member,
            None => return Err(anyhow!("Not on a guild")),
        };

        member.add_role(&ctx.http, self.fanclub_role).await?;

        let response = MessageBuilder::new()
            .push_mono_safe(&member.nick.unwrap_or(member.user.name))
            .push(" joined the fan club.")
            .build();

        int.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|m| m.content(&response))
        })
        .await?;

        Ok(())
    }

    async fn cmd_leave(&self, ctx: &Context, int: &ACInt) -> Result<()> {
        let mut member = match int.member.clone() {
            Some(member) => member,
            None => return Err(anyhow!("Not on a guild")),
        };

        member.remove_role(&ctx.http, self.fanclub_role).await?;

        let response = MessageBuilder::new()
            .push_mono_safe(&member.nick.unwrap_or(member.user.name))
            .push(" left the fan club.")
            .build();

        int.create_interaction_response(&ctx.http, |r| {
            r.interaction_response_data(|m| m.content(&response))
        })
        .await?;

        Ok(())
    }

    async fn handle_fallible(&self, ctx: &Context, int: &ACInt) {
        let result = match int.data.name.as_str() {
            "nick" => self.cmd_nick(ctx, int).await,
            "ramos" => self.cmd_ramos(ctx, int).await,
            "join" => self.cmd_join(ctx, int).await,
            "leave" => self.cmd_leave(ctx, int).await,
            _ => unreachable!(),
        };

        if let Err(err) = result {
            self.response(ctx, int, &format!("Error: {}", err))
                .await
                .ok();
        }
    }

    async fn response(
        &self,
        ctx: &Context,
        int: &ACInt,
        response: &str,
    ) -> Result<()> {
        int.create_interaction_response(&ctx.http, |res| {
            res.interaction_response_data(|msg| msg.content(response))
        })
        .await
        .map_err(|e| e.into())
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        if let Interaction::ApplicationCommand(int) = int {
            self.handle_fallible(&ctx, &int).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        self.guild_id
            .create_application_command(&ctx.http, |cmd| {
                cmd.name("nick")
                    .description("Set a user's nickname")
                    .create_option(|opt| {
                        opt.name("user")
                            .description("The user to set nickname")
                            .kind(OptionType::User)
                            .required(true)
                    })
                    .create_option(|opt| {
                        opt.name("nickname")
                            .description("The nickname to set")
                            .kind(OptionType::String)
                            .required(true)
                    })
            })
            .await
            .unwrap();

        self.guild_id
            .create_application_command(&ctx.http, |cmd| {
                cmd.name("ramos")
                    .description("Set Ramos' nickname")
                    .create_option(|opt| {
                        opt.name("nickname")
                            .description("The nickname to set")
                            .kind(OptionType::String)
                            .required(true)
                    })
            })
            .await
            .unwrap();

        self.guild_id
            .create_application_command(&ctx.http, |cmd| {
                cmd.name("join").description("Joins the fanclub.")
            })
            .await
            .unwrap();

        self.guild_id
            .create_application_command(&ctx.http, |cmd| {
                cmd.name("leave").description("Leaves the fanclub.")
            })
            .await
            .unwrap();
    }
}
