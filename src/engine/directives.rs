use super::{ParseError, Script};

pub trait Directive: Sized {
    fn from_context(ctx: &str) -> Result<Self, ParseError>;
}

#[derive(Clone, Debug)]
pub struct JumpDirective {
    pub choices: Option<(String, String)>,
    pub endpoint: Script,
}

#[derive(Clone, Debug)]
pub struct SpriteDirective {
    pub name: String,
    pub sprite_path: Option<String>,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub show: bool,
}

#[derive(Clone, Debug)]
pub struct LoadBGDirective {
    pub bg_path: String,
}

impl Directive for JumpDirective {
    /// Return a jump directive from context
    /// "A", "B", endpoint.script to jump to endpoint.script if A is taken or
    /// endpoint.script
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let ctx = ctx.split(",").take(3).collect::<Vec<_>>();
        let (choices, endpoint) = match &ctx[..] {
            [a, b, endpoint] => (Some((a.to_string(), b.to_string())), endpoint),
            [endpoint] => (None, endpoint),
            _ => {
                return Err(ParseError::DirectiveError(
                    "jump",
                    format!("jump directive expects 1 or 3 arguments, got {}", ctx.len()),
                ))
            }
        };
        Ok(Self {
            choices,
            endpoint: Script::from_file(&endpoint.split_whitespace().collect::<String>())?,
        })
    }
}

impl Directive for SpriteDirective {
    /// Return a sprite directive from context
    /// name,display,x,y,show|hide
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let ctx = ctx.split_whitespace().collect::<String>();

        Ok(match &ctx.split(",").collect::<Vec<&str>>()[..] {
            [name, display, x, y, visibility] => Self {
                name: name.to_string(),
                sprite_path: Some(display.to_string()),
                x: Some(x.parse().expect("x must be an integer")),
                y: Some(y.parse().expect("y must be an integer")),
                show: match *visibility {
                    "show" => true,
                    "hide" => false,
                    _ => {
                        return Err(ParseError::DirectiveError(
                            "sprite",
                            "visibility must either be show or hide".into(),
                        ))
                    }
                },
            },
            [name, visibility] => Self {
                name: name.to_string(),
                sprite_path: None,
                x: None,
                y: None,
                show: match *visibility {
                    "hide" => false,
                    _ => {
                        return Err(ParseError::DirectiveError(
                            "sprite",
                            "Non-hidden sprite directives expect 5 arguments".into(),
                        ))
                    }
                },
            },

            _ => {
                return Err(ParseError::DirectiveError(
                    "sprite",
                    "directives expect 5 arguments for show and 2 arguments for hide".into(),
                ))
            }
        })
    }
}

impl Directive for LoadBGDirective {
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        Ok(Self {
            bg_path: ctx.to_string(),
        })
    }
}
