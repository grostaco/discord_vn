use image::{io::Reader, DynamicImage, ImageBuffer, ImageError, Pixel, Rgba};
use rusttype::{Font, Point, PositionedGlyph, Scale};

const WHITESPACE_PAD: u32 = 20;

pub fn draw_words<'a, 'i>(
    text: &str,
    color: &Rgba<u8>,
    shadow_color: &Rgba<u8>,
    image: &'i mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font<'a>,
    scale: Scale,
    point: Point<f32>,
    xmax: u32,
) -> &'i mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

    let glyphs_vec: Vec<_> = text
        .split(" ")
        .map(|word| font.layout(word, scale, point).collect::<Vec<_>>())
        .collect();

    let (mut xcur, mut ycur) = (0, 0);
    for glyphs in glyphs_vec {
        if glyphs.len() != 0 {
            let width = glyphs_width(&glyphs);

            if xcur + width + WHITESPACE_PAD > xmax {
                xcur = 0;
                ycur += glyphs_height;
            }

            for xc in vec![-0.75, 0.75] {
                for yc in vec![-0.75, 0.75] {
                    draw_layout(
                        image,
                        &glyphs,
                        shadow_color,
                        xcur as f32 + xc as f32,
                        ycur as f32 + yc as f32,
                    );
                }
            }

            draw_layout(image, &glyphs, color, xcur as f32, ycur as f32);

            xcur += width + WHITESPACE_PAD;
        }
    }

    image
}

pub fn draw_text<'a, 'i>(
    text: &str,
    color: &Rgba<u8>,
    shadow_color: &Rgba<u8>,
    image: &'i mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font<'a>,
    scale: Scale,
    point: Point<f32>,
) -> &'i mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    let glyphs: Vec<_> = font.layout(text, scale, point).collect();

    for xc in vec![-1, 1] {
        for glyph in &glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let image_x = ((x + bounding_box.min.x as u32) as i32 + xc) as u32;
                    let image_y = y + bounding_box.min.y as u32;

                    let pixel = image.get_pixel(image_x, image_y);
                    let pix = pixel.map2(shadow_color, |p, q| {
                        ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                    });
                    image.put_pixel(image_x, image_y, pix)
                })
            };
        }
    }

    for yc in vec![-1, 1] {
        for glyph in &glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let image_x = x + bounding_box.min.x as u32;
                    let image_y = ((y + bounding_box.min.y as u32) as i32 + yc) as u32;

                    let pixel = image.get_pixel(image_x, image_y);
                    let pix = pixel.map2(shadow_color, |p, q| {
                        ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                    });
                    image.put_pixel(image_x, image_y, pix)
                })
            };
        }
    }

    for glyph in &glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let image_x = x + bounding_box.min.x as u32;
                let image_y = y + bounding_box.min.y as u32;

                let pixel = image.get_pixel(image_x, image_y);
                let pix = pixel.map2(color, |p, q| {
                    ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                });
                image.put_pixel(image_x, image_y, pix)
            })
        };
    }

    image
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

#[inline]
fn layout_width(layout: &Vec<PositionedGlyph>) -> (i32, i32) {
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

#[inline]
pub fn glyphs_width(glyphs: &Vec<PositionedGlyph>) -> u32 {
    let (min_x, max_x) = layout_width(glyphs);
    (max_x - min_x) as u32
}

pub fn as_glyphs<'a>(
    text: &str,
    font: &'a Font<'a>,
    scale: Scale,
    point: Point<f32>,
) -> Vec<PositionedGlyph<'a>> {
    font.layout(text, scale, point).collect::<Vec<_>>()
}

pub fn load_sprite(path: &str) -> Result<DynamicImage, ImageError> {
    Reader::open(path).map_or_else(
        |_e| panic!("Cannot open background file {}", path),
        |r| r.decode(),
    )
}
