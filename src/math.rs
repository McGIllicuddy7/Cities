

pub use nalgebra_glm::*;

use crate::context::*;
pub type Vector2 = TVec2<f64>;
#[allow(unused)]
pub fn project_vector2_line(point: Vector2, start: Vector2, end: Vector2) -> Vector2 {
    if end == start {
        return end;
    }
    let s = point - start;
    let e = end - start;
    let delta = e.normalize();
    let v = dot(&s, &delta) * delta;
    v + start
}
#[allow(unused)]
pub fn is_between_points(point: Vector2, start: Vector2, end: Vector2) -> bool {
    let first = {
        let p = point - start;
        let e = end - start;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        dp < de
    };
    let second = {
        let p = point - end;
        let e = start - end;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        de < dp
    };
     first && second
}
#[allow(unused)]
pub fn dist_point_to_line(point: Vector2, start: Vector2, end: Vector2) -> f64 {
    let proj = project_vector2_line(point, start, end);
    let btwn = is_between_points(proj, start, end);
    if btwn {
        distance(&point, &proj)
    } else {
        let d0 = distance(&point, &start);
        let d1 = distance(&point, &end);
        if d0 < d1 {
            d0
        } else {
            d1
        }
    }
}
#[allow(unused)]
pub fn to_raylib_vec(v: Vector2) -> raylib::ffi::Vector2 {
    raylib::ffi::Vector2 {
        x: v.x as f32,
        y: v.y as f32,
    }
}
#[allow(unused)]
pub fn gradient(v:&[Vector2], point:Vector2)->Vector2{
    let mut out = vec2(0.0, 0.0);
    for p in v{
        let delta = p-point;
        let grad = delta/length2(&delta);
        out += grad;
    }
    out
}
#[allow(unused)]
pub fn gradient_clamped(v:&[Vector2], point:Vector2, max_radius:f64)->Vector2{
    let mut out = vec2(0.0, 0.0);
    for p in v{
        if distance(p, &point)<max_radius{
            let delta = p-point;
            let grad = delta/(length2(&delta));
            out += grad;
        }
    }
    out
}
#[allow(unused)]
pub fn int_power<T>(base:T, other:usize)->T
where T: std::ops::MulAssign+Copy+From<i32>
{
    let mut out = T::from(1);
    for i in 0..other{
        out *= base;
    }
    out
}
pub struct Rectangle{
    pub v0:Vector2,
    pub v1:Vector2,
    pub v2:Vector2, 
    pub v3:Vector2,
}
pub fn rect(v0:Vector2, v1:Vector2, v2:Vector2, v3:Vector2)->Rectangle{
    Rectangle{v0, v1, v2, v3}
}
impl Rectangle{
    #[allow(unused)]
    pub fn scale(&self, scale:f64)->Rectangle{
        let center = (self.v0+self.v1+self.v2+self.v3)/ 4_f64;
        let dv0 = self.v0-center;
        let dv1 = self.v1-center;
        let dv2 = self.v2-center;
        let dv3 = self.v3-center;
        Self{v0:center+dv0*scale, v1:center+dv1*scale, v2:center+dv2*scale, v3:center+dv3*scale}
    }
    #[allow(unused)]
    pub fn scale_mut(&mut self, scale:f64){
        *self = self.scale(scale);
    }
    #[allow(unused)]
    pub fn from(v:[Vector2;4])->Self{
         Self { v0: v[0], v1: v[1],v2:v[2] ,v3:v[3] }
    }
    #[allow(unused)]
    pub fn into(&self)->[Vector2;4]{
        [self.v0,self.v1,self.v2,self.v3]
    }
}
#[allow(unused)]
struct NoiseOctave1d{
    pub points:Vec<f64>,
    pub point_dist:f64
}
#[allow(unused)]
pub struct NoiseGenerator1d{
    octaves:Vec<NoiseOctave1d>,
    norm:f64
}
impl NoiseOctave1d{
    pub fn new(length:f64, point_dist:f64,context:&Context)->Self{
        let p0 = context.get_random_float();
        let mut points = vec![];
        let count = (length/point_dist).floor() as usize;
        for _ in 1..count{
            points.push(context.get_random_float());
        }
        points.push(p0);
        Self{points, point_dist}
    }
    pub fn get_value(&self,location:f64)->f64{
        let a = (location/self.point_dist).floor();
        let b = (location/self.point_dist).ceil();
        let l_val = location/self.point_dist-a;
        let a_ind = a as usize % self.points.len();
        let b_ind = b as usize % self.points.len();
        self.points[a_ind]*(1.0-l_val)+self.points[b_ind]*l_val
    }
}
impl NoiseGenerator1d{
    pub fn new(length:f64, point_dist:f64, octaves_count:usize, context:&Context)->Self{
        let octaves = (0..octaves_count).
        map(|i| NoiseOctave1d::new(length, point_dist/(int_power(2, i) as f64), context)).
        collect();
        let norm = (0..octaves_count).map(|i| int_power(0.5, i)).sum();
        return Self{octaves, norm};
    }
    pub fn get_value(&self,location:f64)->f64{
        ( 0..self.octaves.len()).map(|i| self.octaves[i].get_value(location)*int_power(0.5, i)/self.norm).sum()
    }
}