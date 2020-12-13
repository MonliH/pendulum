use std::f32::consts::PI;

#[derive(Debug, PartialEq)]
pub struct XY {
    x: f32,
    y: f32,
}

impl XY {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    #[allow(dead_code)]
    fn round(mut self) -> Self {
        self.x = self.x.round();
        self.y = self.y.round();
        self
    }

    pub fn to_nannou(&self) -> nannou::geom::point::Point2 {
        nannou::geom::Vector2::new(
            -self.x as nannou::geom::scalar::Default,
            -self.y as nannou::geom::scalar::Default,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Polar {
    pub length: f32,
    pub angle: f32,
}

impl Polar {
    pub fn new(length: f32, angle: f32) -> Self {
        Self { length, angle }
    }

    pub fn from_xy(xy: XY) -> Self {
        let length = (xy.x * xy.x + xy.y * xy.y).sqrt();
        let angle = xy.y.atan2(xy.x);

        Self { angle, length }
    }

    pub fn to_xy(&self) -> XY {
        let x = self.angle.sin() * self.length;
        let y = self.angle.cos() * self.length;
        XY { x, y }
    }
}

#[test]
fn test_polar() {
    let polar = Polar::new(1.0, PI / 2.0);
    assert_eq!(XY { x: 1.0, y: 0.0 }, polar.to_xy().round());
}
