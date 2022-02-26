use std::{io, process::exit};

use image_rpg::{
    engine::{ScriptContext, ScriptDirective},
    Config, Engine, Scene, Size,
};
use rusttype::{Font, Scale};

macro_rules! log {
    (dbg,$x:expr) => {
        println!("[*] {}", $x);
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
    println!("[*] Discord VN scripting engine\n[*] ver 1.0.0");

    let font_data = include_bytes!("../../resources/fonts/cour.ttf");
    println!("[*] Loading font");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    println!("[*] Font loaded");

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
    println!("[*] Engine initializing, searching for {}", script_path);
    match Engine::from_file(script_path.as_str(), scene) {
        Ok(mut engine) => {
            println!("[*] Engine initialized. Rendering...");
            println!("[!] It should be noted that if there are conditional jumps in the script, you will be prompted.");
            while let Some(ctx) = engine.current() {
                let mut choice = false;
                match ctx {
                    ScriptContext::Dialogue(dialogue) => {
                        println!(
                            "[*] Rendering dialogue: \"{}\"",
                            dialogue.dialogues.iter().fold(String::new(), |a, b| a + b)
                        )
                    }
                    ScriptContext::Directive(directive) => match directive {
                        ScriptDirective::LoadBG(loadbg) => {
                            println!("[*] Loading background {}", loadbg.bg_path);
                        }
                        ScriptDirective::Jump(jump) => match &jump.choices {
                            Some((a, b)) => {
                                let mut buf = String::new();
                                loop {
                                    println!(
                                    "[?] A conditional choice was found, choose 1 or 2.\n ├──[1] {}\n └──[2] {}",
                                    a, b
                                );
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
                                            _ => println!(
                                            "[!] The choice must be either 1 or 2. Reprompting."
                                        ),
                                        },
                                        Err(_) => {
                                            println!("[!] The choice number must be an integer. Reprompting.");
                                        }
                                    }
                                    buf.clear();
                                }
                            }
                            None => println!("[*] Jumping to {}", jump.endpoint.script_path),
                        },
                        ScriptDirective::Sprite(sprite) => {
                            if let Some(sprite_path) = &sprite.sprite_path {
                                println!("[*] Loading sprite {}", sprite_path)
                            } else {
                                println!("[*] Unloading sprite {}", sprite.name)
                            }
                        }
                        ScriptDirective::Cattr(cattr) => {
                            println!("[*] Setting attribute for character \"{}\" with dialogue color \"{:?}\" and text color \"{:?}\"", cattr.character, cattr.dialogue_color, cattr.text_color);
                        }
                        ScriptDirective::Custom(custom) => {
                            println!("[*] Ignoring custom directive {:#?}", custom)
                        }
                    },
                };
                engine.render_to(&format!("resources/render/render_{}.png", rendered));
                rendered += 1;
                if let Err(e) = engine.next(choice) {
                    log!(err, e);
                }
            }
        }
        Err(e) => {
            println!("[!] {}", e);
            println!("[!] Cannot initialize script.txt. Aborting.");
            exit(0);
        }
    }
}
