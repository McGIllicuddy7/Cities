use raylib::ffi::GetRandomValue;

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
pub fn generate_road(start: Vector2, context: &Context) -> Road {
    let center = vec2((context.width / 2) as f64, (context.height / 2) as f64);
    let mut points: Vec<Vector2> = vec![start];
    let mut velocity = normalize(&(center - start));
    let mut most_recent = start;
    let mut arc_length = 0.0;
    loop {
        let length = unsafe { raylib::ffi::GetRandomValue(16, 32) } as f64;
        let point = most_recent + velocity * length;
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

fn figure_out_direction(locations: &[Road], input_vec: Vector2, location: Vector2) -> Vector2 {
    let grad = road_gradient_clamped(locations, location, 50.0);
    let mut l = length(&grad);
    l *= l * l * l;
    if l > 1.0 {
        l = 1.0;
    }
    let theta0 = angle(&-grad, &input_vec);
    let theta1 = unsafe { GetRandomValue(-3140, 3140) } as f64 / 3000.0;
    let theta = theta0 * l + theta1 * (1.0 - l);
    return rotate_vec2(&input_vec, theta);
}

fn generate_roads_internal(
    start: Vector2,
    radius: f64,
    context: &Context,
    in_velocity: Vector2,
    depth: u64,
    inroads: &[Road],
) -> Vec<Road> {
    println!("{}", depth);
    let center = vec2((context.width / 2) as f64, (context.height / 2) as f64);
    let mut out: Vec<Road> = vec![];
    for r in inroads {
        out.push(r.clone());
    }
    let mut points: Vec<Vector2> = vec![start];
    let mut velocity = in_velocity;
    let mut most_recent = start;
    let mut arc_length = 0.0;
    let mut split_count = 0;
    loop {
        let length = unsafe { raylib::ffi::GetRandomValue(32, 64) } as f64;
        let point = most_recent + velocity * length;
        most_recent = point;
        velocity = figure_out_direction(out.as_slice(), velocity, most_recent);
        points.push(point);
        arc_length += length;
        if arc_length > 1000.0 || distance(&center, &most_recent) > radius {
            break;
        }
        if arc_length > 800. {
            let d = distance(&center, &most_recent) / radius;
            if d < unsafe { GetRandomValue(0, 100) as f64 / 100. } {
                break;
            }
        }
        if depth < 4 {
            if arc_length / split_count as f64 > 150.0 {
                if unsafe { GetRandomValue(0, depth as i32 * 5 + 1) } < 1 {
                    let mut tmp = generate_roads_internal(
                        most_recent,
                        radius,
                        context,
                        rotate_vec2(
                            &velocity,
                            3.141592 / 2.0
                                * unsafe { raylib::ffi::GetRandomValue(0, 2) * 2 - 1 } as f64,
                        ),
                        depth + 1,
                        out.as_slice(),
                    );
                    split_count += 1;
                    out.append(&mut tmp);
                }
            }
        }
    }
    let tmp = Road { points: points };
    out.push(tmp);
    return out;
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
    let points = vec![v0, v1];
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
pub fn generate_roads_by_ring_system(max_radius: f64, context: &Context) -> Vec<Road> {
    let mut out = vec![];
    let mut spines = vec![];
    let dradius = 75.0;
    let count = (max_radius / dradius) as i32;
    let disp = 20 as f64;
    let resolution = 50.0;
    let base = generate_ring(dradius / 2 as f64, disp, resolution, context);
    out.push(base);
    for i in 1..count {
        let radius = i as f64 * dradius;
        let tmp = generate_ring(radius, disp, resolution, context);
        let mut new_spines = link_roads(&tmp, &out[out.len() - 1]);
        spines.append(&mut new_spines);
        out.push(tmp);
    }
    out.append(&mut spines);
    return out;
}
#[allow(unused)]
pub fn generate_roads(radius: f64, context: &Context) -> Vec<Road> {
    let center = vec2((context.width / 2) as f64, (context.height / 2) as f64);
    let start = {
        let theta = unsafe { (raylib::ffi::GetRandomValue(0, 628) as f64) / 100.0 };
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        vec2(x, y) + center
    };
    let velocity = normalize(&(center - start));
    let mut locs = vec![];
    let out = generate_roads_internal(start, radius, context, velocity, 0, &mut locs);
    println!("finished generation");
    return out;
}
