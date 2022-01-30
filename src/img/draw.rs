use image::{DynamicImage, ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};

use super::{glyphs_width, size::Size};

const WHITESPACE_PAD: u32 = 20;

pub struct Scene<'a> {
    pub font: Font<'a>,
    pub scale: Scale,
    pub screen: Size,
    pub sprite: Size,
    pub text: Size,
}

impl<'a> Scene<'a> {
    pub fn draw(&self, character_name: &str, dialogue: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();

        let character_name_glyphs = self
            .font
            .layout(
                character_name,
                self.scale,
                point(
                    self.text.xmin as f32,
                    self.text.ymin as f32 + v_metrics.ascent,
                ),
            )
            .collect::<Vec<_>>();

        let glyphs_vec = dialogue
            .split(" ")
            .map(|word| {
                self.font
                    .layout(
                        word,
                        self.scale,
                        point(
                            self.text.xmin as f32,
                            self.text.ymin as f32 + v_metrics.ascent * 3.0,
                        ),
                    )
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (mut xcur, mut ycur) = (0, 0);

        for glyph in character_name_glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([255, 255, 255, (v * 255.0) as u8]),
                    )
                });
            }
        }

        for glyphs in glyphs_vec {
            let width = glyphs_width(&glyphs);
            if xcur + width + WHITESPACE_PAD > self.text.xmax {
                xcur = 0;
                ycur += glyphs_height;
            }

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        image.put_pixel(
                            // Offset the position by the glyph bounding box
                            xcur + x + bounding_box.min.x as u32,
                            ycur + y + bounding_box.min.y as u32,
                            // Turn the coverage into an alpha value
                            Rgba([255, 255, 255, (v * 255.0) as u8]),
                        )
                    });
                }
            }
            xcur += width + WHITESPACE_PAD;
        }

        image
    }
}
