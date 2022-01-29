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

        for line in fs::read_to_string(path)?.split("\n") {
            if line.len() == 0 {
                continue;
            }

            if line.chars().nth(0).unwrap() == '[' {
                fields.push(Field {
                    header: line
                        .get(1..line.rfind("]").expect("Cannot find matching end ]"))
                        .map(|s| s.to_owned()),
                    values: Vec::new(),
                });
            } else {
                let line = strip_whitespace(line.to_owned());
                let kv: Vec<&str> = line.split("=").collect();
                fields
                    .last_mut()
                    .unwrap()
                    .values
                    .push((kv[0].to_owned(), kv[1].to_owned()));
            }
        }

        Ok(Self { fields })
    }
}
