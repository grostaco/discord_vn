use std::collections::HashMap;

use image::DynamicImage;

use super::{
    script::{ScriptContext, ScriptDirective},
    ParseError, Script, SpriteDirective,
};
use crate::{
    img::{error::LoadImageError, load_image},
    Scene,
};

pub struct Engine {
    pub script: Script,
    pub iscript: usize,
    scene: Scene,
    sprites: HashMap<String, SpriteDirective>,
    cached_bgs: HashMap<String, DynamicImage>,
    bg_path: Option<String>,
    /// Character attributes
    character_attribute: HashMap<String, CharacterAttribute>,
}

pub struct CharacterAttribute {
    text_color: Option<u32>,
    dialogue_color: Option<u32>,
}

impl Engine {
    pub fn from_file(script_path: &str, scene: Scene) -> Result<Self, ParseError> {
        Ok(Self {
            script: Script::from_file(script_path)?,
            iscript: 0,
            scene,
            sprites: HashMap::new(),
            cached_bgs: HashMap::new(),
            bg_path: None,
            character_attribute: HashMap::new(),
        })
    }

    pub fn current(&self) -> Option<&ScriptContext> {
        self.script.ctx.get(self.iscript)
    }

    pub fn next(&mut self, choice: bool) -> Result<Option<&ScriptContext>, LoadImageError> {
        if let Some(ctx) = self.script.ctx.get_mut(self.iscript) {
            if let ScriptContext::Directive(directive) = ctx {
                match directive {
                    ScriptDirective::Jump(jump) => match &jump.choices {
                        Some(_) => {
                            if choice {
                                self.script = jump.endpoint.load();
                                self.iscript = 0;
                            } else {
                                self.iscript += 1
                            }
                        }
                        None => {
                            self.iscript = 0;
                            self.script = jump.endpoint.load()
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
                        self.bg_path = Some(bg.bg_path.to_string());
                        if !self.cached_bgs.contains_key(&bg.bg_path) {
                            self.cached_bgs
                                .insert(bg.bg_path.to_string(), load_image(&bg.bg_path)?);
                        }
                        self.iscript += 1;
                    }
                    ScriptDirective::Cattr(cattr) => {
                        match self.character_attribute.get_mut(cattr.character.as_str()) {
                            Some(s_cattr) => {
                                if let Some(v) = cattr.dialogue_color {
                                    s_cattr.dialogue_color = Some(v)
                                }
                                if let Some(v) = cattr.text_color {
                                    s_cattr.text_color = Some(v);
                                }
                            }
                            None => {
                                self.character_attribute.insert(
                                    cattr.character.to_string(),
                                    CharacterAttribute {
                                        text_color: cattr.text_color,
                                        dialogue_color: cattr.dialogue_color,
                                    },
                                );
                            }
                        }
                        self.iscript += 1;
                    }
                    ScriptDirective::Custom(_) => {
                        self.iscript += 1;
                    }
                }
            } else if let ScriptContext::Dialogue(_) = ctx {
                self.iscript += 1;
            }
        }
        Ok(self.script.ctx.get(self.iscript))
    }

    pub fn next_until<P>(&mut self, predicate: P) -> Result<Option<&ScriptContext>, LoadImageError>
    where
        P: Fn(&ScriptContext) -> bool,
    {
        while let Some(context) = self.current() {
            if predicate(context) {
                break;
            }
            self.next(false)?;
        }
        Ok(self.current())
    }

    pub fn next_until_renderable(&mut self) -> Result<Option<&ScriptContext>, LoadImageError> {
        while let Some(context) = self.current() {
            match context {
                ScriptContext::Dialogue(_) => break,
                ScriptContext::Directive(directive) => {
                    if let ScriptDirective::Jump(jump) = directive {
                        if jump.choices.is_some() {
                            break;
                        }
                    }
                }
            };
            self.next(false)?;
        }
        Ok(self.current())
    }

    pub fn render(&self) {
        self.render_to(&format!("{}_{}.png", self.script.name, self.iscript));
    }

    pub fn render_to(&self, path: &str) {
        if let Some(current) = self.current() {
            if let Some(image) = match current {
                ScriptContext::Dialogue(dialogue) => Some(
                    self.scene.draw_dialogue(
                        self.bg_path
                            .as_ref()
                            .and_then(|bg_path| self.cached_bgs.get(bg_path)),
                        self.sprites.values().collect::<Vec<_>>(),
                        &dialogue.character_name,
                        &dialogue
                            .dialogues
                            .iter()
                            .fold(String::new(), |a, b| a + " " + b),
                        self.character_attribute
                            .get(&dialogue.character_name)
                            .map(|cattr| {
                                cattr.dialogue_color.map(|c| {
                                    let a = c & 0xFF;
                                    let b = (c >> 8) & 0xFF;
                                    let g = (c >> 16) & 0xFF;
                                    let r = (c >> 24) & 0xFF;
                                    [r as u8, g as u8, b as u8, a as u8]
                                })
                            })
                            .unwrap_or(None),
                    ),
                ),
                ScriptContext::Directive(directive) => match directive {
                    ScriptDirective::Jump(jump) => {
                        if let Some((a, b)) = &jump.choices {
                            Some(
                                self.scene.draw_choice(
                                    self.bg_path
                                        .as_ref()
                                        .and_then(|bg_path| self.cached_bgs.get(bg_path)),
                                    &(a.as_str(), b.as_str()),
                                ),
                            )
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            } {
                image.save(path).expect("Unable to save image");
            }
        }
    }
}
