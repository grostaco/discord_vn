use std::{collections::HashMap, fs};

use super::{ParseError, SyntaxError};

#[derive(Debug)]
pub struct Config {
    pub fields: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ParseError> {
        let mut fields = HashMap::new();
        let mut last_key: Option<&str> = None;

        for (i, line) in fs::read_to_string(path)?.split("\n").enumerate() {
            if line.trim().len() == 0 {
                continue;
            }

            if line.chars().nth(0).unwrap() == '[' {
                last_key = line.get(
                    1..line.rfind("]").ok_or_else(|| SyntaxError {
                        file: path.to_owned(),
                        line: i,
                        character: line.len(),
                        why: "Expected closing ]",
                    })?,
                );

                fields.insert(last_key.unwrap().to_owned(), HashMap::new());
            } else {
                let kv = line.split("=").take(2).collect::<Vec<&str>>();
                if let [key, value] = &kv[..] {
                    fields
                        .get_mut(last_key.unwrap())
                        .unwrap()
                        .insert(key.trim().to_string(), value.trim().to_string());
                } else {
                    Err(SyntaxError {
                        file: path.to_owned(),
                        line: i,
                        character: line.len(),
                        why: "Key and values must be separated by =",
                    })?;
                }
            }
        }

        Ok(Self { fields })
    }
}
