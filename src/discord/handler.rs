use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, id::GuildId, interactions::Interaction},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {}

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild = GuildId(
            env::var("GUILD_ID")
                .expect("GUILD_ID not set")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        let guild_command = GuildId::set_application_commands(&guild, &ctx.http, |commands| {
            commands.create_application_command(|command| command.name("begin").description(":>"))
        })
        .await
        .expect("Unable to add guild commands");

        println!(
            "The bot has registered the following guild slash commands {:#?}",
            guild_command,
        );
    }
}
