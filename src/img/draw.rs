use image::{imageops::overlay, io::Reader, DynamicImage, ImageBuffer, Pixel, Rgba};
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
    pub fn draw(
        &self,
        bg_path: Option<&str>,
        character_name: &str,
        dialogue: &str,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let color = Rgba::from_slice(&[255, 255, 255, 255]);

        if let Some(bg_path) = bg_path {
            let bg_img = Reader::open(bg_path)
                .expect("Cannot open background file")
                .decode()
                .unwrap();

            overlay(&mut image, &bg_img, 0, 0);
        }

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
                glyph.draw(|x, y, v| {
                    let image_x = x + bounding_box.min.x as u32;
                    let image_y = y + bounding_box.min.y as u32;

                    let pixel = image.get_pixel(image_x, image_y);
                    let pix = pixel.map2(&color, |p, q| {
                        ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                    });
                    image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        pix,
                    )
                });
            }
        }

        for glyphs in glyphs_vec {
            if glyphs.len() == 0 {
                continue;
            }
            let width = glyphs_width(&glyphs);
            if xcur + width + WHITESPACE_PAD > self.text.xmax {
                xcur = 0;
                ycur += glyphs_height;
            }

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let image_x = xcur + x + bounding_box.min.x as u32;
                        let image_y = ycur + y + bounding_box.min.y as u32;

                        let pixel = image.get_pixel(image_x, image_y);
                        let pix = pixel.map2(&color, |p, q| {
                            ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                        });
                        image.put_pixel(image_x, image_y, pix)
                    });
                }
            }
            xcur += width + WHITESPACE_PAD;
        }

        image
    }
}
