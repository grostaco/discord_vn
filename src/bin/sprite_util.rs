use image::DynamicImage;
use nannou::{
    event::{Update, WindowEvent},
    prelude::{pt2, Point2},
    wgpu::Texture,
    App, Frame, LoopMode,
};
use nannou_conrod::{
    widget, widget_ids, Borderable, Colorable, Labelable, Positionable, RawWindowEvent, Sizeable,
    Ui, Widget,
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    index: usize,
    sprites: Vec<Sprite>,
}

struct Sprite {
    name: String,
    // original_image: DynamicImage,
    // scaled_image: Option<DynamicImage>,
    texture: Texture,
    scale: f32,
    position: Point2,
}

widget_ids! {
    struct Ids {
        scale,
        rotation,
        random_color,
        load_sprite,
        position,
    }
}

impl Sprite {
    fn new(app: &App, name: &str, image: DynamicImage, position: Point2) -> Self {
        let texture = Texture::from_image(app, &image);
        Self {
            name: name.to_string(),
            // original_image: image,
            // scaled_image: None,
            scale: 1.,
            texture,
            position,
        }
    }

    // fn get_image(&self) -> &DynamicImage {
    //     self.scaled_image.as_ref().unwrap_or(&self.original_image)
    // }

    // fn scale(&mut self, scale: f32) {
    //     self.scale = scale;
    //     if (1. - scale).abs() < f32::EPSILON {
    //         self.scaled_image = None;
    //         return;
    //     }

    //     let (width, height) = self.original_image.dimensions();
    //     self.scaled_image = Some(image::DynamicImage::ImageRgba8(resize(
    //         &self.original_image,
    //         (width as f32 * scale) as u32,
    //         (height as f32 * scale) as u32,
    //         image::imageops::FilterType::Gaussian,
    //     )));
    // }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    let w_id = app
        .new_window()
        .event(event)
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();

    let mut ui = nannou_conrod::builder(app).window(w_id).build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        ui,
        ids,
        index: 0,
        sprites: Vec::new(),
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    if let WindowEvent::DroppedFile(path) = event {
        let buffer = std::fs::read(&path).unwrap();
        let image = image::load_from_memory(&buffer).unwrap();
        model.sprites.push(Sprite::new(
            app,
            path.file_stem().unwrap().to_str().unwrap(),
            image,
            pt2(0., 0.),
        ));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    draw.rect().x(140.).y(100.).w_h(640., 480.).rgb(1., 1., 1.);

    for sprite in &model.sprites {
        let texture = &sprite.texture;
        let [width, height] = texture.size();
        let width = width as f32 * sprite.scale;
        let height = height as f32 * sprite.scale;
        let xy = sprite.position + pt2(140., 100.) - pt2(320., 240.)
            + Point2::new(width / 2., height / 2.);
        draw.texture(&texture).w_h(width, height).xy(xy);
    }
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn raw_window_event(app: &App, model: &mut Model, event: &RawWindowEvent) {
    model.ui.handle_raw_event(app, event);
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200., 30.)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1., 1., 1.)
            .border(0.0)
    }

    if let Some(sprite) = model.sprites.get_mut(model.index) {
        if let Some(value) = slider(sprite.scale, 0., 2.)
            .top_left_with_margin(20.)
            .label(&format!("Scale {:.2}", sprite.scale))
            .set(model.ids.scale, ui)
        {
            sprite.scale = value;
        }

        if let Some((x, y)) =
            widget::XYPad::new(sprite.position.x, 0., 640., sprite.position.y, 0., 480.)
                .down(10.)
                .w_h(150., 150.)
                .label("Position")
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.)
                .set(model.ids.position, ui)
        {
            sprite.position = Point2::new(x, y);
        }

        if let Some(index) = widget::DropDownList::new(
            &model.sprites.iter().map(|s| &s.name).collect::<Vec<_>>(),
            Some(model.index),
        )
        .down(10.)
        .w_h(250., 30.)
        .set(model.ids.load_sprite, ui)
        {
            model.index = index;
        }
    }
}
