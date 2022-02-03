use super::{
    directives::{Directive, JumpDirective, LoadBGDirective, SpriteDirective},
    ScriptError, SyntaxError,
};
use std::{fmt::Debug, fs};
#[derive(Clone, Debug)]
pub struct Script {
    /// Name of the script file
    pub name: String,
    /// Parsed script's content
    pub ctx: Vec<ScriptContext>,
    // References to other script files
    //refs: Option<Box<HashMap<String, Script>>>,
}

#[derive(Clone, Debug)]
pub enum ScriptContext {
    Dialogue(ScriptDialogue),
    Directive(ScriptDirective),
}

#[derive(Clone, Debug)]
pub struct ScriptDialogue {
    pub character_name: String,
    pub dialogues: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum ScriptDirective {
    Jump(JumpDirective),
    Sprite(SpriteDirective),
    LoadBG(LoadBGDirective),
}

impl Script {
    pub fn from_file(path: &str) -> Result<Self, ScriptError> {
        let mut ctx: Vec<ScriptContext> = Vec::new();

        for (i, line) in fs::read_to_string(path)?
            .split("\n")
            .enumerate()
            .map(|(i, line)| (i + 1, line))
        {
            if line.starts_with("[") {
                let iend = line.rfind("]").ok_or(SyntaxError {
                    file: path.to_string(),
                    line: i,
                    character: line.len(),
                    why: "Expected closing ]".to_string(),
                })?;
                if line.starts_with("[!meta") {
                    let directive_iend = line.find('(').ok_or(SyntaxError {
                        file: path.to_string(),
                        line: i,
                        character: line.len(),
                        why: "Expected (".to_string(),
                    })?;
                    let directive_ibegin =
                        line.get(..directive_iend)
                            .unwrap()
                            .rfind(' ')
                            .ok_or(SyntaxError {
                                file: path.to_string(),
                                line: i,
                                character: line.len(),
                                why: "Directives must be separated by a space".to_string(),
                            })?
                            + 1;
                    let ctx_iend = line.rfind(')').ok_or(SyntaxError {
                        file: path.to_string(),
                        line: i,
                        character: line.len(),
                        why: "Expected closing parentheses".to_string(),
                    })?;
                    let context = line.get(directive_iend + 1..ctx_iend).unwrap();

                    ctx.push(ScriptContext::Directive(
                        match line.get(directive_ibegin..directive_iend).unwrap() {
                            "jump" => ScriptDirective::Jump(
                                JumpDirective::from_context(context).map_err(|d| SyntaxError {
                                    file: path.to_string(),
                                    line: i,
                                    character: line.len(),
                                    why: d.why.to_string(),
                                })?,
                            ),
                            "sprite" => ScriptDirective::Sprite(
                                SpriteDirective::from_context(context).map_err(|d| {
                                    SyntaxError {
                                        file: path.to_string(),
                                        line: i,
                                        character: line.len(),
                                        why: d.why.to_string(),
                                    }
                                })?,
                            ),
                            "loadbg" => {
                                ScriptDirective::LoadBG(LoadBGDirective::from_context(context)?)
                            }

                            directive => Err(SyntaxError {
                                file: path.to_string(),
                                line: i,
                                character: line.len(),
                                why: format!("Unknown directive {}", directive),
                            })?,
                        },
                    ));
                } else {
                    ctx.push(ScriptContext::Dialogue(ScriptDialogue {
                        character_name: line.get(1..iend).map(str::to_string).unwrap(),
                        dialogues: Vec::new(),
                    }))
                }
            } else if line.len() != 0 {
                // the rest here must be dialogues
                match ctx.last_mut().ok_or(SyntaxError {
                    file: path.to_string(),
                    line: i,
                    character: 0,
                    why: "Unmatched dialogue with character".to_string(),
                })? {
                    ScriptContext::Dialogue(dialogue) => dialogue.dialogues.push({
                        let mut line = line.to_string();
                        line.retain(|c| c != '\r');
                        line
                    }),
                    _ => Err(SyntaxError {
                        file: path.to_string(),
                        line: i,
                        character: 0,
                        why: "Unmatched dialogue with character".to_string(),
                    })?,
                }
            }
        }

        Ok(Self {
            name: path.to_string(),
            ctx,
        })
    }
}
