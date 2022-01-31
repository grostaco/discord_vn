use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, id::GuildId, interactions::Interaction},
};

use crate::Scene;

use super::display::Begin;

pub struct Handler<'a> {
    pub scene: Scene<'a>,
}

#[async_trait]
impl<'a> EventHandler for Handler<'a> {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "begin" => Begin::new("resources/config.conf", "resources/script.txt", &self.scene)
                    .handle_interaction(&ctx.http, command, &ctx.shard)
                    .await
                    .expect("Cannot run begin command"),
                _ => panic!("Unable to handle command!"),
            }
        }
    }

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
