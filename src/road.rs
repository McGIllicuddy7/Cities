use crate::context::Context;

use crate::math::*;
#[allow(unused)]
use std::ffi::c_void;
#[allow(unused)]
pub struct Road {
    pub points: Vec<Vector2>,
}
impl Road {
    #[allow(unused)]
    pub fn new(points: &[Vector2]) -> Self {
        let mut v = vec![];
        for p in points {
            v.push(*p);
        }
        return Road { points: v };
    }
    #[allow(unused)]
    pub fn distance_to(&self, point: Vector2) -> f64 {
        let mut min = distance(&self.points[0], &point);
        for i in 0..self.points.len() - 1 {
            let dist = dist_point_to_line(point, self.points[i], self.points[i + 1]);
            if (dist < min) {
                min = dist;
            }
        }
        return min;
    }

    #[allow(unused)]
    pub fn draw(&self, context: &crate::context::Context) {
        unsafe {
            for i in 0..self.points.len() - 1 {
                let s = self.points[i];
                let e = self.points[(i + 1) % self.points.len()];
                raylib::ffi::DrawLineEx(
                    to_raylib_vec(s),
                    to_raylib_vec(e),
                    4 as f32,
                    raylib::color::Color::BLACK.into(),
                )
            }
        }
    }
}
#[allow(unused)]
pub fn generate_road(start: Vector2, context: &Context) -> Road {
    let center = vec2((context.width / 2) as f64, (context.height / 2) as f64);
    let mut points: Vec<Vector2> = vec![start];
    let mut velocity = normalize(&(center - start));
    let mut most_recent = start;
    let mut arc_length = 0.0;
    loop {
        let length = unsafe { raylib::ffi::GetRandomValue(16, 32) } as f64;
        let point = most_recent + velocity * length;
        let theta = 0.0;
        velocity = rotate_vec2(
            &velocity,
            unsafe { raylib::ffi::GetRandomValue(-314, 314) } as f64 / 600.0,
        );
        most_recent = point;
        points.push(point);
        arc_length += length;
        if arc_length > 10000.0 {
            break;
        }
    }
    return Road { points: points };
}
pub fn generate_roads(radius: f64, context: &Context) -> Vec<Road> {
    let center = vec2((context.width / 2) as f64, (context.height / 2) as f64);
    let start = {
        let theta = unsafe { (raylib::ffi::GetRandomValue(0, 628) as f64) / 100.0 };
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        vec2(x, y) + center
    };
    let mut out = vec![];
    let mut points: Vec<Vector2> = vec![start];
    let mut velocity = normalize(&(center - start));
    let mut most_recent = start;
    let mut arc_length = 0.0;
    loop {
        let length = unsafe { raylib::ffi::GetRandomValue(16, 32) } as f64;
        let point = most_recent + velocity * length;
        let theta = 0.0;
        velocity = rotate_vec2(
            &velocity,
            unsafe { raylib::ffi::GetRandomValue(-314, 314) } as f64 / 600.0,
        );
        most_recent = point;
        points.push(point);
        arc_length += length;
        if arc_length > 10000.0 {
            break;
        }
    }

    return out;
}
