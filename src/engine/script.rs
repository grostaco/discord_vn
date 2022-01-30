pub struct Script {}

struct ScriptNode {
    name: String,
    ctx: Vec<ScriptContext>,
    refs: Option<Box<Vec<ScriptNode>>>,
    choice: Option<Choice>,
}

enum ScriptContext {
    Dialogue(ScriptDialogue),
    Meta(ScriptMeta),
}

struct ScriptDialogue {
    character_name: String,
    dialogues: Vec<String>,
}
struct ScriptMeta {
    directive_type: ScriptMetaDirective,
    context: String,
}

enum ScriptMetaDirective {
    Joke,
    Sprite,
    Ending,
}

struct Choice {
    option: String,
    connector: Box<ScriptNode>,
}

impl Script {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let lines = fs::read_to_string(path)?.split("\n");

        Ok(Self {})
    }
}
