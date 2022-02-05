use std::collections::HashMap;

use super::{
    script::{ScriptContext, ScriptDirective},
    ParseError, Script, SpriteDirective,
};
use crate::Scene;

pub struct Engine<'a> {
    pub script: Script,
    pub iscript: usize,
    scene: &'a Scene<'a>,
    sprites: HashMap<String, SpriteDirective>,
    bg: Option<String>,
}

impl<'a> Engine<'a> {
    pub fn from_file(script_path: &str, scene: &'a Scene<'a>) -> Result<Self, ParseError> {
        Ok(Self {
            script: Script::from_file(script_path)?,
            iscript: 0,
            scene,
            sprites: HashMap::new(),
            bg: None,
        })
    }

    pub fn current(&self) -> Option<&ScriptContext> {
        self.script.ctx.get(self.iscript)
    }

    pub fn next(&mut self, choice: bool) -> Option<&ScriptContext> {
        if let Some(ctx) = self.script.ctx.get(self.iscript) {
            if let ScriptContext::Directive(directive) = ctx {
                match directive {
                    ScriptDirective::Jump(jump) => match &jump.choices {
                        Some(_) => {
                            if choice {
                                self.script = jump.endpoint.clone();
                                self.iscript = 0;
                            } else {
                                self.iscript += 1
                            }
                        }
                        None => {
                            self.iscript = 0;
                            self.script = jump.endpoint.clone()
                        }
                    },
                    ScriptDirective::Sprite(sprite) => {
                        if !sprite.show {
                            self.sprites.remove(&sprite.name);
                        } else {
                            self.sprites.insert(sprite.name.to_owned(), sprite.clone());
                        }
                        self.iscript += 1;
                    }
                    ScriptDirective::LoadBG(bg) => {
                        self.bg = Some(bg.bg_path.to_owned());
                        self.iscript += 1;
                    }
                }
            } else if let ScriptContext::Dialogue(_) = ctx {
                self.iscript += 1;
            }
        }
        self.script.ctx.get(self.iscript)
    }

    pub fn next_until<P>(&mut self, predicate: P) -> Option<&ScriptContext>
    where
        P: Fn(&ScriptContext) -> bool,
    {
        while let Some(context) = self.current() {
            if predicate(context) {
                break;
            }
            self.next(false);
        }
        self.current()
    }

    pub fn next_until_renderable(&mut self) -> Option<&ScriptContext> {
        while let Some(context) = self.current() {
            match context {
                ScriptContext::Dialogue(_) => break,
                ScriptContext::Directive(directive) => match directive {
                    ScriptDirective::Jump(jump) => match &jump.choices {
                        Some(_) => break,
                        _ => {}
                    },
                    _ => {}
                },
            };
            self.next(false);
        }
        self.current()
    }

    pub fn render(&self) {
        self.render_to(&format!("{}_{}.png", self.script.name, self.iscript));
    }

    pub fn render_to(&self, path: &str) {
        if let Some(current) = self.current() {
            if let Some(image) = match current {
                ScriptContext::Dialogue(dialogue) => Some(
                    self.scene.draw_dialogue(
                        self.bg.as_ref().map(|bg| bg.as_str()),
                        self.sprites.values().collect::<Vec<_>>(),
                        &dialogue.character_name,
                        &dialogue
                            .dialogues
                            .iter()
                            .fold(String::new(), |a, b| a + " " + &b),
                    ),
                ),
                ScriptContext::Directive(directive) => match directive {
                    ScriptDirective::Jump(jump) => match &jump.choices {
                        Some((a, b)) => Some(self.scene.draw_choice(
                            self.bg.as_ref().map(|bg| bg.as_str()),
                            &(a.as_str(), b.as_str()),
                        )),
                        None => None,
                    },
                    _ => None,
                },
            } {
                image.save(path).expect("Unable to save image");
            }
        }
    }
}
