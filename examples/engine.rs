use image_rpg::{Engine, Scene, Size};
use rusttype::{Font, Scale};

fn main() {
    let font_data = include_bytes!("../resources/fonts/cour.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let s = Scene {
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

    let mut engine = Engine::from_file("resources/script.txt", s).expect("Cannot construct engine");

    while engine.current().is_some() {
        engine.render_to(&format!(
            "resources/render/{}_{}.png",
            engine
                .script
                .name
                .get(engine.script.name.rfind('/').unwrap()..)
                .unwrap(),
            engine.iscript
        ));
        engine.next(true).unwrap();
    }
}
