use super::{
    script::{ScriptContext, ScriptDirective},
    Script,
};
use crate::Scene;

pub struct Engine<'a> {
    pub script: Script,
    iscript: usize,
    scene: Scene<'a>,
    bg: Option<String>,
}

impl<'a> Engine<'a> {
    pub fn from_file(script_path: &str, scene: Scene<'a>) -> Self {
        Self {
            script: Script::from_file(script_path).expect("Cannot create script"),
            iscript: 0,
            scene,
            bg: None,
        }
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
                                self.script = jump.endpoint.clone()
                            } else {
                                self.iscript += 1
                            }
                        }
                        None => self.script = jump.endpoint.clone(),
                    },
                    ScriptDirective::LoadBG(bg) => {
                        self.bg = Some(bg.bg_path.to_owned());
                        self.iscript += 1;
                    }
                    _ => panic!("Unsupported directive {:?}", directive),
                }
            } else if let ScriptContext::Dialogue(_) = ctx {
                self.iscript += 1;
            }
        }
        self.script.ctx.get(self.iscript)
    }

    pub fn render(&self) {
        if let Some(current) = self.current() {
            if let ScriptContext::Dialogue(dialogue) = current {
                let image = self.scene.draw(
                    self.bg.as_ref().map(|bg| bg.as_str()),
                    &dialogue.character_name,
                    &dialogue
                        .dialogues
                        .iter()
                        .fold(String::new(), |a, b| a + " " + &b),
                );

                image
                    .save(&format!("{}_{}.png", self.script.name, self.iscript))
                    .expect("Cannot save image");
            }
        }
    }

    pub fn render_to(&self, path: &str) {
        if let Some(current) = self.current() {
            if let ScriptContext::Dialogue(dialogue) = current {
                let image = self.scene.draw(
                    self.bg.as_ref().map(|bg| bg.as_str()),
                    &dialogue.character_name,
                    &dialogue
                        .dialogues
                        .iter()
                        .fold(String::new(), |a, b| a + " " + &b),
                );

                image.save(path).expect("Cannot save image");
            }
        }
    }
}
