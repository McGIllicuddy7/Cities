use crate::context::Context;
use crate::math::*;
use crate::road;
use std::f64::consts::PI;

#[derive(Clone, Copy)]
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
        let p = self.into();
        let mut min = 1000 as f64;
        let mut max = 0 as f64;
        for i in 0..p.len() {
            for j in 0..p.len() {
                if i == j {
                    continue;
                }
                let d = distance(&p[i], &p[j]);
                if d < min {
                    min = d;
                }
                if d > max {
                    max = d;
                }
            }
        }
        let rat = 4.0;
        min / max > rat || max / min > rat
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

pub fn generate_blocks(rings: &[road::Ring], context: &Context) -> Vec<Block> {
    let mut out = vec![];
    for r in rings {
        let tmp = road::ring_available_locations(r, context);
        for t in tmp {
            out.push(t);
        }
    }
    out
}

#[derive(Clone)]
pub struct Block {
    pub buildings: Vec<Building>,
}
impl Block {
    pub fn center_mass(&self) -> Vector2 {
        let mut out = vec2(0.0, 0.0);
        for i in &self.buildings {
            out += i.center_mass();
        }
        out / (self.buildings.len() as f64)
    }
    pub fn distance_to_center(&self, context: &Context) -> f64 {
        distance(&self.center_mass(), &context.center())
    }
    #[allow(unused)]
    pub fn is_degenerate(&self) -> bool {
        false
    }
}

pub fn filter_blocks(blocks: &[Block], context: &Context) -> Vec<Block> {
    let mut out = vec![];
    let noise = NoiseGenerator2d::new(5, 1000.0, context);
    for b in blocks {
        let d = b.distance_to_center(context);
        let r =
            (noise.perlin(b.center_mass() - context.center()) * (context.width * 32) as f64).abs();
        if d > r {
            continue;
        }
        out.push(b.clone());
    }
    out
}

pub fn filter_buildings(buildings: &[Building], scaler: f64, context: &Context) -> Vec<Building> {
    let mut out = vec![];
    for b in buildings {
        if distance(&b.center_mass(), &context.center()) > (context.width / 2) as f64 * scaler {
            continue;
        }
        out.push(b.clone());
    }
    out
}

pub fn purge_degenerates(buildings: &[Building]) -> Vec<Building> {
    let mut out = vec![];
    for b in buildings {
        if !b.is_degenerate() {
            out.push(b.clone());
        }
    }
    out
}
