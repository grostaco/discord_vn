use image_rpg::{Scene, Size};
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

    let image = s.draw_dialogue( Some("resources/bg1.png"), "Frog", "AAAAAAAAAAAAAAAAAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA AAAAAAAAA");
    image.save("image_dialogue.png").unwrap();
    let image = s.draw_choice(Some("resources/bg1.png"), &("Choice one", "Choice two"));
    image.save("image_choice.png").unwrap();
}
