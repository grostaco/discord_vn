use image::{imageops::overlay, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use rusttype::{point, Font, Scale};

use crate::engine::{engine::Attributes, SpriteDirective};
use log::{trace, warn};

use super::{
    draw::{as_glyphs, draw_words, glyphs_width, load_image},
    draw_text,
    size::Size,
};

#[derive(Clone, Debug)]
pub struct Scene {
    pub font: Font<'static>,
    pub scale: Scale,
    pub screen: Size,
    pub sprite: Size,
    pub text: Size,
}

impl Scene {
    pub fn draw_dialogue(
        &self,
        bg: Option<&DynamicImage>,
        sprites: Vec<&SpriteDirective>,
        character_name: &str,
        dialogue: &str,
        attributes: &Attributes,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();

        let white = Rgba::from_slice(&[255, 255, 255, 255]);

        let mut text_box: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.screen.xmax, self.text.ymax - self.text.ymin);

        if let Some(bg) = bg {
            let resized_bg = bg.resize_exact(
                self.screen.xmax - self.screen.xmin,
                self.screen.ymax - self.screen.ymin,
                image::imageops::FilterType::Gaussian,
            );
            overlay(&mut image, &resized_bg, 0, 0);
        }

        for sprite in sprites {
            if let Some(sprite_path) = &sprite.sprite_path {
                let mut sprite_img = load_image(sprite_path).expect("Unable to load sprite");
                let (width, height) = sprite_img.dimensions();
                if let Some(scale) = attributes
                    .get_path(&format!("sprite.{}.scale", character_name))
                    .map(|f| f.as_value().unwrap().parse::<f64>())
                {
                    trace!("{}", "Scaling character");
                    if let Ok(scale) = scale {
                        sprite_img = sprite_img.resize_exact(
                            (width as f64 * scale) as u32,
                            (height as f64 * scale) as u32,
                            image::imageops::FilterType::Gaussian,
                        );
                    } else {
                        warn!("{}", "scale cannot be parsed as a float. Ignoring scaling");
                    }
                }
                overlay(
                    &mut image,
                    &sprite_img,
                    (sprite.x.unwrap() - width / 2).clamp(self.screen.xmin, self.screen.xmax),
                    (sprite.y.unwrap() - height / 2).clamp(self.screen.xmin, self.screen.xmax),
                );
            }
        }
        let dialogue_background = attributes
            .get(character_name)
            .map(|val| {
                let c = u32::from_str_radix(val.as_value().unwrap(), 16).unwrap();
                let a = c & 0xFF;
                let b = (c >> 8) & 0xFF;
                let g = (c >> 16) & 0xFF;
                let r = (c >> 24) & 0xFF;
                [r as u8, g as u8, b as u8, a as u8]
            })
            .unwrap_or([0, 0, 0, 255 / 2]);
        for pixel in text_box.pixels_mut() {
            pixel.0 = dialogue_background;
        }

        overlay(
            &mut image,
            &text_box,
            0,
            self.text.ymin - (v_metrics.ascent) as u32,
        );

        draw_text(
            character_name,
            white,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.text.xmin as f32,
                self.text.ymin as f32 + v_metrics.ascent,
            ),
        );

        let name_height = {
            let v = self.font.v_metrics(self.scale);
            v.ascent - v.descent
        };

        let mut scale = self.scale;
        let vertical_pad = (v_metrics.ascent * {
            if character_name.is_empty() {
                1.0
            } else {
                3.0
            }
        })
        .ceil() as u32;

        let ycur = self.text.ymin;
        let mut v_metrics;
        let mut glyphs_height;
        loop {
            v_metrics = self.font.v_metrics(scale);
            glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            let whitespace_width = glyphs_width(
                &self
                    .font
                    .layout("_", scale, point(0., 0.))
                    .collect::<Vec<_>>(),
            );
            let (y, _) = dialogue
                .split(' ')
                .map(|word| as_glyphs(word, &self.font, scale, point(0., 0.)))
                .fold(
                    (ycur, self.text.xmin + whitespace_width),
                    |(mut ycur, mut xcur), glyphs| {
                        if !glyphs.is_empty() {
                            let width = glyphs_width(&glyphs);
                            if xcur + width + whitespace_width > self.text.xmax {
                                ycur += glyphs_height;
                                xcur = 0;
                            }
                            xcur += width + whitespace_width;
                        }

                        (ycur, xcur)
                    },
                );
            if y + vertical_pad + glyphs_height * 2 < self.text.ymax {
                break;
            }
            scale = Scale::uniform(scale.x * 0.95);
        }

        draw_words(
            dialogue,
            white,
            &mut image,
            &self.font,
            scale,
            point(
                self.text.xmin as f32,
                self.text.ymin as f32 + {
                    if character_name.is_empty() {
                        glyphs_height as f32
                    } else {
                        name_height + glyphs_height as f32
                    }
                },
            ),
            self.text.xmax,
        );

        image
    }

    pub fn draw_choice(
        &self,
        bg: Option<&DynamicImage>,
        choices: &(&str, &str),
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyph_height = v_metrics.ascent - v_metrics.descent;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let white = Rgba::from_slice(&[255, 255, 255, 255]);

        let mut opacity_box: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(self.screen.xmax, (glyph_height * 1.5) as u32);

        for pixel in opacity_box.pixels_mut() {
            pixel.0 = [0, 0, 0, 255 / 2];
        }

        if let Some(bg) = bg {
            let resized_bg = bg.resize_exact(
                self.screen.xmax - self.screen.xmin,
                self.screen.ymax - self.screen.ymin,
                image::imageops::FilterType::Gaussian,
            );
            overlay(&mut image, &resized_bg, 0, 0);
        }

        let a_glyphs = &as_glyphs(
            choices.0,
            &self.font,
            self.scale,
            point(self.screen.xmax as f32 / 2.0, self.screen.ymax as f32 / 4.0),
        );
        let a_width = glyphs_width(a_glyphs);
        let b_glyphs = &as_glyphs(
            choices.1,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0,
                self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0,
            ),
        );
        let b_width = glyphs_width(b_glyphs);

        overlay(
            &mut image,
            &opacity_box,
            0,
            (self.screen.ymax as f32 / 4.0 - glyph_height) as u32,
        );

        draw_text(
            choices.0,
            white,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0 - a_width as f32 / 2.0,
                self.screen.ymax as f32 / 4.0,
            ),
        );

        /*

        let min_x = a_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap() as u32;
        let max_y = a_glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().max.y)
            .unwrap() as u32;
        for x in min_x - a_width / 2..self.screen.xmax - (min_x - a_width / 2) {
            image.put_pixel(x, max_y + (v_metrics.ascent * 0.5) as u32, *white);
        }
        */

        overlay(
            &mut image,
            &opacity_box,
            0,
            (self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0 - glyph_height) as u32,
        );

        draw_text(
            choices.1,
            white,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0 - b_width as f32 / 2.0,
                self.screen.ymax as f32 / 4.0 + v_metrics.ascent * 5.0,
            ),
        );

        image
    }
}
