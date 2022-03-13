use image::{io::Reader, DynamicImage, ImageBuffer, Pixel, Rgba};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut, Canvas},
    rect::Rect,
};
use rusttype::{Font, Point, PositionedGlyph, Scale};

use super::error::LoadImageError;

pub fn draw_words<'a, 'i>(
    text: &str,
    color: &Rgba<u8>,
    image: &'i mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font<'a>,
    scale: Scale,
    point: Point<f32>,
    xmax: u32,
) -> &'i mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

    let glyphs_vec: Vec<_> = text
        .split(' ')
        .map(|word| font.layout(word, scale, point).collect::<Vec<_>>())
        .collect();
    let whitespace_width = glyphs_width(&font.layout("_", scale, point).collect::<Vec<_>>());

    let (mut xcur, mut ycur) = (0, 0);
    for glyphs in glyphs_vec {
        if !glyphs.is_empty() {
            let width = glyphs_width(&glyphs);

            if xcur + width + whitespace_width > xmax {
                xcur = 0;
                ycur += glyphs_height;
            }

            draw_layout(image, &glyphs, color, xcur as f32, ycur as f32);

            xcur += width + whitespace_width;
        }
    }

    image
}

pub fn draw_text<'a, 'i>(
    text: &str,
    color: &Rgba<u8>,
    image: &'i mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font<'a>,
    scale: Scale,
    point: Point<f32>,
) -> &'i mut ImageBuffer<Rgba<u8>, Vec<u8>> {
    let glyphs: Vec<_> = font.layout(text, scale, point).collect();

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
    layout: &[PositionedGlyph],
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
fn layout_width(layout: &[PositionedGlyph]) -> (i32, i32) {
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
pub fn glyphs_width(glyphs: &[PositionedGlyph]) -> u32 {
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

pub fn load_image(path: &str) -> Result<DynamicImage, LoadImageError> {
    Reader::open(path)
        .map_err(LoadImageError::IoError)?
        .decode()
        .map_err(LoadImageError::ImageError)
}

pub fn draw_rounded_rect<C>(
    canvas: &mut C,
    top_left: (u32, u32),
    bottom_right: (u32, u32),
    color: C::Pixel,
    corner_radius: u32,
) where
    C: Canvas,
    C::Pixel: 'static,
{
    let mut x = 0i32;
    let mut y = corner_radius as i32;
    let mut p = 1 - y;

    let xc = bottom_right.0 - corner_radius;
    let yc = top_left.1 + corner_radius;

    let width = bottom_right.0 - top_left.0;
    let height = bottom_right.1 - top_left.1;

    let adjusted_width = width - corner_radius * 2;
    let adjusted_height = height - corner_radius * 2;

    while x <= y {
        draw_line_segment_mut(
            canvas,
            (
                xc as f32 + x as f32,
                (yc + adjusted_height) as f32 + y as f32,
            ),
            (
                (xc - adjusted_width) as f32 - x as f32,
                (yc + adjusted_height) as f32 + y as f32,
            ),
            color,
        );

        draw_line_segment_mut(
            canvas,
            (
                xc as f32 + y as f32,
                (yc + adjusted_height) as f32 + x as f32,
            ),
            (
                (xc - adjusted_width) as f32 - y as f32,
                (yc + adjusted_height) as f32 + x as f32,
            ),
            color,
        );

        draw_line_segment_mut(
            canvas,
            (
                (xc - adjusted_width) as f32 - x as f32,
                yc as f32 - y as f32,
            ),
            (xc as f32 + x as f32, yc as f32 - y as f32),
            color,
        );

        draw_line_segment_mut(
            canvas,
            (
                (xc - adjusted_width) as f32 - y as f32,
                yc as f32 - x as f32,
            ),
            (xc as f32 + y as f32, yc as f32 - x as f32),
            color,
        );

        canvas.draw_pixel(xc - adjusted_width - x as u32, yc - y as u32, color);
        canvas.draw_pixel(xc - adjusted_width - y as u32, yc - x as u32, color);

        canvas.draw_pixel(xc + x as u32, yc - y as u32, color);
        canvas.draw_pixel(xc + y as u32, yc - x as u32, color);

        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
        x += 1;
    }

    draw_filled_rect_mut(
        canvas,
        Rect::at(top_left.0 as i32, (top_left.1 + corner_radius) as i32)
            .of_size(width + 1, height - corner_radius * 2),
        color,
    )
}
