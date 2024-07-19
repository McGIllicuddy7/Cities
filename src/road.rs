use crate::building::generate_building_from_rectangle;
use crate::building::Block;
use crate::context::Context;
#[allow(unused)]
use crate::math::*;
use crate::prof_frame;
use std::collections::hash_map::HashMap;
#[allow(unused)]
use std::f64::consts::PI;
#[allow(unused)]
use std::f64::consts::TAU;
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
                (self.width*1.0) as f32,
                raylib::color::Color::WHITE.into()
             )
        }
    }

    #[allow(unused)]
    pub unsafe fn draw_as_water(&self, context: &crate::context::Context) {
        for i in 0..self.points.len() - 1 {
            let s = self.points[i];
            let e = self.points[(i + 1) % self.points.len()];
            let col = raylib::color::Color::BLACK;
            raylib::ffi::DrawCircleV(to_raylib_vec(e), (self.width/2.0)as f32, col.into());
            raylib::ffi::DrawLineEx(
                to_raylib_vec(s),
                to_raylib_vec(e),
                self.width as f32,
                col.into(),
            )
        }
        raylib::ffi::DrawCircleV(to_raylib_vec(self.points[0]), (self.width/2.0)as f32, raylib::color::Color::DARKBLUE.into());
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
    pub fn nearest_point_discrete_idx(&self, location: &Vector2) -> usize{
        prof_frame!("Road::nearest_point_discrete()");
        let mut min =0;
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
        return rotate_vec2(&delta, 90.0);
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
    pub fn get_point_idx(&self, v: Vector2) -> Option<usize> {
        let p: (i64, i64) = (v.x.round() as i64, v.y.round() as i64);
        self.point_index_map.get(&p).map(|i| *i)
    }
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
        let mut d_theta = theta_noise.perlin(vec2(min_r /32.0, theta_0 / 8.0)) * 2.0;
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
fn link_roads(r0: &Road, r1: &Road, idx: usize, context: &Context) -> Vec<Road> {
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
        let width = {
            if context.get_random_float() > 0.4 || idx % 5 == 0 {
                context.large_width*2.0
            } else {
                context.medium_width
            }
        };
        out.push(link_points_with_road(a.points[idx], b.points[i],width));
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
    let resolution = 50.0;
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
    rings.push(base);
    let base_idx = context.get_random_value(0, 10000) as usize;
    for i in 1..count {
        let radius = i as f64 * dradius;
        let ring_width = {
            if context.get_random_float() > 0.8 || i % 3== 0 {
                context.large_width * 1.0
            } else {
                context.small_width * 1.0
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
        let new_spines = link_roads(
            &tmp,
            &rings[rings.len() - 1],
            i as usize + base_idx,
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
fn make_new_location_make_sense(
    vc: Vector2,
    guess: Vector2,
    center: Vector2,
    road1: &Road,
    road2: &Road,
) -> Vector2 {
    prof_frame!("Road::make_new_location_make_sense()");
    let mut v = vc;
    if distance(&(guess + v), &guess) < distance(&guess, &center) {
        if road1.are_on_same_side_of(guess + v, center)
            && road2.are_on_same_side_of(guess + v, center)
        {
            return v;
        }
        if road1.are_on_same_side_of(guess + 2.0 * v, center)
            && road2.are_on_same_side_of(guess + 2.0 * v, center)
        {
            return 2.0 * v;
        }
    } else {
        v = -v;
        if road1.are_on_same_side_of(guess + v, center)
            && road2.are_on_same_side_of(guess + v, center)
        {
            return v;
        }
        if road1.are_on_same_side_of(guess + 2.0 * v, center)
            && road2.are_on_same_side_of(guess + 2.0 * v, center)
        {
            return 2.0 * v;
        }
    }
    return vec2(0.0, 0.0);
}

#[allow(unused)]
fn calc_push_imp(
    idx: usize,
    rect: &[Vector2; 4],
    roads: &[&Road; 4],
    center: Vector2,
    guess: Vector2,
    base :&Rectangle
) -> Option<Vector2> {
    prof_frame!("Road::calc_push_imp()");
    let nearest_idx = {
        let mut min_idx = 0;
        let mut min_dist = roads[0].distance_to(rect[idx]);
        for i in 0..4 {
            let dist = roads[i].distance_to(rect[idx]);
            if dist < min_dist {
                min_idx = i;
                min_dist = dist;
            }
        }
        min_idx
    };
    let second_nearest_idx = {
        let mut min_idx = (nearest_idx + 1) % 4;
        let mut min_dist = roads[min_idx].distance_to(rect[idx]);
        for i in 0..4 {
            if i == nearest_idx {
                continue;
            }
            let dist = roads[i].distance_to(rect[idx]);
            if dist < min_dist {
                min_idx = i;
                min_dist = dist;
            }
        }
        min_idx
    };
    let n0 = roads[nearest_idx];
    let n1 = roads[second_nearest_idx];
    let w = min(n0.width,n1.width);
    let failsafe = {
        let tmp =  n0.get_normal_at_location_toward(rect[idx],center)*n0.width+n1.get_normal_at_location_toward(rect[idx], center)*n1.width;
        if rectangle_contains_point(base, &tmp){
            Some(tmp)
        } else{
            Some((guess-rect[idx])*w)
        }
    };
    let road1_idx_opt = {
        let mut tmp = None;
        for i in 0..4 {
            if let Some(p) = roads[i].get_point_idx(rect[idx]) {
                tmp = Some(i)
            }
        }
        tmp
    };
    if road1_idx_opt.is_none() {
        return failsafe;
    }
    let road1_idx = road1_idx_opt.unwrap();
    let road2_idx_opt = {
        let mut tmp = None;
        for i in 0..4 {
            if let Some(p) = roads[i].get_point_idx(rect[idx]) {
                tmp = Some(i)
            }
        }
        tmp
    };
    if road2_idx_opt.is_none() {
        return failsafe;
    }
    let road2_idx = road2_idx_opt.unwrap();
    let road1 = roads[road1_idx];
    let road2 = roads[road2_idx];
    let failsafe = {
        let tmp = road1.get_normal_at_location_toward(rect[idx],center)*road1.width+road2.get_normal_at_location_toward(rect[idx], center)*road2.width;
        if rectangle_contains_point(base, &tmp){
            Some(tmp)
        } else{
            failsafe
        }
    };
    let road1_other_opt = {
        let mut tmp = None;
        for i in 0..4 {
            if i == idx {
                continue;
            }
            if let Some(p) = road1.get_point_idx(rect[i]) {
                tmp = Some(i);
            }
        }
        tmp
    };
    if road1_other_opt.is_none() {
        return failsafe;
    }
    let road1_other = road1_other_opt.unwrap();
    let road2_other_opt = {
        let mut tmp = None;
        for i in 0..4 {
            if i == idx {
                continue;
            }
            if let Some(p) = road2.get_point_idx(rect[i]) {
                tmp = Some(i);
            }
        }
        tmp
    };
    if road2_other_opt.is_none() {
        return failsafe;
    }
    let road2_other = road2_other_opt.unwrap();
    let r1nxt = {
        if road1_other > road1_idx {
            (road1_idx + 1) % road1.points.len()
        } else {
            if road1_idx != 0 {
                (road1_idx - 1)
            } else {
                (road1.points.len() - 1)
            }
        }
    };
    let r2nxt = {
        if road2_other > road2_idx {
            (road2_idx + 1) % road2.points.len()
        } else {
            if road2_idx != 0 {
                (road2_idx - 1)
            } else {
                (road2.points.len() - 1)
            }
        }
    };
    let r1p = road1.points[r1nxt];
    let r2p = road2.points[r2nxt];
    let dp = normalize(&(rect[idx] - center));
    let r1v = {
        let tmp = normalize(&(r1p - rect[idx]));
        let rot = rotate_vec2(&tmp, 90.0);
        if dot(&rot, &dp) < 0.0 {
            -rot * road1.width
        } else {
            rot * road1.width
        }
    };
    let r2v = {
        let tmp = normalize(&(r2p - rect[idx]));
        let rot = rotate_vec2(&tmp, 90.0);
        if dot(&rot, &dp) < 0.0 {
            -rot * road2.width
        } else {
            rot * road2.width
        }
    };
    let out = r1v + r2v;
    let tmp = make_new_location_make_sense(out, guess, center, road1, road2);
    if !rectangle_contains_point(base, &(guess+tmp)){
        return None;
    }
  Some ( tmp)
}

#[allow(unused)]
fn scale_rect_to_roads(
    base: &Rectangle,
    top: &Road,
    bottom: &Road,
    left: &Road,
    right: &Road,
    noise: &NoiseGenerator2d,
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
            if let Some(p)= calc_push_imp(i, &a, &[top, bottom, left, right], center, b[i], base){
                b[i] += p
            }else{
                return None;
            }
        }
        out = b;
        count += 1;
        if count >1{
            break;
        }
    }
    let s= noise.perlin(base.center()*100.0)*0.2;
    Some(Rectangle::from(out).scale(0.8+s))
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
    );
    if base_opt.is_none(){
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
