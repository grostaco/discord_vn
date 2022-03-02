use env_logger::fmt::Color;
use image_rpg::{
    engine::{ScriptContext, ScriptDirective},
    Config, Engine, Scene, Size,
};
use log::{debug, error, info, Level};
use rusttype::{Font, Scale};
use std::{
    fs,
    io::{self, Write},
    process::exit,
};

macro_rules! log {
    (dbg,$x:expr) => {
        info!("{}", $x);
    };
    (info,$x:expr) => {
        println!("[!] {}", $x);
    };
    (input,$x:expr) => {
        println!("[?] {}", $x);
    };
    (err,$x:expr) => {{
        println!("[!!] {}. Aborting.", $x);
        exit(0);
    }};
}

fn main() {
    env_logger::init();

    env_logger::builder().format(|buf, record| {
        let normal_style = buf.style();
        let mut info_style = buf.style();
        info_style.set_color(Color::Green);
        writeln!(
            buf,
            "{}: {}",
            match record.level() {
                Level::Info => info_style.value("I"),
                _ => normal_style.value("?"),
            },
            record.args()
        )
    });
    info!("Discord VN scripting engine v1.0.0");

    let font_data = include_bytes!("../../resources/fonts/cour.ttf");
    debug!("Loading font");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    debug!("Font loaded");

    let scene = Scene {
        font,
        scale: Scale::uniform(24.0),
        screen: Size {
            xmin: 0,
            ymin: 0,
            ymax: 480,
            xmax: 640,
        },
        sprite: Size {
            xmin: 0,
            ymin: 0,
            ymax: 0,
            xmax: 0,
        },
        text: Size {
            xmin: 20,
            xmax: 620,
            ymin: 340,
            ymax: 480,
        },
    };

    let config = Config::from_file("resources/config.conf").unwrap_or_else(|e| log!(err, e));
    let mut rendered = 0;
    let script_path = config
        .fields
        .get("Path")
        .unwrap_or_else(|| log!(err, "Cannot find [Path] in config file"))
        .get("script_path")
        .unwrap_or_else(|| log!(err, "script_path not set in [Path]"));
    debug!("Engine initializing, searching for {}", script_path);
    match Engine::from_file(script_path.as_str(), scene) {
        Ok(mut engine) => {
            debug!("Engine initialized. Rendering...");
            info!("It should be noted that if there are conditional jumps in the script, you will be prompted.");
            debug!("Removing previous render files");
            for file in fs::read_dir("resources/render").unwrap().flatten() {
                debug!("Removing file {}", file.file_name().to_str().unwrap());
                fs::remove_file(file.path()).unwrap();
            }
            while let Some(ctx) = engine.current() {
                let mut choice = false;
                match ctx {
                    ScriptContext::Dialogue(dialogue) => {
                        debug!(
                            "Rendering dialogue: \"{}\"",
                            dialogue.dialogues.iter().fold(String::new(), |a, b| a + b)
                        );
                        rendered += 1;
                    }
                    ScriptContext::Directive(directive) => match directive {
                        ScriptDirective::LoadBG(loadbg) => {
                            debug!("Loading background {}", loadbg.bg_path);
                        }
                        ScriptDirective::Jump(jump) => match &jump.choices {
                            Some((a, b)) => {
                                rendered += 1;
                                let mut buf = String::new();
                                loop {
                                    info!("A conditional jump was found, choose 1 or 2.",);
                                    print!(
                                        "+{nothing:-<width$}+\n\
                                        | [1] {a:<xwidth$}|\n\
                                        | [2] {b:<xwidth$}|\n\
                                        +{nothing:-<width$}+\n\
                                        (1 or 2) > ",
                                        nothing = "",
                                        width = a.len().max(b.len()) * 2,
                                        xwidth = a.len().max(b.len()) * 2 - 5,
                                    );
                                    io::stdout().flush().unwrap();
                                    io::stdin().read_line(&mut buf).unwrap();
                                    match buf.trim().parse::<u64>() {
                                        Ok(num) => match num {
                                            1 => {
                                                choice = true;
                                                break;
                                            }
                                            2 => {
                                                choice = false;
                                                break;
                                            }
                                            _ => error!(
                                                "The choice must be either 1 or 2. Reprompting."
                                            ),
                                        },
                                        Err(_) => {
                                            error!("The choice number must be an integer. Reprompting.");
                                        }
                                    }
                                    buf.clear();
                                }
                            }
                            None => debug!("Jumping to {}", jump.endpoint.script_path),
                        },
                        ScriptDirective::Sprite(sprite) => {
                            if let Some(sprite_path) = &sprite.sprite_path {
                                debug!("Loading sprite {}", sprite_path)
                            } else {
                                debug!("Unloading sprite {}", sprite.name)
                            }
                        }
                        ScriptDirective::Attr(attr) => {
                            debug!("Got attribute {:?}", attr);
                        }
                        ScriptDirective::Custom(custom) => {
                            debug!("Ignoring custom directive {:#?}", custom)
                        }
                    },
                };
                engine.render_to(&format!("resources/render/render_{}.png", rendered));
                if let Err(e) = engine.next(choice) {
                    error!("Cannot continue loading script: {}", e);
                }
            }
        }
        Err(e) => {
            error!("{}", e);
            error!("Cannot initialize script.txt. Aborting.");
            exit(0);
        }
    }
}
