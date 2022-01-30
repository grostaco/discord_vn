use super::Script;

pub trait Directive {
    fn from_context(ctx: &str) -> Self;
}

#[derive(Debug)]
pub struct JumpDirective {
    pub choices: Option<(String, String)>,
    pub endpoint: Script,
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

struct SpriteDirective {
    location: Option<String>,
    sprite_type: Option<String>,
}

struct EndingDirective {}
