use std::env;

use image_rpg::{Handler, Scene, Size};
use rusttype::{Font, Scale};
use serenity::Client;

#[tokio::main]
async fn main() {
    let font_data = include_bytes!("../resources/fonts/cour.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

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

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("Application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler { scene })
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
