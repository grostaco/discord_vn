use image::DynamicImage;
use std::collections::HashMap;

use super::{
    script::{ScriptContext, ScriptDirective},
    AttributeDirective, ParseError, Script, SpriteDirective,
};
use crate::{
    img::{error::LoadImageError, load_image},
    Scene,
};

macro_rules! attr_cast {
    ($val:expr => $variant:path) => {
        match $val {
            $variant(v) => v,
            _ => panic!("Cannot convert to variant {}", stringify!($variant)),
        }
    };
}

pub struct Engine {
    pub script: Script,
    pub iscript: usize,
    scene: Scene,
    sprites: HashMap<String, SpriteDirective>,
    cached_bgs: HashMap<String, DynamicImage>,
    bg_path: Option<String>,
    /// Character attributes
    attributes: Attributes,
}

#[derive(Clone, Copy, Debug)]
pub enum Number {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Attributes(HashMap<String, AttributeValue>);
#[derive(Debug, Clone)]
pub enum AttributeValue {
    Attribute(Attributes),
    Value(String),
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}
impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self.0.get(key)
    }

    pub fn get_path(&self, path: &str) -> Option<&AttributeValue> {
        let mut attrs = path.trim().split('.');
        let mut ret = match self.get(attrs.next().unwrap()) {
            Some(r) => r,
            None => return None,
        };
        for attr in attrs {
            if let Some(a) = ret.as_attribute() {
                ret = match a.get(attr) {
                    Some(a) => a,
                    None => return None,
                }
            } else {
                return None;
            }
        }
        Some(ret)
    }

    pub fn add_attribute(&mut self, attr: &AttributeDirective) {
        let mut attrs = attr.path.split('.');
        let s = attrs.next().unwrap();

        if !s.is_empty() {
            if !self.0.contains_key(s) {
                self.0
                    .insert(s.to_string(), AttributeValue::Attribute(Self::new()));
            }
            let mut v = self.0.get_mut(s).unwrap().as_attribute_mut().unwrap();
            for attr in attrs {
                if !v.0.contains_key(attr) {
                    v.0.insert(attr.to_string(), AttributeValue::Attribute(Self::new()));
                }
                v = attr_cast!(v.0.get_mut(attr).unwrap() => AttributeValue::Attribute);
            }
            v.0.insert(attr.key.clone(), AttributeValue::Value(attr.value.clone()));
        } else {
            self.0
                .insert(attr.key.clone(), AttributeValue::Value(attr.value.clone()));
        }
    }
}

impl AttributeValue {
    pub fn is_attribute(&self) -> bool {
        self.as_attribute().is_some()
    }
    pub fn as_attribute(&self) -> Option<&Attributes> {
        match self {
            Self::Attribute(attr) => Some(attr),
            _ => None,
        }
    }

    pub fn as_attribute_mut(&mut self) -> Option<&mut Attributes> {
        match self {
            Self::Attribute(attr) => Some(attr),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&str> {
        match self {
            Self::Value(value) => Some(value),
            _ => None,
        }
    }
}

impl Number {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::Float(f) => Some(*f),
            Number::PosInt(_) | Number::NegInt(_) => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::NegInt(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Number::PosInt(u) => Some(*u),
            _ => None,
        }
    }
}

impl Engine {
    pub fn from_file(script_path: &str, scene: Scene) -> Result<Self, ParseError> {
        Ok(Self {
            script: Script::from_file(script_path)?,
            iscript: 0,
            scene,
            sprites: HashMap::new(),
            cached_bgs: HashMap::new(),
            bg_path: None,
            attributes: Attributes::default(),
        })
    }

    pub fn current(&self) -> Option<&ScriptContext> {
        self.script.ctx.get(self.iscript)
    }

    pub fn next(&mut self, choice: bool) -> Result<Option<&ScriptContext>, LoadImageError> {
        if let Some(ctx) = self.script.ctx.get_mut(self.iscript) {
            if let ScriptContext::Directive(directive) = ctx {
                match directive {
                    ScriptDirective::Jump(jump) => match &jump.choices {
                        Some(_) => {
                            if choice {
                                self.script = jump.endpoint.load();
                                self.iscript = 0;
                            } else {
                                self.iscript += 1
                            }
                        }
                        None => {
                            self.iscript = 0;
                            self.script = jump.endpoint.load()
                        }
                    },
                    ScriptDirective::Sprite(sprite) => {
                        if !sprite.show {
                            self.sprites.remove(&sprite.name);
                        } else {
                            self.sprites.insert(sprite.name.to_owned(), sprite.clone());
                        }
                        self.iscript += 1;
                    }
                    ScriptDirective::LoadBG(bg) => {
                        self.bg_path = Some(bg.bg_path.to_string());
                        if !self.cached_bgs.contains_key(&bg.bg_path) {
                            self.cached_bgs
                                .insert(bg.bg_path.to_string(), load_image(&bg.bg_path)?);
                        }
                        self.iscript += 1;
                    }
                    ScriptDirective::Attr(attr) => {
                        self.attributes.add_attribute(attr);
                        self.iscript += 1;
                    }
                    ScriptDirective::Custom(_) => {
                        self.iscript += 1;
                    }
                }
            } else if let ScriptContext::Dialogue(_) = ctx {
                self.iscript += 1;
            }
        }
        Ok(self.script.ctx.get(self.iscript))
    }

    pub fn next_until<P>(&mut self, predicate: P) -> Result<Option<&ScriptContext>, LoadImageError>
    where
        P: Fn(&ScriptContext) -> bool,
    {
        while let Some(context) = self.current() {
            if predicate(context) {
                break;
            }
            self.next(false)?;
        }
        Ok(self.current())
    }

    pub fn next_until_renderable(&mut self) -> Result<Option<&ScriptContext>, LoadImageError> {
        while let Some(context) = self.current() {
            match context {
                ScriptContext::Dialogue(_) => break,
                ScriptContext::Directive(directive) => {
                    if let ScriptDirective::Jump(jump) = directive {
                        if jump.choices.is_some() {
                            break;
                        }
                    }
                }
            };
            self.next(false)?;
        }
        Ok(self.current())
    }

    pub fn render(&self) {
        self.render_to(&format!("{}_{}.png", self.script.name, self.iscript));
    }

    pub fn render_to(&self, path: &str) {
        if let Some(current) = self.current() {
            if let Some(image) = match current {
                ScriptContext::Dialogue(dialogue) => Some(
                    self.scene.draw_dialogue(
                        self.bg_path
                            .as_ref()
                            .and_then(|bg_path| self.cached_bgs.get(bg_path)),
                        self.sprites.values().collect(),
                        &dialogue.character_name,
                        &dialogue
                            .dialogues
                            .iter()
                            .fold(String::new(), |a, b| a + " " + b),
                        &self.attributes,
                    ),
                ),
                ScriptContext::Directive(directive) => match directive {
                    ScriptDirective::Jump(jump) => {
                        if let Some((a, b)) = &jump.choices {
                            Some(
                                self.scene.draw_choice(
                                    self.bg_path
                                        .as_ref()
                                        .and_then(|bg_path| self.cached_bgs.get(bg_path)),
                                    &(a.as_str(), b.as_str()),
                                ),
                            )
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            } {
                image.save(path).expect("Unable to save image");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{AttributeDirective, Directive};

    use super::Attributes;

    #[test]
    fn foo() {
        let mut attrs = Attributes::new();
        attrs.add_attribute(&AttributeDirective::from_context("a.b.c, 1").unwrap());
        //attrs.add_attribute(AttributeDirective::from_context("a.b, 1").unwrap());
        attrs.add_attribute(&AttributeDirective::from_context("a.b.d, 2").unwrap());
        attrs.add_attribute(&AttributeDirective::from_context("a.e, 3").unwrap());
        attrs.add_attribute(&AttributeDirective::from_context("d, 2").unwrap());
        println!("{:#?}", attrs.get_path("a.b.c.d"));
    }
}
