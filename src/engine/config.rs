use super::util::strip_whitespace;
use std::{collections::HashMap, fs, io};

#[derive(Debug)]
pub struct Config {
    pub fields: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let mut fields = HashMap::new();
        let mut last_key: Option<&str> = None;

        for (i, line) in fs::read_to_string(path)?.split("\n").enumerate() {
            if line.len() == 0 {
                continue;
            }

            if line.chars().nth(0).unwrap() == '[' {
                last_key = line.get(
                    1..line.rfind("]").expect(&format!(
                        "{}:{}:{} Expected closing ]",
                        path,
                        i,
                        line.len()
                    )),
                );

                fields.insert(last_key.unwrap().to_owned(), HashMap::new());
            } else {
                let line = strip_whitespace(line.to_owned());
                let kv = line.split("=").take(2).collect::<Vec<&str>>();
                if let [key, value] = &kv[..] {
                    fields
                        .get_mut(last_key.unwrap())
                        .unwrap()
                        .insert(key.to_string(), value.to_string());
                } else {
                    panic!(
                        "{}:{}:{} Key and values must be separated by =",
                        path,
                        i,
                        line.len()
                    );
                }
            }
        }

        Ok(Self { fields })
    }
}
