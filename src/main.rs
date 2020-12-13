mod coords;
use coords::{Polar, XY};

use nannou::prelude::*;
use nannou::state::mouse::ButtonPosition;

#[derive(Default)]
struct MouseInput {
    p1_angle_prev: f32,
    p2_angle_prev: f32,
    is_original_angle: bool,

    closer_to_p1: bool,

    pressed: bool,
    frames_pressed: f32,
}

struct Model {
    p1: Polar,
    p2: Polar,

    m1: f32,
    m2: f32,

    a1: f32,
    a2: f32,

    v1: f32,
    v2: f32,

    dampening: f32,

    gravity: f32,
    mouse: MouseInput,
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(_app: &App) -> Model {
    Model {
        p1: Polar::new(200.0, PI / 2.0),
        p2: Polar::new(200.0, PI / 2.0),

        m1: 40.0,
        m2: 40.0,

        a1: 0.0,
        a2: 0.0,

        v1: 0.0,
        v2: 0.0,

        dampening: 0.0,
        gravity: 1.0,

        mouse: MouseInput {
            p1_angle_prev: 0.0,
            p2_angle_prev: 0.0,

            closer_to_p1: false,

            is_original_angle: false,
            pressed: false,

            frames_pressed: 0.0,
        },
    }
}

fn get_polar_rel_to(x_mouse: f32, y_mouse: f32, x_start: f32, y_start: f32) -> Polar {
    let xy = XY::new(x_mouse - x_start, y_mouse - y_start);
    let polar = Polar::from_xy(xy);
    polar
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let win = app.window_rect().pad_top(100.0);

    let offset1 = m.p1.to_xy().to_nannou();
    let b1 = Rect::from_w_h(m.m1, m.m1).mid_top_of(win).shift(offset1);

    let offset2 = m.p2.to_xy().to_nannou();
    let b2 = Rect::from_w_h(m.m2, m.m2).middle_of(b1).shift(offset2);

    if let ButtonPosition::Down(orig_pos) = app.mouse.buttons.left() {
        m.mouse.frames_pressed += 1.0;
        if !m.mouse.is_original_angle {
            m.mouse.is_original_angle = true;
            m.mouse.p1_angle_prev = m.p1.angle;
            m.mouse.p2_angle_prev = m.p2.angle;
            let p1_rel = get_polar_rel_to(orig_pos.x, orig_pos.y, b1.x(), b1.y());
            let p2_rel = get_polar_rel_to(orig_pos.x, orig_pos.y, b2.x(), b2.y());
            m.mouse.closer_to_p1 = p1_rel.length < p2_rel.length;
        }

        if m.mouse.closer_to_p1 {
            m.v1 = 0.0;
            m.v2 = 0.0;

            m.a1 = 0.0;
            m.a2 = 0.0;

            let win_top_mid = win.mid_top();
            let p1 = get_polar_rel_to(app.mouse.x, app.mouse.y, win_top_mid.x, win_top_mid.y);
            let orig_angle = get_polar_rel_to(orig_pos.x, orig_pos.y, win_top_mid.x, win_top_mid.y);
            m.p1.angle = m.mouse.p1_angle_prev + orig_angle.angle - p1.angle;
            m.v1 = (orig_angle.angle - p1.angle) / m.mouse.frames_pressed;
        } else {
            m.v1 = 0.0;
            m.v2 = 0.0;

            m.a1 = 0.0;
            m.a2 = 0.0;

            let p2 = get_polar_rel_to(app.mouse.x, app.mouse.y, b1.x(), b1.y());
            let orig_angle = get_polar_rel_to(orig_pos.x, orig_pos.y, b1.x(), b1.y());
            m.p2.angle = m.mouse.p2_angle_prev + orig_angle.angle - p2.angle;
            m.v2 = (orig_angle.angle - p2.angle) / m.mouse.frames_pressed;
        }

        m.mouse.pressed = true;
    } else {
        m.mouse.is_original_angle = false;
        m.mouse.pressed = false;
        m.mouse.frames_pressed = 0.0;
    }

    if !m.mouse.pressed {
        let num = -m.gravity * (2.0 * m.m1 + m.m2) * m.p1.angle.sin()
            - m.m2 * m.gravity * (m.p1.angle - 2.0 * m.p2.angle).sin()
            - 2.0
                * (m.p1.angle - m.p2.angle).sin()
                * m.m2
                * (m.v2 * m.v2 * m.p2.length
                    + m.v1 * m.v1 * m.p1.length * (m.p1.angle - m.p2.angle).cos());
        let den =
            m.p1.length * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.p1.angle - 2.0 * m.p2.angle).cos());
        m.a1 = num / den;

        let num = 2.0
            * (m.p1.angle - m.p2.angle).sin()
            * (m.v1 * m.v1 * m.p1.length * (m.m1 + m.m2)
                + m.gravity * (m.m1 + m.m2) * m.p1.angle.cos()
                + m.v2 * m.v2 * m.p2.length * m.m2 * (m.p1.angle - m.p2.angle).cos());
        let den =
            m.p2.length * (2.0 * m.m1 + m.m2 - m.m2 * (2.0 * m.p1.angle - 2.0 * m.p2.angle).cos());
        m.a2 = num / den;

        m.v1 += m.a1;
        m.v2 += m.a2;
        m.p1.angle += m.v1;
        m.p2.angle += m.v2;
        m.v1 *= 1.0 - m.dampening;
        m.v2 *= 1.0 - m.dampening;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect().pad_top(100.0);

    let offset1 = model.p1.to_xy().to_nannou();
    let b1 = Rect::from_w_h(model.m1, model.m1)
        .mid_top_of(win)
        .shift(offset1);

    let offset2 = model.p2.to_xy().to_nannou();
    let b2 = Rect::from_w_h(model.m2, model.m2)
        .middle_of(b1)
        .shift(offset2);

    draw.line()
        .start(win.mid_top())
        .end(b1.xy())
        .stroke_weight(3.0)
        .color(GRAY);
    draw.line()
        .start(b1.xy())
        .end(b2.xy())
        .stroke_weight(3.0)
        .color(GRAY);

    draw.ellipse().xy(b1.xy()).wh(b1.wh()).color(BLACK);
    draw.ellipse().xy(b2.xy()).wh(b2.wh()).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
