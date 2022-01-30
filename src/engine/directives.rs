trait Directive {
    fn want_prefix(prefix: &str) -> bool;
    fn from_context(ctx: &str) -> Self;
}

struct JokeDirective {}

struct SpriteDirective {}

struct EndingDirective {}
