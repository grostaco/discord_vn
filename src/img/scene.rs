use image::{imageops::overlay, io::Reader, DynamicImage, ImageBuffer, Pixel, Rgba};
use rusttype::{point, Font, Point, Scale};

use crate::engine::SpriteDirective;

use super::{
    draw::{as_glyphs, draw_words, glyphs_width, load_sprite},
    draw_text,
    size::Size,
};

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
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let white = Rgba::from_slice(&[255, 255, 255, 255]);
        let black = Rgba::from_slice(&[0, 0, 0, 255]);

        if let Some(bg_path) = bg_path {
            let bg_img = load_sprite(bg_path).expect("Unable to load sprite");
            overlay(&mut image, &bg_img, 0, 0);
        }

        for sprite in sprites {
            let sprite_img = load_sprite(&sprite.sprite_path).expect("Unable to load sprite");
            overlay(&mut image, &sprite_img, sprite.x, sprite.y);
        }

        draw_text(
            character_name,
            white,
            black,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.text.xmin as f32,
                self.text.ymin as f32 + v_metrics.ascent,
            ),
        );

        draw_words(
            dialogue,
            white,
            black,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.text.xmin as f32,
                self.text.ymin as f32 + v_metrics.ascent * 3.0,
            ),
            self.text.xmax,
        );

        image
    }

    pub fn draw_choice(
        &self,
        bg_path: Option<&str>,
        choices: &(&str, &str),
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let v_metrics = self.font.v_metrics(self.scale);
        let mut image = DynamicImage::new_rgba8(self.screen.xmax, self.screen.ymax).to_rgba8();
        let black = Rgba::from_slice(&[0, 0, 0, 255]);
        let white = Rgba::from_slice(&[255, 255, 255, 255]);

        if let Some(bg_path) = bg_path {
            let bg_img = load_sprite(bg_path).expect("Unable to load sprite");
            overlay(&mut image, &bg_img, 0, 0);
        }

        let a_width = glyphs_width(&as_glyphs(
            choices.0,
            &self.font,
            self.scale,
            Point { x: 0.0, y: 0.0 },
        ));
        let adjusted_a_width = glyphs_width(&as_glyphs(
            choices.0,
            &self.font,
            Scale {
                x: self.scale.x * 1.05,
                y: self.scale.y * 1.05,
            },
            Point { x: 0.0, y: 0.0 },
        ));

        //let b_width = glyphs_width(&choice_b_glyphs);

        draw_text(
            choices.0,
            &white,
            &black,
            &mut image,
            &self.font,
            self.scale,
            point(
                self.screen.xmax as f32 / 2.0 - a_width as f32 / 2.0,
                self.screen.ymax as f32 / 4.0,
            ),
        );

        /*
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
        */
        image
    }
}
