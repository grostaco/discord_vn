use std::mem::swap;

use image::{DynamicImage, GenericImage, GenericImageView};

// taken from https://docs.rs/imageproc/latest/src/imageproc/drawing/line.rs.html#170-184
struct BresenhamLineIter {
    dx: f32,
    dy: f32,
    x: i32,
    y: i32,
    error: f32,
    end_x: i32,
    is_steep: bool,
    y_step: i32,
}

impl BresenhamLineIter {
    pub fn new(start: (f32, f32), end: (f32, f32)) -> BresenhamLineIter {
        let (mut x0, mut y0) = start;
        let (mut x1, mut y1) = end;

        let is_steep = (y1 - y0).abs() > (x1 - x0).abs();
        if is_steep {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;

        Self {
            dx,
            dy: (y1 - y0).abs(),
            x: x0 as i32,
            y: y0 as i32,
            error: dx / 2f32,
            end_x: x1 as i32,
            is_steep,
            y_step: if y0 < y1 { 1 } else { -1 },
        }
    }
}

impl Iterator for BresenhamLineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.x > self.end_x {
            None
        } else {
            let ret = if self.is_steep {
                (self.y, self.x)
            } else {
                (self.x, self.y)
            };

            self.x += 1;
            self.error -= self.dy;
            if self.error < 0f32 {
                self.y += self.y_step;
                self.error += self.dx;
            }

            Some(ret)
        }
    }
}

fn draw_line_segment(image: &mut DynamicImage, start: (f32, f32), end: (f32, f32)) {
    let (width, height) = image.dimensions();
    let line_iterator = BresenhamLineIter::new(start, end);

    for point in line_iterator {
        let (x, y) = point;
        if x >= 0 && x < width as i32 && y >= 0 && y <= height as i32 {
            image.put_pixel(x as u32, y as u32, [0, 0, 0, 255 / 2].into())
        }
    }
}

const WHITE: [u8; 4] = [255, 255, 255, 255 / 2];
const WIDTH: f32 = 530.;
const HEIGHT: f32 = 80.;
fn main() {
    let mut img = DynamicImage::new_rgba8(640, 480);

    let mut x = 0i32;
    let mut y = 40;
    let mut p = 1 - y;

    let xc = WIDTH as i32 + y + 10;
    let yc = 430i32;

    while x <= y {
        // draw_line_segment(
        //     &mut img,
        //     (xc as f32, (yc + y) as f32),
        //     ((x + xc) as f32, (y + yc) as f32),
        // );

        draw_line_segment(
            &mut img,
            ((x + xc) as f32, (y + yc) as f32),
            ((xc - x) as f32 - WIDTH, (y + yc) as f32),
        );

        draw_line_segment(
            &mut img,
            ((xc + y) as f32, (yc + x) as f32),
            ((xc - y) as f32 - WIDTH, (x + yc) as f32),
        );

        // draw_line_segment(
        //     &mut img,
        //     ((y + yc) as f32, (x + xc) as f32),
        //     ((yc + y) as f32 - WIDTH, (xc - x) as f32),
        // );

        draw_line_segment(
            &mut img,
            ((x + xc) as f32, (yc - y) as f32 - HEIGHT),
            ((xc - x) as f32 - WIDTH, (yc - y) as f32 - HEIGHT),
        );

        draw_line_segment(
            &mut img,
            ((xc + y) as f32, (yc - x) as f32 - HEIGHT),
            ((xc - y) as f32 - WIDTH, (yc - x) as f32 - HEIGHT),
        );

        // draw_line_segment(
        //     &mut img,
        //     ((x - xc) as f32, (yc - y) as f32 - HEIGHT),
        //     ((xc + x) as f32 - WIDTH, (yc - y) as f32 - HEIGHT),
        // );

        // img.put_pixel((xc + x) as u32, (yc + y) as u32, WHITE.into());
        img.put_pixel((xc + y) as u32, (yc + x) as u32, WHITE.into());
        img.put_pixel(
            (xc - y - WIDTH as i32) as u32,
            (yc + x) as u32,
            WHITE.into(),
        );

        // img.put_pixel(y as u32 + yc, x as u32 + xc as u32, WHITE.into());
        // img.put_pixel(y as u32 + yc as u32, x as u32 + xc as u32, WHITE.into());

        // img.put_pixel(y as u32 + yc as u32, xc as u32 - x as u32, WHITE.into());

        // img.put_pixel(
        //     xc as u32 - x as u32 - 200,
        //     y as u32 + yc as u32,
        //     WHITE.into(),
        // );
        // img.put_pixel(
        //     yc as u32 - y as u32 - 200,
        //     x as u32 + xc as u32,
        //     WHITE.into(),
        // );

        // img.put_pixel(x as u32 + xc as u32, yc - y as u32, WHITE.into());
        // img.put_pixel(xc as u32 - x as u32, yc - y as u32, WHITE.into());

        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
        x += 1;
    }

    for y in 0..(HEIGHT as u32) {
        draw_line_segment(
            &mut img,
            (0., 460. - HEIGHT / 2. - y as f32),
            (10. + WIDTH, 460. - HEIGHT / 2. - y as f32),
        );
    }

    img.save("out.png").unwrap();
}
