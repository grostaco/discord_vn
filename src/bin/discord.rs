use std::process::exit;

use image_rpg::{Config, Handler, Scene, Size};
use rusttype::{Font, Scale};
use serenity::Client;
use songbird::SerenityInit;

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

#[tokio::main]
async fn main() {
    log!(dbg, "Discord VN engine 1.0.0");
    log!(dbg, "Loading config file resources/config.conf");

    let config = match Config::from_file("resources/config.conf") {
        Ok(conf) => conf,
        Err(err) => log!(err, err),
    };

    let font_data = include_bytes!("../../resources/fonts/Calibri Light.ttf");
    let font = Font::try_from_bytes(font_data as &[u8])
        .unwrap_or_else(|| log!(err, "Error constructing Font"));
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
            xmax: 0,
            ymin: 0,
            ymax: 0,
        },
        text: Size {
            xmin: 20,
            xmax: 620,
            ymin: 340,
            ymax: 480,
        },
    };

    let discord = config
        .fields
        .get("Discord")
        .unwrap_or_else(|| log!(err, "Discord field not found in config file"));
    let path = config
        .fields
        .get("Path")
        .unwrap_or_else(|| log!(err, "Path field not found in config file"));

    log!(dbg, "Loading application id");
    let application_id: u64 = discord
        .get("application_id")
        .unwrap_or_else(|| log!(err, "Unable to find application_id in [Discord]"))
        .parse()
        .unwrap_or_else(|_| log!(err, "application_id must be an integer"));

    log!(dbg, "Loading guild id");
    let guild_id: u64 = discord
        .get("guild_id")
        .unwrap_or_else(|| log!(err, "Unable to find guild_id in [Discord]"))
        .parse()
        .unwrap_or_else(|_| log!(err, "guild_id must be an integer"));

    let token = discord
        .get("discord_token")
        .unwrap_or_else(|| log!(err, "Unable to find discord_token in [Discord]"));

    let mut client = Client::builder(token)
        .event_handler(Handler {
            config_path: "resources/config.conf".to_owned(),
            script_path: path
                .get("script_path")
                .unwrap_or_else(|| log!(err, "Unable to find script_path in [Path]"))
                .to_owned(),
            guild_id,
            scene,
        })
        .application_id(application_id)
        .register_songbird()
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
