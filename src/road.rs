use crate::building::generate_building_from_rectangle;
use crate::building::Block;
use crate::context::Context;
#[allow(unused)]
use crate::math::*;
use crate::prof_frame;
use std::collections::hash_map::HashMap;
#[allow(unused)]
#[derive(Clone)]
pub struct Road {
    pub points: Vec<Vector2>,
    pub point_index_map: HashMap<(i64, i64), usize>,
    pub width: f64,
}

impl Road {
    #[allow(unused)]
    pub fn new(points: &[Vector2], width: f64) -> Self {
        prof_frame!("Road::new()");
        let mut v = vec![];
        let mut map = HashMap::new();
        let mut i = 0;
        for p in points {
            v.push(*p);
            map.insert((p.x as i64, p.y as i64), i);
            i += 1;
        }
        Road {
            points: v,
            point_index_map: map,
            width,
        }
    }

    #[allow(unused)]
    pub fn distance_to(&self, point: Vector2) -> f64 {
        prof_frame!("Road::distance_to");
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
                (self.width * 1.0) as f32,
                raylib::color::Color::WHITE.into(),
            )
        }
    }

    #[allow(unused)]
    pub unsafe fn draw_as_water(&self, context: &crate::context::Context) {
        for i in 0..self.points.len() - 1 {
            let s = self.points[i];
            let e = self.points[(i + 1) % self.points.len()];
            let col = raylib::color::Color::DARKGRAY;
            raylib::ffi::DrawCircleV(to_raylib_vec(e), (self.width / 2.0) as f32, col.into());
            raylib::ffi::DrawLineEx(
                to_raylib_vec(s),
                to_raylib_vec(e),
                self.width as f32,
                col.into(),
            )
        }
        raylib::ffi::DrawCircleV(
            to_raylib_vec(self.points[0]),
            (self.width / 2.0) as f32,
            raylib::color::Color::DARKBLUE.into(),
        );
    }

    #[allow(unused)]
    pub fn point_index(&self, point: Vector2) -> Option<usize> {
        prof_frame!("Road::point_index");
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
        prof_frame!("Road::nearest_point_discrete()");
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
    pub fn nearest_point_discrete_idx(&self, location: &Vector2) -> usize {
        prof_frame!("Road::nearest_point_discrete()");
        let mut min = 0;
        let mut min_dist = distance(&self.points[min], &location);
        for i in 0..self.points.len() {
            let d = distance(&self.points[i], &location);
            if d < min_dist {
                min_dist = d;
                min = i;
            }
        }
        min
    }
    #[allow(unused)]
    pub fn get_start_offset_upper(&self) -> Vector2 {
        let v = self.after_start() - self.get_start();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -PI / 2.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_start_offset_lower(&self) -> Vector2 {
        let v = self.after_start() - self.get_start();
        let t = v.normalize();
        let rot = rotate_vec2(&t, PI / 2.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_end_offset_upper(&self) -> Vector2 {
        let v = self.after_end() - self.get_end();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -PI / 2.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_end_offset_lower(&self) -> Vector2 {
        let v = self.after_end() - self.get_end();
        let t = v.normalize();
        let rot = rotate_vec2(&t, -PI / 2.0);
        -rot * self.width
    }
    #[allow(unused)]
    pub fn get_nearest_point_continuous(&self, location: Vector2) -> Vector2 {
        prof_frame!("Road::get_nearest_point_continuous()");
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
    fn normal_from_start_idx(&self, idx: usize) -> Vector2 {
        prof_frame!("Road::normal_from_start()");
        let delta = normalize(&(self.points[idx + 1] - self.points[idx]));
        return rotate_vec2(&delta, PI / 2.0);
    }
    #[allow(unused)]
    pub fn get_normal_at_location_toward(
        &self,
        location: Vector2,
        location_towards: Vector2,
    ) -> Vector2 {
        prof_frame!("Road::get_normal_at_location_toward");
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
            return vec2(0.0, 0.0);
        }
        let norm = if self.points[start_idx] == location {
            if start_idx == 0 {
                return self.normal_from_start_idx(0);
            }
            normalize(
                &(self.normal_from_start_idx(start_idx - 1)
                    + self.normal_from_start_idx(start_idx)),
            )
        } else if self.points[end_idx] == location {
            if end_idx == self.points.len() - 1 {
                self.normal_from_start_idx(end_idx - 1)
            } else {
                normalize(
                    &(self.normal_from_start_idx(end_idx - 1)
                        + self.normal_from_start_idx(end_idx)),
                )
            }
        } else {
            self.normal_from_start_idx(start_idx)
        };
        if dot(&norm, &(location_towards - location)) > 0.0 {
            normalize(&norm)
        } else {
            -normalize(&norm)
        }
    }
    #[allow(unused)]
    pub fn get_point_idx(&self, v: Vector2) -> Option<usize> {
        let p: (i64, i64) = (v.x.round() as i64, v.y.round() as i64);
        self.point_index_map.get(&p).map(|i| *i)
    }
    #[allow(unused)]
    pub fn are_on_same_side_of(&self, a: Vector2, b: Vector2) -> bool {
        let p0 = self.get_nearest_point_continuous(a);
        let p1 = self.get_nearest_point_continuous(b);
        let v1 = normalize(&(a - p0));
        let v2 = normalize(&(b - p1));
        dot(&v1, &v2) > 0.0
    }
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
    prof_frame!("Road::generate_ring()");
    let mut points: Vec<Vector2> = vec![];
    let circumference = 2.0 * PI * radius;
    let count = {
        let tmp_count = (circumference / resolution).floor()
            + (context.get_random_float() * context.get_random_float() * 6.0).floor();
        if tmp_count < 4.0 {
            4.0
        } else {
            tmp_count
        }
    };
    let min_r = radius - disp;
    let max_r = radius + disp;
    let theta_disp = TAU / count;
    let theta_offset = (PI / count * 2.0).round() as i32;
    let theta_base = (tbase_noise.perlin(vec2(min_r / 100.0, max_r / 100.0)) * 0.1);
    let cx = context.width as f64 / 2.0;
    let cy = context.height as f64 / 2.0;
    for i in 0..count as i32 {
        let theta_0 = theta_disp * (i as f64);
        let mut d_theta = theta_noise.perlin(vec2(min_r / 32.0, theta_0 / 8.0)) * 2.0;
        let theta = theta_0 + d_theta + theta_base;
        let rad = rad_noise.perlin(vec2(min_r / 32.0, theta / 8.0)).abs() * 160.0
            + context.get_random_value(min_r as i32 * 1000, max_r as i32 * 1000) as f64 / 1000.0;
        let p = vec2(theta.cos() * rad + cx, theta.sin() * rad + cy);
        points.push(p);
    }
    points.push(points[0]);
    Road::new(&points, width)
}

#[allow(unused)]
fn link_points_with_road(v0: Vector2, v1: Vector2, width: f64) -> Road {
    prof_frame!("Road::link_points_with_road()");
    let mid = (v0 + v1) / 2_f64;
    let points = vec![v1, mid, v0];
    Road::new(&points, width)
}

#[allow(unused)]
fn link_roads(
    r0: &Road,
    r1: &Road,
    idx: usize,
    angles: &mut [f64],
    context: &Context,
) -> Vec<Road> {
    fn is_nearest(idx: usize, points: &[Vector2], theta: f64, context: &Context) -> bool {
        let mut min = 10000.0;
        let mut min_idx = 0;
        for i in 0..points.len() {
            let phi = angle(&normalize(&(points[i] - context.center())), &vec2(1.0, 0.0));
            if (theta - phi).abs() < min {
                min = (theta - phi).abs();
                min_idx = i;
            }
        }
        let t = angle(
            &normalize(&(points[idx] - context.center())),
            &vec2(1.0, 0.0),
        );
        idx == min_idx
    }
    prof_frame!("Road::link_roads()");
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
        let theta = angle(
            &normalize(&(a.points[idx] - context.center())),
            &vec2(1.0, 0.0),
        );

        let width = {
            let mut near = false;
            for j in 0..angles.len() {
                if is_nearest(i, &a.points, angles[j], context) {
                    near = true;
                    break;
                }
            }
            if near {
                context.large_width
            } else {
                context.small_width
            }
        };
        out.push(link_points_with_road(a.points[idx], b.points[i], width));
    }
    out
}

#[allow(unused)]
#[derive(Clone)]
pub struct Ring {
    pub inner: Road,
    pub outer: Road,
    pub spines: Vec<Road>,
}

#[allow(unused)]
pub fn generate_ring_system(max_radius: f64, context: &Context) -> Vec<Ring> {
    prof_frame!("road::generate_ring_system()");
    let mut out = vec![];
    let mut rings = vec![];
    let mut spines = vec![];
    let dradius = 50.0;
    let count = (max_radius / dradius) as i32;
    let disp = 10.0;
    let resolution = 40.0;
    let theta_base_noise = NoiseGenerator2d::new(5, 100.0, context);
    let theta_noise = NoiseGenerator2d::new(5, 100.0, context);
    let rad_noise = NoiseGenerator2d::new(4, 50.0, context);
    let base = generate_ring(
        dradius / 2_f64,
        disp,
        resolution,
        context.small_width,
        &theta_base_noise,
        &theta_noise,
        &rad_noise,
        context,
    );
    let mut idxes = vec![];
    rings.push(base);
    let base_idx = context.get_random_value(0, 10000) as usize;
    for i in 1..count {
        let radius = i as f64 * dradius;
        let ring_width = {
            if i % 5 == 0 {
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
        if i % 100000 == 0 && !i == 3 {
            idxes.push(context.get_random_angle());
        } else if i == 3 {
            let count = context.get_random_value(16, 32);
            let base = context.get_random_angle();
            for i in 0..count {
                idxes.push({
                    let mut tmp = i as f64 * TAU / count as f64
                        + (context.get_random_float() * 2.0 - 1.0) / 100.0
                        + base;
                    if tmp > TAU {
                        tmp -= TAU
                    };
                    tmp
                });
            }
        }
        let new_spines = link_roads(
            &tmp,
            &rings[rings.len() - 1],
            i as usize + base_idx,
            &mut idxes,
            context,
        );
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
    prof_frame!("Road::collect_rings_to_roads()");
    let mut out = vec![rings[0].inner.clone(), rings[0].outer.clone()];
    out.append(&mut rings[0].spines.clone());
    for i in 1..rings.len() {
        out.push(rings[i].outer.clone());
        out.append(&mut rings[i].spines.clone());
    }
    out
}

#[allow(unused)]
fn calc_push(
    idx: usize,
    array: &[Vector2; 4],
    top: &Road,
    bottom: &Road,
    left: &Road,
    right: &Road,
    center: Vector2,
    context: &Context,
) -> Option<Vector2> {
    fn hacky_max_sum(a: Vector2, b: Vector2) -> Vector2 {
        let sum = a + b;
        //let m = max_vec(a, b);
        //max_vec(sum, m)
        sum
    }
    //0 == lower_side start,
    //1 == upper side start,
    //2 = lower_side end,
    //3  = upper_side end
    //top is outer,
    //bottom is inner,
    //left is lower_side,
    //right is upper_side,
    // assert!(top.width > 0.0);
    //assert!(bottom.width > 0.0);
    // assert!(left.width > 0.0);
    //assert!(right.width > 0.0);
    let c = context.center();
    let base = normalize(&-((array[0] + array[1]) / 2.0 - c));
    let out_vec = base * bottom.width;
    //    let in_vec = normalize(&-((array[1] + array[2]) / 2.0 - c)) * top.width * 1.0;
    //   let left_vec = normalize(&-((array[0] + array[1]) / 2.0 - c)) * left.width * 1.0;
    //  let right_vec = normalize(&-((array[2] + array[3]) / 2.0 - c)) * right.width * 1.0;
    let in_vec = rotate_vec2(&base, PI) * top.width;
    let left_vec = rotate_vec2(&base, -PI / 2.0) * left.width;
    let right_vec = rotate_vec2(&base, PI / 2.0) * right.width;
    let mut out = None;
    if idx == 0 {
        out = Some(hacky_max_sum(out_vec, left_vec));
    } else if idx == 1 {
        out = Some(hacky_max_sum(in_vec, left_vec));
    } else if idx == 2 {
        out = Some(hacky_max_sum(out_vec, right_vec));
    } else if idx == 3 {
        out = Some(hacky_max_sum(in_vec, right_vec));
    }
    if !rectangle_contains_point(&Rectangle::from(*array).scale(1.2), &(array[idx] + out?)) {
        return None;
    }
    out
}

#[allow(unused)]
fn scale_rect_to_roads(
    base: &Rectangle,
    top: &Road,
    bottom: &Road,
    left: &Road,
    right: &Road,
    noise: &NoiseGenerator2d,
    context: &Context,
) -> Option<Rectangle> {
    prof_frame!("Road::scale_rect_to_roads()");
    let s = base.scale(1.0);
    let a = s.as_array();
    let center = s.center();
    let mut out = a;
    let mut count = 0;
    loop {
        let mut b = out;
        for i in 0..4 {
            //b[i] +=  calc_push_imp(i, &a, &[top, bottom, left, right], center, b[i], base)?
            b[i] += calc_push(i, &a, top, bottom, left, right, center, context)?;
        }
        out = b;
        count += 1;
        if count > 1 {
            break;
        }
    }
    let _s = noise.perlin(base.center() * 100.0) * 0.05;
    Some(Rectangle::from(out).scale(1.0))
}

#[allow(unused)]
fn segment_available_locations(
    inner: &Road,
    outer: &Road,
    lower_side: &Road,
    upper_side: &Road,
    noise: &NoiseGenerator2d,
    context: &Context,
) -> Vec<Rectangle> {
    prof_frame!("Road::segment_available_locations()");
    let two = 2 as f64;
    let base_opt = scale_rect_to_roads(
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
        noise,
        context,
    );
    if base_opt.is_none() {
        ///println!("base opt was none");
        return vec![];
    }
    let base = base_opt.unwrap();
    let v0 = base.v0;
    let v1 = base.v1;
    let v2 = base.v2;
    let v3 = base.v3;
    let bmid = (v0 + v1) / two;
    let idx0opt = outer.point_index(upper_side.get_end());
    if idx0opt.is_none() {
        return vec![];
    }
    let idx0 = idx0opt.unwrap();
    let idx1opt = outer.point_index(lower_side.get_end());
    if idx1opt.is_none() {
        return vec![];
    }
    let idx1 = idx1opt.unwrap();
    let tmid = {
        let idx = (idx0 + 1) % outer.points.len();
        let idx2 = idx1;
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
pub fn ring_available_locations(
    ring: &Ring,
    noise: &NoiseGenerator2d,
    context: &Context,
) -> Vec<Block> {
    prof_frame!("Road::ring_available_locations()");
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
                noise,
                context,
            )
            .into_iter()
            .map(|x| generate_building_from_rectangle(x))
            .collect(),
        };
        out.push(tmp);
    }
    out
}
