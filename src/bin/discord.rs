use std::process::exit;

use image_rpg::{Config, Handler, Scene, Size};
use log::{debug, error, info};
use rusttype::{Font, Scale};
use serenity::Client;
use songbird::SerenityInit;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("{}", "Discord VN engine 1.0.0");
    debug!("{}", "Loading config file resources/config.conf");

    let config = match Config::from_file("resources/config.conf") {
        Ok(conf) => conf,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    let font_data = include_bytes!("../../resources/fonts/Calibri Light.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap_or_else(|| {
        error!("{}", "Error constructing Font");
        exit(1);
    });
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

    let discord = config.fields.get("Discord").unwrap_or_else(|| {
        error!("{}", "Discord field not found in config file");
        exit(1);
    });
    let path = config.fields.get("Path").unwrap_or_else(|| {
        error!("{}", "Path field not found in config file");
        exit(1);
    });

    debug!("{}", "Loading application id");
    let application_id: u64 = discord
        .get("application_id")
        .unwrap_or_else(|| {
            error!("{}", "Unable to find application_id in [Discord]");
            exit(1);
        })
        .parse()
        .unwrap_or_else(|_| {
            error!("{}", "application_id must be an integer");
            exit(1);
        });

    debug!("{}", "Loading guild id");
    let guild_id: u64 = discord
        .get("guild_id")
        .unwrap_or_else(|| {
            error!("{}", "Unable to find guild_id in [Discord]");
            exit(1);
        })
        .parse()
        .unwrap_or_else(|_| {
            error!("{}", "guild_id must be an integer");
            exit(1);
        });

    let token = discord.get("discord_token").unwrap_or_else(|| {
        error!("{}", "Unable to find discord_token in [Discord]");
        exit(1);
    });

    let mut client = Client::builder(token)
        .event_handler(Handler {
            config_path: "resources/config.conf".to_owned(),
            script_path: path
                .get("script_path")
                .unwrap_or_else(|| {
                    error!("{}", "Unable to find script_path in [Path]");
                    exit(1);
                })
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
