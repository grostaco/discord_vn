use image::{imageops::overlay, io::Reader, DynamicImage, ImageBuffer, Pixel, Rgba};
use rusttype::{point, Font, Scale};

use crate::engine::SpriteDirective;

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
    pub fn draw_dialogue(
        &self,
        bg_path: Option<&str>,
        sprites: Vec<&SpriteDirective>,
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

        for sprite in sprites {
            let sprite_img = Reader::open(&sprite.sprite_path)
                .expect("Cannot load sprite")
                .decode()
                .unwrap();

            overlay(&mut image, &sprite_img, sprite.x, sprite.y);
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

    pub fn draw_choice(
        &self,
        bg_path: Option<&str>,
        choices: &(&str, &str),
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let color = Rgba::from_slice(&[255, 255, 255, 255]);

        if let Some(bg_path) = bg_path {
            let bg_img = Reader::open(bg_path)
                .expect("Cannot open background file")
                .decode()
                .unwrap();

            overlay(&mut image, &bg_img, 0, 0);
        }

        let choice_a_glyphs = self
            .font
            .layout(
                choices.0,
                self.scale,
                point(self.screen.xmax as f32 / 2.0, self.screen.ymax as f32 / 4.0),
            )
            .collect::<Vec<_>>();
        let choice_b_glyphs = self
            .font
            .layout(
                choices.1,
                self.scale,
                point(
                    self.screen.xmax as f32 / 2.0,
                    self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0,
                ),
            )
            .collect::<Vec<_>>();

        let a_width = glyphs_width(&choice_a_glyphs);
        let b_width = glyphs_width(&choice_b_glyphs);

        for glyph in &choice_a_glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let image_x = x + bounding_box.min.x as u32;
                    let image_y = y + bounding_box.min.y as u32;

                    let pixel = image.get_pixel(image_x - a_width / 2, image_y);
                    let pix = pixel.map2(&color, |p, q| {
                        ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                    });
                    image.put_pixel(
                        x + bounding_box.min.x as u32 - a_width / 2,
                        y + bounding_box.min.y as u32,
                        pix,
                    );
                });
            }
        }
        let min_x = choice_a_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap() as u32;
        let max_y = choice_a_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().max.y)
            .unwrap() as u32;

        for x in min_x - a_width / 2..self.screen.xmax - (min_x - a_width / 2) {
            image.put_pixel(x, max_y + (v_metrics.ascent * 0.5) as u32, *color);
        }

        for glyph in &choice_b_glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let image_x = x + bounding_box.min.x as u32;
                    let image_y = y + bounding_box.min.y as u32;

                    let pixel = image.get_pixel(image_x - b_width / 2, image_y);
                    let pix = pixel.map2(&color, |p, q| {
                        ((p as f32 * (1.0 - v) + q as f32 * v) as u8).clamp(0, 255)
                    });
                    image.put_pixel(
                        x + bounding_box.min.x as u32 - b_width / 2,
                        y + bounding_box.min.y as u32,
                        pix,
                    );
                });
            }
        }

        let min_x = choice_b_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap() as u32;
        let max_y = choice_b_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().max.y)
            .unwrap() as u32;

        for x in min_x - b_width / 2..self.screen.xmax - (min_x - b_width / 2) {
            image.put_pixel(x, max_y + (v_metrics.ascent * 0.5) as u32, *color);
        }

        image
    }
}
