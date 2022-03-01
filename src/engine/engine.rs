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

macro_rules! cast {
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

#[derive(Debug, Clone)]
pub struct Attributes(HashMap<String, AttributeValue>);
// a.b.c x
// {'a': <attribute B>}
// {'a': {'b': <attribute C>}}
// {'a': {'b': {'c': x}}}
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

    pub fn add_attribute(&mut self, attr: &AttributeDirective) {
        let mut attrs = attr.path.split('.');
        let s = attrs.next().unwrap();

        if !s.is_empty() {
            if !self.0.contains_key(s) {
                self.0
                    .insert(s.to_string(), AttributeValue::Attribute(Self::new()));
            }
            let mut v = cast!(self.0.get_mut(s).unwrap() => AttributeValue::Attribute);
            for attr in attrs {
                if !v.0.contains_key(attr) {
                    v.0.insert(attr.to_string(), AttributeValue::Attribute(Self::new()));
                }
                v = cast!(v.0.get_mut(attr).unwrap() => AttributeValue::Attribute);
            }
            v.0.insert(attr.key.clone(), AttributeValue::Value(attr.value.clone()));
        } else {
            self.0
                .insert(attr.key.clone(), AttributeValue::Value(attr.value.clone()));
        }
    }
}
/*

let mut ctab = self.0.get_mut(attrs.next().unwrap()).unwrap();
        if let Some((last, attrs)) = .collect::<Vec<_>>().split_last() {
            for attr in attrs {
                ctab = cast!(ctab.get_mut(*attr).unwrap() => AttributeValue::Attribute);

                // ctab = ctab.0.get_mut(*attr).unwrap_or_else(|| {
                // let v = Self::new();
                // ctab.0.insert(attr, AttributeValue::Attribute(v)).unwrap();
                // v
                // });
            }
        }
        loop {
            attr = match *attr.field {
                Field::Attribute(attr) => match *attr.field {
                    Field::Attribute(_) => attr,
                    Field::Value(_) => break,
                },
                _ => break,
            };

            match ctab.get_mut(&attr.path).unwrap() {
                AttributeValue::Attribute(a) => &a.0,
                AttributeValue::Value(_) => break,
            }
        }
        println!("{:#?}", ctab);
        // if let Field::Value(v) = *attr.clone().field {
        // ctab.0
        // .insert(attr.header, AttributeValue::Value(v.to_string()));
        // }

    let mut ctab = self;
    let f = attr;
    while let Field::Attribute(f) = *attr.field {
        if let Field::Attribute(f) = *f.field {
            if let Field::Value(_) = *f.field {
                break;
            }
        }
        match ctab.0.get_mut(&f.header).unwrap_or_else(|| {
            &mut ctab
                .0
                .insert(f.header, AttributeValue::Attribute(Self::new()))
                .unwrap()
        }) {
            AttributeValue::Attribute(attr) => ctab = attr,
            _ => {
                error!("{}", "receieved a value to an attribute")
            }
        }
    }
    if let Field::Value(v) = *f.field {
        ctab.0.insert(f.header, AttributeValue::Value(v));
    }
} */

// pub struct CharacterAttribute {
// text_color: Option<u32>,
// dialogue_color: Option<u32>,
// }

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
                        /*
                        match self.character_attribute.get_mut(cattr.character.as_str()) {
                            Some(s_cattr) => {
                                if let Some(v) = cattr.dialogue_color {
                                    s_cattr.dialogue_color = Some(v)
                                }
                                if let Some(v) = cattr.text_color {
                                    s_cattr.text_color = Some(v);
                                }
                            }
                            None => {
                                self.character_attribute.insert(
                                    cattr.character.to_string(),
                                    CharacterAttribute {
                                        text_color: cattr.text_color,
                                        dialogue_color: cattr.dialogue_color,
                                    },
                                );
                            }
                        }  */
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
                        self.sprites.values().collect::<Vec<_>>(),
                        &dialogue.character_name,
                        &dialogue
                            .dialogues
                            .iter()
                            .fold(String::new(), |a, b| a + " " + b),
                        match self.attributes.get("character") {
                            Some(attr) => Some(
                                cast!(attr => AttributeValue::Attribute)
                                    .get(&dialogue.character_name)
                                    .map(|val| {
                                        let c = u32::from_str_radix(
                                            cast!(val => AttributeValue::Value),
                                            16,
                                        )
                                        .unwrap();
                                        let a = c & 0xFF;
                                        let b = (c >> 8) & 0xFF;
                                        let g = (c >> 16) & 0xFF;
                                        let r = (c >> 24) & 0xFF;
                                        [r as u8, g as u8, b as u8, a as u8]
                                    })
                                    .unwrap_or([0, 0, 0, 255 / 2]),
                            ),
                            None => Some([0, 0, 0, 255 / 2]),
                        }, /*self.attributes
                           .get("character")
                           (&dialogue.character_name)
                           .map(|cattr| {
                               // cattr.dialogue_color.map(|c| {
                               // let a = c & 0xFF;
                               // let b = (c >> 8) & 0xFF;
                               // let g = (c >> 16) & 0xFF;
                               // let r = (c >> 24) & 0xFF;
                               // [r as u8, g as u8, b as u8, a as u8]
                               Some([0, 0, 0, 255 / 2])
                               // })
                           })
                           .unwrap_or(None)*/
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
        attrs.add_attribute(&AttributeDirective::from_context("a.e, 2").unwrap());
        attrs.add_attribute(&AttributeDirective::from_context("d, 2").unwrap());
        println!("{:#?}", attrs);
    }
}
