use std::{
    io::{self},
    process::exit,
};

use image_rpg::{
    engine::{ScriptContext, ScriptDirective},
    Engine, Scene, Size,
};
use rusttype::{Font, Scale};

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
    println!("[*] Engine initializing, searching for script.txt");
    if let Ok(mut engine) = Engine::from_file("script.txt", &scene) {
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
                        None => println!("[*] Jumping to {}", jump.endpoint.name),
                    },
                    ScriptDirective::Sprite(sprite) => {
                        println!("[*] Loading sprite {}", sprite.sprite_path)
                    }
                },
            };
            engine.render_to(&format!(
                "resources/render/{}_{}.png",
                engine
                    .script
                    .name
                    .get(engine.script.name.rfind("/").unwrap_or(0)..)
                    .unwrap(),
                engine.iscript
            ));
            engine.next(choice);
        }
    } else {
        println!("[!] Cannot find scripts.txt. Aborting.");
        exit(0);
    }
}
