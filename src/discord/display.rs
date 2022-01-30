use crate::Script;

pub struct Begin {
    script: Script,
}

impl Begin {
    pub fn new(script_file: &str) -> Self {
        Self {
            script: Script::from_file(script_file).expect("Cannot load script file"),
        }
    }
}
