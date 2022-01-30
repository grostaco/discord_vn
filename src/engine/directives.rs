use super::Script;

pub trait Directive {
    fn from_context(ctx: &str) -> Self;
}

#[derive(Debug)]
pub struct JumpDirective {
    pub choices: Option<(String, String)>,
    pub endpoint: Script,
}

#[derive(Debug)]
pub struct SpriteDirective {
    location: Option<String>,
    sprite_type: Option<String>,
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
            endpoint: Script::from_file(endpoint).expect("Unable to load script file"),
        }
    }
}

impl Directive for SpriteDirective {
    /// Return a sprite directive from context
    /// loc=[left|right], display=[any]
    fn from_context(ctx: &str) -> Self {
        let ctx = ctx.split_whitespace().collect::<String>();
        let mut sprite_directive = Self {
            location: None,
            sprite_type: None,
        };

        for kv in &ctx.split(",").collect::<Vec<&str>>()[..] {
            match &kv.split("=").take(2).collect::<Vec<&str>>()[..] {
                [key, value] => match key {
                    &"loc" => sprite_directive.location = Some(value.to_string()),
                    &"display" => sprite_directive.sprite_type = Some(value.to_string()),
                    _ => panic!("Unknown key {} for sprite directive", key),
                },
                _ => panic!("Sprite directive's argument must be split by ="),
            }
        }

        sprite_directive
    }
}
