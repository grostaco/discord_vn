use super::{DirectiveError, Script};

pub trait Directive: Sized {
    fn from_context(ctx: &str) -> Result<Self, DirectiveError>;
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
    fn from_context(ctx: &str) -> Result<Self, DirectiveError> {
        let ctx = ctx.split(",").take(3).collect::<Vec<_>>();
        let (choices, endpoint) = match &ctx[..] {
            [a, b, endpoint] => (Some((a.to_string(), b.to_string())), endpoint),
            [endpoint] => (None, endpoint),
            _ => panic!("A jump directive cannot be empty"),
        };
        Ok(Self {
            choices,
            endpoint: Script::from_file(&endpoint.split_whitespace().collect::<String>()).map_err(
                |_| DirectiveError {
                    why: "Cannot open script file",
                },
            )?,
        })
    }
}

impl Directive for SpriteDirective {
    /// Return a sprite directive from context
    /// name,display,x,y,show|hide
    fn from_context(ctx: &str) -> Result<Self, DirectiveError> {
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
                    _ => panic!("Visibility must either be show or hide"),
                },
            },
            [name, visibility] => Self {
                name: name.to_string(),
                sprite_path: None,
                x: None,
                y: None,
                show: match *visibility {
                    "hide" => false,
                    _ => Err(DirectiveError {
                        why: "Non-hidden sprite directives expect 5 arguments",
                    })?,
                },
            },
            _ => Err(DirectiveError {
                why: "Sprite directives expect 5 arguments for show and 2 arguments for hide",
            })?,
        })
    }
}

impl Directive for LoadBGDirective {
    fn from_context(ctx: &str) -> Result<Self, DirectiveError> {
        Ok(Self {
            bg_path: ctx.to_string(),
        })
    }
}
