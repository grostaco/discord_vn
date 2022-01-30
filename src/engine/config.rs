use super::util::strip_whitespace;
use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    header: Option<String>,
    values: Vec<(String, String)>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let mut fields = Vec::new();

        for (i, line) in fs::read_to_string(path)?.split("\n").enumerate() {
            if line.len() == 0 {
                continue;
            }

            if line.chars().nth(0).unwrap() == '[' {
                fields.push(Field {
                    header: line
                        .get(
                            1..line.rfind("]").expect(&format!(
                                "{}:{}:{} Expected closing ]",
                                path,
                                i,
                                line.len()
                            )),
                        )
                        .map(|s| s.to_owned()),
                    values: Vec::new(),
                });
            } else {
                let line = strip_whitespace(line.to_owned());
                let kv = line.split("=").take(2).collect::<Vec<&str>>();
                if let [key, value] = &kv[..] {
                    fields
                        .last_mut()
                        .unwrap()
                        .values
                        .push((key.to_string(), value.to_string()));
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
