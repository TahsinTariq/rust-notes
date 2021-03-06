// https://dev.to/deciduously/creative-coding-in-rust-with-nannou-1lbl
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    r: f32,
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

fn model(app: &App) -> Model {
    let r = 50.0;
    let x = 100.0;
    let y = 100.0;
    let x_speed = 2.5;
    let y_speed = 2.0;

    let _window = app.new_window().size(640, 480).view(view).build().unwrap();
    Model {
        r,
        x,
        y,
        x_speed,
        y_speed,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Add the current speed to the position
    model.x += model.x_speed;
    model.y += model.y_speed;

    let win_rect = app.window_rect();

    if (model.x > win_rect.right()) || (model.x < win_rect.left()) {
        model.x_speed *= -1.0;
    }
    if (model.y > win_rect.top()) || (model.y < win_rect.bottom()) {
        model.y_speed *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(model.r, model.r)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
