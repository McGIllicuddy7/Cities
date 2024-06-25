

use crate::context::Context;
#[allow(unused)]
use crate::math::*;
#[allow(unused)]
use std::f64::consts::PI;
#[allow(unused)]
use std::f64::consts::TAU;
#[allow(unused)]
use std::ffi::c_void;

#[allow(unused)]
#[derive(Clone)]
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
                    8 as f32,
                    raylib::color::Color::BLACK.into(),
                )
            }
        }
    }
    #[allow(unused)]
    pub fn point_index(&self, point: Vector2) -> Option<usize> {
        for i in 0..self.points.len(){
            if distance(&self.points[i],&point) <1.1{
                return Some(i);
            }
        }
        return None;
    }
    #[allow(unused)]
    pub fn get_start(&self)->Vector2{
        return self.points[0];
    }
    #[allow(unused)]
    pub fn get_end(&self)->Vector2{
        return self.points[self.points.len()-1];
    }
    #[allow(unused)]
    pub fn after_start(&self)->Vector2{
        return self.points[1];
    }
    #[allow(unused)]
    pub fn after_end(&self)->Vector2{
        return self.points[self.points.len()-2];
    }
}

fn single_road_gradient(road: &Road, location: Vector2) -> Vector2 {
    let mu = 0.01;
    let base = road.distance_to(location);
    let partial_x = {
        let delta = vec2(mu, 0.0);
        let test_location0 = location + delta;
        let test_location1 = location - delta;
        let gradient_x_0 = base - road.distance_to(test_location0);
        let gradient_x_1 = base - road.distance_to(test_location1);
        vec2((gradient_x_0 - gradient_x_1) / mu, 0.0)
    };
    let partial_y = {
        let delta = vec2(0.0, mu);
        let test_location0 = location + delta;
        let test_location1 = location - delta;
        let gradient_y_0 = base - road.distance_to(test_location0);
        let gradient_y_1 = base - road.distance_to(test_location1);
        vec2(0.0, (gradient_y_0 - gradient_y_1) / mu)
    };
    return partial_x + partial_y;
}

#[allow(unused)]
fn single_road_gradient_clamped(road: &Road, location: Vector2, radius: f64) -> Vector2 {
    let mu = 0.01;
    let base = road.distance_to(location);
    if base > radius {
        return vec2(0.0, 0.0);
    }
    let partial_x = {
        let delta = vec2(mu, 0.0);
        let test_location0 = location + delta;
        let test_location1 = location - delta;
        let gradient_x_0 = base - road.distance_to(test_location0);
        let gradient_x_1 = base - road.distance_to(test_location1);
        vec2((gradient_x_0 - gradient_x_1) / mu, 0.0)
    };
    let partial_y = {
        let delta = vec2(0.0, mu);
        let test_location0 = location + delta;
        let test_location1 = location - delta;
        let gradient_y_0 = base - road.distance_to(test_location0);
        let gradient_y_1 = base - road.distance_to(test_location1);
        vec2(0.0, (gradient_y_0 - gradient_y_1) / mu)
    };
    return partial_x + partial_y;
}

#[allow(unused)]
pub fn road_gradient(roads: &[Road], location: Vector2) -> Vector2 {
    let mut out = vec2(0.0, 0.0);
    for r in roads {
        out += single_road_gradient(r, location);
    }
    return 1.0 * out;
}

#[allow(unused)]
pub fn road_gradient_clamped(roads: &[Road], location: Vector2, radius: f64) -> Vector2 {
    let mut out = vec2(0.0, 0.0);
    if roads.len() == 0 {
        return vec2(0.0, 0.0);
    }
    for i in 0..roads.len() - 1 {
        let r = &roads[i];
        out += single_road_gradient_clamped(r, location, radius);
    }
    return 1.0 * out;
}

#[allow(unused)]
fn generate_ring(radius: f64, disp: f64, resolution: f64, context: &Context) -> Road {
    let mut points: Vec<Vector2> = vec![];
    let circumference = 2.0 * PI * radius;
    let count = (circumference / resolution).floor();
    let min_r = radius - disp;
    let max_r = radius + disp;
    let theta_disp = TAU / count;
    let theta_offset = (PI / count * 500.0) as i32;
    let cx = context.width as f64 / 2.0;
    let cy = context.height as f64 / 2.0;
    for i in 0..count as i32 {
        let theta_0 = theta_disp * (i as f64);
        let d_theta =
            unsafe { raylib::ffi::GetRandomValue(-theta_offset, theta_offset) } as f64 / 1000.0;
        let theta = theta_0 + d_theta;
        let rad = unsafe { raylib::ffi::GetRandomValue(min_r as i32 * 1000, max_r as i32 * 1000) }
            as f64
            / 1000.0;
        let p = vec2(theta.cos() * rad + cx, theta.sin() * rad + cy);
        points.push(p);
    }
    points.push(points[0]);
    return Road { points: points };
}

#[allow(unused)]
fn link_points_with_road(v0: Vector2, v1: Vector2) -> Road {
    let mid  = (v0+v1)/2 as f64;
    let points = vec![v1,mid, v0];
    return Road { points: points };
}

#[allow(unused)]
fn link_roads(r0: &Road, r1: &Road) -> Vec<Road> {
    let a = {
        if r0.points.len() > r1.points.len() {
            r0
        } else {
            r1
        }
    };
    let b = {
        if r1.points.len() < r0.points.len() {
            r1
        } else {
            r0
        }
    };
    let ratio = a.points.len() as f64 / b.points.len() as f64;
    let mut out: Vec<Road> = vec![];
    for i in 0..b.points.len() {
        let idx = (i as f64 * ratio).floor() as usize;
        out.push(link_points_with_road(a.points[idx], b.points[i]));
    }
    return out;
}
#[allow(unused)]
pub struct Ring {
    pub inner: Road,
    pub outer: Road,
    pub spines: Vec<Road>,
}
#[allow(unused)]
pub fn generate_ring_system(max_radius: f64, context: &Context) -> Vec<Ring> {
    let mut out = vec![];
    let mut rings = vec![];
    let mut spines = vec![];
    let dradius = 75.0;
    let count = (max_radius / dradius) as i32;
    let disp = 20 as f64;
    let resolution = 50.0;
    let base = generate_ring(dradius / 2 as f64, disp, resolution, context);
    rings.push(base);
    for i in 1..count {
        let radius = i as f64 * dradius;
        let tmp = generate_ring(radius, disp, resolution, context);
        let new_spines = link_roads(&tmp, &rings[rings.len() - 1]);
        rings.push(tmp);
        spines.push(new_spines);
    }
    for i in 1..rings.len() {
        let tmp = Ring {
            inner: rings[i - 1].clone(),
            outer: rings[i].clone(),
            spines: spines[i - 1].clone(),
        };
        out.push(tmp);
    }
    return out;
}
#[allow(unused)]
pub fn collect_rings_to_roads(rings: &Vec<Ring>) -> Vec<Road> {
    let mut out = vec![rings[0].inner.clone(), rings[0].outer.clone()];
    out.append(&mut rings[0].spines.clone());
    for i in 1..rings.len() {
        out.push(rings[i].outer.clone());
        out.append(&mut rings[i].spines.clone());
    }
    return out;
}
#[allow(unused)]
fn segment_available_locations(_inner: &Road, _outer: &Road, lower_side: &Road, upper_side: &Road)->Vec<Rectangle> {
    let two = 2 as f64;
    let v0 = lower_side.get_start();
    let v1 = upper_side.get_start();
    let v2 =lower_side.get_end();
    let v3 =  upper_side.get_end();
    let bmid = (v0+v1)/two;
    let tmid = (v2+v3)/two;
    let lmid =(v0+v2)/two;
    let rmid = (v1+v3)/two;
    let center = (v0+v1+v2+v3)/(4 as f64);
    vec![
        rect(v0, bmid, lmid, center),
        rect(bmid, v1, center, rmid),
        rect(lmid, v2, center,tmid),
        rect(center, tmid, rmid, v3)
    ]
}
#[allow(unused)]
pub fn ring_available_locations(ring: &Ring) -> Vec<Rectangle> {
    let mut out = vec![];
    let inner = &ring.inner;
    let outer = &ring.outer;
    let len = ring.spines.len();
    for i in 0..len{
        let mut tmp = segment_available_locations(inner, outer, &ring.spines[(i+1)%len], &ring.spines[i]);
        out.append(&mut tmp);
    }
    return out;
}
