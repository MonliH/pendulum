mod coords;
use coords::Polar;

use nannou::prelude::*;

struct Model {
    p1: Polar,
    m1: f32,
    p2: Polar,
    m2: f32,
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(_app: &App) -> Model {
    Model {
        p1: Polar::new(200.0, PI),
        m1: 20.0,
        p2: Polar::new(200.0, PI/2.0),
        m2: 20.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.p1.angle += 0.01;
    model.p2.angle -= 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();

    let offset1 = model.p1.to_xy().to_nannou();
    let b1 = Rect::from_w_h(model.m1, model.m1)
        .mid_top_of(win)
        .shift(offset1);

    draw.ellipse().xy(b1.xy()).wh(b1.wh()).color(BLACK);

    let offset2 = model.p2.to_xy().to_nannou() + offset1;
    let b2 = Rect::from_w_h(model.m2, model.m2)
        .mid_top_of(win)
        .shift(offset2);

    draw.ellipse().xy(b2.xy()).wh(b2.wh()).color(BLACK);

    draw.line().start(win.mid_top()).end(b1.xy()).stroke_weight(3.0).color(BLACK);
    draw.line().start(b1.xy()).end(b2.xy()).stroke_weight(3.0).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
