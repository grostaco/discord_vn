use image::DynamicImage;
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut, Canvas},
    rect::Rect,
};

fn draw_rounded_rect<C>(
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
            .of_size(width, height - corner_radius * 2),
        color,
    )
}

fn main() {
    let mut img = DynamicImage::new_rgba8(640, 480);

    let (x, y) = (620, 460);
    draw_rounded_rect(&mut img, (20, 300), (x, y), [0, 0, 0, 255 / 2].into(), 16);

    img.save("out.png").unwrap();
}

// canvas.draw_pixel(xc + x as u32, yc + adjusted_height + y as u32, color);
// canvas.draw_pixel(xc + y as u32, yc + adjusted_height + x as u32, color);

// canvas.draw_pixel(
//     xc - adjusted_width - x as u32,
//     yc + adjusted_height + y as u32,
//     color,
// );
// canvas.draw_pixel(
//     xc - adjusted_width - y as u32,
//     yc + adjusted_height + x as u32,
//     color,
// );

// canvas.draw_pixel(xc - y as u32, yc - x as u32, color);

// canvas.draw_pixel(xc + x as u32, yc - y as u32, color);
// canvas.draw_pixel(xc + y as u32, yc - x as u32, color);

// draw_line_segment_mut(
//     canvas,
//     (
//         top_left.0 as f32,
//         (top_left.1 - height + corner_radius * 3) as f32,
//     ),
//     (top_left.0 as f32, (bottom_right.1 - corner_radius) as f32),
//     color,
// );

//img.put_pixel(x, y, [255, 0, 0, 255].into());
//img.put_pixel(400, 460, [255, 0, 0, 255].into());
