#[allow(unused)]
use rust_raylib::math::{Vector2, *};
#[allow(unused)]
pub struct LineSpline {
    pub vertices: Vec<Vector2>,
}
#[inline]
pub fn dist_to_line(point: Vector2, start: Vector2, end: Vector2) -> f32 {
    //https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    let x0 = point.x;
    let y0 = point.x;
    let x1 = start.x;
    let y1 = start.y;
    let x2 = end.x;
    let y2 = end.y;
    ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs()
        / (((y2 - y1) * (y2 - y1) + (x2 - x1) * (x2 - x1)).sqrt())
}
impl LineSpline {
    #[allow(unused)]
    pub fn new(points: &[Vector2]) -> Self {
        let mut v = Vec::new();
        for i in 0..points.len() {
            v.push(points[i]);
        }
        return Self { vertices: v };
    }
    #[allow(unused)]
    pub fn min_distance_to(&self, point: Vector2) -> f32 {
        let mut min: f32 = 0 as f32;
        for i in 0..self.vertices.len() - 1 {
            let dist = dist_to_line(point, self.vertices[i], self.vertices[i + 1]);
            min = if dist < min { dist } else { min };
        }
        return min;
    }
    #[allow(unused)]
    pub fn draw(&self) {
        for y in 0..1000 {
            for x in 0..1000 {
                let location: Vector2 = Vector2 {
                    x: x as f32,
                    y: y as f32,
                };
                if self.min_distance_to(location) < 10.0 {
                    unsafe {
                        rust_raylib::ffi::DrawPixel(x, y, rust_raylib::color::Color::RED.into());
                    }
                }
            }
        }
    }
}
