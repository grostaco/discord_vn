use image::{DynamicImage, GenericImageView};
use nannou::{
    color::Rgb,
    event::{Update, WindowEvent},
    geom::Rect,
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
    original_image: DynamicImage,
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
    fn new(name: &str, image: DynamicImage, position: Point2) -> Self {
        Self {
            name: name.to_string(),
            original_image: image,
            scale: 1.,
            position,
        }
    }

    fn get_scaled(&self) -> (f32, f32) {
        let (x, y) = self.original_image.dimensions();
        (x as f32 * self.scale, y as f32 * self.scale)
    }

    fn get_image(&self) -> &DynamicImage {
        &self.original_image
    }

    fn get_texture(&self, app: &App) -> Option<Texture> {
        Some(Texture::from_image(app, self.get_image()))
    }

    fn set_position(&mut self, xy: Point2) {
        self.position = xy;
    }
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

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    if let WindowEvent::DroppedFile(path) = event {
        let buffer = std::fs::read(&path).unwrap();
        let image = image::load_from_memory(&buffer).unwrap();
        model.sprites.push(Sprite::new(
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
        if let Some(texture) = sprite.get_texture(app) {
            let (width, height) = sprite.get_scaled();
            draw.scissor(Rect::from_x_y_w_h(140., -100., 640., 480.))
                .texture(&texture)
                .w_h(width as f32 * sprite.scale, height as f32 * sprite.scale)
                .xy(sprite.position + pt2(140., 100.) - pt2(320., 240.));
            draw.ellipse()
                .resolution(16.)
                .radius(4.)
                .color(Rgb::from_components((1., 0.3, 0.3)))
                .xy(sprite.position + pt2(140., 100.) - pt2(320., 240.));
        }
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
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.)
                .set(model.ids.position, ui)
        {
            sprite.set_position(Point2::new(x.round(), y.round()));
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
