use std::f64::consts::TAU;


use crate::context::Context;
use crate::math::*;
use crate::prof_frame;
use crate::road;
#[derive(Clone, Copy, PartialEq)]
pub struct Building {
    pub p0: Vector2,
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
}
impl Building {
    pub unsafe fn draw(&self, _context: &Context) {
        let a = to_raylib_vec(self.p0);
        let b = to_raylib_vec(self.p1);
        let c = to_raylib_vec(self.p2);
        let d = to_raylib_vec(self.p3);
        let col = raylib::color::Color::BLACK.into();
        raylib::ffi::DrawLineV(a, b, col);
        raylib::ffi::DrawLineV(a, c, col);
        raylib::ffi::DrawLineV(b, d, col);
        raylib::ffi::DrawLineV(c, d, col);
    }
    pub fn center_mass(&self) -> Vector2 {
        (self.p0 + self.p1 + self.p2 + self.p3) / 4_f64
    }
    #[allow(unused)]
    pub fn from(v: [Vector2; 4]) -> Self {
        Self {
            p0: v[0],
            p1: v[1],
            p2: v[2],
            p3: v[3],
        }
    }

    pub fn into(&self) -> [Vector2; 4] {
        [self.p0, self.p1, self.p2, self.p3]
    }
    #[allow(unused)]
    pub fn get_deltas(&self) -> [Vector2; 4] {
        [
            self.p1 - self.p0,
            self.p2 - self.p0,
            self.p3 - self.p1,
            self.p3 - self.p2,
        ]
    }
    fn is_degenerate(&self) -> bool {
        prof_frame!("Building::is_degenerate()");
        fn lrotate(points: [Vector2; 4]) -> [Vector2; 4] {
            [points[1], points[2], points[3], points[0]]
        }
        fn is_degen_tmp(p: [Vector2; 4]) -> bool {
            if Building::from(p).area() < 10.0 {
                return true;
            }
            let d1 = distance(&p[0], &p[1]);
            let d2 = distance(&p[0], &p[2]);
            let d3 = distance(&p[0], &p[3]);
            let e1 = distance(&p[3], &p[1]);
            let e2 = distance(&p[3], &p[2]);
            d1 <= d3 && d2 <= d3 && e1 <= d3 && e2 <= d3
        }
        let p0 = self.into();
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    if distance(&p0[i], &p0[j]) < 10.0 {
                        return true;
                    }
                }
            }
        }
        if !is_degen_tmp(p0) {
            return false;
        }
        let p1 = lrotate(p0);
        if !is_degen_tmp(p1) {
            return false;
        }
        let p2 = lrotate(p1);
        if !is_degen_tmp(p2) {
            return false;
        }
        let p3 = lrotate(p2);
        if !is_degen_tmp(p3) {
            return false;
        }
        return true;
    }
    fn i_fix_pls(&self) -> Option<Self> {
        let p = self.into();
        let d1 = distance(&p[0], &p[1]);
        let d2 = distance(&p[0], &p[2]);
        let d3 = distance(&p[0], &p[3]);
        let e1 = distance(&p[3], &p[1]);
        let e2 = distance(&p[3], &p[2]);
        let mut out = p.clone();
        if d1 > d3 {
            out[0] = p[1];
            out[3] = p[0];
        }
        if d2 > d3 {
            out[0] = p[2];
            out[3] = p[0];
        }
        if e1 > d3 {
            out[3] = p[1];
            out[1] = p[3];
        }
        if e2 > d3 {
            out[3] = p[2];
            out[2] = p[3];
        }
        let out_a = Self::from(out);
        if out_a.is_degenerate() {
            None
        } else {
            Some(out_a)
        }
    }
    pub fn to_rect(&self) -> Rectangle {
        Rectangle {
            v0: self.p0,
            v1: self.p1,
            v2: self.p2,
            v3: self.p3,
        }
    }
    ///returns the approximate area assuming the thing is mostly rectangular, the longest length
    ///times the shortest length
    #[allow(unused)]
    pub fn area(&self) -> f64 {
        prof_frame!("Building::area()");
        let points = self.to_rect().as_array();
        let mut max = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                if j == i {
                    continue;
                }
                let d = distance(&points[i], &points[j]);
                if d > max {
                    max = d;
                }
            }
        }
        let mut min = max;
        for i in 0..4 {
            for j in 0..4 {
                if j == i {
                    continue;
                }
                let d = distance(&points[i], &points[j]);
                if d < min {
                    min = d;
                }
            }
        }
        return max * min;
    }
}

pub fn generate_building_from_rectangle(rect: Rectangle) -> Building {
    Building {
        p0: rect.v0,
        p1: rect.v1,
        p2: rect.v2,
        p3: rect.v3,
    }
}

pub fn generate_blocks(rings: Vec<road::Ring>, context: &Context) -> Vec<Block> {
    use std::sync::Arc;
    use std::thread;
    fn generation_thread(
        rings_arc: Arc<Vec<road::Ring>>,
        start: usize,
        end: usize,
        noise_arc: Arc<NoiseGenerator2d>,
        context_arc: Arc<Context>,
    ) -> Vec<Block> {
        let rings = &rings_arc;
        let context = &context_arc;
        let noise = &noise_arc;
        let mut out = vec![];
        for i in start..end {
            out.push(road::ring_available_locations(&rings[i], noise, context))
        }
        out.into_iter().flatten().collect()
    }
    prof_frame!("Building::generate_blocks()");
    let noise = NoiseGenerator2d::new(10, 500.0, context);
    let noise_arc = Arc::new(noise);
    let l = rings.len();
    let arc = Arc::new(rings);
    let a0 = arc.clone();
    let a1 = arc.clone();
    let a2 = arc.clone();
    let a3 = arc.clone();
    let n0 = noise_arc.clone();
    let n1 = noise_arc.clone();
    let n2 = noise_arc.clone();
    let n3 = noise_arc.clone();
    let con = Arc::new(context.clone());
    let c0 = con.clone();
    let c1 = con.clone();
    let c2 = con.clone();
    let c3 = con.clone();
    let t0 = thread::spawn(move || generation_thread(a0, 0, l / 4, n0, c0));
    let t1 = thread::spawn(move || generation_thread(a1, l / 4, 2 * l / 4, n1, c1));
    let t2 = thread::spawn(move || generation_thread(a2, 2 * l / 4, 3 * l / 4, n2, c2));
    let t3 = generation_thread(a3, 3 * l / 4, l, n3, c3);
    [
        &t0.join().unwrap()[..],
        &t1.join().unwrap()[..],
        &t2.join().unwrap()[..],
        &t3[..],
    ]
    .concat()
}

#[derive(Clone)]
pub struct Block {
    pub buildings: Vec<Building>,
}
impl Block {
    #[allow(unused)]
    pub fn center_mass(&self) -> Vector2 {
        prof_frame!("Block::center_mass()");
        let mut out = vec2(0.0, 0.0);
        for i in &self.buildings {
            out += i.center_mass();
        }
        out / (self.buildings.len() as f64)
    }
    #[allow(unused)]
    pub fn distance_to_center(&self, context: &Context) -> f64 {
        distance(&self.center_mass(), &context.center())
    }
    #[allow(unused)]
    pub fn is_degenerate(&self) -> bool {
        false
    }
}

pub fn filter_buildings(buildings: &[Building], scaler: f64, context: &Context) -> Vec<Building> {
    fn building_outside(b: &Building, scaler: f64, noise:&NoiseGenerator1d,context: &Context) -> bool {
        let mut outside = false;
        let a = b.to_rect().as_array();
        for i in &a {
            if !((i.y > (context.height / 2) as f64 - (context.height / 2) as f64 * scaler
                && i.y < (context.height) as f64 * scaler)
                && (i.x > (context.width / 2) as f64 - (context.width / 2) as f64 * scaler
                    && i.x < (context.width) as f64 * scaler))
            {
                outside = true;
                break;
            }
        }
        let inside = {
            let delta = b.center_mass()-context.center();
            let theta = angle(&delta, &vec2(1.0,0.0));
            let rad = length(&delta);
            let max_rad = (context.width) as f64 * (0.5+(noise.get_value(theta)+1.0/2.0)*0.5);
            rad<max_rad
        };
        outside || !inside
    }
    prof_frame!("Building::filter_buildings()");
    let mut out = vec![];
    let noise =NoiseGenerator1d::new(TAU, 1.0,10, context);
    for b in buildings {
        if distance(&b.center_mass(), &context.center()) > (context.width / 2) as f64 * scaler*2_f64.sqrt() {
           continue;
        }
        if building_outside(b, scaler, &noise,context) {
            continue;
        }
        if b.area() > 1000.0 {
            out.push(Building::from(b.to_rect().scale(0.9).as_array()));
        } else {
            out.push(b.clone());
        }
    }
    out
}

#[allow(unused)]
fn exterminadus(
    buildings_arc: std::sync::Arc<[Building]>,
    start: usize,
    end: usize,
    amap:std::sync::Arc<HashGrid<Building>>,
) -> Vec<Building> {
    prof_frame!("Building::exterminadus()");
    let buildings: &[Building] = &buildings_arc;
    let map = &amap;
    let mut out = vec![];
    for i in start..end {
        let mut overlaps = false;
        let points = buildings[i].to_rect().as_array();
        for j in points {
            let p = map.get((j.x, j.y));
            for k in p {
                if buildings[i] == *k {
                    continue;
                }
                if rectangles_overlap(&buildings[i].to_rect(), &k.to_rect()) {
                    if buildings[i].area() > k.area() {
                        overlaps = true;
                        break;
                    }
                }
            }
        }
        if !overlaps {
            out.push(buildings[i]);
        }
    }
    out
}

#[allow(unused)]
fn purge_degenerates_second_stage(state0:Vec<Building>, buildings:&[Building])->Vec<Building>{
    use std::sync::Arc;
    use std::thread;
    let s: Arc<[Building]> = state0.into();
    let s0 = s.clone();
    let s1 = s.clone();
    let s2 = s.clone();
    let s3 = s.clone();
    let mut v = vec![];
    for b in buildings {
        for i in b.to_rect().as_array() {
            let t = (i.x, i.y, *b);
            v.push(t);
        }
    }
    let map = HashGrid::new(&v, 128);
    let amap = Arc::new(map);
    let m0 = amap.clone();
    let m1 = amap.clone();
    let m2 = amap.clone();
    let m3 = amap.clone();
    let l = s.len();
    let t0 = thread::spawn(move || (exterminadus(s0, 0, l / 4,m0)));
    let t1 = thread::spawn(move || (exterminadus(s1, l / 4, l / 2,m1)));
    let t2 = thread::spawn(move || (exterminadus(s2, l / 2, 3 * l / 4,m2)));
    let t3 = exterminadus(s3, 3 * l / 4, l,m3);
    vec![
        t0.join().unwrap(),
        t1.join().unwrap(),
        t2.join().unwrap(),
        t3,
    ]
    .into_iter()
    .flatten()
    .collect()
}
pub fn purge_degenerates(buildings: &[Building]) -> Vec<Building> {
    prof_frame!("Building::purge_degenerates()");
    let mut state0 = vec![];
    for b in buildings {
        if !b.is_degenerate() {
            state0.push(b.clone());
        } else {
            if let Some(p) = b.i_fix_pls() {
                state0.push(p);
            }
        }
    }
    return state0;
}
