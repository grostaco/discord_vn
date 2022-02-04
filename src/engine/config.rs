use std::{collections::HashMap, fs, io};

use super::ParseError;

#[derive(Debug)]
pub struct Config {
    pub fields: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ParseError> {
        let mut fields = HashMap::new();
        let mut last_key: Option<&str> = None;

        for (i, line) in fs::read_to_string(path)
            .map_err(|e| match e {
                _ if e.kind() == io::ErrorKind::NotFound => {
                    ParseError::NoFileExists(path.to_owned())
                }
                _ => panic!("Cannot open file {} because {}", path, e),
            })?
            .split("\n")
            .enumerate()
            .map(|(i, line)| (i + 1, line))
        {
            if line.trim().len() == 0 {
                continue;
            }

            if line.chars().nth(0).unwrap() == '[' {
                last_key = line.get(
                    1..line.rfind("]").ok_or(ParseError::SyntaxError(
                        path.to_string(),
                        i,
                        line.len(),
                        "Expected closing ]".into(),
                    ))?,
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
                    return Err(ParseError::SyntaxError(
                        path.to_string(),
                        i,
                        line.len(),
                        "Values must be separated by =".into(),
                    ));
                }
            }
        }

        Ok(Self { fields })
    }
}
