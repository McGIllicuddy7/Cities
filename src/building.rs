
use crate::context::Context;
use crate::math::*;
use crate::road;
#[allow(unused)]
#[derive(Clone, Copy)]
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
    pub fn center_mass(&self)->Vector2{
        (self.p0+self.p1+self.p2+self.p3)/4_f64
    }
    #[allow(unused)]
    pub fn iter_points<T>(&self, task:fn(Vector2)->T)->[T;4]{
        let  out:[T;4] = [task(self.p0),task(self.p1),task(self.p2),task(self.p3)];
        out
    }
    #[allow(unused)]
    pub fn from(v:[Vector2;4])->Self{
        Self { p0: v[0], p1: v[1],p2:v[2] ,p3:v[3] }
    }
    #[allow(unused)]
    pub fn into(&self)->[Vector2;4]{
        [self.p0,self.p1,self.p2,self.p3] 
    }
}
#[allow(unused)]
pub fn generate_building_from_rectangle(rect: Rectangle) -> Building {
    Building {
        p0: rect.v0,
        p1: rect.v1,
        p2: rect.v2,
        p3: rect.v3,
    }
}
pub fn generate_blocks(rings: &[road::Ring]) -> Vec<Block> {
    let mut out = vec![];
    for r in rings {
        let tmp = road::ring_available_locations(r);
        for t in tmp {
            out.push(t);
        }
    }
    out 
}

#[allow(unused)]
#[derive(Clone)]
pub struct Block {
    pub buildings: Vec<Building>,
}
impl Block{
    pub fn center_mass(&self)->Vector2{
        let mut out = vec2(0.0, 0.0);
        for i in &self.buildings{
            out += i.center_mass();
        }   
         out/(self.buildings.len() as f64)
    }
    pub fn distance_to_center(&self, context: &Context)->f64{
        distance(&self.center_mass(), &context.center())
    }
}
#[allow(unused)]
pub fn filter_blocks(blocks: &[Block], context: &Context) -> Vec<Block> {
    let mut out = vec![];
    for b in blocks{
        let d = b.distance_to_center(context);
        let r = unsafe{raylib::ffi::GetRandomValue(50, 1000)} as f64;
        if r>d{
            continue;
        }
        out.push(b.clone());
    }
    out
}
#[allow(unused)]
pub fn filter_buildings(buildings:&[Building], context:&Context)->Vec<Building>{
    let mut out = vec![];
    for b in buildings{
        
    }
    out
}