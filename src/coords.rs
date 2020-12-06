#[derive(Debug, PartialEq)]
pub struct XY {
    x: f32,
    y: f32,
}

impl XY {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn round(mut self) -> Self {
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

    pub fn to_xy(&self) -> XY {
        let x = self.angle.sin() * self.length;
        let y = self.angle.cos() * self.length;
        XY { x, y }
    }
}

#[test]
fn test_polar() {
    use std::f32::consts::PI;
    let polar = Polar::new(1.0, PI / 2.0);
    assert_eq!(XY::new(1.0, 0.0), polar.to_xy().round());
}
