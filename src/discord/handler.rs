use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, id::GuildId, interactions::Interaction},
};

use crate::Scene;

use super::display::Begin;

pub struct Handler<'a> {
    pub config_path: String,
    pub guild_id: u64,
    pub script_path: String,
    pub scene: Scene<'a>,
}

#[async_trait]
impl<'a> EventHandler for Handler<'a> {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "begin" => Begin::new(
                    self.config_path.as_str(),
                    self.script_path.as_str(),
                    &self.scene,
                )
                .handle_interaction(&ctx.http, command, &ctx.shard)
                .await
                .expect("Cannot run begin command"),
                _ => panic!("Unable to handle command!"),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild = GuildId(self.guild_id);
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
