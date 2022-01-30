trait Directive {
    fn want_prefix(prefix: &str) -> bool;
    fn from_context(ctx: &str) -> Self;
}

struct JumpDirective {}

struct SpriteDirective {}

struct EndingDirective {}
