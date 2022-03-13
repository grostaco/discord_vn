use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use image::{imageops::overlay, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use imageproc::drawing::draw_filled_circle_mut;
use rusttype::{point, Font, Scale};

use crate::engine::{engine::Attributes, SpriteDirective};
use log::{trace, warn};

use super::{
    draw::{as_glyphs, draw_rounded_rect, draw_words, glyphs_width, load_image},
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
    pub fn dialogue_hash(
        &self,
        bg: Option<&DynamicImage>,
        sprites: &[SpriteDirective],
        character_name: &str,
        dialogue: &str,
        attributes: &Attributes,
    ) -> u64 {
        let mut hasher = DefaultHasher::default();
        let text_color = attributes
            .get_path(&format!("character.{}.text_color", character_name))
            .map(|val| {
                let c = u32::from_str_radix(val.as_value().unwrap(), 16).unwrap();
                let a = c & 0xFF;
                let b = (c >> 8) & 0xFF;
                let g = (c >> 16) & 0xFF;
                let r = (c >> 24) & 0xFF;
                [r as u8, g as u8, b as u8, a as u8]
            })
            .unwrap_or([255, 255, 255, 255]);
        let dialogue_background = attributes
            .get_path(&format!("character.{}.dialogue_color", character_name))
            .map(|val| {
                let c = u32::from_str_radix(val.as_value().unwrap(), 16).unwrap();
                let a = c & 0xFF;
                let b = (c >> 8) & 0xFF;
                let g = (c >> 16) & 0xFF;
                let r = (c >> 24) & 0xFF;
                [r as u8, g as u8, b as u8, a as u8]
            })
            .unwrap_or([0, 0, 0, 255 / 2]);

        if let Some(bg) = bg {
            bg.hash(&mut hasher);
        }
        text_color.hash(&mut hasher);
        dialogue_background.hash(&mut hasher);

        for sprite in sprites.iter().filter(|s| s.show) {
            sprite.hash(&mut hasher);
            if let Some(Ok(scale)) = attributes
                .get_path(&format!("sprite.{}.scale", sprite.name))
                .map(|f| f.as_value().unwrap().parse::<f64>())
            {
                ((scale * 100.) as u64).hash(&mut hasher); // approximate scale as floating points have nuances making it undesirable to be hashed
            }
            // Priorities doesn't matter if sprites are loaded in the same order
            // if let Some(priority) = attributes
            //     .get_path(&format!("sprite.{}.priority", sprite.name))
            //     .map(|v| v.as_value().unwrap())
            // {
            //     priority.hash(&mut hasher);
            // }
        }
        dialogue.hash(&mut hasher);

        hasher.finish()
    }
    pub fn draw_dialogue(
        &self,
        bg: Option<&DynamicImage>,
        sprites: &[SpriteDirective],
        character_name: &str,
        dialogue: &str,
        attributes: &Attributes,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let height = v_metrics.ascent - v_metrics.descent;
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();

        let text_color = attributes
            .get_path(&format!("character.{}.text_color", character_name))
            .map(|val| {
                let c = u32::from_str_radix(val.as_value().unwrap(), 16).unwrap();
                let a = c & 0xFF;
                let b = (c >> 8) & 0xFF;
                let g = (c >> 16) & 0xFF;
                let r = (c >> 24) & 0xFF;
                [r as u8, g as u8, b as u8, a as u8]
            })
            .unwrap_or([255, 255, 255, 255]);
        let text_color = Rgba::from_slice(&text_color);

        let mut text_box: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(
            self.screen.xmax,
            self.text.ymax - self.text.ymin + height as u32 + 20,
        );

        if let Some(bg) = bg {
            let resized_bg = bg.resize_exact(
                self.screen.xmax - self.screen.xmin,
                self.screen.ymax - self.screen.ymin,
                image::imageops::FilterType::Nearest,
            );
            overlay(&mut image, &resized_bg, 0, 0);
        }

        for sprite in sprites.iter().filter(|s| s.show) {
            if let Some(sprite_path) = &sprite.sprite_path {
                let mut sprite_img = load_image(sprite_path).expect("Unable to load sprite");
                let (mut width, mut height) = sprite_img.dimensions();
                if let Some(scale) = attributes
                    .get_path(&format!("sprite.{}.scale", sprite.name))
                    .map(|f| f.as_value().unwrap().parse::<f64>())
                {
                    trace!("{}", "Scaling character");
                    if let Ok(scale) = scale {
                        sprite_img = sprite_img.resize_exact(
                            (width as f64 * scale) as u32,
                            (height as f64 * scale) as u32,
                            image::imageops::FilterType::Gaussian,
                        );
                        (width, height) = sprite_img.dimensions();
                    } else {
                        warn!("{}", "scale cannot be parsed as a float. Ignoring scaling");
                    }
                }
                let (x, y) = (sprite.x.unwrap() as i32, sprite.y.unwrap() as i32);
                let left = (width as i32 / 2 - x).max(self.screen.xmin as i32);
                let top = (height as i32 / 2 - y).max(self.screen.ymin as i32);

                let mut effective_x = sprite.x.unwrap() as i32 - width as i32 / 2;
                let mut effective_y = sprite.y.unwrap() as i32 - height as i32 / 2;
                if left > 0 || top > 0 {
                    sprite_img = sprite_img.crop_imm(left as u32, top as u32, width, height);
                    effective_x += left;
                    effective_y += top;
                }

                overlay(
                    &mut image,
                    &sprite_img,
                    effective_x as u32,
                    effective_y as u32,
                );
            }
        }
        let dialogue_background = attributes
            .get_path(&format!("character.{}.dialogue_color", character_name))
            .map(|val| {
                let c = u32::from_str_radix(val.as_value().unwrap(), 16).unwrap();
                let a = c & 0xFF;
                let b = (c >> 8) & 0xFF;
                let g = (c >> 16) & 0xFF;
                let r = (c >> 24) & 0xFF;
                [r as u8, g as u8, b as u8, a as u8]
            })
            .unwrap_or([0, 0, 0, 255 / 2]);

        draw_rounded_rect(
            &mut text_box,
            (0, height as u32 + 20),
            (
                self.text.xmax - self.text.xmin,
                self.text.ymax - self.text.ymin + height as u32 + 20,
            ),
            dialogue_background.into(),
            8,
        );

        if !character_name.is_empty() {
            let name_width = glyphs_width(&as_glyphs(
                character_name,
                &self.font,
                self.scale,
                point(0., 0.),
            ));

            let height = v_metrics.ascent - v_metrics.descent;

            draw_rounded_rect(
                &mut text_box,
                (0, 0),
                (name_width, height as u32 + 30),
                dialogue_background.into(),
                8,
            );

            draw_filled_circle_mut(
                &mut text_box,
                (name_width as i32, height as i32 + 20),
                height as i32 + 20,
                dialogue_background.into(),
            );

            draw_text(
                character_name,
                text_color,
                &mut text_box,
                &self.font,
                self.scale,
                point(15., height + 5.),
            );

            // overlay(
            //     &mut image,
            //     &character_box,
            //     self.text.xmin,
            //     self.text.ymin - height as u32 - 18,
            // );
        }
        overlay(
            &mut image,
            &text_box,
            self.text.xmin,
            self.text.ymin - height as u32 - 20,
        );

        // let name_height = {
        //     let v = self.font.v_metrics(self.scale);
        //     v.ascent - v.descent
        // };

        let mut scale = self.scale;
        let vertical_pad = v_metrics.ascent as u32;

        let ycur = self.text.ymin;
        let mut v_metrics;
        let mut glyphs_height;
        let mut whitespace_width;
        loop {
            v_metrics = self.font.v_metrics(scale);
            glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            whitespace_width = glyphs_width(
                &self
                    .font
                    .layout("_", scale, point(0., 0.))
                    .collect::<Vec<_>>(),
            );
            let (y, _) = dialogue
                .split(' ')
                .map(|word| as_glyphs(word, &self.font, scale, point(self.text.xmin as f32, 0.)))
                .fold(
                    (ycur, self.text.xmin + whitespace_width),
                    |(mut ycur, mut xcur), glyphs| {
                        if !glyphs.is_empty() {
                            let width = glyphs_width(&glyphs);
                            if xcur + width + whitespace_width > self.text.xmax - self.text.xmin {
                                ycur += glyphs_height;
                                xcur = self.text.xmin;
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
            text_color,
            &mut image,
            &self.font,
            scale,
            point(
                whitespace_width as f32 + self.text.xmin as f32,
                whitespace_width as f32 + self.text.ymin as f32 + glyphs_height as f32,
            ),
            self.text.xmax - self.text.xmin - whitespace_width,
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
