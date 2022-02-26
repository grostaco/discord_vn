use super::{
    directives::{Directive, JumpDirective, LoadBGDirective, SpriteDirective},
    CharacterAttributeDirective, CustomDirective, ParseError,
};
use std::{fmt::Debug, fs, io};
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
    Cattr(CharacterAttributeDirective),
    Custom(CustomDirective),
}

macro_rules! to_syntax_error {
    ($err:expr,$path:expr,$line:expr,$character:expr) => {
        $err.map_err(|e| ParseError::SyntaxError($path, $line, $character, format!("{}", e)))
    };
}

impl Script {
    pub fn from_file(path: &str) -> Result<Self, ParseError> {
        let mut ctx: Vec<ScriptContext> = Vec::new();

        for (i, line) in fs::read_to_string(path)
            .map_err(|e| match e {
                _ if e.kind() == io::ErrorKind::NotFound => {
                    ParseError::NoFileExists(path.to_owned())
                }
                _ => panic!("Cannot open file {} because {}", path, e),
            })?
            .split('\n')
            .enumerate()
            .map(|(i, line)| (i + 1, line))
        {
            if line.starts_with('[') {
                let iend = line.rfind(']').ok_or_else(|| {
                    ParseError::SyntaxError(
                        path.to_string(),
                        i,
                        line.len(),
                        "Expected closing ]".into(),
                    )
                })?;

                ctx.push(ScriptContext::Dialogue(ScriptDialogue {
                    character_name: line.get(1..iend).map(str::to_string).unwrap(),
                    dialogues: Vec::new(),
                }))
            } else if line.starts_with('#') {
                continue;
            } else if line.starts_with('@') {
                let directive_iend = line.find('(').ok_or_else(|| {
                    ParseError::SyntaxError(path.to_string(), i, line.len(), "Expected (".into())
                })?;
                let directive = line.get(1..directive_iend).unwrap();
                let ctx_iend = line.rfind(')').ok_or_else(|| {
                    ParseError::SyntaxError(
                        path.to_string(),
                        i,
                        line.len(),
                        "Expected closing parentheses".into(),
                    )
                })?;
                let context = line.get(directive_iend + 1..ctx_iend).unwrap();

                ctx.push(ScriptContext::Directive(match directive {
                    "jump" => ScriptDirective::Jump(to_syntax_error!(
                        JumpDirective::from_context(context),
                        path.to_string(),
                        i,
                        line.len()
                    )?),
                    "sprite" => ScriptDirective::Sprite(to_syntax_error!(
                        SpriteDirective::from_context(context),
                        path.to_string(),
                        i,
                        line.len()
                    )?),
                    "loadbg" => ScriptDirective::LoadBG(to_syntax_error!(
                        LoadBGDirective::from_context(context),
                        path.to_string(),
                        i,
                        line.len()
                    )?),
                    "cattr" => ScriptDirective::Cattr(to_syntax_error!(
                        CharacterAttributeDirective::from_context(context),
                        path.to_string(),
                        i,
                        line.len()
                    )?),
                    "custom" => ScriptDirective::Custom(to_syntax_error!(
                        CustomDirective::from_context(context),
                        path.to_string(),
                        i,
                        line.len()
                    )?),

                    directive => {
                        return Err(ParseError::SyntaxError(
                            path.to_string(),
                            i,
                            line.len(),
                            format!("Unknown directive {}", directive),
                        ))
                    }
                }));
            } else if !line.trim().is_empty() {
                // the rest here must be dialogues
                match ctx.last_mut().ok_or_else(|| {
                    ParseError::SyntaxError(
                        path.to_string(),
                        i,
                        0,
                        "Unmatched dialogue with character".to_string(),
                    )
                })? {
                    ScriptContext::Dialogue(dialogue) => dialogue.dialogues.push({
                        let mut line = line.to_string();
                        line.retain(|c| c != '\r');
                        line
                    }),
                    _ => {
                        return Err(ParseError::SyntaxError(
                            path.to_string(),
                            i,
                            0,
                            "Unmatched dialogue with character".to_string(),
                        ))
                    }
                }
            }
        }

        Ok(Self {
            name: path.to_string(),
            ctx,
        })
    }
}
