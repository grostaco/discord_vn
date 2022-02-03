use image::{DynamicImage, ImageBuffer, Pixel, Rgba};
use rusttype::{Font, Point, PositionedGlyph, Scale};

pub fn layout_width(layout: &Vec<PositionedGlyph>) -> (i32, i32) {
    let min_x = layout
        .first()
        .map(|g| g.pixel_bounding_box().unwrap().min.x)
        .unwrap();
    let max_x = layout
        .last()
        .map(|g| g.pixel_bounding_box().unwrap().max.x)
        .unwrap();
    (min_x, max_x)
}

fn draw_layout<'a>(
    image: &'a mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    layout: &Vec<PositionedGlyph>,
    color: &Rgba<u8>,
    xoffset: f32,
    yoffset: f32,
) -> &'a mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    for glyph in layout {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let image_x = ((x + bounding_box.min.x as u32) as f32 + xoffset)
                    .floor()
                    .max(0.0) as u32;
                let image_y = ((y + bounding_box.min.y as u32) as f32 + yoffset)
                    .floor()
                    .max(0.0) as u32;

                let pixel = image.get_pixel(image_x, image_y);

                let pix = pixel.map2(color, |p, q| {
                    ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                });
                image.put_pixel(image_x, image_y, pix)
            })
        }
    }
    image
}
fn main() {
    let font_data = include_bytes!("../resources/fonts/Calibri Light.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let scale = Scale::uniform(32.0);
    let mut image = DynamicImage::new_rgba8(640, 480).to_rgba8();
    image.fill(255);
    let shadow_color = &Rgba([0, 0, 0, 255]);
    let color = &Rgba([255, 255, 255, 255]);

    let text = "Outline me!";
    let layout = font
        .layout(text, scale, Point { x: 320.0, y: 240.0 })
        .collect::<Vec<_>>();

    for xc in vec![-0.75, 0.75] {
        for yc in vec![-0.75, 0.75] {
            draw_layout(&mut image, &layout, shadow_color, xc, yc);
        }
    }

    draw_layout(&mut image, &layout, color, 0.0, 0.0);

    image.save("image.png").expect("!");
}
