use std::sync::Arc;

use serenity::{
    builder::{CreateComponents, CreateInteractionResponse},
    client::{bridge::gateway::ShardMessenger, Context},
    futures::StreamExt,
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
    engine::{ParseError, ScriptContext, ScriptDirective},
    Config, Engine, Scene,
};

use super::voice::play_url;

struct PlayInfo(u64, u64, String);
pub struct Begin<'s> {
    config: Config,
    engine: Engine<'s>,
}

impl<'s> Begin<'s> {
    pub fn new(
        config_file: &str,
        script_file: &str,
        scene: &'s Scene<'s>,
    ) -> Result<Self, ParseError> {
        Ok(Self {
            config: Config::from_file(config_file)?,
            engine: Engine::from_file(script_file, scene)?,
        })
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
                    .title(&format!(
                        "You are currently playing {}",
                        self.config.fields.get("Game").unwrap().get("name").unwrap()
                    ))
                    .description(&match self.engine.current().unwrap() {
                        ScriptContext::Dialogue(dialogue) => format!(
                            "{}: {}",
                            dialogue.character_name,
                            dialogue.dialogues.join(" ")
                        ),
                        ScriptContext::Directive(directive) => {
                            if let ScriptDirective::Jump(jump) = directive {
                                format!(
                                    "You are presented with two choices:\n[1] {}\n[2] {}",
                                    jump.choices.as_ref().unwrap().0,
                                    jump.choices.as_ref().unwrap().1
                                )
                            } else {
                                panic!("Unexpected directive found during discord rendering")
                            }
                        }
                    })
                    .image(display_link)
            })
        })
    }

    pub async fn handle_interaction(
        &mut self,
        http: &Arc<Http>,
        context: &Context,
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

        let renderable = |ctx: &ScriptContext| match ctx {
            ScriptContext::Dialogue(_) => true,
            ScriptContext::Directive(directive) => match directive {
                ScriptDirective::Jump(jump) if jump.choices.is_some() => true,
                ScriptDirective::Custom(custom) if custom.name == "play" => true,
                _ => false,
            },
        };
        let mut play_info: Option<PlayInfo> = None;

        while let Some(ctx) = self.engine.next_until(renderable).unwrap() {
            if let ScriptContext::Directive(ScriptDirective::Custom(custom)) = ctx {
                let guild_id = custom
                    .args
                    .get(0)
                    .expect("Guild id expected for play")
                    .parse()
                    .expect("Guild id must be an integer");
                let channel_id = custom
                    .args
                    .get(1)
                    .expect("Channel id expected for play")
                    .parse()
                    .expect("Channel id must be an integer");

                play_info = Some(PlayInfo(
                    guild_id,
                    channel_id,
                    custom.args.get(2).expect("URL not provided").to_string(),
                ));
                self.engine.next(false).unwrap();
            } else {
                break;
            }
        }
        if self.engine.current().is_none() {
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

        if let Some(play_info) = play_info.take() {
            play_url(context, play_info.0, play_info.1, &play_info.2)
                .await
                .expect("Cannot play URL");
        }
        let mut collector = interaction
            .get_interaction_response(http)
            .await?
            .await_component_interactions(shard_messenger)
            .await;

        while let Some(mci) = collector.next().await {
            let choice = match mci.data.custom_id.as_str() {
                "right_page_select" => false,
                "first_choice_select" => true,
                "second_choice_select" => false,
                id => panic!("Cannot handle interaction custom_id {}", id),
            };
            self.engine.next(choice).unwrap();
            if let Some(_ctx) = self.engine.next_until_renderable().unwrap() {
                while let Some(ctx) = self.engine.next_until(renderable).unwrap() {
                    if let ScriptContext::Directive(ScriptDirective::Custom(custom)) = ctx {
                        let guild_id = custom
                            .args
                            .get(0)
                            .expect("Guild id expected for play")
                            .parse()
                            .expect("Guild id must be an integer");
                        let channel_id = custom
                            .args
                            .get(1)
                            .expect("Channel id expected for play")
                            .parse()
                            .expect("Channel id must be an integer");

                        play_info = Some(PlayInfo(
                            guild_id,
                            channel_id,
                            custom.args.get(2).expect("URL not provided").to_string(),
                        ));
                        self.engine.next(false).unwrap();
                    } else {
                        break;
                    }
                }
                //println!("{:#?}", self.engine.current());
                self.engine.render_to("resources/tmp.png");

                let message = temp_channel
                    .send_files(http, vec!["resources/tmp.png"], |m| m)
                    .await
                    .expect("Cannot send file");
                mci.create_interaction_response(http, |ir| {
                    self.delegate_interaction_response(ir, &message.attachments[0].url)
                        .kind(InteractionResponseType::UpdateMessage)
                })
                .await
                .expect("Cannot update interaction");
                if let Some(play_info) = play_info.take() {
                    play_url(context, play_info.0, play_info.1, &play_info.2)
                        .await
                        .expect("Cannot play URL");
                }
            } else {
                mci.create_interaction_response(http, |ir| {
                    ir.interaction_response_data(|ird| {
                        ird.create_embed(|embed| {
                            embed
                                .title("Thank you for using Gary's VN engine!")
                                .description(&format!(
                                    "You just finished playing {}",
                                    self.config
                                        .fields
                                        .get("Game")
                                        .and_then(|g| g.get("name"))
                                        .unwrap_or(&"(name not provided)".to_owned())
                                ))
                        })
                        .components(|c| c)
                    })
                    .kind(InteractionResponseType::UpdateMessage)
                })
                .await
                .expect("Unable to update interaction");
                break;
            }
        }

        Ok(())
    }
}
