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
pub struct CharacterAttributeDirective {
    pub character: String,
    pub text_color: Option<u32>,
    pub dialogue_color: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct AttributeDirective {
    pub path: String,
    pub key: String,
    pub value: String,
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
            [a, b, endpoint] => (Some((a.trim().to_string(), b.trim().to_string())), endpoint),
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

        Ok(
            match &ctx.split(',').map(str::trim).collect::<Vec<&str>>()[..] {
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
            },
        )
    }
}

impl Directive for LoadBGDirective {
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        Ok(Self {
            bg_path: ctx.to_owned(),
        })
    }
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

impl Directive for CharacterAttributeDirective {
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let mut args = ctx.split(',').take(3);
        let character = args.next().ok_or_else(|| {
            ParseError::DirectiveError(
                "cattr",
                "expected character to set attribute to".to_string(),
            )
        })?;
        let attribute = args
            .next()
            .ok_or_else(|| ParseError::DirectiveError("cattr", "expected attribute".to_string()))?
            .trim();
        let value = args
            .next()
            .ok_or_else(|| {
                ParseError::DirectiveError("cattr", "expected value of attribute".to_string())
            })?
            .trim();

        let mut cattr = Self {
            character: character.trim().to_string(),
            text_color: None,
            dialogue_color: None,
        };
        match attribute {
            "dialogue_color" => {
                cattr.dialogue_color = Some(u32::from_str_radix(value, 16).map_err(|_| {
                    ParseError::DirectiveError(
                        "cattr",
                        format!("cannot parse dialogue_color {}", value),
                    )
                })?);
            }
            _ => {
                return Err(ParseError::DirectiveError(
                    "cattr",
                    format!("unknown attribute {}", attribute),
                ));
            }
        }

        Ok(cattr)
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

impl Directive for AttributeDirective {
    fn from_context(ctx: &str) -> Result<Self, ParseError> {
        let mut pair = ctx.split(',').map(str::trim).map(str::to_string).take(2);
        let path = pair
            .next()
            .ok_or_else(|| ParseError::DirectiveError("attr", "expected key".to_string()))?;
        let value = pair
            .next()
            .ok_or_else(|| ParseError::DirectiveError("attr", "expected value".to_string()))?;
        let (path, key) = path.split_at(path.rfind('.').unwrap_or(0));
        Ok(Self {
            path: path.to_string(),
            key: key.trim_matches('.').to_string(),
            value,
        })
    }
}
