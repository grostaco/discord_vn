use image::DynamicImage;

use super::{ParseError, Script};

pub trait Directive: Sized {
    fn from_context(ctx: &str) -> Result<Self, ParseError>;
}

#[derive(Clone, Debug)]
pub struct JumpDirective {
    pub choices: Option<(String, String)>,
    pub endpoint: LazilyLoadedScript,
}

#[derive(Clone, Debug)]
pub struct LazilyLoadedScript {
    pub script_path: String,
    pub script: Option<Script>,
}

#[derive(Clone, Debug)]
pub struct SpriteDirective {
    pub name: String,
    pub sprite_path: Option<String>,
    pub sprite: Option<DynamicImage>,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub show: bool,
}

#[derive(Clone, Debug)]
pub struct LoadBGDirective {
    pub bg_path: String,
}

#[derive(Clone, Debug)]
pub struct CustomDirective {
    pub name: String,
    pub args: Vec<String>,
}

impl Directive for JumpDirective {
    /// Return a jump directive from context
    /// "A", "B", endpoint.script to jump to endpoint.script if A is taken or
    /// endpoint.script
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let ctx = ctx.split(',').take(3).collect::<Vec<_>>();
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
            endpoint: LazilyLoadedScript::new(&endpoint.split_whitespace().collect::<String>()),
        })
    }
}

impl Directive for SpriteDirective {
    /// Return a sprite directive from context
    /// name,display,x,y,show|hide
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let ctx = ctx.split_whitespace().collect::<String>();

        Ok(match &ctx.split(',').collect::<Vec<&str>>()[..] {
            [name, display, x, y, visibility] => Self {
                name: name.to_string(),
                sprite_path: Some(display.to_string()),
                sprite: None,
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
                sprite: None,
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
            bg_path: ctx.to_owned(),
        })
    }
    /*
                bg: load_image(ctx).map_err(|e| match e {
                LoadImageError::ImageError(img) => {
                    ParseError::ImageError(ctx.to_string(), img.to_string().to_ascii_lowercase())
                }
                LoadImageError::IoError(_) => ParseError::NoFileExists(ctx.to_string()),
            })?,
    */
}

impl Directive for CustomDirective {
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let directive_iend = ctx
            .find('(')
            .ok_or_else(|| ParseError::DirectiveError("custom", "expected opening (".into()))?;
        let directive = ctx.get(..directive_iend).unwrap().to_string();
        let args = ctx
            .get(directive_iend + 1..ctx.len() - 1)
            .unwrap()
            .split(',')
            .map(str::trim)
            .map(str::to_string)
            .collect();

        Ok(Self {
            name: directive,
            args,
        })
    }
}

impl LazilyLoadedScript {
    pub fn new(script_path: &str) -> Self {
        Self {
            script_path: script_path.to_string(),
            script: None,
        }
    }

    pub fn load(&mut self) -> Script {
        self.script
            .get_or_insert(Script::from_file(&self.script_path).unwrap())
            .clone()
    }
}
