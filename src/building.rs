use crate::context::Context;
use crate::math::*;
use crate::road;
#[allow(unused)]
pub struct Building {
    pub p0: Vector2,
    pub p1: Vector2,
    pub p2: Vector2,
    pub p3: Vector2,
}
impl Building {
    #[allow(unused)]
    pub fn draw(&self, context: &Context) {
        unsafe {
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
    }
}
#[allow(unused)]
pub fn generate_building_from_rectangle(rect: Rectangle) -> Building {
    return Building {
        p0: rect.v0,
        p1: rect.v1,
        p2: rect.v2,
        p3: rect.v3,
    };
}
pub fn generate_blocks(rings: &[road::Ring]) -> Vec<Block> {
    let mut out = vec![];
    for r in rings {
        let tmp = road::ring_available_locations(r);
        for t in tmp {
            out.push(t);
        }
    }
    return out;
}

#[allow(unused)]
pub struct Block {
    pub buildings: Vec<Building>,
}
#[allow(unused)]
pub fn filter_blocks(blocks: &[Block], context: &Context) -> Vec<Block> {
    let mut out = vec![];
    return out;
}
