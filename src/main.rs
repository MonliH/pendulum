mod coords;
use coords::Polar;

use nannou::prelude::*;

struct Model {
    p1: Polar,
    p2: Polar,

    m1: f32,
    m2: f32,

    a1: f32,
    a2: f32,

    v1: f32,
    v2: f32,

    gravity: f32,
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(_app: &App) -> Model {
    Model {
        p1: Polar::new(200.0, PI),
        p2: Polar::new(200.0, PI / 2.0),

        m1: 20.0,
        m2: 20.0,

        a1: 0.0,
        a2: 0.0,

        v1: 0.0,
        v2: 0.0,

        gravity: 1.0,
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    let num = -m.gravity * (2.0 * m.m1 + m.m2) * m.p1.angle.sin()
        - m.m2 * m.gravity * (m.p1.angle - 2.0 * m.p2.angle).sin()
        - 2.0
            * (m.p1.angle - m.p2.angle).sin()
            * m.m2
            * (m.v2 * m.v2 * m.p2.length
                + m.v1 * m.v1 * m.p1.length * (m.p1.angle - m.p2.angle).cos());
    let den = m.p1.length * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.p1.angle - 2.0 * m.p2.angle));
    m.a1 = num / den;

    let num = 2.0
        * (m.p1.angle - m.p2.angle).sin()
        * (m.v1 * m.v1 * m.p1.length * (m.m1 + m.m2)
            + m.gravity * (m.m1 + m.m2) * m.p1.angle.cos()
            + m.v2 * m.v2 * m.p2.length * m.m2 * (m.p1.angle - m.p2.angle).cos());
    let den = m.p2.length * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.p1.angle - 2.0 * m.p2.angle));
    m.a2 = num / den;

    m.p1.angle += m.v1;
    m.p2.angle += m.v2;
    m.v1 += m.a1;
    m.v2 += m.a2;
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

    draw.line()
        .start(win.mid_top())
        .end(b1.xy())
        .stroke_weight(3.0)
        .color(BLACK);
    draw.line()
        .start(b1.xy())
        .end(b2.xy())
        .stroke_weight(3.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
