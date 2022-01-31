use std::sync::Arc;

use serenity::{
    builder::{CreateComponents, CreateInteractionResponse},
    client::bridge::gateway::ShardMessenger,
    futures::{lock::Mutex, StreamExt},
    http::Http,
    model::{
        id::ChannelId,
        interactions::{
            application_command::ApplicationCommandInteraction, message_component::ButtonStyle,
            InteractionResponseType,
        },
    },
};

use crate::{
    engine::{ScriptContext, ScriptDirective},
    Config, Engine, Scene,
};

pub struct Begin<'s> {
    config: Config,
    engine: Engine<'s>,
}

impl<'s> Begin<'s> {
    pub fn new(config_file: &str, script_file: &str, scene: &'s Scene<'s>) -> Self {
        Self {
            config: Config::from_file(config_file).expect("Unable to load config file"),
            engine: Engine::from_file(script_file, scene),
        }
    }

    pub fn delegate_component<'a>(
        &self,
        component: &'a mut CreateComponents,
    ) -> &'a mut CreateComponents {
        component.create_action_row(|row| match self.engine.current().unwrap() {
            ScriptContext::Dialogue(_) => row.create_button(|button| {
                button
                    .label("➡️")
                    .custom_id("right_page_select")
                    .style(ButtonStyle::Primary)
            }),
            ScriptContext::Directive(directive) => {
                if let ScriptDirective::Jump(jump) = directive {
                    if let Some((choice_a, choice_b)) = &jump.choices {
                        row.create_button(|button| {
                            button
                                .label(choice_a)
                                .custom_id("first_choice_select")
                                .style(ButtonStyle::Primary)
                        })
                        .create_button(|button| {
                            button
                                .label(choice_b)
                                .custom_id("second_choice_select")
                                .style(ButtonStyle::Primary)
                        })
                    } else {
                        row
                    }
                } else {
                    row
                }
            }
        })
    }

    fn delegate_interaction_response<'a>(
        &self,
        interaction: &'a mut CreateInteractionResponse,
        display_link: &str,
    ) -> &'a mut CreateInteractionResponse {
        interaction.interaction_response_data(|data| {
            data.components(|components| {
                let components = self.delegate_component(components);
                components
            })
            .create_embed(|embed| {
                embed
                    .title("Gary's VN Engine")
                    .description(&format!(
                        "You are currently playing {}",
                        self.config.fields.get("Game").unwrap().get("name").unwrap()
                    ))
                    .image(display_link)
            })
        })
    }

    pub async fn handle_interaction(
        &mut self,
        http: &Arc<Http>,
        interaction: ApplicationCommandInteraction,
        shard_messenger: &ShardMessenger,
    ) -> Result<(), serenity::Error> {
        let temp_channel = ChannelId(
            self.config
                .fields
                .get("Discord")
                .expect("Expected discord field in config file")
                .get("image_channel")
                .expect("Expected image_channel in [Discord] config file")
                .parse()
                .expect("image_channel must be an integer"),
        );
        let ctx = self.engine.next_until_renderable();

        if ctx.is_none() {
            return Ok(());
        }

        self.engine.render_to("resources/tmp.png");
        let message = temp_channel
            .send_files(http, vec!["resources/tmp.png"], |m| m)
            .await
            .expect("Cannot send file");

        interaction
            .create_interaction_response(http, |ir| {
                self.delegate_interaction_response(ir, &message.attachments[0].url)
            })
            .await
            .expect("Unable to create interaction");
        // let components = Mutex::new(Arc::new(self));
        let collector = interaction
            .get_interaction_response(http)
            .await?
            .await_component_interactions(shard_messenger)
            .await;

        let begin = &Arc::new(Mutex::new(self));

        collector
            .for_each(|interaction| async move {
                let mut begin = begin.lock().await;

                let choice = match interaction.data.custom_id.as_str() {
                    "right_page_select" => false,
                    "first_choice_select" => true,
                    "second_choice_select" => false,
                    id => panic!("Cannot handle interaction custom_id {}", id),
                };
                begin.engine.next(choice);
                if let Some(_) = begin.engine.next_until_renderable() {
                    begin.engine.render_to("resources/tmp.png");

                    let message = temp_channel
                        .send_files(http, vec!["resources/tmp.png"], |m| m)
                        .await
                        .expect("Cannot send file");
                    interaction
                        .create_interaction_response(http, |ir| {
                            begin
                                .delegate_interaction_response(ir, &message.attachments[0].url)
                                .kind(InteractionResponseType::UpdateMessage)
                        })
                        .await
                        .expect("Cannot update interaction");
                }
            })
            .await;

        Ok(())
    }
}
