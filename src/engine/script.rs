use super::directives::{Directive, JumpDirective};
use std::{fmt::Debug, fs, io};
#[derive(Debug)]
pub struct Script {
    /// Name of the script file
    name: String,
    /// Parsed script's content
    ctx: Vec<ScriptContext>,
    // References to other script files
    //refs: Option<Box<HashMap<String, Script>>>,
}

#[derive(Debug)]
enum ScriptContext {
    Dialogue(ScriptDialogue),
    Directive(ScriptDirective),
}

#[derive(Debug)]
struct ScriptDialogue {
    character_name: String,
    dialogues: Vec<String>,
}

#[derive(Debug)]
enum ScriptDirective {
    Jump(JumpDirective),
    Sprite,
    Ending,
}

impl Script {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let mut ctx: Vec<ScriptContext> = Vec::new();

        for (i, line) in fs::read_to_string(path)?.split("\n").enumerate() {
            if line.starts_with("[") {
                let iend = line.rfind("]").expect(&format!(
                    "{}:{}:{} Expected closing ]",
                    path,
                    i,
                    line.len()
                ));
                if line.starts_with("[!meta") {
                    let directive_iend = line.find('(').expect("Expected opening parenthesis");
                    let directive_ibegin = line
                        .get(..directive_iend)
                        .unwrap()
                        .rfind(' ')
                        .expect("directives must be separated by a space")
                        + 1;
                    let ctx_iend = line.rfind(')').expect("Expected closing parenthesis");

                    ctx.push(ScriptContext::Directive(
                        match line.get(directive_ibegin..directive_iend).unwrap() {
                            "jump" => ScriptDirective::Jump(JumpDirective::from_context(
                                line.get(directive_iend + 1..ctx_iend).unwrap(),
                            )),
                            "sprite" => ScriptDirective::Sprite,
                            "ending" => ScriptDirective::Ending,
                            directive => panic!("Unrecognized directive {}!", directive),
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
                match ctx.last_mut().expect(&format!(
                    "{}:{}:{} Unmatched dialogue with character",
                    path, i, 0
                )) {
                    ScriptContext::Dialogue(dialogue) => dialogue.dialogues.push({
                        let mut line = line.to_string();
                        line.retain(|c| c != '\r');
                        line
                    }),
                    _ => panic!("Unmatched dialogue with character"),
                }
            }
        }

        Ok(Self {
            name: path.to_string(),
            ctx,
        })
    }
}
