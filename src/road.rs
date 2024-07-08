use crate::building::generate_building_from_rectangle;
use crate::building::Block;
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
    pub width: f64,
}

impl Road {
    #[allow(unused)]
    pub fn new(points: &[Vector2], width: f64) -> Self {
        let mut v = vec![];
        for p in points {
            v.push(*p);
        }
        Road { points: v, width }
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
        min
    }

    #[allow(unused)]
    pub unsafe fn draw(&self, context: &crate::context::Context) {
        for i in 0..self.points.len() - 1 {
            let s = self.points[i];
            let e = self.points[(i + 1) % self.points.len()];
            raylib::ffi::DrawLineEx(
                to_raylib_vec(s),
                to_raylib_vec(e),
                4_f32,
                raylib::color::Color::WHITESMOKE.into(),
            )
        }
    }

    #[allow(unused)]
    pub unsafe fn draw_as_water(&self, context: &crate::context::Context) {
        for i in 0..self.points.len() - 1 {
            let s = self.points[i];
            let e = self.points[(i + 1) % self.points.len()];
            raylib::ffi::DrawLineEx(
                to_raylib_vec(s),
                to_raylib_vec(e),
                4_f32,
                raylib::color::Color::DARKBLUE.into(),
            )
        }
    }

    #[allow(unused)]
    pub fn point_index(&self, point: Vector2) -> Option<usize> {
        for i in 0..self.points.len() {
            if distance(&self.points[i], &point) < 1.1 {
                return Some(i);
            }
        }
        None
    }

    #[allow(unused)]
    pub fn get_start(&self) -> Vector2 {
        self.points[0]
    }

    #[allow(unused)]
    pub fn get_end(&self) -> Vector2 {
        self.points[self.points.len() - 1]
    }

    #[allow(unused)]
    pub fn after_start(&self) -> Vector2 {
        self.points[1]
    }

    #[allow(unused)]
    pub fn after_end(&self) -> Vector2 {
        self.points[self.points.len() - 2]
    }

    #[allow(unused)]
    pub fn nearest_point_discrete(&self, location: &Vector2) -> Vector2 {
        let mut min = self.points[0];
        let mut min_dist = distance(&min, &location);
        for i in &self.points {
            let d = distance(i, &location);
            if d < min_dist {
                min_dist = d;
                min = *i;
            }
        }
        min
    }
    #[allow(unused)]
    pub fn get_start_offset_upper(&self) -> Vector2 {
        let v = self.after_start() - self.get_start();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -90.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_start_offset_lower(&self) -> Vector2 {
        let v = self.after_start() - self.get_start();
        let t = v.normalize();
        let rot = rotate_vec2(&t, 90.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_end_offset_upper(&self) -> Vector2 {
        let v = self.after_end() - self.get_end();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -90.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_end_offset_lower(&self) -> Vector2 {
        let v = self.after_end() - self.get_end();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -90.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_nearest_point_continuous(&self, location: Vector2) -> Vector2 {
        let mut min = self.points[0];
        let mut min_dist = distance(&min, &location);
        for i in 0..self.points.len() - 1 {
            let start = self.points[i];
            let end = self.points[i + 1];
            let p = nearest_point_on_line_segment(location, start, end);
            let d = distance(&p, &location);
            if d < min_dist {
                min = p;
                min_dist = d;
            }
        }
        min
    }
    #[allow(unused)]
    fn normal_from_start_idx(&self, idx: usize) -> Option<Vector2> {
        if idx >= self.points.len() - 1 {
            return None;
        }
        let delta = normalize(&(self.points[idx + 1] - self.points[idx]));
        return Some(rotate_vec2(&delta, 90.0));
    }
    #[allow(unused)]
    pub fn get_normal_at_location_toward(
        &self,
        location: Vector2,
        location_towards: Vector2,
    ) -> Option<Vector2> {
        let mut start_idx = 0;
        let mut end_idx = 1;
        let mut found = false;
        for i in 0..self.points.len() - 1 {
            let s = self.points[i];
            let e = self.points[i + 1];
            if is_between_points(location, s, e) || s == location || e == location {
                found = true;
                start_idx = i;
                end_idx = i + 1;
            }
        }
        if (!found) {
            println!("not found\n");
            return None;
        }
        let norm = if self.points[start_idx] == location {
            if start_idx == 0 {
                return self.normal_from_start_idx(0);
            }
            normalize(
                &(self.normal_from_start_idx(start_idx - 1)?
                    + self.normal_from_start_idx(start_idx)?),
            )
        } else if self.points[end_idx] == location {
            if end_idx == self.points.len() - 1 {
                self.normal_from_start_idx(end_idx - 1)?
            } else {
                normalize(
                    &(self.normal_from_start_idx(end_idx - 1)?
                        + self.normal_from_start_idx(end_idx)?),
                )
            }
        } else {
            self.normal_from_start_idx(start_idx)?
        };
        if dot(&norm, &(location_towards - location)) > 0.0 {
            Some(normalize(&norm))
        } else {
            Some(-normalize(&norm))
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
    partial_x + partial_y
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
    partial_x + partial_y
}

#[allow(unused)]
pub fn road_gradient(roads: &[Road], location: Vector2) -> Vector2 {
    let mut out = vec2(0.0, 0.0);
    for r in roads {
        out += single_road_gradient(r, location);
    }
    1.0 * out
}

#[allow(unused)]
pub fn road_gradient_clamped(roads: &[Road], location: Vector2, radius: f64) -> Vector2 {
    let mut out = vec2(0.0, 0.0);
    if roads.is_empty() {
        return vec2(0.0, 0.0);
    }
    for i in 0..roads.len() - 1 {
        let r = &roads[i];
        out += single_road_gradient_clamped(r, location, radius);
    }
    1.0 * out
}

#[allow(unused)]
fn generate_ring(
    radius: f64,
    disp: f64,
    resolution: f64,
    width: f64,
    tbase_noise: &NoiseGenerator2d,
    theta_noise: &NoiseGenerator2d,
    rad_noise: &NoiseGenerator2d,
    context: &Context,
) -> Road {
    let mut points: Vec<Vector2> = vec![];
    let circumference = 2.0 * PI * radius;
    let count = (circumference / resolution).floor();
    let min_r = radius - disp;
    let max_r = radius + disp;
    let theta_disp = TAU / count;
    let theta_offset = (PI / count * 2.0).round() as i32;
    let theta_base = (tbase_noise.perlin(vec2(radius.ln() * 1000.0, disp)) * 5.0
        + context.get_random_value(-314, 314) as f64 / 40000 as f64)
        * 0.0;
    let cx = context.width as f64 / 2.0;
    let cy = context.height as f64 / 2.0;
    for i in 0..count as i32 {
        let theta_0 = theta_disp * (i as f64);
        let mut d_theta = theta_noise.perlin(vec2(min_r / 100.0, theta_0))
            + context.get_random_value(-628, 628) as f64 / 20000.0 / (radius.sqrt() / 7.0);
        if d_theta > 0.5 {
            d_theta = 0.5;
        }
        let theta = theta_0 + d_theta + theta_base;
        let rad = rad_noise.perlin(vec2(min_r / 1000.0, theta))
            + context.get_random_value(min_r as i32 * 1000, max_r as i32 * 1000) as f64 / 1000.0;
        let p = vec2(theta.cos() * rad + cx, theta.sin() * rad + cy);
        points.push(p);
    }
    points.push(points[0]);
    Road { points, width }
}

#[allow(unused)]
fn link_points_with_road(v0: Vector2, v1: Vector2, width: f64) -> Road {
    let mid = (v0 + v1) / 2_f64;
    let points = vec![v1, mid, v0];
    return Road { points, width };
}

#[allow(unused)]
fn link_roads(r0: &Road, r1: &Road, context: &Context) -> Vec<Road> {
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
        let idx = (i as f64 * ratio).round() as usize % a.points.len();
        let width = {
            if context.get_random_float() > 0.8 {
                context.large_width
            } else {
                context.medium_width
            }
        };
        out.push(link_points_with_road(a.points[idx], b.points[i], width));
    }
    out
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
    let dradius = 50.0;
    let count = (max_radius / dradius) as i32;
    let disp = 10.0;
    let resolution = 50.0;
    let theta_base_noise = NoiseGenerator2d::new(5, 100.0, context);
    let theta_noise = NoiseGenerator2d::new(5, 100.0, context);
    let rad_noise = NoiseGenerator2d::new(5, 100.0, context);
    let base = generate_ring(
        dradius / 2_f64,
        disp,
        resolution,
        context.large_width,
        &theta_base_noise,
        &theta_noise,
        &rad_noise,
        context,
    );
    rings.push(base);
    for i in 1..count {
        let radius = i as f64 * dradius;
        let ring_width = {
            if context.get_random_float() > 0.8 {
                context.large_width
            } else {
                context.small_width
            }
        };
        let tmp = generate_ring(
            radius,
            disp,
            resolution,
            ring_width,
            &theta_base_noise,
            &theta_noise,
            &rad_noise,
            context,
        );
        let new_spines = link_roads(&tmp, &rings[rings.len() - 1], context);
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
    out
}

#[allow(unused)]
pub fn collect_rings_to_roads(rings: &Vec<Ring>) -> Vec<Road> {
    let mut out = vec![rings[0].inner.clone(), rings[0].outer.clone()];
    out.append(&mut rings[0].spines.clone());
    for i in 1..rings.len() {
        out.push(rings[i].outer.clone());
        out.append(&mut rings[i].spines.clone());
    }
    out
}

#[allow(unused)]
//calculates new position so that point isn't in road
fn calc_push(point: Vector2, road: &Road, center: &Vector2) -> Vector2 {
    if road.distance_to(point) > (*road).width {
        return vec2(0.0, 0.0);
    }
    let mut p = road.get_nearest_point_continuous(point);
    let dp = normalize(&(point-p));
    let dc = normalize(&(center-p));
    let point = {
        if dot(&dc, &dp)<0.0{
            point
        } else{
            point-2.0*(point-p)
        }
    };
    let norm = road
        .get_normal_at_location_toward(p, *center)
        .expect("this had better work");
    let base_dist = distance(&point, &center);
    let mut dist = road.width - (distance(&point, &p));
    if base_dist< distance(&(norm*dist), &point){
        dist = 1.0*dist;
    }
    return norm * dist;
}

#[allow(unused)]
fn scale_rect_to_roads(
    base: &Rectangle,
    top: &Road,
    bottom: &Road,
    left: &Road,
    right: &Road,
) -> Rectangle {
    let s = base.scale(0.95);
    let a = s.as_array();
    let center = s.center();
    let mut out = a;
    let mut count = 0;
    loop{
        let mut b = out;
        for i in 0..4 {
            b[i] += calc_push(b[i], top, &center);
            b[i] += calc_push(b[i], bottom, &center);
            b[i] += calc_push(b[i], left, &center);
            b[i] += calc_push(b[i], right, &center);
        }
        if distance(&Rectangle::from(b).center(),&Rectangle::from(out).center())<0.1{
            break;
        }
        out = b;
        count += 1;
        if count>4{
            break;
        }
    }
    Rectangle::from(out)
}

#[allow(unused)]
fn segment_available_locations(
    inner: &Road,
    outer: &Road,
    lower_side: &Road,
    upper_side: &Road,
    context: &Context,
) -> Vec<Rectangle> {
    let two = 2 as f64;
    let base = scale_rect_to_roads(
        &Rectangle {
            v0: lower_side.get_start(),
            v1: upper_side.get_start(),
            v2: lower_side.get_end(),
            v3: upper_side.get_end(),
        },
        &outer,
        &inner,
        &lower_side,
        &upper_side,
    );
    if context.get_random_value(0, 100) < context.whole_block_buildings_percent {
        return vec![base];
    }
    let v0 = base.v0;
    let v1 = base.v1;
    let v2 = base.v2;
    let v3 = base.v3;
    let bmid = (v0 + v1) / two;
    let tmid = {
        let idx = (outer.point_index(upper_side.get_end()).unwrap() + 1) % outer.points.len();
        let idx2 = outer.point_index(lower_side.get_end()).unwrap();
        if idx2 != idx {
            outer.points[idx]
        } else {
            (v2 + v3) / two
        }
    };
    let lmid = (v0 + v2) / two;
    let rmid = (v1 + v3) / two;
    let center = (v0 + v1 + v2 + v3) / (4_f64);
    let scaler = context.building_scale;
    vec![
        rect(v0, bmid, lmid, center).scale(scaler),
        rect(bmid, v1, center, rmid).scale(scaler),
        rect(lmid, v2, center, tmid).scale(scaler),
        rect(center, tmid, rmid, v3).scale(scaler),
    ]
}

#[allow(unused)]
pub fn ring_available_locations(ring: &Ring, context: &Context) -> Vec<Block> {
    let mut out = vec![];
    let inner = &ring.inner;
    let outer = &ring.outer;
    let len = ring.spines.len();
    for i in 0..len {
        let tmp = Block {
            buildings: segment_available_locations(
                inner,
                outer,
                &ring.spines[(i + 1) % len],
                &ring.spines[i],
                context,
            )
            .into_iter()
            .map(|x| generate_building_from_rectangle(x.scale(0.9)))
            .collect(),
        };
        out.push(tmp);
    }
    out
}
