use super::Script;

pub trait Directive {
    fn from_context(ctx: &str) -> Self;
}

#[derive(Clone, Debug)]
pub struct JumpDirective {
    pub choices: Option<(String, String)>,
    pub endpoint: Script,
}

#[derive(Clone, Debug)]
pub struct SpriteDirective {
    pub name: String,
    pub sprite_path: String,
    pub x: u32,
    pub y: u32,
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
    fn from_context(ctx: &str) -> Self {
        let ctx = ctx.split(",").take(3).collect::<Vec<_>>();
        let (choices, endpoint) = match &ctx[..] {
            [a, b, endpoint] => (Some((a.to_string(), b.to_string())), endpoint),
            [endpoint] => (None, endpoint),
            _ => panic!("A jump directive cannot be empty"),
        };
        Self {
            choices,
            endpoint: Script::from_file(&endpoint.split_whitespace().collect::<String>())
                .expect("Unable to load script file"),
        }
    }
}

impl Directive for SpriteDirective {
    /// Return a sprite directive from context
    /// name,display,x,y,show|hide
    fn from_context(ctx: &str) -> Self {
        let ctx = ctx.split_whitespace().collect::<String>();

        match &ctx.split(",").collect::<Vec<&str>>()[..] {
            [name, display, x, y, visibility] => {
                return Self {
                    name: name.to_string(),
                    sprite_path: display.to_string(),
                    x: x.parse().expect("x must be an integer"),
                    y: y.parse().expect("y must be an integer"),
                    show: match *visibility {
                        "show" => true,
                        "hide" => false,
                        _ => panic!("Visibility must either be show or hide"),
                    },
                }
            }
            _ => panic!("Sprite directive's argument must be split by ="),
        }
    }
}

impl Directive for LoadBGDirective {
    fn from_context(ctx: &str) -> Self {
        Self {
            bg_path: ctx.to_string(),
        }
    }
}
